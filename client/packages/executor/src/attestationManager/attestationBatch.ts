/**
 * Batch Confirmation Type
 *
 * @group AttestationManager
 */
type ConfirmationBatch = {
    newCommittee: string[];
    bannedCommittee: string[];
    committedSfx: string[];
    revertedSFXs: string[];
    index: number;
    expectedBatchHash: string;
    signatures: string[];
  };
  
  export type { ConfirmationBatch };
  