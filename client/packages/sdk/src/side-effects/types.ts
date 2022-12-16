export enum SfxType {
	Transfer,
}

export enum SfxStatus {
    Ready,
    Bidding,
    PendingExecution,
    ExecutedOnTarget,
    Confirmed,
    Dropped,
    Reverted
}

export enum XtxStatus {
    PendingBidding,
    Ready,
    FinishedAllSteps,
    DroppedAtBidding,
    RevertTimedOut,
}

export enum SecurityLevel {
    Optimistic,
    Escrow
}
