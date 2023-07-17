import "@t3rn/types";
import { AccountId32, H256 } from "@polkadot/types/interfaces";
import { SideEffect } from "./sideEffect";
// @ts-ignore - Typescript does not know about this type
import { T3rnTypesSideEffect } from "@polkadot/types/lookup";
import { EventEmitter } from "events";

import {
  SecurityLevel,
  SfxStatus,
  XtxStatus,
} from "@t3rn/sdk/side-effects/types";
import { Sdk } from "@t3rn/sdk";
import { StrategyEngine } from "../strategy";
import { BiddingEngine } from "../bidding";
import { Logger } from "pino";
import { EventData } from "src/circuit/listener";

/**
 * Class used for tracking the life-cycle of an XTX. Contains all required parameters and methods for executing the XTX.
 *
 * @group Execution Manager
 */
export class Execution extends EventEmitter {
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
    public logger: Logger,
    public circuitSignerAddress: string,
    eventData: EventData,
    sdk: Sdk,
    strategyEngine: StrategyEngine,
    biddingEngine: BiddingEngine,
  ) {
    super();
    this.owner = eventData[0];
    this.xtxId = eventData[1];
    this.id = this.xtxId.toHex();
    this.humanId = this.id.slice(0, 8);
    this.initializeSideEffects(
      eventData[2],
      eventData[3],
      sdk,
      strategyEngine,
      biddingEngine,
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
    biddingEngine: BiddingEngine,
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
        this.logger,
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
    for (const [, sfx] of this.sideEffects) {
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
    for (const [, sfx] of this.sideEffects) {
      sfx.droppedAtBidding();
    }
    this.logger.info(`Dropped XTX: ${this.humanId}`);
    this.addLog({ msg: "Dropped XTX", xtxId: this.id });
  }

  /** Update XTX and all its SFX status to reverted. */
  revertTimeout() {
    this.status = XtxStatus.RevertTimedOut;
    for (const [, sfx] of this.sideEffects) {
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
    for (const [, sfx] of this.sideEffects) {
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

  private addLog(
    msg: {
      msg: string;
      component?: string;
      id?: string;
      xtxId?: string;
    },
    debug = true,
  ) {
    msg.component = "XTX";
    msg.id = this.id;

    if (debug) {
      this.logger.debug(msg);
    } else {
      this.logger.error(msg);
    }
  }
}
