import "@t3rn/types";
import { AccountId32, H256 } from "@polkadot/types/interfaces";
import { SideEffect } from "./sideEffect";
// @ts-ignore
import { T3rnTypesSideEffect } from "@polkadot/types/lookup"
import { EventEmitter } from "events"

import { SecurityLevel, SfxStatus, XtxStatus } from "@t3rn/sdk/dist/src/side-effects/types"
import { Sdk } from "@t3rn/sdk"
import { Config } from "../../config/config"
import { StrategyEngine } from "../strategy"
import { BiddingEngine } from "../bidding"
import { createLogger } from "../utils"

/**
 * Extra data to recreate a serialized execution
 *
 * @group Execution Manager
 */
export interface Miscellaneous {
    executorName: string
    logsDir: string
    circuitRpc: string
    circuitSignerAddress: string
    circuitSignerSecret: string
}

/**
 * JSON serializable execution
 *
 * @group Execution Manager
 */
export interface SerializableExecution {
    id: string
    status: XtxStatus
    humanId: string
    owner: string
    sideEffects: { [key: string]: SideEffect }
    phases: string[][]
    currentPhase: number
    misc: Miscellaneous
}

/**
 * Class used for tracking the life-cycle of an XTX. Contains all required parameters and methods for executing the XTX.
 *
 * @group Execution Manager
 */
export class Execution extends EventEmitter {
<<<<<<< HEAD
  /** The current status of the XTX. */
  status: XtxStatus = XtxStatus.PendingBidding;
  /** SCALE encoded XTX */
  xtxId: H256;
  /** XTX id as String */
  id: string;
  /** XTX id as String, shortened to 8 characters */
  humanId: string;
  /** The owner of the XTX. This is the on-chain creator */
  owner: AccountId32;
  /** Mapping of the included SFXs */
  sideEffects: Map<string, SideEffect> = new Map<string, SideEffect>();
  /** Stores SFX ids in the corresponding XTX phase */
  phases: string[][] = [[], []];
  /** The current phase of the XTX */
  currentPhase: number;
  /** The circuit signer address */
  circuitSignerAddress: string;
  logger: any;

  /**
   * Creates a new Execution instance.
   *
   * @param eventData The event data of the XTX creation event.
   * @param sdk The @t3rn/sdk instance.
   * @param strategyEngine The strategy engine instance.
   * @param biddingEngine The bidding engine instance.
   * @param circuitSignerAddress The circuit signer address.
   * @param logger The logger instance.
   */
  constructor(
    eventData: any,
    sdk: Sdk,
    strategyEngine: StrategyEngine,
    biddingEngine: BiddingEngine,
    circuitSignerAddress: string,
    logger: any
  ) {
    super();
    this.owner = eventData[0];
    this.xtxId = eventData[1];
    this.id = this.xtxId.toHex();
    this.humanId = this.id.slice(0, 8);
    this.circuitSignerAddress = circuitSignerAddress;
    this.logger = logger;
    this.initializeSideEffects(
      eventData[2],
      eventData[3],
      sdk,
      strategyEngine,
      biddingEngine
    );
    this.currentPhase = 0;
  }

  /**
   * Creates the new SideEffect instances, maps them locally and generates the phases as done in circuit.
   *
   * @param sideEffects Array of SCALE encoded SFXs
   * @param ids Array of SCALE encoded SFX ids
   * @param sdk The @t3rn/sdk instance.
   * @param strategyEngine The strategy engine instance.
   * @param biddingEngine The bidding engine instance.
   */
  initializeSideEffects(
    sideEffects: T3rnTypesSideEffect[],
    ids: H256[],
    sdk: Sdk,
    strategyEngine: StrategyEngine,
    biddingEngine: BiddingEngine
  ) {
    for (let i = 0; i < sideEffects.length; i++) {
      const sideEffect = new SideEffect(
        sideEffects[i],
        ids[i].toHex(),
        this.xtxId.toHex(),
        sdk,
        strategyEngine,
        biddingEngine,
        this.circuitSignerAddress,
        this.logger
      );
      this.sideEffects.set(sideEffect.id, sideEffect);

      if (sideEffect.securityLevel === SecurityLevel.Escrow) {
        // group escrow phases into one step
        this.phases[0].push(ids[i].toHex());
      } else {
        this.phases[1].push(ids[i].toHex()); // optimistic get their own step
      }
    }

    // remove escrow phases, if there are none
    if (this.phases[0].length === 0) {
      this.phases = [this.phases[1]];
=======
    /** The current status of the XTX. */
    status: XtxStatus = XtxStatus.PendingBidding
    /** SCALE encoded XTX */
    xtxId: H256
    /** XTX id as String */
    id: string
    /** XTX id as String, shortened to 8 characters */
    humanId: string
    /** The owner of the XTX. This is the on-chain creator */
    owner: AccountId32
    /** Mapping of the included SFXs */
    sideEffects: Map<string, SideEffect> = new Map<string, SideEffect>()
    /** Stores SFX ids in the corresponding XTX phase */
    phases: string[][] = [[], []]
    /** The current phase of the XTX */
    currentPhase: number

    logger: any
    sdk: Sdk
    biddingEngine: BiddingEngine
    strategyEngine: StrategyEngine
    misc: Miscellaneous
    config: Config

    /**
     * Creates a new Execution instance.
     *
     * @param eventData The event data of the XTX creation event.
     * @param sdk The @t3rn/sdk instance.
     * @param strategyEngine The strategy engine instance.
     * @param biddingEngine The bidding engine instance.
     * @param logger The logger instance.
     * @param misc Add. data required for de/serialization.
     */
    constructor(eventData: any, sdk: Sdk, strategyEngine: StrategyEngine, biddingEngine: BiddingEngine, logger: any, misc: Miscellaneous) {
        super()
        this.owner = eventData[0]
        this.xtxId = eventData[1]
        this.id = this.xtxId.toHex()
        this.humanId = this.id.slice(0, 8)
        this.logger = logger
        this.initializeSideEffects(eventData[2], eventData[3], sdk, strategyEngine, biddingEngine)
        this.currentPhase = 0
        this.misc = misc
    }

    /** Custom JSON serialization. */
    toJSON(): SerializableExecution {
        return {
            id: this.id,
            status: this.status,
            humanId: this.humanId,
            owner: this.owner.toString(),
            sideEffects: Object.fromEntries(this.sideEffects),
            phases: this.phases,
            currentPhase: this.currentPhase,
            misc: {
                executorName: this.misc.executorName,
                logsDir: this.misc.logsDir,
                circuitRpc: this.misc.circuitRpc,
                circuitSignerAddress: this.misc.circuitSignerAddress,
                circuitSignerSecret: this.misc.circuitSignerSecret,
            },
        }
    }

    /** Custom JSON deserialization. */
    static fromJSON(o: SerializableExecution): Execution {
        const logger = createLogger(o.misc.executorName, o.misc.logsDir)
        return new Execution(
            [
                o.owner,
                {
                    toHex() {
                        return o.id
                    },
                },
                Object.values(o.sideEffects),
                Object.keys(o.sideEffects),
            ],
            new Sdk(o.misc.circuitRpc, o.misc.circuitSignerSecret),
            new StrategyEngine(),
            new BiddingEngine(logger),
            logger,
            o.misc
        )
    }

    /**
     * Creates the new SideEffect instances, maps them locally and generates the phases as done in circuit.
     *
     * @param sideEffects Array of SCALE encoded SFXs
     * @param ids Array of SCALE encoded SFX ids
     * @param sdk The @t3rn/sdk instance.
     * @param strategyEngine The strategy engine instance.
     * @param biddingEngine The bidding engine instance.
     */
    async initializeSideEffects(
        sideEffects: T3rnTypesSideEffect[],
        ids: H256[],
        sdk: Sdk,
        strategyEngine: StrategyEngine,
        biddingEngine: BiddingEngine
    ) {
        for (let i = 0; i < sideEffects.length; i++) {
            //   const records = (await api.rpc.xdns.fetchFullRecords());
            //   let res: Record<string, Gateway> = {};

            //   for (let i = 0; i < records.length; i++) {
            //     const gateway = new Gateway(records[i]);
            //     res[gateway.id] = gateway;
            //   }

            const sideEffect = new SideEffect(
                sideEffects[i],
                ids[i].toHex(),
                this.xtxId.toHex(),
                sdk,
                strategyEngine,
                biddingEngine,
                this.logger,
                this.misc
            )
            this.sideEffects.set(sideEffect.id, sideEffect)

            if (sideEffect.securityLevel === SecurityLevel.Escrow) {
                // group escrow phases into one step
                this.phases[0].push(ids[i].toHex())
            } else {
                this.phases[1].push(ids[i].toHex()) // optimistic get their own step
            }
        }

        // remove escrow phases, if there are none
        if (this.phases[0].length === 0) {
            this.phases = [this.phases[1]]
        }

        // set the step index for each sfx
        for (let [sfxId, sfx] of this.sideEffects) {
            for (let i = 0; i < this.phases.length; i++) {
                if (this.phases[i].includes(sfxId)) {
                    sfx.setPhase(i)
                }
            }
        }
>>>>>>> b6fb64b5... feat: xtx json serialization
    }

    // set the step index for each sfx
    for (const [sfxId, sfx] of this.sideEffects) {
      for (let i = 0; i < this.phases.length; i++) {
        if (this.phases[i].includes(sfxId)) {
          sfx.setPhase(i);
        }
      }
    }
  }

  /** Update XTX and all its SFX status to ready. */
  readyToExecute() {
    this.status = XtxStatus.ReadyToExecute;

    //Updates each Sfx
    for (const [_sfxId, sfx] of this.sideEffects) {
      sfx.readyToExecute();
    }

    this.logger.info(`Ready XTX: ${this.humanId}`);
    this.addLog({ msg: "Ready XTX" });
  }

  /** Update XTX status to complete */
  completed() {
    this.status = XtxStatus.FinishedAllSteps;
    this.logger.info(`Completed XTX: ✨${this.humanId}✨`);
    this.addLog({ msg: "Completed XTX" });
  }

  /** Update XTX and all its SFX status to ready. */
  droppedAtBidding() {
    this.status = XtxStatus.DroppedAtBidding;
    for (const [_sfxId, sfx] of this.sideEffects) {
      sfx.droppedAtBidding();
    }
    this.logger.info(`Dropped XTX: ${this.humanId}`);
    this.addLog({ msg: "Dropped XTX", xtxId: this.id });
  }

  /** Update XTX and all its SFX status to reverted. */
  revertTimeout() {
    this.status = XtxStatus.RevertTimedOut;
    for (const [_sfxId, sfx] of this.sideEffects) {
      sfx.reverted();
    }

    this.logger.info(`Revert XTX: ${this.humanId}`);
    this.addLog({ msg: "Revert XTX", xtxId: this.id });
  }

  /**
   * Returns the sfxs that ready to execute. This depends on the SFX status, if the executor has won the bid and if the SFX is in the current phase.
   *
   * @returns {SideEffect[]} Array of SideEffect instances that are ready
   */
  getReadyToExecute(): SideEffect[] {
    const result: SideEffect[] = [];
    for (const [_sfxId, sfx] of this.sideEffects) {
      if (
        sfx.status === SfxStatus.ReadyToExecute &&
        sfx.isBidder &&
        sfx.phase === this.currentPhase
      ) {
        result.push(sfx);
      }
    }
    return result;
  }

  private addLog(msg: any, debug = true) {
    msg.component = "XTX";
    msg.id = this.id;

    if (debug) {
      this.logger.debug(msg);
    } else {
      this.logger.error(msg);
    }
  }
}
