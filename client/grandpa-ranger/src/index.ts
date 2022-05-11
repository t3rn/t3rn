import Relayer from "./relayer"
import config from "../config.json"
import RelaychainListener from "./listeners/relaychain"
import ParachainListener from "./listeners/parachain"
import createDebug from "debug"

class InstanceManager {
  static debug = createDebug("instance-manager")

  // handles circuit communitcation
  relayer: Relayer
  // stores relay/parachains ranger instances
  gateways: {
    [id: string]: any
  } = {}

  // used for mapping parachain instances to its respective relaychain
  relayLookup: {
    [id: string]: string[]
  } = {}

  constructor() {
    this.relayer = new Relayer()
  }

  async setup() {
    await this.relayer.setup(config.circuit.rpc)
    await this.initializeRelaychainListeners()
    await this.initializeParachainListeners()
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

      // forward SubmitHeaderRange request to relayer
      instance.on("SubmitHeaderRange", (data: any) => {
        InstanceManager.debug("Received SubmitHeaderRange")
        this.relayer.submitHeaderRange(
          data.gatewayId,
          data.range,
          data.anchorHeader,
          data.anchorIndex
        )
      })

      instance.start()

      // store relaychain instance in mapping
      this.gateways[entry.id] = instance
      this.relayLookup[entry.id] = []
    }
  }

  // initialize parachain instances as defined in config.json
  async initializeParachainListeners() {
    for (let i = 0; i < config.parachains.length; i++) {
      const entry = config.parachains[i]

      if (!this.relayLookup[entry.relaychainId]) {
        throw new Error(`Setup Failed! Relaychain of ${entry.name} not found!`)
      }

      this.relayLookup[entry.relaychainId].push(entry.id)

      let instance = new ParachainListener()
      await instance.setup(entry.rpc, entry.id, entry.parachainId)

      // forward SubmitHeaderRange request to relayer
      instance.on("SubmitHeaderRange", (data: any) => {
        InstanceManager.debug("Received SubmitHeaderRange")
        this.relayer.submitHeaderRange(
          data.gatewayId,
          data.range,
          data.anchorHeader,
          data.anchorIndex
        )
      })

      // store instance in mapping
      this.gateways[entry.id] = instance
    }
  }

  // routes relayer notification to respective function
  async initializeEventListeners() {
    // once a relaychains finality proof has been submitted
    this.relayer.on("FinalityProofSubmitted", async (data: any) => {
      InstanceManager.debug("FinalityProofSubmitted")
      // Once the relaychain has called submitFinalityProof, we can add relaychain headers
      await this.triggerParachainHeaderVerification(data)
      // the relaychain can submit a header range immediatly
      this.gateways[data.gatewayId].submitHeaderRange(data.anchorIndex)
    })

    // once the headerRange has been submitted, we remove the header from respective instance by using anchorIndex
    this.relayer.on("SubmittedHeaderRange", (data: any) => {
      InstanceManager.debug("SubmittedHeaderRange")
      this.gateways[data.gatewayId].finalize(data.anchorIndex)
    })

    // once a parachain has submitted a header, a headerRange can be passed
    this.relayer.on("ParachainHeaderSubmitted", (data: any) => {
      InstanceManager.debug("ParachainHeaderSubmitted")
      InstanceManager.debug(data)
      this.gateways[data.gatewayId].submitHeaderRange(data.anchorHash)
    })
  }

  // iterates through a relaychains parachain and submittes header
  async triggerParachainHeaderVerification(data: any) {
    // we iterate over a relaychains parachains
    let promises: Promise<any>[] = this.relayLookup[data.gatewayId].map(
      entry => {
        return new Promise(async (res, rej) => {
          //generate a storage read proof for the header we're looking to verify. We also return the decoded headerHash, as this is the anchor for the parachain once header is complete
          let [storageProof, headerNumber] = await this.gateways[
            data.gatewayId
          ].getStorageProof(data.anchorHash, this.gateways[entry].parachainId)
          this.relayer.submitParachainHeader(
            entry,
            data.anchorHash,
            storageProof.toJSON().proof,
            headerNumber //this is the number of the parachain header we are verifying. We later use it to generate a matching range
          )
          res
        })
      }
    )

    return Promise.all(promises)
    // .then(() => InstanceManager.debug("ran promises"))
  }
}

async function main() {
  let exec = new InstanceManager()
  await exec.setup()
}

main()
