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
        this.circuitListener.on("XTransactionReadyForExec", async (xtxId: string) => {
            InstanceManager.debug("XTransactionReadyForExec")
            this.executionManager.xtxReady(xtxId)
        })

        this.circuitListener.on("SideEffectInsuranceReceived", async (sfxId: string) => {
            InstanceManager.debug("SideEffectInsuranceReceived")
            this.executionManager.sfxReady(sfxId)
        })

        this.circuitListener.on("NewExecution", async (execution: Execution) => {
            InstanceManager.debug("NewExecutionReceived")
            this.executionManager.addExecution(execution);
        })

        this.executionManager.on("ExecuteSideEffect", async sideEffect => {
            InstanceManager.debug("ExecuteSideEffects")
            await this.instances[sideEffect.target].executeTx(sideEffect)
        })

        this.executionManager.on(
            "ConfirmSideEffects",
            (sideEffects: SideEffect[]) => {
              InstanceManager.debug("ConfirmSideEffects")
              this.circuitRelayer.confirmSideEffects(sideEffects)
          }
        )

        this.circuitRelayer.on("SideEffectConfirmed", (id: string) => {
            InstanceManager.debug("SideEffectConfirmed")
            this.executionManager.sideEffectConfirmed(id)
        })

        this.circuitListener.on("NewHeaderRangeAvailable", data => {
            InstanceManager.debug("NewHeaderRangeAvailable:", data.height, data.gatewayId)
            this.executionManager.updateGatewayHeight(data.gatewayId, data.height)
        })
    }
}

async function main() {
  const instanceManager = new InstanceManager()
  await instanceManager.setup()
}

main()
