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

readline.emitKeypressEvents(process.stdin)
process.stdin.setRawMode(true)
process.stdin.resume()

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

/**
 * Class used for initializing the executor
 *
 * @group Utils
 */
class InstanceManager {
    circuitClient: ApiPromise
    executionManager: ExecutionManager
    sdk: Sdk
    signer: any

    async setup(name: string = "example") {
        const configFile = `${homedir()}/.t3rn-executor-${name}/config.json`
        const configDir = dirname(configFile)
        await mkdir(configDir, { recursive: true, mode: 600 })

        const persistedConfig = await readFile(configFile)
            .then((buf) => {
                try {
                    return JSON.parse(buf.toString())
                } catch (_err) {
                    console.warn(`${configFile} contains invalid JSON`)
                    return {}
                }
            })
            .catch((_err) => {
                // if the persisted config file does not exist yet we wanna
                // handle it gracefully because it is probly an initial run
                return {}
            })

        const config = { ...defaultConfig, ...persistedConfig }
        if (!config.circuit.signerKey.startsWith("0x")) {
            config.circuit.signerKey = process.env.CIRCUIT_SIGNER_KEY as string
        }
        config.gateways.forEach((gateway) => {
            if (gateway.signerKey !== undefined && !gateway.signerKey.startsWith("0x")) {
                gateway.signerKey = process.env[`${gateway.name.toUpperCase()}_GATEWAY_SIGNER_KEY`] as string
            }
        })

        await writeFile(configFile, JSON.stringify(config))

        if (!config.circuit.signerKey) {
            throw Error("InstanceManager::setup: missing signer keys")
        }

        await cryptoWaitReady()
        const keyring = new Keyring({ type: "sr25519" })

        this.signer = config.circuit.signerKey
            ? keyring.addFromMnemonic(config.circuit.signerKey)
            : keyring.addFromUri("//Executor//default")

        this.sdk = new Sdk(config.circuit.rpc, this.signer)

        // @ts-ignore
        this.circuitClient = await this.sdk.init()

        this.executionManager = new ExecutionManager(this.circuitClient, this.sdk, logger)
        await this.executionManager.setup(config.gateways, config.vendors)

        process.stdin.on("keypress", async (ch, key) => {
            if (key && key.ctrl && key.name == "k") {
                console.log("shutting down...")
                await this.executionManager.shutdown()
                process.exit(0)
            }
        })

        logger.info("Executor: setup complete")
    }
}

export {
    InstanceManager,
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
    const instanceManager = new InstanceManager()
    await instanceManager.setup(process.env.EXECUTOR)
}

main()
