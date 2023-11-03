/**
 * Describes the different SFX types that are available.
 */
export enum SfxType {
  /* Native Transfer on Target */
  Transfer,
  /* Transfer Asset between Source and Destination */
  TransferAsset,
  /* Call EVM Contract with given input on Destination */
  CallEVM,
  /* Call WASM Contract with given input on Destination */
  CallWASM,
  /* Call Generic (e.g. Substrate's Pallet) with given input on Destination */
  CallGeneric,
}

/**
 * Describes the status of an SFX
 */
export enum SfxStatus {
  /* Created, but unprocessed */
  Requested,
  /* SFX is in bidding stage */
  InBidding,
  /* SFX was dropped at bidding, refunding all participants */
  Dropped,
  /* SFX is ready to be executed */
  ReadyToExecute,
  /* SFX was executed on target successfully */
  ExecutedOnTarget,
  /* SFX was confirmed on circuit */
  Confirmed,
  /* SFX was reverted */
  Reverted,
}

export enum XtxStatus {
  /* Bidding phase in progress */
  PendingBidding,
  /* XTX was dropped at the bidding. refunding all participants */
  DroppedAtBidding,
  /* XTX is ready containing SFXs can be executed */
  ReadyToExecute,
  /* All XTX phases are confirmed, completing the lifecycle */
  FinishedAllSteps,
  /* The XTX has reverted */
  RevertTimedOut,
}

export enum SecurityLevel {
  Optimistic,
  Escrow,
}
