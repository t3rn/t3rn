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
import type { CircuitStandaloneRuntimeOriginCaller, PalletContractsWasmDeterminism, PalletEth2FinalityVerifierBeaconBlockHeader, PalletEth2FinalityVerifierEthereumEventInclusionProof, PalletEth2FinalityVerifierEthereumReceiptInclusionProof, PalletEth2FinalityVerifierExecutionHeader, PalletEth2FinalityVerifierExecutionPayload, PalletEth2FinalityVerifierMerkleProof, PalletEth2FinalityVerifierSyncCommittee, PalletGrandpaFinalityVerifierBridgesHeaderChainJustificationGrandpaJustification, PalletIdentityBitFlags, PalletIdentityIdentityInfo, PalletIdentityJudgement, PalletSepoliaFinalityVerifierBeaconBlockHeader, PalletSepoliaFinalityVerifierEpochUpdate, PalletSepoliaFinalityVerifierEpochUpdateSkippedSlot, PalletSepoliaFinalityVerifierEthereumEventInclusionProof, PalletSepoliaFinalityVerifierEthereumReceiptInclusionProof, PalletSepoliaFinalityVerifierExecutionHeader, PalletSepoliaFinalityVerifierExecutionPayload, PalletSepoliaFinalityVerifierMerkleProof, PalletSepoliaFinalityVerifierSyncCommittee, SpConsensusGrandpaEquivocationProof, SpCoreVoid, SpRuntimeHeader, SpWeightsWeightV2Weight, T3rnAbiRecodeCodec, T3rnAbiSfxAbi, T3rnPrimitivesAccountManagerOutcome, T3rnPrimitivesCircuitTypesOrderSFX, T3rnPrimitivesClaimableBenefitSource, T3rnPrimitivesClaimableCircuitRole, T3rnPrimitivesContractsRegistryRegistryContract, T3rnPrimitivesExecutionVendor, T3rnPrimitivesGatewayVendor, T3rnPrimitivesSpeedMode, T3rnPrimitivesTokenInfo, T3rnPrimitivesXdnsTopology, T3rnTypesSfxConfirmedSideEffect, T3rnTypesSfxSecurityLvl, T3rnTypesSfxSideEffect, XcmV3MultiLocation } from '@polkadot/types/lookup';

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
       * See [`Pallet::register_invulnerable_attester`].
       **/
      registerInvulnerableAttester: AugmentedSubmittable<(selfNominateAmount: u128 | AnyNumber | Uint8Array, ecdsaKey: U8aFixed | string | Uint8Array, ed25519Key: U8aFixed | string | Uint8Array, sr25519Key: U8aFixed | string | Uint8Array, customCommission: Option<Percent> | null | Uint8Array | Percent | AnyNumber) => SubmittableExtrinsic<ApiType>, [u128, U8aFixed, U8aFixed, U8aFixed, Option<Percent>]>;
      /**
       * See [`Pallet::remove_attestation_target`].
       **/
      removeAttestationTarget: AugmentedSubmittable<(target: U8aFixed | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [U8aFixed]>;
      /**
       * See [`Pallet::submit_attestation`].
       **/
      submitAttestation: AugmentedSubmittable<(message: H256 | string | Uint8Array, signature: Bytes | string | Uint8Array, target: U8aFixed | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [H256, Bytes, U8aFixed]>;
      /**
       * See [`Pallet::submit_for_influx_attestation`].
       **/
      submitForInfluxAttestation: AugmentedSubmittable<(message: H256 | string | Uint8Array, messageHash: H256 | string | Uint8Array, heightThere: u32 | AnyNumber | Uint8Array, target: U8aFixed | string | Uint8Array, signature: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [H256, H256, u32, U8aFixed, Bytes]>;
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
      onExtrinsicTrigger: AugmentedSubmittable<(sideEffects: Vec<T3rnTypesSfxSideEffect> | (T3rnTypesSfxSideEffect | { target?: any; maxReward?: any; insurance?: any; action?: any; encodedArgs?: any; signature?: any; enforceExecutor?: any; rewardAssetId?: any } | string | Uint8Array)[], speedMode: T3rnPrimitivesSpeedMode | 'Fast' | 'Rational' | 'Finalized' | 'Instant' | number | Uint8Array, preferredSecurityLevel: T3rnTypesSfxSecurityLvl | 'Optimistic' | 'Escrow' | number | Uint8Array) => SubmittableExtrinsic<ApiType>, [Vec<T3rnTypesSfxSideEffect>, T3rnPrimitivesSpeedMode, T3rnTypesSfxSecurityLvl]>;
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
      onRemoteOriginTrigger: AugmentedSubmittable<(orderOrigin: AccountId32 | string | Uint8Array, sideEffects: Vec<T3rnTypesSfxSideEffect> | (T3rnTypesSfxSideEffect | { target?: any; maxReward?: any; insurance?: any; action?: any; encodedArgs?: any; signature?: any; enforceExecutor?: any; rewardAssetId?: any } | string | Uint8Array)[], speedMode: T3rnPrimitivesSpeedMode | 'Fast' | 'Rational' | 'Finalized' | 'Instant' | number | Uint8Array) => SubmittableExtrinsic<ApiType>, [AccountId32, Vec<T3rnTypesSfxSideEffect>, T3rnPrimitivesSpeedMode]>;
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
       * See [`Pallet::set_owner`].
       **/
      setOwner: AugmentedSubmittable<(owner: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [AccountId32]>;
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
       * See [`Pallet::submit_epoch_skipped_slot_debug`].
       **/
      submitEpochSkippedSlotDebug: AugmentedSubmittable<(beaconHeaders: Vec<PalletEth2FinalityVerifierBeaconBlockHeader> | (PalletEth2FinalityVerifierBeaconBlockHeader | { slot?: any; proposerIndex?: any; parentRoot?: any; stateRoot?: any; bodyRoot?: any } | string | Uint8Array)[], signature: U8aFixed | string | Uint8Array, signerBits: Vec<bool> | (bool | boolean | Uint8Array)[], justifiedProof: PalletEth2FinalityVerifierMerkleProof | { gIndex?: any; witness?: any } | string | Uint8Array, executionPayload: PalletEth2FinalityVerifierExecutionPayload | { parentHash?: any; feeRecipient?: any; stateRoot?: any; receiptsRoot?: any; logsBloom?: any; prevRandao?: any; blockNumber?: any; gasLimit?: any; gasUsed?: any; timestamp?: any; extraData?: any; baseFeePerGas?: any; blockHash?: any; transactionsRoot?: any; withdrawalsRoot?: any } | string | Uint8Array, payloadProof: PalletEth2FinalityVerifierMerkleProof | { gIndex?: any; witness?: any } | string | Uint8Array, executionRange: Vec<PalletEth2FinalityVerifierExecutionHeader> | (PalletEth2FinalityVerifierExecutionHeader | { parentHash?: any; ommersHash?: any; beneficiary?: any; stateRoot?: any; transactionsRoot?: any; receiptsRoot?: any; logsBloom?: any; difficulty?: any; number?: any; gasLimit?: any; gasUsed?: any; timestamp?: any; extraData?: any; mixHash?: any; nonce?: any; baseFeePerGas?: any; withdrawalsRoot?: any } | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [Vec<PalletEth2FinalityVerifierBeaconBlockHeader>, U8aFixed, Vec<bool>, PalletEth2FinalityVerifierMerkleProof, PalletEth2FinalityVerifierExecutionPayload, PalletEth2FinalityVerifierMerkleProof, Vec<PalletEth2FinalityVerifierExecutionHeader>]>;
      /**
       * See [`Pallet::submit_fork`].
       **/
      submitFork: AugmentedSubmittable<(encodedNewUpdate: Bytes | string | Uint8Array, encodedOldUpdate: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes, Bytes]>;
      /**
       * See [`Pallet::verify_event_inclusion`].
       **/
      verifyEventInclusion: AugmentedSubmittable<(proof: PalletEth2FinalityVerifierEthereumEventInclusionProof | { blockNumber?: any; witness?: any; index?: any; event?: any } | string | Uint8Array, speedMode: T3rnPrimitivesSpeedMode | 'Fast' | 'Rational' | 'Finalized' | 'Instant' | number | Uint8Array, sourceAddress: Option<H160> | null | Uint8Array | H160 | string) => SubmittableExtrinsic<ApiType>, [PalletEth2FinalityVerifierEthereumEventInclusionProof, T3rnPrimitivesSpeedMode, Option<H160>]>;
      /**
       * See [`Pallet::verify_receipt_inclusion`].
       **/
      verifyReceiptInclusion: AugmentedSubmittable<(proof: PalletEth2FinalityVerifierEthereumReceiptInclusionProof | { blockNumber?: any; witness?: any; index?: any } | string | Uint8Array, speedMode: T3rnPrimitivesSpeedMode | 'Fast' | 'Rational' | 'Finalized' | 'Instant' | number | Uint8Array) => SubmittableExtrinsic<ApiType>, [PalletEth2FinalityVerifierEthereumReceiptInclusionProof, T3rnPrimitivesSpeedMode]>;
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
    grandpa: {
      /**
       * See [`Pallet::note_stalled`].
       **/
      noteStalled: AugmentedSubmittable<(delay: u32 | AnyNumber | Uint8Array, bestFinalizedBlockNumber: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32]>;
      /**
       * See [`Pallet::report_equivocation`].
       **/
      reportEquivocation: AugmentedSubmittable<(equivocationProof: SpConsensusGrandpaEquivocationProof | { setId?: any; equivocation?: any } | string | Uint8Array, keyOwnerProof: SpCoreVoid | null) => SubmittableExtrinsic<ApiType>, [SpConsensusGrandpaEquivocationProof, SpCoreVoid]>;
      /**
       * See [`Pallet::report_equivocation_unsigned`].
       **/
      reportEquivocationUnsigned: AugmentedSubmittable<(equivocationProof: SpConsensusGrandpaEquivocationProof | { setId?: any; equivocation?: any } | string | Uint8Array, keyOwnerProof: SpCoreVoid | null) => SubmittableExtrinsic<ApiType>, [SpConsensusGrandpaEquivocationProof, SpCoreVoid]>;
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
    maintenanceMode: {
      /**
       * See [`Pallet::enter_maintenance_mode`].
       **/
      enterMaintenanceMode: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
      /**
       * See [`Pallet::resume_normal_operation`].
       **/
      resumeNormalOperation: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
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
    portal: {
      /**
       * See [`Pallet::register_gateway`].
       **/
      registerGateway: AugmentedSubmittable<(gatewayId: U8aFixed | string | Uint8Array, tokenId: u32 | AnyNumber | Uint8Array, verificationVendor: T3rnPrimitivesGatewayVendor | 'Polkadot' | 'Kusama' | 'Rococo' | 'Ethereum' | 'Sepolia' | 'XBI' | 'Attesters' | number | Uint8Array, executionVendor: T3rnPrimitivesExecutionVendor | 'Substrate' | 'EVM' | number | Uint8Array, codec: T3rnAbiRecodeCodec | 'Scale' | 'Rlp' | number | Uint8Array, registrant: Option<AccountId32> | null | Uint8Array | AccountId32 | string, escrowAccount: Option<AccountId32> | null | Uint8Array | AccountId32 | string, allowedSideEffects: Vec<ITuple<[U8aFixed, Option<u8>]>> | ([U8aFixed | string | Uint8Array, Option<u8> | null | Uint8Array | u8 | AnyNumber])[], tokenProps: T3rnPrimitivesTokenInfo | { Substrate: any } | { Ethereum: any } | string | Uint8Array, tokenLocation: Option<XcmV3MultiLocation> | null | Uint8Array | XcmV3MultiLocation | { parents?: any; interior?: any } | string, encodedRegistrationData: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [U8aFixed, u32, T3rnPrimitivesGatewayVendor, T3rnPrimitivesExecutionVendor, T3rnAbiRecodeCodec, Option<AccountId32>, Option<AccountId32>, Vec<ITuple<[U8aFixed, Option<u8>]>>, T3rnPrimitivesTokenInfo, Option<XcmV3MultiLocation>, Bytes]>;
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
       * See [`Pallet::set_owner`].
       **/
      setOwner: AugmentedSubmittable<(owner: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [AccountId32]>;
      /**
       * See [`Pallet::submit_epoch`].
       **/
      submitEpoch: AugmentedSubmittable<(encodedUpdate: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes]>;
      /**
       * See [`Pallet::submit_epoch_decoded`].
       **/
      submitEpochDecoded: AugmentedSubmittable<(attestedBeaconHeader: PalletSepoliaFinalityVerifierBeaconBlockHeader | { slot?: any; proposerIndex?: any; parentRoot?: any; stateRoot?: any; bodyRoot?: any } | string | Uint8Array, signature: U8aFixed | string | Uint8Array, signerBits: Vec<bool> | (bool | boolean | Uint8Array)[], justifiedProof: PalletSepoliaFinalityVerifierMerkleProof | { gIndex?: any; witness?: any } | string | Uint8Array, executionPayload: PalletSepoliaFinalityVerifierExecutionPayload | { parentHash?: any; feeRecipient?: any; stateRoot?: any; receiptsRoot?: any; logsBloom?: any; prevRandao?: any; blockNumber?: any; gasLimit?: any; gasUsed?: any; timestamp?: any; extraData?: any; baseFeePerGas?: any; blockHash?: any; transactionsRoot?: any; withdrawalsRoot?: any } | string | Uint8Array, payloadProof: PalletSepoliaFinalityVerifierMerkleProof | { gIndex?: any; witness?: any } | string | Uint8Array, executionRange: Vec<PalletSepoliaFinalityVerifierExecutionHeader> | (PalletSepoliaFinalityVerifierExecutionHeader | { parentHash?: any; ommersHash?: any; beneficiary?: any; stateRoot?: any; transactionsRoot?: any; receiptsRoot?: any; logsBloom?: any; difficulty?: any; number?: any; gasLimit?: any; gasUsed?: any; timestamp?: any; extraData?: any; mixHash?: any; nonce?: any; baseFeePerGas?: any; withdrawalsRoot?: any } | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [PalletSepoliaFinalityVerifierBeaconBlockHeader, U8aFixed, Vec<bool>, PalletSepoliaFinalityVerifierMerkleProof, PalletSepoliaFinalityVerifierExecutionPayload, PalletSepoliaFinalityVerifierMerkleProof, Vec<PalletSepoliaFinalityVerifierExecutionHeader>]>;
      /**
       * See [`Pallet::submit_epoch_skipped_slot`].
       **/
      submitEpochSkippedSlot: AugmentedSubmittable<(encodedUpdate: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes]>;
      /**
       * See [`Pallet::submit_epoch_skipped_slot_decoded`].
       **/
      submitEpochSkippedSlotDecoded: AugmentedSubmittable<(beaconHeaders: Vec<PalletSepoliaFinalityVerifierBeaconBlockHeader> | (PalletSepoliaFinalityVerifierBeaconBlockHeader | { slot?: any; proposerIndex?: any; parentRoot?: any; stateRoot?: any; bodyRoot?: any } | string | Uint8Array)[], signature: U8aFixed | string | Uint8Array, signerBits: Vec<bool> | (bool | boolean | Uint8Array)[], justifiedProof: PalletSepoliaFinalityVerifierMerkleProof | { gIndex?: any; witness?: any } | string | Uint8Array, executionPayload: PalletSepoliaFinalityVerifierExecutionPayload | { parentHash?: any; feeRecipient?: any; stateRoot?: any; receiptsRoot?: any; logsBloom?: any; prevRandao?: any; blockNumber?: any; gasLimit?: any; gasUsed?: any; timestamp?: any; extraData?: any; baseFeePerGas?: any; blockHash?: any; transactionsRoot?: any; withdrawalsRoot?: any } | string | Uint8Array, payloadProof: PalletSepoliaFinalityVerifierMerkleProof | { gIndex?: any; witness?: any } | string | Uint8Array, executionRange: Vec<PalletSepoliaFinalityVerifierExecutionHeader> | (PalletSepoliaFinalityVerifierExecutionHeader | { parentHash?: any; ommersHash?: any; beneficiary?: any; stateRoot?: any; transactionsRoot?: any; receiptsRoot?: any; logsBloom?: any; difficulty?: any; number?: any; gasLimit?: any; gasUsed?: any; timestamp?: any; extraData?: any; mixHash?: any; nonce?: any; baseFeePerGas?: any; withdrawalsRoot?: any } | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [Vec<PalletSepoliaFinalityVerifierBeaconBlockHeader>, U8aFixed, Vec<bool>, PalletSepoliaFinalityVerifierMerkleProof, PalletSepoliaFinalityVerifierExecutionPayload, PalletSepoliaFinalityVerifierMerkleProof, Vec<PalletSepoliaFinalityVerifierExecutionHeader>]>;
      /**
       * See [`Pallet::submit_fork`].
       **/
      submitFork: AugmentedSubmittable<(encodedNewUpdate: Bytes | string | Uint8Array, encodedOldUpdate: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes, Bytes]>;
      /**
       * See [`Pallet::submit_unsigned_epoch_decoded`].
       **/
      submitUnsignedEpochDecoded: AugmentedSubmittable<(updates: Vec<PalletSepoliaFinalityVerifierEpochUpdate> | (PalletSepoliaFinalityVerifierEpochUpdate | { attestedBeaconHeader?: any; signature?: any; signerBits?: any; justifiedProof?: any; executionPayload?: any; payloadProof?: any; executionRange?: any } | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [Vec<PalletSepoliaFinalityVerifierEpochUpdate>]>;
      /**
       * See [`Pallet::submit_unsigned_epoch_skipped_slot_decoded`].
       **/
      submitUnsignedEpochSkippedSlotDecoded: AugmentedSubmittable<(updates: Vec<PalletSepoliaFinalityVerifierEpochUpdateSkippedSlot> | (PalletSepoliaFinalityVerifierEpochUpdateSkippedSlot | { attestedBeaconHeader?: any; checkpointBeaconHeader?: any; signature?: any; signerBits?: any; justifiedProof?: any; executionPayload?: any; payloadProof?: any; executionRange?: any } | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [Vec<PalletSepoliaFinalityVerifierEpochUpdateSkippedSlot>]>;
      /**
       * See [`Pallet::verify_event_inclusion`].
       **/
      verifyEventInclusion: AugmentedSubmittable<(proof: PalletSepoliaFinalityVerifierEthereumEventInclusionProof | { blockNumber?: any; witness?: any; index?: any; event?: any } | string | Uint8Array, speedMode: T3rnPrimitivesSpeedMode | 'Fast' | 'Rational' | 'Finalized' | 'Instant' | number | Uint8Array, sourceAddress: Option<H160> | null | Uint8Array | H160 | string) => SubmittableExtrinsic<ApiType>, [PalletSepoliaFinalityVerifierEthereumEventInclusionProof, T3rnPrimitivesSpeedMode, Option<H160>]>;
      /**
       * See [`Pallet::verify_receipt_inclusion`].
       **/
      verifyReceiptInclusion: AugmentedSubmittable<(proof: PalletSepoliaFinalityVerifierEthereumReceiptInclusionProof | { blockNumber?: any; witness?: any; index?: any } | string | Uint8Array, speedMode: T3rnPrimitivesSpeedMode | 'Fast' | 'Rational' | 'Finalized' | 'Instant' | number | Uint8Array) => SubmittableExtrinsic<ApiType>, [PalletSepoliaFinalityVerifierEthereumReceiptInclusionProof, T3rnPrimitivesSpeedMode]>;
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
      dispatchAs: AugmentedSubmittable<(asOrigin: CircuitStandaloneRuntimeOriginCaller | { system: any } | { Void: any } | string | Uint8Array, call: Call | IMethod | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [CircuitStandaloneRuntimeOriginCaller, Call]>;
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
      order: AugmentedSubmittable<(sfxActions: Vec<T3rnPrimitivesCircuitTypesOrderSFX> | (T3rnPrimitivesCircuitTypesOrderSFX | { sfxAction?: any; maxReward?: any; rewardAsset?: any; insurance?: any; remoteOriginNonce?: any } | string | Uint8Array)[], speedMode: T3rnPrimitivesSpeedMode | 'Fast' | 'Rational' | 'Finalized' | 'Instant' | number | Uint8Array) => SubmittableExtrinsic<ApiType>, [Vec<T3rnPrimitivesCircuitTypesOrderSFX>, T3rnPrimitivesSpeedMode]>;
      /**
       * See [`Pallet::read_all_pending_orders_status`].
       **/
      readAllPendingOrdersStatus: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
      /**
       * See [`Pallet::read_order_status`].
       **/
      readOrderStatus: AugmentedSubmittable<(xtxId: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [H256]>;
      /**
       * See [`Pallet::remote_order`].
       **/
      remoteOrder: AugmentedSubmittable<(orderRemoteProof: Bytes | string | Uint8Array, remoteTargetId: U8aFixed | string | Uint8Array, speedMode: T3rnPrimitivesSpeedMode | 'Fast' | 'Rational' | 'Finalized' | 'Instant' | number | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes, U8aFixed, T3rnPrimitivesSpeedMode]>;
      /**
       * See [`Pallet::single_order`].
       **/
      singleOrder: AugmentedSubmittable<(destination: U8aFixed | string | Uint8Array, asset: u32 | AnyNumber | Uint8Array, amount: u128 | AnyNumber | Uint8Array, rewardAsset: u32 | AnyNumber | Uint8Array, maxReward: u128 | AnyNumber | Uint8Array, insurance: u128 | AnyNumber | Uint8Array, targetAccount: AccountId32 | string | Uint8Array, speedMode: T3rnPrimitivesSpeedMode | 'Fast' | 'Rational' | 'Finalized' | 'Instant' | number | Uint8Array) => SubmittableExtrinsic<ApiType>, [U8aFixed, u32, u128, u32, u128, u128, AccountId32, T3rnPrimitivesSpeedMode]>;
    };
    xdns: {
      /**
       * See [`Pallet::add_remote_order_address`].
       **/
      addRemoteOrderAddress: AugmentedSubmittable<(targetId: U8aFixed | string | Uint8Array, remoteAddress: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [U8aFixed, H256]>;
      /**
       * See [`Pallet::add_supported_bridging_asset`].
       **/
      addSupportedBridgingAsset: AugmentedSubmittable<(assetId: u32 | AnyNumber | Uint8Array, targetId: U8aFixed | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, U8aFixed]>;
      /**
       * See [`Pallet::enroll_bridge_asset`].
       **/
      enrollBridgeAsset: AugmentedSubmittable<(assetId: u32 | AnyNumber | Uint8Array, targetId: U8aFixed | string | Uint8Array, tokenInfo: T3rnPrimitivesTokenInfo | { Substrate: any } | { Ethereum: any } | string | Uint8Array, tokenLocation: Option<XcmV3MultiLocation> | null | Uint8Array | XcmV3MultiLocation | { parents?: any; interior?: any } | string) => SubmittableExtrinsic<ApiType>, [u32, U8aFixed, T3rnPrimitivesTokenInfo, Option<XcmV3MultiLocation>]>;
      /**
       * See [`Pallet::enroll_new_abi_to_selected_gateway`].
       **/
      enrollNewAbiToSelectedGateway: AugmentedSubmittable<(targetId: U8aFixed | string | Uint8Array, sfx4bId: U8aFixed | string | Uint8Array, sfxExpectedAbi: Option<T3rnAbiSfxAbi> | null | Uint8Array | T3rnAbiSfxAbi | { argsNames?: any; maybePrefixMemo?: any; egressAbiDescriptors?: any; ingressAbiDescriptors?: any } | string, maybePalletId: Option<u8> | null | Uint8Array | u8 | AnyNumber) => SubmittableExtrinsic<ApiType>, [U8aFixed, U8aFixed, Option<T3rnAbiSfxAbi>, Option<u8>]>;
      /**
       * See [`Pallet::force_add_new_gateway`].
       **/
      forceAddNewGateway: AugmentedSubmittable<(gatewayId: U8aFixed | string | Uint8Array, verificationVendor: T3rnPrimitivesGatewayVendor | 'Polkadot' | 'Kusama' | 'Rococo' | 'Ethereum' | 'Sepolia' | 'XBI' | 'Attesters' | number | Uint8Array, executionVendor: T3rnPrimitivesExecutionVendor | 'Substrate' | 'EVM' | number | Uint8Array, codec: T3rnAbiRecodeCodec | 'Scale' | 'Rlp' | number | Uint8Array, registrant: Option<AccountId32> | null | Uint8Array | AccountId32 | string, escrowAccount: Option<AccountId32> | null | Uint8Array | AccountId32 | string, allowedSideEffects: Vec<ITuple<[U8aFixed, Option<u8>]>> | ([U8aFixed | string | Uint8Array, Option<u8> | null | Uint8Array | u8 | AnyNumber])[]) => SubmittableExtrinsic<ApiType>, [U8aFixed, T3rnPrimitivesGatewayVendor, T3rnPrimitivesExecutionVendor, T3rnAbiRecodeCodec, Option<AccountId32>, Option<AccountId32>, Vec<ITuple<[U8aFixed, Option<u8>]>>]>;
      /**
       * See [`Pallet::link_token`].
       **/
      linkToken: AugmentedSubmittable<(gatewayId: U8aFixed | string | Uint8Array, tokenId: u32 | AnyNumber | Uint8Array, tokenProps: T3rnPrimitivesTokenInfo | { Substrate: any } | { Ethereum: any } | string | Uint8Array, tokenLocation: Option<XcmV3MultiLocation> | null | Uint8Array | XcmV3MultiLocation | { parents?: any; interior?: any } | string) => SubmittableExtrinsic<ApiType>, [U8aFixed, u32, T3rnPrimitivesTokenInfo, Option<XcmV3MultiLocation>]>;
      /**
       * See [`Pallet::purge_gateway_record`].
       **/
      purgeGatewayRecord: AugmentedSubmittable<(requester: AccountId32 | string | Uint8Array, gatewayId: U8aFixed | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [AccountId32, U8aFixed]>;
      /**
       * See [`Pallet::purge_supported_bridging_asset`].
       **/
      purgeSupportedBridgingAsset: AugmentedSubmittable<(assetId: u32 | AnyNumber | Uint8Array, targetId: U8aFixed | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, U8aFixed]>;
      /**
       * See [`Pallet::purge_token_record`].
       **/
      purgeTokenRecord: AugmentedSubmittable<(tokenId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * See [`Pallet::reboot_self_gateway`].
       **/
      rebootSelfGateway: AugmentedSubmittable<(vendor: T3rnPrimitivesGatewayVendor | 'Polkadot' | 'Kusama' | 'Rococo' | 'Ethereum' | 'Sepolia' | 'XBI' | 'Attesters' | number | Uint8Array) => SubmittableExtrinsic<ApiType>, [T3rnPrimitivesGatewayVendor]>;
      /**
       * See [`Pallet::unlink_token`].
       **/
      unlinkToken: AugmentedSubmittable<(gatewayId: U8aFixed | string | Uint8Array, tokenId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [U8aFixed, u32]>;
      /**
       * See [`Pallet::unroll_abi_of_selected_gateway`].
       **/
      unrollAbiOfSelectedGateway: AugmentedSubmittable<(targetId: U8aFixed | string | Uint8Array, sfx4bId: U8aFixed | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [U8aFixed, U8aFixed]>;
      /**
       * See [`Pallet::unzip_topology`].
       **/
      unzipTopology: AugmentedSubmittable<(topologyDecoded: Option<T3rnPrimitivesXdnsTopology> | null | Uint8Array | T3rnPrimitivesXdnsTopology | { gateways?: any; assets?: any } | string, topologyEncoded: Option<Bytes> | null | Uint8Array | Bytes | string) => SubmittableExtrinsic<ApiType>, [Option<T3rnPrimitivesXdnsTopology>, Option<Bytes>]>;
      /**
       * See [`Pallet::zip_topology`].
       **/
      zipTopology: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
    };
  } // AugmentedSubmittables
} // declare module
