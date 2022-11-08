import {Execution} from "./executionManager/execution";
import { Sdk, Types } from "@t3rn/sdk";
import { Keyring } from "@polkadot/api"
require('dotenv').config()
import "@t3rn/types"
import {SideEffect} from "./executionManager/sideEffect";
import {CircuitListener, EventData, Events} from "./circuit/listener"
import CircuitRelayer from "./circuit/relayer"
import SubstrateRelayer from "./gateways/substrate/relayer"
import { ExecutionManager } from "./executionManager/execMan"
import { GatewayDataService } from "./utils/gatewayDataService";

import createDebug from "debug"
import { ApiPromise, WsProvider } from "@polkadot/api"
import config from "./config/config";

// @ts-ignore
import { T3rnPrimitivesXdnsXdnsRecord } from "@polkadot/types/lookup"
import {PriceEngine} from "./pricing";
import { cryptoWaitReady } from '@polkadot/util-crypto';
import { ExecutionLayerType } from "@t3rn/sdk/dist/src/gateways/types";
import Estimator from "./gateways/substrate/estimator";

class InstanceManager {
    static debug = createDebug("instance-manager")

    circuitClient: ApiPromise;
    priceEngine: PriceEngine;
    circuitListener: CircuitListener;
    circuitRelayer: CircuitRelayer;
    executionManager: ExecutionManager;
    relayers: { [key: string]: SubstrateRelayer } = {};
    gatewayDataService: GatewayDataService;
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
        // this.priceEngine = new PriceEngine();
        this.executionManager = new ExecutionManager();
        // @ts-ignore
        // this.circuitRelayer = new CircuitRelayer(this.circuitClient, config.circuit.signer)
        this.circuitListener = new CircuitListener(this.circuitClient)

        // await this.gatewayDataService.init()
        await this.circuitListener.start()

        await this.initializeGateways()
        await this.initializeEventListeners()

        InstanceManager.debug("executor setup")
    }

    async initializeGateways() {
        const gatewayKeys = Object.keys(this.sdk.gateways);
        for (let i = 0; i < gatewayKeys.length; i++) {
            const entry = this.sdk.gateways[gatewayKeys[i]]

            if (entry.executionLayerType === ExecutionLayerType.Substrate) {
                let relayer = new SubstrateRelayer()
                await relayer.setup(entry.rpc, undefined)

                const estimator = new Estimator(relayer)

                // setup in executionManager
                this.executionManager.addGateway(entry.id, estimator)
                // store relayer instance locally
                this.relayers[entry.id] = relayer
            }
        }
    }
    async initializeEventListeners() {
        this.circuitListener.on("Event", async (eventData: EventData) => {
            switch (eventData.type) {
                case Events.NewSideEffectsAvailable:
                    console.log("NewSideEffectsAvailable")
                    const xtx = new Execution(eventData.data, this.sdk)
                    this.executionManager.addXtx(xtx)
            }

        })
       //  // Insurance for all SideEffects has been posted, ready to execute
       //  this.circuitListener.on("XTransactionReadyForExec", async (xtxId: string) => {
       //      this.executionManager.xtxReady(xtxId)
       //  })
       //
       //  // Insurance for SideEffect has been posted
       // this.circuitListener.on("SideEffectInsuranceReceived",  (sfxId: string, executor: any) => {
       //      const iAmExecuting = this.circuitRelayer.signer.addressRaw.toString() == executor.toU8a().toString();
       //      this.executionManager.insuranceBonded(sfxId, iAmExecuting)
       //  })
       //
       //  // new Execution has been received
       //  this.circuitListener.on("NewExecution", async (execution: Execution) => {
       //      this.executionManager.createExecution(execution);
       //  })
       //
       //  //SideEffect has been confirmed on Circuit
       //  this.circuitListener.on("SideEffectConfirmed", (sfxId: string) => {
       //      this.executionManager.sideEffectConfirmed(sfxId)
       //  })
       //
       //  // The execution is complete -> COMMIT
       //  this.circuitListener.on("ExecutionComplete",  (xtxId: string) => {
       //      this.executionManager.executionComplete(xtxId)
       //  })
       //
       //  // New header range has been received
       //  this.circuitListener.on("NewHeaderRangeAvailable", data => {
       //      this.executionManager.updateGatewayHeight(data.gatewayId, data.height)
       //  })
       //
       //  // trigger SideEffect confirmation on circuit
       //  this.executionManager.on("BondInsurance", (sideEffects: SideEffect[]) => {
       //        this.circuitRelayer.bondInsuranceDeposits(sideEffects)
       //  })
       //
       //  // Execute SideEffect on Target
       //  this.executionManager.on("ExecuteSideEffect", async sideEffect => {
       //      await this.relayers[sideEffect.target].executeTx(sideEffect)
       //  })
       //
       //  // trigger SideEffect confirmation on circuit
       //  this.executionManager.on("ConfirmSideEffects", (sideEffects: SideEffect[]) => {
       //        this.circuitRelayer.confirmSideEffects(sideEffects)
       //  })
       //
       //  // SideEffect was executed on Target
       //  this.circuitRelayer.on("SideEffectExecuted", (sfxId: string) => {
       //      this.executionManager.sideEffectExecuted(sfxId)
       //  })
    }

}

async function main() {
  const instanceManager = new InstanceManager()
  await instanceManager.setup(undefined)
}

main()
