import "@polkadot/api-augment"
import { Sdk } from "@t3rn/sdk"
import { Keyring } from "@polkadot/api"
require("dotenv").config()
import "@t3rn/types"
import { SubstrateRelayer, CostEstimator, Estimator, Estimate, InclusionData } from "./gateways/substrate/relayer"
import { ExecutionManager, Queue } from "./executionManager"
import { ApiPromise } from "@polkadot/api"
import config from "../config/config"
import { BiddingEngine } from "./bidding"
import { PriceEngine, CoingeckoPricing } from "./pricing"
import { StrategyEngine, SfxStrategy, XtxStrategy } from "./strategy"
import { SideEffect, Notification, NotificationType, TxOutput, TxStatus } from "./executionManager/sideEffect"
import { Execution } from "./executionManager/execution"
import { CircuitListener, ListenerEvents, ListenerEventData } from "./circuit/listener"
import { CircuitRelayer } from "./circuit/relayer"
// @ts-ignore
import { T3rnPrimitivesXdnsXdnsRecord } from "@polkadot/types/lookup"
import { cryptoWaitReady } from "@polkadot/util-crypto"
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
    relayers: { [key: string]: SubstrateRelayer } = {}
    sdk: Sdk
    signer: any

    async setup(signer: string | undefined) {
        await cryptoWaitReady()
        const keyring = new Keyring({ type: "sr25519" })
        this.signer = signer === undefined ? keyring.addFromUri("//Executor//default") : keyring.addFromMnemonic(signer)

        this.sdk = new Sdk(config.circuit.rpc, this.signer)

        // @ts-ignore
        this.circuitClient = await this.sdk.init()

        this.executionManager = new ExecutionManager(this.circuitClient, this.sdk, logger)
        await this.executionManager.setup()

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
    InclusionData,
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
}

async function main() {
    const instanceManager = new InstanceManager()
    await instanceManager.setup(undefined)
}

main()
