import "@t3rn/types";
import { EventEmitter } from "events";
import { Sdk } from "@t3rn/sdk";

/**
 * Enum for the different types of events emitted by the relayer
 *
 * @group t3rn Circuit
 */
export enum ListenerEvents {
  /** A new XTX was detected on Circuit */
  NewSideEffectsAvailable,
  /** A new SFX bid was detected */
  SFXNewBidReceived,
  /** An XTX is ready to be executed */
  XTransactionReadyForExec,
  /** New headers where detected for a specific gateway */
  HeaderSubmitted,
  /** A SFX was confirmed on circuit */
  SideEffectConfirmed,
  /** A XTX was finalized */
  XtxCompleted,
  /** A XTX was dropped at bidding */
  DroppedAtBidding,
  /** A XTX was reverted */
  RevertTimedOut,
}

export type PropEventData = {
  vendor?: string;
  height?: number;
};

export type ListEventData = Array<{
  toString: () => string;
  toNumber: () => number;
}>;

/**
 * Type for transporting events
 *
 * @group t3rn Circuit
 */
export type ListenerEventData = {
  type: ListenerEvents;
  data: ListEventData | PropEventData;
};

/** @group t3rn Circuit */
export class CircuitListener extends EventEmitter {
  stop: () => void;

  constructor(public client: Sdk["client"]) {
    super();
  }

  async start() {
    this.stop = await this.client.query.system.events((notifications) => {
      for (let i = 0; i < notifications.length; i++) {
        if (notifications[i].event.method === "NewSideEffectsAvailable") {
          // receives new side effects
          this.emit("Event", <ListenerEventData>{
            type: ListenerEvents.NewSideEffectsAvailable,
            data: notifications[i].event.data,
          });
        } else if (notifications[i].event.method === "SFXNewBidReceived") {
          this.emit("Event", <ListenerEventData>{
            type: ListenerEvents.SFXNewBidReceived,
            data: notifications[i].event.data,
          });
        } else if (
          notifications[i].event.method === "XTransactionReadyForExec"
        ) {
          this.emit("Event", <ListenerEventData>{
            type: ListenerEvents.XTransactionReadyForExec,
            data: notifications[i].event.data,
          });
        } else if (notifications[i].event.method === "HeadersAdded") {
          console.log(notifications[i].toHuman());

          let vendor = "";
          if (notifications[i].event.section === "rococoBridge") {
            vendor = "Rococo";
          }
          const data = {
            vendor,
            height: parseInt(String(notifications[i].event.data[0])),
          };
          this.emit("Event", <ListenerEventData>{
            type: ListenerEvents.HeaderSubmitted,
            data,
          });
        } else if (notifications[i].event.method === "SideEffectConfirmed") {
          this.emit("Event", <ListenerEventData>{
            type: ListenerEvents.SideEffectConfirmed,
            data: notifications[i].event.data,
          });
        } else if (
          notifications[i].event.method ===
          "XTransactionXtxFinishedExecAllSteps"
        ) {
          this.emit("Event", <ListenerEventData>{
            type: ListenerEvents.XtxCompleted,
            data: notifications[i].event.data,
          });
        } else if (
          notifications[i].event.method === "XTransactionXtxDroppedAtBidding"
        ) {
          this.emit("Event", <ListenerEventData>{
            type: ListenerEvents.DroppedAtBidding,
            data: notifications[i].event.data,
          });
        } else if (
          notifications[i].event.method ===
          "XTransactionXtxRevertedAfterTimeOut"
        ) {
          this.emit("Event", <ListenerEventData>{
            type: ListenerEvents.RevertTimedOut,
            data: notifications[i].event.data,
          });
        }
      }
    });
  }
}
