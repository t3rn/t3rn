import createDebug from "debug"
import {Execution} from "./execution";
import {Notification, NotificationType, SideEffect} from "./sideEffect";
import Estimator from "../gateways/substrate/estimator";
import SubstrateRelayer from "../gateways/substrate/relayer";
import {PriceEngine} from "../pricing";
import {StrategyEngine} from "../strategy";
import {Sdk} from "@t3rn/sdk";
import {BiddingEngine} from "../bidding";
import {CircuitListener, ListenerEventData, ListenerEvents} from "../circuit/listener"
import {ApiPromise} from "@polkadot/api";
import {CircuitRelayer} from "../circuit/relayer";
import {ExecutionLayerType} from "@t3rn/sdk/dist/src/gateways/types";
import {RelayerEventData, RelayerEvents} from "../gateways/types";
import {XtxStatus} from "@t3rn/sdk/dist/src/side-effects/types";


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
		completed: string[],
		dropped: string[],
		reverted: string[],
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
        const gatewayKeys = Object.keys(this.sdk.gateways);
        for (let i = 0; i < gatewayKeys.length; i++) {
            const entry = this.sdk.gateways[gatewayKeys[i]]

            if (entry.executionLayerType === ExecutionLayerType.Substrate) {
                const relayer = new SubstrateRelayer()
                await relayer.setup(entry.rpc, undefined, entry.id)

                const estimator = new Estimator(relayer)

                // setup in executionManager
                this.queue[entry.id] = {
					blockHeight: 0,
					isBidding: [],
					isExecuting: [],
					isConfirming: {},
					completed: [],
					dropped: [],
					reverted: [],
				}

				this.targetEstimator[entry.id] = estimator;
                // store relayer instance locally
                this.relayers[entry.id] = relayer

				relayer.on("Event", async (eventData: RelayerEventData) => {
					switch (eventData.type) {
						case RelayerEvents.SfxExecutedOnTarget:
							this.removeFromQueue("isExecuting", eventData.sfxId, eventData.target)

							// create array if first for block
							if (!this.queue[eventData.target].isConfirming[eventData.blockNumber]) {
								this.queue[eventData.target].isConfirming[eventData.blockNumber] = []
							}
							// adds to queue
							this.queue[eventData.target].isConfirming[eventData.blockNumber].push(eventData.sfxId)

							console.log(this.queue)
							break;
						case RelayerEvents.SfxExecutionError:
							break;

					}
				})
            }
        }
    }

	async initializeEventListeners() {
        this.circuitListener.on("Event", async (eventData: ListenerEventData) => {
            switch (eventData.type) {
                case ListenerEvents.NewSideEffectsAvailable:
                    console.log("NewSideEffectsAvailable")
                    this.addXtx(eventData.data, this.sdk)
					break;
				case ListenerEvents.SFXNewBidReceived:
					console.log("SFXNewBidReceived")
					this.addBid(eventData.data)
					break;
				case ListenerEvents.XTransactionReadyForExec:
					console.log("XTransactionReadyForExec")
					this.xtxReadyForExec(eventData.data[0].toString())
					break;
				case ListenerEvents.HeaderSubmitted:
					console.log("HeaderSubmitted")
					this.updateGatewayHeight(eventData.data.gatewayId, eventData.data.height)
					break;
				case ListenerEvents.SideEffectConfirmed:
					console.log("SideEffectConfirmed")
					const sfxId = eventData.data[0].toString()
					this.xtx[this.sfxToXtx[sfxId]].sideEffects.get(sfxId)!.confirmedOnCircuit()
					break;
				case ListenerEvents.XtxCompleted:
					console.log("XtxCompleted")
					this.xtx[eventData.data[0].toString()].completed()
					break;
				case ListenerEvents.DroppedAtBidding:
					this.droppedAtBidding(eventData.data[0].toString());
					break;
				case ListenerEvents.RevertTimedOut:
					console.log("RevertTimedOut")
					console.log()
					this.revertTimeout(eventData.data[0].toString())

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
			this.queue[sfx.target].isBidding.push(sfxId)
			await this.addRiskRewardParameters(sfx)

		}
	}

	addBid(bidData: any) {
		const sfxId = bidData[0].toString()
		const bidder = bidData[1].toString()
		const amt = bidData[2].toNumber()
		console.log(sfxId, bidder, amt)
		this.xtx[this.sfxToXtx[sfxId]].sideEffects.get(sfxId)?.processBid(bidder, amt)
	}

	async xtxReadyForExec(xtxId: string) {
		this.xtx[xtxId].setReadyToExecute();
		const ready = this.xtx[xtxId].getReadyToExecute();
		for(const sfx of ready) {
			// move on the queue
			this.removeFromQueue("isBidding", sfx.id, sfx.target)
			this.queue[sfx.target].isExecuting.push(sfx.id)
			// execute
			this.relayers[sfx.target].executeTx(sfx)
				.then()
		}
	}

	// New header range was submitted on circuit
    // update local params and check which SideEffects can be confirmed
    updateGatewayHeight(gatewayId: string, blockHeight: number) {
		console.log("Update Gateway Height", gatewayId, blockHeight)
        if (this.queue[gatewayId]) {
            this.queue[gatewayId].blockHeight = blockHeight
            this.executeConfirmationQueue(gatewayId)
        }
	}

	// checks which executed SideEffects can be confirmed on circuit
    async executeConfirmationQueue(gatewayId: string) {
        // contains the sfxIds of SideEffects that could be confirmed based on the current blockHeight of the light client
        let readyByHeight: string[] = [];
		// stores the block height of the SideEffects that are ready to be confirmed. Needed for clearing the queue
		let batchBlocks: string[] = [];
		const queuedBlocks = Object.keys(this.queue[gatewayId].isConfirming);
		for(let i = 0; i < queuedBlocks.length; i++) {
            if(parseInt(queuedBlocks[i]) <= this.queue[gatewayId].blockHeight) {
				batchBlocks.push(queuedBlocks[i])
                readyByHeight = readyByHeight.concat(this.queue[gatewayId].isConfirming[queuedBlocks[i]])
            }
		}

		console.log("Ready by height", readyByHeight)

        const readyByStep: SideEffect[] = [];

        // filter the SideEffects that can be confirmed in the current step
		for(let i = 0; i < readyByHeight.length; i++) {
            const sfx: SideEffect = this.xtx[this.sfxToXtx[readyByHeight[i]]].sideEffects.get(readyByHeight[i])!;
            if(sfx.step === this.xtx[sfx.xtxId].currentStep) {
                readyByStep.push(sfx)
            }
        }

        ExecutionManager.debug(`${gatewayId} - ${this.queue[gatewayId].blockHeight}: ReadyToConfirm: ${readyByStep.length}`)

		if(readyByStep.length > 0) {
			this.circuitRelayer.confirmSideEffects(readyByStep)
				.then((res: any) => {
					// remove from queue and update status
					this.processConfirmationBatch(readyByStep, batchBlocks, gatewayId)
				})

		}
	}

	// updates queue and sfx state after confirmation batch was submitted. This always has the same target
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

	async initSfxListeners(sfx: SideEffect) {
		sfx.on("Notification", (notification: Notification) => {
			switch(notification.type) {
				case NotificationType.SubmitBid: {
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

	droppedAtBidding(xtxId: string) {
		const xtx = this.xtx[xtxId]

		if(xtx && !(xtx.status === XtxStatus.DroppedAtBidding)) { // ToDo remove once 504 is fixed
			console.log("Dropped at bidding", xtxId)
			xtx.droppedAtBidding()
			for(const sfx of xtx.sideEffects.values()) {
				this.removeFromQueue("isBidding", sfx.id, sfx.target)
				this.queue[sfx.target].dropped.push(sfx.id)
			}
		}

		console.log(this.queue)
	}

	revertTimeout(xtxId: string) {
		const xtx = this.xtx[xtxId]
		for(const sfx of xtx.sideEffects.values()) {
			// sfx could either be in isExecuting or isConfirming
			this.removeFromQueue("isExecuting", sfx.id, sfx.target)
			let confirmBatch = this.queue[sfx.target].isConfirming[sfx.targetInclusionHeight.toString()];
			if(!confirmBatch) confirmBatch = [];
			if(confirmBatch.includes(sfx.id)) {
				const index = confirmBatch.indexOf(sfx.id)
				confirmBatch.splice(index, 1)
			}

			// add to reverted queue
			this.queue[sfx.target].reverted.push(sfx.id)
		}
		this.xtx[xtxId].revertTimeout()
		console.log(this.queue)
	}

	// removes sfx from queue
    private removeFromQueue(queue: string, id: string, gatewayId: string) {
        const index = this.queue[gatewayId][queue].indexOf(id)
		if(index > -1) {
        	this.queue[gatewayId][queue].splice(index, 1)
		}
    }
}