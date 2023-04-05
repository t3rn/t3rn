// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

import type { ApiTypes } from '@polkadot/api-base/types';
import type { Data } from '@polkadot/types';
import type { Bytes, Option, U8aFixed, Vec, bool, u128, u32, u64, u8 } from '@polkadot/types-codec';
import type { AnyNumber, ITuple } from '@polkadot/types-codec/types';
import type { AccountId32, H160, H256 } from '@polkadot/types/interfaces/runtime';
import type { FrameSupportWeightsPerDispatchClassU64, FrameSystemAccountInfo, FrameSystemEventRecord, FrameSystemLastRuntimeUpgradeInfo, FrameSystemPhase, PalletAssetsApproval, PalletAssetsAssetAccount, PalletAssetsAssetDetails, PalletAssetsAssetMetadata, PalletAuthorshipUncleEntryItem, PalletBalancesAccountData, PalletBalancesBalanceLock, PalletBalancesReleases, PalletBalancesReserveData, PalletCircuitStateXExecSignal, PalletContractsStorageDeletedContract, PalletContractsStorageRawContractInfo, PalletContractsWasmOwnerInfo, PalletContractsWasmPrefabWasmModule, PalletEvmThreeVmInfo, PalletGrandpaFinalityVerifierBridgesHeaderChainAuthoritySet, PalletGrandpaFinalityVerifierParachainRegistrationData, PalletGrandpaStoredPendingChange, PalletGrandpaStoredState, PalletIdentityRegistrarInfo, PalletIdentityRegistration, PalletTransactionPaymentReleases, PalletTreasuryProposal, SpConsensusAuraSr25519AppSr25519Public, SpRuntimeDigest, SpRuntimeHeader, T3rnAbiSfxAbi, T3rnPrimitivesAccountManagerRequestCharge, T3rnPrimitivesAccountManagerSettlement, T3rnPrimitivesClaimableClaimableArtifacts, T3rnPrimitivesCommonRoundInfo, T3rnPrimitivesContractsRegistryRegistryContract, T3rnPrimitivesVolatileLocalState, T3rnPrimitivesXdnsGatewayRecord, T3rnPrimitivesXdnsTokenRecord, T3rnPrimitivesXdnsXdnsRecord, T3rnSdkPrimitivesSignalExecutionSignal, T3rnTypesFsxFullSideEffect } from '@polkadot/types/lookup';
import type { Observable } from '@polkadot/types/types';

declare module '@polkadot/api-base/types/storage' {
  export interface AugmentedQueries<ApiType extends ApiTypes> {
    accountManager: {
      contractsRegistryExecutionNonce: AugmentedQuery<ApiType, () => Observable<u64>, []>;
      pendingCharges: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Option<T3rnPrimitivesAccountManagerRequestCharge>>, [H256]>;
      settlementsPerRound: AugmentedQuery<ApiType, (arg1: T3rnPrimitivesCommonRoundInfo | { index?: any; head?: any; term?: any } | string | Uint8Array, arg2: H256 | string | Uint8Array) => Observable<Option<T3rnPrimitivesAccountManagerSettlement>>, [T3rnPrimitivesCommonRoundInfo, H256]>;
    };
    assets: {
      /**
       * The holdings of a specific account for a specific asset.
       **/
      account: AugmentedQuery<ApiType, (arg1: u32 | AnyNumber | Uint8Array, arg2: AccountId32 | string | Uint8Array) => Observable<Option<PalletAssetsAssetAccount>>, [u32, AccountId32]>;
      /**
       * Approved balance transfers. First balance is the amount approved for transfer. Second
       * is the amount of `T::Currency` reserved for storing this.
       * First key is the asset ID, second key is the owner and third key is the delegate.
       **/
      approvals: AugmentedQuery<ApiType, (arg1: u32 | AnyNumber | Uint8Array, arg2: AccountId32 | string | Uint8Array, arg3: AccountId32 | string | Uint8Array) => Observable<Option<PalletAssetsApproval>>, [u32, AccountId32, AccountId32]>;
      /**
       * Details of an asset.
       **/
      asset: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<PalletAssetsAssetDetails>>, [u32]>;
      /**
       * Metadata of an asset.
       **/
      metadata: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<PalletAssetsAssetMetadata>, [u32]>;
    };
    aura: {
      /**
       * The current authority set.
       **/
      authorities: AugmentedQuery<ApiType, () => Observable<Vec<SpConsensusAuraSr25519AppSr25519Public>>, []>;
      /**
       * The current slot of this block.
       * 
       * This will be set in `on_initialize`.
       **/
      currentSlot: AugmentedQuery<ApiType, () => Observable<u64>, []>;
    };
    authorship: {
      /**
       * Author of current block.
       **/
      author: AugmentedQuery<ApiType, () => Observable<Option<AccountId32>>, []>;
      /**
       * Whether uncles were already set in this block.
       **/
      didSetUncles: AugmentedQuery<ApiType, () => Observable<bool>, []>;
      /**
       * Uncles
       **/
      uncles: AugmentedQuery<ApiType, () => Observable<Vec<PalletAuthorshipUncleEntryItem>>, []>;
    };
    balances: {
      /**
       * The Balances pallet example of storing the balance of an account.
       * 
       * # Example
       * 
       * ```nocompile
       * impl pallet_balances::Config for Runtime {
       * type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>
       * }
       * ```
       * 
       * You can also store the balance of an account in the `System` pallet.
       * 
       * # Example
       * 
       * ```nocompile
       * impl pallet_balances::Config for Runtime {
       * type AccountStore = System
       * }
       * ```
       * 
       * But this comes with tradeoffs, storing account balances in the system pallet stores
       * `frame_system` data alongside the account data contrary to storing account balances in the
       * `Balances` pallet, which uses a `StorageMap` to store balances data only.
       * NOTE: This is only used in the case that this pallet is used to store balances.
       **/
      account: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<PalletBalancesAccountData>, [AccountId32]>;
      /**
       * Any liquidity locks on some account balances.
       * NOTE: Should only be accessed when setting, changing and freeing a lock.
       **/
      locks: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<Vec<PalletBalancesBalanceLock>>, [AccountId32]>;
      /**
       * Named reserves on some account balances.
       **/
      reserves: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<Vec<PalletBalancesReserveData>>, [AccountId32]>;
      /**
       * Storage version of the pallet.
       * 
       * This is set to v2.0.0 for new networks.
       **/
      storageVersion: AugmentedQuery<ApiType, () => Observable<PalletBalancesReleases>, []>;
      /**
       * The total units issued in the system.
       **/
      totalIssuance: AugmentedQuery<ApiType, () => Observable<u128>, []>;
    };
    circuit: {
      /**
       * Current Circuit's context of active full side effects (requested + confirmation proofs)
       * Lifecycle tips:
       * FSX entries are created at the time of Xtx submission, where still uncertain whether Xtx will be accepted
       * for execution (picked up in the bidding process).
       * - @Circuit::Requested: create FSX array without confirmations or bids
       * - @Circuit::Bonded -> Ready: add bids to FSX
       * - @Circuit::PendingExecution -> add more confirmations at receipt
       * 
       * If no bids have been received @Circuit::PendingBidding, FSX entries will stay - just without the Bid.
       * The details on Xtx status might be played back by looking up with the SFX2XTXLinksMap
       **/
      fullSideEffects: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Option<Vec<Vec<T3rnTypesFsxFullSideEffect>>>>, [H256]>;
      /**
       * LocalXtxStates stores the map of LocalState - additional state to be used to communicate between SFX that belong to the same Xtx
       * 
       * - @Circuit::Requested: create LocalXtxStates array without confirmations or bids
       * - @Circuit::PendingExecution: entries to LocalState can be updated.
       * If no bids have been received @Circuit::PendingBidding, LocalXtxStates entries are removed since Xtx won't be executed
       **/
      localXtxStates: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Option<T3rnPrimitivesVolatileLocalState>>, [H256]>;
      /**
       * Temporary bidding timeouts map for SFX executions. Cleaned out each Config::BidsInterval,
       * where for each FSX::best_bid bidders are assigned for SFX::enforce_executor or Xtx is dropped.
       **/
      pendingXtxBidsTimeoutsMap: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Option<u32>>, [H256]>;
      /**
       * Current Circuit's context of active Xtx used for the on_initialize clock to discover
       * the ones pending for execution too long, that eventually need to be killed
       * 
       **/
      pendingXtxTimeoutsMap: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Option<u32>>, [H256]>;
      /**
       * Links mapping SFX 2 XTX
       * 
       **/
      sfx2xtxLinksMap: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Option<H256>>, [H256]>;
      /**
       * Handles queued signals
       * 
       * This operation is performed lazily in `on_initialize`.
       **/
      signalQueue: AugmentedQuery<ApiType, () => Observable<Vec<ITuple<[AccountId32, T3rnSdkPrimitivesSignalExecutionSignal]>>>, []>;
      storageMigrations: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      /**
       * Current Circuit's context of all accepted for execution cross-chain transactions.
       * 
       * All Xtx that has been initially paid out by users will be left here.
       * Even if the timeout has been exceeded, they will eventually end with the Circuit::RevertedTimeout
       * 
       **/
      xExecSignals: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Option<PalletCircuitStateXExecSignal>>, [H256]>;
    };
    clock: {
      claimableArtifactsPerRound: AugmentedQuery<ApiType, (arg: T3rnPrimitivesCommonRoundInfo | { index?: any; head?: any; term?: any } | string | Uint8Array) => Observable<Option<Vec<T3rnPrimitivesClaimableClaimableArtifacts>>>, [T3rnPrimitivesCommonRoundInfo]>;
      /**
       * Information on the current round.
       **/
      currentRound: AugmentedQuery<ApiType, () => Observable<T3rnPrimitivesCommonRoundInfo>, []>;
      lastClaims: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<Option<T3rnPrimitivesCommonRoundInfo>>, [AccountId32]>;
    };
    contracts: {
      /**
       * A mapping between an original code hash and instrumented wasm code, ready for execution.
       **/
      codeStorage: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Option<PalletContractsWasmPrefabWasmModule>>, [H256]>;
      /**
       * The code associated with a given account.
       * 
       * TWOX-NOTE: SAFE since `AccountId` is a secure hash.
       **/
      contractInfoOf: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<Option<PalletContractsStorageRawContractInfo>>, [AccountId32]>;
      /**
       * Evicted contracts that await child trie deletion.
       * 
       * Child trie deletion is a heavy operation depending on the amount of storage items
       * stored in said trie. Therefore this operation is performed lazily in `on_initialize`.
       **/
      deletionQueue: AugmentedQuery<ApiType, () => Observable<Vec<PalletContractsStorageDeletedContract>>, []>;
      /**
       * This is a **monotonic** counter incremented on contract instantiation.
       * 
       * This is used in order to generate unique trie ids for contracts.
       * The trie id of a new contract is calculated from hash(account_id, nonce).
       * The nonce is required because otherwise the following sequence would lead to
       * a possible collision of storage:
       * 
       * 1. Create a new contract.
       * 2. Terminate the contract.
       * 3. Immediately recreate the contract with the same account_id.
       * 
       * This is bad because the contents of a trie are deleted lazily and there might be
       * storage of the old instantiation still in it when the new contract is created. Please
       * note that we can't replace the counter by the block number because the sequence above
       * can happen in the same block. We also can't keep the account counter in memory only
       * because storage is the only way to communicate across different extrinsics in the
       * same block.
       * 
       * # Note
       * 
       * Do not use it to determine the number of contracts. It won't be decremented if
       * a contract is destroyed.
       **/
      nonce: AugmentedQuery<ApiType, () => Observable<u64>, []>;
      /**
       * A mapping between an original code hash and its owner information.
       **/
      ownerInfoOf: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Option<PalletContractsWasmOwnerInfo>>, [H256]>;
      /**
       * A mapping from an original code hash to the original code, untouched by instrumentation.
       **/
      pristineCode: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Option<Bytes>>, [H256]>;
    };
    contractsRegistry: {
      /**
       * The pre-validated composable contracts on-chain registry.
       **/
      contractsRegistry: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Option<T3rnPrimitivesContractsRegistryRegistryContract>>, [H256]>;
    };
    evm: {
      account3vmInfo: AugmentedQuery<ApiType, (arg: H160 | string | Uint8Array) => Observable<Option<PalletEvmThreeVmInfo>>, [H160]>;
      accountCodes: AugmentedQuery<ApiType, (arg: H160 | string | Uint8Array) => Observable<Bytes>, [H160]>;
      accountEvmAddressMapping: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<Option<H160>>, [AccountId32]>;
      /**
       * The storages for EVM contracts.
       * 
       * AccountStorages: double_map EvmAddress, H256 => H256
       **/
      accountStorages: AugmentedQuery<ApiType, (arg1: H160 | string | Uint8Array, arg2: H256 | string | Uint8Array) => Observable<H256>, [H160, H256]>;
      evmAccountAddressMapping: AugmentedQuery<ApiType, (arg: H160 | string | Uint8Array) => Observable<Option<AccountId32>>, [H160]>;
    };
    grandpa: {
      /**
       * The number of changes (both in terms of keys and underlying economic responsibilities)
       * in the "set" of Grandpa validators from genesis.
       **/
      currentSetId: AugmentedQuery<ApiType, () => Observable<u64>, []>;
      /**
       * next block number where we can force a change.
       **/
      nextForced: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []>;
      /**
       * Pending change: (signaled at, scheduled change).
       **/
      pendingChange: AugmentedQuery<ApiType, () => Observable<Option<PalletGrandpaStoredPendingChange>>, []>;
      /**
       * A mapping from grandpa set ID to the index of the *most recent* session for which its
       * members were responsible.
       * 
       * TWOX-NOTE: `SetId` is not under user control.
       **/
      setIdSession: AugmentedQuery<ApiType, (arg: u64 | AnyNumber | Uint8Array) => Observable<Option<u32>>, [u64]>;
      /**
       * `true` if we are currently stalled.
       **/
      stalled: AugmentedQuery<ApiType, () => Observable<Option<ITuple<[u32, u32]>>>, []>;
      /**
       * State of the current authority set.
       **/
      state: AugmentedQuery<ApiType, () => Observable<PalletGrandpaStoredState>, []>;
    };
    identity: {
      /**
       * Information that is pertinent to identify the entity behind an account.
       * 
       * TWOX-NOTE: OK ― `AccountId` is a secure hash.
       **/
      identityOf: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<Option<PalletIdentityRegistration>>, [AccountId32]>;
      /**
       * The set of registrars. Not expected to get very big as can only be added through a
       * special origin (likely a council motion).
       * 
       * The index into this can be cast to `RegistrarIndex` to get a valid value.
       **/
      registrars: AugmentedQuery<ApiType, () => Observable<Vec<Option<PalletIdentityRegistrarInfo>>>, []>;
      /**
       * Alternative "sub" identities of this account.
       * 
       * The first item is the deposit, the second is a vector of the accounts.
       * 
       * TWOX-NOTE: OK ― `AccountId` is a secure hash.
       **/
      subsOf: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<ITuple<[u128, Vec<AccountId32>]>>, [AccountId32]>;
      /**
       * The super-identity of an alternative "sub" identity together with its name, within that
       * context. If the account is not some other account's sub-identity, then just `None`.
       **/
      superOf: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<Option<ITuple<[AccountId32, Data]>>>, [AccountId32]>;
    };
    kusamaBridge: {
      /**
       * Hash of the best finalized header.
       **/
      bestFinalizedHash: AugmentedQuery<ApiType, () => Observable<Option<H256>>, []>;
      /**
       * The current GRANDPA Authority set.
       **/
      currentAuthoritySet: AugmentedQuery<ApiType, () => Observable<Option<PalletGrandpaFinalityVerifierBridgesHeaderChainAuthoritySet>>, []>;
      /**
       * If true, all pallet transactions are failed immediately.
       **/
      everInitialized: AugmentedQuery<ApiType, () => Observable<bool>, []>;
      /**
       * A ring buffer of imported hashes. Ordered by the insertion time.
       **/
      importedHashes: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<H256>>, [u32]>;
      /**
       * Current ring buffer position.
       **/
      importedHashesPointer: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []>;
      /**
       * Headers which have been imported into the pallet.
       **/
      importedHeaders: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Option<SpRuntimeHeader>>, [H256]>;
      /**
       * Hash of the header used to bootstrap the pallet.
       **/
      initialHash: AugmentedQuery<ApiType, () => Observable<Option<H256>>, []>;
      /**
       * If true, all pallet transactions are failed immediately.
       **/
      isHalted: AugmentedQuery<ApiType, () => Observable<bool>, []>;
      /**
       * Optional pallet owner.
       * 
       * Pallet owner has a right to halt all pallet operations and then resume it. If it is
       * `None`, then there are no direct ways to halt/resume pallet operations, but other
       * runtime methods may still be used to do that (i.e. democracy::referendum to update halt
       * flag directly or call the `halt_operations`).
       **/
      palletOwner: AugmentedQuery<ApiType, () => Observable<Option<AccountId32>>, []>;
      /**
       * Maps a parachain chain_id to the corresponding chain ID.
       **/
      parachainIdMap: AugmentedQuery<ApiType, (arg: U8aFixed | string | Uint8Array) => Observable<Option<PalletGrandpaFinalityVerifierParachainRegistrationData>>, [U8aFixed]>;
      relayChainId: AugmentedQuery<ApiType, () => Observable<Option<U8aFixed>>, []>;
    };
    polkadotBridge: {
      /**
       * Hash of the best finalized header.
       **/
      bestFinalizedHash: AugmentedQuery<ApiType, () => Observable<Option<H256>>, []>;
      /**
       * The current GRANDPA Authority set.
       **/
      currentAuthoritySet: AugmentedQuery<ApiType, () => Observable<Option<PalletGrandpaFinalityVerifierBridgesHeaderChainAuthoritySet>>, []>;
      /**
       * If true, all pallet transactions are failed immediately.
       **/
      everInitialized: AugmentedQuery<ApiType, () => Observable<bool>, []>;
      /**
       * A ring buffer of imported hashes. Ordered by the insertion time.
       **/
      importedHashes: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<H256>>, [u32]>;
      /**
       * Current ring buffer position.
       **/
      importedHashesPointer: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []>;
      /**
       * Headers which have been imported into the pallet.
       **/
      importedHeaders: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Option<SpRuntimeHeader>>, [H256]>;
      /**
       * Hash of the header used to bootstrap the pallet.
       **/
      initialHash: AugmentedQuery<ApiType, () => Observable<Option<H256>>, []>;
      /**
       * If true, all pallet transactions are failed immediately.
       **/
      isHalted: AugmentedQuery<ApiType, () => Observable<bool>, []>;
      /**
       * Optional pallet owner.
       * 
       * Pallet owner has a right to halt all pallet operations and then resume it. If it is
       * `None`, then there are no direct ways to halt/resume pallet operations, but other
       * runtime methods may still be used to do that (i.e. democracy::referendum to update halt
       * flag directly or call the `halt_operations`).
       **/
      palletOwner: AugmentedQuery<ApiType, () => Observable<Option<AccountId32>>, []>;
      /**
       * Maps a parachain chain_id to the corresponding chain ID.
       **/
      parachainIdMap: AugmentedQuery<ApiType, (arg: U8aFixed | string | Uint8Array) => Observable<Option<PalletGrandpaFinalityVerifierParachainRegistrationData>>, [U8aFixed]>;
      relayChainId: AugmentedQuery<ApiType, () => Observable<Option<U8aFixed>>, []>;
    };
    portal: {
    };
    randomnessCollectiveFlip: {
      /**
       * Series of block headers from the last 81 blocks that acts as random seed material. This
       * is arranged as a ring buffer with `block_number % 81` being the index into the `Vec` of
       * the oldest hash.
       **/
      randomMaterial: AugmentedQuery<ApiType, () => Observable<Vec<H256>>, []>;
    };
    rococoBridge: {
      /**
       * Hash of the best finalized header.
       **/
      bestFinalizedHash: AugmentedQuery<ApiType, () => Observable<Option<H256>>, []>;
      /**
       * The current GRANDPA Authority set.
       **/
      currentAuthoritySet: AugmentedQuery<ApiType, () => Observable<Option<PalletGrandpaFinalityVerifierBridgesHeaderChainAuthoritySet>>, []>;
      /**
       * If true, all pallet transactions are failed immediately.
       **/
      everInitialized: AugmentedQuery<ApiType, () => Observable<bool>, []>;
      /**
       * A ring buffer of imported hashes. Ordered by the insertion time.
       **/
      importedHashes: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<H256>>, [u32]>;
      /**
       * Current ring buffer position.
       **/
      importedHashesPointer: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []>;
      /**
       * Headers which have been imported into the pallet.
       **/
      importedHeaders: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Option<SpRuntimeHeader>>, [H256]>;
      /**
       * Hash of the header used to bootstrap the pallet.
       **/
      initialHash: AugmentedQuery<ApiType, () => Observable<Option<H256>>, []>;
      /**
       * If true, all pallet transactions are failed immediately.
       **/
      isHalted: AugmentedQuery<ApiType, () => Observable<bool>, []>;
      /**
       * Optional pallet owner.
       * 
       * Pallet owner has a right to halt all pallet operations and then resume it. If it is
       * `None`, then there are no direct ways to halt/resume pallet operations, but other
       * runtime methods may still be used to do that (i.e. democracy::referendum to update halt
       * flag directly or call the `halt_operations`).
       **/
      palletOwner: AugmentedQuery<ApiType, () => Observable<Option<AccountId32>>, []>;
      /**
       * Maps a parachain chain_id to the corresponding chain ID.
       **/
      parachainIdMap: AugmentedQuery<ApiType, (arg: U8aFixed | string | Uint8Array) => Observable<Option<PalletGrandpaFinalityVerifierParachainRegistrationData>>, [U8aFixed]>;
      relayChainId: AugmentedQuery<ApiType, () => Observable<Option<U8aFixed>>, []>;
    };
    sudo: {
      /**
       * The `AccountId` of the sudo key.
       **/
      key: AugmentedQuery<ApiType, () => Observable<Option<AccountId32>>, []>;
    };
    system: {
      /**
       * The full account information for a particular account ID.
       **/
      account: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<FrameSystemAccountInfo>, [AccountId32]>;
      /**
       * Total length (in bytes) for all extrinsics put together, for the current block.
       **/
      allExtrinsicsLen: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []>;
      /**
       * Map of block numbers to block hashes.
       **/
      blockHash: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<H256>, [u32]>;
      /**
       * The current weight for the block.
       **/
      blockWeight: AugmentedQuery<ApiType, () => Observable<FrameSupportWeightsPerDispatchClassU64>, []>;
      /**
       * Digest of the current block, also part of the block header.
       **/
      digest: AugmentedQuery<ApiType, () => Observable<SpRuntimeDigest>, []>;
      /**
       * The number of events in the `Events<T>` list.
       **/
      eventCount: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      /**
       * Events deposited for the current block.
       * 
       * NOTE: The item is unbound and should therefore never be read on chain.
       * It could otherwise inflate the PoV size of a block.
       * 
       * Events have a large in-memory size. Box the events to not go out-of-memory
       * just in case someone still reads them from within the runtime.
       **/
      events: AugmentedQuery<ApiType, () => Observable<Vec<FrameSystemEventRecord>>, []>;
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
      eventTopics: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Vec<ITuple<[u32, u32]>>>, [H256]>;
      /**
       * The execution phase of the block.
       **/
      executionPhase: AugmentedQuery<ApiType, () => Observable<Option<FrameSystemPhase>>, []>;
      /**
       * Total extrinsics count for the current block.
       **/
      extrinsicCount: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []>;
      /**
       * Extrinsics data for the current block (maps an extrinsic's index to its data).
       **/
      extrinsicData: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Bytes>, [u32]>;
      /**
       * Stores the `spec_version` and `spec_name` of when the last runtime upgrade happened.
       **/
      lastRuntimeUpgrade: AugmentedQuery<ApiType, () => Observable<Option<FrameSystemLastRuntimeUpgradeInfo>>, []>;
      /**
       * The current block number being processed. Set by `execute_block`.
       **/
      number: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      /**
       * Hash of the previous block.
       **/
      parentHash: AugmentedQuery<ApiType, () => Observable<H256>, []>;
      /**
       * True if we have upgraded so that AccountInfo contains three types of `RefCount`. False
       * (default) if not.
       **/
      upgradedToTripleRefCount: AugmentedQuery<ApiType, () => Observable<bool>, []>;
      /**
       * True if we have upgraded so that `type RefCount` is `u32`. False (default) if not.
       **/
      upgradedToU32RefCount: AugmentedQuery<ApiType, () => Observable<bool>, []>;
    };
    threeVm: {
      /**
       * A mapping of a contract's address to its author.
       **/
      authorOf: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<Option<AccountId32>>, [AccountId32]>;
      /**
       * A mapping of precompile pointers
       **/
      precompileIndex: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Option<u8>>, [H256]>;
      /**
       * Holds the amount of times the signal was posted or attempted to be posted
       **/
      signals: AugmentedQuery<ApiType, (arg1: H256 | string | Uint8Array, arg2: u32 | AnyNumber | Uint8Array) => Observable<Option<u32>>, [H256, u32]>;
    };
    timestamp: {
      /**
       * Did the timestamp get updated in this block?
       **/
      didUpdate: AugmentedQuery<ApiType, () => Observable<bool>, []>;
      /**
       * Current time for the current block.
       **/
      now: AugmentedQuery<ApiType, () => Observable<u64>, []>;
    };
    transactionPayment: {
      nextFeeMultiplier: AugmentedQuery<ApiType, () => Observable<u128>, []>;
      storageVersion: AugmentedQuery<ApiType, () => Observable<PalletTransactionPaymentReleases>, []>;
    };
    treasury: {
      /**
       * Proposal indices that have been approved but not yet awarded.
       **/
      approvals: AugmentedQuery<ApiType, () => Observable<Vec<u32>>, []>;
      /**
       * Number of proposals that have been made.
       **/
      proposalCount: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      /**
       * Proposals that have been made.
       **/
      proposals: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<PalletTreasuryProposal>>, [u32]>;
    };
    xdns: {
      customSideEffects: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Option<Bytes>>, [H256]>;
      gateways: AugmentedQuery<ApiType, (arg: U8aFixed | string | Uint8Array) => Observable<Option<T3rnPrimitivesXdnsGatewayRecord>>, [U8aFixed]>;
      sfxabiRegistry: AugmentedQuery<ApiType, (arg1: U8aFixed | string | Uint8Array, arg2: U8aFixed | string | Uint8Array) => Observable<Option<T3rnAbiSfxAbi>>, [U8aFixed, U8aFixed]>;
      standardSFXABIs: AugmentedQuery<ApiType, (arg: U8aFixed | string | Uint8Array) => Observable<Option<T3rnAbiSfxAbi>>, [U8aFixed]>;
      standardSideEffects: AugmentedQuery<ApiType, (arg: U8aFixed | string | Uint8Array) => Observable<Option<Bytes>>, [U8aFixed]>;
      storageMigrations: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      tokens: AugmentedQuery<ApiType, (arg: U8aFixed | string | Uint8Array) => Observable<Option<T3rnPrimitivesXdnsTokenRecord>>, [U8aFixed]>;
      /**
       * The pre-validated composable xdns_records on-chain registry.
       **/
      xdnsRegistry: AugmentedQuery<ApiType, (arg: U8aFixed | string | Uint8Array) => Observable<Option<T3rnPrimitivesXdnsXdnsRecord>>, [U8aFixed]>;
    };
  } // AugmentedQueries
} // declare module
