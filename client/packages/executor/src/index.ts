import '@polkadot/api-augment';
import { Sdk } from "@t3rn/sdk";
import { Keyring } from "@polkadot/api"
require('dotenv').config()
import "@t3rn/types"
import SubstrateRelayer from "./gateways/substrate/relayer"
import { ExecutionManager } from "./executionManager/execMan"
import { BN } from "@polkadot/util"
import createDebug from "debug"
import { ApiPromise, WsProvider } from "@polkadot/api"
import config from "./config/config";
import { BiddingEngine} from "./bidding";
import { PriceEngine} from "./pricing";
import { StrategyEngine} from "./strategy";
import { SideEffect } from "./executionManager/sideEffect";
import { Execution} from "./executionManager/execution";
import {CircuitListener} from "./circuit/listener";
import {CircuitRelayer} from "./circuit/relayer";



// @ts-ignore
import { T3rnPrimitivesXdnsXdnsRecord } from "@polkadot/types/lookup"
import { cryptoWaitReady } from '@polkadot/util-crypto';

class InstanceManager {
    static debug = createDebug("instance-manager")

    circuitClient: ApiPromise;
    executionManager: ExecutionManager;
    relayers: { [key: string]: SubstrateRelayer } = {};
    sdk: Sdk;
    signer: any;

    async setup(signer: string | undefined) {
        await cryptoWaitReady();
        const keyring = new Keyring({ type: "sr25519" })
        this.signer =
            signer === undefined
                ? keyring.addFromUri("//Executor//default")
                : keyring.addFromMnemonic(signer)

        this.sdk = new Sdk(config.circuit.rpc, this.signer)

        // @ts-ignore
        this.circuitClient = await this.sdk.init()

        this.executionManager = new ExecutionManager(this.circuitClient, this.sdk);
        await this.executionManager.setup()


        InstanceManager.debug("executor setup")
    }
}

export{ExecutionManager, SubstrateRelayer, InstanceManager, BiddingEngine, StrategyEngine, PriceEngine, Execution, SideEffect, CircuitListener, CircuitRelayer}

async function main() {
  const instanceManager = new InstanceManager()
  await instanceManager.setup(undefined)
}

main()
