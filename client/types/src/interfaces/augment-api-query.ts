// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

import type { ApiTypes } from '@polkadot/api/types';
import type { Bytes, Option, U256, Vec, bool, u32, u64 } from '@polkadot/types';
import type { AccountData, BalanceLock, ReserveData } from '@polkadot/types/interfaces/balances';
import type { BeefyNextAuthoritySet } from '@polkadot/types/interfaces/beefy';
import type { BridgedBlockHash, BridgedHeader, ChainId, InboundLaneData, LaneId, MessageData, MessageKey, OperatingMode, OutboundLaneData } from '@polkadot/types/interfaces/bridges';
import type { CodeHash, ContractInfo, DeletedContract, PrefabWasmModule } from '@polkadot/types/interfaces/contracts';
import type { AuthoritySet, SetId, StoredPendingChange, StoredState } from '@polkadot/types/interfaces/grandpa';
import type { AccountId, Balance, BlockNumber, H160, H256, Hash, KeyTypeId, Moment, Releases, ValidatorId } from '@polkadot/types/interfaces/runtime';
import type { Keys, SessionIndex } from '@polkadot/types/interfaces/session';
import type { AccountInfo, ConsumedWeight, DigestOf, EventIndex, EventRecord, LastRuntimeUpgradeInfo, Phase } from '@polkadot/types/interfaces/system';
import type { Multiplier } from '@polkadot/types/interfaces/txpayment';
import type { AnyNumber, ITuple, Observable } from '@polkadot/types/types';
import type { RegistryContract, RegistryContractId } from 't3rn-circuit-typegen/interfaces/contracts_registry';
import type { Xtx, XtxId } from 't3rn-circuit-typegen/interfaces/execution_delivery';
import type { XdnsRecord, XdnsRecordId } from 't3rn-circuit-typegen/interfaces/xdns';

declare module '@polkadot/api/types/storage' {
  export interface AugmentedQueries<ApiType> {
    balances: {
      /**
       * The balance of an account.
       * 
       * NOTE: This is only used in the case that this pallet is used to store balances.
       **/
      account: AugmentedQuery<ApiType, (arg: AccountId | string | Uint8Array) => Observable<AccountData>, [AccountId]> & QueryableStorageEntry<ApiType, [AccountId]>;
      /**
       * Any liquidity locks on some account balances.
       * NOTE: Should only be accessed when setting, changing and freeing a lock.
       **/
      locks: AugmentedQuery<ApiType, (arg: AccountId | string | Uint8Array) => Observable<Vec<BalanceLock>>, [AccountId]> & QueryableStorageEntry<ApiType, [AccountId]>;
      /**
       * Named reserves on some account balances.
       **/
      reserves: AugmentedQuery<ApiType, (arg: AccountId | string | Uint8Array) => Observable<Vec<ReserveData>>, [AccountId]> & QueryableStorageEntry<ApiType, [AccountId]>;
      /**
       * Storage version of the pallet.
       * 
       * This is set to v2.0.0 for new networks.
       **/
      storageVersion: AugmentedQuery<ApiType, () => Observable<Releases>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * The total units issued in the system.
       **/
      totalIssuance: AugmentedQuery<ApiType, () => Observable<Balance>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    bridgeGatewayGrandpa: {
      /**
       * Hash of the best finalized header.
       **/
      bestFinalized: AugmentedQuery<ApiType, () => Observable<BridgedBlockHash>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * The current GRANDPA Authority set.
       **/
      currentAuthoritySet: AugmentedQuery<ApiType, () => Observable<AuthoritySet>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * A ring buffer of imported hashes. Ordered by the insertion time.
       **/
      importedHashes: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<BridgedBlockHash>>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * Current ring buffer position.
       **/
      importedHashesPointer: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Headers which have been imported into the pallet.
       **/
      importedHeaders: AugmentedQuery<ApiType, (arg: BridgedBlockHash | string | Uint8Array) => Observable<Option<BridgedHeader>>, [BridgedBlockHash]> & QueryableStorageEntry<ApiType, [BridgedBlockHash]>;
      /**
       * Hash of the header used to bootstrap the pallet.
       **/
      initialHash: AugmentedQuery<ApiType, () => Observable<BridgedBlockHash>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * If true, all pallet transactions are failed immediately.
       **/
      isHalted: AugmentedQuery<ApiType, () => Observable<bool>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Optional pallet owner.
       * 
       * Pallet owner has a right to halt all pallet operations and then resume it. If it is
       * `None`, then there are no direct ways to halt/resume pallet operations, but other
       * runtime methods may still be used to do that (i.e. democracy::referendum to update halt
       * flag directly or call the `halt_operations`).
       **/
      palletOwner: AugmentedQuery<ApiType, () => Observable<Option<AccountId>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * The current number of requests which have written to storage.
       * 
       * If the `RequestCount` hits `MaxRequests`, no more calls will be allowed to the pallet until
       * the request capacity is increased.
       * 
       * The `RequestCount` is decreased by one at the beginning of every block. This is to ensure
       * that the pallet can always make progress.
       **/
      requestCount: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    bridgeGatewayMessages: {
      /**
       * Map of lane id => inbound lane data.
       **/
      inboundLanes: AugmentedQuery<ApiType, (arg: LaneId | string | Uint8Array) => Observable<InboundLaneData>, [LaneId]> & QueryableStorageEntry<ApiType, [LaneId]>;
      /**
       * Map of lane id => outbound lane data.
       **/
      outboundLanes: AugmentedQuery<ApiType, (arg: LaneId | string | Uint8Array) => Observable<OutboundLaneData>, [LaneId]> & QueryableStorageEntry<ApiType, [LaneId]>;
      /**
       * All queued outbound messages.
       **/
      outboundMessages: AugmentedQuery<ApiType, (arg: MessageKey | { laneId?: any; nonce?: any } | string | Uint8Array) => Observable<Option<MessageData>>, [MessageKey]> & QueryableStorageEntry<ApiType, [MessageKey]>;
      /**
       * The current operating mode of the pallet.
       * 
       * Depending on the mode either all, some, or no transactions will be allowed.
       **/
      palletOperatingMode: AugmentedQuery<ApiType, () => Observable<OperatingMode>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Optional pallet owner.
       * 
       * Pallet owner has a right to halt all pallet operations and then resume it. If it is
       * `None`, then there are no direct ways to halt/resume pallet operations, but other
       * runtime methods may still be used to do that (i.e. democracy::referendum to update halt
       * flag directly or call the `halt_operations`).
       **/
      palletOwner: AugmentedQuery<ApiType, () => Observable<Option<AccountId>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    bridgePolkadotLikeMultiFinalityVerifier: {
      /**
       * Map of hashes of the best finalized header.
       **/
      bestFinalizedMap: AugmentedQuery<ApiType, (arg: ChainId | string | Uint8Array) => Observable<Option<BridgedBlockHash>>, [ChainId]> & QueryableStorageEntry<ApiType, [ChainId]>;
      /**
       * The current GRANDPA Authority set map.
       **/
      currentAuthoritySetMap: AugmentedQuery<ApiType, (arg: ChainId | string | Uint8Array) => Observable<Option<AuthoritySet>>, [ChainId]> & QueryableStorageEntry<ApiType, [ChainId]>;
      /**
       * Hash of the header used to bootstrap the pallet.
       **/
      initialHashMap: AugmentedQuery<ApiType, (arg: ChainId | string | Uint8Array) => Observable<Option<BridgedBlockHash>>, [ChainId]> & QueryableStorageEntry<ApiType, [ChainId]>;
      /**
       * Map of instance ids of gateways which are active
       **/
      instantiatedGatewaysMap: AugmentedQuery<ApiType, () => Observable<Vec<ChainId>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * If true, all pallet transactions are failed immediately.
       **/
      isHalted: AugmentedQuery<ApiType, () => Observable<bool>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * If true, all pallet transactions are failed immediately.
       **/
      isHaltedMap: AugmentedQuery<ApiType, (arg: ChainId | string | Uint8Array) => Observable<Option<bool>>, [ChainId]> & QueryableStorageEntry<ApiType, [ChainId]>;
      /**
       * A ring buffer of imported hashes. Ordered by the insertion time.
       **/
      multiImportedHashes: AugmentedQuery<ApiType, (arg1: ChainId | string | Uint8Array, arg2: u32 | AnyNumber | Uint8Array) => Observable<Option<BridgedBlockHash>>, [ChainId, u32]> & QueryableStorageEntry<ApiType, [ChainId, u32]>;
      /**
       * Current ring buffer position.
       **/
      multiImportedHashesPointer: AugmentedQuery<ApiType, (arg: ChainId | string | Uint8Array) => Observable<Option<u32>>, [ChainId]> & QueryableStorageEntry<ApiType, [ChainId]>;
      /**
       * Headers which have been imported into the pallet.
       **/
      multiImportedHeaders: AugmentedQuery<ApiType, (arg1: ChainId | string | Uint8Array, arg2: BridgedBlockHash | string | Uint8Array) => Observable<Option<BridgedHeader>>, [ChainId, BridgedBlockHash]> & QueryableStorageEntry<ApiType, [ChainId, BridgedBlockHash]>;
      /**
       * Roots (ExtrinsicsRoot + StateRoot) which have been imported into the pallet for a given gateway.
       **/
      multiImportedRoots: AugmentedQuery<ApiType, (arg1: ChainId | string | Uint8Array, arg2: BridgedBlockHash | string | Uint8Array) => Observable<Option<ITuple<[BridgedBlockHash, BridgedBlockHash]>>>, [ChainId, BridgedBlockHash]> & QueryableStorageEntry<ApiType, [ChainId, BridgedBlockHash]>;
      /**
       * Optional pallet owner.
       * 
       * Pallet owner has a right to halt all pallet operations and then resume it. If it is
       * `None`, then there are no direct ways to halt/resume pallet operations, but other
       * runtime methods may still be used to do that (i.e. democracy::referendum to update halt
       * flag directly or call the `halt_operations`).
       **/
      palletOwner: AugmentedQuery<ApiType, () => Observable<Option<AccountId>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Optional pallet owner.
       * 
       * Pallet owner has a right to halt all pallet operations and then resume it. If it is
       * `None`, then there are no direct ways to halt/resume pallet operations, but other
       * runtime methods may still be used to do that (i.e. democracy::referendum to update halt
       * flag directly or call the `halt_operations`).
       **/
      palletOwnerMap: AugmentedQuery<ApiType, (arg: ChainId | string | Uint8Array) => Observable<Option<AccountId>>, [ChainId]> & QueryableStorageEntry<ApiType, [ChainId]>;
      /**
       * The current number of requests which have written to storage.
       * 
       * If the `RequestCount` hits `MaxRequests`, no more calls will be allowed to the pallet until
       * the request capacity is increased.
       * 
       * The `RequestCount` is decreased by one at the beginning of every block. This is to ensure
       * that the pallet can always make progress.
       **/
      requestCountMap: AugmentedQuery<ApiType, (arg: ChainId | string | Uint8Array) => Observable<Option<u32>>, [ChainId]> & QueryableStorageEntry<ApiType, [ChainId]>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    contracts: {
      /**
       * The subtrie counter.
       **/
      accountCounter: AugmentedQuery<ApiType, () => Observable<u64>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * A mapping between an original code hash and instrumented wasm code, ready for execution.
       **/
      codeStorage: AugmentedQuery<ApiType, (arg: CodeHash | string | Uint8Array) => Observable<Option<PrefabWasmModule>>, [CodeHash]> & QueryableStorageEntry<ApiType, [CodeHash]>;
      /**
       * The code associated with a given account.
       * 
       * TWOX-NOTE: SAFE since `AccountId` is a secure hash.
       **/
      contractInfoOf: AugmentedQuery<ApiType, (arg: AccountId | string | Uint8Array) => Observable<Option<ContractInfo>>, [AccountId]> & QueryableStorageEntry<ApiType, [AccountId]>;
      /**
       * Evicted contracts that await child trie deletion.
       * 
       * Child trie deletion is a heavy operation depending on the amount of storage items
       * stored in said trie. Therefore this operation is performed lazily in `on_initialize`.
       **/
      deletionQueue: AugmentedQuery<ApiType, () => Observable<Vec<DeletedContract>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * A mapping from an original code hash to the original code, untouched by instrumentation.
       **/
      pristineCode: AugmentedQuery<ApiType, (arg: CodeHash | string | Uint8Array) => Observable<Option<Bytes>>, [CodeHash]> & QueryableStorageEntry<ApiType, [CodeHash]>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    contractsRegistry: {
      /**
       * The pre-validated composable contracts on-chain registry.
       **/
      contractsRegistry: AugmentedQuery<ApiType, (arg: RegistryContractId | string | Uint8Array) => Observable<Option<RegistryContract>>, [RegistryContractId]> & QueryableStorageEntry<ApiType, [RegistryContractId]>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    ethereumLightClient: {
      /**
       * Best known block.
       **/
      bestBlock: AugmentedQuery<ApiType, () => Observable<ITuple<[EthereumHeaderId, U256]>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Range of blocks that we want to prune.
       **/
      blocksToPrune: AugmentedQuery<ApiType, () => Observable<PruningRange>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Best finalized block.
       **/
      finalizedBlock: AugmentedQuery<ApiType, () => Observable<EthereumHeaderId>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Map of imported headers by hash.
       **/
      headers: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Option<StoredHeader>>, [H256]> & QueryableStorageEntry<ApiType, [H256]>;
      /**
       * Map of imported header hashes by number.
       **/
      headersByNumber: AugmentedQuery<ApiType, (arg: u64 | AnyNumber | Uint8Array) => Observable<Option<Vec<H256>>>, [u64]> & QueryableStorageEntry<ApiType, [u64]>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    evm: {
      accountCodes: AugmentedQuery<ApiType, (arg: H160 | string | Uint8Array) => Observable<Bytes>, [H160]> & QueryableStorageEntry<ApiType, [H160]>;
      accountStorages: AugmentedQuery<ApiType, (arg1: H160 | string | Uint8Array, arg2: H256 | string | Uint8Array) => Observable<H256>, [H160, H256]> & QueryableStorageEntry<ApiType, [H160, H256]>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    execDelivery: {
      /**
       * Current Circuit's context of active transactions
       * 
       * The currently active composable transactions, indexed according to the order of creation.
       **/
      activeXtxMap: AugmentedQuery<ApiType, (arg: XtxId | string | Uint8Array) => Observable<Option<Xtx>>, [XtxId]> & QueryableStorageEntry<ApiType, [XtxId]>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    grandpa: {
      /**
       * The number of changes (both in terms of keys and underlying economic responsibilities)
       * in the "set" of Grandpa validators from genesis.
       **/
      currentSetId: AugmentedQuery<ApiType, () => Observable<SetId>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * next block number where we can force a change.
       **/
      nextForced: AugmentedQuery<ApiType, () => Observable<Option<BlockNumber>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Pending change: (signaled at, scheduled change).
       **/
      pendingChange: AugmentedQuery<ApiType, () => Observable<Option<StoredPendingChange>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * A mapping from grandpa set ID to the index of the *most recent* session for which its
       * members were responsible.
       * 
       * TWOX-NOTE: `SetId` is not under user control.
       **/
      setIdSession: AugmentedQuery<ApiType, (arg: SetId | AnyNumber | Uint8Array) => Observable<Option<SessionIndex>>, [SetId]> & QueryableStorageEntry<ApiType, [SetId]>;
      /**
       * `true` if we are currently stalled.
       **/
      stalled: AugmentedQuery<ApiType, () => Observable<Option<ITuple<[BlockNumber, BlockNumber]>>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * State of the current authority set.
       **/
      state: AugmentedQuery<ApiType, () => Observable<StoredState>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    mmr: {
      /**
       * Hashes of the nodes in the MMR.
       * 
       * Note this collection only contains MMR peaks, the inner nodes (and leaves)
       * are pruned and only stored in the Offchain DB.
       **/
      nodes: AugmentedQuery<ApiType, (arg: u64 | AnyNumber | Uint8Array) => Observable<Option<Hash>>, [u64]> & QueryableStorageEntry<ApiType, [u64]>;
      /**
       * Current size of the MMR (number of leaves).
       **/
      numberOfLeaves: AugmentedQuery<ApiType, () => Observable<u64>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Latest MMR Root hash.
       **/
      rootHash: AugmentedQuery<ApiType, () => Observable<Hash>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    mmrLeaf: {
      /**
       * Details of next BEEFY authority set.
       * 
       * This storage entry is used as cache for calls to [`update_beefy_next_authority_set`].
       **/
      beefyNextAuthorities: AugmentedQuery<ApiType, () => Observable<BeefyNextAuthoritySet>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    randomness: {
      /**
       * Series of block headers from the last 81 blocks that acts as random seed material. This
       * is arranged as a ring buffer with `block_number % 81` being the index into the `Vec` of
       * the oldest hash.
       **/
      randomMaterial: AugmentedQuery<ApiType, () => Observable<Vec<Hash>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    session: {
      /**
       * Current index of the session.
       **/
      currentIndex: AugmentedQuery<ApiType, () => Observable<SessionIndex>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Indices of disabled validators.
       * 
       * The set is cleared when `on_session_ending` returns a new set of identities.
       **/
      disabledValidators: AugmentedQuery<ApiType, () => Observable<Vec<u32>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * The owner of a key. The key is the `KeyTypeId` + the encoded key.
       **/
      keyOwner: AugmentedQuery<ApiType, (arg: ITuple<[KeyTypeId, Bytes]> | [KeyTypeId | AnyNumber | Uint8Array, Bytes | string | Uint8Array]) => Observable<Option<ValidatorId>>, [ITuple<[KeyTypeId, Bytes]>]> & QueryableStorageEntry<ApiType, [ITuple<[KeyTypeId, Bytes]>]>;
      /**
       * The next session keys for a validator.
       **/
      nextKeys: AugmentedQuery<ApiType, (arg: ValidatorId | string | Uint8Array) => Observable<Option<Keys>>, [ValidatorId]> & QueryableStorageEntry<ApiType, [ValidatorId]>;
      /**
       * True if the underlying economic identities or weighting behind the validators
       * has changed in the queued validator set.
       **/
      queuedChanged: AugmentedQuery<ApiType, () => Observable<bool>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * The queued keys for the next session. When the next session begins, these keys
       * will be used to determine the validator's session keys.
       **/
      queuedKeys: AugmentedQuery<ApiType, () => Observable<Vec<ITuple<[ValidatorId, Keys]>>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * The current set of validators.
       **/
      validators: AugmentedQuery<ApiType, () => Observable<Vec<ValidatorId>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    sudo: {
      /**
       * The `AccountId` of the sudo key.
       **/
      key: AugmentedQuery<ApiType, () => Observable<AccountId>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    system: {
      /**
       * The full account information for a particular account ID.
       **/
      account: AugmentedQuery<ApiType, (arg: AccountId | string | Uint8Array) => Observable<AccountInfo>, [AccountId]> & QueryableStorageEntry<ApiType, [AccountId]>;
      /**
       * Total length (in bytes) for all extrinsics put together, for the current block.
       **/
      allExtrinsicsLen: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Map of block numbers to block hashes.
       **/
      blockHash: AugmentedQuery<ApiType, (arg: BlockNumber | AnyNumber | Uint8Array) => Observable<Hash>, [BlockNumber]> & QueryableStorageEntry<ApiType, [BlockNumber]>;
      /**
       * The current weight for the block.
       **/
      blockWeight: AugmentedQuery<ApiType, () => Observable<ConsumedWeight>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Digest of the current block, also part of the block header.
       **/
      digest: AugmentedQuery<ApiType, () => Observable<DigestOf>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * The number of events in the `Events<T>` list.
       **/
      eventCount: AugmentedQuery<ApiType, () => Observable<EventIndex>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Events deposited for the current block.
       **/
      events: AugmentedQuery<ApiType, () => Observable<Vec<EventRecord>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Mapping between a topic (represented by T::Hash) and a vector of indexes
       * of events in the `<Events<T>>` list.
       * 
       * All topic vectors have deterministic storage locations depending on the topic. This
       * allows light-clients to leverage the changes trie storage tracking mechanism and
       * in case of changes fetch the list of events of interest.
       * 
       * The value has the type `(T::BlockNumber, EventIndex)` because if we used only just
       * the `EventIndex` then in case if the topic has the same contents on the next block
       * no notification will be triggered thus the event might be lost.
       **/
      eventTopics: AugmentedQuery<ApiType, (arg: Hash | string | Uint8Array) => Observable<Vec<ITuple<[BlockNumber, EventIndex]>>>, [Hash]> & QueryableStorageEntry<ApiType, [Hash]>;
      /**
       * The execution phase of the block.
       **/
      executionPhase: AugmentedQuery<ApiType, () => Observable<Option<Phase>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Total extrinsics count for the current block.
       **/
      extrinsicCount: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Extrinsics data for the current block (maps an extrinsic's index to its data).
       **/
      extrinsicData: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Bytes>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * Stores the `spec_version` and `spec_name` of when the last runtime upgrade happened.
       **/
      lastRuntimeUpgrade: AugmentedQuery<ApiType, () => Observable<Option<LastRuntimeUpgradeInfo>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * The current block number being processed. Set by `execute_block`.
       **/
      number: AugmentedQuery<ApiType, () => Observable<BlockNumber>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Hash of the previous block.
       **/
      parentHash: AugmentedQuery<ApiType, () => Observable<Hash>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * True if we have upgraded so that AccountInfo contains three types of `RefCount`. False
       * (default) if not.
       **/
      upgradedToTripleRefCount: AugmentedQuery<ApiType, () => Observable<bool>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * True if we have upgraded so that `type RefCount` is `u32`. False (default) if not.
       **/
      upgradedToU32RefCount: AugmentedQuery<ApiType, () => Observable<bool>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    timestamp: {
      /**
       * Did the timestamp get updated in this block?
       **/
      didUpdate: AugmentedQuery<ApiType, () => Observable<bool>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Current time for the current block.
       **/
      now: AugmentedQuery<ApiType, () => Observable<Moment>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    transactionPayment: {
      nextFeeMultiplier: AugmentedQuery<ApiType, () => Observable<Multiplier>, []> & QueryableStorageEntry<ApiType, []>;
      storageVersion: AugmentedQuery<ApiType, () => Observable<Releases>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    volatileVm: {
      /**
       * The subtrie counter.
       **/
      accountCounter: AugmentedQuery<ApiType, () => Observable<u64>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * The code associated with a given account.
       * 
       * TWOX-NOTE: SAFE since `AccountId` is a secure hash.
       **/
      contractInfoOf: AugmentedQuery<ApiType, (arg: AccountId | string | Uint8Array) => Observable<Option<ContractInfo>>, [AccountId]> & QueryableStorageEntry<ApiType, [AccountId]>;
      /**
       * Declared by purchaser gateway foreign targets associated with target addresses (to)
       * None for targets at Circuit.
       **/
      declaredTargets: AugmentedQuery<ApiType, (arg: AccountId | string | Uint8Array) => Observable<Option<TargetId>>, [AccountId]> & QueryableStorageEntry<ApiType, [AccountId]>;
      /**
       * Evicted contracts that await child trie deletion.
       * 
       * Child trie deletion is a heavy operation depending on the amount of storage items
       * stored in said trie. Therefore this operation is performed lazily in `on_initialize`.
       **/
      deletionQueue: AugmentedQuery<ApiType, () => Observable<Vec<DeletedContract>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * A mapping between an original code hash and instrumented wasm code, ready for execution.
       **/
      dryRunCodeCandidates: AugmentedQuery<ApiType, (arg: CodeHash | string | Uint8Array) => Observable<Option<PrefabWasmModule>>, [CodeHash]> & QueryableStorageEntry<ApiType, [CodeHash]>;
      /**
       * A mapping from an original code hash to the original code, untouched by instrumentation.
       **/
      pristineCode: AugmentedQuery<ApiType, (arg: CodeHash | string | Uint8Array) => Observable<Option<Bytes>>, [CodeHash]> & QueryableStorageEntry<ApiType, [CodeHash]>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    xdns: {
      /**
       * The pre-validated composable xdns_records on-chain registry.
       **/
      xdnsRegistry: AugmentedQuery<ApiType, (arg: XdnsRecordId | {  } | string | Uint8Array) => Observable<Option<XdnsRecord>>, [XdnsRecordId]> & QueryableStorageEntry<ApiType, [XdnsRecordId]>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
  }

  export interface QueryableStorage<ApiType extends ApiTypes> extends AugmentedQueries<ApiType> {
    [key: string]: QueryableModuleStorage<ApiType>;
  }
}
