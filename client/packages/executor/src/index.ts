import "@polkadot/api-augment"
// @ts-ignore
import { Sdk } from "@t3rn/sdk"
import { ApiPromise, Keyring } from "@polkadot/api"
import { cryptoWaitReady } from "@polkadot/util-crypto"
import { KeyringPair } from "@polkadot/keyring/types"
import { PathLike, existsSync } from "fs"
import { readFile, writeFile, mkdir } from "fs/promises"
import { join } from "path"
import { homedir } from "os"
import readline from "readline"
require("dotenv").config()
import "@t3rn/types"
import { SubstrateRelayer, CostEstimator, Estimator, Estimate, InclusionProof } from "./gateways/substrate/relayer"
import { ExecutionManager, PersistedState, Queue } from "./executionManager"
import { Config, Gateway, Circuit, Strategy } from "../config/config"
import { BiddingEngine, BiddingStrategy } from "./bidding"
import { PriceEngine, CoingeckoPricing } from "./pricing"
import { StrategyEngine, SfxStrategy, XtxStrategy } from "./strategy"
import { SideEffect, Notification, NotificationType, TxOutput, TxStatus } from "./executionManager/sideEffect"
import { Execution } from "./executionManager/execution"
import { CircuitListener, ListenerEvents, ListenerEventData } from "./circuit/listener"
import { CircuitRelayer } from "./circuit/relayer"
import * as defaultConfig from "../config.json"
import { problySubstrateSeed } from "./utils"
import { default as pino, Logger } from "pino"

/** An executor instance. */
class Instance {
    name: string
    circuitClient: ApiPromise
    executionManager: ExecutionManager
    sdk: Sdk
    signer: KeyringPair
    config: Config
    logger: Logger
    baseDir: PathLike
    logsDir: PathLike
    configFile: PathLike
    stateFile: PathLike
    logToDisk: boolean

    /**
     * Initializes an executor instance.
     *
     * @param name Display name and config identifier for an instance
     * @param logToDisk Write logs to disk within ~/.t3rn-executor-${name}/logs
     */
    constructor(name: string = "example", logToDisk: boolean = false) {
        this.name = name
        this.logToDisk = logToDisk
        this.baseDir = join(homedir(), `.t3rn-executor-${name}`)
        this.logsDir = join(this.baseDir.toString(), "logs")
        this.stateFile = join(this.baseDir.toString(), "state.json")
        this.configFile = join(homedir(), `.t3rn-executor-${name}`, "config.json")
    }

    /**
     * Sets up and configures an executor instance.
     *
     * @param name Display name and config identifier for an instance
     * @param logToDisk Write logs to disk within ~/.t3rn-executor-${name}/logs
     * @returns Instance
     */
    async setup(): Promise<Instance> {
        await this.configureLogging(this.name, this.logToDisk)
        const config = await this.loadConfig()
        await cryptoWaitReady()
        this.signer = new Keyring({ type: "sr25519" })
            // loadConfig asserts that config.circuit.signerKey is set
            .addFromSeed(Uint8Array.from(Buffer.from(config.circuit.signerKey!.slice(2), "hex")))
        this.sdk = new Sdk(config.circuit.rpc, this.signer)
        // @ts-ignore
        this.circuitClient = await this.sdk.init()
        this.executionManager = new ExecutionManager(this.circuitClient, this.sdk, this.logger)
        this.injectState()
        await this.executionManager.setup(config.gateways, config.vendors)
        this.registerExitListener()
        this.logger.info("setup complete")
        return this
    }

    /**
     * Loads an instance's config file thereby updating any config changes
     * staged at ../config.json by persisting the effective instance config
     * at ~/.t3rn-executor-${name}/config.json.
     *
     * @param name Executor instance name
     * @returns Instance configuration
     */
    async loadConfig(): Promise<Config> {
        await mkdir(this.baseDir, { recursive: true })
        const persistedConfig = await readFile(this.configFile)
            // if the persisted config file does not exist yet we wanna
            // handle it gracefully because it is probly an initial run
            .then((buf) => {
                try {
                    return JSON.parse(buf.toString())
                } catch (err) {
                    this.logger.warn(`failed reading ${this.configFile} ${err}`)
                    return {}
                }
            })
            .catch((err) => {
                this.logger.warn(`failed reading ${this.configFile} ${err}`)
                return {}
            })
        const config = { ...defaultConfig, ...persistedConfig }
        await writeFile(this.configFile, JSON.stringify(config))
        if (!config.circuit.signerKey?.startsWith("0x")) {
            config.circuit.signerKey = process.env.CIRCUIT_SIGNER_KEY as string
        }
        config.gateways.forEach((gateway) => {
            if (gateway.signerKey !== undefined && !problySubstrateSeed(gateway.signerKey)) {
                gateway.signerKey = process.env[`${gateway.id.toUpperCase()}_GATEWAY_SIGNER_KEY`] as string
            }
        })
        if (!problySubstrateSeed(config.circuit.signerKey)) {
            throw Error("Instance::loadConfig: missing circuit signer key")
        }
        if (!config.gateways.some((gateway) => problySubstrateSeed(gateway.signerKey))) {
            throw Error("Instance::loadConfig: missing gateway signer key")
        }
        this.config = config
        return config
    }

    /**
     * Loads persisted execution state.
     * @returns Instance
     */
    async injectState(): Promise<Instance> {
        if (existsSync(this.stateFile)) {
            const state = await readFile(this.stateFile, "utf8").then(JSON.parse)
            this.executionManager.inject(state)
        }
        return this
    }

    /** Configures the instance's pino logger.
     *
     * @param name Display name and config identifier for an instance
     * @param logToDisk Write logs to disk within ~/.t3rn-executor-${name}/logs
     */
    private async configureLogging(name: string, logToDisk: boolean): Promise<Instance> {
        if (logToDisk) {
            await mkdir(this.logsDir, { recursive: true })
            this.logger = pino(
                {
                    level: process.env.LOG_LEVEL || "info",
                    formatters: {
                        bindings(bindings) {
                            return { ...bindings, name }
                        },
                    },
                },
                pino.destination(join(this.logsDir.toString(), `${Date.now()}.log`))
            )
        } else {
            this.logger = pino({
                level: process.env.LOG_LEVEL || "info",
                formatters: {
                    bindings(bindings) {
                        return { ...bindings, name }
                    },
                },
            })
        }
        return this
    }

    /** Registers a keypress listener for Ctrl+C that initiates instance shutdown. */
    private registerExitListener(): Instance {
        // soft exit
        readline.emitKeypressEvents(process.stdin)
        process.stdin.on("keypress", async (_, { ctrl, name }) => {
            if (ctrl && name === "c") {
                this.logger.info("shutting down...")
                await this.executionManager.shutdown()
                process.exit(0)
            }
        })
        process.stdin.setRawMode(true)
        process.stdin.resume()
        // hard exit
        process.once("exit", async () => {
            const serializedState = JSON.stringify({
                queue: this.executionManager.queue,
                xtx: this.executionManager.xtx,
                sfxToXtx: this.executionManager.sfxToXtx,
                targetEstimator: this.executionManager.targetEstimator,
                relayers: this.executionManager.relayers,
            })
            await writeFile(join(this.baseDir.toString(), "state.json"), serializedState)
        })
        return this
    }
}

export {
    Instance,
    ExecutionManager,
    Queue,
    Execution,
    SideEffect,
    Notification,
    NotificationType,
    TxOutput,
    TxStatus,
    SubstrateRelayer,
    Estimator,
    CostEstimator,
    Estimate,
    InclusionProof,
    BiddingEngine,
    StrategyEngine,
    SfxStrategy,
    XtxStrategy,
    PriceEngine,
    CoingeckoPricing,
    CircuitListener,
    ListenerEvents,
    ListenerEventData,
    CircuitRelayer,
    Config,
    Circuit,
    Gateway,
    Strategy,
    BiddingStrategy,
}
