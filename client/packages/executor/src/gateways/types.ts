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

/** Type for creating the inclusion proof for a given SFX in substrate
 * @group Gateways
 * @category Substrate
 */
export type InclusionData = {
    encoded_payload: string,
    proof: {
        trieNodes: string,
    },
    block_hash: string,
}
