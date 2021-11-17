import { SideEffect } from './../../../types/src/interfaces/primitives/types';
import { XtxId } from './../../../types/src/interfaces/execution_delivery/types';
import type { Hash, AccountId } from '@polkadot/types/interfaces/runtime';
import type { Vec } from '@polkadot/types';
import events from 'events';

export interface TransactionResult {
  blockHash: Hash;
  status: boolean;
}

export interface NewSideEffectsAvailableEvent {
  requester: AccountId,
  xtx_id: XtxId,
  sideEffects: Vec<SideEffect>
}

export declare interface Emitter {
  on(event: 'NewSideEffect', listener: (payload: NewSideEffectsAvailableEvent) => void): this;
}

export class Emitter extends events.EventEmitter {
  emitSideEffect(payload: NewSideEffectsAvailableEvent): void {
    this.emit('NewSideEffect', payload);
  }
}