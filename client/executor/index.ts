import {Execution} from "./executionManager/execution";

require('dotenv').config()
import "@t3rn/types"
import {SideEffect} from "./executionManager/sideEffect";
import CircuitListener from "./circuit/listener"
import CircuitRelayer from "./circuit/relayer"
import SubstrateRelayer from "./gateways/substrate/relayer"
import { ExecutionManager } from "./executionManager"
import { GatewayDataService } from "./utils/gatewayDataService";

import createDebug from "debug"
import { ApiPromise, WsProvider } from "@polkadot/api"
import types from "./config/types.json"
import rpc from "./config/rpc.json"
import config from "./config/config";
import { T3rnPrimitivesXdnsXdnsRecord } from "@polkadot/types/lookup"
import {PriceEngine} from "./pricing";
class InstanceManager {
    static debug = createDebug("instance-manager")

    circuitClient: ApiPromise;
    priceEngine: PriceEngine;
    circuitListener: CircuitListener;
    circuitRelayer: CircuitRelayer;
    executionManager: ExecutionManager;
    instances: { [key: string]: SubstrateRelayer } = {};
    gatewayDataService: GatewayDataService;

    async setup() {
        this.circuitClient = await ApiPromise.create({
            provider: new WsProvider(config.circuit.rpc),
            types: types as any,
            rpc: rpc as any
        })
        this.gatewayDataService = new GatewayDataService(this.circuitClient, config)
        this.priceEngine = new PriceEngine();
        this.executionManager = new ExecutionManager(this.gatewayDataService, this.priceEngine);
        // @ts-ignore
        this.circuitRelayer = new CircuitRelayer(this.circuitClient, config.circuit.signer)
        this.circuitListener = new CircuitListener(this.circuitClient)

        await this.gatewayDataService.init()
        await this.circuitListener.start()

        await this.initializeGateways()
        await this.initializeEventListeners()

        InstanceManager.debug("executor setup")
    }

    async initializeGateways() {
        for (let i = 0; i < this.gatewayDataService.gateways.length; i++) {
            const entry = this.gatewayDataService.gateways[i]

            if (entry.type === "Substrate") {
                let instance = new SubstrateRelayer()
                await instance.setup(entry.rpcClient, entry.signer)

                instance.on("SideEffectExecuted", (id: string) => {
                    this.executionManager.sideEffectExecuted(id)
                })

                console.log("calling!!")

                // setup in executionManager
                this.executionManager.addGateway(entry.id, instance)
                // store relayer instance locally
                this.instances[entry.id] = instance
            }
        }
    }
    async initializeEventListeners() {
        // Insurance for all SideEffects has been posted, ready to execute
        this.circuitListener.on("XTransactionReadyForExec", async (xtxId: string) => {
            this.executionManager.xtxReady(xtxId)
        })

        // Insurance for SideEffect has been posted
       this.circuitListener.on("SideEffectInsuranceReceived",  (sfxId: string, executor: any) => {
            const iAmExecuting = this.circuitRelayer.signer.addressRaw.toString() == executor.toU8a().toString();
            this.executionManager.insuranceBonded(sfxId, iAmExecuting)
        })

        // new Execution has been received
        this.circuitListener.on("NewExecution", async (execution: Execution) => {
            this.executionManager.createExecution(execution);
        })

        //SideEffect has been confirmed on Circuit
        this.circuitListener.on("SideEffectConfirmed", (sfxId: string) => {
            this.executionManager.sideEffectConfirmed(sfxId)
        })

        // The execution is complete -> COMMIT
        this.circuitListener.on("ExecutionComplete",  (xtxId: string) => {
            this.executionManager.executionComplete(xtxId)
        })

        // New header range has been received
        this.circuitListener.on("NewHeaderRangeAvailable", data => {
            this.executionManager.updateGatewayHeight(data.gatewayId, data.height)
        })

        // trigger SideEffect confirmation on circuit
        this.executionManager.on("BondInsurance", (sideEffects: SideEffect[]) => {
              this.circuitRelayer.bondInsuranceDeposits(sideEffects)
        })

        // Execute SideEffect on Target
        this.executionManager.on("ExecuteSideEffect", async sideEffect => {
            await this.instances[sideEffect.target].executeTx(sideEffect)
        })

        // trigger SideEffect confirmation on circuit
        this.executionManager.on("ConfirmSideEffects", (sideEffects: SideEffect[]) => {
              this.circuitRelayer.confirmSideEffects(sideEffects)
        })

        // SideEffect was executed on Target
        this.circuitRelayer.on("SideEffectExecuted", (sfxId: string) => {
            this.executionManager.sideEffectExecuted(sfxId)
        })
    }

    async fetchXdnsData() {
        // @ts-ignore
        const records: T3rnPrimitivesXdnsXdnsRecord[] =  await this.circuitClient.rpc.xdns.fetchRecords();


        console.log(records['xdns_records'].toHuman())

    }
}

async function main() {
  const instanceManager = new InstanceManager()
  await instanceManager.setup()
}

main()
