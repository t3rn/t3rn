require('dotenv').config()
import CircuitListener from "./circuit/listener"
import CircuitRelayer from "./circuit/relayer"
import SubstrateRelayer from "./gateways/substrate/relayer"
import config from "./config.json"
import { SideEffect } from "./utils/types"
import { ExecutionManager } from "./executionManager"
import createDebug from "debug"

if (!process.env.SIGNER_KEY) {
  throw Error("missing env var SIGNER_KEY")
}

class InstanceManager {
  static debug = createDebug("instance-manager")

  circuitListener: CircuitListener
  circuitRelayer: CircuitRelayer
  executionManager: ExecutionManager

  instances: { [key: string]: SubstrateRelayer } = {}
  xtxSfxMap: { [key: string]: SideEffect[] } = {}

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
    this.circuitListener.on("XTransactionReadyForExec", async xtxId => {
      InstanceManager.debug("XTransactionReadyForExec")

      if (this.xtxSfxMap[xtxId]) {
        this.executionManager.addSideEffects(this.xtxSfxMap[xtxId])
      } else {
        InstanceManager.debug("cannot map xtx id to side effects.")
      }
    })

    this.circuitListener.on(
      "NewSideEffects",
      async (sideEffects: SideEffect[]) => {
        InstanceManager.debug("NewSideEffects")
        sideEffects.forEach(async sideEffect => {
          if (!this.xtxSfxMap[sideEffect.xtxId]) {
            this.xtxSfxMap[sideEffect.xtxId] = [sideEffect]
          } else {
            this.xtxSfxMap[sideEffect.xtxId].push(sideEffect)
          }
        })
        await this.circuitRelayer.bondInsuranceDeposits(sideEffects)
      }
    )

    this.executionManager.on("ExecuteSideEffects", async sideEffects => {
      InstanceManager.debug("ExecuteSideEffects")
      for (const sideEffect of sideEffects) {
        await this.instances[sideEffect.getTarget()].executeTx(sideEffect)
      }
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
      this.executionManager.finalize(id)
    })

    this.circuitListener.on("NewHeaderRangeAvailable", data => {
      InstanceManager.debug("NewHeaderRangeAvailable", data.gatewayId)
      this.executionManager.updateGatewayHeight(data.gatewayId, data.height)
    })
  }
}

async function main() {
  const instanceManager = new InstanceManager()
  await instanceManager.setup()
}

main()
