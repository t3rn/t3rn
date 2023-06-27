/**
 * Batch Confirmation Type
 *
 * @group AttestationManager
 */
type ConfirmationBatch = Batch & {
  messageHash: string;
  signatures: string[];
};

type Batch = {
  nextCommittee: string[];
  bannedCommittee: string[];
  committedSfx: string[];
  revertedSfx: string[];
  index: number;
};

export type { ConfirmationBatch, Batch };
