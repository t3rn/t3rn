// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import "@polkadot/api-base/types/storage";

import type {
  ApiTypes,
  AugmentedQuery,
  QueryableStorageEntry,
} from "@polkadot/api-base/types";
import type {
  Bytes,
  Option,
  Struct,
  U8aFixed,
  Vec,
  bool,
  u128,
  u32,
  u64,
  u8,
} from "@polkadot/types-codec";
import type { AnyNumber, ITuple } from "@polkadot/types-codec/types";
import type {
  AccountId32,
  H160,
  H256,
} from "@polkadot/types/interfaces/runtime";
import type {
  FrameSupportWeightsPerDispatchClassU64,
  FrameSystemAccountInfo,
  FrameSystemEventRecord,
  FrameSystemLastRuntimeUpgradeInfo,
  FrameSystemPhase,
  OrmlTokensAccountData,
  OrmlTokensBalanceLock,
  PalletBalancesAccountData,
  PalletBalancesBalanceLock,
  PalletBalancesReleases,
  PalletBalancesReserveData,
  PalletCircuitStateInsuranceDeposit,
  PalletCircuitStateXExecSignal,
  PalletContractsStorageDeletedContract,
  PalletContractsStorageRawContractInfo,
  PalletContractsWasmOwnerInfo,
  PalletContractsWasmPrefabWasmModule,
  PalletEvmThreeVmInfo,
  PalletGrandpaFinalityVerifierBridgesHeaderChainAuthoritySet,
  PalletGrandpaFinalityVerifierParachain,
  PalletGrandpaStoredPendingChange,
  PalletGrandpaStoredState,
  PalletTransactionPaymentReleases,
  PalletTreasuryInflationInflationInfo,
  PalletXbiPortalCall,
  PalletXbiPortalXbiFormatXbiCheckIn,
  PalletXbiPortalXbiFormatXbiCheckOut,
  SpConsensusAuraSr25519AppSr25519Public,
  SpRuntimeDigest,
  SpRuntimeHeader,
  T3rnPrimitivesAccountManagerRequestCharge,
  T3rnPrimitivesAccountManagerSettlement,
  T3rnPrimitivesClaimableClaimableArtifacts,
  T3rnPrimitivesCommonRoundInfo,
  T3rnPrimitivesContractsRegistryRegistryContract,
  T3rnPrimitivesMonetaryBeneficiaryRole,
  T3rnPrimitivesMonetaryInflationAllocation,
  T3rnPrimitivesSideEffectFullSideEffect,
  T3rnPrimitivesSideEffectInterfaceSideEffectInterface,
  T3rnPrimitivesVolatileLocalState,
  T3rnPrimitivesXdnsXdnsRecord,
  T3rnSdkPrimitivesSignalExecutionSignal,
} from "@polkadot/types/lookup";
import type { Observable } from "@polkadot/types/types";

export type __AugmentedQuery<ApiType extends ApiTypes> = AugmentedQuery<
  ApiType,
  () => unknown
>;
export type __QueryableStorageEntry<ApiType extends ApiTypes> =
  QueryableStorageEntry<ApiType>;

declare module "@polkadot/api-base/types/storage" {
  interface AugmentedQueries<ApiType extends ApiTypes> {
    accountManager: {
      contractsRegistryExecutionNonce: AugmentedQuery<
        ApiType,
        () => Observable<u64>,
        []
      > &
        QueryableStorageEntry<ApiType, []>;
      pendingChargesPerRound: AugmentedQuery<
        ApiType,
        (
          arg1:
            | T3rnPrimitivesCommonRoundInfo
            | { index?: any; head?: any; term?: any }
            | string
            | Uint8Array,
          arg2: H256 | string | Uint8Array
        ) => Observable<Option<T3rnPrimitivesAccountManagerRequestCharge>>,
        [T3rnPrimitivesCommonRoundInfo, H256]
      > &
        QueryableStorageEntry<ApiType, [T3rnPrimitivesCommonRoundInfo, H256]>;
      settlementsPerRound: AugmentedQuery<
        ApiType,
        (
          arg1:
            | T3rnPrimitivesCommonRoundInfo
            | { index?: any; head?: any; term?: any }
            | string
            | Uint8Array,
          arg2: H256 | string | Uint8Array
        ) => Observable<Option<T3rnPrimitivesAccountManagerSettlement>>,
        [T3rnPrimitivesCommonRoundInfo, H256]
      > &
        QueryableStorageEntry<ApiType, [T3rnPrimitivesCommonRoundInfo, H256]>;
      /** Generic query */
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    aura: {
      /** The current authority set. */
      authorities: AugmentedQuery<
        ApiType,
        () => Observable<Vec<SpConsensusAuraSr25519AppSr25519Public>>,
        []
      > &
        QueryableStorageEntry<ApiType, []>;
      /**
       * The current slot of this block.
       *
       * This will be set in `on_initialize`.
       */
      currentSlot: AugmentedQuery<ApiType, () => Observable<u64>, []> &
        QueryableStorageEntry<ApiType, []>;
      /** Generic query */
      [key: string]: QueryableStorageEntry<ApiType>;
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
       * But this comes with tradeoffs, storing account balances in the system
       * pallet stores `frame_system` data alongside the account data contrary
       * to storing account balances in the `Balances` pallet, which uses a
       * `StorageMap` to store balances data only. NOTE: This is only used in
       * the case that this pallet is used to store balances.
       */
      account: AugmentedQuery<
        ApiType,
        (
          arg: AccountId32 | string | Uint8Array
        ) => Observable<PalletBalancesAccountData>,
        [AccountId32]
      > &
        QueryableStorageEntry<ApiType, [AccountId32]>;
      /**
       * Any liquidity locks on some account balances. NOTE: Should only be
       * accessed when setting, changing and freeing a lock.
       */
      locks: AugmentedQuery<
        ApiType,
        (
          arg: AccountId32 | string | Uint8Array
        ) => Observable<Vec<PalletBalancesBalanceLock>>,
        [AccountId32]
      > &
        QueryableStorageEntry<ApiType, [AccountId32]>;
      /** Named reserves on some account balances. */
      reserves: AugmentedQuery<
        ApiType,
        (
          arg: AccountId32 | string | Uint8Array
        ) => Observable<Vec<PalletBalancesReserveData>>,
        [AccountId32]
      > &
        QueryableStorageEntry<ApiType, [AccountId32]>;
      /**
       * Storage version of the pallet.
       *
       * This is set to v2.0.0 for new networks.
       */
      storageVersion: AugmentedQuery<
        ApiType,
        () => Observable<PalletBalancesReleases>,
        []
      > &
        QueryableStorageEntry<ApiType, []>;
      /** The total units issued in the system. */
      totalIssuance: AugmentedQuery<ApiType, () => Observable<u128>, []> &
        QueryableStorageEntry<ApiType, []>;
      /** Generic query */
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    circuit: {
      /** Current Circuit's context of active insurance deposits */
      activeXExecSignalsTimingLinks: AugmentedQuery<
        ApiType,
        (arg: H256 | string | Uint8Array) => Observable<Option<u32>>,
        [H256]
      > &
        QueryableStorageEntry<ApiType, [H256]>;
      /**
       * Current Circuit's context of active full side effects (requested +
       * confirmation proofs)
       */
      fullSideEffects: AugmentedQuery<
        ApiType,
        (
          arg: H256 | string | Uint8Array
        ) => Observable<
          Option<Vec<Vec<T3rnPrimitivesSideEffectFullSideEffect>>>
        >,
        [H256]
      > &
        QueryableStorageEntry<ApiType, [H256]>;
      /** Current Circuit's context of active insurance deposits */
      insuranceDeposits: AugmentedQuery<
        ApiType,
        (
          arg1: H256 | string | Uint8Array,
          arg2: H256 | string | Uint8Array
        ) => Observable<Option<PalletCircuitStateInsuranceDeposit>>,
        [H256, H256]
      > &
        QueryableStorageEntry<ApiType, [H256, H256]>;
      /** Current Circuit's context of active insurance deposits */
      localSideEffectToXtxIdLinks: AugmentedQuery<
        ApiType,
        (arg: H256 | string | Uint8Array) => Observable<Option<H256>>,
        [H256]
      > &
        QueryableStorageEntry<ApiType, [H256]>;
      /**
       * Current Circuit's context of active full side effects (requested +
       * confirmation proofs)
       */
      localXtxStates: AugmentedQuery<
        ApiType,
        (
          arg: H256 | string | Uint8Array
        ) => Observable<Option<T3rnPrimitivesVolatileLocalState>>,
        [H256]
      > &
        QueryableStorageEntry<ApiType, [H256]>;
      /**
       * Handles queued signals
       *
       * This operation is performed lazily in `on_initialize`.
       */
      signalQueue: AugmentedQuery<
        ApiType,
        () => Observable<
          Vec<ITuple<[AccountId32, T3rnSdkPrimitivesSignalExecutionSignal]>>
        >,
        []
      > &
        QueryableStorageEntry<ApiType, []>;
      /** Current Circuit's context of active transactions */
      xExecSignals: AugmentedQuery<
        ApiType,
        (
          arg: H256 | string | Uint8Array
        ) => Observable<Option<PalletCircuitStateXExecSignal>>,
        [H256]
      > &
        QueryableStorageEntry<ApiType, [H256]>;
      /**
       * Current Circuit's context of active full side effects (requested +
       * confirmation proofs)
       */
      xtxInsuranceLinks: AugmentedQuery<
        ApiType,
        (arg: H256 | string | Uint8Array) => Observable<Vec<H256>>,
        [H256]
      > &
        QueryableStorageEntry<ApiType, [H256]>;
      /** Generic query */
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    clock: {
      claimableArtifactsPerRound: AugmentedQuery<
        ApiType,
        (
          arg:
            | T3rnPrimitivesCommonRoundInfo
            | { index?: any; head?: any; term?: any }
            | string
            | Uint8Array
        ) => Observable<Option<Vec<T3rnPrimitivesClaimableClaimableArtifacts>>>,
        [T3rnPrimitivesCommonRoundInfo]
      > &
        QueryableStorageEntry<ApiType, [T3rnPrimitivesCommonRoundInfo]>;
      lastClaims: AugmentedQuery<
        ApiType,
        (
          arg: AccountId32 | string | Uint8Array
        ) => Observable<Option<T3rnPrimitivesCommonRoundInfo>>,
        [AccountId32]
      > &
        QueryableStorageEntry<ApiType, [AccountId32]>;
      /** Generic query */
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    contracts: {
      /**
       * A mapping between an original code hash and instrumented wasm code,
       * ready for execution.
       */
      codeStorage: AugmentedQuery<
        ApiType,
        (
          arg: H256 | string | Uint8Array
        ) => Observable<Option<PalletContractsWasmPrefabWasmModule>>,
        [H256]
      > &
        QueryableStorageEntry<ApiType, [H256]>;
      /**
       * The code associated with a given account.
       *
       * TWOX-NOTE: SAFE since `AccountId` is a secure hash.
       */
      contractInfoOf: AugmentedQuery<
        ApiType,
        (
          arg: AccountId32 | string | Uint8Array
        ) => Observable<Option<PalletContractsStorageRawContractInfo>>,
        [AccountId32]
      > &
        QueryableStorageEntry<ApiType, [AccountId32]>;
      /**
       * Evicted contracts that await child trie deletion.
       *
       * Child trie deletion is a heavy operation depending on the amount of
       * storage items stored in said trie. Therefore this operation is
       * performed lazily in `on_initialize`.
       */
      deletionQueue: AugmentedQuery<
        ApiType,
        () => Observable<Vec<PalletContractsStorageDeletedContract>>,
        []
      > &
        QueryableStorageEntry<ApiType, []>;
      /**
       * This is a **monotonic** counter incremented on contract instantiation.
       *
       * This is used in order to generate unique trie ids for contracts. The
       * trie id of a new contract is calculated from hash(account_id, nonce).
       * The nonce is required because otherwise the following sequence would
       * lead to a possible collision of storage:
       *
       * 1. Create a new contract.
       * 2. Terminate the contract.
       * 3. Immediately recreate the contract with the same account_id.
       *
       * This is bad because the contents of a trie are deleted lazily and there
       * might be storage of the old instantiation still in it when the new
       * contract is created. Please note that we can't replace the counter by
       * the block number because the sequence above can happen in the same
       * block. We also can't keep the account counter in memory only because
       * storage is the only way to communicate across different extrinsics in
       * the same block.
       *
       * # Note
       *
       * Do not use it to determine the number of contracts. It won't be
       * decremented if a contract is destroyed.
       */
      nonce: AugmentedQuery<ApiType, () => Observable<u64>, []> &
        QueryableStorageEntry<ApiType, []>;
      /** A mapping between an original code hash and its owner information. */
      ownerInfoOf: AugmentedQuery<
        ApiType,
        (
          arg: H256 | string | Uint8Array
        ) => Observable<Option<PalletContractsWasmOwnerInfo>>,
        [H256]
      > &
        QueryableStorageEntry<ApiType, [H256]>;
      /**
       * A mapping from an original code hash to the original code, untouched by
       * instrumentation.
       */
      pristineCode: AugmentedQuery<
        ApiType,
        (arg: H256 | string | Uint8Array) => Observable<Option<Bytes>>,
        [H256]
      > &
        QueryableStorageEntry<ApiType, [H256]>;
      /** Generic query */
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    contractsRegistry: {
      /** The pre-validated composable contracts on-chain registry. */
      contractsRegistry: AugmentedQuery<
        ApiType,
        (
          arg: H256 | string | Uint8Array
        ) => Observable<
          Option<T3rnPrimitivesContractsRegistryRegistryContract>
        >,
        [H256]
      > &
        QueryableStorageEntry<ApiType, [H256]>;
      /** Generic query */
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    evm: {
      account3vmInfo: AugmentedQuery<
        ApiType,
        (
          arg: H160 | string | Uint8Array
        ) => Observable<Option<PalletEvmThreeVmInfo>>,
        [H160]
      > &
        QueryableStorageEntry<ApiType, [H160]>;
      accountCodes: AugmentedQuery<
        ApiType,
        (arg: H160 | string | Uint8Array) => Observable<Bytes>,
        [H160]
      > &
        QueryableStorageEntry<ApiType, [H160]>;
      accountEvmAddressMapping: AugmentedQuery<
        ApiType,
        (arg: AccountId32 | string | Uint8Array) => Observable<Option<H160>>,
        [AccountId32]
      > &
        QueryableStorageEntry<ApiType, [AccountId32]>;
      /**
       * The storages for EVM contracts.
       *
       * AccountStorages: double_map EvmAddress, H256 => H256
       */
      accountStorages: AugmentedQuery<
        ApiType,
        (
          arg1: H160 | string | Uint8Array,
          arg2: H256 | string | Uint8Array
        ) => Observable<H256>,
        [H160, H256]
      > &
        QueryableStorageEntry<ApiType, [H160, H256]>;
      evmAccountAddressMapping: AugmentedQuery<
        ApiType,
        (arg: H160 | string | Uint8Array) => Observable<Option<AccountId32>>,
        [H160]
      > &
        QueryableStorageEntry<ApiType, [H160]>;
      /** Generic query */
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    grandpa: {
      /**
       * The number of changes (both in terms of keys and underlying economic
       * responsibilities) in the "set" of Grandpa validators from genesis.
       */
      currentSetId: AugmentedQuery<ApiType, () => Observable<u64>, []> &
        QueryableStorageEntry<ApiType, []>;
      /** Next block number where we can force a change. */
      nextForced: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []> &
        QueryableStorageEntry<ApiType, []>;
      /** Pending change: (signaled at, scheduled change). */
      pendingChange: AugmentedQuery<
        ApiType,
        () => Observable<Option<PalletGrandpaStoredPendingChange>>,
        []
      > &
        QueryableStorageEntry<ApiType, []>;
      /**
       * A mapping from grandpa set ID to the index of the _most recent_ session
       * for which its members were responsible.
       *
       * TWOX-NOTE: `SetId` is not under user control.
       */
      setIdSession: AugmentedQuery<
        ApiType,
        (arg: u64 | AnyNumber | Uint8Array) => Observable<Option<u32>>,
        [u64]
      > &
        QueryableStorageEntry<ApiType, [u64]>;
      /** `true` if we are currently stalled. */
      stalled: AugmentedQuery<
        ApiType,
        () => Observable<Option<ITuple<[u32, u32]>>>,
        []
      > &
        QueryableStorageEntry<ApiType, []>;
      /** State of the current authority set. */
      state: AugmentedQuery<
        ApiType,
        () => Observable<PalletGrandpaStoredState>,
        []
      > &
        QueryableStorageEntry<ApiType, []>;
      /** Generic query */
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    ormlTokens: {
      /**
       * The balance of a token type under an account.
       *
       * NOTE: If the total is ever zero, decrease account ref account.
       *
       * NOTE: This is only used in the case that this module is used to store balances.
       */
      accounts: AugmentedQuery<
        ApiType,
        (
          arg1: AccountId32 | string | Uint8Array,
          arg2: u32 | AnyNumber | Uint8Array
        ) => Observable<OrmlTokensAccountData>,
        [AccountId32, u32]
      > &
        QueryableStorageEntry<ApiType, [AccountId32, u32]>;
      /**
       * Any liquidity locks of a token type under an account. NOTE: Should only
       * be accessed when setting, changing and freeing a lock.
       */
      locks: AugmentedQuery<
        ApiType,
        (
          arg1: AccountId32 | string | Uint8Array,
          arg2: u32 | AnyNumber | Uint8Array
        ) => Observable<Vec<OrmlTokensBalanceLock>>,
        [AccountId32, u32]
      > &
        QueryableStorageEntry<ApiType, [AccountId32, u32]>;
      /** The total issuance of a token type. */
      totalIssuance: AugmentedQuery<
        ApiType,
        (arg: u32 | AnyNumber | Uint8Array) => Observable<u128>,
        [u32]
      > &
        QueryableStorageEntry<ApiType, [u32]>;
      /** Generic query */
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    portal: {
      /** Generic query */
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    randomnessCollectiveFlip: {
      /**
       * Series of block headers from the last 81 blocks that acts as random
       * seed material. This is arranged as a ring buffer with `block_number %
       * 81` being the index into the `Vec` of the oldest hash.
       */
      randomMaterial: AugmentedQuery<ApiType, () => Observable<Vec<H256>>, []> &
        QueryableStorageEntry<ApiType, []>;
      /** Generic query */
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    rococoBridge: {
      /** Map of hashes of the best finalized header. */
      bestFinalizedMap: AugmentedQuery<
        ApiType,
        (arg: U8aFixed | string | Uint8Array) => Observable<Option<H256>>,
        [U8aFixed]
      > &
        QueryableStorageEntry<ApiType, [U8aFixed]>;
      /** The current GRANDPA Authority set. */
      currentAuthoritySet: AugmentedQuery<
        ApiType,
        () => Observable<
          Option<PalletGrandpaFinalityVerifierBridgesHeaderChainAuthoritySet>
        >,
        []
      > &
        QueryableStorageEntry<ApiType, []>;
      /** Hash of the header used to bootstrap the pallet. */
      initialHashMap: AugmentedQuery<
        ApiType,
        (arg: U8aFixed | string | Uint8Array) => Observable<Option<H256>>,
        [U8aFixed]
      > &
        QueryableStorageEntry<ApiType, [U8aFixed]>;
      /** Map of instance ids of gateways which are active */
      instantiatedGatewaysMap: AugmentedQuery<
        ApiType,
        () => Observable<Vec<U8aFixed>>,
        []
      > &
        QueryableStorageEntry<ApiType, []>;
      /** If true, all pallet transactions are failed immediately. */
      isHalted: AugmentedQuery<ApiType, () => Observable<bool>, []> &
        QueryableStorageEntry<ApiType, []>;
      /** If true, all pallet transactions are failed immediately. */
      isHaltedMap: AugmentedQuery<
        ApiType,
        (arg: U8aFixed | string | Uint8Array) => Observable<Option<bool>>,
        [U8aFixed]
      > &
        QueryableStorageEntry<ApiType, [U8aFixed]>;
      /** A ring buffer of imported hashes. Ordered by the insertion time. */
      multiImportedHashes: AugmentedQuery<
        ApiType,
        (
          arg1: U8aFixed | string | Uint8Array,
          arg2: u32 | AnyNumber | Uint8Array
        ) => Observable<Option<H256>>,
        [U8aFixed, u32]
      > &
        QueryableStorageEntry<ApiType, [U8aFixed, u32]>;
      /** Current ring buffer position. */
      multiImportedHashesPointer: AugmentedQuery<
        ApiType,
        (arg: U8aFixed | string | Uint8Array) => Observable<Option<u32>>,
        [U8aFixed]
      > &
        QueryableStorageEntry<ApiType, [U8aFixed]>;
      /** Headers which have been imported into the pallet. */
      multiImportedHeaders: AugmentedQuery<
        ApiType,
        (
          arg1: U8aFixed | string | Uint8Array,
          arg2: H256 | string | Uint8Array
        ) => Observable<Option<SpRuntimeHeader>>,
        [U8aFixed, H256]
      > &
        QueryableStorageEntry<ApiType, [U8aFixed, H256]>;
      /**
       * Roots (ExtrinsicsRoot + StateRoot) which have been imported into the
       * pallet for a given gateway.
       */
      multiImportedRoots: AugmentedQuery<
        ApiType,
        (
          arg1: U8aFixed | string | Uint8Array,
          arg2: H256 | string | Uint8Array
        ) => Observable<Option<ITuple<[H256, H256]>>>,
        [U8aFixed, H256]
      > &
        QueryableStorageEntry<ApiType, [U8aFixed, H256]>;
      /**
       * Optional pallet owner.
       *
       * Pallet owner has a right to halt all pallet operations and then resume
       * it. If it is `None`, then there are no direct ways to halt/resume
       * pallet operations, but other runtime methods may still be used to do
       * that (i.e. democracy::referendum to update halt flag directly or call
       * the `halt_operations`).
       */
      palletOwnerMap: AugmentedQuery<
        ApiType,
        (
          arg: U8aFixed | string | Uint8Array
        ) => Observable<Option<AccountId32>>,
        [U8aFixed]
      > &
        QueryableStorageEntry<ApiType, [U8aFixed]>;
      /** The current GRANDPA Authority set. */
      parachainIdMap: AugmentedQuery<
        ApiType,
        (
          arg: U8aFixed | string | Uint8Array
        ) => Observable<Option<PalletGrandpaFinalityVerifierParachain>>,
        [U8aFixed]
      > &
        QueryableStorageEntry<ApiType, [U8aFixed]>;
      relayChainId: AugmentedQuery<
        ApiType,
        () => Observable<Option<U8aFixed>>,
        []
      > &
        QueryableStorageEntry<ApiType, []>;
      /** Generic query */
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    sudo: {
      /** The `AccountId` of the sudo key. */
      key: AugmentedQuery<ApiType, () => Observable<Option<AccountId32>>, []> &
        QueryableStorageEntry<ApiType, []>;
      /** Generic query */
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    system: {
      /** The full account information for a particular account ID. */
      account: AugmentedQuery<
        ApiType,
        (
          arg: AccountId32 | string | Uint8Array
        ) => Observable<FrameSystemAccountInfo>,
        [AccountId32]
      > &
        QueryableStorageEntry<ApiType, [AccountId32]>;
      /** Total length (in bytes) for all extrinsics put together, for the current block. */
      allExtrinsicsLen: AugmentedQuery<
        ApiType,
        () => Observable<Option<u32>>,
        []
      > &
        QueryableStorageEntry<ApiType, []>;
      /** Map of block numbers to block hashes. */
      blockHash: AugmentedQuery<
        ApiType,
        (arg: u32 | AnyNumber | Uint8Array) => Observable<H256>,
        [u32]
      > &
        QueryableStorageEntry<ApiType, [u32]>;
      /** The current weight for the block. */
      blockWeight: AugmentedQuery<
        ApiType,
        () => Observable<FrameSupportWeightsPerDispatchClassU64>,
        []
      > &
        QueryableStorageEntry<ApiType, []>;
      /** Digest of the current block, also part of the block header. */
      digest: AugmentedQuery<ApiType, () => Observable<SpRuntimeDigest>, []> &
        QueryableStorageEntry<ApiType, []>;
      /** The number of events in the `Events<T>` list. */
      eventCount: AugmentedQuery<ApiType, () => Observable<u32>, []> &
        QueryableStorageEntry<ApiType, []>;
      /**
       * Events deposited for the current block.
       *
       * NOTE: This storage item is explicitly unbounded since it is never
       * intended to be read from within the runtime.
       */
      events: AugmentedQuery<
        ApiType,
        () => Observable<Vec<FrameSystemEventRecord>>,
        []
      > &
        QueryableStorageEntry<ApiType, []>;
      /**
       * Mapping between a topic (represented by T::Hash) and a vector of
       * indexes of events in the `<Events<T>>` list.
       *
       * All topic vectors have deterministic storage locations depending on the
       * topic. This allows light-clients to leverage the changes trie storage
       * tracking mechanism and in case of changes fetch the list of events of interest.
       *
       * The value has the type `(T::BlockNumber, EventIndex)` because if we
       * used only just the `EventIndex` then in case if the topic has the same
       * contents on the next block no notification will be triggered thus the
       * event might be lost.
       */
      eventTopics: AugmentedQuery<
        ApiType,
        (
          arg: H256 | string | Uint8Array
        ) => Observable<Vec<ITuple<[u32, u32]>>>,
        [H256]
      > &
        QueryableStorageEntry<ApiType, [H256]>;
      /** The execution phase of the block. */
      executionPhase: AugmentedQuery<
        ApiType,
        () => Observable<Option<FrameSystemPhase>>,
        []
      > &
        QueryableStorageEntry<ApiType, []>;
      /** Total extrinsics count for the current block. */
      extrinsicCount: AugmentedQuery<
        ApiType,
        () => Observable<Option<u32>>,
        []
      > &
        QueryableStorageEntry<ApiType, []>;
      /** Extrinsics data for the current block (maps an extrinsic's index to its data). */
      extrinsicData: AugmentedQuery<
        ApiType,
        (arg: u32 | AnyNumber | Uint8Array) => Observable<Bytes>,
        [u32]
      > &
        QueryableStorageEntry<ApiType, [u32]>;
      /**
       * Stores the `spec_version` and `spec_name` of when the last runtime
       * upgrade happened.
       */
      lastRuntimeUpgrade: AugmentedQuery<
        ApiType,
        () => Observable<Option<FrameSystemLastRuntimeUpgradeInfo>>,
        []
      > &
        QueryableStorageEntry<ApiType, []>;
      /** The current block number being processed. Set by `execute_block`. */
      number: AugmentedQuery<ApiType, () => Observable<u32>, []> &
        QueryableStorageEntry<ApiType, []>;
      /** Hash of the previous block. */
      parentHash: AugmentedQuery<ApiType, () => Observable<H256>, []> &
        QueryableStorageEntry<ApiType, []>;
      /**
       * True if we have upgraded so that AccountInfo contains three types of
       * `RefCount`. False (default) if not.
       */
      upgradedToTripleRefCount: AugmentedQuery<
        ApiType,
        () => Observable<bool>,
        []
      > &
        QueryableStorageEntry<ApiType, []>;
      /**
       * True if we have upgraded so that `type RefCount` is `u32`. False
       * (default) if not.
       */
      upgradedToU32RefCount: AugmentedQuery<
        ApiType,
        () => Observable<bool>,
        []
      > &
        QueryableStorageEntry<ApiType, []>;
      /** Generic query */
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    threeVm: {
      /** A mapping of a contract's address to its author. */
      authorOf: AugmentedQuery<
        ApiType,
        (
          arg: AccountId32 | string | Uint8Array
        ) => Observable<Option<AccountId32>>,
        [AccountId32]
      > &
        QueryableStorageEntry<ApiType, [AccountId32]>;
      /** A mapping of precompile pointers */
      precompileIndex: AugmentedQuery<
        ApiType,
        (arg: H256 | string | Uint8Array) => Observable<Option<u8>>,
        [H256]
      > &
        QueryableStorageEntry<ApiType, [H256]>;
      /** Holds the amount of times the signal was posted or attempted to be posted */
      signals: AugmentedQuery<
        ApiType,
        (
          arg1: H256 | string | Uint8Array,
          arg2: u32 | AnyNumber | Uint8Array
        ) => Observable<Option<u32>>,
        [H256, u32]
      > &
        QueryableStorageEntry<ApiType, [H256, u32]>;
      /** Generic query */
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    timestamp: {
      /** Did the timestamp get updated in this block? */
      didUpdate: AugmentedQuery<ApiType, () => Observable<bool>, []> &
        QueryableStorageEntry<ApiType, []>;
      /** Current time for the current block. */
      now: AugmentedQuery<ApiType, () => Observable<u64>, []> &
        QueryableStorageEntry<ApiType, []>;
      /** Generic query */
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    transactionPayment: {
      nextFeeMultiplier: AugmentedQuery<ApiType, () => Observable<u128>, []> &
        QueryableStorageEntry<ApiType, []>;
      storageVersion: AugmentedQuery<
        ApiType,
        () => Observable<PalletTransactionPaymentReleases>,
        []
      > &
        QueryableStorageEntry<ApiType, []>;
      /** Generic query */
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    treasury: {
      beneficiaries: AugmentedQuery<
        ApiType,
        (
          arg1: AccountId32 | string | Uint8Array,
          arg2:
            | T3rnPrimitivesMonetaryBeneficiaryRole
            | "Developer"
            | "Executor"
            | number
            | Uint8Array
        ) => Observable<Option<u128>>,
        [AccountId32, T3rnPrimitivesMonetaryBeneficiaryRole]
      > &
        QueryableStorageEntry<
          ApiType,
          [AccountId32, T3rnPrimitivesMonetaryBeneficiaryRole]
        >;
      beneficiaryRoundRewards: AugmentedQuery<
        ApiType,
        (
          arg1: AccountId32 | string | Uint8Array,
          arg2: u32 | AnyNumber | Uint8Array
        ) => Observable<u128>,
        [AccountId32, u32]
      > &
        QueryableStorageEntry<ApiType, [AccountId32, u32]>;
      /** Information on the current treasury round. */
      currentRound: AugmentedQuery<
        ApiType,
        () => Observable<T3rnPrimitivesCommonRoundInfo>,
        []
      > &
        QueryableStorageEntry<ApiType, []>;
      /** The pallet's rewards allocation config. */
      inflationAlloc: AugmentedQuery<
        ApiType,
        () => Observable<T3rnPrimitivesMonetaryInflationAllocation>,
        []
      > &
        QueryableStorageEntry<ApiType, []>;
      /** The pallet's inflation mechanism configuration. */
      inflationConfig: AugmentedQuery<
        ApiType,
        () => Observable<PalletTreasuryInflationInflationInfo>,
        []
      > &
        QueryableStorageEntry<ApiType, []>;
      /**
       * Expected total stake sum of active executors and collators. Active
       * means member of the respective active set.
       */
      totalStakeExpectation: AugmentedQuery<
        ApiType,
        () => Observable<
          {
            readonly min: u128;
            readonly ideal: u128;
            readonly max: u128;
          } & Struct
        >,
        []
      > &
        QueryableStorageEntry<ApiType, []>;
      /** Generic query */
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    xbiPortal: {
      /** XBI called for execution */
      xbiCheckIns: AugmentedQuery<
        ApiType,
        (
          arg: H256 | string | Uint8Array
        ) => Observable<Option<PalletXbiPortalXbiFormatXbiCheckIn>>,
        [H256]
      > &
        QueryableStorageEntry<ApiType, [H256]>;
      /** Processed XBI queue pending for execution */
      xbiCheckInsPending: AugmentedQuery<
        ApiType,
        (
          arg: H256 | string | Uint8Array
        ) => Observable<Option<PalletXbiPortalXbiFormatXbiCheckIn>>,
        [H256]
      > &
        QueryableStorageEntry<ApiType, [H256]>;
      /** Queue XBI for batch execution */
      xbiCheckInsQueued: AugmentedQuery<
        ApiType,
        (
          arg: H256 | string | Uint8Array
        ) => Observable<Option<PalletXbiPortalXbiFormatXbiCheckIn>>,
        [H256]
      > &
        QueryableStorageEntry<ApiType, [H256]>;
      /** XBI Results of execution on local (here) Parachain */
      xbiCheckOuts: AugmentedQuery<
        ApiType,
        (
          arg: H256 | string | Uint8Array
        ) => Observable<Option<PalletXbiPortalXbiFormatXbiCheckOut>>,
        [H256]
      > &
        QueryableStorageEntry<ApiType, [H256]>;
      /**
       * Lifecycle: If executed: XBICheckInsPending -> XBICheckIns ->
       * XBICheckOutsQueued Lifecycle: If not executed: XBICheckInsPending ->
       * XBICheckOutsQueued
       */
      xbiCheckOutsQueued: AugmentedQuery<
        ApiType,
        (
          arg: H256 | string | Uint8Array
        ) => Observable<Option<PalletXbiPortalXbiFormatXbiCheckOut>>,
        [H256]
      > &
        QueryableStorageEntry<ApiType, [H256]>;
      /** Processed XBI queue pending for execution */
      xbiPromisePendingCallbacks: AugmentedQuery<
        ApiType,
        (
          arg: H256 | string | Uint8Array
        ) => Observable<Option<PalletXbiPortalCall>>,
        [H256]
      > &
        QueryableStorageEntry<ApiType, [H256]>;
      /** Generic query */
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    xdns: {
      customSideEffects: AugmentedQuery<
        ApiType,
        (
          arg: H256 | string | Uint8Array
        ) => Observable<
          Option<T3rnPrimitivesSideEffectInterfaceSideEffectInterface>
        >,
        [H256]
      > &
        QueryableStorageEntry<ApiType, [H256]>;
      standardSideEffects: AugmentedQuery<
        ApiType,
        (
          arg: U8aFixed | string | Uint8Array
        ) => Observable<
          Option<T3rnPrimitivesSideEffectInterfaceSideEffectInterface>
        >,
        [U8aFixed]
      > &
        QueryableStorageEntry<ApiType, [U8aFixed]>;
      /** The pre-validated composable xdns_records on-chain registry. */
      xdnsRegistry: AugmentedQuery<
        ApiType,
        (
          arg: U8aFixed | string | Uint8Array
        ) => Observable<Option<T3rnPrimitivesXdnsXdnsRecord>>,
        [U8aFixed]
      > &
        QueryableStorageEntry<ApiType, [U8aFixed]>;
      /** Generic query */
      [key: string]: QueryableStorageEntry<ApiType>;
    };
  } // AugmentedQueries
} // declare module
