// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

import type { ApiTypes } from "@polkadot/api-base/types";
import type {
  U8aFixed,
  Vec,
  u128,
  u16,
  u32,
  u64,
  u8,
} from "@polkadot/types-codec";
import type { Codec } from "@polkadot/types-codec/types";
import type { AccountId32 } from "@polkadot/types/interfaces/runtime";
import type {
  FrameSupportPalletId,
  FrameSupportWeightsRuntimeDbWeight,
  FrameSupportWeightsWeightToFeeCoefficient,
  FrameSystemLimitsBlockLength,
  FrameSystemLimitsBlockWeights,
  PalletWasmContractsSchedule,
  SpVersionRuntimeVersion,
} from "@polkadot/types/lookup";

declare module "@polkadot/api-base/types/consts" {
  export interface AugmentedConsts<ApiType extends ApiTypes> {
    balances: {
      /** The minimum amount required to keep an account open. */
      existentialDeposit: u128 & AugmentedConst<ApiType>;
      /**
       * The maximum number of locks that should exist on an account. Not
       * strictly enforced, but used for weight estimation.
       */
      maxLocks: u32 & AugmentedConst<ApiType>;
      /** The maximum number of named reserves that can exist on an account. */
      maxReserves: u32 & AugmentedConst<ApiType>;
      /** Generic const */
      [key: string]: Codec;
    };
    circuit: {
      /**
       * The Circuit's deletion queue limit - preventing potential delay when
       * queue is too long in on_initialize
       */
      deletionQueueLimit: u32 & AugmentedConst<ApiType>;
      /** The Circuit's pallet id */
      palletId: FrameSupportPalletId & AugmentedConst<ApiType>;
      /** The Circuit's self gateway id */
      selfGatewayId: U8aFixed & AugmentedConst<ApiType>;
<<<<<<< HEAD
      /** The Circuit's Xtx timeout check interval */
      xtxTimeoutCheckInterval: u32 & AugmentedConst<ApiType>;
      /** The Circuit's Default Xtx timeout */
      xtxTimeoutDefault: u32 & AugmentedConst<ApiType>;
=======
>>>>>>> df7a772d... Add pallet-inflation types
      /** Generic const */
      [key: string]: Codec;
    };
    contracts: {
      /**
       * The maximum number of contracts that can be pending for deletion.
       *
       * When a contract is deleted by calling `seal_terminate` it becomes
       * inaccessible immediately, but the deletion of the storage items it has
       * accumulated is performed later. The contract is put into the deletion
       * queue. This defines how many contracts can be queued up at the same
       * time. If that limit is reached `seal_terminate` will fail. The action
       * must be retried in a later block in that case.
       *
       * The reasons for limiting the queue depth are:
       *
       * 1. The queue is in storage in order to be persistent between blocks. We
       *    want to limit the amount of storage that can be consumed.
       * 2. The queue is stored in a vector and needs to be decoded as a whole when
       *    reading it at the end of each block. Longer queues take more weight
       *    to decode and hence limit the amount of items that can be deleted per block.
       */
      deletionQueueDepth: u32 & AugmentedConst<ApiType>;
      /**
       * The maximum amount of weight that can be consumed per block for lazy
       * trie removal.
       *
       * The amount of weight that is dedicated per block to work on the
       * deletion queue. Larger values allow more trie keys to be deleted in
       * each block but reduce the amount of weight that is left for
       * transactions. See [`Self::DeletionQueueDepth`] for more information
       * about the deletion queue.
       */
      deletionWeightLimit: u64 & AugmentedConst<ApiType>;
      /**
       * The amount of balance a caller has to pay for each byte of storage.
       *
       * # Note
       *
       * Changing this value for an existing chain might need a storage migration.
       */
      depositPerByte: u128 & AugmentedConst<ApiType>;
      /**
       * The amount of balance a caller has to pay for each storage item.
       *
       * # Note
       *
       * Changing this value for an existing chain might need a storage migration.
       */
      depositPerItem: u128 & AugmentedConst<ApiType>;
      /** Cost schedule and limits. */
      schedule: PalletWasmContractsSchedule & AugmentedConst<ApiType>;
      /** Generic const */
      [key: string]: Codec;
    };
    grandpa: {
      /** Max Authorities in use */
      maxAuthorities: u32 & AugmentedConst<ApiType>;
      /** Generic const */
      [key: string]: Codec;
    };
    inflation: {
      defaultBlocksPerRound: u32 & AugmentedConst<ApiType>;
      tokenCirculationAtGenesis: u32 & AugmentedConst<ApiType>;
      treasuryAccount: AccountId32 & AugmentedConst<ApiType>;
      /** Generic const */
      [key: string]: Codec;
    };
    multiFinalityVerifierDefault: {
      /**
       * Maximal number of finalized headers to keep in the storage.
       *
       * The setting is there to prevent growing the on-chain state
       * indefinitely. Note the setting does not relate to block numbers - we
       * will simply keep as much items in the storage, so it doesn't guarantee
       * any fixed timeframe for finality headers.
       */
      headersToKeep: u32 & AugmentedConst<ApiType>;
      /**
       * The upper bound on the number of requests allowed by the pallet.
       *
       * A request refers to an action which writes a header to storage.
       *
       * Once this bound is reached the pallet will not allow any dispatchables
       * to be called until the request count has decreased.
       */
      maxRequests: u32 & AugmentedConst<ApiType>;
      /** Generic const */
      [key: string]: Codec;
    };
    multiFinalityVerifierEthereumLike: {
      /**
       * Maximal number of finalized headers to keep in the storage.
       *
       * The setting is there to prevent growing the on-chain state
       * indefinitely. Note the setting does not relate to block numbers - we
       * will simply keep as much items in the storage, so it doesn't guarantee
       * any fixed timeframe for finality headers.
       */
      headersToKeep: u32 & AugmentedConst<ApiType>;
      /**
       * The upper bound on the number of requests allowed by the pallet.
       *
       * A request refers to an action which writes a header to storage.
       *
       * Once this bound is reached the pallet will not allow any dispatchables
       * to be called until the request count has decreased.
       */
      maxRequests: u32 & AugmentedConst<ApiType>;
      /** Generic const */
      [key: string]: Codec;
    };
    multiFinalityVerifierGenericLike: {
      /**
       * Maximal number of finalized headers to keep in the storage.
       *
       * The setting is there to prevent growing the on-chain state
       * indefinitely. Note the setting does not relate to block numbers - we
       * will simply keep as much items in the storage, so it doesn't guarantee
       * any fixed timeframe for finality headers.
       */
      headersToKeep: u32 & AugmentedConst<ApiType>;
      /**
       * The upper bound on the number of requests allowed by the pallet.
       *
       * A request refers to an action which writes a header to storage.
       *
       * Once this bound is reached the pallet will not allow any dispatchables
       * to be called until the request count has decreased.
       */
      maxRequests: u32 & AugmentedConst<ApiType>;
      /** Generic const */
      [key: string]: Codec;
    };
    multiFinalityVerifierPolkadotLike: {
      /**
       * Maximal number of finalized headers to keep in the storage.
       *
       * The setting is there to prevent growing the on-chain state
       * indefinitely. Note the setting does not relate to block numbers - we
       * will simply keep as much items in the storage, so it doesn't guarantee
       * any fixed timeframe for finality headers.
       */
      headersToKeep: u32 & AugmentedConst<ApiType>;
      /**
       * The upper bound on the number of requests allowed by the pallet.
       *
       * A request refers to an action which writes a header to storage.
       *
       * Once this bound is reached the pallet will not allow any dispatchables
       * to be called until the request count has decreased.
       */
      maxRequests: u32 & AugmentedConst<ApiType>;
      /** Generic const */
      [key: string]: Codec;
    };
    multiFinalityVerifierSubstrateLike: {
      /**
       * Maximal number of finalized headers to keep in the storage.
       *
       * The setting is there to prevent growing the on-chain state
       * indefinitely. Note the setting does not relate to block numbers - we
       * will simply keep as much items in the storage, so it doesn't guarantee
       * any fixed timeframe for finality headers.
       */
      headersToKeep: u32 & AugmentedConst<ApiType>;
      /**
       * The upper bound on the number of requests allowed by the pallet.
       *
       * A request refers to an action which writes a header to storage.
       *
       * Once this bound is reached the pallet will not allow any dispatchables
       * to be called until the request count has decreased.
       */
      maxRequests: u32 & AugmentedConst<ApiType>;
      /** Generic const */
      [key: string]: Codec;
    };
    ormlTokens: {
      maxLocks: u32 & AugmentedConst<ApiType>;
      /** Generic const */
      [key: string]: Codec;
    };
    system: {
      /**
       * Maximum number of block number to block hash mappings to keep (oldest
       * pruned first).
       */
      blockHashCount: u32 & AugmentedConst<ApiType>;
      /** The maximum length of a block (in bytes). */
      blockLength: FrameSystemLimitsBlockLength & AugmentedConst<ApiType>;
      /** Block & extrinsics weights: base values and limits. */
      blockWeights: FrameSystemLimitsBlockWeights & AugmentedConst<ApiType>;
      /** The weight of runtime database operations the runtime can invoke. */
      dbWeight: FrameSupportWeightsRuntimeDbWeight & AugmentedConst<ApiType>;
      /**
       * The designated SS85 prefix of this chain.
       *
       * This replaces the "ss58Format" property declared in the chain spec.
       * Reason is that the runtime should know about the prefix in order to
       * make use of it as an identifier of the chain.
       */
      ss58Prefix: u16 & AugmentedConst<ApiType>;
      /** Get the chain's current version. */
      version: SpVersionRuntimeVersion & AugmentedConst<ApiType>;
      /** Generic const */
      [key: string]: Codec;
    };
    timestamp: {
      /**
       * The minimum period between blocks. Beware that this is different to the
       * _expected_ period that the block production apparatus provides. Your
       * chosen consensus system will generally work with this to determine a
       * sensible block time. e.g. For Aura, it will be double this period on
       * default settings.
       */
      minimumPeriod: u64 & AugmentedConst<ApiType>;
      /** Generic const */
      [key: string]: Codec;
    };
    transactionPayment: {
      /**
       * A fee mulitplier for `Operational` extrinsics to compute "virtual tip"
       * to boost their `priority`
       *
       * This value is multipled by the `final_fee` to obtain a "virtual tip"
       * that is later added to a tip component in regular `priority`
       * calculations. It means that a `Normal` transaction can front-run a
       * similarly-sized `Operational` extrinsic (with no tip), by including a
       * tip value greater than the virtual tip.
       *
       * ```rust,ignore
       * // For `Normal`
       * let priority = priority_calc(tip);
       *
       * // For `Operational`
       * let virtual_tip = (inclusion_fee + tip) * OperationalFeeMultiplier;
       * let priority = priority_calc(tip + virtual_tip);
       * ```
       *
       * Note that since we use `final_fee` the multiplier applies also to the
       * regular `tip` sent with the transaction. So, not only does the
       * transaction get a priority bump based on the `inclusion_fee`, but we
       * also amplify the impact of tips applied to `Operational` transactions.
       */
      operationalFeeMultiplier: u8 & AugmentedConst<ApiType>;
      /** The fee to be paid for making a transaction; the per-byte portion. */
      transactionByteFee: u128 & AugmentedConst<ApiType>;
      /** The polynomial that is applied in order to derive fee from weight. */
      weightToFee: Vec<FrameSupportWeightsWeightToFeeCoefficient> &
        AugmentedConst<ApiType>;
      /** Generic const */
      [key: string]: Codec;
    };
    utility: {
      /** The limit on the number of batched calls. */
      batchedCallsLimit: u32 & AugmentedConst<ApiType>;
      /** Generic const */
      [key: string]: Codec;
    };
  } // AugmentedConsts
} // declare module
