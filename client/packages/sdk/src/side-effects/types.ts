export enum SfxType {
	Transfer,
}

export enum SfxStatus {
    Ready,
    PendingExecution,
    ExecutedOnTarget,
    Confirmed,
    Bidding
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
