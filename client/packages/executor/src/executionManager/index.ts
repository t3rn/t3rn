import {Execution} from "./execution"
import {Notification, NotificationType, SideEffect} from "./sideEffect"
import Estimator from "../gateways/substrate/estimator"
import {SubstrateRelayer} from "../gateways/substrate/relayer"
import {PriceEngine} from "../pricing"
import {StrategyEngine} from "../strategy"
import {Sdk} from "@t3rn/sdk"
import {BiddingEngine} from "../bidding"
import {CircuitListener, ListenerEventData, ListenerEvents} from "../circuit/listener"
import {ApiPromise} from "@polkadot/api"
import {CircuitRelayer} from "../circuit/relayer"
import {RelayerEventData, RelayerEvents} from "../gateways/types"
import {XtxStatus} from "@t3rn/sdk/dist/src/side-effects/types"
import {Gateway} from "../../config/config"

// A type used for storing the different SideEffects throughout their respective life-cycle.
// Please note that waitingForInsurance and readyToExecute are only used to track the progress. The actual logic is handeled in the execution
/**
 * The queue type is used to track the incoming SFXs throughout their life-cycle. Each gateway has its own queue, tracking its height on the
 * light-client. When an SFX was executed, it is moved to the isConfirming queue. Once the gateway has reached the required block height,
 * the SFXs can be confirmed.
 *
 * @group Execution Manager
 */
export type Queue = {
    /** Each gateway has its own queue, which can be accessed via gateway id */
    gateways: {
        /** Stores the latest block height know by the corresponding circuit light client */
        blockHeight: number
        /** SFXs that are currently in bidding stage */
        isBidding: string[]
        /** SFXs that are currently being executed */
        isExecuting: string[]
        /** SFXs that are waiting to be confirmed */
        isConfirming: {
            [block: number]: string[]
        }
        /** SFXs that are completed */
        completed: string[]
        /** SFXs that are dropped */
        dropped: string[]
        /** SFXs that are reverted */
        reverted: string[]
    }
}

/**
 * The ExecutionManager lies at the heart of the t3rn executor. It is responsible for managing and coordinating the execution of incoming
 * XTXs and the corresponding SFXs. It processes incoming events, triggering the creation/execution/confirmation of SFXs.
 *
 * @group Execution Manager
 */
export class ExecutionManager {
    // we map the current state in the queue
    /** Queue used to track SFXs and gateways height */
    queue: Queue = <Queue>{}
    // maps xtxId to Execution instance
    /** Maps XTX id to their corresponding Execution instance */
    xtx: {
        [id: string]: Execution
    } = {}
    /** Maps SFX id to their corresponding XTX id. Used for lookups */
    sfxToXtx: {
        [sfxId: string]: string
    } = {}

    /** Tx cost estimator instances for the specific targets */
    targetEstimator: {
        [id: string]: Estimator
    } = {}
    /** Relayer instances for the specific targets */
    relayers: { [key: string]: SubstrateRelayer } = {}

    priceEngine: PriceEngine
    strategyEngine: StrategyEngine
    biddingEngine: BiddingEngine
    sdk: Sdk
    circuitClient: ApiPromise
    circuitListener: CircuitListener
    circuitRelayer: CircuitRelayer
    signer: any
    logger: any

    constructor(circuitClient: ApiPromise, sdk: Sdk, logger: any) {
        this.priceEngine = new PriceEngine()
        this.strategyEngine = new StrategyEngine()
        this.biddingEngine = new BiddingEngine(logger)
        this.circuitClient = circuitClient
        this.circuitListener = new CircuitListener(this.circuitClient)
        this.circuitRelayer = new CircuitRelayer(sdk)
        this.sdk = sdk
        this.logger = logger
    }

    /** Setup all instances and listeners for the execution manager */
    async setup(gatewayConfig: Gateway[], vendors: string[]) {
        this.initializeVendors(vendors)
        await this.initializeGateways(gatewayConfig)
        await this.circuitListener.start()
        await this.initializeEventListeners()
        this.addLog({ msg: "Setup Successful" })
    }

    /** Initiates the shutdown sequence. */
    async shutdown() {
        const self = this
        await this.circuitListener.stop()
        return new Promise((resolve) => {
            function recheckQueue() {
                const done = Object.entries(self.sdk.gateways)
                    .map(([_, gtwy]) => gtwy.id)
                    .every(
                        (gtwyId) =>
                            self.queue[gtwyId].isBidding.length === 0 &&
                            self.queue[gtwyId].isExecuting.length === 0 &&
                            self.queue[gtwyId].isConfirming.length === 0
                    )
                if (done) {
                    resolve(undefined)
                } else {
                    self.circuitListener.once("Event", recheckQueue)
                }
            }
            this.circuitListener.once("Event", recheckQueue)
            recheckQueue()
        })
    }

    initializeVendors(vendors: string[]) {
        for(let i = 0; i < vendors.length; i++) {
            this.queue[vendors[i]] = {
                blockHeight: 0,
                isBidding: [],
                isExecuting: [],
                isConfirming: {},
                completed: [],
                dropped: [],
                reverted: [],
            }
        }
    }

    /** Initialize all gateways and their corresponding relayers, event listeners and estimators */
    async initializeGateways(gatewayConfig: Gateway[]) {
        const gatewayKeys = Object.keys(this.sdk.gateways)
        for (let i = 0; i < gatewayKeys.length; i++) {
            const entry = this.sdk.gateways[gatewayKeys[i]]

            const config = gatewayConfig.find((g) => g.id === entry.id)

            if(!config) { // skip over gateways we have no configs for
                continue;
            }
            if (entry.executionVendor === "Substrate") {
                // initialize gateway relayer
                const relayer = new SubstrateRelayer()


                await relayer.setup(config, this.logger)

                this.relayers[entry.id] = relayer

                // initialize gateway estimator
                this.targetEstimator[entry.id] = new Estimator(relayer)

                relayer.on("Event", async (eventData: RelayerEventData) => {
                    switch (eventData.type) {
                        case RelayerEvents.SfxExecutedOnTarget:
                            // @ts-ignore
                            const vendor = this.xtx[this.sfxToXtx[eventData.sfxId]].sideEffects.get(eventData.sfxId).vendor
                            this.removeFromQueue("isExecuting", eventData.sfxId, vendor)

                            // create array if first for block
                            if (!this.queue[vendor].isConfirming[eventData.blockNumber]) {
                                this.queue[vendor].isConfirming[eventData.blockNumber] = []
                            }
                            // adds to queue
                            this.queue[vendor].isConfirming[eventData.blockNumber].push(eventData.sfxId)

                            this.addLog({
                                msg: "moved sfx from isExecuting to isConfirming",
                                sfxId: eventData.sfxId,
                                xtxId: this.sfxToXtx[eventData.sfxId],
                            })

                            break
                        case RelayerEvents.HeaderInclusionProofRequest:
                            const proof = await this.relayers[eventData.target].generateHeaderInclusionProof(
                                eventData.blockNumber,
                                parseInt(eventData.data)
                            )

                            const blockHash = await this.relayers[eventData.target].getBlockHash(eventData.blockNumber)

                            this.xtx[this.sfxToXtx[eventData.sfxId]].sideEffects.get(eventData.sfxId)?.addHeaderProof(proof.toJSON().proof, blockHash)
                            break;
                        case RelayerEvents.SfxExecutionError:
                            // ToDo figure out how to handle this
                            break
                    }
                })
            }
        }
        this.addLog({ msg: "Gateways Initialized" })
    }

    /** Initialize the circuit listeners */
    async initializeEventListeners() {
        this.circuitListener.on("Event", async (eventData: ListenerEventData) => {
            switch (eventData.type) {
                case ListenerEvents.NewSideEffectsAvailable:
                    this.addXtx(eventData.data, this.sdk)
                    break
                case ListenerEvents.SFXNewBidReceived:
                    this.addBid(eventData.data)
                    break
                case ListenerEvents.XTransactionReadyForExec:
                    this.xtxReadyForExec(eventData.data[0].toString())
                    break
                case ListenerEvents.HeaderSubmitted:
                    this.updateGatewayHeight(eventData.data.vendor, eventData.data.height)
                    break
                case ListenerEvents.SideEffectConfirmed:
                    const sfxId = eventData.data[0].toString()
                    this.xtx[this.sfxToXtx[sfxId]].sideEffects.get(sfxId)!.confirmedOnCircuit()
                    this.addLog({
                        msg: "Sfx confirmed",
                        sfxId: sfxId,
                        xtxId: this.sfxToXtx[sfxId],
                    })
                    break
                case ListenerEvents.XtxCompleted:
                    this.xtx[eventData.data[0].toString()].completed()
                    break
                case ListenerEvents.DroppedAtBidding:
                    this.droppedAtBidding(eventData.data[0].toString())
                    break
                case ListenerEvents.RevertTimedOut:
                    this.revertTimeout(eventData.data[0].toString())
            }
        })
    }

    /**
     * Add a new XTX to the execution manager. This is triggered when a new XTX is available on the circuit.
     *
     * @param xtxData The SCALE encoded XTX data, as emitted by the circuit
     * @param sdk The SDK instance
     */
    async addXtx(xtxData: any, sdk: Sdk) {
        // create the XTX object
        const xtx = new Execution(xtxData, sdk, this.strategyEngine, this.biddingEngine, sdk.signer.address, this.logger)

        // Run the XTX strategy checks
        try {
            this.strategyEngine.evaluateXtx(xtx)
            this.addLog({ msg: "XTX strategy passed!", xtxId: xtx.id })
        } catch (e) {
            // XTX does not meet strategy requirements
            this.addLog({
                msg: "XTX strategy reject! " + e.toString(),
                xtxId: xtx.id,
            })
            return
        }
        this.logger.info(`Received XTX ${xtx.humanId} 🌱`) // XTX is valid for execution

        // add XTX and init required event listeners
        this.xtx[xtx.id] = xtx
        for (const [sfxId, sfx] of xtx.sideEffects.entries()) {
            this.initSfxListeners(sfx)
            this.sfxToXtx[sfxId] = xtx.id
            this.queue[sfx.vendor].isBidding.push(sfxId)
            await this.addRiskRewardParameters(sfx)
        }
        this.addLog({ msg: "XTX initialized", xtxId: xtx.id })
    }

    /**
     * Add an incoming bid to the corresponding SFX
     *
     * @param bidData SCALE encoded bid data, as emitted by the circuit
     */
    addBid(bidData: any) {
        const sfxId = bidData[0].toString()
        const bidder = bidData[1].toString()
        const amt = bidData[2].toNumber()

        const conversionId = this.sfxToXtx[sfxId]
        const sfxFromXtx = this.xtx[conversionId].sideEffects
        const actualSfx = sfxFromXtx.get(sfxId)
        if (actualSfx !== undefined) {
            actualSfx.processBid(bidder, amt)
        } else {
            throw new Error(`Could not find SFX with id ${sfxId}`)
        }
    }

    /**
     * Update a XTX status to ready. This is triggered by an incoming circuit event Next, this will trigger the execution of SFX the
     * executor has won the bid on.
     *
     * @param xtxId The XTX ID
     */
    async xtxReadyForExec(xtxId: string) {
        if (this.xtx[xtxId]) {
            this.xtx[xtxId].readyToExecute()
            // Get the SFX that the executor has won the bid on and can execute now
            const ready = this.xtx[xtxId].getReadyToExecute()
            if (ready.length > 0) {
                this.logger.info(`Won bids for XTX ${this.xtx[xtxId].humanId}: ${ready.map((sfx) => sfx.humanId)} 🏆`)
            }
            for (const sfx of ready) {
                // move on the queue
                this.removeFromQueue("isBidding", sfx.id, sfx.vendor)
                this.queue[sfx.vendor].isExecuting.push(sfx.id)
                // execute
                this.relayers[sfx.target].executeTx(sfx).then()
            }
        }
    }

    // New header range was submitted on circuit
    // update local params and check which SideEffects can be confirmed
    /**
     * Update the vendor height in the queue. This is triggered by an incoming circuit event. Next, this will trigger the confirmation of
     * SFX that have been executed.
     *
     * @param vendor Id of the gateway
     * @param blockHeight The latest block height
     */
    updateGatewayHeight(vendor: string, blockHeight: number) {
        this.addLog({
            msg: "Update Gateway Height",
            vendor: vendor,
            blockHeight: blockHeight,
        })
        this.logger.info(`Gateway height updated: ${vendor} #${blockHeight} 🧱`)
        if (this.queue[vendor]) {
            this.queue[vendor].blockHeight = blockHeight
            this.executeConfirmationQueue(vendor)
        }
    }

    // checks which executed SideEffects can be confirmed on circuit
    /**
     * Trigger the confirmation of SFX that have been executed. When the gateway height is updated, this will check the isConfirming queue
     * for the gateway. The confirmation of any waiting SFXs is now triggered. Requirement for the confirmation is that the circuit has
     * received the corresponding headers, the SFXs where included in.
     *
     * @param gatewayId Of the updated gateway
     */
    async executeConfirmationQueue(vendor: string) {
        // contains the sfxIds of SideEffects that could be confirmed based on the current blockHeight of the light client
        let readyByHeight: string[] = []
        // stores the block height of the SideEffects that are ready to be confirmed. Needed for clearing the queue
        let batchBlocks: string[] = []
        const queuedBlocks = Object.keys(this.queue[vendor].isConfirming)
        // we check which headers are available and collect the SFX ids
        for (let i = 0; i < queuedBlocks.length; i++) {
            if (parseInt(queuedBlocks[i]) <= this.queue[vendor].blockHeight) {
                batchBlocks.push(queuedBlocks[i])
                readyByHeight = readyByHeight.concat(this.queue[vendor].isConfirming[queuedBlocks[i]])
            }
        }

        const readyByStep: SideEffect[] = []

        // In case we have executed SFXs from the next phase already, we ensure that we only confirm the SFXs of the current phase
        for (let i = 0; i < readyByHeight.length; i++) {
            const sfx: SideEffect = this.xtx[this.sfxToXtx[readyByHeight[i]]].sideEffects.get(readyByHeight[i])!
            if (sfx.phase === this.xtx[sfx.xtxId].currentPhase) {
                readyByStep.push(sfx)
            }
        }

        // if we found SFXs, we confirm them
        if (readyByStep.length > 0) {
            this.addLog({
                msg: "Execute confirmation queue",
                gatewayId: vendor,
                sfxIds: readyByStep.map((sfx) => sfx.id),
            })
            this.circuitRelayer
                .confirmSideEffects(readyByStep)
                .then((res: any) => {
                    // remove from queue and update status
                    this.logger.info(`Confirmed SFXs: ${readyByStep.map((sfx) => sfx.humanId)} 📜`)
                    this.processConfirmationBatch(readyByStep, batchBlocks, vendor)
                    this.addLog({
                        msg: "Confirmation batch successful",
                        vendor: vendor,
                    })
                })
                .catch((err: any) => {
                    this.addLog(
                        {
                            msg: "Error confirming side effects",
                            vendor: vendor,
                            sfxIds: readyByStep.map((sfx) => sfx.id),
                            error: err,
                        },
                        false
                    )
                })
        }
    }

    // updates queue and sfx state after confirmation batch was submitted. This always has the same target
    /**
     * Update the queue and SFX state after a confirmation batch was submitted.
     *
     * @param sfxs Array of sfx objects
     * @param batchBlocks Array of block heights of the sfxs where confirmed for. Needed for cleaning up the queue
     * @param gatewayId Id of the gateway
     */
    processConfirmationBatch(sfxs: SideEffect[], batchBlocks: string[], gatewayId: string) {
        // remove from queue
        batchBlocks.forEach((block) => {
            delete this.queue[gatewayId].isConfirming[block]
        })

        // add to completed queue and update status
        for (const sfx of sfxs) {
            this.queue[gatewayId].completed.push(sfx.id)
            sfx.confirmedOnCircuit() // maybe we leave this part and trigger via event, which is done in any case
        }
    }

    /**
     * Initialize SFX event listeners.
     *
     * @param sfx Object of the sfx
     */
    initSfxListeners(sfx: SideEffect) {
        sfx.on("Notification", (notification: Notification) => {
            switch (notification.type) {
                case NotificationType.SubmitBid: {
                    this.circuitRelayer
                        .bidSfx(notification.payload.sfxId, notification.payload.bidAmount)
                        .then(() => {
                            sfx.bidAccepted(notification.payload.bidAmount)
                        })
                        .catch((e) => {
                            this.logger.info(`Bid rejected for SFX ${sfx.humanId} ❌`)
                            sfx.bidRejected(e)
                        })
                }
            }
        })
    }

    /**
     * Gather and add the required risk/reward parameters for a new SFX.
     *
     * @param sfx The sfx object
     */
    async addRiskRewardParameters(sfx: SideEffect) {
        // get txCost on target
        const txCostSubject = await this.targetEstimator[sfx.target].getNativeTxCostSubject(sfx)
        // get price of native token on target
        const nativeAssetPriceSubject = this.priceEngine.getAssetPrice(sfx.gateway.ticker)

        const txOutput = sfx.getTxOutputs()
        // get tx output cost. E.g. tran 1 Eth this returns the current price of Eth
        const txOutputPriceSubject = this.priceEngine.getAssetPrice(txOutput.asset)
        // get price of the reward asset
        const rewardAssetPriceSubject = this.priceEngine.getAssetPrice("TRN")

        sfx.setRiskRewardParameters(txCostSubject, nativeAssetPriceSubject, txOutputPriceSubject, rewardAssetPriceSubject)
    }

    /**
     * Update XTX status after it was dropped on circuit. Cleans up queue and updates the SFXs
     *
     * @param xtxId Id of XTX that was dropped
     */
    droppedAtBidding(xtxId: string) {
        const xtx = this.xtx[xtxId]
        if (xtx && !(xtx.status === XtxStatus.DroppedAtBidding)) {
            xtx.droppedAtBidding()
            for (const sfx of xtx.sideEffects.values()) {
                this.removeFromQueue("isBidding", sfx.id, sfx.vendor)
                this.queue[sfx.vendor].dropped.push(sfx.id)
            }
        }
    }

    /**
     * Update XTX status after it was reverted on circuit. Cleans up queue and updates the SFXs
     *
     * @param xtxId Id of XTX that was reverted
     */
    revertTimeout(xtxId: string) {
        const xtx = this.xtx[xtxId]
        if (xtx) {
            for (const sfx of xtx.sideEffects.values()) {
                // sfx could either be in isExecuting or isConfirming
                this.removeFromQueue("isExecuting", sfx.id, sfx.vendor)
                let confirmBatch = this.queue[sfx.vendor].isConfirming[sfx.targetInclusionHeight.toString()]
                if (!confirmBatch) confirmBatch = []
                if (confirmBatch.includes(sfx.id)) {
                    const index = confirmBatch.indexOf(sfx.id)
                    confirmBatch.splice(index, 1)
                }

                // add to reverted queue
                this.queue[sfx.vendor].reverted.push(sfx.id)
            }
            this.xtx[xtxId].revertTimeout()
        }
    }

    // removes sfx from queue
    private removeFromQueue(queue: string, id: string, gatewayId: string) {
        const index = this.queue[gatewayId][queue].indexOf(id)
        if (index > -1) {
            this.queue[gatewayId][queue].splice(index, 1)
        }
        this.biddingEngine.cleanUp(id)
    }

    private addLog(msg: object, debug: boolean = true) {
        msg["component"] = "ExecutionManager"

        if (debug) {
            this.logger.debug(msg)
        } else {
            this.logger.error(msg)
        }
    }
}
