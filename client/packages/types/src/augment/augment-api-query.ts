// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/api-base/types/storage';

import type { ApiTypes, AugmentedQuery, QueryableStorageEntry } from '@polkadot/api-base/types';
import type { Data } from '@polkadot/types';
import type { BTreeMap, Bytes, Option, Struct, U8aFixed, Vec, bool, u128, u16, u32, u64, u8 } from '@polkadot/types-codec';
import type { AnyNumber, ITuple } from '@polkadot/types-codec/types';
import type { AccountId32, H160, H256, Percent } from '@polkadot/types/interfaces/runtime';
import type { CumulusPalletDmpQueueConfigData, CumulusPalletDmpQueuePageIndexData, CumulusPalletParachainSystemCodeUpgradeAuthorization, CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot, CumulusPalletXcmpQueueInboundChannelDetails, CumulusPalletXcmpQueueOutboundChannelDetails, CumulusPalletXcmpQueueQueueConfigData, FrameSupportDispatchPerDispatchClassWeight, FrameSystemAccountInfo, FrameSystemEventRecord, FrameSystemLastRuntimeUpgradeInfo, FrameSystemPhase, PalletAssetRegistryAssetInfo, PalletAssetsApproval, PalletAssetsAssetAccount, PalletAssetsAssetDetails, PalletAssetsAssetMetadata, PalletAttestersBatchMessage, PalletBalancesAccountData, PalletBalancesBalanceLock, PalletBalancesIdAmount, PalletBalancesReserveData, PalletCollatorSelectionCandidateInfo, PalletContractsStorageContractInfo, PalletContractsStorageDeletionQueueManager, PalletContractsWasmCodeInfo, PalletEth2FinalityVerifierBeaconBlockHeader, PalletEth2FinalityVerifierCheckpoint, PalletEth2FinalityVerifierExecutionHeader, PalletEth2FinalityVerifierSyncCommittee, PalletEvmCodeMetadata, PalletGrandpaFinalityVerifierBridgesHeaderChainAuthoritySet, PalletGrandpaFinalityVerifierParachainRegistrationData, PalletIdentityRegistrarInfo, PalletIdentityRegistration, PalletPreimageRequestStatus, PalletRewardsAssetType, PalletRewardsDistributionRecord, PalletRewardsTreasuryBalanceSheet, PalletSchedulerScheduled, PalletSepoliaFinalityVerifierBeaconBlockHeader, PalletSepoliaFinalityVerifierCheckpoint, PalletSepoliaFinalityVerifierExecutionHeader, PalletSepoliaFinalityVerifierSyncCommittee, PalletTransactionPaymentReleases, PalletTreasuryProposal, PalletXcmQueryStatus, PalletXcmRemoteLockedFungibleRecord, PalletXcmVersionMigrationStage, PolkadotCorePrimitivesOutboundHrmpMessage, PolkadotPrimitivesV5AbridgedHostConfiguration, PolkadotPrimitivesV5PersistedValidationData, PolkadotPrimitivesV5UpgradeRestriction, SpConsensusAuraSr25519AppSr25519Public, SpCoreCryptoKeyTypeId, SpRuntimeDigest, SpRuntimeHeader, SpTrieStorageProof, SpWeightsWeightV2Weight, T0rnParachainRuntimeParachainConfigSessionKeys, T0rnParachainRuntimeRuntimeHoldReason, T3rnAbiSfxAbi, T3rnPrimitivesAccountManagerRequestCharge, T3rnPrimitivesAccountManagerSettlement, T3rnPrimitivesAttestersAttesterInfo, T3rnPrimitivesCircuitTypesAdaptiveTimeout, T3rnPrimitivesCircuitTypesXExecSignal, T3rnPrimitivesClaimableClaimableArtifacts, T3rnPrimitivesCommonRoundInfo, T3rnPrimitivesContractsRegistryRegistryContract, T3rnPrimitivesFinalityVerifierActivity, T3rnPrimitivesGatewayActivity, T3rnPrimitivesGatewayVendor, T3rnPrimitivesSpeedMode, T3rnPrimitivesVolatileLocalState, T3rnPrimitivesXdnsEpochEstimate, T3rnPrimitivesXdnsGatewayRecord, T3rnPrimitivesXdnsTokenRecord, T3rnSdkPrimitivesSignalExecutionSignal, T3rnTypesFsxFullSideEffect, XcmV3MultiLocation, XcmVersionedAssetId, XcmVersionedMultiLocation } from '@polkadot/types/lookup';
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
    assetRegistry: {
      assetMetadata: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<PalletAssetRegistryAssetInfo>>, [u32]>;
      locationMapping: AugmentedQuery<ApiType, (arg: XcmV3MultiLocation | { parents?: any; interior?: any } | string | Uint8Array) => Observable<Option<u32>>, [XcmV3MultiLocation]>;
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
      attestationTargets: AugmentedQuery<ApiType, () => Observable<Vec<U8aFixed>>, []>;
      attesters: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<Option<T3rnPrimitivesAttestersAttesterInfo>>, [AccountId32]>;
      attestersAgreements: AugmentedQuery<ApiType, (arg1: AccountId32 | string | Uint8Array, arg2: U8aFixed | string | Uint8Array) => Observable<Option<Bytes>>, [AccountId32, U8aFixed]>;
      batches: AugmentedQuery<ApiType, (arg: U8aFixed | string | Uint8Array) => Observable<Option<Vec<PalletAttestersBatchMessage>>>, [U8aFixed]>;
      batchesToSign: AugmentedQuery<ApiType, (arg: U8aFixed | string | Uint8Array) => Observable<Option<Vec<PalletAttestersBatchMessage>>>, [U8aFixed]>;
      committeeTransitionOn: AugmentedQuery<ApiType, (arg: U8aFixed | string | Uint8Array) => Observable<Option<u32>>, [U8aFixed]>;
      currentCommittee: AugmentedQuery<ApiType, () => Observable<Vec<AccountId32>>, []>;
      currentRetributionPerSFXPercentage: AugmentedQuery<ApiType, () => Observable<Percent>, []>;
      currentSlashTreasuryBalance: AugmentedQuery<ApiType, () => Observable<Percent>, []>;
      fastConfirmationCost: AugmentedQuery<ApiType, (arg: U8aFixed | string | Uint8Array) => Observable<Option<u128>>, [U8aFixed]>;
      nextBatch: AugmentedQuery<ApiType, (arg: U8aFixed | string | Uint8Array) => Observable<Option<PalletAttestersBatchMessage>>, [U8aFixed]>;
      nextCommittee: AugmentedQuery<ApiType, () => Observable<Vec<AccountId32>>, []>;
      nextCommitteeOnTarget: AugmentedQuery<ApiType, (arg: U8aFixed | string | Uint8Array) => Observable<Option<Vec<ITuple<[u32, Bytes]>>>>, [U8aFixed]>;
      nominations: AugmentedQuery<ApiType, (arg1: AccountId32 | string | Uint8Array, arg2: AccountId32 | string | Uint8Array) => Observable<Option<u128>>, [AccountId32, AccountId32]>;
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
    auraExt: {
      /**
       * Serves as cache for the authorities.
       * 
       * The authorities in AuRa are overwritten in `on_initialize` when we switch to a new session,
       * but we require the old authorities to verify the seal when validating a PoV. This will always
       * be updated to the latest AuRa authorities in `on_finalize`.
       **/
      authorities: AugmentedQuery<ApiType, () => Observable<Vec<SpConsensusAuraSr25519AppSr25519Public>>, []>;
    };
    authorship: {
      /**
       * Author of current block.
       **/
      author: AugmentedQuery<ApiType, () => Observable<Option<AccountId32>>, []>;
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
    readonly id: T0rnParachainRuntimeRuntimeHoldReason;
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
    collatorSelection: {
      /**
       * Fixed amount to deposit to become a collator.
       * 
       * When a collator calls `leave_intent` they immediately receive the deposit back.
       **/
      candidacyBond: AugmentedQuery<ApiType, () => Observable<u128>, []>;
      /**
       * The (community, limited) collation candidates. `Candidates` and `Invulnerables` should be
       * mutually exclusive.
       **/
      candidates: AugmentedQuery<ApiType, () => Observable<Vec<PalletCollatorSelectionCandidateInfo>>, []>;
      /**
       * Desired number of candidates.
       * 
       * This should ideally always be less than [`Config::MaxCandidates`] for weights to be correct.
       **/
      desiredCandidates: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      /**
       * The invulnerable, permissioned collators. This list must be sorted.
       **/
      invulnerables: AugmentedQuery<ApiType, () => Observable<Vec<AccountId32>>, []>;
      /**
       * Last block authored by collator.
       **/
      lastAuthoredBlock: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<u32>, [AccountId32]>;
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
    dmpQueue: {
      /**
       * The configuration.
       **/
      configuration: AugmentedQuery<ApiType, () => Observable<CumulusPalletDmpQueueConfigData>, []>;
      /**
       * Counter for the related counted storage map
       **/
      counterForOverweight: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      /**
       * The overweight messages.
       **/
      overweight: AugmentedQuery<ApiType, (arg: u64 | AnyNumber | Uint8Array) => Observable<Option<ITuple<[u32, Bytes]>>>, [u64]>;
      /**
       * The page index.
       **/
      pageIndex: AugmentedQuery<ApiType, () => Observable<CumulusPalletDmpQueuePageIndexData>, []>;
      /**
       * The queue pages.
       **/
      pages: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Vec<ITuple<[u32, Bytes]>>>, [u32]>;
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
    maintenance: {
      /**
       * Whether the site is in maintenance mode
       **/
      maintenanceMode: AugmentedQuery<ApiType, () => Observable<bool>, []>;
    };
    parachainInfo: {
      parachainId: AugmentedQuery<ApiType, () => Observable<u32>, []>;
    };
    parachainSystem: {
      /**
       * The number of HRMP messages we observed in `on_initialize` and thus used that number for
       * announcing the weight of `on_initialize` and `on_finalize`.
       **/
      announcedHrmpMessagesPerCandidate: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      /**
       * The next authorized upgrade, if there is one.
       **/
      authorizedUpgrade: AugmentedQuery<ApiType, () => Observable<Option<CumulusPalletParachainSystemCodeUpgradeAuthorization>>, []>;
      /**
       * A custom head data that should be returned as result of `validate_block`.
       * 
       * See `Pallet::set_custom_validation_head_data` for more information.
       **/
      customValidationHeadData: AugmentedQuery<ApiType, () => Observable<Option<Bytes>>, []>;
      /**
       * Were the validation data set to notify the relay chain?
       **/
      didSetValidationCode: AugmentedQuery<ApiType, () => Observable<bool>, []>;
      /**
       * The parachain host configuration that was obtained from the relay parent.
       * 
       * This field is meant to be updated each block with the validation data inherent. Therefore,
       * before processing of the inherent, e.g. in `on_initialize` this data may be stale.
       * 
       * This data is also absent from the genesis.
       **/
      hostConfiguration: AugmentedQuery<ApiType, () => Observable<Option<PolkadotPrimitivesV5AbridgedHostConfiguration>>, []>;
      /**
       * HRMP messages that were sent in a block.
       * 
       * This will be cleared in `on_initialize` of each new block.
       **/
      hrmpOutboundMessages: AugmentedQuery<ApiType, () => Observable<Vec<PolkadotCorePrimitivesOutboundHrmpMessage>>, []>;
      /**
       * HRMP watermark that was set in a block.
       * 
       * This will be cleared in `on_initialize` of each new block.
       **/
      hrmpWatermark: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      /**
       * The last downward message queue chain head we have observed.
       * 
       * This value is loaded before and saved after processing inbound downward messages carried
       * by the system inherent.
       **/
      lastDmqMqcHead: AugmentedQuery<ApiType, () => Observable<H256>, []>;
      /**
       * The message queue chain heads we have observed per each channel incoming channel.
       * 
       * This value is loaded before and saved after processing inbound downward messages carried
       * by the system inherent.
       **/
      lastHrmpMqcHeads: AugmentedQuery<ApiType, () => Observable<BTreeMap<u32, H256>>, []>;
      /**
       * The relay chain block number associated with the last parachain block.
       **/
      lastRelayChainBlockNumber: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      /**
       * Validation code that is set by the parachain and is to be communicated to collator and
       * consequently the relay-chain.
       * 
       * This will be cleared in `on_initialize` of each new block if no other pallet already set
       * the value.
       **/
      newValidationCode: AugmentedQuery<ApiType, () => Observable<Option<Bytes>>, []>;
      /**
       * Upward messages that are still pending and not yet send to the relay chain.
       **/
      pendingUpwardMessages: AugmentedQuery<ApiType, () => Observable<Vec<Bytes>>, []>;
      /**
       * In case of a scheduled upgrade, this storage field contains the validation code to be applied.
       * 
       * As soon as the relay chain gives us the go-ahead signal, we will overwrite the [`:code`][sp_core::storage::well_known_keys::CODE]
       * which will result the next block process with the new validation code. This concludes the upgrade process.
       **/
      pendingValidationCode: AugmentedQuery<ApiType, () => Observable<Bytes>, []>;
      /**
       * Number of downward messages processed in a block.
       * 
       * This will be cleared in `on_initialize` of each new block.
       **/
      processedDownwardMessages: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      /**
       * The state proof for the last relay parent block.
       * 
       * This field is meant to be updated each block with the validation data inherent. Therefore,
       * before processing of the inherent, e.g. in `on_initialize` this data may be stale.
       * 
       * This data is also absent from the genesis.
       **/
      relayStateProof: AugmentedQuery<ApiType, () => Observable<Option<SpTrieStorageProof>>, []>;
      /**
       * The snapshot of some state related to messaging relevant to the current parachain as per
       * the relay parent.
       * 
       * This field is meant to be updated each block with the validation data inherent. Therefore,
       * before processing of the inherent, e.g. in `on_initialize` this data may be stale.
       * 
       * This data is also absent from the genesis.
       **/
      relevantMessagingState: AugmentedQuery<ApiType, () => Observable<Option<CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot>>, []>;
      /**
       * The weight we reserve at the beginning of the block for processing DMP messages. This
       * overrides the amount set in the Config trait.
       **/
      reservedDmpWeightOverride: AugmentedQuery<ApiType, () => Observable<Option<SpWeightsWeightV2Weight>>, []>;
      /**
       * The weight we reserve at the beginning of the block for processing XCMP messages. This
       * overrides the amount set in the Config trait.
       **/
      reservedXcmpWeightOverride: AugmentedQuery<ApiType, () => Observable<Option<SpWeightsWeightV2Weight>>, []>;
      /**
       * An option which indicates if the relay-chain restricts signalling a validation code upgrade.
       * In other words, if this is `Some` and [`NewValidationCode`] is `Some` then the produced
       * candidate will be invalid.
       * 
       * This storage item is a mirror of the corresponding value for the current parachain from the
       * relay-chain. This value is ephemeral which means it doesn't hit the storage. This value is
       * set after the inherent.
       **/
      upgradeRestrictionSignal: AugmentedQuery<ApiType, () => Observable<Option<PolkadotPrimitivesV5UpgradeRestriction>>, []>;
      /**
       * Upward messages that were sent in a block.
       * 
       * This will be cleared in `on_initialize` of each new block.
       **/
      upwardMessages: AugmentedQuery<ApiType, () => Observable<Vec<Bytes>>, []>;
      /**
       * The [`PersistedValidationData`] set for this block.
       * This value is expected to be set only once per block and it's never stored
       * in the trie.
       **/
      validationData: AugmentedQuery<ApiType, () => Observable<Option<PolkadotPrimitivesV5PersistedValidationData>>, []>;
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
    polkadotXcm: {
      /**
       * The existing asset traps.
       * 
       * Key is the blake2 256 hash of (origin, versioned `MultiAssets`) pair. Value is the number of
       * times this pair has been trapped (usually just 1 if it exists at all).
       **/
      assetTraps: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<u32>, [H256]>;
      /**
       * The current migration's stage, if any.
       **/
      currentMigration: AugmentedQuery<ApiType, () => Observable<Option<PalletXcmVersionMigrationStage>>, []>;
      /**
       * Fungible assets which we know are locked on this chain.
       **/
      lockedFungibles: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<Option<Vec<ITuple<[u128, XcmVersionedMultiLocation]>>>>, [AccountId32]>;
      /**
       * The ongoing queries.
       **/
      queries: AugmentedQuery<ApiType, (arg: u64 | AnyNumber | Uint8Array) => Observable<Option<PalletXcmQueryStatus>>, [u64]>;
      /**
       * The latest available query index.
       **/
      queryCounter: AugmentedQuery<ApiType, () => Observable<u64>, []>;
      /**
       * Fungible assets which we know are locked on a remote chain.
       **/
      remoteLockedFungibles: AugmentedQuery<ApiType, (arg1: u32 | AnyNumber | Uint8Array, arg2: AccountId32 | string | Uint8Array, arg3: XcmVersionedAssetId | { V3: any } | string | Uint8Array) => Observable<Option<PalletXcmRemoteLockedFungibleRecord>>, [u32, AccountId32, XcmVersionedAssetId]>;
      /**
       * Default version to encode XCM when latest version of destination is unknown. If `None`,
       * then the destinations whose XCM version is unknown are considered unreachable.
       **/
      safeXcmVersion: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []>;
      /**
       * The Latest versions that we know various locations support.
       **/
      supportedVersion: AugmentedQuery<ApiType, (arg1: u32 | AnyNumber | Uint8Array, arg2: XcmVersionedMultiLocation | { V2: any } | { V3: any } | string | Uint8Array) => Observable<Option<u32>>, [u32, XcmVersionedMultiLocation]>;
      /**
       * Destinations whose latest XCM version we would like to know. Duplicates not allowed, and
       * the `u32` counter is the number of times that a send to the destination has been attempted,
       * which is used as a prioritization.
       **/
      versionDiscoveryQueue: AugmentedQuery<ApiType, () => Observable<Vec<ITuple<[XcmVersionedMultiLocation, u32]>>>, []>;
      /**
       * All locations that we have requested version notifications from.
       **/
      versionNotifiers: AugmentedQuery<ApiType, (arg1: u32 | AnyNumber | Uint8Array, arg2: XcmVersionedMultiLocation | { V2: any } | { V3: any } | string | Uint8Array) => Observable<Option<u64>>, [u32, XcmVersionedMultiLocation]>;
      /**
       * The target locations that are subscribed to our version changes, as well as the most recent
       * of our versions we informed them of.
       **/
      versionNotifyTargets: AugmentedQuery<ApiType, (arg1: u32 | AnyNumber | Uint8Array, arg2: XcmVersionedMultiLocation | { V2: any } | { V3: any } | string | Uint8Array) => Observable<Option<ITuple<[u64, SpWeightsWeightV2Weight, u32]>>>, [u32, XcmVersionedMultiLocation]>;
      /**
       * Global suspension state of the XCM executor.
       **/
      xcmExecutionSuspended: AugmentedQuery<ApiType, () => Observable<bool>, []>;
    };
    preimage: {
      preimageFor: AugmentedQuery<ApiType, (arg: ITuple<[H256, u32]> | [H256 | string | Uint8Array, u32 | AnyNumber | Uint8Array]) => Observable<Option<Bytes>>, [ITuple<[H256, u32]>]>;
      /**
       * The request status of a given hash.
       **/
      statusFor: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Option<PalletPreimageRequestStatus>>, [H256]>;
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
    scheduler: {
      /**
       * Items to be executed, indexed by the block number that they should be executed on.
       **/
      agenda: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Vec<Option<PalletSchedulerScheduled>>>, [u32]>;
      incompleteSince: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []>;
      /**
       * Lookup from a name to the block number and index of the task.
       * 
       * For v3 -> v4 the previously unbounded identities are Blake2-256 hashed to form the v4
       * identities.
       **/
      lookup: AugmentedQuery<ApiType, (arg: U8aFixed | string | Uint8Array) => Observable<Option<ITuple<[u32, u32]>>>, [U8aFixed]>;
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
    session: {
      /**
       * Current index of the session.
       **/
      currentIndex: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      /**
       * Indices of disabled validators.
       * 
       * The vec is always kept sorted so that we can find whether a given validator is
       * disabled using binary search. It gets cleared when `on_session_ending` returns
       * a new set of identities.
       **/
      disabledValidators: AugmentedQuery<ApiType, () => Observable<Vec<u32>>, []>;
      /**
       * The owner of a key. The key is the `KeyTypeId` + the encoded key.
       **/
      keyOwner: AugmentedQuery<ApiType, (arg: ITuple<[SpCoreCryptoKeyTypeId, Bytes]> | [SpCoreCryptoKeyTypeId | string | Uint8Array, Bytes | string | Uint8Array]) => Observable<Option<AccountId32>>, [ITuple<[SpCoreCryptoKeyTypeId, Bytes]>]>;
      /**
       * The next session keys for a validator.
       **/
      nextKeys: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<Option<T0rnParachainRuntimeParachainConfigSessionKeys>>, [AccountId32]>;
      /**
       * True if the underlying economic identities or weighting behind the validators
       * has changed in the queued validator set.
       **/
      queuedChanged: AugmentedQuery<ApiType, () => Observable<bool>, []>;
      /**
       * The queued keys for the next session. When the next session begins, these keys
       * will be used to determine the validator's session keys.
       **/
      queuedKeys: AugmentedQuery<ApiType, () => Observable<Vec<ITuple<[AccountId32, T0rnParachainRuntimeParachainConfigSessionKeys]>>>, []>;
      /**
       * The current set of validators.
       **/
      validators: AugmentedQuery<ApiType, () => Observable<Vec<AccountId32>>, []>;
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
    xcmpQueue: {
      /**
       * Counter for the related counted storage map
       **/
      counterForOverweight: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      /**
       * Inbound aggregate XCMP messages. It can only be one per ParaId/block.
       **/
      inboundXcmpMessages: AugmentedQuery<ApiType, (arg1: u32 | AnyNumber | Uint8Array, arg2: u32 | AnyNumber | Uint8Array) => Observable<Bytes>, [u32, u32]>;
      /**
       * Status of the inbound XCMP channels.
       **/
      inboundXcmpStatus: AugmentedQuery<ApiType, () => Observable<Vec<CumulusPalletXcmpQueueInboundChannelDetails>>, []>;
      /**
       * The messages outbound in a given XCMP channel.
       **/
      outboundXcmpMessages: AugmentedQuery<ApiType, (arg1: u32 | AnyNumber | Uint8Array, arg2: u16 | AnyNumber | Uint8Array) => Observable<Bytes>, [u32, u16]>;
      /**
       * The non-empty XCMP channels in order of becoming non-empty, and the index of the first
       * and last outbound message. If the two indices are equal, then it indicates an empty
       * queue and there must be a non-`Ok` `OutboundStatus`. We assume queues grow no greater
       * than 65535 items. Queue indices for normal messages begin at one; zero is reserved in
       * case of the need to send a high-priority signal message this block.
       * The bool is true if there is a signal message waiting to be sent.
       **/
      outboundXcmpStatus: AugmentedQuery<ApiType, () => Observable<Vec<CumulusPalletXcmpQueueOutboundChannelDetails>>, []>;
      /**
       * The messages that exceeded max individual message weight budget.
       * 
       * These message stay in this storage map until they are manually dispatched via
       * `service_overweight`.
       **/
      overweight: AugmentedQuery<ApiType, (arg: u64 | AnyNumber | Uint8Array) => Observable<Option<ITuple<[u32, u32, Bytes]>>>, [u64]>;
      /**
       * The number of overweight messages ever recorded in `Overweight`. Also doubles as the next
       * available free overweight index.
       **/
      overweightCount: AugmentedQuery<ApiType, () => Observable<u64>, []>;
      /**
       * The configuration which controls the dynamics of the outbound queue.
       **/
      queueConfig: AugmentedQuery<ApiType, () => Observable<CumulusPalletXcmpQueueQueueConfigData>, []>;
      /**
       * Whether or not the XCMP queue is suspended from executing incoming XCMs or not.
       **/
      queueSuspended: AugmentedQuery<ApiType, () => Observable<bool>, []>;
      /**
       * Any signal messages waiting to be sent.
       **/
      signalMessages: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Bytes>, [u32]>;
    };
    xdns: {
      allGatewayIds: AugmentedQuery<ApiType, () => Observable<Vec<U8aFixed>>, []>;
      allTokenIds: AugmentedQuery<ApiType, () => Observable<Vec<u32>>, []>;
      assetCostEstimatesInNative: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<u128>, [u32]>;
      assetEstimates: AugmentedQuery<ApiType, (arg: ITuple<[u32, u32]> | [u32 | AnyNumber | Uint8Array, u32 | AnyNumber | Uint8Array]) => Observable<u128>, [ITuple<[u32, u32]>]>;
      assetEstimatesInNative: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<u128>, [u32]>;
      customSideEffects: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Option<Bytes>>, [H256]>;
      epochHistory: AugmentedQuery<ApiType, (arg: T3rnPrimitivesGatewayVendor | 'Polkadot' | 'Kusama' | 'Rococo' | 'Ethereum' | 'Sepolia' | 'XBI' | number | Uint8Array) => Observable<Option<Vec<T3rnPrimitivesXdnsEpochEstimate>>>, [T3rnPrimitivesGatewayVendor]>;
      gateways: AugmentedQuery<ApiType, (arg: U8aFixed | string | Uint8Array) => Observable<Option<T3rnPrimitivesXdnsGatewayRecord>>, [U8aFixed]>;
      gatewaysOverviewStore: AugmentedQuery<ApiType, () => Observable<Vec<T3rnPrimitivesGatewayActivity>>, []>;
      gatewaysOverviewStoreHistory: AugmentedQuery<ApiType, (arg: U8aFixed | string | Uint8Array) => Observable<Vec<T3rnPrimitivesGatewayActivity>>, [U8aFixed]>;
      gatewayTokens: AugmentedQuery<ApiType, (arg: U8aFixed | string | Uint8Array) => Observable<Vec<u32>>, [U8aFixed]>;
      perTargetAssetEstimates: AugmentedQuery<ApiType, (arg1: U8aFixed | string | Uint8Array, arg2: ITuple<[u32, u32]> | [u32 | AnyNumber | Uint8Array, u32 | AnyNumber | Uint8Array]) => Observable<u128>, [U8aFixed, ITuple<[u32, u32]>]>;
      sfxabiRegistry: AugmentedQuery<ApiType, (arg1: U8aFixed | string | Uint8Array, arg2: U8aFixed | string | Uint8Array) => Observable<Option<T3rnAbiSfxAbi>>, [U8aFixed, U8aFixed]>;
      standardSFXABIs: AugmentedQuery<ApiType, (arg: U8aFixed | string | Uint8Array) => Observable<Option<T3rnAbiSfxAbi>>, [U8aFixed]>;
      standardSideEffects: AugmentedQuery<ApiType, (arg: U8aFixed | string | Uint8Array) => Observable<Option<Bytes>>, [U8aFixed]>;
      storageMigrations: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      tokens: AugmentedQuery<ApiType, (arg1: u32 | AnyNumber | Uint8Array, arg2: U8aFixed | string | Uint8Array) => Observable<Option<T3rnPrimitivesXdnsTokenRecord>>, [u32, U8aFixed]>;
      verifierOverviewStore: AugmentedQuery<ApiType, () => Observable<Vec<T3rnPrimitivesFinalityVerifierActivity>>, []>;
      verifierOverviewStoreHistory: AugmentedQuery<ApiType, (arg: T3rnPrimitivesGatewayVendor | 'Polkadot' | 'Kusama' | 'Rococo' | 'Ethereum' | 'Sepolia' | 'XBI' | number | Uint8Array) => Observable<Vec<T3rnPrimitivesFinalityVerifierActivity>>, [T3rnPrimitivesGatewayVendor]>;
    };
  } // AugmentedQueries
} // declare module
