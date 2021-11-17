import { SideEffect } from './../../../types/src/interfaces/primitives/types';
import { XtxId } from './../../../types/src/interfaces/execution_delivery/types';
import type { Hash, AccountId } from '@polkadot/types/interfaces/runtime';
import type { Vec } from '@polkadot/types';
export interface TransactionResult {
  blockHash: Hash;
  status: boolean;
}

export interface NewSideEffectsAvailableEvent
{
  requester: AccountId,
  xtx_id: XtxId,
  sideEffects: Vec<SideEffect>
}