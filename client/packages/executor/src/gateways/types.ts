export enum RelayerEvents {
  SfxExecutedOnTarget,
  SfxExecutionError,
  HeaderInclusionProofRequest,
}

export type RelayerEventData = {
  type: RelayerEvents;
  data: string;
  sfxId: string;
  target: string;
  blockNumber: number;
};

/**
 * Type for creating the inclusion proof for a given SFX in substrate
 *
 * @category Substrate
 * @group Gateways
 */
export type InclusionProof = {
  encoded_payload: string;
  payload_proof: {
    trieNodes: string;
  };
  header_proof?: {
    trieNodes: string;
  };
  block_hash: string;
};
