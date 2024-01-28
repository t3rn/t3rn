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
        })

        // once the headerRange has been submitted, we remove the header from respective instance by using anchorIndex
        this.relayer.on("SubmittedHeaderRange", (data: any) => {
            console.log("SubmittedHeaderRange")
        })

        this.listener.on("SubmitHeader", (data: any) => {
            console.log("received event")
            this.relayer.submitHeader(data)

        })
    }
}

async function main() {
    let exec = new InstanceManager()
    await exec.setup()
}

main()
