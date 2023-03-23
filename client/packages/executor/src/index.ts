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
import { readFile,writeFile} from "fs/promises"
import * as defaultConfig from "../config.json"


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
        const configFile = `~/.t3rn-executor-${name}/config.json`
        const persistedConfig = await readFile(configFile).then(buf => {
            try {
                return JSON.parse(buf.toString())
            }catch (_err) {
                console.warn(`could not load persisted config from ${configFile}`)
                return {}
            }
        })
  
        const config = { ...defaultConfig, ...persistedConfig }
        if (!config.circuit.signerKey) {
            config.circuit.signerKey = process.env.CIRCUIT_SIGNER_KEY as string
        }
        
        config.gateways.forEach(gateway => {
            if (!gateway.signerKey) {
                gateway.signerKey = process.env[`${gateway.name.toUpperCase()}_GATEWAY_SIGNER_KEY`] as string
            }
        })

        await writeFile(configFile, JSON.stringify(config))
        
        if (!config.circuit.signerKey || !config.gateways.every(gtwy => !!gtwy.signerKey)) {
            throw Error("InstanceManager::setup: missing signer keys")
        }

        // register keypress listener 4 ctrl+k (kill) wich would init the shutdown process:
        // print "shutting down" - don't take new executions - finish pending - then die

        await cryptoWaitReady()
        const keyring = new Keyring({ type: "sr25519" })

         this.signer = config.circuit.signerKey ? keyring.addFromMnemonic(config.circuit.signerKey) : keyring.addFromUri("//Executor//default")

        this.sdk = new Sdk(config.circuit.rpc, this.signer)

        // @ts-ignore
        this.circuitClient = await this.sdk.init()

        this.executionManager = new ExecutionManager(this.circuitClient, this.sdk, logger)
        await this.executionManager.setup(config.gateways, config.vendors)

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
