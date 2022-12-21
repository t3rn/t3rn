/**
 * Enum describing the different SFX types that are available.
 */
export enum SfxType {
    /* Native Transfer on Target */
	Transfer,
}

export enum SfxStatus {
    Requested,
    InBidding,
    Dropped,
    ReadyToExecute,
    ExecutedOnTarget,
    Confirmed,
    Reverted
}

export enum XtxStatus {
    PendingBidding,
    DroppedAtBidding,
    ReadyToExecute,
    FinishedAllSteps,
    RevertTimedOut,
}

export enum SecurityLevel {
    Optimistic,
    Escrow
}
