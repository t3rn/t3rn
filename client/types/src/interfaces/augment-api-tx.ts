// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import "@polkadot/api-base/types/submittable";

import type {
  ApiTypes,
  AugmentedSubmittable,
  SubmittableExtrinsic,
  SubmittableExtrinsicFunction,
} from "@polkadot/api-base/types";
import type {
  Bytes,
  Compact,
  Option,
  Struct,
  U256,
  U8aFixed,
  Vec,
  bool,
  u128,
  u16,
  u32,
  u64,
  u8,
} from "@polkadot/types-codec";
import type { AnyNumber, IMethod, ITuple } from "@polkadot/types-codec/types";
import type {
  AccountId32,
  Call,
  H160,
  H256,
  MultiAddress,
  Perbill,
} from "@polkadot/types/interfaces/runtime";
import type {
  CircuitStandaloneRuntimeOriginCaller,
  PalletAssetsDestroyWitness,
  PalletXbiPortalXbiFormat,
  PalletXbiPortalXbiFormatXbiCheckIn,
  SpCoreVoid,
  SpFinalityGrandpaEquivocationProof,
  T3rnPrimitivesAccountManagerOutcome,
  T3rnPrimitivesClaimableBenefitSource,
  T3rnPrimitivesClaimableCircuitRole,
  T3rnPrimitivesContractsRegistryRegistryContract,
  T3rnPrimitivesGatewayGenesisConfig,
  T3rnPrimitivesGatewaySysProps,
  T3rnPrimitivesGatewayType,
  T3rnPrimitivesGatewayVendor,
  T3rnPrimitivesMonetaryBeneficiaryRole,
  T3rnPrimitivesMonetaryInflationAllocation,
  T3rnTypesAbiGatewayABIConfig,
  T3rnTypesAbiType,
  T3rnTypesSideEffect,
  T3rnTypesSideEffectConfirmedSideEffect,
  XcmV2Xcm,
} from "@polkadot/types/lookup";

export type __AugmentedSubmittable = AugmentedSubmittable<() => unknown>;
export type __SubmittableExtrinsic<ApiType extends ApiTypes> =
  SubmittableExtrinsic<ApiType>;
export type __SubmittableExtrinsicFunction<ApiType extends ApiTypes> =
  SubmittableExtrinsicFunction<ApiType>;

declare module "@polkadot/api-base/types/submittable" {
  interface AugmentedSubmittables<ApiType extends ApiTypes> {
    accountManager: {
      deposit: AugmentedSubmittable<
        (
          chargeId: H256 | string | Uint8Array,
          payee: AccountId32 | string | Uint8Array,
          chargeFee: u128 | AnyNumber | Uint8Array,
          offeredReward: u128 | AnyNumber | Uint8Array,
          source:
            | T3rnPrimitivesClaimableBenefitSource
            | "TrafficFees"
            | "TrafficRewards"
            | "BootstrapPool"
            | "Inflation"
            | number
            | Uint8Array,
          role:
            | T3rnPrimitivesClaimableCircuitRole
            | "Ambassador"
            | "Executor"
            | "Attester"
            | "Staker"
            | "Collator"
            | "ContractAuthor"
            | "Relayer"
            | "Requester"
            | "Local"
            | number
            | Uint8Array,
          maybeRecipient:
            | Option<AccountId32>
            | null
            | Uint8Array
            | AccountId32
            | string
        ) => SubmittableExtrinsic<ApiType>,
        [
          H256,
          AccountId32,
          u128,
          u128,
          T3rnPrimitivesClaimableBenefitSource,
          T3rnPrimitivesClaimableCircuitRole,
          Option<AccountId32>
        ]
      >;
      finalize: AugmentedSubmittable<
        (
          chargeId: H256 | string | Uint8Array,
          outcome:
            | T3rnPrimitivesAccountManagerOutcome
            | "UnexpectedFailure"
            | "Revert"
            | "Commit"
            | number
            | Uint8Array,
          maybeRecipient:
            | Option<AccountId32>
            | null
            | Uint8Array
            | AccountId32
            | string,
          maybeActualFees: Option<u128> | null | Uint8Array | u128 | AnyNumber
        ) => SubmittableExtrinsic<ApiType>,
        [
          H256,
          T3rnPrimitivesAccountManagerOutcome,
          Option<AccountId32>,
          Option<u128>
        ]
      >;
      /** Generic tx */
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    assets: {
      /**
       * Approve an amount of asset for transfer by a delegated third-party account.
       *
       * Origin must be Signed.
       *
       * Ensures that `ApprovalDeposit` worth of `Currency` is reserved from
       * signing account for the purpose of holding the approval. If some
       * non-zero amount of assets is already approved from signing account to
       * `delegate`, then it is topped up or unreserved to meet the right value.
       *
       * NOTE: The signing account does not need to own `amount` of assets at
       * the point of making this call.
       *
       * - `id`: The identifier of the asset.
       * - `delegate`: The account to delegate permission to transfer asset.
       * - `amount`: The amount of asset that may be transferred by `delegate`. If
       *   there is already an approval in place, then this acts additively.
       *
       * Emits `ApprovedTransfer` on success.
       *
       * Weight: `O(1)`
       */
      approveTransfer: AugmentedSubmittable<
        (
          id: Compact<u32> | AnyNumber | Uint8Array,
          delegate:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          amount: Compact<u128> | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Compact<u32>, MultiAddress, Compact<u128>]
      >;
      /**
       * Reduce the balance of `who` by as much as possible up to `amount` assets of `id`.
       *
       * Origin must be Signed and the sender should be the Manager of the asset `id`.
       *
       * Bails with `NoAccount` if the `who` is already dead.
       *
       * - `id`: The identifier of the asset to have some amount burned.
       * - `who`: The account to be debited from.
       * - `amount`: The maximum amount by which `who`'s balance should be reduced.
       *
       * Emits `Burned` with the actual amount burned. If this takes the balance
       * to below the minimum for the asset, then the amount burned is increased
       * to take it to zero.
       *
       * Weight: `O(1)` Modes: Post-existence of `who`; Pre & post Zombie-status of `who`.
       */
      burn: AugmentedSubmittable<
        (
          id: Compact<u32> | AnyNumber | Uint8Array,
          who:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          amount: Compact<u128> | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Compact<u32>, MultiAddress, Compact<u128>]
      >;
      /**
       * Cancel all of some asset approved for delegated transfer by a
       * third-party account.
       *
       * Origin must be Signed and there must be an approval in place between
       * signer and `delegate`.
       *
       * Unreserves any deposit previously reserved by `approve_transfer` for
       * the approval.
       *
       * - `id`: The identifier of the asset.
       * - `delegate`: The account delegated permission to transfer asset.
       *
       * Emits `ApprovalCancelled` on success.
       *
       * Weight: `O(1)`
       */
      cancelApproval: AugmentedSubmittable<
        (
          id: Compact<u32> | AnyNumber | Uint8Array,
          delegate:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Compact<u32>, MultiAddress]
      >;
      /**
       * Clear the metadata for an asset.
       *
       * Origin must be Signed and the sender should be the Owner of the asset `id`.
       *
       * Any deposit is freed for the asset owner.
       *
       * - `id`: The identifier of the asset to clear.
       *
       * Emits `MetadataCleared`.
       *
       * Weight: `O(1)`
       */
      clearMetadata: AugmentedSubmittable<
        (
          id: Compact<u32> | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Compact<u32>]
      >;
      /**
       * Issue a new class of fungible assets from a public origin.
       *
       * This new asset class has no assets initially and its owner is the origin.
       *
       * The origin must be Signed and the sender must have sufficient funds free.
       *
       * Funds of sender are reserved by `AssetDeposit`.
       *
       * Parameters:
       *
       * - `id`: The identifier of the new asset. This must not be currently in
       *   use to identify an existing asset.
       * - `admin`: The admin of this class of assets. The admin is the initial
       *   address of each member of the asset class's admin team.
       * - `min_balance`: The minimum balance of this new asset that any single
       *   account must have. If an account's balance is reduced below this,
       *   then it collapses to zero.
       *
       * Emits `Created` event when successful.
       *
       * Weight: `O(1)`
       */
      create: AugmentedSubmittable<
        (
          id: Compact<u32> | AnyNumber | Uint8Array,
          admin:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          minBalance: u128 | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Compact<u32>, MultiAddress, u128]
      >;
      /**
       * Destroy a class of fungible assets.
       *
       * The origin must conform to `ForceOrigin` or must be Signed and the
       * sender must be the owner of the asset `id`.
       *
       * - `id`: The identifier of the asset to be destroyed. This must identify
       *   an existing asset.
       *
       * Emits `Destroyed` event when successful.
       *
       * NOTE: It can be helpful to first freeze an asset before destroying it
       * so that you can provide accurate witness information and prevent users
       * from manipulating state in a way that can make it harder to destroy.
       *
       * Weight: `O(c + p + a)` where:
       *
       * - `c = (witness.accounts - witness.sufficients)`
       * - `s = witness.sufficients`
       * - `a = witness.approvals`
       */
      destroy: AugmentedSubmittable<
        (
          id: Compact<u32> | AnyNumber | Uint8Array,
          witness:
            | PalletAssetsDestroyWitness
            | { accounts?: any; sufficients?: any; approvals?: any }
            | string
            | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Compact<u32>, PalletAssetsDestroyWitness]
      >;
      /**
       * Alter the attributes of a given asset.
       *
       * Origin must be `ForceOrigin`.
       *
       * - `id`: The identifier of the asset.
       * - `owner`: The new Owner of this asset.
       * - `issuer`: The new Issuer of this asset.
       * - `admin`: The new Admin of this asset.
       * - `freezer`: The new Freezer of this asset.
       * - `min_balance`: The minimum balance of this new asset that any single
       *   account must have. If an account's balance is reduced below this,
       *   then it collapses to zero.
       * - `is_sufficient`: Whether a non-zero balance of this asset is deposit of
       *   sufficient value to account for the state bloat associated with its
       *   balance storage. If set to `true`, then non-zero balances may be
       *   stored without a `consumer` reference (and thus an ED in the Balances
       *   pallet or whatever else is used to control user-account state growth).
       * - `is_frozen`: Whether this asset class is frozen except for
       *   permissioned/admin instructions.
       *
       * Emits `AssetStatusChanged` with the identity of the asset.
       *
       * Weight: `O(1)`
       */
      forceAssetStatus: AugmentedSubmittable<
        (
          id: Compact<u32> | AnyNumber | Uint8Array,
          owner:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          issuer:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          admin:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          freezer:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          minBalance: Compact<u128> | AnyNumber | Uint8Array,
          isSufficient: bool | boolean | Uint8Array,
          isFrozen: bool | boolean | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [
          Compact<u32>,
          MultiAddress,
          MultiAddress,
          MultiAddress,
          MultiAddress,
          Compact<u128>,
          bool,
          bool
        ]
      >;
      /**
       * Cancel all of some asset approved for delegated transfer by a
       * third-party account.
       *
       * Origin must be either ForceOrigin or Signed origin with the signer
       * being the Admin account of the asset `id`.
       *
       * Unreserves any deposit previously reserved by `approve_transfer` for
       * the approval.
       *
       * - `id`: The identifier of the asset.
       * - `delegate`: The account delegated permission to transfer asset.
       *
       * Emits `ApprovalCancelled` on success.
       *
       * Weight: `O(1)`
       */
      forceCancelApproval: AugmentedSubmittable<
        (
          id: Compact<u32> | AnyNumber | Uint8Array,
          owner:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          delegate:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Compact<u32>, MultiAddress, MultiAddress]
      >;
      /**
       * Clear the metadata for an asset.
       *
       * Origin must be ForceOrigin.
       *
       * Any deposit is returned.
       *
       * - `id`: The identifier of the asset to clear.
       *
       * Emits `MetadataCleared`.
       *
       * Weight: `O(1)`
       */
      forceClearMetadata: AugmentedSubmittable<
        (
          id: Compact<u32> | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Compact<u32>]
      >;
      /**
       * Issue a new class of fungible assets from a privileged origin.
       *
       * This new asset class has no assets initially.
       *
       * The origin must conform to `ForceOrigin`.
       *
       * Unlike `create`, no funds are reserved.
       *
       * - `id`: The identifier of the new asset. This must not be currently in
       *   use to identify an existing asset.
       * - `owner`: The owner of this class of assets. The owner has full
       *   superuser permissions over this asset, but may later change and
       *   configure the permissions using `transfer_ownership` and `set_team`.
       * - `min_balance`: The minimum balance of this new asset that any single
       *   account must have. If an account's balance is reduced below this,
       *   then it collapses to zero.
       *
       * Emits `ForceCreated` event when successful.
       *
       * Weight: `O(1)`
       */
      forceCreate: AugmentedSubmittable<
        (
          id: Compact<u32> | AnyNumber | Uint8Array,
          owner:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          isSufficient: bool | boolean | Uint8Array,
          minBalance: Compact<u128> | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Compact<u32>, MultiAddress, bool, Compact<u128>]
      >;
      /**
       * Force the metadata for an asset to some value.
       *
       * Origin must be ForceOrigin.
       *
       * Any deposit is left alone.
       *
       * - `id`: The identifier of the asset to update.
       * - `name`: The user friendly name of this asset. Limited in length by `StringLimit`.
       * - `symbol`: The exchange symbol for this asset. Limited in length by `StringLimit`.
       * - `decimals`: The number of decimals this asset uses to represent one unit.
       *
       * Emits `MetadataSet`.
       *
       * Weight: `O(N + S)` where N and S are the length of the name and symbol
       * respectively.
       */
      forceSetMetadata: AugmentedSubmittable<
        (
          id: Compact<u32> | AnyNumber | Uint8Array,
          name: Bytes | string | Uint8Array,
          symbol: Bytes | string | Uint8Array,
          decimals: u8 | AnyNumber | Uint8Array,
          isFrozen: bool | boolean | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Compact<u32>, Bytes, Bytes, u8, bool]
      >;
      /**
       * Move some assets from one account to another.
       *
       * Origin must be Signed and the sender should be the Admin of the asset `id`.
       *
       * - `id`: The identifier of the asset to have some amount transferred.
       * - `source`: The account to be debited.
       * - `dest`: The account to be credited.
       * - `amount`: The amount by which the `source`'s balance of assets should
       *   be reduced and `dest`'s balance increased. The amount actually
       *   transferred may be slightly greater in the case that the transfer
       *   would otherwise take the `source` balance above zero but below the
       *   minimum balance. Must be greater than zero.
       *
       * Emits `Transferred` with the actual amount transferred. If this takes
       * the source balance to below the minimum for the asset, then the amount
       * transferred is increased to take it to zero.
       *
       * Weight: `O(1)` Modes: Pre-existence of `dest`; Post-existence of
       * `source`; Account pre-existence of `dest`.
       */
      forceTransfer: AugmentedSubmittable<
        (
          id: Compact<u32> | AnyNumber | Uint8Array,
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
          amount: Compact<u128> | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Compact<u32>, MultiAddress, MultiAddress, Compact<u128>]
      >;
      /**
       * Disallow further unprivileged transfers from an account.
       *
       * Origin must be Signed and the sender should be the Freezer of the asset `id`.
       *
       * - `id`: The identifier of the asset to be frozen.
       * - `who`: The account to be frozen.
       *
       * Emits `Frozen`.
       *
       * Weight: `O(1)`
       */
      freeze: AugmentedSubmittable<
        (
          id: Compact<u32> | AnyNumber | Uint8Array,
          who:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Compact<u32>, MultiAddress]
      >;
      /**
       * Disallow further unprivileged transfers for the asset class.
       *
       * Origin must be Signed and the sender should be the Freezer of the asset `id`.
       *
       * - `id`: The identifier of the asset to be frozen.
       *
       * Emits `Frozen`.
       *
       * Weight: `O(1)`
       */
      freezeAsset: AugmentedSubmittable<
        (
          id: Compact<u32> | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Compact<u32>]
      >;
      /**
       * Mint assets of a particular class.
       *
       * The origin must be Signed and the sender must be the Issuer of the asset `id`.
       *
       * - `id`: The identifier of the asset to have some amount minted.
       * - `beneficiary`: The account to be credited with the minted assets.
       * - `amount`: The amount of the asset to be minted.
       *
       * Emits `Issued` event when successful.
       *
       * Weight: `O(1)` Modes: Pre-existing balance of `beneficiary`; Account
       * pre-existence of `beneficiary`.
       */
      mint: AugmentedSubmittable<
        (
          id: Compact<u32> | AnyNumber | Uint8Array,
          beneficiary:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          amount: Compact<u128> | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Compact<u32>, MultiAddress, Compact<u128>]
      >;
      /**
       * Return the deposit (if any) of an asset account.
       *
       * The origin must be Signed.
       *
       * - `id`: The identifier of the asset for the account to be created.
       * - `allow_burn`: If `true` then assets may be destroyed in order to
       *   complete the refund.
       *
       * Emits `Refunded` event when successful.
       */
      refund: AugmentedSubmittable<
        (
          id: Compact<u32> | AnyNumber | Uint8Array,
          allowBurn: bool | boolean | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Compact<u32>, bool]
      >;
      /**
       * Set the metadata for an asset.
       *
       * Origin must be Signed and the sender should be the Owner of the asset `id`.
       *
       * Funds of sender are reserved according to the formula:
       * `MetadataDepositBase + MetadataDepositPerByte * (name.len +
       * symbol.len)` taking into account any already reserved funds.
       *
       * - `id`: The identifier of the asset to update.
       * - `name`: The user friendly name of this asset. Limited in length by `StringLimit`.
       * - `symbol`: The exchange symbol for this asset. Limited in length by `StringLimit`.
       * - `decimals`: The number of decimals this asset uses to represent one unit.
       *
       * Emits `MetadataSet`.
       *
       * Weight: `O(1)`
       */
      setMetadata: AugmentedSubmittable<
        (
          id: Compact<u32> | AnyNumber | Uint8Array,
          name: Bytes | string | Uint8Array,
          symbol: Bytes | string | Uint8Array,
          decimals: u8 | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Compact<u32>, Bytes, Bytes, u8]
      >;
      /**
       * Change the Issuer, Admin and Freezer of an asset.
       *
       * Origin must be Signed and the sender should be the Owner of the asset `id`.
       *
       * - `id`: The identifier of the asset to be frozen.
       * - `issuer`: The new Issuer of this asset.
       * - `admin`: The new Admin of this asset.
       * - `freezer`: The new Freezer of this asset.
       *
       * Emits `TeamChanged`.
       *
       * Weight: `O(1)`
       */
      setTeam: AugmentedSubmittable<
        (
          id: Compact<u32> | AnyNumber | Uint8Array,
          issuer:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          admin:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          freezer:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Compact<u32>, MultiAddress, MultiAddress, MultiAddress]
      >;
      /**
       * Allow unprivileged transfers from an account again.
       *
       * Origin must be Signed and the sender should be the Admin of the asset `id`.
       *
       * - `id`: The identifier of the asset to be frozen.
       * - `who`: The account to be unfrozen.
       *
       * Emits `Thawed`.
       *
       * Weight: `O(1)`
       */
      thaw: AugmentedSubmittable<
        (
          id: Compact<u32> | AnyNumber | Uint8Array,
          who:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Compact<u32>, MultiAddress]
      >;
      /**
       * Allow unprivileged transfers for the asset again.
       *
       * Origin must be Signed and the sender should be the Admin of the asset `id`.
       *
       * - `id`: The identifier of the asset to be thawed.
       *
       * Emits `Thawed`.
       *
       * Weight: `O(1)`
       */
      thawAsset: AugmentedSubmittable<
        (
          id: Compact<u32> | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Compact<u32>]
      >;
      /**
       * Create an asset account for non-provider assets.
       *
       * A deposit will be taken from the signer account.
       *
       * - `origin`: Must be Signed; the signer account must have sufficient funds
       *   for a deposit to be taken.
       * - `id`: The identifier of the asset for the account to be created.
       *
       * Emits `Touched` event when successful.
       */
      touch: AugmentedSubmittable<
        (
          id: Compact<u32> | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Compact<u32>]
      >;
      /**
       * Move some assets from the sender account to another.
       *
       * Origin must be Signed.
       *
       * - `id`: The identifier of the asset to have some amount transferred.
       * - `target`: The account to be credited.
       * - `amount`: The amount by which the sender's balance of assets should be
       *   reduced and `target`'s balance increased. The amount actually
       *   transferred may be slightly greater in the case that the transfer
       *   would otherwise take the sender balance above zero but below the
       *   minimum balance. Must be greater than zero.
       *
       * Emits `Transferred` with the actual amount transferred. If this takes
       * the source balance to below the minimum for the asset, then the amount
       * transferred is increased to take it to zero.
       *
       * Weight: `O(1)` Modes: Pre-existence of `target`; Post-existence of
       * sender; Account pre-existence of `target`.
       */
      transfer: AugmentedSubmittable<
        (
          id: Compact<u32> | AnyNumber | Uint8Array,
          target:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          amount: Compact<u128> | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Compact<u32>, MultiAddress, Compact<u128>]
      >;
      /**
       * Transfer some asset balance from a previously delegated account to some
       * third-party account.
       *
       * Origin must be Signed and there must be an approval in place by the
       * `owner` to the signer.
       *
       * If the entire amount approved for transfer is transferred, then any
       * deposit previously reserved by `approve_transfer` is unreserved.
       *
       * - `id`: The identifier of the asset.
       * - `owner`: The account which previously approved for a transfer of at
       *   least `amount` and from which the asset balance will be withdrawn.
       * - `destination`: The account to which the asset balance of `amount` will
       *   be transferred.
       * - `amount`: The amount of assets to transfer.
       *
       * Emits `TransferredApproved` on success.
       *
       * Weight: `O(1)`
       */
      transferApproved: AugmentedSubmittable<
        (
          id: Compact<u32> | AnyNumber | Uint8Array,
          owner:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          destination:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          amount: Compact<u128> | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Compact<u32>, MultiAddress, MultiAddress, Compact<u128>]
      >;
      /**
       * Move some assets from the sender account to another, keeping the sender
       * account alive.
       *
       * Origin must be Signed.
       *
       * - `id`: The identifier of the asset to have some amount transferred.
       * - `target`: The account to be credited.
       * - `amount`: The amount by which the sender's balance of assets should be
       *   reduced and `target`'s balance increased. The amount actually
       *   transferred may be slightly greater in the case that the transfer
       *   would otherwise take the sender balance above zero but below the
       *   minimum balance. Must be greater than zero.
       *
       * Emits `Transferred` with the actual amount transferred. If this takes
       * the source balance to below the minimum for the asset, then the amount
       * transferred is increased to take it to zero.
       *
       * Weight: `O(1)` Modes: Pre-existence of `target`; Post-existence of
       * sender; Account pre-existence of `target`.
       */
      transferKeepAlive: AugmentedSubmittable<
        (
          id: Compact<u32> | AnyNumber | Uint8Array,
          target:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          amount: Compact<u128> | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Compact<u32>, MultiAddress, Compact<u128>]
      >;
      /**
       * Change the Owner of an asset.
       *
       * Origin must be Signed and the sender should be the Owner of the asset `id`.
       *
       * - `id`: The identifier of the asset.
       * - `owner`: The new Owner of this asset.
       *
       * Emits `OwnerChanged`.
       *
       * Weight: `O(1)`
       */
      transferOwnership: AugmentedSubmittable<
        (
          id: Compact<u32> | AnyNumber | Uint8Array,
          owner:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Compact<u32>, MultiAddress]
      >;
      /** Generic tx */
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
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
            | T3rnTypesSideEffect
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
            | T3rnTypesSideEffectConfirmedSideEffect
            | {
                err?: any;
                output?: any;
                inclusionData?: any;
                executioner?: any;
                receivedAt?: any;
                cost?: any;
              }
            | string
            | Uint8Array,
          inclusionProof:
            | Option<Vec<Bytes>>
            | null
            | Uint8Array
            | Vec<Bytes>
            | (Bytes | string | Uint8Array)[],
          blockHash: Option<Bytes> | null | Uint8Array | Bytes | string
        ) => SubmittableExtrinsic<ApiType>,
        [
          H256,
          T3rnTypesSideEffect,
          T3rnTypesSideEffectConfirmedSideEffect,
          Option<Vec<Bytes>>,
          Option<Bytes>
        ]
      >;
      executeSideEffectsWithXbi: AugmentedSubmittable<
        (
          xtxId: H256 | string | Uint8Array,
          sideEffect:
            | T3rnTypesSideEffect
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
          maxExecCost: u128 | AnyNumber | Uint8Array,
          maxNotificationsCost: u128 | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [H256, T3rnTypesSideEffect, u128, u128]
      >;
      onExtrinsicTrigger: AugmentedSubmittable<
        (
          sideEffects:
            | Vec<T3rnTypesSideEffect>
            | (
                | T3rnTypesSideEffect
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
        [Vec<T3rnTypesSideEffect>, u128, bool]
      >;
      /** Used by other pallets that want to create the exec order */
      onLocalTrigger: AugmentedSubmittable<
        (trigger: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
        [Bytes]
      >;
      onRemoteGatewayTrigger: AugmentedSubmittable<
        () => SubmittableExtrinsic<ApiType>,
        []
      >;
      onXbiSfxResolved: AugmentedSubmittable<
        (sfxId: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
        [H256]
      >;
      onXcmTrigger: AugmentedSubmittable<
        () => SubmittableExtrinsic<ApiType>,
        []
      >;
      /** Generic tx */
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    contracts: {
      /**
       * Makes a call to an account, optionally transferring some balance.
       *
       * # Parameters
       *
       * - `dest`: Address of the contract to call.
       * - `value`: The balance to transfer from the `origin` to `dest`.
       * - `gas_limit`: The gas limit enforced when executing the constructor.
       * - `storage_deposit_limit`: The maximum amount of balance that can be
       *   charged from the caller to pay for the storage consumed.
       * - `data`: The input data to pass to the contract.
       * - If the account is a smart-contract account, the associated code will be
       *   executed and any value will be transferred.
       * - If the account is a regular account, any value will be transferred.
       * - If no account exists and the call value is not less than
       *   `existential_deposit`, a regular account will be created and any
       *   value will be transferred.
       */
      call: AugmentedSubmittable<
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
          value: Compact<u128> | AnyNumber | Uint8Array,
          gasLimit: Compact<u64> | AnyNumber | Uint8Array,
          storageDepositLimit:
            | Option<Compact<u128>>
            | null
            | Uint8Array
            | Compact<u128>
            | AnyNumber,
          data: Bytes | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [
          MultiAddress,
          Compact<u128>,
          Compact<u64>,
          Option<Compact<u128>>,
          Bytes
        ]
      >;
      /**
       * Instantiates a contract from a previously deployed wasm binary.
       *
       * This function is identical to [`Self::instantiate_with_code`] but
       * without the code deployment step. Instead, the `code_hash` of an
       * on-chain deployed wasm binary must be supplied.
       */
      instantiate: AugmentedSubmittable<
        (
          value: Compact<u128> | AnyNumber | Uint8Array,
          gasLimit: Compact<u64> | AnyNumber | Uint8Array,
          storageDepositLimit:
            | Option<Compact<u128>>
            | null
            | Uint8Array
            | Compact<u128>
            | AnyNumber,
          codeHash: H256 | string | Uint8Array,
          data: Bytes | string | Uint8Array,
          salt: Bytes | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Compact<u128>, Compact<u64>, Option<Compact<u128>>, H256, Bytes, Bytes]
      >;
      /**
       * Instantiates a new contract from the supplied `code` optionally
       * transferring some balance.
       *
       * This dispatchable has the same effect as calling [`Self::upload_code`]
       * + [`Self::instantiate`]. Bundling them together provides efficiency
       * gains. Please also check the documentation of [`Self::upload_code`].
       *
       * # Parameters
       *
       * - `value`: The balance to transfer from the `origin` to the newly created contract.
       * - `gas_limit`: The gas limit enforced when executing the constructor.
       * - `storage_deposit_limit`: The maximum amount of balance that can be
       *   charged/reserved from the caller to pay for the storage consumed.
       * - `code`: The contract code to deploy in raw bytes.
       * - `data`: The input data to pass to the contract constructor.
       * - `salt`: Used for the address derivation. See [`Pallet::contract_address`].
       *
       * Instantiation is executed as follows:
       *
       * - The supplied `code` is instrumented, deployed, and a `code_hash` is
       *   created for that code.
       * - If the `code_hash` already exists on the chain the underlying `code`
       *   will be shared.
       * - The destination address is computed based on the sender, code_hash and the salt.
       * - The smart-contract account is created at the computed address.
       * - The `value` is transferred to the new account.
       * - The `deploy` function is executed in the context of the newly-created account.
       */
      instantiateWithCode: AugmentedSubmittable<
        (
          value: Compact<u128> | AnyNumber | Uint8Array,
          gasLimit: Compact<u64> | AnyNumber | Uint8Array,
          storageDepositLimit:
            | Option<Compact<u128>>
            | null
            | Uint8Array
            | Compact<u128>
            | AnyNumber,
          code: Bytes | string | Uint8Array,
          data: Bytes | string | Uint8Array,
          salt: Bytes | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [
          Compact<u128>,
          Compact<u64>,
          Option<Compact<u128>>,
          Bytes,
          Bytes,
          Bytes
        ]
      >;
      /**
       * Remove the code stored under `code_hash` and refund the deposit to its owner.
       *
       * A code can only be removed by its original uploader (its owner) and
       * only if it is not used by any contract.
       */
      removeCode: AugmentedSubmittable<
        (codeHash: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
        [H256]
      >;
      /**
       * Upload new `code` without instantiating a contract from it.
       *
       * If the code does not already exist a deposit is reserved from the
       * caller and unreserved only when [`Self::remove_code`] is called. The
       * size of the reserve depends on the instrumented size of the the supplied `code`.
       *
       * If the code already exists in storage it will still return `Ok` and
       * upgrades the in storage version to the current
       * [`InstructionWeights::version`](InstructionWeights).
       *
       * # Note
       *
       * Anyone can instantiate a contract from any uploaded code and thus
       * prevent its removal. To avoid this situation a constructor could employ
       * access control so that it can only be instantiated by permissioned
       * entities. The same is true when uploading through
       * [`Self::instantiate_with_code`].
       */
      uploadCode: AugmentedSubmittable<
        (
          code: Bytes | string | Uint8Array,
          storageDepositLimit:
            | Option<Compact<u128>>
            | null
            | Uint8Array
            | Compact<u128>
            | AnyNumber
        ) => SubmittableExtrinsic<ApiType>,
        [Bytes, Option<Compact<u128>>]
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
            | T3rnPrimitivesContractsRegistryRegistryContract
            | {
                codeTxt?: any;
                bytes?: any;
                author?: any;
                abi?: any;
                actionDescriptions?: any;
                info?: any;
                meta?: any;
              }
            | string
            | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [AccountId32, T3rnPrimitivesContractsRegistryRegistryContract]
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
    evm: {
      /**
       * Issue an EVM call operation. This is similar to a message call
       * transaction in Ethereum.
       */
      call: AugmentedSubmittable<
        (
          target: H160 | string | Uint8Array,
          input: Bytes | string | Uint8Array,
          value: U256 | AnyNumber | Uint8Array,
          gasLimit: u64 | AnyNumber | Uint8Array,
          maxFeePerGas: U256 | AnyNumber | Uint8Array,
          maxPriorityFeePerGas:
            | Option<U256>
            | null
            | Uint8Array
            | U256
            | AnyNumber,
          nonce: Option<U256> | null | Uint8Array | U256 | AnyNumber,
          accessList:
            | Vec<ITuple<[H160, Vec<H256>]>>
            | [
                H160 | string | Uint8Array,
                Vec<H256> | (H256 | string | Uint8Array)[]
              ][]
        ) => SubmittableExtrinsic<ApiType>,
        [
          H160,
          Bytes,
          U256,
          u64,
          U256,
          Option<U256>,
          Option<U256>,
          Vec<ITuple<[H160, Vec<H256>]>>
        ]
      >;
      /** Claim an evm address, used to claim an evm address that is compatible with EVM. */
      claim: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
      /**
       * Issue an EVM create operation. This is similar to a contract creation
       * transaction in Ethereum.
       */
      create: AugmentedSubmittable<
        (
          init: Bytes | string | Uint8Array,
          value: U256 | AnyNumber | Uint8Array,
          gasLimit: u64 | AnyNumber | Uint8Array,
          maxFeePerGas: U256 | AnyNumber | Uint8Array,
          maxPriorityFeePerGas:
            | Option<U256>
            | null
            | Uint8Array
            | U256
            | AnyNumber,
          nonce: Option<U256> | null | Uint8Array | U256 | AnyNumber,
          accessList:
            | Vec<ITuple<[H160, Vec<H256>]>>
            | [
                H160 | string | Uint8Array,
                Vec<H256> | (H256 | string | Uint8Array)[]
              ][]
        ) => SubmittableExtrinsic<ApiType>,
        [
          Bytes,
          U256,
          u64,
          U256,
          Option<U256>,
          Option<U256>,
          Vec<ITuple<[H160, Vec<H256>]>>
        ]
      >;
      /** Issue an EVM create2 operation. */
      create2: AugmentedSubmittable<
        (
          init: Bytes | string | Uint8Array,
          salt: H256 | string | Uint8Array,
          value: U256 | AnyNumber | Uint8Array,
          gasLimit: u64 | AnyNumber | Uint8Array,
          maxFeePerGas: U256 | AnyNumber | Uint8Array,
          maxPriorityFeePerGas:
            | Option<U256>
            | null
            | Uint8Array
            | U256
            | AnyNumber,
          nonce: Option<U256> | null | Uint8Array | U256 | AnyNumber,
          accessList:
            | Vec<ITuple<[H160, Vec<H256>]>>
            | [
                H160 | string | Uint8Array,
                Vec<H256> | (H256 | string | Uint8Array)[]
              ][]
        ) => SubmittableExtrinsic<ApiType>,
        [
          Bytes,
          H256,
          U256,
          u64,
          U256,
          Option<U256>,
          Option<U256>,
          Vec<ITuple<[H160, Vec<H256>]>>
        ]
      >;
      /** Withdraw balance from EVM into currency/balances pallet. */
      withdraw: AugmentedSubmittable<
        (
          address: H160 | string | Uint8Array,
          value: u128 | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [H160, u128]
      >;
      /** Generic tx */
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    grandpa: {
      /**
       * Note that the current authority set of the GRANDPA finality gadget has stalled.
       *
       * This will trigger a forced authority set change at the beginning of the
       * next session, to be enacted `delay` blocks after that. The `delay`
       * should be high enough to safely assume that the block signalling the
       * forced change will not be re-orged e.g. 1000 blocks. The block
       * production rate (which may be slowed down because of finality lagging)
       * should be taken into account when choosing the `delay`. The GRANDPA
       * voters based on the new authority will start voting on top of
       * `best_finalized_block_number` for new finalized blocks.
       * `best_finalized_block_number` should be the highest of the latest
       * finalized block of all validators of the new authority set.
       *
       * Only callable by root.
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
    portal: {
      registerGateway: AugmentedSubmittable<
        (
          url: Bytes | string | Uint8Array,
          gatewayId: U8aFixed | string | Uint8Array,
          gatewayAbi:
            | T3rnTypesAbiGatewayABIConfig
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
            | "Polkadot"
            | "Kusama"
            | "Rococo"
            | "Ethereum"
            | number
            | Uint8Array,
          gatewayType:
            | T3rnPrimitivesGatewayType
            | { ProgrammableInternal: any }
            | { ProgrammableExternal: any }
            | { TxOnly: any }
            | { OnCircuit: any }
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
          allowedSideEffects: Vec<U8aFixed>,
          encodedRegistrationData: Bytes | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [
          Bytes,
          U8aFixed,
          T3rnTypesAbiGatewayABIConfig,
          T3rnPrimitivesGatewayVendor,
          T3rnPrimitivesGatewayType,
          T3rnPrimitivesGatewayGenesisConfig,
          T3rnPrimitivesGatewaySysProps,
          Vec<U8aFixed>,
          Bytes
        ]
      >;
      setOperational: AugmentedSubmittable<
        (
          gatewayId: U8aFixed | string | Uint8Array,
          operational: bool | boolean | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [U8aFixed, bool]
      >;
      setOwner: AugmentedSubmittable<
        (
          gatewayId: U8aFixed | string | Uint8Array,
          encodedNewOwner: Bytes | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [U8aFixed, Bytes]
      >;
      submitHeaders: AugmentedSubmittable<
        (
          gatewayId: U8aFixed | string | Uint8Array,
          encodedHeaderData: Bytes | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [U8aFixed, Bytes]
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
          call: Call | IMethod | string | Uint8Array
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
          call: Call | IMethod | string | Uint8Array
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
          call: Call | IMethod | string | Uint8Array,
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
    threeVm: {
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
    treasury: {
      addBeneficiary: AugmentedSubmittable<
        (
          beneficiary: AccountId32 | string | Uint8Array,
          role:
            | T3rnPrimitivesMonetaryBeneficiaryRole
            | "Developer"
            | "Executor"
            | number
            | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [AccountId32, T3rnPrimitivesMonetaryBeneficiaryRole]
      >;
      claimRewards: AugmentedSubmittable<
        () => SubmittableExtrinsic<ApiType>,
        []
      >;
      /**
       * Mints tokens for given round. TODO: maybe ensure can only be called
       * once per round TODO: exec, infl
       */
      mintForRound: AugmentedSubmittable<
        (
          roundIndex: u32 | AnyNumber | Uint8Array,
          amount: Compact<u128> | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [u32, Compact<u128>]
      >;
      removeBeneficiary: AugmentedSubmittable<
        (
          beneficiary: AccountId32 | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [AccountId32]
      >;
      /** Sets the annual inflation rate to derive per-round inflation */
      setInflation: AugmentedSubmittable<
        (
          annualInflationConfig:
            | ({
                readonly min: Perbill;
                readonly ideal: Perbill;
                readonly max: Perbill;
              } & Struct)
            | { min?: any; ideal?: any; max?: any }
            | string
            | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [
          {
            readonly min: Perbill;
            readonly ideal: Perbill;
            readonly max: Perbill;
          } & Struct
        ]
      >;
      /** Sets the reward percentage to be allocated amongst t3rn actors */
      setInflationAlloc: AugmentedSubmittable<
        (
          inflationAlloc:
            | T3rnPrimitivesMonetaryInflationAllocation
            | { developer?: any; executor?: any }
            | string
            | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [T3rnPrimitivesMonetaryInflationAllocation]
      >;
      /**
       * Set blocks per round
       *
       * - If called with `new` less than term of current round, will transition
       *   immediately in the next block
       * - Also updates per-round inflation config
       */
      setRoundTerm: AugmentedSubmittable<
        (
          updated: u32 | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [u32]
      >;
      /**
       * Set the expectations for total staked. These expectations determine the
       * issuance for the round according to logic in `fn compute_round_issuance`.
       */
      setTotalStakeExpectation: AugmentedSubmittable<
        (
          expectations:
            | ({
                readonly min: u128;
                readonly ideal: u128;
                readonly max: u128;
              } & Struct)
            | { min?: any; ideal?: any; max?: any }
            | string
            | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [
          {
            readonly min: u128;
            readonly ideal: u128;
            readonly max: u128;
          } & Struct
        ]
      >;
      /** Generic tx */
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    utility: {
      /**
       * Send a call through an indexed pseudonym of the sender.
       *
       * Filter from origin are passed along. The call will be dispatched with
       * an origin which use the same filter as the origin of this call.
       *
       * NOTE: If you need to ensure that any account-based filtering is not
       * honored (i.e. because you expect `proxy` to have been used prior in the
       * call stack and you do not want the call restrictions to apply to any
       * sub-accounts), then use `as_multi_threshold_1` in the Multisig pallet instead.
       *
       * NOTE: Prior to version *12, this was called `as_limited_sub`.
       *
       * The dispatch origin for this call must be _Signed_.
       */
      asDerivative: AugmentedSubmittable<
        (
          index: u16 | AnyNumber | Uint8Array,
          call: Call | IMethod | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [u16, Call]
      >;
      /**
       * Send a batch of dispatch calls.
       *
       * May be called from any origin.
       *
       * - `calls`: The calls to be dispatched from the same origin. The number of
       *   call must not exceed the constant: `batched_calls_limit` (available
       *   in constant metadata).
       *
       * If origin is root then call are dispatch without checking origin
       * filter. (This includes bypassing `frame_system::Config::BaseCallFilter`).
       *
       * # <weight>
       *
       * - Complexity: O(C) where C is the number of calls to be batched.
       *
       * # </weight>
       *
       * This will return `Ok` in all circumstances. To determine the success of
       * the batch, an event is deposited. If a call failed and the batch was
       * interrupted, then the `BatchInterrupted` event is deposited, along with
       * the number of successful calls made and the error of the failed call.
       * If all were successful, then the `BatchCompleted` event is deposited.
       */
      batch: AugmentedSubmittable<
        (
          calls: Vec<Call> | (Call | IMethod | string | Uint8Array)[]
        ) => SubmittableExtrinsic<ApiType>,
        [Vec<Call>]
      >;
      /**
       * Send a batch of dispatch calls and atomically execute them. The whole
       * transaction will rollback and fail if any of the calls failed.
       *
       * May be called from any origin.
       *
       * - `calls`: The calls to be dispatched from the same origin. The number of
       *   call must not exceed the constant: `batched_calls_limit` (available
       *   in constant metadata).
       *
       * If origin is root then call are dispatch without checking origin
       * filter. (This includes bypassing `frame_system::Config::BaseCallFilter`).
       *
       * # <weight>
       *
       * - Complexity: O(C) where C is the number of calls to be batched.
       *
       * # </weight>
       */
      batchAll: AugmentedSubmittable<
        (
          calls: Vec<Call> | (Call | IMethod | string | Uint8Array)[]
        ) => SubmittableExtrinsic<ApiType>,
        [Vec<Call>]
      >;
      /**
       * Dispatches a function call with a provided origin.
       *
       * The dispatch origin for this call must be _Root_.
       *
       * # <weight>
       *
       * - O(1).
       * - Limited storage reads.
       * - One DB write (event).
       * - Weight of derivative `call` execution + T::WeightInfo::dispatch_as().
       *
       * # </weight>
       */
      dispatchAs: AugmentedSubmittable<
        (
          asOrigin:
            | CircuitStandaloneRuntimeOriginCaller
            | { system: any }
            | { Void: any }
            | string
            | Uint8Array,
          call: Call | IMethod | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [CircuitStandaloneRuntimeOriginCaller, Call]
      >;
      /**
       * Send a batch of dispatch calls. Unlike `batch`, it allows errors and
       * won't interrupt.
       *
       * May be called from any origin.
       *
       * - `calls`: The calls to be dispatched from the same origin. The number of
       *   call must not exceed the constant: `batched_calls_limit` (available
       *   in constant metadata).
       *
       * If origin is root then call are dispatch without checking origin
       * filter. (This includes bypassing `frame_system::Config::BaseCallFilter`).
       *
       * # <weight>
       *
       * - Complexity: O(C) where C is the number of calls to be batched.
       *
       * # </weight>
       */
      forceBatch: AugmentedSubmittable<
        (
          calls: Vec<Call> | (Call | IMethod | string | Uint8Array)[]
        ) => SubmittableExtrinsic<ApiType>,
        [Vec<Call>]
      >;
      /** Generic tx */
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    xbiPortal: {
      checkInXbi: AugmentedSubmittable<
        (
          xbi:
            | PalletXbiPortalXbiFormat
            | { instr?: any; metadata?: any }
            | string
            | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [PalletXbiPortalXbiFormat]
      >;
      cleanup: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
      /**
       * Enter might be weight heavy - calls for execution into EVMs and if
       * necessary sends the response If returns XBICheckOut means that executed
       * instantly and the XBI order can be removed from pending checkouts
       */
      enterCall: AugmentedSubmittable<
        (
          checkin:
            | PalletXbiPortalXbiFormatXbiCheckIn
            | {
                xbi?: any;
                notificationDeliveryTimeout?: any;
                notificationExecutionTimeout?: any;
              }
            | string
            | Uint8Array,
          xbiId: H256 | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [PalletXbiPortalXbiFormatXbiCheckIn, H256]
      >;
      executeXcm: AugmentedSubmittable<
        (xcm: XcmV2Xcm) => SubmittableExtrinsic<ApiType>,
        [XcmV2Xcm]
      >;
      /** Generic tx */
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    xdns: {
      addSideEffect: AugmentedSubmittable<
        (
          id: U8aFixed | string | Uint8Array,
          name: Bytes | string | Uint8Array,
          argumentAbi:
            | Vec<T3rnTypesAbiType>
            | (
                | T3rnTypesAbiType
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
          Vec<T3rnTypesAbiType>,
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
