import createDebug from "debug"
import {Execution} from "./execution";
import {Notification, NotificationType, SideEffect} from "./sideEffect";
import Estimator from "../gateways/substrate/estimator";
import SubstrateRelayer from "../gateways/substrate/relayer";
import {PriceEngine} from "../pricing";
import {StrategyEngine} from "../strategy";
import {Sdk} from "@t3rn/sdk";
import {BiddingEngine} from "../bidding";
import {CircuitListener, EventData, Events} from "../circuit/listener"
import {ApiPromise} from "@polkadot/api";
import CircuitRelayer from "../circuit/relayer";
import {ExecutionLayerType} from "@t3rn/sdk/dist/src/gateways/types";


// A type used for storing the different SideEffects throughout their respective life-cycle.
// Please note that waitingForInsurance and readyToExecute are only used to track the progress. The actual logic is handeled in the execution
type Queue = {
    gateways: {
        blockHeight: number
		isBidding: string[],
		isExecuting: string[],
        // Executed sfx and their respective execution block.
        isConfirming: {
            [block: number]: string[]
        },
		complete: string[]
    }
}


export class ExecutionManager {
	static debug = createDebug("execution-manager")

	// we map the current state in the queue
	queue: Queue = <Queue>{}
    // maps xtxId to Execution instance
    xtx: {
        [id: string]: Execution
    } = {}
    // a lookup mapping, to find a sfx xtxId
    sfxToXtx: {
        [sfxId: string]: string
    } = {};

	targetEstimator: {
        [id: string]: Estimator
    } = {};

	priceEngine: PriceEngine;
	strategyEngine: StrategyEngine;
	biddingEngine: BiddingEngine;


    sdk: Sdk;
    circuitClient: ApiPromise;
    circuitListener: CircuitListener;
    circuitRelayer: CircuitRelayer;
    relayers: { [key: string]: SubstrateRelayer } = {};
    signer: any;

	constructor(circuitClient: ApiPromise, sdk: Sdk) {
		this.priceEngine = new PriceEngine();
		this.strategyEngine = new StrategyEngine();
		this.biddingEngine = new BiddingEngine();
		this.circuitClient = circuitClient;
		this.circuitListener = new CircuitListener(this.circuitClient)
		this.circuitRelayer = new CircuitRelayer(sdk)
		this.sdk = sdk;
	}

	async setup() {
		await this.initializeGateways()
		await this.circuitListener.start()
		await this.initializeEventListeners()
	}

	async initializeGateways() {
		// @ts-ignore
		console.log(this.circuitRelayer.signer.address)
        const gatewayKeys = Object.keys(this.sdk.gateways);
        for (let i = 0; i < gatewayKeys.length; i++) {
            const entry = this.sdk.gateways[gatewayKeys[i]]

            if (entry.executionLayerType === ExecutionLayerType.Substrate) {
                let relayer = new SubstrateRelayer()
                await relayer.setup(entry.rpc, undefined)

                const estimator = new Estimator(relayer)

                // setup in executionManager
                this.queue[entry.id] = {
					blockHeight: 0,
					waitingForInsurance: [],
					readyToExecute: [],
					readyToConfirm: {},
				}

				this.targetEstimator[entry.id] = estimator;
                // store relayer instance locally
                this.relayers[entry.id] = relayer
            }
        }
    }

	async initializeEventListeners() {
        this.circuitListener.on("Event", async (eventData: EventData) => {
            switch (eventData.type) {
                case Events.NewSideEffectsAvailable:
                    console.log("NewSideEffectsAvailable")
                    this.addXtx(eventData.data, this.sdk)
					break;
				case Events.SFXNewBidReceived:
					console.log("SFXNewBidReceived")
					this.addBid(eventData.data)
					break;

            }

        })
	}


	async addXtx(xtxData: any, sdk: Sdk) {
		// @ts-ignore
		const xtx = new Execution(xtxData, sdk, this.strategyEngine, this.biddingEngine, this.circuitRelayer.signer.address);
		try {
			this.strategyEngine.evaluateXtx(xtx)

		} catch(e) {
			console.log(`Xtx ${xtx.humanId} eval failed!`, e.toString())
			return
		}

		this.xtx[xtx.id] = xtx
		for (const [sfxId, sfx] of xtx.sideEffects.entries()) {
			this.initSfxListeners(sfx);
			this.sfxToXtx[sfxId] = xtx.id
			await this.addRiskRewardParameters(sfx)

		}
	}

	addBid(bidData: any) {
		const sfxId = bidData[0].toString()
		const bidder = bidData[1].toString()
		const amt = bidData[2].toNumber()
		console.log(sfxId, bidder, amt)
		this.xtx[this.sfxToXtx[sfxId]].sideEffects.get(sfxId)?.processBid(bidder, amt)
		// const xtxId = this.sfxToXtx[bidData.sfxId]
		// const xtx = this.xtx[xtxId]
		// const sfx = xtx.sideEffects.get(bidData.sfxId)
		// sfx.addBid(bidData)
	}

	async initSfxListeners(sfx: SideEffect) {
		sfx.on("Notification", (notification: Notification) => {
			switch(notification.type) {
				case NotificationType.SubmitBid: {
					console.log("Submit bid")
					this.circuitRelayer.bidSfx(notification.payload.sfxId, notification.payload.bidAmount)
						.then(() => sfx.bidAccepted(notification.payload.bidAmount))
						.catch((e) => {
							console.log("Bid rejected", e)
							sfx.bidRejected()
						})

				}
			}
		})
	}

	async addRiskRewardParameters(sfx: SideEffect) {
		const txCostSubject = await this.targetEstimator[sfx.target].getNativeTxCostSubject(sfx);
		const nativeAssetPriceSubject = this.priceEngine.getAssetPrice(sfx.gateway.ticker);

		const txOutput = sfx.getTxOutputs()
		const txOutputPriceSubject = this.priceEngine.getAssetPrice(txOutput.asset);
		const rewardAssetPriceSubject = this.priceEngine.getAssetPrice("TRN");

		sfx.setRiskRewardParameters(txCostSubject, nativeAssetPriceSubject, txOutputPriceSubject, rewardAssetPriceSubject)
	}



}