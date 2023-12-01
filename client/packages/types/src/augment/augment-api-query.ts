// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/api-base/types/storage';

import type { ApiTypes, AugmentedQuery, QueryableStorageEntry } from '@polkadot/api-base/types';
import type { Data } from '@polkadot/types';
import type { BTreeMap, Bytes, Option, Struct, U8aFixed, Vec, bool, u128, u32, u64, u8 } from '@polkadot/types-codec';
import type { AnyNumber, ITuple } from '@polkadot/types-codec/types';
import type { AccountId32, H160, H256, Percent } from '@polkadot/types/interfaces/runtime';
import type { CircuitStandaloneRuntimeRuntimeHoldReason, FrameSupportDispatchPerDispatchClassWeight, FrameSystemAccountInfo, FrameSystemEventRecord, FrameSystemLastRuntimeUpgradeInfo, FrameSystemPhase, PalletAssetsApproval, PalletAssetsAssetAccount, PalletAssetsAssetDetails, PalletAssetsAssetMetadata, PalletAttestersBatchMessage, PalletAttestersInfluxMessage, PalletBalancesAccountData, PalletBalancesBalanceLock, PalletBalancesIdAmount, PalletBalancesReserveData, PalletContractsStorageContractInfo, PalletContractsStorageDeletionQueueManager, PalletContractsWasmCodeInfo, PalletEth2FinalityVerifierBeaconBlockHeader, PalletEth2FinalityVerifierCheckpoint, PalletEth2FinalityVerifierExecutionHeader, PalletEth2FinalityVerifierSyncCommittee, PalletEvmCodeMetadata, PalletGrandpaFinalityVerifierBridgesHeaderChainAuthoritySet, PalletGrandpaFinalityVerifierParachainRegistrationData, PalletGrandpaStoredPendingChange, PalletGrandpaStoredState, PalletIdentityRegistrarInfo, PalletIdentityRegistration, PalletRewardsAssetType, PalletRewardsDistributionRecord, PalletRewardsTreasuryBalanceSheet, PalletSepoliaFinalityVerifierBeaconBlockHeader, PalletSepoliaFinalityVerifierCheckpoint, PalletSepoliaFinalityVerifierExecutionHeader, PalletSepoliaFinalityVerifierSyncCommittee, PalletTransactionPaymentReleases, PalletTreasuryProposal, SpConsensusAuraSr25519AppSr25519Public, SpRuntimeDigest, SpRuntimeHeader, T3rnAbiSfxAbi, T3rnPrimitivesAccountManagerRequestCharge, T3rnPrimitivesAccountManagerSettlement, T3rnPrimitivesAttestersAttesterInfo, T3rnPrimitivesCircuitTypesAdaptiveTimeout, T3rnPrimitivesCircuitTypesXExecSignal, T3rnPrimitivesClaimableClaimableArtifacts, T3rnPrimitivesCommonRoundInfo, T3rnPrimitivesContractsRegistryRegistryContract, T3rnPrimitivesFinalityVerifierActivity, T3rnPrimitivesGatewayActivity, T3rnPrimitivesGatewayVendor, T3rnPrimitivesSpeedMode, T3rnPrimitivesVolatileLocalState, T3rnPrimitivesXdnsEpochEstimate, T3rnPrimitivesXdnsGatewayRecord, T3rnPrimitivesXdnsTokenRecord, T3rnSdkPrimitivesSignalExecutionSignal, T3rnTypesFsxFullSideEffect } from '@polkadot/types/lookup';
import type { Observable } from '@polkadot/types/types';

export type __AugmentedQuery<ApiType extends ApiTypes> = AugmentedQuery<ApiType, () => unknown>;
export type __QueryableStorageEntry<ApiType extends ApiTypes> = QueryableStorageEntry<ApiType>;

declare module '@polkadot/api-base/types/storage' {
  interface AugmentedQueries<ApiType extends ApiTypes> {
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
    attesters: {
      activeSet: AugmentedQuery<ApiType, () => Observable<Vec<AccountId32>>, []>;
      attestationsInflux: AugmentedQuery<ApiType, (arg1: U8aFixed | string | Uint8Array, arg2: H256 | string | Uint8Array) => Observable<Option<PalletAttestersInfluxMessage>>, [U8aFixed, H256]>;
      attestationTargets: AugmentedQuery<ApiType, () => Observable<Vec<U8aFixed>>, []>;
      attesters: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<Option<T3rnPrimitivesAttestersAttesterInfo>>, [AccountId32]>;
      attestersAgreements: AugmentedQuery<ApiType, (arg1: AccountId32 | string | Uint8Array, arg2: U8aFixed | string | Uint8Array) => Observable<Option<Bytes>>, [AccountId32, U8aFixed]>;
      batches: AugmentedQuery<ApiType, (arg: U8aFixed | string | Uint8Array) => Observable<Option<Vec<PalletAttestersBatchMessage>>>, [U8aFixed]>;
      batchesToSign: AugmentedQuery<ApiType, (arg: U8aFixed | string | Uint8Array) => Observable<Option<Vec<PalletAttestersBatchMessage>>>, [U8aFixed]>;
      committeeTransitionOn: AugmentedQuery<ApiType, (arg: U8aFixed | string | Uint8Array) => Observable<Option<u32>>, [U8aFixed]>;
      currentCommittee: AugmentedQuery<ApiType, () => Observable<Vec<AccountId32>>, []>;
      currentRetributionPerSFXPercentage: AugmentedQuery<ApiType, () => Observable<Percent>, []>;
      currentSlashTreasuryBalance: AugmentedQuery<ApiType, () => Observable<Percent>, []>;
      invulnerableAttester: AugmentedQuery<ApiType, () => Observable<Option<AccountId32>>, []>;
      nextBatch: AugmentedQuery<ApiType, (arg: U8aFixed | string | Uint8Array) => Observable<Option<PalletAttestersBatchMessage>>, [U8aFixed]>;
      nextCommittee: AugmentedQuery<ApiType, () => Observable<Vec<AccountId32>>, []>;
      nextCommitteeOnTarget: AugmentedQuery<ApiType, (arg: U8aFixed | string | Uint8Array) => Observable<Option<Vec<ITuple<[u32, Bytes]>>>>, [U8aFixed]>;
      nominations: AugmentedQuery<ApiType, (arg1: AccountId32 | string | Uint8Array, arg2: AccountId32 | string | Uint8Array) => Observable<Option<u128>>, [AccountId32, AccountId32]>;
      paidFinalityFees: AugmentedQuery<ApiType, (arg: U8aFixed | string | Uint8Array) => Observable<Option<Vec<u128>>>, [U8aFixed]>;
      pendingAttestationTargets: AugmentedQuery<ApiType, () => Observable<Vec<U8aFixed>>, []>;
      pendingUnnominations: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<Option<Vec<ITuple<[AccountId32, u128, u32]>>>>, [AccountId32]>;
      permanentSlashes: AugmentedQuery<ApiType, () => Observable<Vec<AccountId32>>, []>;
      previousCommittee: AugmentedQuery<ApiType, () => Observable<Vec<AccountId32>>, []>;
      sortedNominatedAttesters: AugmentedQuery<ApiType, () => Observable<Vec<ITuple<[AccountId32, u128]>>>, []>;
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
       * Freeze locks on account balances.
       **/
      freezes: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<Vec<PalletBalancesIdAmount>>, [AccountId32]>;
      /**
       * Holds on account balances.
       **/
      holds: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<Vec<{
    readonly id: CircuitStandaloneRuntimeRuntimeHoldReason;
    readonly amount: u128;
  } & Struct>>, [AccountId32]>;
      /**
       * The total units of outstanding deactivated balance in the system.
       **/
      inactiveIssuance: AugmentedQuery<ApiType, () => Observable<u128>, []>;
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
       * The total units issued in the system.
       **/
      totalIssuance: AugmentedQuery<ApiType, () => Observable<u128>, []>;
    };
    circuit: {
      dlq: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Option<ITuple<[u32, Vec<U8aFixed>, T3rnPrimitivesSpeedMode]>>>, [H256]>;
      finalizedXtx: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Option<u32>>, [H256]>;
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
      gmp: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Option<H256>>, [H256]>;
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
      pendingXtxTimeoutsMap: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Option<T3rnPrimitivesCircuitTypesAdaptiveTimeout>>, [H256]>;
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
      xExecSignals: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Option<T3rnPrimitivesCircuitTypesXExecSignal>>, [H256]>;
    };
    clock: {
      /**
       * Information on the current round.
       **/
      currentRound: AugmentedQuery<ApiType, () => Observable<T3rnPrimitivesCommonRoundInfo>, []>;
    };
    contracts: {
      /**
       * A mapping from a contract's code hash to its code info.
       **/
      codeInfoOf: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Option<PalletContractsWasmCodeInfo>>, [H256]>;
      /**
       * The code associated with a given account.
       * 
       * TWOX-NOTE: SAFE since `AccountId` is a secure hash.
       **/
      contractInfoOf: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<Option<PalletContractsStorageContractInfo>>, [AccountId32]>;
      /**
       * Evicted contracts that await child trie deletion.
       * 
       * Child trie deletion is a heavy operation depending on the amount of storage items
       * stored in said trie. Therefore this operation is performed lazily in `on_idle`.
       **/
      deletionQueue: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<Bytes>>, [u32]>;
      /**
       * A pair of monotonic counters used to track the latest contract marked for deletion
       * and the latest deleted contract in queue.
       **/
      deletionQueueCounter: AugmentedQuery<ApiType, () => Observable<PalletContractsStorageDeletionQueueManager>, []>;
      /**
       * A migration can span across multiple blocks. This storage defines a cursor to track the
       * progress of the migration, enabling us to resume from the last completed position.
       **/
      migrationInProgress: AugmentedQuery<ApiType, () => Observable<Option<Bytes>>, []>;
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
       * A mapping from a contract's code hash to its code.
       **/
      pristineCode: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Option<Bytes>>, [H256]>;
    };
    contractsRegistry: {
      /**
       * The pre-validated composable contracts on-chain registry.
       **/
      contractsRegistry: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Option<T3rnPrimitivesContractsRegistryRegistryContract>>, [H256]>;
    };
    escrowTreasury: {
      /**
       * Proposal indices that have been approved but not yet awarded.
       **/
      approvals: AugmentedQuery<ApiType, () => Observable<Vec<u32>>, []>;
      /**
       * The amount which has been reported as inactive to Currency.
       **/
      deactivated: AugmentedQuery<ApiType, () => Observable<u128>, []>;
      /**
       * Number of proposals that have been made.
       **/
      proposalCount: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      /**
       * Proposals that have been made.
       **/
      proposals: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<PalletTreasuryProposal>>, [u32]>;
    };
    ethereumBridge: {
      beaconCheckpointHeaders: AugmentedQuery<ApiType, (arg: u64 | AnyNumber | Uint8Array) => Observable<Option<PalletEth2FinalityVerifierBeaconBlockHeader>>, [u64]>;
      currentCheckpoint: AugmentedQuery<ApiType, () => Observable<PalletEth2FinalityVerifierCheckpoint>, []>;
      currentSyncCommitteePeriod: AugmentedQuery<ApiType, () => Observable<Option<u64>>, []>;
      executionHeaderMap: AugmentedQuery<ApiType, (arg: u64 | AnyNumber | Uint8Array) => Observable<Option<PalletEth2FinalityVerifierExecutionHeader>>, [u64]>;
      executionHeaderPointer: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      importedExecutionBlockNumber: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<u64>>, [u32]>;
      isHalted: AugmentedQuery<ApiType, () => Observable<bool>, []>;
      isInitialized: AugmentedQuery<ApiType, () => Observable<bool>, []>;
      latestBeaconBlockHeader: AugmentedQuery<ApiType, () => Observable<PalletEth2FinalityVerifierBeaconBlockHeader>, []>;
      latestExecutionHeader: AugmentedQuery<ApiType, () => Observable<PalletEth2FinalityVerifierExecutionHeader>, []>;
      owner: AugmentedQuery<ApiType, () => Observable<Option<AccountId32>>, []>;
      syncCommitteeMap: AugmentedQuery<ApiType, (arg: u64 | AnyNumber | Uint8Array) => Observable<Option<PalletEth2FinalityVerifierSyncCommittee>>, [u64]>;
    };
    evm: {
      accountCodes: AugmentedQuery<ApiType, (arg: H160 | string | Uint8Array) => Observable<Bytes>, [H160]>;
      accountCodesMetadata: AugmentedQuery<ApiType, (arg: H160 | string | Uint8Array) => Observable<Option<PalletEvmCodeMetadata>>, [H160]>;
      accountStorages: AugmentedQuery<ApiType, (arg1: H160 | string | Uint8Array, arg2: H256 | string | Uint8Array) => Observable<H256>, [H160, H256]>;
    };
    feeTreasury: {
      /**
       * Proposal indices that have been approved but not yet awarded.
       **/
      approvals: AugmentedQuery<ApiType, () => Observable<Vec<u32>>, []>;
      /**
       * The amount which has been reported as inactive to Currency.
       **/
      deactivated: AugmentedQuery<ApiType, () => Observable<u128>, []>;
      /**
       * Number of proposals that have been made.
       **/
      proposalCount: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      /**
       * Proposals that have been made.
       **/
      proposals: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<PalletTreasuryProposal>>, [u32]>;
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
       * This is only used for validating equivocation proofs. An equivocation proof must
       * contains a key-ownership proof for a given session, therefore we need a way to tie
       * together sessions and GRANDPA set ids, i.e. we need to validate that a validator
       * was the owner of a given key on a given session, and what the active set ID was
       * during that session.
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
      /**
       * Count successful submissions.
       **/
      submissionsCounter: AugmentedQuery<ApiType, () => Observable<u32>, []>;
    };
    maintenanceMode: {
      /**
       * Whether the site is in maintenance mode
       **/
      maintenanceMode: AugmentedQuery<ApiType, () => Observable<bool>, []>;
    };
    parachainTreasury: {
      /**
       * Proposal indices that have been approved but not yet awarded.
       **/
      approvals: AugmentedQuery<ApiType, () => Observable<Vec<u32>>, []>;
      /**
       * The amount which has been reported as inactive to Currency.
       **/
      deactivated: AugmentedQuery<ApiType, () => Observable<u128>, []>;
      /**
       * Number of proposals that have been made.
       **/
      proposalCount: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      /**
       * Proposals that have been made.
       **/
      proposals: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<PalletTreasuryProposal>>, [u32]>;
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
      /**
       * Count successful submissions.
       **/
      submissionsCounter: AugmentedQuery<ApiType, () => Observable<u32>, []>;
    };
    randomnessCollectiveFlip: {
      /**
       * Series of block headers from the last 81 blocks that acts as random seed material. This
       * is arranged as a ring buffer with `block_number % 81` being the index into the `Vec` of
       * the oldest hash.
       **/
      randomMaterial: AugmentedQuery<ApiType, () => Observable<Vec<H256>>, []>;
    };
    rewards: {
      /**
       * Accumulated settlements per executor per asset id.
       **/
      accumulatedSettlements: AugmentedQuery<ApiType, (arg1: AccountId32 | string | Uint8Array, arg2: PalletRewardsAssetType | { Native: any } | { NonNative: any } | string | Uint8Array) => Observable<Option<u128>>, [AccountId32, PalletRewardsAssetType]>;
      attesters: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<Option<u32>>, [AccountId32]>;
      authors: AugmentedQuery<ApiType, () => Observable<BTreeMap<AccountId32, u32>>, []>;
      authorsThisPeriod: AugmentedQuery<ApiType, () => Observable<BTreeMap<AccountId32, u32>>, []>;
      collators: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<Option<u32>>, [AccountId32]>;
      distributionBlock: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []>;
      distributionHistory: AugmentedQuery<ApiType, () => Observable<Vec<PalletRewardsDistributionRecord>>, []>;
      estimatedTreasuryBalance: AugmentedQuery<ApiType, () => Observable<PalletRewardsTreasuryBalanceSheet>, []>;
      isClaimingHalted: AugmentedQuery<ApiType, () => Observable<bool>, []>;
      isDistributionHalted: AugmentedQuery<ApiType, () => Observable<bool>, []>;
      isSettlementAccumulationHalted: AugmentedQuery<ApiType, () => Observable<bool>, []>;
      lastProcessedRound: AugmentedQuery<ApiType, () => Observable<Option<T3rnPrimitivesCommonRoundInfo>>, []>;
      maxRewardExecutorsKickback: AugmentedQuery<ApiType, () => Observable<Percent>, []>;
      pendingClaims: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<Option<Vec<T3rnPrimitivesClaimableClaimableArtifacts>>>, [AccountId32]>;
      repatriationPercentage: AugmentedQuery<ApiType, () => Observable<Percent>, []>;
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
      /**
       * Count successful submissions.
       **/
      submissionsCounter: AugmentedQuery<ApiType, () => Observable<u32>, []>;
    };
    sepoliaBridge: {
      beaconCheckpointHeaders: AugmentedQuery<ApiType, (arg: u64 | AnyNumber | Uint8Array) => Observable<Option<PalletSepoliaFinalityVerifierBeaconBlockHeader>>, [u64]>;
      currentCheckpoint: AugmentedQuery<ApiType, () => Observable<PalletSepoliaFinalityVerifierCheckpoint>, []>;
      currentSyncCommitteePeriod: AugmentedQuery<ApiType, () => Observable<Option<u64>>, []>;
      executionHeaderMap: AugmentedQuery<ApiType, (arg: u64 | AnyNumber | Uint8Array) => Observable<Option<PalletSepoliaFinalityVerifierExecutionHeader>>, [u64]>;
      executionHeaderPointer: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      importedExecutionBlockNumber: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<u64>>, [u32]>;
      isHalted: AugmentedQuery<ApiType, () => Observable<bool>, []>;
      isInitialized: AugmentedQuery<ApiType, () => Observable<bool>, []>;
      latestBeaconBlockHeader: AugmentedQuery<ApiType, () => Observable<PalletSepoliaFinalityVerifierBeaconBlockHeader>, []>;
      latestExecutionHeader: AugmentedQuery<ApiType, () => Observable<PalletSepoliaFinalityVerifierExecutionHeader>, []>;
      owner: AugmentedQuery<ApiType, () => Observable<Option<AccountId32>>, []>;
      syncCommitteeMap: AugmentedQuery<ApiType, (arg: u64 | AnyNumber | Uint8Array) => Observable<Option<PalletSepoliaFinalityVerifierSyncCommittee>>, [u64]>;
    };
    slashTreasury: {
      /**
       * Proposal indices that have been approved but not yet awarded.
       **/
      approvals: AugmentedQuery<ApiType, () => Observable<Vec<u32>>, []>;
      /**
       * The amount which has been reported as inactive to Currency.
       **/
      deactivated: AugmentedQuery<ApiType, () => Observable<u128>, []>;
      /**
       * Number of proposals that have been made.
       **/
      proposalCount: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      /**
       * Proposals that have been made.
       **/
      proposals: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<PalletTreasuryProposal>>, [u32]>;
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
      blockWeight: AugmentedQuery<ApiType, () => Observable<FrameSupportDispatchPerDispatchClassWeight>, []>;
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
       * The value has the type `(BlockNumberFor<T>, EventIndex)` because if we used only just
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
       * The amount which has been reported as inactive to Currency.
       **/
      deactivated: AugmentedQuery<ApiType, () => Observable<u128>, []>;
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
      allGatewayIds: AugmentedQuery<ApiType, () => Observable<Vec<U8aFixed>>, []>;
      allTokenIds: AugmentedQuery<ApiType, () => Observable<Vec<u32>>, []>;
      assetCostEstimatesInNative: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<u128>, [u32]>;
      assetEstimates: AugmentedQuery<ApiType, (arg: ITuple<[u32, u32]> | [u32 | AnyNumber | Uint8Array, u32 | AnyNumber | Uint8Array]) => Observable<u128>, [ITuple<[u32, u32]>]>;
      assetEstimatesInNative: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<u128>, [u32]>;
      authorizedMintAssets: AugmentedQuery<ApiType, () => Observable<Vec<ITuple<[u32, U8aFixed]>>>, []>;
      customSideEffects: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Option<Bytes>>, [H256]>;
      epochHistory: AugmentedQuery<ApiType, (arg: T3rnPrimitivesGatewayVendor | 'Polkadot' | 'Kusama' | 'Rococo' | 'Ethereum' | 'Sepolia' | 'XBI' | 'Attesters' | number | Uint8Array) => Observable<Option<Vec<T3rnPrimitivesXdnsEpochEstimate>>>, [T3rnPrimitivesGatewayVendor]>;
      gateways: AugmentedQuery<ApiType, (arg: U8aFixed | string | Uint8Array) => Observable<Option<T3rnPrimitivesXdnsGatewayRecord>>, [U8aFixed]>;
      gatewaysOverviewStore: AugmentedQuery<ApiType, () => Observable<Vec<T3rnPrimitivesGatewayActivity>>, []>;
      gatewaysOverviewStoreHistory: AugmentedQuery<ApiType, (arg: U8aFixed | string | Uint8Array) => Observable<Vec<T3rnPrimitivesGatewayActivity>>, [U8aFixed]>;
      gatewayTokens: AugmentedQuery<ApiType, (arg: U8aFixed | string | Uint8Array) => Observable<Vec<u32>>, [U8aFixed]>;
      perTargetAssetEstimates: AugmentedQuery<ApiType, (arg1: U8aFixed | string | Uint8Array, arg2: ITuple<[u32, u32]> | [u32 | AnyNumber | Uint8Array, u32 | AnyNumber | Uint8Array]) => Observable<u128>, [U8aFixed, ITuple<[u32, u32]>]>;
      remoteOrderAddresses: AugmentedQuery<ApiType, (arg: U8aFixed | string | Uint8Array) => Observable<Option<H256>>, [U8aFixed]>;
      sfxabiRegistry: AugmentedQuery<ApiType, (arg1: U8aFixed | string | Uint8Array, arg2: U8aFixed | string | Uint8Array) => Observable<Option<T3rnAbiSfxAbi>>, [U8aFixed, U8aFixed]>;
      standardSFXABIs: AugmentedQuery<ApiType, (arg: U8aFixed | string | Uint8Array) => Observable<Option<T3rnAbiSfxAbi>>, [U8aFixed]>;
      standardSideEffects: AugmentedQuery<ApiType, (arg: U8aFixed | string | Uint8Array) => Observable<Option<Bytes>>, [U8aFixed]>;
      storageMigrations: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      tokens: AugmentedQuery<ApiType, (arg1: u32 | AnyNumber | Uint8Array, arg2: U8aFixed | string | Uint8Array) => Observable<Option<T3rnPrimitivesXdnsTokenRecord>>, [u32, U8aFixed]>;
      verifierOverviewStore: AugmentedQuery<ApiType, () => Observable<Vec<T3rnPrimitivesFinalityVerifierActivity>>, []>;
      verifierOverviewStoreHistory: AugmentedQuery<ApiType, (arg: T3rnPrimitivesGatewayVendor | 'Polkadot' | 'Kusama' | 'Rococo' | 'Ethereum' | 'Sepolia' | 'XBI' | 'Attesters' | number | Uint8Array) => Observable<Vec<T3rnPrimitivesFinalityVerifierActivity>>, [T3rnPrimitivesGatewayVendor]>;
    };
  } // AugmentedQueries
} // declare module
