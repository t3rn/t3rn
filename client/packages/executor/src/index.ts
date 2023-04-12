import "@polkadot/api-augment"
// @ts-ignore
import { Sdk } from "@t3rn/sdk"
import { ApiPromise, Keyring } from "@polkadot/api"
import { cryptoWaitReady } from "@polkadot/util-crypto"
import { KeyringPair } from "@polkadot/keyring/types"
import { readFile, writeFile, mkdir } from "fs/promises"
import { dirname, join } from "path"
import { homedir } from "os"
import readline from "readline"
require("dotenv").config()
import "@t3rn/types"
import { SubstrateRelayer, CostEstimator, Estimator, Estimate, InclusionProof } from "./gateways/substrate/relayer"
import { ExecutionManager, Queue } from "./executionManager"
import { Config, Gateway, Circuit, Strategy } from "../config/config"
import { BiddingEngine, BiddingStrategy } from "./bidding"
import { PriceEngine, CoingeckoPricing } from "./pricing"
import { StrategyEngine, SfxStrategy, XtxStrategy } from "./strategy"
import { SideEffect, Notification, NotificationType, TxOutput, TxStatus } from "./executionManager/sideEffect"
import { Execution } from "./executionManager/execution"
import { CircuitListener, ListenerEvents, ListenerEventData } from "./circuit/listener"
import { CircuitRelayer } from "./circuit/relayer"
// @ts-ignore
import { T3rnPrimitivesXdnsXdnsRecord } from "@polkadot/types/lookup"
import * as defaultConfig from "../config.json"
import { problySubstrateSeed } from "./utils"
import { default as pino, Logger, P } from "pino"
// const pino = require("pino")
// const logger = pino(
//     {
//         level: process.env.LOG_LEVEL || "info",
//         formatters: {
//             level: (label) => {
//                 return { level: `${name} ${label}` }
//             },
//         },
//         base: undefined,
//     }
//     // pino.destination(`${__dirname}/logger.log`) // remove comment to export to file
// )

/** An executor instance. */
class Instance {
    circuitClient: ApiPromise
    executionManager: ExecutionManager
    sdk: Sdk
    signer: KeyringPair
    config: Config
    logger: Logger

    /**
     * Sets up and configures an executor instance.
     *
     * @param name Display name and config identifier for an instance
     * @param logToDisk Write logs to disk within ~/.t3rn-executor-${name}/logs
     * @returns Instance
     */
    async setup(name: string = "example", logToDisk = false): Promise<Instance> {
        const config = await this.loadConfig(name)
        if (logToDisk) {
            const logDir = join(homedir(), `.t3rn-executor-${name}`, "logs")
            await mkdir(logDir, { recursive: true })
            this.logger = pino(
                {
                    level: process.env.LOG_LEVEL || "info",
                    formatters: {
                        bindings(bindings) {
                            return { ...bindings, name }
                        },
                    },
                },
                pino.destination(join(logDir, `${Date.now()}.log`))
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
        await cryptoWaitReady()
        this.signer = new Keyring({ type: "sr25519" })
            // loadConfig asserts that config.circuit.signerKey is set
            .addFromSeed(Uint8Array.from(Buffer.from(config.circuit.signerKey!.slice(2), "hex")))
        this.sdk = new Sdk(config.circuit.rpc, this.signer)
        // @ts-ignore
        this.circuitClient = await this.sdk.init()
        this.executionManager = new ExecutionManager(this.circuitClient, this.sdk, this.logger)
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
    async loadConfig(name: string): Promise<Config> {
        const configFile = join(homedir(), `.t3rn-executor-${name}`, "config.json")
        const configDir = dirname(configFile)
        await mkdir(configDir, { recursive: true })
        const persistedConfig = await readFile(configFile)
            // if the persisted config file does not exist yet we wanna
            // handle it gracefully because it is probly an initial run
            .then((buf) => {
                try {
                    return JSON.parse(buf.toString())
                } catch (err) {
                    this.logger.warn(`failed reading ${configFile} ${err}`)
                    return {}
                }
            })
            .catch((err) => {
                this.logger.warn(`failed reading ${configFile} ${err}`)
                return {}
            })
        const config = { ...defaultConfig, ...persistedConfig }
        await writeFile(configFile, JSON.stringify(config))
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

    /** Registers a keypress listener for Ctrl+C that initiates instance shutdown. */
    private registerExitListener() {
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
