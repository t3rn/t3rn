import "@polkadot/api-augment"
import { Sdk } from "@t3rn/sdk"
import { Keyring } from "@polkadot/api"
require("dotenv").config()
import "@t3rn/types"
import SubstrateRelayer from "./gateways/substrate/relayer"
import { ExecutionManager } from "./executionManager/execMan"
import { ApiPromise } from "@polkadot/api"
import config from "../config/config"
import { BiddingEngine } from "./bidding"
import { PriceEngine } from "./pricing"
import { StrategyEngine } from "./strategy"
import { SideEffect } from "./executionManager/sideEffect"
import { Execution } from "./executionManager/execution"
import { CircuitListener } from "./circuit/listener"
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
    ExecutionManager,
    SubstrateRelayer,
    InstanceManager,
    BiddingEngine,
    StrategyEngine,
    PriceEngine,
    Execution,
    SideEffect,
    CircuitListener,
    CircuitRelayer,
}

async function main() {
    const instanceManager = new InstanceManager()
    await instanceManager.setup(undefined)
}

main()
