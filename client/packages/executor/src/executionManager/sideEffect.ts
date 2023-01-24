import "@t3rn/types"
// @ts-ignore
import { T3rnTypesSideEffect } from "@polkadot/types/lookup"
import { TextDecoder } from "util"
import { SecurityLevel, SfxStatus, SfxType } from "@t3rn/sdk/dist/src/side-effects/types"
import { Sdk } from "@t3rn/sdk"
import { BehaviorSubject } from "rxjs"
import { Gateway } from "@t3rn/sdk/dist/src/gateways"
import { StrategyEngine } from "../strategy"
import { BiddingEngine } from "../bidding"
import { EventEmitter } from "events"
import { floatToBn, toFloat } from "@t3rn/sdk/dist/src/circuit"
import { bnToFloat } from "@t3rn/sdk/dist/src/converters/amounts"
import { InclusionData } from "../gateways/types"

/** Map event names to SfxType enum */
export const EventMapper = ["Transfer", "MultiTransfer"]

/**
 * Type used for representing a Txoutput
 *
 * @typedef {Object} TxOutput
 * @group Execution Manager
 */
export type TxOutput = {
    /** Output amount as integer */
    amount: BigInt
    /** Output amount in human-readable float */
    amountHuman: number
    /** Output asset tickker */
    asset: string
}

/**
 * Event notifications
 *
 * @group Execution Manager
 */
export enum NotificationType {
    SubmitBid,
}

/**
 * Status for executing a side effect
 *
 * @group Execution Manager
 */
export enum TxStatus {
    /** The execution is currently pending, meaning another instance is in progress */
    Pending,
    /** The execution ready to be submitted */
    Ready,
}

/**
 * Event notification type
 *
 * @group Execution Manager
 */
export type Notification = {
    type: NotificationType
    payload: any
}

/**
 * Class used for tracking the state of a side effect. It contains all needed data and helper functions to go through the lifecycle of a XTX.
 *
 * @group Execution Manager
 */
export class SideEffect extends EventEmitter {
    /** The phase in which the SFX is part of. First Escrow, then Optimistic */
    phase: number
    /** SFX status, always starting with Bidding on creation */
    status: SfxStatus = SfxStatus.InBidding
    /** SFX action, e.g. tran, swap, etc */
    action: SfxType
    /** Acts as mutex lock to prevent parallel txs on the same SFX */
    txStatus: TxStatus = TxStatus.Ready
    /** Target gateway of the SFX */
    target: string
    /** Gateway helper instance */
    gateway: Gateway
    /** Security Level, e.g. Escrow or Optimistic */
    securityLevel: SecurityLevel
    /** Executor address on circuit */
    circuitSignerAddress: string
    /** Is currently the winning bidder of the SFX */
    isBidder: boolean = false
    /** The minimum profit in USD required for executing this SFX. Number is computed by strategy engine */
    minProfitUsd: number = 0

    /** If the executor leading the bid changes, store the change */
    changedBidLeader: boolean = false
    /** Value of the last bid */
    lastBids: number[]
    /** Collection of xtx (ids) that have bid on certain sfx (ids) */
    // xtxsBiddingHere: [string, number[]][]
    xtxBidHistory: Map<string, number[]>

    // SideEffect data
    id: string
    humanId: string
    xtxId: string
    /** Encoded arguments, containing the description of what should be executed */
    arguments: string[]
    /** Insurance required for executing this SFX. Amount is deposited throughout the XTX lifecycle and refunded on confirmation */
    insurance: number
    /** The current reward paid by the user for executing this SFX. This amount can reduce through executor bidding */
    reward: BehaviorSubject<number>
    /** The raw SideEffect, encoded in SCALE */
    raw: T3rnTypesSideEffect

    // TargetConfirmation
    /** Required data for confirming the inclusion on circuit. contains encoded payload, inclusionProof, and blockHash */
    inclusionData: any
    /** The block number in which the SFX was included on the target */
    targetInclusionHeight: number = 0
    /** The address of this SFXs executor */
    executor: string

    // Risk/Reward Parameters:
    /** Tx cost in the native currency of the target */
    txCostNative: BehaviorSubject<number>
    /** Cost of targets native asset in USD. Used for tx cost calculation */
    nativeAssetPrice: BehaviorSubject<number>
    /** Current price of the assets that are used for the sfx execution */
    txOutputAssetPrice: BehaviorSubject<number>
    /** Current max profit in USD that can be made when executing */
    maxProfitUsd: BehaviorSubject<number> = new BehaviorSubject<number>(0)
    /** Price for reward assert in USD */
    rewardAssetPrice: BehaviorSubject<number> = new BehaviorSubject<number>(0)

    subscriptions: any[] = []
    /** Tx cost in USD */
    txCostUsd: number = 0
    /** Cost of output assets in USD */
    txOutputCostUsd: number = 0
    /** Total reward value in USD. This is not profit, as it includes the payment for the assets that are being spent/output by the executor */
    rewardUsd: number = 0

    /** Tx receipt of the execution on target */
    txReceipt: any // store tx receipt
    logger: any

    strategyEngine: StrategyEngine
    biddingEngine: BiddingEngine

    /**
     * @param sideEffect Scale encoded side effect
     * @param id Id of the SFX
     * @param xtxId Id of the SFXs XTX
     * @param sdk Instance of @t3rn/sdk
     * @param strategyEngine Instance of the strategy engine
     * @param biddingEngine Instance of the bidding engine
     * @param circuitSignerAddress Address of the executor account used for transaction on circuit
     * @param logger The logger instance
     * @returns SideEffect instance
     */
    constructor(
        sideEffect: T3rnTypesSideEffect,
        id: string,
        xtxId: string,
        sdk: Sdk,
        strategyEngine: StrategyEngine,
        biddingEngine: BiddingEngine,
        circuitSignerAddress: string,
        logger: any
    ) {
        super()
        if (this.decodeAction(sideEffect.encodedAction)) {
            this.raw = sideEffect
            this.id = id
            this.humanId = id.substring(0, 8)
            this.xtxId = xtxId
            this.arguments = sideEffect.encodedArgs.map((entry) => entry.toString())
            this.target = new TextDecoder().decode(sideEffect.target.toU8a())
            this.gateway = sdk.gateways[this.target]
            this.securityLevel = this.evalSecurityLevel(this.gateway.gatewayType)
            this.reward = new BehaviorSubject(sdk.circuit.toFloat(sideEffect.maxReward)) // this is always in TRN (native asset)
            this.insurance = sdk.circuit.toFloat(sideEffect.insurance) // this is always in TRN (native asset)
            this.strategyEngine = strategyEngine
            this.biddingEngine = biddingEngine
            this.circuitSignerAddress = circuitSignerAddress
            this.logger = logger
        }
    }

    /**
     * Evaluates the security level of the SFX based on the target gateway type.
     *
     * @param gatewayType
     * @returns SecurityLevel
     */
    //ToDo fix parameter type
    evalSecurityLevel(gatewayType: any): SecurityLevel {
        if (gatewayType.ProgrammableExternal === "0" || gatewayType.OnCircuit === "0") {
            return SecurityLevel.Escrow
        } else {
            return SecurityLevel.Optimistic
        }
    }

    /**
     * Set the correct phase index
     *
     * @param phase The index of the SFXs phase
     */
    setPhase(phase: number) {
        this.phase = phase
    }

    /**
     * Adds the required risk parameter subjects to the SFX instance. These values are used to determine if the SFX is profitable to
     * execute. The values are added as subjects to allow for dynamic updates, triggering the re-evaluation of the SFXs profitability.
     *
     * @param txCostNative Tx cost in the native currency of the target
     * @param nativeAssetPrice Cost of targets native asset in USD. Used for tx cost calculation
     * @param txOutputAssetPrice Current price of the assets that are used for the sfx execution
     * @param rewardAssetPrice Price for reward assert in USD
     */
    setRiskRewardParameters(
        txCostNative: BehaviorSubject<number>,
        nativeAssetPrice: BehaviorSubject<number>,
        txOutputAssetPrice: BehaviorSubject<number>,
        rewardAssetPrice: BehaviorSubject<number>
    ) {
        this.txCostNative = txCostNative
        this.nativeAssetPrice = nativeAssetPrice
        this.txOutputAssetPrice = txOutputAssetPrice
        this.rewardAssetPrice = rewardAssetPrice

        this.addLog({
            msg: "Set risk parameters and subscriptions",
            txCostNative: txCostNative.getValue(),
            nativeAssetPrice: nativeAssetPrice.getValue(),
            txOutputAssetPrice: txOutputAssetPrice.getValue(),
            rewardAssetPrice: rewardAssetPrice.getValue(),
        })

        const txCostNativeSubscription = this.txCostNative.subscribe(() => {
            this.recomputeMaxProfit()
        })

        this.subscriptions.push(txCostNativeSubscription)

        const nativeAssetPriceSubscription = this.nativeAssetPrice.subscribe(() => {
            this.recomputeMaxProfit()
        })

        this.subscriptions.push(nativeAssetPriceSubscription)

        const txOutputAssetPriceSubscription = this.txOutputAssetPrice.subscribe(() => {
            this.recomputeMaxProfit()
        })

        this.subscriptions.push(txOutputAssetPriceSubscription)

        const rewardAssetPriceSubscription = this.rewardAssetPrice.subscribe(() => {
            this.recomputeMaxProfit()
        })

        this.subscriptions.push(rewardAssetPriceSubscription)

        const rewardSubscription = this.reward.subscribe(() => {
            this.recomputeMaxProfit()
        })

        this.subscriptions.push(rewardSubscription)

        this.recomputeMaxProfit()
    }

    /**
     * Computes the potential profit of the SFX based on the current risk/reward parameters. This function is primarily used to react to
     * changes in the risk/reward parameters and reevalute the bidding decision. If a new maxProfit has been computed, the bidding engine is
     * used to determine if another bid should be placed.
     */
    recomputeMaxProfit() {
        const txCostUsd = this.gateway.toFloat(this.txCostNative.getValue()) * this.nativeAssetPrice.getValue()
        this.txCostUsd = txCostUsd
        const txOutputCostUsd = this.txOutputAssetPrice.getValue() * this.getTxOutputs().amountHuman
        this.txOutputCostUsd = txOutputCostUsd
        const rewardValueUsd = this.rewardAssetPrice.getValue() * this.reward.getValue()
        this.rewardUsd = rewardValueUsd
        const maxProfitUsd = rewardValueUsd - txCostUsd - txOutputCostUsd
        if (maxProfitUsd !== this.maxProfitUsd.getValue()) {
            this.maxProfitUsd.next(maxProfitUsd)
            this.triggerBid()
        }
    }

    /** Triggers the bidding engine to place a new bid for the SFX. */
    triggerBid() {
        const result = this.generateBid()
        if (result?.trigger) {
            this.addLog({
                msg: "Bid generated",
                bid: result?.bidAmount.toString(),
            })
            this.logger.info(`Bidding on SFX ${this.humanId}: ${bnToFloat(result.bidAmount, 12)} TRN 🎰`)

            this.emit("Notification", {
                type: NotificationType.SubmitBid,
                payload: {
                    sfxId: this.id,
                    bidAmount: result.bidAmount, // converts human to native
                },
            })
        } else {
            this.addLog({ msg: "Not bidding", reason: result.reason })
        }
    }

    /**
     * Evaluate the SFX via the strategy engine. If the SFX passes all constraints defined by the executor, the bidding engine is triggered,
     * computing the bid price. The bid price is then returned
     *
     * @private
     * @returns Any
     */
    // ToDo fix return type
    private generateBid(): any {
        if (this.isBidder) return { trigger: false, reason: "Already a bidder" }
        if (this.txStatus !== TxStatus.Ready) return { trigger: false, reason: "Tx not ready" }
        if (this.status !== SfxStatus.InBidding) return { trigger: false, reason: "Not in bidding phase" }

        try {
            this.strategyEngine.evaluateSfx(this)
        } catch (e: any) {
            this.logger.info(`Not bidding SFX ${this.humanId}`)
            return { trigger: false, reason: e.toString() }
        }

        // we have passed all checks and need to compute the bid amount
        this.txStatus = TxStatus.Pending // acts as mutex lock
        this.minProfitUsd = this.strategyEngine.getMinProfitUsd(this)
        const bidUsd = this.biddingEngine.computeBid(this)
        // TODO: should this be here or in "processBid"?
        // this.lastBids.push(bidUsd)
        const bidRewardAsset = bidUsd / this.rewardAssetPrice.getValue()

        return { trigger: true, bidAmount: floatToBn(bidRewardAsset) }
    }

    /**
     * Generate an array of arguments for the SFX execution.
     *
     * @returns Any[] - Array of arguments for the SFX execution in the corresponding type
     */
    execute(): any[] {
        switch (this.action) {
            case SfxType.Transfer: {
                return this.getTransferArguments()
            }
        }
    }

    /**
     * Returns TxOutput containing the outputs required for a SFXs execution
     *
     * @returns TxOutput.
     */
    getTxOutputs(): TxOutput {
        switch (this.action) {
            case SfxType.Transfer: {
                const amount = this.getTransferArguments()[1]
                return {
                    amount: amount,
                    amountHuman: this.gateway.toFloat(amount), // converts to human format
                    asset: this.gateway.ticker,
                }
            }
        }
    }

    /**
     * Perform state updates if our bid has been accepted.
     *
     * @param bidAmount The bidding amount that was accepted. This is the reward amount, which is added to the subject
     */
    bidAccepted(bidAmount: number) {
        this.txStatus = TxStatus.Ready // open mutex lock

        // usually, event fire quicker then a TX resolves. This prevents that we overwrite the TX status, when a lower bid was in the same block
        if (this.reward.getValue() >= this.gateway.toFloat(bidAmount)) {
            this.isBidder = true
            this.reward.next(this.gateway.toFloat(bidAmount)) // not sure if we want to do this tbh. Reacting to other bids should be sufficient
            this.logger.info(`Bid accepted for SFX ${this.humanId} ✅`)
            this.addLog({ msg: "Bid accepted", bidAmount: bidAmount.toString() })
        } else {
            this.triggerBid() // trigger another bid, as we have been outbid. The risk parameters are updated automatically by events.
            this.logger.info(`Bid undercut in block for SFX ${this.humanId} ❌`)
            this.addLog({ msg: "Bid accepted, but undercut in same block", bidAmount: bidAmount.toString() })
        }
    }

    /**
     * Perform state updates if out bid has been rejected.
     *
     * @param error Error message used for logging
     */
    bidRejected(error: any) {
        // a better bid was submitted before this one was accepted. A new eval will be triggered with the incoming bid event
        this.txStatus = TxStatus.Ready // open mutex lock
        this.isBidder = false
        this.addLog({ msg: "Bid rejected", err: error.toString() })
        this.triggerBid()
    }

    /**
     * Keep a record of how much a certain executor bid on the SFX.
     * 
     * @param xtxId ID of the executor
     * @param bid The amount the executor bid
     */
    storeXtxBidHistory(xtxId: string, bid: number) {
        if (this.xtxBidHistory !== undefined) {
            let previousXtxBids = this.xtxBidHistory.get(xtxId)
            if (previousXtxBids !== undefined) {
                previousXtxBids.push(bid);
                this.xtxBidHistory.set(xtxId, previousXtxBids);
            } else {
                this.xtxBidHistory.set(xtxId, [bid]);
            }
        } else {
            this.xtxBidHistory = new Map([[xtxId, [bid]]]);
        }
    }

    /**
     * Process an incoming bid event. The bid amount is now the new reward amount and the SFX is evaluated again, potentially triggering a
     * counter bid.
     *
     * @param signer Signer of the incoming bid
     * @param bidAmount Amount of the incoming bid
     */
    processBid(signer: string, bidAmount: number) {
        // Add the executor bid to the list 
        this.biddingEngine.storeWhoBidOnWhat(this.id, signer);
        // TODO: should this be here or in "generateBid"?
        // Add how much it bid
        this.lastBids.push(bidAmount);
        // Add the xtx's bid to its history
        this.storeXtxBidHistory(signer, bidAmount);

        // if this is not own bid, update reward and isBidder
        if (signer !== this.circuitSignerAddress) {
            this.logger.info(`Competing bid on SFX ${this.humanId}: Exec: ${signer} ${toFloat(bidAmount)} TRN 🎰`)
            this.addLog({ msg: "Competing bid received", signer, bidAmount })
            this.isBidder = false
            this.reward.next(this.gateway.toFloat(bidAmount)) // this will trigger the re-eval of submitting a new bid
        } else {
            this.addLog({ msg: "Own bid detected", signer, bidAmount })
        }
    }

    /** Update the SFX status */
    readyToExecute() {
        this.status = SfxStatus.ReadyToExecute
    }

    // sfx was successfully executed on target and has the inclusion proof data
    /**
     * SFX was successfully executed and the required proof data generated
     *
     * @param inclusionData Inclusion proof data
     * @param executor Address of the executor on target
     * @param targetInclusionHeight Block height on target where transaction was included
     */
    executedOnTarget(inclusionData: InclusionData, executor: string, targetInclusionHeight: number) {
        this.inclusionData = inclusionData
        this.executor = executor
        this.targetInclusionHeight = targetInclusionHeight
        this.status = SfxStatus.ExecutedOnTarget
    }

    /** SFX is confirmed, so wecan update the status and emit an event */
    confirmedOnCircuit() {
        this.status = SfxStatus.Confirmed

        // unsubscribing from all subjects, as no longer needed
        this.unsubscribe()
    }

    /** Update the SFX status to dropped at bidding and unsubscribe all subjects */
    droppedAtBidding() {
        this.status = SfxStatus.Dropped
        this.unsubscribe()
    }

    /** Update the SFX status to reverted and unsubscribe all subjects */
    reverted() {
        this.status = SfxStatus.Reverted
        this.unsubscribe()
    }

    /** Maps action to enum */
    private decodeAction(encodedAction: any): boolean {
        switch (encodedAction.toHuman()) {
            case "tran": {
                this.action = SfxType.Transfer
                return true
                break
            }
            default: {
                return false
            }
        }
    }

    // returns the arguments
    private getTransferArguments(): any[] {
        return [this.arguments[1], this.gateway.parseLe(this.arguments[2]).toNumber()]
    }

    private unsubscribe() {
        this.subscriptions.forEach((subscription) => {
            subscription.unsubscribe()
        })
    }

    private addLog(msg: any, debug: boolean = true) {
        msg.component = "SFX"
        msg.sfxId = this.id
        msg.xtxId = this.xtxId

        if (debug) {
            this.logger.debug(msg)
        } else {
            this.logger.error(msg)
        }
    }
}
