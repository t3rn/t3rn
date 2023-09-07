// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/api-base/types/submittable';

import type { ApiTypes, AugmentedSubmittable, SubmittableExtrinsic, SubmittableExtrinsicFunction } from '@polkadot/api-base/types';
import type { Data } from '@polkadot/types';
import type { Bytes, Compact, Option, U256, U8aFixed, Vec, bool, u128, u16, u32, u64, u8 } from '@polkadot/types-codec';
import type { AnyNumber, IMethod, ITuple } from '@polkadot/types-codec/types';
import type { AccountId32, Call, H160, H256, MultiAddress, Percent } from '@polkadot/types/interfaces/runtime';
import type { CumulusPrimitivesParachainInherentParachainInherentData, PalletAssetRegistryAssetInfo, PalletContractsWasmDeterminism, PalletEth2FinalityVerifierBeaconBlockHeader, PalletEth2FinalityVerifierEthereumEventInclusionProof, PalletEth2FinalityVerifierEthereumReceiptInclusionProof, PalletEth2FinalityVerifierExecutionHeader, PalletEth2FinalityVerifierExecutionPayload, PalletEth2FinalityVerifierMerkleProof, PalletEth2FinalityVerifierSyncCommittee, PalletGrandpaFinalityVerifierBridgesHeaderChainJustificationGrandpaJustification, PalletIdentityBitFlags, PalletIdentityIdentityInfo, PalletIdentityJudgement, PalletSepoliaFinalityVerifierBeaconBlockHeader, PalletSepoliaFinalityVerifierEthereumEventInclusionProof, PalletSepoliaFinalityVerifierEthereumReceiptInclusionProof, PalletSepoliaFinalityVerifierExecutionHeader, PalletSepoliaFinalityVerifierExecutionPayload, PalletSepoliaFinalityVerifierMerkleProof, PalletSepoliaFinalityVerifierSyncCommittee, SpRuntimeHeader, SpWeightsWeightV2Weight, T0rnParachainRuntimeOriginCaller, T0rnParachainRuntimeParachainConfigSessionKeys, T3rnAbiRecodeCodec, T3rnPrimitivesAccountManagerOutcome, T3rnPrimitivesCircuitTypesOrderSFX, T3rnPrimitivesClaimableBenefitSource, T3rnPrimitivesClaimableCircuitRole, T3rnPrimitivesContractsRegistryRegistryContract, T3rnPrimitivesExecutionVendor, T3rnPrimitivesGatewayVendor, T3rnPrimitivesSpeedMode, T3rnPrimitivesTokenInfo, T3rnTypesSfxConfirmedSideEffect, T3rnTypesSfxSecurityLvl, T3rnTypesSfxSideEffect, XcmV3MultiLocation, XcmV3WeightLimit, XcmVersionedMultiAssets, XcmVersionedMultiLocation, XcmVersionedXcm } from '@polkadot/types/lookup';

export type __AugmentedSubmittable = AugmentedSubmittable<() => unknown>;
export type __SubmittableExtrinsic<ApiType extends ApiTypes> = SubmittableExtrinsic<ApiType>;
export type __SubmittableExtrinsicFunction<ApiType extends ApiTypes> = SubmittableExtrinsicFunction<ApiType>;

declare module '@polkadot/api-base/types/submittable' {
  interface AugmentedSubmittables<ApiType extends ApiTypes> {
    accountManager: {
      /**
       * See [`Pallet::deposit`].
       **/
      deposit: AugmentedSubmittable<(chargeId: H256 | string | Uint8Array, payee: AccountId32 | string | Uint8Array, chargeFee: u128 | AnyNumber | Uint8Array, offeredReward: u128 | AnyNumber | Uint8Array, source: T3rnPrimitivesClaimableBenefitSource | 'BootstrapPool' | 'Inflation' | 'TrafficFees' | 'TrafficRewards' | 'Unsettled' | 'SlashTreasury' | number | Uint8Array, role: T3rnPrimitivesClaimableCircuitRole | 'Ambassador' | 'Executor' | 'Attester' | 'Staker' | 'Collator' | 'ContractAuthor' | 'Relayer' | 'Requester' | 'Local' | number | Uint8Array, recipient: Option<AccountId32> | null | Uint8Array | AccountId32 | string, maybeAssetId: Option<u32> | null | Uint8Array | u32 | AnyNumber) => SubmittableExtrinsic<ApiType>, [H256, AccountId32, u128, u128, T3rnPrimitivesClaimableBenefitSource, T3rnPrimitivesClaimableCircuitRole, Option<AccountId32>, Option<u32>]>;
      /**
       * See [`Pallet::finalize`].
       **/
      finalize: AugmentedSubmittable<(chargeId: H256 | string | Uint8Array, outcome: T3rnPrimitivesAccountManagerOutcome | 'UnexpectedFailure' | 'Revert' | 'Commit' | 'Slash' | number | Uint8Array, maybeRecipient: Option<AccountId32> | null | Uint8Array | AccountId32 | string, maybeActualFees: Option<u128> | null | Uint8Array | u128 | AnyNumber) => SubmittableExtrinsic<ApiType>, [H256, T3rnPrimitivesAccountManagerOutcome, Option<AccountId32>, Option<u128>]>;
    };
    assetRegistry: {
      /**
       * See [`Pallet::register`].
       **/
      register: AugmentedSubmittable<(location: XcmV3MultiLocation | { parents?: any; interior?: any } | string | Uint8Array, id: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [XcmV3MultiLocation, u32]>;
      /**
       * See [`Pallet::register_info`].
       **/
      registerInfo: AugmentedSubmittable<(info: PalletAssetRegistryAssetInfo | { id?: any; capabilities?: any; location?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [PalletAssetRegistryAssetInfo]>;
    };
    assets: {
      /**
       * See [`Pallet::approve_transfer`].
       **/
      approveTransfer: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array, delegate: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, amount: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, MultiAddress, Compact<u128>]>;
      /**
       * See [`Pallet::block`].
       **/
      block: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array, who: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, MultiAddress]>;
      /**
       * See [`Pallet::burn`].
       **/
      burn: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array, who: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, amount: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, MultiAddress, Compact<u128>]>;
      /**
       * See [`Pallet::cancel_approval`].
       **/
      cancelApproval: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array, delegate: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, MultiAddress]>;
      /**
       * See [`Pallet::clear_metadata`].
       **/
      clearMetadata: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * See [`Pallet::create`].
       **/
      create: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array, admin: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, minBalance: u128 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, MultiAddress, u128]>;
      /**
       * See [`Pallet::destroy_accounts`].
       **/
      destroyAccounts: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * See [`Pallet::destroy_approvals`].
       **/
      destroyApprovals: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * See [`Pallet::finish_destroy`].
       **/
      finishDestroy: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * See [`Pallet::force_asset_status`].
       **/
      forceAssetStatus: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array, owner: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, issuer: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, admin: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, freezer: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, minBalance: Compact<u128> | AnyNumber | Uint8Array, isSufficient: bool | boolean | Uint8Array, isFrozen: bool | boolean | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, MultiAddress, MultiAddress, MultiAddress, MultiAddress, Compact<u128>, bool, bool]>;
      /**
       * See [`Pallet::force_cancel_approval`].
       **/
      forceCancelApproval: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array, owner: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, delegate: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, MultiAddress, MultiAddress]>;
      /**
       * See [`Pallet::force_clear_metadata`].
       **/
      forceClearMetadata: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * See [`Pallet::force_create`].
       **/
      forceCreate: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array, owner: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, isSufficient: bool | boolean | Uint8Array, minBalance: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, MultiAddress, bool, Compact<u128>]>;
      /**
       * See [`Pallet::force_set_metadata`].
       **/
      forceSetMetadata: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array, name: Bytes | string | Uint8Array, symbol: Bytes | string | Uint8Array, decimals: u8 | AnyNumber | Uint8Array, isFrozen: bool | boolean | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, Bytes, Bytes, u8, bool]>;
      /**
       * See [`Pallet::force_transfer`].
       **/
      forceTransfer: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array, source: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, dest: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, amount: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, MultiAddress, MultiAddress, Compact<u128>]>;
      /**
       * See [`Pallet::freeze`].
       **/
      freeze: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array, who: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, MultiAddress]>;
      /**
       * See [`Pallet::freeze_asset`].
       **/
      freezeAsset: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * See [`Pallet::mint`].
       **/
      mint: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array, beneficiary: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, amount: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, MultiAddress, Compact<u128>]>;
      /**
       * See [`Pallet::refund`].
       **/
      refund: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array, allowBurn: bool | boolean | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, bool]>;
      /**
       * See [`Pallet::refund_other`].
       **/
      refundOther: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array, who: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, MultiAddress]>;
      /**
       * See [`Pallet::set_metadata`].
       **/
      setMetadata: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array, name: Bytes | string | Uint8Array, symbol: Bytes | string | Uint8Array, decimals: u8 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, Bytes, Bytes, u8]>;
      /**
       * See [`Pallet::set_min_balance`].
       **/
      setMinBalance: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array, minBalance: u128 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u128]>;
      /**
       * See [`Pallet::set_team`].
       **/
      setTeam: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array, issuer: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, admin: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, freezer: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, MultiAddress, MultiAddress, MultiAddress]>;
      /**
       * See [`Pallet::start_destroy`].
       **/
      startDestroy: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * See [`Pallet::thaw`].
       **/
      thaw: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array, who: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, MultiAddress]>;
      /**
       * See [`Pallet::thaw_asset`].
       **/
      thawAsset: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * See [`Pallet::touch`].
       **/
      touch: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * See [`Pallet::touch_other`].
       **/
      touchOther: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array, who: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, MultiAddress]>;
      /**
       * See [`Pallet::transfer`].
       **/
      transfer: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array, target: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, amount: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, MultiAddress, Compact<u128>]>;
      /**
       * See [`Pallet::transfer_approved`].
       **/
      transferApproved: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array, owner: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, destination: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, amount: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, MultiAddress, MultiAddress, Compact<u128>]>;
      /**
       * See [`Pallet::transfer_keep_alive`].
       **/
      transferKeepAlive: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array, target: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, amount: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, MultiAddress, Compact<u128>]>;
      /**
       * See [`Pallet::transfer_ownership`].
       **/
      transferOwnership: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array, owner: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, MultiAddress]>;
    };
    attesters: {
      /**
       * See [`Pallet::add_attestation_target`].
       **/
      addAttestationTarget: AugmentedSubmittable<(target: U8aFixed | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [U8aFixed]>;
      /**
       * See [`Pallet::agree_to_new_attestation_target`].
       **/
      agreeToNewAttestationTarget: AugmentedSubmittable<(target: U8aFixed | string | Uint8Array, recoverable: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [U8aFixed, Bytes]>;
      /**
       * See [`Pallet::commit_batch`].
       **/
      commitBatch: AugmentedSubmittable<(target: U8aFixed | string | Uint8Array, targetInclusionProofEncoded: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [U8aFixed, Bytes]>;
      /**
       * See [`Pallet::deregister_attester`].
       **/
      deregisterAttester: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
      /**
       * See [`Pallet::estimate_user_finality_fee`].
       **/
      estimateUserFinalityFee: AugmentedSubmittable<(target: U8aFixed | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [U8aFixed]>;
      /**
       * See [`Pallet::force_activate_target`].
       **/
      forceActivateTarget: AugmentedSubmittable<(target: U8aFixed | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [U8aFixed]>;
      /**
       * See [`Pallet::nominate`].
       **/
      nominate: AugmentedSubmittable<(attester: AccountId32 | string | Uint8Array, amount: u128 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [AccountId32, u128]>;
      /**
       * See [`Pallet::read_latest_batching_factor_overview`].
       **/
      readLatestBatchingFactorOverview: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
      /**
       * See [`Pallet::read_pending_batches`].
       **/
      readPendingBatches: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
      /**
       * See [`Pallet::register_attester`].
       **/
      registerAttester: AugmentedSubmittable<(selfNominateAmount: u128 | AnyNumber | Uint8Array, ecdsaKey: U8aFixed | string | Uint8Array, ed25519Key: U8aFixed | string | Uint8Array, sr25519Key: U8aFixed | string | Uint8Array, customCommission: Option<Percent> | null | Uint8Array | Percent | AnyNumber) => SubmittableExtrinsic<ApiType>, [u128, U8aFixed, U8aFixed, U8aFixed, Option<Percent>]>;
      /**
       * See [`Pallet::remove_attestation_target`].
       **/
      removeAttestationTarget: AugmentedSubmittable<(target: U8aFixed | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [U8aFixed]>;
      /**
       * See [`Pallet::submit_attestation`].
       **/
      submitAttestation: AugmentedSubmittable<(message: H256 | string | Uint8Array, signature: Bytes | string | Uint8Array, target: U8aFixed | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [H256, Bytes, U8aFixed]>;
      /**
       * See [`Pallet::unnominate`].
       **/
      unnominate: AugmentedSubmittable<(attester: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [AccountId32]>;
    };
    balances: {
      /**
       * See [`Pallet::force_set_balance`].
       **/
      forceSetBalance: AugmentedSubmittable<(who: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, newFree: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, Compact<u128>]>;
      /**
       * See [`Pallet::force_transfer`].
       **/
      forceTransfer: AugmentedSubmittable<(source: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, dest: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, value: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, MultiAddress, Compact<u128>]>;
      /**
       * See [`Pallet::force_unreserve`].
       **/
      forceUnreserve: AugmentedSubmittable<(who: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, amount: u128 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, u128]>;
      /**
       * See [`Pallet::set_balance_deprecated`].
       **/
      setBalanceDeprecated: AugmentedSubmittable<(who: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, newFree: Compact<u128> | AnyNumber | Uint8Array, oldReserved: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, Compact<u128>, Compact<u128>]>;
      /**
       * See [`Pallet::transfer`].
       **/
      transfer: AugmentedSubmittable<(dest: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, value: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, Compact<u128>]>;
      /**
       * See [`Pallet::transfer_all`].
       **/
      transferAll: AugmentedSubmittable<(dest: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, keepAlive: bool | boolean | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, bool]>;
      /**
       * See [`Pallet::transfer_allow_death`].
       **/
      transferAllowDeath: AugmentedSubmittable<(dest: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, value: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, Compact<u128>]>;
      /**
       * See [`Pallet::transfer_keep_alive`].
       **/
      transferKeepAlive: AugmentedSubmittable<(dest: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, value: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, Compact<u128>]>;
      /**
       * See [`Pallet::upgrade_accounts`].
       **/
      upgradeAccounts: AugmentedSubmittable<(who: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [Vec<AccountId32>]>;
    };
    circuit: {
      /**
       * See [`Pallet::bid_sfx`].
       **/
      bidSfx: AugmentedSubmittable<(sfxId: H256 | string | Uint8Array, bidAmount: u128 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [H256, u128]>;
      /**
       * See [`Pallet::cancel_xtx`].
       **/
      cancelXtx: AugmentedSubmittable<(xtxId: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [H256]>;
      /**
       * See [`Pallet::confirm_side_effect`].
       **/
      confirmSideEffect: AugmentedSubmittable<(sfxId: H256 | string | Uint8Array, confirmation: T3rnTypesSfxConfirmedSideEffect | { err?: any; output?: any; inclusionData?: any; executioner?: any; receivedAt?: any; cost?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [H256, T3rnTypesSfxConfirmedSideEffect]>;
      /**
       * See [`Pallet::on_extrinsic_trigger`].
       **/
      onExtrinsicTrigger: AugmentedSubmittable<(sideEffects: Vec<T3rnTypesSfxSideEffect> | (T3rnTypesSfxSideEffect | { target?: any; maxReward?: any; insurance?: any; action?: any; encodedArgs?: any; signature?: any; enforceExecutor?: any; rewardAssetId?: any } | string | Uint8Array)[], speedMode: T3rnPrimitivesSpeedMode | 'Fast' | 'Rational' | 'Finalized' | number | Uint8Array, preferredSecurityLevel: T3rnTypesSfxSecurityLvl | 'Optimistic' | 'Escrow' | number | Uint8Array) => SubmittableExtrinsic<ApiType>, [Vec<T3rnTypesSfxSideEffect>, T3rnPrimitivesSpeedMode, T3rnTypesSfxSecurityLvl]>;
      /**
       * See [`Pallet::on_local_trigger`].
       **/
      onLocalTrigger: AugmentedSubmittable<(trigger: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes]>;
      /**
       * See [`Pallet::on_remote_gateway_trigger`].
       **/
      onRemoteGatewayTrigger: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
      /**
       * See [`Pallet::on_remote_origin_trigger`].
       **/
      onRemoteOriginTrigger: AugmentedSubmittable<(orderOrigin: AccountId32 | string | Uint8Array, sideEffects: Vec<T3rnTypesSfxSideEffect> | (T3rnTypesSfxSideEffect | { target?: any; maxReward?: any; insurance?: any; action?: any; encodedArgs?: any; signature?: any; enforceExecutor?: any; rewardAssetId?: any } | string | Uint8Array)[], speedMode: T3rnPrimitivesSpeedMode | 'Fast' | 'Rational' | 'Finalized' | number | Uint8Array) => SubmittableExtrinsic<ApiType>, [AccountId32, Vec<T3rnTypesSfxSideEffect>, T3rnPrimitivesSpeedMode]>;
      /**
       * See [`Pallet::on_xcm_trigger`].
       **/
      onXcmTrigger: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
      /**
       * See [`Pallet::revert`].
       **/
      revert: AugmentedSubmittable<(xtxId: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [H256]>;
      /**
       * See [`Pallet::trigger_dlq`].
       **/
      triggerDlq: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
    };
    collatorSelection: {
      /**
       * See [`Pallet::add_invulnerable`].
       **/
      addInvulnerable: AugmentedSubmittable<(who: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [AccountId32]>;
      /**
       * See [`Pallet::leave_intent`].
       **/
      leaveIntent: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
      /**
       * See [`Pallet::register_as_candidate`].
       **/
      registerAsCandidate: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
      /**
       * See [`Pallet::remove_invulnerable`].
       **/
      removeInvulnerable: AugmentedSubmittable<(who: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [AccountId32]>;
      /**
       * See [`Pallet::set_candidacy_bond`].
       **/
      setCandidacyBond: AugmentedSubmittable<(bond: u128 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u128]>;
      /**
       * See [`Pallet::set_desired_candidates`].
       **/
      setDesiredCandidates: AugmentedSubmittable<(max: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * See [`Pallet::set_invulnerables`].
       **/
      setInvulnerables: AugmentedSubmittable<(updated: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [Vec<AccountId32>]>;
    };
    contracts: {
      /**
       * See [`Pallet::call`].
       **/
      call: AugmentedSubmittable<(dest: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, value: Compact<u128> | AnyNumber | Uint8Array, gasLimit: SpWeightsWeightV2Weight | { refTime?: any; proofSize?: any } | string | Uint8Array, storageDepositLimit: Option<Compact<u128>> | null | Uint8Array | Compact<u128> | AnyNumber, data: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, Compact<u128>, SpWeightsWeightV2Weight, Option<Compact<u128>>, Bytes]>;
      /**
       * See [`Pallet::call_old_weight`].
       **/
      callOldWeight: AugmentedSubmittable<(dest: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, value: Compact<u128> | AnyNumber | Uint8Array, gasLimit: Compact<u64> | AnyNumber | Uint8Array, storageDepositLimit: Option<Compact<u128>> | null | Uint8Array | Compact<u128> | AnyNumber, data: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, Compact<u128>, Compact<u64>, Option<Compact<u128>>, Bytes]>;
      /**
       * See [`Pallet::instantiate`].
       **/
      instantiate: AugmentedSubmittable<(value: Compact<u128> | AnyNumber | Uint8Array, gasLimit: SpWeightsWeightV2Weight | { refTime?: any; proofSize?: any } | string | Uint8Array, storageDepositLimit: Option<Compact<u128>> | null | Uint8Array | Compact<u128> | AnyNumber, codeHash: H256 | string | Uint8Array, data: Bytes | string | Uint8Array, salt: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u128>, SpWeightsWeightV2Weight, Option<Compact<u128>>, H256, Bytes, Bytes]>;
      /**
       * See [`Pallet::instantiate_old_weight`].
       **/
      instantiateOldWeight: AugmentedSubmittable<(value: Compact<u128> | AnyNumber | Uint8Array, gasLimit: Compact<u64> | AnyNumber | Uint8Array, storageDepositLimit: Option<Compact<u128>> | null | Uint8Array | Compact<u128> | AnyNumber, codeHash: H256 | string | Uint8Array, data: Bytes | string | Uint8Array, salt: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u128>, Compact<u64>, Option<Compact<u128>>, H256, Bytes, Bytes]>;
      /**
       * See [`Pallet::instantiate_with_code`].
       **/
      instantiateWithCode: AugmentedSubmittable<(value: Compact<u128> | AnyNumber | Uint8Array, gasLimit: SpWeightsWeightV2Weight | { refTime?: any; proofSize?: any } | string | Uint8Array, storageDepositLimit: Option<Compact<u128>> | null | Uint8Array | Compact<u128> | AnyNumber, code: Bytes | string | Uint8Array, data: Bytes | string | Uint8Array, salt: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u128>, SpWeightsWeightV2Weight, Option<Compact<u128>>, Bytes, Bytes, Bytes]>;
      /**
       * See [`Pallet::instantiate_with_code_old_weight`].
       **/
      instantiateWithCodeOldWeight: AugmentedSubmittable<(value: Compact<u128> | AnyNumber | Uint8Array, gasLimit: Compact<u64> | AnyNumber | Uint8Array, storageDepositLimit: Option<Compact<u128>> | null | Uint8Array | Compact<u128> | AnyNumber, code: Bytes | string | Uint8Array, data: Bytes | string | Uint8Array, salt: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u128>, Compact<u64>, Option<Compact<u128>>, Bytes, Bytes, Bytes]>;
      /**
       * See [`Pallet::migrate`].
       **/
      migrate: AugmentedSubmittable<(weightLimit: SpWeightsWeightV2Weight | { refTime?: any; proofSize?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [SpWeightsWeightV2Weight]>;
      /**
       * See [`Pallet::remove_code`].
       **/
      removeCode: AugmentedSubmittable<(codeHash: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [H256]>;
      /**
       * See [`Pallet::set_code`].
       **/
      setCode: AugmentedSubmittable<(dest: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, codeHash: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, H256]>;
      /**
       * See [`Pallet::upload_code`].
       **/
      uploadCode: AugmentedSubmittable<(code: Bytes | string | Uint8Array, storageDepositLimit: Option<Compact<u128>> | null | Uint8Array | Compact<u128> | AnyNumber, determinism: PalletContractsWasmDeterminism | 'Enforced' | 'Relaxed' | number | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes, Option<Compact<u128>>, PalletContractsWasmDeterminism]>;
    };
    contractsRegistry: {
      /**
       * See [`Pallet::add_new_contract`].
       **/
      addNewContract: AugmentedSubmittable<(requester: AccountId32 | string | Uint8Array, contract: T3rnPrimitivesContractsRegistryRegistryContract | { codeTxt?: any; bytes?: any; author?: any; abi?: any; actionDescriptions?: any; info?: any; meta?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [AccountId32, T3rnPrimitivesContractsRegistryRegistryContract]>;
      /**
       * See [`Pallet::purge`].
       **/
      purge: AugmentedSubmittable<(requester: AccountId32 | string | Uint8Array, contractId: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [AccountId32, H256]>;
    };
    cumulusXcm: {
    };
    dmpQueue: {
      /**
       * See [`Pallet::service_overweight`].
       **/
      serviceOverweight: AugmentedSubmittable<(index: u64 | AnyNumber | Uint8Array, weightLimit: SpWeightsWeightV2Weight | { refTime?: any; proofSize?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u64, SpWeightsWeightV2Weight]>;
    };
    escrowTreasury: {
      /**
       * See [`Pallet::approve_proposal`].
       **/
      approveProposal: AugmentedSubmittable<(proposalId: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u32>]>;
      /**
       * See [`Pallet::propose_spend`].
       **/
      proposeSpend: AugmentedSubmittable<(value: Compact<u128> | AnyNumber | Uint8Array, beneficiary: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u128>, MultiAddress]>;
      /**
       * See [`Pallet::reject_proposal`].
       **/
      rejectProposal: AugmentedSubmittable<(proposalId: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u32>]>;
      /**
       * See [`Pallet::remove_approval`].
       **/
      removeApproval: AugmentedSubmittable<(proposalId: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u32>]>;
      /**
       * See [`Pallet::spend`].
       **/
      spend: AugmentedSubmittable<(amount: Compact<u128> | AnyNumber | Uint8Array, beneficiary: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u128>, MultiAddress]>;
    };
    ethereumBridge: {
      /**
       * See [`Pallet::add_next_sync_committee`].
       **/
      addNextSyncCommittee: AugmentedSubmittable<(nextSyncCommittee: PalletEth2FinalityVerifierSyncCommittee | { pubs?: any; aggr?: any } | string | Uint8Array, proof: PalletEth2FinalityVerifierMerkleProof | { gIndex?: any; witness?: any } | string | Uint8Array, proofSlot: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [PalletEth2FinalityVerifierSyncCommittee, PalletEth2FinalityVerifierMerkleProof, u64]>;
      /**
       * See [`Pallet::reset`].
       **/
      reset: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
      /**
       * See [`Pallet::submit_epoch`].
       **/
      submitEpoch: AugmentedSubmittable<(encodedUpdate: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes]>;
      /**
       * See [`Pallet::submit_epoch_debug`].
       **/
      submitEpochDebug: AugmentedSubmittable<(attestedBeaconHeader: PalletEth2FinalityVerifierBeaconBlockHeader | { slot?: any; proposerIndex?: any; parentRoot?: any; stateRoot?: any; bodyRoot?: any } | string | Uint8Array, signature: U8aFixed | string | Uint8Array, signerBits: Vec<bool> | (bool | boolean | Uint8Array)[], justifiedProof: PalletEth2FinalityVerifierMerkleProof | { gIndex?: any; witness?: any } | string | Uint8Array, executionPayload: PalletEth2FinalityVerifierExecutionPayload | { parentHash?: any; feeRecipient?: any; stateRoot?: any; receiptsRoot?: any; logsBloom?: any; prevRandao?: any; blockNumber?: any; gasLimit?: any; gasUsed?: any; timestamp?: any; extraData?: any; baseFeePerGas?: any; blockHash?: any; transactionsRoot?: any; withdrawalsRoot?: any } | string | Uint8Array, payloadProof: PalletEth2FinalityVerifierMerkleProof | { gIndex?: any; witness?: any } | string | Uint8Array, executionRange: Vec<PalletEth2FinalityVerifierExecutionHeader> | (PalletEth2FinalityVerifierExecutionHeader | { parentHash?: any; ommersHash?: any; beneficiary?: any; stateRoot?: any; transactionsRoot?: any; receiptsRoot?: any; logsBloom?: any; difficulty?: any; number?: any; gasLimit?: any; gasUsed?: any; timestamp?: any; extraData?: any; mixHash?: any; nonce?: any; baseFeePerGas?: any; withdrawalsRoot?: any } | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [PalletEth2FinalityVerifierBeaconBlockHeader, U8aFixed, Vec<bool>, PalletEth2FinalityVerifierMerkleProof, PalletEth2FinalityVerifierExecutionPayload, PalletEth2FinalityVerifierMerkleProof, Vec<PalletEth2FinalityVerifierExecutionHeader>]>;
      /**
       * See [`Pallet::submit_epoch_skipped_slot`].
       **/
      submitEpochSkippedSlot: AugmentedSubmittable<(encodedUpdate: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes]>;
      /**
       * See [`Pallet::submit_fork`].
       **/
      submitFork: AugmentedSubmittable<(encodedNewUpdate: Bytes | string | Uint8Array, encodedOldUpdate: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes, Bytes]>;
      /**
       * See [`Pallet::verify_event_inclusion`].
       **/
      verifyEventInclusion: AugmentedSubmittable<(proof: PalletEth2FinalityVerifierEthereumEventInclusionProof | { blockNumber?: any; witness?: any; index?: any; event?: any } | string | Uint8Array, speedMode: T3rnPrimitivesSpeedMode | 'Fast' | 'Rational' | 'Finalized' | number | Uint8Array, sourceAddress: Option<H160> | null | Uint8Array | H160 | string) => SubmittableExtrinsic<ApiType>, [PalletEth2FinalityVerifierEthereumEventInclusionProof, T3rnPrimitivesSpeedMode, Option<H160>]>;
      /**
       * See [`Pallet::verify_receipt_inclusion`].
       **/
      verifyReceiptInclusion: AugmentedSubmittable<(proof: PalletEth2FinalityVerifierEthereumReceiptInclusionProof | { blockNumber?: any; witness?: any; index?: any } | string | Uint8Array, speedMode: T3rnPrimitivesSpeedMode | 'Fast' | 'Rational' | 'Finalized' | number | Uint8Array) => SubmittableExtrinsic<ApiType>, [PalletEth2FinalityVerifierEthereumReceiptInclusionProof, T3rnPrimitivesSpeedMode]>;
    };
    evm: {
      /**
       * See [`Pallet::call`].
       **/
      call: AugmentedSubmittable<(source: H160 | string | Uint8Array, target: H160 | string | Uint8Array, input: Bytes | string | Uint8Array, value: U256 | AnyNumber | Uint8Array, gasLimit: u64 | AnyNumber | Uint8Array, maxFeePerGas: U256 | AnyNumber | Uint8Array, maxPriorityFeePerGas: Option<U256> | null | Uint8Array | U256 | AnyNumber, nonce: Option<U256> | null | Uint8Array | U256 | AnyNumber, accessList: Vec<ITuple<[H160, Vec<H256>]>> | ([H160 | string | Uint8Array, Vec<H256> | (H256 | string | Uint8Array)[]])[]) => SubmittableExtrinsic<ApiType>, [H160, H160, Bytes, U256, u64, U256, Option<U256>, Option<U256>, Vec<ITuple<[H160, Vec<H256>]>>]>;
      /**
       * See [`Pallet::create`].
       **/
      create: AugmentedSubmittable<(source: H160 | string | Uint8Array, init: Bytes | string | Uint8Array, value: U256 | AnyNumber | Uint8Array, gasLimit: u64 | AnyNumber | Uint8Array, maxFeePerGas: U256 | AnyNumber | Uint8Array, maxPriorityFeePerGas: Option<U256> | null | Uint8Array | U256 | AnyNumber, nonce: Option<U256> | null | Uint8Array | U256 | AnyNumber, accessList: Vec<ITuple<[H160, Vec<H256>]>> | ([H160 | string | Uint8Array, Vec<H256> | (H256 | string | Uint8Array)[]])[]) => SubmittableExtrinsic<ApiType>, [H160, Bytes, U256, u64, U256, Option<U256>, Option<U256>, Vec<ITuple<[H160, Vec<H256>]>>]>;
      /**
       * See [`Pallet::create2`].
       **/
      create2: AugmentedSubmittable<(source: H160 | string | Uint8Array, init: Bytes | string | Uint8Array, salt: H256 | string | Uint8Array, value: U256 | AnyNumber | Uint8Array, gasLimit: u64 | AnyNumber | Uint8Array, maxFeePerGas: U256 | AnyNumber | Uint8Array, maxPriorityFeePerGas: Option<U256> | null | Uint8Array | U256 | AnyNumber, nonce: Option<U256> | null | Uint8Array | U256 | AnyNumber, accessList: Vec<ITuple<[H160, Vec<H256>]>> | ([H160 | string | Uint8Array, Vec<H256> | (H256 | string | Uint8Array)[]])[]) => SubmittableExtrinsic<ApiType>, [H160, Bytes, H256, U256, u64, U256, Option<U256>, Option<U256>, Vec<ITuple<[H160, Vec<H256>]>>]>;
      /**
       * See [`Pallet::withdraw`].
       **/
      withdraw: AugmentedSubmittable<(address: H160 | string | Uint8Array, value: u128 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [H160, u128]>;
    };
    feeTreasury: {
      /**
       * See [`Pallet::approve_proposal`].
       **/
      approveProposal: AugmentedSubmittable<(proposalId: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u32>]>;
      /**
       * See [`Pallet::propose_spend`].
       **/
      proposeSpend: AugmentedSubmittable<(value: Compact<u128> | AnyNumber | Uint8Array, beneficiary: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u128>, MultiAddress]>;
      /**
       * See [`Pallet::reject_proposal`].
       **/
      rejectProposal: AugmentedSubmittable<(proposalId: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u32>]>;
      /**
       * See [`Pallet::remove_approval`].
       **/
      removeApproval: AugmentedSubmittable<(proposalId: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u32>]>;
      /**
       * See [`Pallet::spend`].
       **/
      spend: AugmentedSubmittable<(amount: Compact<u128> | AnyNumber | Uint8Array, beneficiary: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u128>, MultiAddress]>;
    };
    identity: {
      /**
       * See [`Pallet::add_registrar`].
       **/
      addRegistrar: AugmentedSubmittable<(account: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress]>;
      /**
       * See [`Pallet::add_sub`].
       **/
      addSub: AugmentedSubmittable<(sub: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, data: Data | { None: any } | { Raw: any } | { BlakeTwo256: any } | { Sha256: any } | { Keccak256: any } | { ShaThree256: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, Data]>;
      /**
       * See [`Pallet::cancel_request`].
       **/
      cancelRequest: AugmentedSubmittable<(regIndex: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * See [`Pallet::clear_identity`].
       **/
      clearIdentity: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
      /**
       * See [`Pallet::kill_identity`].
       **/
      killIdentity: AugmentedSubmittable<(target: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress]>;
      /**
       * See [`Pallet::provide_judgement`].
       **/
      provideJudgement: AugmentedSubmittable<(regIndex: Compact<u32> | AnyNumber | Uint8Array, target: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, judgement: PalletIdentityJudgement | { Unknown: any } | { FeePaid: any } | { Reasonable: any } | { KnownGood: any } | { OutOfDate: any } | { LowQuality: any } | { Erroneous: any } | string | Uint8Array, identity: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u32>, MultiAddress, PalletIdentityJudgement, H256]>;
      /**
       * See [`Pallet::quit_sub`].
       **/
      quitSub: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
      /**
       * See [`Pallet::remove_sub`].
       **/
      removeSub: AugmentedSubmittable<(sub: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress]>;
      /**
       * See [`Pallet::rename_sub`].
       **/
      renameSub: AugmentedSubmittable<(sub: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, data: Data | { None: any } | { Raw: any } | { BlakeTwo256: any } | { Sha256: any } | { Keccak256: any } | { ShaThree256: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, Data]>;
      /**
       * See [`Pallet::request_judgement`].
       **/
      requestJudgement: AugmentedSubmittable<(regIndex: Compact<u32> | AnyNumber | Uint8Array, maxFee: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u32>, Compact<u128>]>;
      /**
       * See [`Pallet::set_account_id`].
       **/
      setAccountId: AugmentedSubmittable<(index: Compact<u32> | AnyNumber | Uint8Array, updated: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u32>, MultiAddress]>;
      /**
       * See [`Pallet::set_fee`].
       **/
      setFee: AugmentedSubmittable<(index: Compact<u32> | AnyNumber | Uint8Array, fee: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u32>, Compact<u128>]>;
      /**
       * See [`Pallet::set_fields`].
       **/
      setFields: AugmentedSubmittable<(index: Compact<u32> | AnyNumber | Uint8Array, fields: PalletIdentityBitFlags) => SubmittableExtrinsic<ApiType>, [Compact<u32>, PalletIdentityBitFlags]>;
      /**
       * See [`Pallet::set_identity`].
       **/
      setIdentity: AugmentedSubmittable<(info: PalletIdentityIdentityInfo | { additional?: any; display?: any; legal?: any; web?: any; riot?: any; email?: any; pgpFingerprint?: any; image?: any; twitter?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [PalletIdentityIdentityInfo]>;
      /**
       * See [`Pallet::set_subs`].
       **/
      setSubs: AugmentedSubmittable<(subs: Vec<ITuple<[AccountId32, Data]>> | ([AccountId32 | string | Uint8Array, Data | { None: any } | { Raw: any } | { BlakeTwo256: any } | { Sha256: any } | { Keccak256: any } | { ShaThree256: any } | string | Uint8Array])[]) => SubmittableExtrinsic<ApiType>, [Vec<ITuple<[AccountId32, Data]>>]>;
    };
    kusamaBridge: {
      /**
       * See [`Pallet::reset`].
       **/
      reset: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
      /**
       * See [`Pallet::submit_headers`].
       **/
      submitHeaders: AugmentedSubmittable<(range: Vec<SpRuntimeHeader> | (SpRuntimeHeader | { parentHash?: any; number?: any; stateRoot?: any; extrinsicsRoot?: any; digest?: any } | string | Uint8Array)[], signedHeader: SpRuntimeHeader | { parentHash?: any; number?: any; stateRoot?: any; extrinsicsRoot?: any; digest?: any } | string | Uint8Array, justification: PalletGrandpaFinalityVerifierBridgesHeaderChainJustificationGrandpaJustification | { round?: any; commit?: any; votesAncestries?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Vec<SpRuntimeHeader>, SpRuntimeHeader, PalletGrandpaFinalityVerifierBridgesHeaderChainJustificationGrandpaJustification]>;
    };
    maintenance: {
      /**
       * See [`Pallet::enter_maintenance_mode`].
       **/
      enterMaintenanceMode: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
      /**
       * See [`Pallet::resume_normal_operation`].
       **/
      resumeNormalOperation: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
    };
    parachainInfo: {
    };
    parachainSystem: {
      /**
       * See [`Pallet::authorize_upgrade`].
       **/
      authorizeUpgrade: AugmentedSubmittable<(codeHash: H256 | string | Uint8Array, checkVersion: bool | boolean | Uint8Array) => SubmittableExtrinsic<ApiType>, [H256, bool]>;
      /**
       * See [`Pallet::enact_authorized_upgrade`].
       **/
      enactAuthorizedUpgrade: AugmentedSubmittable<(code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes]>;
      /**
       * See [`Pallet::set_validation_data`].
       **/
      setValidationData: AugmentedSubmittable<(data: CumulusPrimitivesParachainInherentParachainInherentData | { validationData?: any; relayChainState?: any; downwardMessages?: any; horizontalMessages?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [CumulusPrimitivesParachainInherentParachainInherentData]>;
      /**
       * See [`Pallet::sudo_send_upward_message`].
       **/
      sudoSendUpwardMessage: AugmentedSubmittable<(message: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes]>;
    };
    parachainTreasury: {
      /**
       * See [`Pallet::approve_proposal`].
       **/
      approveProposal: AugmentedSubmittable<(proposalId: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u32>]>;
      /**
       * See [`Pallet::propose_spend`].
       **/
      proposeSpend: AugmentedSubmittable<(value: Compact<u128> | AnyNumber | Uint8Array, beneficiary: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u128>, MultiAddress]>;
      /**
       * See [`Pallet::reject_proposal`].
       **/
      rejectProposal: AugmentedSubmittable<(proposalId: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u32>]>;
      /**
       * See [`Pallet::remove_approval`].
       **/
      removeApproval: AugmentedSubmittable<(proposalId: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u32>]>;
      /**
       * See [`Pallet::spend`].
       **/
      spend: AugmentedSubmittable<(amount: Compact<u128> | AnyNumber | Uint8Array, beneficiary: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u128>, MultiAddress]>;
    };
    polkadotBridge: {
      /**
       * See [`Pallet::reset`].
       **/
      reset: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
      /**
       * See [`Pallet::submit_headers`].
       **/
      submitHeaders: AugmentedSubmittable<(range: Vec<SpRuntimeHeader> | (SpRuntimeHeader | { parentHash?: any; number?: any; stateRoot?: any; extrinsicsRoot?: any; digest?: any } | string | Uint8Array)[], signedHeader: SpRuntimeHeader | { parentHash?: any; number?: any; stateRoot?: any; extrinsicsRoot?: any; digest?: any } | string | Uint8Array, justification: PalletGrandpaFinalityVerifierBridgesHeaderChainJustificationGrandpaJustification | { round?: any; commit?: any; votesAncestries?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Vec<SpRuntimeHeader>, SpRuntimeHeader, PalletGrandpaFinalityVerifierBridgesHeaderChainJustificationGrandpaJustification]>;
    };
    polkadotXcm: {
      /**
       * See [`Pallet::execute`].
       **/
      execute: AugmentedSubmittable<(message: XcmVersionedXcm | { V2: any } | { V3: any } | string | Uint8Array, maxWeight: SpWeightsWeightV2Weight | { refTime?: any; proofSize?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [XcmVersionedXcm, SpWeightsWeightV2Weight]>;
      /**
       * See [`Pallet::force_default_xcm_version`].
       **/
      forceDefaultXcmVersion: AugmentedSubmittable<(maybeXcmVersion: Option<u32> | null | Uint8Array | u32 | AnyNumber) => SubmittableExtrinsic<ApiType>, [Option<u32>]>;
      /**
       * See [`Pallet::force_subscribe_version_notify`].
       **/
      forceSubscribeVersionNotify: AugmentedSubmittable<(location: XcmVersionedMultiLocation | { V2: any } | { V3: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [XcmVersionedMultiLocation]>;
      /**
       * See [`Pallet::force_suspension`].
       **/
      forceSuspension: AugmentedSubmittable<(suspended: bool | boolean | Uint8Array) => SubmittableExtrinsic<ApiType>, [bool]>;
      /**
       * See [`Pallet::force_unsubscribe_version_notify`].
       **/
      forceUnsubscribeVersionNotify: AugmentedSubmittable<(location: XcmVersionedMultiLocation | { V2: any } | { V3: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [XcmVersionedMultiLocation]>;
      /**
       * See [`Pallet::force_xcm_version`].
       **/
      forceXcmVersion: AugmentedSubmittable<(location: XcmV3MultiLocation | { parents?: any; interior?: any } | string | Uint8Array, version: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [XcmV3MultiLocation, u32]>;
      /**
       * See [`Pallet::limited_reserve_transfer_assets`].
       **/
      limitedReserveTransferAssets: AugmentedSubmittable<(dest: XcmVersionedMultiLocation | { V2: any } | { V3: any } | string | Uint8Array, beneficiary: XcmVersionedMultiLocation | { V2: any } | { V3: any } | string | Uint8Array, assets: XcmVersionedMultiAssets | { V2: any } | { V3: any } | string | Uint8Array, feeAssetItem: u32 | AnyNumber | Uint8Array, weightLimit: XcmV3WeightLimit | { Unlimited: any } | { Limited: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [XcmVersionedMultiLocation, XcmVersionedMultiLocation, XcmVersionedMultiAssets, u32, XcmV3WeightLimit]>;
      /**
       * See [`Pallet::limited_teleport_assets`].
       **/
      limitedTeleportAssets: AugmentedSubmittable<(dest: XcmVersionedMultiLocation | { V2: any } | { V3: any } | string | Uint8Array, beneficiary: XcmVersionedMultiLocation | { V2: any } | { V3: any } | string | Uint8Array, assets: XcmVersionedMultiAssets | { V2: any } | { V3: any } | string | Uint8Array, feeAssetItem: u32 | AnyNumber | Uint8Array, weightLimit: XcmV3WeightLimit | { Unlimited: any } | { Limited: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [XcmVersionedMultiLocation, XcmVersionedMultiLocation, XcmVersionedMultiAssets, u32, XcmV3WeightLimit]>;
      /**
       * See [`Pallet::reserve_transfer_assets`].
       **/
      reserveTransferAssets: AugmentedSubmittable<(dest: XcmVersionedMultiLocation | { V2: any } | { V3: any } | string | Uint8Array, beneficiary: XcmVersionedMultiLocation | { V2: any } | { V3: any } | string | Uint8Array, assets: XcmVersionedMultiAssets | { V2: any } | { V3: any } | string | Uint8Array, feeAssetItem: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [XcmVersionedMultiLocation, XcmVersionedMultiLocation, XcmVersionedMultiAssets, u32]>;
      /**
       * See [`Pallet::send`].
       **/
      send: AugmentedSubmittable<(dest: XcmVersionedMultiLocation | { V2: any } | { V3: any } | string | Uint8Array, message: XcmVersionedXcm | { V2: any } | { V3: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [XcmVersionedMultiLocation, XcmVersionedXcm]>;
      /**
       * See [`Pallet::teleport_assets`].
       **/
      teleportAssets: AugmentedSubmittable<(dest: XcmVersionedMultiLocation | { V2: any } | { V3: any } | string | Uint8Array, beneficiary: XcmVersionedMultiLocation | { V2: any } | { V3: any } | string | Uint8Array, assets: XcmVersionedMultiAssets | { V2: any } | { V3: any } | string | Uint8Array, feeAssetItem: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [XcmVersionedMultiLocation, XcmVersionedMultiLocation, XcmVersionedMultiAssets, u32]>;
    };
    portal: {
      /**
       * See [`Pallet::register_gateway`].
       **/
      registerGateway: AugmentedSubmittable<(gatewayId: U8aFixed | string | Uint8Array, tokenId: u32 | AnyNumber | Uint8Array, verificationVendor: T3rnPrimitivesGatewayVendor | 'Polkadot' | 'Kusama' | 'Rococo' | 'Ethereum' | 'Sepolia' | 'XBI' | number | Uint8Array, executionVendor: T3rnPrimitivesExecutionVendor | 'Substrate' | 'EVM' | number | Uint8Array, codec: T3rnAbiRecodeCodec | 'Scale' | 'Rlp' | number | Uint8Array, registrant: Option<AccountId32> | null | Uint8Array | AccountId32 | string, escrowAccount: Option<AccountId32> | null | Uint8Array | AccountId32 | string, allowedSideEffects: Vec<ITuple<[U8aFixed, Option<u8>]>> | ([U8aFixed | string | Uint8Array, Option<u8> | null | Uint8Array | u8 | AnyNumber])[], tokenProps: T3rnPrimitivesTokenInfo | { Substrate: any } | { Ethereum: any } | string | Uint8Array, encodedRegistrationData: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [U8aFixed, u32, T3rnPrimitivesGatewayVendor, T3rnPrimitivesExecutionVendor, T3rnAbiRecodeCodec, Option<AccountId32>, Option<AccountId32>, Vec<ITuple<[U8aFixed, Option<u8>]>>, T3rnPrimitivesTokenInfo, Bytes]>;
    };
    preimage: {
      /**
       * See [`Pallet::note_preimage`].
       **/
      notePreimage: AugmentedSubmittable<(bytes: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes]>;
      /**
       * See [`Pallet::request_preimage`].
       **/
      requestPreimage: AugmentedSubmittable<(hash: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [H256]>;
      /**
       * See [`Pallet::unnote_preimage`].
       **/
      unnotePreimage: AugmentedSubmittable<(hash: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [H256]>;
      /**
       * See [`Pallet::unrequest_preimage`].
       **/
      unrequestPreimage: AugmentedSubmittable<(hash: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [H256]>;
    };
    rewards: {
      /**
       * See [`Pallet::claim`].
       **/
      claim: AugmentedSubmittable<(roleToClaim: Option<T3rnPrimitivesClaimableCircuitRole> | null | Uint8Array | T3rnPrimitivesClaimableCircuitRole | 'Ambassador' | 'Executor' | 'Attester' | 'Staker' | 'Collator' | 'ContractAuthor' | 'Relayer' | 'Requester' | 'Local' | number) => SubmittableExtrinsic<ApiType>, [Option<T3rnPrimitivesClaimableCircuitRole>]>;
      /**
       * See [`Pallet::set_max_rewards_executors_kickback`].
       **/
      setMaxRewardsExecutorsKickback: AugmentedSubmittable<(newKickback: Percent | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Percent]>;
      /**
       * See [`Pallet::trigger_distribution`].
       **/
      triggerDistribution: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
      /**
       * See [`Pallet::turn_on_off_claims`].
       **/
      turnOnOffClaims: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
      /**
       * See [`Pallet::turn_on_off_distribution`].
       **/
      turnOnOffDistribution: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
      /**
       * See [`Pallet::turn_on_off_settlement_accumulation`].
       **/
      turnOnOffSettlementAccumulation: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
    };
    rococoBridge: {
      /**
       * See [`Pallet::reset`].
       **/
      reset: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
      /**
       * See [`Pallet::submit_headers`].
       **/
      submitHeaders: AugmentedSubmittable<(range: Vec<SpRuntimeHeader> | (SpRuntimeHeader | { parentHash?: any; number?: any; stateRoot?: any; extrinsicsRoot?: any; digest?: any } | string | Uint8Array)[], signedHeader: SpRuntimeHeader | { parentHash?: any; number?: any; stateRoot?: any; extrinsicsRoot?: any; digest?: any } | string | Uint8Array, justification: PalletGrandpaFinalityVerifierBridgesHeaderChainJustificationGrandpaJustification | { round?: any; commit?: any; votesAncestries?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Vec<SpRuntimeHeader>, SpRuntimeHeader, PalletGrandpaFinalityVerifierBridgesHeaderChainJustificationGrandpaJustification]>;
    };
    scheduler: {
      /**
       * See [`Pallet::cancel`].
       **/
      cancel: AugmentedSubmittable<(when: u32 | AnyNumber | Uint8Array, index: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32]>;
      /**
       * See [`Pallet::cancel_named`].
       **/
      cancelNamed: AugmentedSubmittable<(id: U8aFixed | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [U8aFixed]>;
      /**
       * See [`Pallet::schedule`].
       **/
      schedule: AugmentedSubmittable<(when: u32 | AnyNumber | Uint8Array, maybePeriodic: Option<ITuple<[u32, u32]>> | null | Uint8Array | ITuple<[u32, u32]> | [u32 | AnyNumber | Uint8Array, u32 | AnyNumber | Uint8Array], priority: u8 | AnyNumber | Uint8Array, call: Call | IMethod | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, Option<ITuple<[u32, u32]>>, u8, Call]>;
      /**
       * See [`Pallet::schedule_after`].
       **/
      scheduleAfter: AugmentedSubmittable<(after: u32 | AnyNumber | Uint8Array, maybePeriodic: Option<ITuple<[u32, u32]>> | null | Uint8Array | ITuple<[u32, u32]> | [u32 | AnyNumber | Uint8Array, u32 | AnyNumber | Uint8Array], priority: u8 | AnyNumber | Uint8Array, call: Call | IMethod | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, Option<ITuple<[u32, u32]>>, u8, Call]>;
      /**
       * See [`Pallet::schedule_named`].
       **/
      scheduleNamed: AugmentedSubmittable<(id: U8aFixed | string | Uint8Array, when: u32 | AnyNumber | Uint8Array, maybePeriodic: Option<ITuple<[u32, u32]>> | null | Uint8Array | ITuple<[u32, u32]> | [u32 | AnyNumber | Uint8Array, u32 | AnyNumber | Uint8Array], priority: u8 | AnyNumber | Uint8Array, call: Call | IMethod | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [U8aFixed, u32, Option<ITuple<[u32, u32]>>, u8, Call]>;
      /**
       * See [`Pallet::schedule_named_after`].
       **/
      scheduleNamedAfter: AugmentedSubmittable<(id: U8aFixed | string | Uint8Array, after: u32 | AnyNumber | Uint8Array, maybePeriodic: Option<ITuple<[u32, u32]>> | null | Uint8Array | ITuple<[u32, u32]> | [u32 | AnyNumber | Uint8Array, u32 | AnyNumber | Uint8Array], priority: u8 | AnyNumber | Uint8Array, call: Call | IMethod | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [U8aFixed, u32, Option<ITuple<[u32, u32]>>, u8, Call]>;
    };
    sepoliaBridge: {
      /**
       * See [`Pallet::add_next_sync_committee`].
       **/
      addNextSyncCommittee: AugmentedSubmittable<(nextSyncCommittee: PalletSepoliaFinalityVerifierSyncCommittee | { pubs?: any; aggr?: any } | string | Uint8Array, proof: PalletSepoliaFinalityVerifierMerkleProof | { gIndex?: any; witness?: any } | string | Uint8Array, proofSlot: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [PalletSepoliaFinalityVerifierSyncCommittee, PalletSepoliaFinalityVerifierMerkleProof, u64]>;
      /**
       * See [`Pallet::reset`].
       **/
      reset: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
      /**
       * See [`Pallet::submit_epoch`].
       **/
      submitEpoch: AugmentedSubmittable<(encodedUpdate: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes]>;
      /**
       * See [`Pallet::submit_epoch_debug`].
       **/
      submitEpochDebug: AugmentedSubmittable<(attestedBeaconHeader: PalletSepoliaFinalityVerifierBeaconBlockHeader | { slot?: any; proposerIndex?: any; parentRoot?: any; stateRoot?: any; bodyRoot?: any } | string | Uint8Array, signature: U8aFixed | string | Uint8Array, signerBits: Vec<bool> | (bool | boolean | Uint8Array)[], justifiedProof: PalletSepoliaFinalityVerifierMerkleProof | { gIndex?: any; witness?: any } | string | Uint8Array, executionPayload: PalletSepoliaFinalityVerifierExecutionPayload | { parentHash?: any; feeRecipient?: any; stateRoot?: any; receiptsRoot?: any; logsBloom?: any; prevRandao?: any; blockNumber?: any; gasLimit?: any; gasUsed?: any; timestamp?: any; extraData?: any; baseFeePerGas?: any; blockHash?: any; transactionsRoot?: any; withdrawalsRoot?: any } | string | Uint8Array, payloadProof: PalletSepoliaFinalityVerifierMerkleProof | { gIndex?: any; witness?: any } | string | Uint8Array, executionRange: Vec<PalletSepoliaFinalityVerifierExecutionHeader> | (PalletSepoliaFinalityVerifierExecutionHeader | { parentHash?: any; ommersHash?: any; beneficiary?: any; stateRoot?: any; transactionsRoot?: any; receiptsRoot?: any; logsBloom?: any; difficulty?: any; number?: any; gasLimit?: any; gasUsed?: any; timestamp?: any; extraData?: any; mixHash?: any; nonce?: any; baseFeePerGas?: any; withdrawalsRoot?: any } | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [PalletSepoliaFinalityVerifierBeaconBlockHeader, U8aFixed, Vec<bool>, PalletSepoliaFinalityVerifierMerkleProof, PalletSepoliaFinalityVerifierExecutionPayload, PalletSepoliaFinalityVerifierMerkleProof, Vec<PalletSepoliaFinalityVerifierExecutionHeader>]>;
      /**
       * See [`Pallet::submit_epoch_skipped_slot`].
       **/
      submitEpochSkippedSlot: AugmentedSubmittable<(encodedUpdate: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes]>;
      /**
       * See [`Pallet::submit_fork`].
       **/
      submitFork: AugmentedSubmittable<(encodedNewUpdate: Bytes | string | Uint8Array, encodedOldUpdate: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes, Bytes]>;
      /**
       * See [`Pallet::verify_event_inclusion`].
       **/
      verifyEventInclusion: AugmentedSubmittable<(proof: PalletSepoliaFinalityVerifierEthereumEventInclusionProof | { blockNumber?: any; witness?: any; index?: any; event?: any } | string | Uint8Array, speedMode: T3rnPrimitivesSpeedMode | 'Fast' | 'Rational' | 'Finalized' | number | Uint8Array, sourceAddress: Option<H160> | null | Uint8Array | H160 | string) => SubmittableExtrinsic<ApiType>, [PalletSepoliaFinalityVerifierEthereumEventInclusionProof, T3rnPrimitivesSpeedMode, Option<H160>]>;
      /**
       * See [`Pallet::verify_receipt_inclusion`].
       **/
      verifyReceiptInclusion: AugmentedSubmittable<(proof: PalletSepoliaFinalityVerifierEthereumReceiptInclusionProof | { blockNumber?: any; witness?: any; index?: any } | string | Uint8Array, speedMode: T3rnPrimitivesSpeedMode | 'Fast' | 'Rational' | 'Finalized' | number | Uint8Array) => SubmittableExtrinsic<ApiType>, [PalletSepoliaFinalityVerifierEthereumReceiptInclusionProof, T3rnPrimitivesSpeedMode]>;
    };
    session: {
      /**
       * See [`Pallet::purge_keys`].
       **/
      purgeKeys: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
      /**
       * See [`Pallet::set_keys`].
       **/
      setKeys: AugmentedSubmittable<(keys: T0rnParachainRuntimeParachainConfigSessionKeys | { aura?: any } | string | Uint8Array, proof: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [T0rnParachainRuntimeParachainConfigSessionKeys, Bytes]>;
    };
    slashTreasury: {
      /**
       * See [`Pallet::approve_proposal`].
       **/
      approveProposal: AugmentedSubmittable<(proposalId: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u32>]>;
      /**
       * See [`Pallet::propose_spend`].
       **/
      proposeSpend: AugmentedSubmittable<(value: Compact<u128> | AnyNumber | Uint8Array, beneficiary: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u128>, MultiAddress]>;
      /**
       * See [`Pallet::reject_proposal`].
       **/
      rejectProposal: AugmentedSubmittable<(proposalId: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u32>]>;
      /**
       * See [`Pallet::remove_approval`].
       **/
      removeApproval: AugmentedSubmittable<(proposalId: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u32>]>;
      /**
       * See [`Pallet::spend`].
       **/
      spend: AugmentedSubmittable<(amount: Compact<u128> | AnyNumber | Uint8Array, beneficiary: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u128>, MultiAddress]>;
    };
    sudo: {
      /**
       * See [`Pallet::set_key`].
       **/
      setKey: AugmentedSubmittable<(updated: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress]>;
      /**
       * See [`Pallet::sudo`].
       **/
      sudo: AugmentedSubmittable<(call: Call | IMethod | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Call]>;
      /**
       * See [`Pallet::sudo_as`].
       **/
      sudoAs: AugmentedSubmittable<(who: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, call: Call | IMethod | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, Call]>;
      /**
       * See [`Pallet::sudo_unchecked_weight`].
       **/
      sudoUncheckedWeight: AugmentedSubmittable<(call: Call | IMethod | string | Uint8Array, weight: SpWeightsWeightV2Weight | { refTime?: any; proofSize?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Call, SpWeightsWeightV2Weight]>;
    };
    system: {
      /**
       * See [`Pallet::kill_prefix`].
       **/
      killPrefix: AugmentedSubmittable<(prefix: Bytes | string | Uint8Array, subkeys: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes, u32]>;
      /**
       * See [`Pallet::kill_storage`].
       **/
      killStorage: AugmentedSubmittable<(keys: Vec<Bytes> | (Bytes | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [Vec<Bytes>]>;
      /**
       * See [`Pallet::remark`].
       **/
      remark: AugmentedSubmittable<(remark: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes]>;
      /**
       * See [`Pallet::remark_with_event`].
       **/
      remarkWithEvent: AugmentedSubmittable<(remark: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes]>;
      /**
       * See [`Pallet::set_code`].
       **/
      setCode: AugmentedSubmittable<(code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes]>;
      /**
       * See [`Pallet::set_code_without_checks`].
       **/
      setCodeWithoutChecks: AugmentedSubmittable<(code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes]>;
      /**
       * See [`Pallet::set_heap_pages`].
       **/
      setHeapPages: AugmentedSubmittable<(pages: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u64]>;
      /**
       * See [`Pallet::set_storage`].
       **/
      setStorage: AugmentedSubmittable<(items: Vec<ITuple<[Bytes, Bytes]>> | ([Bytes | string | Uint8Array, Bytes | string | Uint8Array])[]) => SubmittableExtrinsic<ApiType>, [Vec<ITuple<[Bytes, Bytes]>>]>;
    };
    threeVm: {
    };
    timestamp: {
      /**
       * See [`Pallet::set`].
       **/
      set: AugmentedSubmittable<(now: Compact<u64> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u64>]>;
    };
    treasury: {
      /**
       * See [`Pallet::approve_proposal`].
       **/
      approveProposal: AugmentedSubmittable<(proposalId: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u32>]>;
      /**
       * See [`Pallet::propose_spend`].
       **/
      proposeSpend: AugmentedSubmittable<(value: Compact<u128> | AnyNumber | Uint8Array, beneficiary: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u128>, MultiAddress]>;
      /**
       * See [`Pallet::reject_proposal`].
       **/
      rejectProposal: AugmentedSubmittable<(proposalId: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u32>]>;
      /**
       * See [`Pallet::remove_approval`].
       **/
      removeApproval: AugmentedSubmittable<(proposalId: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u32>]>;
      /**
       * See [`Pallet::spend`].
       **/
      spend: AugmentedSubmittable<(amount: Compact<u128> | AnyNumber | Uint8Array, beneficiary: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u128>, MultiAddress]>;
    };
    utility: {
      /**
       * See [`Pallet::as_derivative`].
       **/
      asDerivative: AugmentedSubmittable<(index: u16 | AnyNumber | Uint8Array, call: Call | IMethod | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u16, Call]>;
      /**
       * See [`Pallet::batch`].
       **/
      batch: AugmentedSubmittable<(calls: Vec<Call> | (Call | IMethod | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [Vec<Call>]>;
      /**
       * See [`Pallet::batch_all`].
       **/
      batchAll: AugmentedSubmittable<(calls: Vec<Call> | (Call | IMethod | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [Vec<Call>]>;
      /**
       * See [`Pallet::dispatch_as`].
       **/
      dispatchAs: AugmentedSubmittable<(asOrigin: T0rnParachainRuntimeOriginCaller | { system: any } | { Void: any } | { PolkadotXcm: any } | { CumulusXcm: any } | string | Uint8Array, call: Call | IMethod | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [T0rnParachainRuntimeOriginCaller, Call]>;
      /**
       * See [`Pallet::force_batch`].
       **/
      forceBatch: AugmentedSubmittable<(calls: Vec<Call> | (Call | IMethod | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [Vec<Call>]>;
      /**
       * See [`Pallet::with_weight`].
       **/
      withWeight: AugmentedSubmittable<(call: Call | IMethod | string | Uint8Array, weight: SpWeightsWeightV2Weight | { refTime?: any; proofSize?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Call, SpWeightsWeightV2Weight]>;
    };
    vacuum: {
      /**
       * See [`Pallet::order`].
       **/
      order: AugmentedSubmittable<(sfxActions: Vec<T3rnPrimitivesCircuitTypesOrderSFX> | (T3rnPrimitivesCircuitTypesOrderSFX | { sfxAction?: any; maxReward?: any; rewardAsset?: any; insurance?: any; remoteOriginNonce?: any } | string | Uint8Array)[], speedMode: T3rnPrimitivesSpeedMode | 'Fast' | 'Rational' | 'Finalized' | number | Uint8Array) => SubmittableExtrinsic<ApiType>, [Vec<T3rnPrimitivesCircuitTypesOrderSFX>, T3rnPrimitivesSpeedMode]>;
      /**
       * See [`Pallet::read_order_status`].
       **/
      readOrderStatus: AugmentedSubmittable<(xtxId: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [H256]>;
    };
    xcmpQueue: {
      /**
       * See [`Pallet::resume_xcm_execution`].
       **/
      resumeXcmExecution: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
      /**
       * See [`Pallet::service_overweight`].
       **/
      serviceOverweight: AugmentedSubmittable<(index: u64 | AnyNumber | Uint8Array, weightLimit: SpWeightsWeightV2Weight | { refTime?: any; proofSize?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u64, SpWeightsWeightV2Weight]>;
      /**
       * See [`Pallet::suspend_xcm_execution`].
       **/
      suspendXcmExecution: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
      /**
       * See [`Pallet::update_drop_threshold`].
       **/
      updateDropThreshold: AugmentedSubmittable<(updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * See [`Pallet::update_resume_threshold`].
       **/
      updateResumeThreshold: AugmentedSubmittable<(updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * See [`Pallet::update_suspend_threshold`].
       **/
      updateSuspendThreshold: AugmentedSubmittable<(updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * See [`Pallet::update_threshold_weight`].
       **/
      updateThresholdWeight: AugmentedSubmittable<(updated: SpWeightsWeightV2Weight | { refTime?: any; proofSize?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [SpWeightsWeightV2Weight]>;
      /**
       * See [`Pallet::update_weight_restrict_decay`].
       **/
      updateWeightRestrictDecay: AugmentedSubmittable<(updated: SpWeightsWeightV2Weight | { refTime?: any; proofSize?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [SpWeightsWeightV2Weight]>;
      /**
       * See [`Pallet::update_xcmp_max_individual_weight`].
       **/
      updateXcmpMaxIndividualWeight: AugmentedSubmittable<(updated: SpWeightsWeightV2Weight | { refTime?: any; proofSize?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [SpWeightsWeightV2Weight]>;
    };
    xdns: {
      /**
       * See [`Pallet::purge_gateway_record`].
       **/
      purgeGatewayRecord: AugmentedSubmittable<(requester: AccountId32 | string | Uint8Array, gatewayId: U8aFixed | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [AccountId32, U8aFixed]>;
      /**
       * See [`Pallet::purge_token_record`].
       **/
      purgeTokenRecord: AugmentedSubmittable<(tokenId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * See [`Pallet::reboot_self_gateway`].
       **/
      rebootSelfGateway: AugmentedSubmittable<(vendor: T3rnPrimitivesGatewayVendor | 'Polkadot' | 'Kusama' | 'Rococo' | 'Ethereum' | 'Sepolia' | 'XBI' | number | Uint8Array) => SubmittableExtrinsic<ApiType>, [T3rnPrimitivesGatewayVendor]>;
      /**
       * See [`Pallet::unlink_token`].
       **/
      unlinkToken: AugmentedSubmittable<(gatewayId: U8aFixed | string | Uint8Array, tokenId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [U8aFixed, u32]>;
    };
  } // AugmentedSubmittables
} // declare module
