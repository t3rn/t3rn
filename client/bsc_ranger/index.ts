import Relayer from "./relayer"
import { config } from "./config"
// import RelaychainListener from "./listeners/relaychain"
// import ParachainListener from "./listeners/parachain"
import BSCListener from "./listener"
import createDebug from "debug"

class InstanceManager {
    static debug = createDebug("instance-manager")

    // handles circuit communitcation
    relayer: Relayer
    // stores relay/parachains ranger instances
    listener: BSCListener
    constructor() {
        this.relayer = new Relayer()
        this.listener = new BSCListener()
    }

    async setup() {
        await this.relayer.setup(config.circuitRpc)
        this.listener.setup()
        this.listener.initListener()
        this.initializeEventListeners()
        InstanceManager.debug("Components Initialzed")
    }

    
    // routes relayer notification to respective function
    async initializeEventListeners() {
        // once a relaychains finality proof has been submitted
        this.relayer.on("HeaderSubmitted", async (data: any) => {
            console.log("HeaderSubmitted")
            const rangeData = this.listener.getHeaderRange(data.headerIndex)
            this.relayer.submitHeaderRange(rangeData.range, rangeData.anchorHeader)
            // // Once the relaychain has called submitFinalityProof, we can add relaychain headers
            // await this.triggerParachainHeaderVerification(data)
            // // the relaychain can submit a header range immediatly
            // this.gateways[data.gatewayId].submitHeaderRange(data.anchorIndex)
        })

        // once the headerRange has been submitted, we remove the header from respective instance by using anchorIndex
        this.relayer.on("SubmittedHeaderRange", (data: any) => {
            console.log("SubmittedHeaderRange")
          
            // this.gateways[data.gatewayId].finalize(data.anchorIndex)
        })

        this.listener.on("SubmitHeader", (data: any) => {
            console.log("received event")
            this.relayer.submitHeader(data)

        })
    }

    // // iterates through a relaychains parachain and submittes header
    // async triggerParachainHeaderVerification(data: any) {
    //     // we iterate over a relaychains parachains
    //     let promises: Promise<any>[] = this.relayLookup[data.gatewayId].map(
    //         entry => {
    //             return new Promise(async (res, rej) => {
    //                 //generate a storage read proof for the header we're looking to verify. We also return the decoded headerHash, as this is the anchor for the parachain once header is complete
    //                 let [storageProof, headerNumber] = await this.gateways[
    //                     data.gatewayId
    //                 ].getStorageProof(data.anchorHash, this.gateways[entry].parachainId)
    //                 this.relayer.submitParachainHeader(
    //                     entry,
    //                     data.anchorHash,
    //                     storageProof.toJSON().proof,
    //                     headerNumber //this is the number of the parachain header we are verifying. We later use it to generate a matching range
    //                 )
    //                 res
    //             })
    //         }
    //     )

    //     return Promise.all(promises)
    //     // .then(() => InstanceManager.debug("ran promises"))
    // }
}

async function main() {
    let exec = new InstanceManager()
    await exec.setup()
}

main()
