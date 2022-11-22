export enum RelayerEvents {
	SfxExecutedOnTarget,
	SfxExecutionError
}

export type RelayerEventData = {
    type: RelayerEvents,
    data: string,
	sfxId: string,
	target: string,
	blockNumber: number
}