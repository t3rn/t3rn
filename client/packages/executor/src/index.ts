import "@polkadot/api-augment"
// @ts-ignore
import { Sdk } from "@t3rn/sdk"
import { Keyring } from "@polkadot/api"
require("dotenv").config()
import "@t3rn/types"
import { SubstrateRelayer, CostEstimator, Estimator, Estimate, InclusionProof } from "./gateways/substrate/relayer"
import { ExecutionManager, Queue } from "./executionManager"
import { ApiPromise } from "@polkadot/api"
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
import { cryptoWaitReady } from "@polkadot/util-crypto"
import { readFile, writeFile, mkdir } from "fs/promises"
import { dirname } from "path"
import { homedir } from "os"
import readline from "readline"
import * as defaultConfig from "../config.json"
import { problySubstrateSeed } from "./utils"

const pino = require("pino")
const logger = pino(
    {
        level: process.env.LOG_LEVEL || "info",
        formatters: {
            level: (label) => {
                return { level: label }
            },
        },
        base: undefined,
    }
    // pino.destination(`${__dirname}/logger.log`) // remove comment to export to file
)

/** An executor instance. */
class Instance {
    circuitClient: ApiPromise
    executionManager: ExecutionManager
    sdk: Sdk
    signer: any

    /**
     * Sets up and configures an executor instance.
     *
     * @param name Display name and config identifier for an instance
     * @returns Instance
     */
    async setup(name: string = "example"): Promise<Instance> {
        const config = await this.loadConfig(name)
        await cryptoWaitReady()
        this.signer = new Keyring({ type: "sr25519" })
            // loadConfig asserts that config.circuit.signerKey is set
            .addFromSeed(Buffer.from(config.circuit.signerKey!, "hex"))
        this.sdk = new Sdk(config.circuit.rpc, this.signer)
        // @ts-ignore
        this.circuitClient = await this.sdk.init()
        this.executionManager = new ExecutionManager(this.circuitClient, this.sdk, logger)
        await this.executionManager.setup(config.gateways, config.vendors)
        this.registerExitListener()
        logger.info("Executor: setup complete")
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
        const configFile = `${homedir()}/.t3rn-executor-${name}/config.json`
        const configDir = dirname(configFile)
        await mkdir(configDir, { recursive: true, mode: 600 })
        const persistedConfig = await readFile(configFile)
            // if the persisted config file does not exist yet we wanna
            // handle it gracefully because it is probly an initial run
            .then((buf) => {
                try {
                    return JSON.parse(buf.toString())
                } catch (err) {
                    logger.warn(`failed reading ${configFile} ${err}`)
                    return {}
                }
            })
            .catch((err) => {
                logger.warn(`failed reading ${configFile} ${err}`)
                return {}
            })
        const config = { ...defaultConfig, ...persistedConfig }
        await writeFile(configFile, JSON.stringify(config))
        if (!config.circuit.signerKey?.startsWith("0x")) {
            config.circuit.signerKey = process.env.CIRCUIT_SIGNER_KEY as string
        }
        config.gateways.forEach((gateway) => {
            if (gateway.signerKey !== undefined && !problySubstrateSeed(gateway.signerKey)) {
                gateway.signerKey = process.env[`${gateway.name.toUpperCase()}_GATEWAY_SIGNER_KEY`] as string
            }
        })
        if (!problySubstrateSeed(config.circuit.signerKey)) {
            throw Error("Instance::loadConfig: missing circuit signer key")
        }
        if (!config.gateways.some((gateway) => problySubstrateSeed(gateway.signerKey))) {
            throw Error("Instance::loadConfig: missing gateway signer key")
        }
        return config
    }

    /** Registers a keypress listener for Ctrl+C that initates instance shutdown. */
    private registerExitListener() {
        readline.emitKeypressEvents(process.stdin)
        process.stdin.on("keypress", async (_, { ctrl, name }) => {
            if (ctrl && name === "c") {
                console.log("shutting down...")
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

async function main() {
    const instanceManager = new Instance()
    await instanceManager.setup(process.env.EXECUTOR)
}

main()
