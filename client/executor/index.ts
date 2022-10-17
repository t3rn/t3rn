import {SideEffect} from "./circuit/executions/sideEffect";

require('dotenv').config()
import CircuitListener from "./circuit/listener"
import CircuitRelayer from "./circuit/relayer"
import SubstrateRelayer from "./gateways/substrate/relayer"
import CostEstimator from "./gateways/substrate/costEstimator";
import config from "./config/config.json"
import { Execution } from "./circuit/executions/execution"
import { ExecutionManager } from "./executionManager"
import createDebug from "debug"

class InstanceManager {
    static debug = createDebug("instance-manager")

    circuitListener: CircuitListener
    circuitRelayer: CircuitRelayer
    executionManager: ExecutionManager
    instances: { [key: string]: SubstrateRelayer } = {}

    constructor() {
        this.circuitListener = new CircuitListener()
        this.circuitRelayer = new CircuitRelayer()
        this.executionManager = new ExecutionManager()
    }

    async setup() {
        await this.circuitListener.setup(config.circuit.rpc)
        await this.circuitRelayer.setup(config.circuit.rpc)
        await this.circuitListener.start()
        await this.initializeGateways()
        await this.initializeEventListeners()
        InstanceManager.debug("executor setup")
    }

    async initializeGateways() {
        for (let i = 0; i < config.gateways.length; i++) {
            const entry = config.gateways[i]
            if (entry.type === "substrate") {
                let instance = new SubstrateRelayer()
                await instance.setup(entry.rpc, entry.name, entry.id)

                instance.on("SideEffectExecuted", (id: string) => {
                    this.executionManager.sideEffectExecuted(id)
                })

                // setup in executionManager
                this.executionManager.addGateway(entry.id)
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
            this.executionManager.addExecution(execution);
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
}

async function main() {
  const instanceManager = new InstanceManager()
  await instanceManager.setup()
}

main()
