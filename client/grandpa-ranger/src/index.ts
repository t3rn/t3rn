import Relayer from "./relayer"
import config from "../config.json"
import RelaychainListener from "./listeners/relaychain"
import ParachainListener from "./listeners/parachain"
import createDebug from "debug"
import { Header } from "@polkadot/types/interfaces"

class InstanceManager {
  static debug = createDebug("instance-manager")

  // handles circuit communitcation
  relayer: Relayer
  // stores relay/parachains ranger instances
  gateways: { [key: string]: any } = {}
  // used for mapping parachain instances to its respective relaychain
  relayParasMap: { [key: string]: string[] } = {}

  constructor() {
    this.relayer = new Relayer()
  }

  async setup() {
    await this.relayer.setup(config.circuit.ws)
    await this.initializeRelaychainListeners()
    // await this.initializeParachainListeners()
    await this.initializeEventListeners()
    InstanceManager.debug("Components Initialzed")
  }

  // initialize relaychain instances as defined in config.json
  async initializeRelaychainListeners() {
    for (let i = 0; i < config.relaychains.length; i++) {
      const entry = config.relaychains[i]

      let instance = new RelaychainListener()
      await instance.setup(entry.rpc, entry.id)

      // forward SubmitFinalityProof request to relayer
      instance.on("SubmitFinalityProof", (data: any) => {
        InstanceManager.debug("Received SubmitFinalityProof")
        this.relayer.submitFinalityProof(
          data.gatewayId,
          data.justification,
          data.anchorHeader,
          data.anchorIndex
        )
      })

      instance.start()

      // store relaychain instance in mapping
      this.gateways[entry.id] = instance
      this.relayParasMap[entry.id] = []
    }
  }

  // initialize parachain instances as defined in config.json
  async initializeParachainListeners() {
    for (let i = 0; i < config.parachains.length; i++) {
      const entry = config.parachains[i] as any

      if (!this.relayParasMap[entry.relaychainId]) {
        throw new Error(`Setup Failed! Relaychain of ${entry.name} not found!`)
      }

      this.relayParasMap[entry.relaychainId].push(entry.id)

      let instance = new ParachainListener()
      await instance.setup(entry.rpc, entry.id, entry.parachainId)

      // store instance in mapping
      this.gateways[entry.id] = instance
    }
  }

  async initializeEventListeners() {
    // once a relaychains finality proof has been submitted
    this.relayer.on("FinalityProofSubmitted", async (data: any) => {
      InstanceManager.debug("FinalityProofSubmitted")

      const _relaychainParams: {
        gatewayId: string
        range: Header[]
        anchorHeader: Header
        anchorIndex: number
      } = this.gateways[data.gatewayId].submitHeaderRangeParams(
        data.anchorIndex
      )
      const relaychainParams: {
        gateway: RelaychainListener
        gatewayId: string
        anchorNumber: number
      } = {
        gateway: this.gateways[_relaychainParams.gatewayId],
        gatewayId: _relaychainParams.gatewayId,
        anchorNumber: _relaychainParams.anchorHeader.number.toNumber(),
      }

      const parachainParams: {
        gateway: ParachainListener
        gatewayId: string
        anchorNumber: number
      }[] = await this.submitParachainHeadersAndGetRangeParams(data).then(
        params =>
          params.map(p => ({ ...p, gateway: this.gateways[p.gatewayId] }))
      )

      const params: {
        gateway: RelaychainListener | ParachainListener
        gatewayId: string
        anchorNumber: number
      }[] = [relaychainParams, ...parachainParams].map(p => ({
        ...p,
        gateway: this.gateways[p.gatewayId],
      }))

      await this.relayer.submitHeaderRanges(params)
    })

    // once the headerRange has been submitted, we remove the header from the respective instance
    this.relayer.on(
      "SubmittedHeaderRanges",
      (
        params: {
          gateway: ParachainListener | RelaychainListener
          gatewayId: string
          anchorNumber: number
        }[]
      ) => {
        InstanceManager.debug("SubmittedHeaderRange")
        params.forEach(p => this.gateways[p.gatewayId].finalize(p.anchorNumber))
      }
    )
  }

  // Submits storage proofs for all parachains given a finalized relaychain block.
  async submitParachainHeadersAndGetRangeParams(data: any) {
    const relaychainId = data.gatewayId
    const relaychain = this.gateways[relaychainId]
    const parachains = this.relayParasMap[relaychainId]

    const params = await Promise.all(
      parachains.map(async chainId => {
        const parachainId = this.gateways[chainId].parachainId

        let [storageProof, headerNumber] = await relaychain.getStorageProof(
          data.anchorHash,
          parachainId
        )

        return {
          gatewayId: chainId,
          blockHash: data.anchorHash,
          proof: storageProof.toJSON().proof,
          // headerNumber is the number of the parachain header we are verifying.
          // We later use it to generate a matching range.
          anchorNumber: headerNumber,
        }
      })
    )

    return await this.relayer.submitParachainHeaders(params)
  }
}

async function main() {
  let exec = new InstanceManager()
  await exec.setup()
}

main()
