import {SideEffect} from "./utils/sideEffect";

require('dotenv').config()
import CircuitListener from "./circuit/listener"
import CircuitRelayer from "./circuit/relayer"
import SubstrateRelayer from "./gateways/substrate/relayer"
import config from "./config.json"
import { Execution } from "./utils/execution"
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
                await instance.setup(entry.rpc, entry.name)

                instance.on("SideEffectExecuted", (id: string) => {
                    InstanceManager.debug("SideEffectExecuted")
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
            InstanceManager.debug("XTransactionReadyForExec")
            this.executionManager.xtxReady(xtxId)
        })

        // Insurance for SideEffect has been posted
       this.circuitListener.on("SideEffectInsuranceReceived",  (sfxId: string, executor: any) => {
            InstanceManager.debug("SideEffectInsuranceReceived")
            const iAmExecuting = this.circuitRelayer.signer.addressRaw.toString() == executor.toU8a().toString();
            this.executionManager.insuranceBonded(sfxId, iAmExecuting)
        })

        // new Execution has been received
        this.circuitListener.on("NewExecution", async (execution: Execution) => {
            InstanceManager.debug("NewExecutionReceived")
            this.executionManager.addExecution(execution);
        })

        //SideEffect has been confirmed on Circuit
        this.circuitListener.on("SideEffectConfirmed", (sfxId: string) => {
            InstanceManager.debug("SideEffectConfirmedOnCircuit")
            this.executionManager.sideEffectConfirmed(sfxId)
        })

        // The execution is complete -> COMMIT
        this.circuitListener.on("ExecutionComplete",  (xtxId: string) => {
            InstanceManager.debug("ExecutionComplete")
            this.executionManager.completeExecution(xtxId)
        })

        // New header range has been received
        this.circuitListener.on("NewHeaderRangeAvailable", data => {
            InstanceManager.debug("NewHeaderRangeAvailable:", data.height, data.gatewayId)
            this.executionManager.updateGatewayHeight(data.gatewayId, data.height)
        })

        // trigger SideEffect confirmation on circuit
        this.executionManager.on("BondInsurance", (sideEffects: SideEffect[]) => {
              InstanceManager.debug("BondInsurance")
              this.circuitRelayer.bondInsuranceDeposits(sideEffects)
        })

        // Execute SideEffect on Target
        this.executionManager.on("ExecuteSideEffect", async sideEffect => {
            InstanceManager.debug("ExecuteSideEffect")
            await this.instances[sideEffect.target].executeTx(sideEffect)
        })

        // trigger SideEffect confirmation on circuit
        this.executionManager.on("ConfirmSideEffects", (sideEffects: SideEffect[]) => {
              InstanceManager.debug("ConfirmSideEffects")
              this.circuitRelayer.confirmSideEffects(sideEffects)
        })

        // SideEffect was executed on Target
        this.circuitRelayer.on("SideEffectExecuted", (sfxId: string) => {
            InstanceManager.debug("SideEffectExecuted")
            this.executionManager.sideEffectExecuted(sfxId)
        })
    }
}

async function main() {
  const instanceManager = new InstanceManager()
  await instanceManager.setup()
}

main()
