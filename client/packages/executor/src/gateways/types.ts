export enum RelayerEvents {
    SfxExecutedOnTarget,
    SfxExecutionError,
}

export type RelayerEventData = {
    type: RelayerEvents
    data: string
    sfxId: string
    target: string
    blockNumber: number
}

export type InclusionData = {
    encoded_payload: string,
    proof: {
        trieNodes: string,
    },
    block_hash: string,
}
