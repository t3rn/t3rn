// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

import type { ApiTypes } from "@polkadot/api-base/types";
import type {
  Bytes,
  Compact,
  Option,
  Struct,
  U8aFixed,
  Vec,
  bool,
  u128,
  u32,
  u64,
} from "@polkadot/types-codec";
import type { AnyNumber, ITuple } from "@polkadot/types-codec/types";
import type {
  AccountId32,
  Call,
  H256,
  MultiAddress,
  Perbill,
} from "@polkadot/types/interfaces/runtime";
import type {
  FinalityGrandpaCommitU32,
  FinalityGrandpaCommitU64,
  PalletContractsRegistryRegistryContract,
  SpCoreVoid,
  SpFinalityGrandpaAppPublic,
  SpFinalityGrandpaEquivocationProof,
  SpRuntimeDigest,
  T3rnPrimitivesAbiGatewayABIConfig,
  T3rnPrimitivesAbiType,
  T3rnPrimitivesGatewayGenesisConfig,
  T3rnPrimitivesGatewaySysProps,
  T3rnPrimitivesGatewayType,
  T3rnPrimitivesGatewayVendor,
  T3rnPrimitivesSideEffect,
  T3rnPrimitivesSideEffectConfirmedSideEffect,
} from "@polkadot/types/lookup";

declare module "@polkadot/api-base/types/submittable" {
  export interface AugmentedSubmittables<ApiType extends ApiTypes> {
    balances: {
      /**
       * Exactly as `transfer`, except the origin must be root and the source
       * account may be specified.
       *
       * # <weight>
       *
       * - Same as transfer, but additional read and write because the source
       *   account is not assumed to be in the overlay.
       *
       * # </weight>
       */
      forceTransfer: AugmentedSubmittable<
        (
          source:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          dest:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          value: Compact<u128> | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [MultiAddress, MultiAddress, Compact<u128>]
      >;
      /**
       * Unreserve some balance from a user by force.
       *
       * Can only be called by ROOT.
       */
      forceUnreserve: AugmentedSubmittable<
        (
          who:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          amount: u128 | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [MultiAddress, u128]
      >;
      /**
       * Set the balances of a given account.
       *
       * This will alter `FreeBalance` and `ReservedBalance` in storage. it will
       * also alter the total issuance of the system (`TotalIssuance`)
       * appropriately. If the new free or reserved balance is below the
       * existential deposit, it will reset the account nonce
       * (`frame_system::AccountNonce`).
       *
       * The dispatch origin for this call is `root`.
       */
      setBalance: AugmentedSubmittable<
        (
          who:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          newFree: Compact<u128> | AnyNumber | Uint8Array,
          newReserved: Compact<u128> | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [MultiAddress, Compact<u128>, Compact<u128>]
      >;
      /**
       * Transfer some liquid free balance to another account.
       *
       * `transfer` will set the `FreeBalance` of the sender and receiver. If
       * the sender's account is below the existential deposit as a result of
       * the transfer, the account will be reaped.
       *
       * The dispatch origin for this call must be `Signed` by the transactor.
       *
       * # <weight>
       *
       * - Dependent on arguments but not critical, given proper implementations
       *   for input config types. See related functions below.
       * - It contains a limited number of reads and writes internally and no
       *   complex computation.
       *
       * Related functions:
       *
       * - `ensure_can_withdraw` is always called internally but has a bounded complexity.
       * - Transferring balances to accounts that did not exist before will cause
       *   `T::OnNewAccount::on_new_account` to be called.
       * - Removing enough funds from an account will trigger
       *   `T::DustRemoval::on_unbalanced`.
       * - `transfer_keep_alive` works the same way as `transfer`, but has an
       *   additional check that the transfer will not kill the origin account.
       *
       * - Origin account is already in memory, so no DB operations for them.
       *
       * # </weight>
       */
      transfer: AugmentedSubmittable<
        (
          dest:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          value: Compact<u128> | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [MultiAddress, Compact<u128>]
      >;
      /**
       * Transfer the entire transferable balance from the caller account.
       *
       * NOTE: This function only attempts to transfer _transferable_ balances.
       * This means that any locked, reserved, or existential deposits (when
       * `keep_alive` is `true`), will not be transferred by this function. To
       * ensure that this function results in a killed account, you might need
       * to prepare the account by removing any reference counters, storage
       * deposits, etc...
       *
       * The dispatch origin of this call must be Signed.
       *
       * - `dest`: The recipient of the transfer.
       * - `keep_alive`: A boolean to determine if the `transfer_all` operation
       *   should send all of the funds the account has, causing the sender
       *   account to be killed (false), or transfer everything except at least
       *   the existential deposit, which will guarantee to keep the sender
       *   account alive (true). # <weight>
       * - O(1). Just like transfer, but reading the user's transferable balance
       *   first. #</weight>
       */
      transferAll: AugmentedSubmittable<
        (
          dest:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          keepAlive: bool | boolean | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [MultiAddress, bool]
      >;
      /**
       * Same as the `transfer` call, but with a check that the transfer will
       * not kill the origin account.
       *
       * 99% of the time you want `transfer` instead.
       */
      transferKeepAlive: AugmentedSubmittable<
        (
          dest:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          value: Compact<u128> | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [MultiAddress, Compact<u128>]
      >;
      /** Generic tx */
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    circuit: {
      bondInsuranceDeposit: AugmentedSubmittable<
        (
          xtxId: H256 | string | Uint8Array,
          sideEffectId: H256 | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [H256, H256]
      >;
      /**
       * Blind version should only be used for testing - unsafe since skips
       * inclusion proof check.
       */
      confirmSideEffect: AugmentedSubmittable<
        (
          xtxId: H256 | string | Uint8Array,
          sideEffect:
            | T3rnPrimitivesSideEffect
            | {
                target?: any;
                prize?: any;
                orderedAt?: any;
                encodedAction?: any;
                encodedArgs?: any;
                signature?: any;
                enforceExecutioner?: any;
              }
            | string
            | Uint8Array,
          confirmation:
            | T3rnPrimitivesSideEffectConfirmedSideEffect
            | {
                err?: any;
                output?: any;
                encodedEffect?: any;
                inclusionProof?: any;
                executioner?: any;
                receivedAt?: any;
                cost?: any;
              }
            | string
            | Uint8Array,
          inclusionProof:
            | Option<Vec<Bytes>>
            | null
            | object
            | string
            | Uint8Array,
          blockHash: Option<Bytes> | null | object | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [
          H256,
          T3rnPrimitivesSideEffect,
          T3rnPrimitivesSideEffectConfirmedSideEffect,
          Option<Vec<Bytes>>,
          Option<Bytes>
        ]
      >;
      onExtrinsicTrigger: AugmentedSubmittable<
        (
          sideEffects:
            | Vec<T3rnPrimitivesSideEffect>
            | (
                | T3rnPrimitivesSideEffect
                | {
                    target?: any;
                    prize?: any;
                    orderedAt?: any;
                    encodedAction?: any;
                    encodedArgs?: any;
                    signature?: any;
                    enforceExecutioner?: any;
                  }
                | string
                | Uint8Array
              )[],
          fee: u128 | AnyNumber | Uint8Array,
          sequential: bool | boolean | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Vec<T3rnPrimitivesSideEffect>, u128, bool]
      >;
      /** Used by other pallets that want to create the exec order */
      onLocalTrigger: AugmentedSubmittable<
        () => SubmittableExtrinsic<ApiType>,
        []
      >;
      onRemoteGatewayTrigger: AugmentedSubmittable<
        () => SubmittableExtrinsic<ApiType>,
        []
      >;
      onXcmTrigger: AugmentedSubmittable<
        () => SubmittableExtrinsic<ApiType>,
        []
      >;
      /** Generic tx */
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    circuitPortal: {
      registerGateway: AugmentedSubmittable<
        (
          url: Bytes | string | Uint8Array,
          gatewayId: U8aFixed | string | Uint8Array,
          gatewayAbi:
            | T3rnPrimitivesAbiGatewayABIConfig
            | {
                blockNumberTypeSize?: any;
                hashSize?: any;
                hasher?: any;
                crypto?: any;
                addressLength?: any;
                valueTypeSize?: any;
                decimals?: any;
                structs?: any;
              }
            | string
            | Uint8Array,
          gatewayVendor:
            | T3rnPrimitivesGatewayVendor
            | "Substrate"
            | "Ethereum"
            | number
            | Uint8Array,
          gatewayType:
            | T3rnPrimitivesGatewayType
            | { ProgrammableInternal: any }
            | { ProgrammableExternal: any }
            | { TxOnly: any }
            | string
            | Uint8Array,
          gatewayGenesis:
            | T3rnPrimitivesGatewayGenesisConfig
            | {
                modulesEncoded?: any;
                extrinsicsVersion?: any;
                genesisHash?: any;
              }
            | string
            | Uint8Array,
          gatewaySysProps:
            | T3rnPrimitivesGatewaySysProps
            | { ss58Format?: any; tokenSymbol?: any; tokenDecimals?: any }
            | string
            | Uint8Array,
          firstHeader: Bytes | string | Uint8Array,
          authorities:
            | Option<Vec<AccountId32>>
            | null
            | object
            | string
            | Uint8Array,
          allowedSideEffects: Vec<U8aFixed>
        ) => SubmittableExtrinsic<ApiType>,
        [
          Bytes,
          U8aFixed,
          T3rnPrimitivesAbiGatewayABIConfig,
          T3rnPrimitivesGatewayVendor,
          T3rnPrimitivesGatewayType,
          T3rnPrimitivesGatewayGenesisConfig,
          T3rnPrimitivesGatewaySysProps,
          Bytes,
          Option<Vec<AccountId32>>,
          Vec<U8aFixed>
        ]
      >;
      updateGateway: AugmentedSubmittable<
        (
          gatewayId: U8aFixed | string | Uint8Array,
          url: Option<Bytes> | null | object | string | Uint8Array,
          gatewayAbi:
            | Option<T3rnPrimitivesAbiGatewayABIConfig>
            | null
            | object
            | string
            | Uint8Array,
          gatewaySysProps:
            | Option<T3rnPrimitivesGatewaySysProps>
            | null
            | object
            | string
            | Uint8Array,
          authorities:
            | Option<Vec<AccountId32>>
            | null
            | object
            | string
            | Uint8Array,
          allowedSideEffects:
            | Option<Vec<U8aFixed>>
            | null
            | object
            | string
            | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [
          U8aFixed,
          Option<Bytes>,
          Option<T3rnPrimitivesAbiGatewayABIConfig>,
          Option<T3rnPrimitivesGatewaySysProps>,
          Option<Vec<AccountId32>>,
          Option<Vec<U8aFixed>>
        ]
      >;
      /** Generic tx */
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    contractsRegistry: {
      /** Inserts a contract into the on-chain registry. Root only access. */
      addNewContract: AugmentedSubmittable<
        (
          requester: AccountId32 | string | Uint8Array,
          contract:
            | PalletContractsRegistryRegistryContract
            | {
                codeTxt?: any;
                bytes?: any;
                author?: any;
                authorFeesPerSingleUse?: any;
                abi?: any;
                actionDescriptions?: any;
                info?: any;
                meta?: any;
              }
            | string
            | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [AccountId32, PalletContractsRegistryRegistryContract]
      >;
      /** Removes a contract from the onchain registry. Root only access. */
      purge: AugmentedSubmittable<
        (
          requester: AccountId32 | string | Uint8Array,
          contractId: H256 | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [AccountId32, H256]
      >;
      /** Generic tx */
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    grandpa: {
      /**
       * Note that the current authority set of the GRANDPA finality gadget has
       * stalled. This will trigger a forced authority set change at the
       * beginning of the next session, to be enacted `delay` blocks after that.
       * The delay should be high enough to safely assume that the block
       * signalling the forced change will not be re-orged (e.g. 1000 blocks).
       * The GRANDPA voters will start the new authority set using the given
       * finalized block as base. Only callable by root.
       */
      noteStalled: AugmentedSubmittable<
        (
          delay: u32 | AnyNumber | Uint8Array,
          bestFinalizedBlockNumber: u32 | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [u32, u32]
      >;
      /**
       * Report voter equivocation/misbehavior. This method will verify the
       * equivocation proof and validate the given key ownership proof against
       * the extracted offender. If both are valid, the offence will be reported.
       */
      reportEquivocation: AugmentedSubmittable<
        (
          equivocationProof:
            | SpFinalityGrandpaEquivocationProof
            | { setId?: any; equivocation?: any }
            | string
            | Uint8Array,
          keyOwnerProof: SpCoreVoid | null
        ) => SubmittableExtrinsic<ApiType>,
        [SpFinalityGrandpaEquivocationProof, SpCoreVoid]
      >;
      /**
       * Report voter equivocation/misbehavior. This method will verify the
       * equivocation proof and validate the given key ownership proof against
       * the extracted offender. If both are valid, the offence will be reported.
       *
       * This extrinsic must be called unsigned and it is expected that only
       * block authors will call it (validated in `ValidateUnsigned`), as such
       * if the block author is defined it will be defined as the equivocation reporter.
       */
      reportEquivocationUnsigned: AugmentedSubmittable<
        (
          equivocationProof:
            | SpFinalityGrandpaEquivocationProof
            | { setId?: any; equivocation?: any }
            | string
            | Uint8Array,
          keyOwnerProof: SpCoreVoid | null
        ) => SubmittableExtrinsic<ApiType>,
        [SpFinalityGrandpaEquivocationProof, SpCoreVoid]
      >;
      /** Generic tx */
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    multiFinalityVerifierDefault: {
      /**
       * Bootstrap the bridge pallet with an initial header and authority set
       * from which to sync.
       *
       * The initial configuration provided does not need to be the genesis
       * header of the bridged chain, it can be any arbirary header. You can
       * also provide the next scheduled set change if it is already know.
       *
       * This function is only allowed to be called from a trusted origin and
       * writes to storage with practically no checks in terms of the validity
       * of the data. It is important that you ensure that valid data is being passed in.
       */
      initializeSingle: AugmentedSubmittable<
        (
          initData:
            | ({
                readonly header: {
                  readonly parentHash: H256;
                  readonly number: Compact<u32>;
                  readonly stateRoot: H256;
                  readonly extrinsicsRoot: H256;
                  readonly digest: SpRuntimeDigest;
                } & Struct;
                readonly authorityList: Vec<
                  ITuple<[SpFinalityGrandpaAppPublic, u64]>
                >;
                readonly setId: u64;
                readonly isHalted: bool;
              } & Struct)
            | { header?: any; authorityList?: any; setId?: any; isHalted?: any }
            | string
            | Uint8Array,
          gatewayId: U8aFixed | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [
          {
            readonly header: {
              readonly parentHash: H256;
              readonly number: Compact<u32>;
              readonly stateRoot: H256;
              readonly extrinsicsRoot: H256;
              readonly digest: SpRuntimeDigest;
            } & Struct;
            readonly authorityList: Vec<
              ITuple<[SpFinalityGrandpaAppPublic, u64]>
            >;
            readonly setId: u64;
            readonly isHalted: bool;
          } & Struct,
          U8aFixed
        ]
      >;
      /**
       * Halt or resume all pallet operations.
       *
       * May only be called either by root, or by `PalletOwner`.
       */
      setOperational: AugmentedSubmittable<
        (
          operational: bool | boolean | Uint8Array,
          gatewayId: U8aFixed | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [bool, U8aFixed]
      >;
      /**
       * Change `PalletOwner`.
       *
       * May only be called either by root, or by `PalletOwner`.
       */
      setOwner: AugmentedSubmittable<
        (
          newOwner: Option<AccountId32> | null | object | string | Uint8Array,
          gatewayId: U8aFixed | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Option<AccountId32>, U8aFixed]
      >;
      /**
       * Verify a target header is finalized according to the given finality proof.
       *
       * It will use the underlying storage pallet to fetch information about
       * the current authorities and best finalized header in order to verify
       * that the header is finalized.
       *
       * If successful in verification, it will write the target header to the
       * underlying storage pallet.
       */
      submitFinalityProof: AugmentedSubmittable<
        (
          finalityTarget:
            | ({
                readonly parentHash: H256;
                readonly number: Compact<u32>;
                readonly stateRoot: H256;
                readonly extrinsicsRoot: H256;
                readonly digest: SpRuntimeDigest;
              } & Struct)
            | {
                parentHash?: any;
                number?: any;
                stateRoot?: any;
                extrinsicsRoot?: any;
                digest?: any;
              }
            | string
            | Uint8Array,
          justification:
            | ({
                readonly round: u64;
                readonly commit: FinalityGrandpaCommitU32;
                readonly votesAncestries: Vec<
                  {
                    readonly parentHash: H256;
                    readonly number: Compact<u32>;
                    readonly stateRoot: H256;
                    readonly extrinsicsRoot: H256;
                    readonly digest: SpRuntimeDigest;
                  } & Struct
                >;
              } & Struct)
            | { round?: any; commit?: any; votesAncestries?: any }
            | string
            | Uint8Array,
          gatewayId: U8aFixed | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [
          {
            readonly parentHash: H256;
            readonly number: Compact<u32>;
            readonly stateRoot: H256;
            readonly extrinsicsRoot: H256;
            readonly digest: SpRuntimeDigest;
          } & Struct,
          {
            readonly round: u64;
            readonly commit: FinalityGrandpaCommitU32;
            readonly votesAncestries: Vec<
              {
                readonly parentHash: H256;
                readonly number: Compact<u32>;
                readonly stateRoot: H256;
                readonly extrinsicsRoot: H256;
                readonly digest: SpRuntimeDigest;
              } & Struct
            >;
          } & Struct,
          U8aFixed
        ]
      >;
      submitHeaderRange: AugmentedSubmittable<
        (
          gatewayId: U8aFixed | string | Uint8Array,
          headersReversed: Vec<
            {
              readonly parentHash: H256;
              readonly number: Compact<u32>;
              readonly stateRoot: H256;
              readonly extrinsicsRoot: H256;
              readonly digest: SpRuntimeDigest;
            } & Struct
          >,
          anchorHeaderHash: H256 | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [
          U8aFixed,
          Vec<
            {
              readonly parentHash: H256;
              readonly number: Compact<u32>;
              readonly stateRoot: H256;
              readonly extrinsicsRoot: H256;
              readonly digest: SpRuntimeDigest;
            } & Struct
          >,
          H256
        ]
      >;
      /** Generic tx */
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    multiFinalityVerifierEthereumLike: {
      /**
       * Bootstrap the bridge pallet with an initial header and authority set
       * from which to sync.
       *
       * The initial configuration provided does not need to be the genesis
       * header of the bridged chain, it can be any arbirary header. You can
       * also provide the next scheduled set change if it is already know.
       *
       * This function is only allowed to be called from a trusted origin and
       * writes to storage with practically no checks in terms of the validity
       * of the data. It is important that you ensure that valid data is being passed in.
       */
      initializeSingle: AugmentedSubmittable<
        (
          initData:
            | ({
                readonly header: {
                  readonly parentHash: H256;
                  readonly number: Compact<u64>;
                  readonly stateRoot: H256;
                  readonly extrinsicsRoot: H256;
                  readonly digest: SpRuntimeDigest;
                } & Struct;
                readonly authorityList: Vec<
                  ITuple<[SpFinalityGrandpaAppPublic, u64]>
                >;
                readonly setId: u64;
                readonly isHalted: bool;
              } & Struct)
            | { header?: any; authorityList?: any; setId?: any; isHalted?: any }
            | string
            | Uint8Array,
          gatewayId: U8aFixed | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [
          {
            readonly header: {
              readonly parentHash: H256;
              readonly number: Compact<u64>;
              readonly stateRoot: H256;
              readonly extrinsicsRoot: H256;
              readonly digest: SpRuntimeDigest;
            } & Struct;
            readonly authorityList: Vec<
              ITuple<[SpFinalityGrandpaAppPublic, u64]>
            >;
            readonly setId: u64;
            readonly isHalted: bool;
          } & Struct,
          U8aFixed
        ]
      >;
      /**
       * Halt or resume all pallet operations.
       *
       * May only be called either by root, or by `PalletOwner`.
       */
      setOperational: AugmentedSubmittable<
        (
          operational: bool | boolean | Uint8Array,
          gatewayId: U8aFixed | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [bool, U8aFixed]
      >;
      /**
       * Change `PalletOwner`.
       *
       * May only be called either by root, or by `PalletOwner`.
       */
      setOwner: AugmentedSubmittable<
        (
          newOwner: Option<AccountId32> | null | object | string | Uint8Array,
          gatewayId: U8aFixed | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Option<AccountId32>, U8aFixed]
      >;
      /**
       * Verify a target header is finalized according to the given finality proof.
       *
       * It will use the underlying storage pallet to fetch information about
       * the current authorities and best finalized header in order to verify
       * that the header is finalized.
       *
       * If successful in verification, it will write the target header to the
       * underlying storage pallet.
       */
      submitFinalityProof: AugmentedSubmittable<
        (
          finalityTarget:
            | ({
                readonly parentHash: H256;
                readonly number: Compact<u64>;
                readonly stateRoot: H256;
                readonly extrinsicsRoot: H256;
                readonly digest: SpRuntimeDigest;
              } & Struct)
            | {
                parentHash?: any;
                number?: any;
                stateRoot?: any;
                extrinsicsRoot?: any;
                digest?: any;
              }
            | string
            | Uint8Array,
          justification:
            | ({
                readonly round: u64;
                readonly commit: FinalityGrandpaCommitU64;
                readonly votesAncestries: Vec<
                  {
                    readonly parentHash: H256;
                    readonly number: Compact<u64>;
                    readonly stateRoot: H256;
                    readonly extrinsicsRoot: H256;
                    readonly digest: SpRuntimeDigest;
                  } & Struct
                >;
              } & Struct)
            | { round?: any; commit?: any; votesAncestries?: any }
            | string
            | Uint8Array,
          gatewayId: U8aFixed | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [
          {
            readonly parentHash: H256;
            readonly number: Compact<u64>;
            readonly stateRoot: H256;
            readonly extrinsicsRoot: H256;
            readonly digest: SpRuntimeDigest;
          } & Struct,
          {
            readonly round: u64;
            readonly commit: FinalityGrandpaCommitU64;
            readonly votesAncestries: Vec<
              {
                readonly parentHash: H256;
                readonly number: Compact<u64>;
                readonly stateRoot: H256;
                readonly extrinsicsRoot: H256;
                readonly digest: SpRuntimeDigest;
              } & Struct
            >;
          } & Struct,
          U8aFixed
        ]
      >;
      submitHeaderRange: AugmentedSubmittable<
        (
          gatewayId: U8aFixed | string | Uint8Array,
          headersReversed: Vec<
            {
              readonly parentHash: H256;
              readonly number: Compact<u64>;
              readonly stateRoot: H256;
              readonly extrinsicsRoot: H256;
              readonly digest: SpRuntimeDigest;
            } & Struct
          >,
          anchorHeaderHash: H256 | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [
          U8aFixed,
          Vec<
            {
              readonly parentHash: H256;
              readonly number: Compact<u64>;
              readonly stateRoot: H256;
              readonly extrinsicsRoot: H256;
              readonly digest: SpRuntimeDigest;
            } & Struct
          >,
          H256
        ]
      >;
      /** Generic tx */
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    multiFinalityVerifierGenericLike: {
      /**
       * Bootstrap the bridge pallet with an initial header and authority set
       * from which to sync.
       *
       * The initial configuration provided does not need to be the genesis
       * header of the bridged chain, it can be any arbirary header. You can
       * also provide the next scheduled set change if it is already know.
       *
       * This function is only allowed to be called from a trusted origin and
       * writes to storage with practically no checks in terms of the validity
       * of the data. It is important that you ensure that valid data is being passed in.
       */
      initializeSingle: AugmentedSubmittable<
        (
          initData:
            | ({
                readonly header: {
                  readonly parentHash: H256;
                  readonly number: Compact<u32>;
                  readonly stateRoot: H256;
                  readonly extrinsicsRoot: H256;
                  readonly digest: SpRuntimeDigest;
                } & Struct;
                readonly authorityList: Vec<
                  ITuple<[SpFinalityGrandpaAppPublic, u64]>
                >;
                readonly setId: u64;
                readonly isHalted: bool;
              } & Struct)
            | { header?: any; authorityList?: any; setId?: any; isHalted?: any }
            | string
            | Uint8Array,
          gatewayId: U8aFixed | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [
          {
            readonly header: {
              readonly parentHash: H256;
              readonly number: Compact<u32>;
              readonly stateRoot: H256;
              readonly extrinsicsRoot: H256;
              readonly digest: SpRuntimeDigest;
            } & Struct;
            readonly authorityList: Vec<
              ITuple<[SpFinalityGrandpaAppPublic, u64]>
            >;
            readonly setId: u64;
            readonly isHalted: bool;
          } & Struct,
          U8aFixed
        ]
      >;
      /**
       * Halt or resume all pallet operations.
       *
       * May only be called either by root, or by `PalletOwner`.
       */
      setOperational: AugmentedSubmittable<
        (
          operational: bool | boolean | Uint8Array,
          gatewayId: U8aFixed | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [bool, U8aFixed]
      >;
      /**
       * Change `PalletOwner`.
       *
       * May only be called either by root, or by `PalletOwner`.
       */
      setOwner: AugmentedSubmittable<
        (
          newOwner: Option<AccountId32> | null | object | string | Uint8Array,
          gatewayId: U8aFixed | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Option<AccountId32>, U8aFixed]
      >;
      /**
       * Verify a target header is finalized according to the given finality proof.
       *
       * It will use the underlying storage pallet to fetch information about
       * the current authorities and best finalized header in order to verify
       * that the header is finalized.
       *
       * If successful in verification, it will write the target header to the
       * underlying storage pallet.
       */
      submitFinalityProof: AugmentedSubmittable<
        (
          finalityTarget:
            | ({
                readonly parentHash: H256;
                readonly number: Compact<u32>;
                readonly stateRoot: H256;
                readonly extrinsicsRoot: H256;
                readonly digest: SpRuntimeDigest;
              } & Struct)
            | {
                parentHash?: any;
                number?: any;
                stateRoot?: any;
                extrinsicsRoot?: any;
                digest?: any;
              }
            | string
            | Uint8Array,
          justification:
            | ({
                readonly round: u64;
                readonly commit: FinalityGrandpaCommitU32;
                readonly votesAncestries: Vec<
                  {
                    readonly parentHash: H256;
                    readonly number: Compact<u32>;
                    readonly stateRoot: H256;
                    readonly extrinsicsRoot: H256;
                    readonly digest: SpRuntimeDigest;
                  } & Struct
                >;
              } & Struct)
            | { round?: any; commit?: any; votesAncestries?: any }
            | string
            | Uint8Array,
          gatewayId: U8aFixed | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [
          {
            readonly parentHash: H256;
            readonly number: Compact<u32>;
            readonly stateRoot: H256;
            readonly extrinsicsRoot: H256;
            readonly digest: SpRuntimeDigest;
          } & Struct,
          {
            readonly round: u64;
            readonly commit: FinalityGrandpaCommitU32;
            readonly votesAncestries: Vec<
              {
                readonly parentHash: H256;
                readonly number: Compact<u32>;
                readonly stateRoot: H256;
                readonly extrinsicsRoot: H256;
                readonly digest: SpRuntimeDigest;
              } & Struct
            >;
          } & Struct,
          U8aFixed
        ]
      >;
      submitHeaderRange: AugmentedSubmittable<
        (
          gatewayId: U8aFixed | string | Uint8Array,
          headersReversed: Vec<
            {
              readonly parentHash: H256;
              readonly number: Compact<u32>;
              readonly stateRoot: H256;
              readonly extrinsicsRoot: H256;
              readonly digest: SpRuntimeDigest;
            } & Struct
          >,
          anchorHeaderHash: H256 | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [
          U8aFixed,
          Vec<
            {
              readonly parentHash: H256;
              readonly number: Compact<u32>;
              readonly stateRoot: H256;
              readonly extrinsicsRoot: H256;
              readonly digest: SpRuntimeDigest;
            } & Struct
          >,
          H256
        ]
      >;
      /** Generic tx */
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    multiFinalityVerifierPolkadotLike: {
      /**
       * Bootstrap the bridge pallet with an initial header and authority set
       * from which to sync.
       *
       * The initial configuration provided does not need to be the genesis
       * header of the bridged chain, it can be any arbirary header. You can
       * also provide the next scheduled set change if it is already know.
       *
       * This function is only allowed to be called from a trusted origin and
       * writes to storage with practically no checks in terms of the validity
       * of the data. It is important that you ensure that valid data is being passed in.
       */
      initializeSingle: AugmentedSubmittable<
        (
          initData:
            | ({
                readonly header: {
                  readonly parentHash: H256;
                  readonly number: Compact<u32>;
                  readonly stateRoot: H256;
                  readonly extrinsicsRoot: H256;
                  readonly digest: SpRuntimeDigest;
                } & Struct;
                readonly authorityList: Vec<
                  ITuple<[SpFinalityGrandpaAppPublic, u64]>
                >;
                readonly setId: u64;
                readonly isHalted: bool;
              } & Struct)
            | { header?: any; authorityList?: any; setId?: any; isHalted?: any }
            | string
            | Uint8Array,
          gatewayId: U8aFixed | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [
          {
            readonly header: {
              readonly parentHash: H256;
              readonly number: Compact<u32>;
              readonly stateRoot: H256;
              readonly extrinsicsRoot: H256;
              readonly digest: SpRuntimeDigest;
            } & Struct;
            readonly authorityList: Vec<
              ITuple<[SpFinalityGrandpaAppPublic, u64]>
            >;
            readonly setId: u64;
            readonly isHalted: bool;
          } & Struct,
          U8aFixed
        ]
      >;
      /**
       * Halt or resume all pallet operations.
       *
       * May only be called either by root, or by `PalletOwner`.
       */
      setOperational: AugmentedSubmittable<
        (
          operational: bool | boolean | Uint8Array,
          gatewayId: U8aFixed | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [bool, U8aFixed]
      >;
      /**
       * Change `PalletOwner`.
       *
       * May only be called either by root, or by `PalletOwner`.
       */
      setOwner: AugmentedSubmittable<
        (
          newOwner: Option<AccountId32> | null | object | string | Uint8Array,
          gatewayId: U8aFixed | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Option<AccountId32>, U8aFixed]
      >;
      /**
       * Verify a target header is finalized according to the given finality proof.
       *
       * It will use the underlying storage pallet to fetch information about
       * the current authorities and best finalized header in order to verify
       * that the header is finalized.
       *
       * If successful in verification, it will write the target header to the
       * underlying storage pallet.
       */
      submitFinalityProof: AugmentedSubmittable<
        (
          finalityTarget:
            | ({
                readonly parentHash: H256;
                readonly number: Compact<u32>;
                readonly stateRoot: H256;
                readonly extrinsicsRoot: H256;
                readonly digest: SpRuntimeDigest;
              } & Struct)
            | {
                parentHash?: any;
                number?: any;
                stateRoot?: any;
                extrinsicsRoot?: any;
                digest?: any;
              }
            | string
            | Uint8Array,
          justification:
            | ({
                readonly round: u64;
                readonly commit: FinalityGrandpaCommitU32;
                readonly votesAncestries: Vec<
                  {
                    readonly parentHash: H256;
                    readonly number: Compact<u32>;
                    readonly stateRoot: H256;
                    readonly extrinsicsRoot: H256;
                    readonly digest: SpRuntimeDigest;
                  } & Struct
                >;
              } & Struct)
            | { round?: any; commit?: any; votesAncestries?: any }
            | string
            | Uint8Array,
          gatewayId: U8aFixed | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [
          {
            readonly parentHash: H256;
            readonly number: Compact<u32>;
            readonly stateRoot: H256;
            readonly extrinsicsRoot: H256;
            readonly digest: SpRuntimeDigest;
          } & Struct,
          {
            readonly round: u64;
            readonly commit: FinalityGrandpaCommitU32;
            readonly votesAncestries: Vec<
              {
                readonly parentHash: H256;
                readonly number: Compact<u32>;
                readonly stateRoot: H256;
                readonly extrinsicsRoot: H256;
                readonly digest: SpRuntimeDigest;
              } & Struct
            >;
          } & Struct,
          U8aFixed
        ]
      >;
      submitHeaderRange: AugmentedSubmittable<
        (
          gatewayId: U8aFixed | string | Uint8Array,
          headersReversed: Vec<
            {
              readonly parentHash: H256;
              readonly number: Compact<u32>;
              readonly stateRoot: H256;
              readonly extrinsicsRoot: H256;
              readonly digest: SpRuntimeDigest;
            } & Struct
          >,
          anchorHeaderHash: H256 | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [
          U8aFixed,
          Vec<
            {
              readonly parentHash: H256;
              readonly number: Compact<u32>;
              readonly stateRoot: H256;
              readonly extrinsicsRoot: H256;
              readonly digest: SpRuntimeDigest;
            } & Struct
          >,
          H256
        ]
      >;
      /** Generic tx */
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    multiFinalityVerifierSubstrateLike: {
      /**
       * Bootstrap the bridge pallet with an initial header and authority set
       * from which to sync.
       *
       * The initial configuration provided does not need to be the genesis
       * header of the bridged chain, it can be any arbirary header. You can
       * also provide the next scheduled set change if it is already know.
       *
       * This function is only allowed to be called from a trusted origin and
       * writes to storage with practically no checks in terms of the validity
       * of the data. It is important that you ensure that valid data is being passed in.
       */
      initializeSingle: AugmentedSubmittable<
        (
          initData:
            | ({
                readonly header: {
                  readonly parentHash: H256;
                  readonly number: Compact<u32>;
                  readonly stateRoot: H256;
                  readonly extrinsicsRoot: H256;
                  readonly digest: SpRuntimeDigest;
                } & Struct;
                readonly authorityList: Vec<
                  ITuple<[SpFinalityGrandpaAppPublic, u64]>
                >;
                readonly setId: u64;
                readonly isHalted: bool;
              } & Struct)
            | { header?: any; authorityList?: any; setId?: any; isHalted?: any }
            | string
            | Uint8Array,
          gatewayId: U8aFixed | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [
          {
            readonly header: {
              readonly parentHash: H256;
              readonly number: Compact<u32>;
              readonly stateRoot: H256;
              readonly extrinsicsRoot: H256;
              readonly digest: SpRuntimeDigest;
            } & Struct;
            readonly authorityList: Vec<
              ITuple<[SpFinalityGrandpaAppPublic, u64]>
            >;
            readonly setId: u64;
            readonly isHalted: bool;
          } & Struct,
          U8aFixed
        ]
      >;
      /**
       * Halt or resume all pallet operations.
       *
       * May only be called either by root, or by `PalletOwner`.
       */
      setOperational: AugmentedSubmittable<
        (
          operational: bool | boolean | Uint8Array,
          gatewayId: U8aFixed | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [bool, U8aFixed]
      >;
      /**
       * Change `PalletOwner`.
       *
       * May only be called either by root, or by `PalletOwner`.
       */
      setOwner: AugmentedSubmittable<
        (
          newOwner: Option<AccountId32> | null | object | string | Uint8Array,
          gatewayId: U8aFixed | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Option<AccountId32>, U8aFixed]
      >;
      /**
       * Verify a target header is finalized according to the given finality proof.
       *
       * It will use the underlying storage pallet to fetch information about
       * the current authorities and best finalized header in order to verify
       * that the header is finalized.
       *
       * If successful in verification, it will write the target header to the
       * underlying storage pallet.
       */
      submitFinalityProof: AugmentedSubmittable<
        (
          finalityTarget:
            | ({
                readonly parentHash: H256;
                readonly number: Compact<u32>;
                readonly stateRoot: H256;
                readonly extrinsicsRoot: H256;
                readonly digest: SpRuntimeDigest;
              } & Struct)
            | {
                parentHash?: any;
                number?: any;
                stateRoot?: any;
                extrinsicsRoot?: any;
                digest?: any;
              }
            | string
            | Uint8Array,
          justification:
            | ({
                readonly round: u64;
                readonly commit: FinalityGrandpaCommitU32;
                readonly votesAncestries: Vec<
                  {
                    readonly parentHash: H256;
                    readonly number: Compact<u32>;
                    readonly stateRoot: H256;
                    readonly extrinsicsRoot: H256;
                    readonly digest: SpRuntimeDigest;
                  } & Struct
                >;
              } & Struct)
            | { round?: any; commit?: any; votesAncestries?: any }
            | string
            | Uint8Array,
          gatewayId: U8aFixed | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [
          {
            readonly parentHash: H256;
            readonly number: Compact<u32>;
            readonly stateRoot: H256;
            readonly extrinsicsRoot: H256;
            readonly digest: SpRuntimeDigest;
          } & Struct,
          {
            readonly round: u64;
            readonly commit: FinalityGrandpaCommitU32;
            readonly votesAncestries: Vec<
              {
                readonly parentHash: H256;
                readonly number: Compact<u32>;
                readonly stateRoot: H256;
                readonly extrinsicsRoot: H256;
                readonly digest: SpRuntimeDigest;
              } & Struct
            >;
          } & Struct,
          U8aFixed
        ]
      >;
      submitHeaderRange: AugmentedSubmittable<
        (
          gatewayId: U8aFixed | string | Uint8Array,
          headersReversed: Vec<
            {
              readonly parentHash: H256;
              readonly number: Compact<u32>;
              readonly stateRoot: H256;
              readonly extrinsicsRoot: H256;
              readonly digest: SpRuntimeDigest;
            } & Struct
          >,
          anchorHeaderHash: H256 | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [
          U8aFixed,
          Vec<
            {
              readonly parentHash: H256;
              readonly number: Compact<u32>;
              readonly stateRoot: H256;
              readonly extrinsicsRoot: H256;
              readonly digest: SpRuntimeDigest;
            } & Struct
          >,
          H256
        ]
      >;
      /** Generic tx */
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    sudo: {
      /**
       * Authenticates the current sudo key and sets the given AccountId (`new`)
       * as the new sudo key.
       *
       * The dispatch origin for this call must be _Signed_.
       *
       * # <weight>
       *
       * - O(1).
       * - Limited storage reads.
       * - One DB change.
       *
       * # </weight>
       */
      setKey: AugmentedSubmittable<
        (
          updated:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [MultiAddress]
      >;
      /**
       * Authenticates the sudo key and dispatches a function call with `Root` origin.
       *
       * The dispatch origin for this call must be _Signed_.
       *
       * # <weight>
       *
       * - O(1).
       * - Limited storage reads.
       * - One DB write (event).
       * - Weight of derivative `call` execution + 10,000.
       *
       * # </weight>
       */
      sudo: AugmentedSubmittable<
        (
          call: Call | { callIndex?: any; args?: any } | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Call]
      >;
      /**
       * Authenticates the sudo key and dispatches a function call with `Signed`
       * origin from a given account.
       *
       * The dispatch origin for this call must be _Signed_.
       *
       * # <weight>
       *
       * - O(1).
       * - Limited storage reads.
       * - One DB write (event).
       * - Weight of derivative `call` execution + 10,000.
       *
       * # </weight>
       */
      sudoAs: AugmentedSubmittable<
        (
          who:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          call: Call | { callIndex?: any; args?: any } | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [MultiAddress, Call]
      >;
      /**
       * Authenticates the sudo key and dispatches a function call with `Root`
       * origin. This function does not check the weight of the call, and
       * instead allows the Sudo user to specify the weight of the call.
       *
       * The dispatch origin for this call must be _Signed_.
       *
       * # <weight>
       *
       * - O(1).
       * - The weight of this call is defined by the caller.
       *
       * # </weight>
       */
      sudoUncheckedWeight: AugmentedSubmittable<
        (
          call: Call | { callIndex?: any; args?: any } | string | Uint8Array,
          weight: u64 | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Call, u64]
      >;
      /** Generic tx */
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    system: {
      /** A dispatch that will fill the block weight up to the given ratio. */
      fillBlock: AugmentedSubmittable<
        (
          ratio: Perbill | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Perbill]
      >;
      /**
       * Kill all storage items with a key that starts with the given prefix.
       *
       * **NOTE:** We rely on the Root origin to provide us the number of
       * subkeys under the prefix we are removing to accurately calculate the
       * weight of this function.
       */
      killPrefix: AugmentedSubmittable<
        (
          prefix: Bytes | string | Uint8Array,
          subkeys: u32 | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Bytes, u32]
      >;
      /** Kill some items from storage. */
      killStorage: AugmentedSubmittable<
        (
          keys: Vec<Bytes> | (Bytes | string | Uint8Array)[]
        ) => SubmittableExtrinsic<ApiType>,
        [Vec<Bytes>]
      >;
      /**
       * Make some on-chain remark.
       *
       * # <weight>
       *
       * - `O(1)`
       *
       * # </weight>
       */
      remark: AugmentedSubmittable<
        (remark: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
        [Bytes]
      >;
      /** Make some on-chain remark and emit event. */
      remarkWithEvent: AugmentedSubmittable<
        (remark: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
        [Bytes]
      >;
      /**
       * Set the new runtime code.
       *
       * # <weight>
       *
       * - `O(C + S)` where `C` length of `code` and `S` complexity of `can_set_code`
       * - 1 call to `can_set_code`: `O(S)` (calls `sp_io::misc::runtime_version`
       *   which is expensive).
       * - 1 storage write (codec `O(C)`).
       * - 1 digest item.
       * - 1 event. The weight of this function is dependent on the runtime, but
       *   generally this is very expensive. We will treat this as a full block.
       *
       * # </weight>
       */
      setCode: AugmentedSubmittable<
        (code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
        [Bytes]
      >;
      /**
       * Set the new runtime code without doing any checks of the given `code`.
       *
       * # <weight>
       *
       * - `O(C)` where `C` length of `code`
       * - 1 storage write (codec `O(C)`).
       * - 1 digest item.
       * - 1 event. The weight of this function is dependent on the runtime. We
       *   will treat this as a full block. # </weight>
       */
      setCodeWithoutChecks: AugmentedSubmittable<
        (code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
        [Bytes]
      >;
      /** Set the number of pages in the WebAssembly environment's heap. */
      setHeapPages: AugmentedSubmittable<
        (pages: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
        [u64]
      >;
      /** Set some items of storage. */
      setStorage: AugmentedSubmittable<
        (
          items:
            | Vec<ITuple<[Bytes, Bytes]>>
            | [Bytes | string | Uint8Array, Bytes | string | Uint8Array][]
        ) => SubmittableExtrinsic<ApiType>,
        [Vec<ITuple<[Bytes, Bytes]>>]
      >;
      /** Generic tx */
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    timestamp: {
      /**
       * Set the current time.
       *
       * This call should be invoked exactly once per block. It will panic at
       * the finalization phase, if this call hasn't been invoked by that time.
       *
       * The timestamp should be greater than the previous one by the amount
       * specified by `MinimumPeriod`.
       *
       * The dispatch origin for this call must be `Inherent`.
       *
       * # <weight>
       *
       * - `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)
       * - 1 storage read and 1 storage mutation (codec `O(1)`). (because of
       *   `DidUpdate::take` in `on_finalize`)
       * - 1 event handler `on_timestamp_set`. Must be `O(1)`.
       *
       * # </weight>
       */
      set: AugmentedSubmittable<
        (
          now: Compact<u64> | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Compact<u64>]
      >;
      /** Generic tx */
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    xdns: {
      /** Inserts a xdns_record into the on-chain registry. Root only access. */
      addNewXdnsRecord: AugmentedSubmittable<
        (
          url: Bytes | string | Uint8Array,
          gatewayId: U8aFixed | string | Uint8Array,
          gatewayAbi:
            | T3rnPrimitivesAbiGatewayABIConfig
            | {
                blockNumberTypeSize?: any;
                hashSize?: any;
                hasher?: any;
                crypto?: any;
                addressLength?: any;
                valueTypeSize?: any;
                decimals?: any;
                structs?: any;
              }
            | string
            | Uint8Array,
          gatewayVendor:
            | T3rnPrimitivesGatewayVendor
            | "Substrate"
            | "Ethereum"
            | number
            | Uint8Array,
          gatewayType:
            | T3rnPrimitivesGatewayType
            | { ProgrammableInternal: any }
            | { ProgrammableExternal: any }
            | { TxOnly: any }
            | string
            | Uint8Array,
          gatewayGenesis:
            | T3rnPrimitivesGatewayGenesisConfig
            | {
                modulesEncoded?: any;
                extrinsicsVersion?: any;
                genesisHash?: any;
              }
            | string
            | Uint8Array,
          gatewaySysProps:
            | T3rnPrimitivesGatewaySysProps
            | { ss58Format?: any; tokenSymbol?: any; tokenDecimals?: any }
            | string
            | Uint8Array,
          allowedSideEffects: Vec<U8aFixed>
        ) => SubmittableExtrinsic<ApiType>,
        [
          Bytes,
          U8aFixed,
          T3rnPrimitivesAbiGatewayABIConfig,
          T3rnPrimitivesGatewayVendor,
          T3rnPrimitivesGatewayType,
          T3rnPrimitivesGatewayGenesisConfig,
          T3rnPrimitivesGatewaySysProps,
          Vec<U8aFixed>
        ]
      >;
      addSideEffect: AugmentedSubmittable<
        (
          id: U8aFixed | string | Uint8Array,
          name: Bytes | string | Uint8Array,
          argumentAbi:
            | Vec<T3rnPrimitivesAbiType>
            | (
                | T3rnPrimitivesAbiType
                | { Address: any }
                | { DynamicAddress: any }
                | { Bool: any }
                | { Int: any }
                | { Uint: any }
                | { Bytes: any }
                | { DynamicBytes: any }
                | { String: any }
                | { Enum: any }
                | { Struct: any }
                | { Mapping: any }
                | { Contract: any }
                | { Ref: any }
                | { Option: any }
                | { OptionalInsurance: any }
                | { OptionalReward: any }
                | { StorageRef: any }
                | { Value: any }
                | { Slice: any }
                | { Hasher: any }
                | { Crypto: any }
                | string
                | Uint8Array
              )[],
          argumentToStateMapper: Vec<Bytes> | (Bytes | string | Uint8Array)[],
          confirmEvents: Vec<Bytes> | (Bytes | string | Uint8Array)[],
          escrowedEvents: Vec<Bytes> | (Bytes | string | Uint8Array)[],
          commitEvents: Vec<Bytes> | (Bytes | string | Uint8Array)[],
          revertEvents: Vec<Bytes> | (Bytes | string | Uint8Array)[]
        ) => SubmittableExtrinsic<ApiType>,
        [
          U8aFixed,
          Bytes,
          Vec<T3rnPrimitivesAbiType>,
          Vec<Bytes>,
          Vec<Bytes>,
          Vec<Bytes>,
          Vec<Bytes>,
          Vec<Bytes>
        ]
      >;
      /** Removes a xdns_record from the onchain registry. Root only access. */
      purgeXdnsRecord: AugmentedSubmittable<
        (
          requester: AccountId32 | string | Uint8Array,
          xdnsRecordId: U8aFixed | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [AccountId32, U8aFixed]
      >;
      /**
       * Updates the last_finalized field for an xdns_record from the onchain
       * registry. Root only access.
       */
      updateTtl: AugmentedSubmittable<
        (
          gatewayId: U8aFixed | string | Uint8Array,
          lastFinalized: u64 | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [U8aFixed, u64]
      >;
      /** Generic tx */
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
  } // AugmentedSubmittables
} // declare module
