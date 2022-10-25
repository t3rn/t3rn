// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import "@polkadot/types/lookup";

import type {
  BTreeMap,
  Bytes,
  Compact,
  Enum,
  Null,
  Option,
  Result,
  Struct,
  Text,
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
import type { ITuple } from "@polkadot/types-codec/types";
import type {
  AccountId32,
  Call,
  H160,
  H256,
  MultiAddress,
  Perbill,
} from "@polkadot/types/interfaces/runtime";
import type { Event } from "@polkadot/types/interfaces/system";

declare module "@polkadot/types/lookup" {
  /** @name FrameSystemAccountInfo (3) */
  interface FrameSystemAccountInfo extends Struct {
    readonly nonce: u32;
    readonly consumers: u32;
    readonly providers: u32;
    readonly sufficients: u32;
    readonly data: PalletBalancesAccountData;
  }

  /** @name PalletBalancesAccountData (5) */
  interface PalletBalancesAccountData extends Struct {
    readonly free: u128;
    readonly reserved: u128;
    readonly miscFrozen: u128;
    readonly feeFrozen: u128;
  }

  /** @name FrameSupportWeightsPerDispatchClassU64 (7) */
  interface FrameSupportWeightsPerDispatchClassU64 extends Struct {
    readonly normal: u64;
    readonly operational: u64;
    readonly mandatory: u64;
  }

  /** @name SpRuntimeDigest (11) */
  interface SpRuntimeDigest extends Struct {
    readonly logs: Vec<SpRuntimeDigestDigestItem>;
  }

  /** @name SpRuntimeDigestDigestItem (13) */
  interface SpRuntimeDigestDigestItem extends Enum {
    readonly isOther: boolean;
    readonly asOther: Bytes;
    readonly isConsensus: boolean;
    readonly asConsensus: ITuple<[U8aFixed, Bytes]>;
    readonly isSeal: boolean;
    readonly asSeal: ITuple<[U8aFixed, Bytes]>;
    readonly isPreRuntime: boolean;
    readonly asPreRuntime: ITuple<[U8aFixed, Bytes]>;
    readonly isRuntimeEnvironmentUpdated: boolean;
    readonly type:
      | "Other"
      | "Consensus"
      | "Seal"
      | "PreRuntime"
      | "RuntimeEnvironmentUpdated";
  }

  /** @name FrameSystemEventRecord (16) */
  interface FrameSystemEventRecord extends Struct {
    readonly phase: FrameSystemPhase;
    readonly event: Event;
    readonly topics: Vec<H256>;
  }

  /** @name FrameSystemEvent (18) */
  interface FrameSystemEvent extends Enum {
    readonly isExtrinsicSuccess: boolean;
    readonly asExtrinsicSuccess: {
      readonly dispatchInfo: FrameSupportWeightsDispatchInfo;
    } & Struct;
    readonly isExtrinsicFailed: boolean;
    readonly asExtrinsicFailed: {
      readonly dispatchError: SpRuntimeDispatchError;
      readonly dispatchInfo: FrameSupportWeightsDispatchInfo;
    } & Struct;
    readonly isCodeUpdated: boolean;
    readonly isNewAccount: boolean;
    readonly asNewAccount: {
      readonly account: AccountId32;
    } & Struct;
    readonly isKilledAccount: boolean;
    readonly asKilledAccount: {
      readonly account: AccountId32;
    } & Struct;
    readonly isRemarked: boolean;
    readonly asRemarked: {
      readonly sender: AccountId32;
      readonly hash_: H256;
    } & Struct;
    readonly type:
      | "ExtrinsicSuccess"
      | "ExtrinsicFailed"
      | "CodeUpdated"
      | "NewAccount"
      | "KilledAccount"
      | "Remarked";
  }

  /** @name FrameSupportWeightsDispatchInfo (19) */
  interface FrameSupportWeightsDispatchInfo extends Struct {
    readonly weight: u64;
    readonly class: FrameSupportWeightsDispatchClass;
    readonly paysFee: FrameSupportWeightsPays;
  }

  /** @name FrameSupportWeightsDispatchClass (20) */
  interface FrameSupportWeightsDispatchClass extends Enum {
    readonly isNormal: boolean;
    readonly isOperational: boolean;
    readonly isMandatory: boolean;
    readonly type: "Normal" | "Operational" | "Mandatory";
  }

  /** @name FrameSupportWeightsPays (21) */
  interface FrameSupportWeightsPays extends Enum {
    readonly isYes: boolean;
    readonly isNo: boolean;
    readonly type: "Yes" | "No";
  }

  /** @name SpRuntimeDispatchError (22) */
  interface SpRuntimeDispatchError extends Enum {
    readonly isOther: boolean;
    readonly isCannotLookup: boolean;
    readonly isBadOrigin: boolean;
    readonly isModule: boolean;
    readonly asModule: SpRuntimeModuleError;
    readonly isConsumerRemaining: boolean;
    readonly isNoProviders: boolean;
    readonly isTooManyConsumers: boolean;
    readonly isToken: boolean;
    readonly asToken: SpRuntimeTokenError;
    readonly isArithmetic: boolean;
    readonly asArithmetic: SpRuntimeArithmeticError;
    readonly isTransactional: boolean;
    readonly asTransactional: SpRuntimeTransactionalError;
    readonly type:
      | "Other"
      | "CannotLookup"
      | "BadOrigin"
      | "Module"
      | "ConsumerRemaining"
      | "NoProviders"
      | "TooManyConsumers"
      | "Token"
      | "Arithmetic"
      | "Transactional";
  }

  /** @name SpRuntimeModuleError (23) */
  interface SpRuntimeModuleError extends Struct {
    readonly index: u8;
    readonly error: U8aFixed;
  }

  /** @name SpRuntimeTokenError (24) */
  interface SpRuntimeTokenError extends Enum {
    readonly isNoFunds: boolean;
    readonly isWouldDie: boolean;
    readonly isBelowMinimum: boolean;
    readonly isCannotCreate: boolean;
    readonly isUnknownAsset: boolean;
    readonly isFrozen: boolean;
    readonly isUnsupported: boolean;
    readonly type:
      | "NoFunds"
      | "WouldDie"
      | "BelowMinimum"
      | "CannotCreate"
      | "UnknownAsset"
      | "Frozen"
      | "Unsupported";
  }

  /** @name SpRuntimeArithmeticError (25) */
  interface SpRuntimeArithmeticError extends Enum {
    readonly isUnderflow: boolean;
    readonly isOverflow: boolean;
    readonly isDivisionByZero: boolean;
    readonly type: "Underflow" | "Overflow" | "DivisionByZero";
  }

  /** @name SpRuntimeTransactionalError (26) */
  interface SpRuntimeTransactionalError extends Enum {
    readonly isLimitReached: boolean;
    readonly isNoLayer: boolean;
    readonly type: "LimitReached" | "NoLayer";
  }

  /** @name PalletGrandpaEvent (27) */
  interface PalletGrandpaEvent extends Enum {
    readonly isNewAuthorities: boolean;
    readonly asNewAuthorities: {
      readonly authoritySet: Vec<ITuple<[SpFinalityGrandpaAppPublic, u64]>>;
    } & Struct;
    readonly isPaused: boolean;
    readonly isResumed: boolean;
    readonly type: "NewAuthorities" | "Paused" | "Resumed";
  }

  /** @name SpFinalityGrandpaAppPublic (30) */
  interface SpFinalityGrandpaAppPublic extends SpCoreEd25519Public {}

  /** @name SpCoreEd25519Public (31) */
  interface SpCoreEd25519Public extends U8aFixed {}

  /** @name PalletSudoEvent (32) */
  interface PalletSudoEvent extends Enum {
    readonly isSudid: boolean;
    readonly asSudid: {
      readonly sudoResult: Result<Null, SpRuntimeDispatchError>;
    } & Struct;
    readonly isKeyChanged: boolean;
    readonly asKeyChanged: {
      readonly oldSudoer: Option<AccountId32>;
    } & Struct;
    readonly isSudoAsDone: boolean;
    readonly asSudoAsDone: {
      readonly sudoResult: Result<Null, SpRuntimeDispatchError>;
    } & Struct;
    readonly type: "Sudid" | "KeyChanged" | "SudoAsDone";
  }

  /** @name PalletUtilityEvent (36) */
  interface PalletUtilityEvent extends Enum {
    readonly isBatchInterrupted: boolean;
    readonly asBatchInterrupted: {
      readonly index: u32;
      readonly error: SpRuntimeDispatchError;
    } & Struct;
    readonly isBatchCompleted: boolean;
    readonly isBatchCompletedWithErrors: boolean;
    readonly isItemCompleted: boolean;
    readonly isItemFailed: boolean;
    readonly asItemFailed: {
      readonly error: SpRuntimeDispatchError;
    } & Struct;
    readonly isDispatchedAs: boolean;
    readonly asDispatchedAs: {
      readonly result: Result<Null, SpRuntimeDispatchError>;
    } & Struct;
    readonly type:
      | "BatchInterrupted"
      | "BatchCompleted"
      | "BatchCompletedWithErrors"
      | "ItemCompleted"
      | "ItemFailed"
      | "DispatchedAs";
  }

  /** @name PalletBalancesEvent (37) */
  interface PalletBalancesEvent extends Enum {
    readonly isEndowed: boolean;
    readonly asEndowed: {
      readonly account: AccountId32;
      readonly freeBalance: u128;
    } & Struct;
    readonly isDustLost: boolean;
    readonly asDustLost: {
      readonly account: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isTransfer: boolean;
    readonly asTransfer: {
      readonly from: AccountId32;
      readonly to: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isBalanceSet: boolean;
    readonly asBalanceSet: {
      readonly who: AccountId32;
      readonly free: u128;
      readonly reserved: u128;
    } & Struct;
    readonly isReserved: boolean;
    readonly asReserved: {
      readonly who: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isUnreserved: boolean;
    readonly asUnreserved: {
      readonly who: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isReserveRepatriated: boolean;
    readonly asReserveRepatriated: {
      readonly from: AccountId32;
      readonly to: AccountId32;
      readonly amount: u128;
      readonly destinationStatus: FrameSupportTokensMiscBalanceStatus;
    } & Struct;
    readonly isDeposit: boolean;
    readonly asDeposit: {
      readonly who: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isWithdraw: boolean;
    readonly asWithdraw: {
      readonly who: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isSlashed: boolean;
    readonly asSlashed: {
      readonly who: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly type:
      | "Endowed"
      | "DustLost"
      | "Transfer"
      | "BalanceSet"
      | "Reserved"
      | "Unreserved"
      | "ReserveRepatriated"
      | "Deposit"
      | "Withdraw"
      | "Slashed";
  }

  /** @name FrameSupportTokensMiscBalanceStatus (38) */
  interface FrameSupportTokensMiscBalanceStatus extends Enum {
    readonly isFree: boolean;
    readonly isReserved: boolean;
    readonly type: "Free" | "Reserved";
  }

  /** @name PalletTransactionPaymentEvent (39) */
  interface PalletTransactionPaymentEvent extends Enum {
    readonly isTransactionFeePaid: boolean;
    readonly asTransactionFeePaid: {
      readonly who: AccountId32;
      readonly actualFee: u128;
      readonly tip: u128;
    } & Struct;
    readonly type: "TransactionFeePaid";
  }

  /** @name PalletAssetsEvent (40) */
  interface PalletAssetsEvent extends Enum {
    readonly isCreated: boolean;
    readonly asCreated: {
      readonly assetId: u32;
      readonly creator: AccountId32;
      readonly owner: AccountId32;
    } & Struct;
    readonly isIssued: boolean;
    readonly asIssued: {
      readonly assetId: u32;
      readonly owner: AccountId32;
      readonly totalSupply: u128;
    } & Struct;
    readonly isTransferred: boolean;
    readonly asTransferred: {
      readonly assetId: u32;
      readonly from: AccountId32;
      readonly to: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isBurned: boolean;
    readonly asBurned: {
      readonly assetId: u32;
      readonly owner: AccountId32;
      readonly balance: u128;
    } & Struct;
    readonly isTeamChanged: boolean;
    readonly asTeamChanged: {
      readonly assetId: u32;
      readonly issuer: AccountId32;
      readonly admin: AccountId32;
      readonly freezer: AccountId32;
    } & Struct;
    readonly isOwnerChanged: boolean;
    readonly asOwnerChanged: {
      readonly assetId: u32;
      readonly owner: AccountId32;
    } & Struct;
    readonly isFrozen: boolean;
    readonly asFrozen: {
      readonly assetId: u32;
      readonly who: AccountId32;
    } & Struct;
    readonly isThawed: boolean;
    readonly asThawed: {
      readonly assetId: u32;
      readonly who: AccountId32;
    } & Struct;
    readonly isAssetFrozen: boolean;
    readonly asAssetFrozen: {
      readonly assetId: u32;
    } & Struct;
    readonly isAssetThawed: boolean;
    readonly asAssetThawed: {
      readonly assetId: u32;
    } & Struct;
    readonly isDestroyed: boolean;
    readonly asDestroyed: {
      readonly assetId: u32;
    } & Struct;
    readonly isForceCreated: boolean;
    readonly asForceCreated: {
      readonly assetId: u32;
      readonly owner: AccountId32;
    } & Struct;
    readonly isMetadataSet: boolean;
    readonly asMetadataSet: {
      readonly assetId: u32;
      readonly name: Bytes;
      readonly symbol: Bytes;
      readonly decimals: u8;
      readonly isFrozen: bool;
    } & Struct;
    readonly isMetadataCleared: boolean;
    readonly asMetadataCleared: {
      readonly assetId: u32;
    } & Struct;
    readonly isApprovedTransfer: boolean;
    readonly asApprovedTransfer: {
      readonly assetId: u32;
      readonly source: AccountId32;
      readonly delegate: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isApprovalCancelled: boolean;
    readonly asApprovalCancelled: {
      readonly assetId: u32;
      readonly owner: AccountId32;
      readonly delegate: AccountId32;
    } & Struct;
    readonly isTransferredApproved: boolean;
    readonly asTransferredApproved: {
      readonly assetId: u32;
      readonly owner: AccountId32;
      readonly delegate: AccountId32;
      readonly destination: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isAssetStatusChanged: boolean;
    readonly asAssetStatusChanged: {
      readonly assetId: u32;
    } & Struct;
    readonly type:
      | "Created"
      | "Issued"
      | "Transferred"
      | "Burned"
      | "TeamChanged"
      | "OwnerChanged"
      | "Frozen"
      | "Thawed"
      | "AssetFrozen"
      | "AssetThawed"
      | "Destroyed"
      | "ForceCreated"
      | "MetadataSet"
      | "MetadataCleared"
      | "ApprovedTransfer"
      | "ApprovalCancelled"
      | "TransferredApproved"
      | "AssetStatusChanged";
  }

  /** @name PalletXdnsEvent (42) */
  interface PalletXdnsEvent extends Enum {
    readonly isXdnsRecordStored: boolean;
    readonly asXdnsRecordStored: U8aFixed;
    readonly isXdnsRecordPurged: boolean;
    readonly asXdnsRecordPurged: ITuple<[AccountId32, U8aFixed]>;
    readonly isXdnsRecordUpdated: boolean;
    readonly asXdnsRecordUpdated: U8aFixed;
    readonly type:
      | "XdnsRecordStored"
      | "XdnsRecordPurged"
      | "XdnsRecordUpdated";
  }

  /** @name PalletContractsRegistryEvent (43) */
  interface PalletContractsRegistryEvent extends Enum {
    readonly isContractStored: boolean;
    readonly asContractStored: ITuple<[AccountId32, H256]>;
    readonly isContractPurged: boolean;
    readonly asContractPurged: ITuple<[AccountId32, H256]>;
    readonly type: "ContractStored" | "ContractPurged";
  }

  /** @name PalletCircuitEvent (44) */
  interface PalletCircuitEvent extends Enum {
    readonly isTransfer: boolean;
    readonly asTransfer: ITuple<[AccountId32, AccountId32, AccountId32, u128]>;
    readonly isTransferAssets: boolean;
    readonly asTransferAssets: ITuple<
      [AccountId32, u64, AccountId32, AccountId32, u128]
    >;
    readonly isTransferORML: boolean;
    readonly asTransferORML: ITuple<
      [AccountId32, u64, AccountId32, AccountId32, u128]
    >;
    readonly isAddLiquidity: boolean;
    readonly asAddLiquidity: ITuple<[AccountId32, u64, u64, u128, u128, u128]>;
    readonly isSwap: boolean;
    readonly asSwap: ITuple<[AccountId32, u64, u64, u128, u128, u128]>;
    readonly isCallNative: boolean;
    readonly asCallNative: ITuple<[AccountId32, Bytes]>;
    readonly isCallEvm: boolean;
    readonly asCallEvm: ITuple<
      [
        AccountId32,
        H160,
        H160,
        U256,
        Bytes,
        u64,
        U256,
        Option<U256>,
        Option<U256>,
        Vec<ITuple<[H160, Vec<H256>]>>
      ]
    >;
    readonly isCallWasm: boolean;
    readonly asCallWasm: ITuple<
      [AccountId32, AccountId32, u128, u64, Option<u128>, Bytes]
    >;
    readonly isCallCustom: boolean;
    readonly asCallCustom: ITuple<
      [AccountId32, AccountId32, AccountId32, u128, Bytes, u64, Bytes]
    >;
    readonly isNotification: boolean;
    readonly asNotification: ITuple<
      [
        AccountId32,
        AccountId32,
        PalletXbiPortalXbiFormatXbiNotificationKind,
        Bytes,
        Bytes
      ]
    >;
    readonly isResult: boolean;
    readonly asResult: ITuple<
      [
        AccountId32,
        AccountId32,
        PalletXbiPortalXbiFormatXbiCheckOutStatus,
        Bytes,
        Bytes
      ]
    >;
    readonly isXTransactionReceivedForExec: boolean;
    readonly asXTransactionReceivedForExec: H256;
    readonly isSideEffectInsuranceReceived: boolean;
    readonly asSideEffectInsuranceReceived: ITuple<[H256, AccountId32]>;
    readonly isSfxNewBidReceived: boolean;
    readonly asSfxNewBidReceived: ITuple<[H256, H256, AccountId32, u128]>;
    readonly isSideEffectConfirmed: boolean;
    readonly asSideEffectConfirmed: H256;
    readonly isXTransactionReadyForExec: boolean;
    readonly asXTransactionReadyForExec: H256;
    readonly isXTransactionStepFinishedExec: boolean;
    readonly asXTransactionStepFinishedExec: H256;
    readonly isXTransactionXtxFinishedExecAllSteps: boolean;
    readonly asXTransactionXtxFinishedExecAllSteps: H256;
    readonly isXTransactionXtxRevertedAfterTimeOut: boolean;
    readonly asXTransactionXtxRevertedAfterTimeOut: H256;
    readonly isNewSideEffectsAvailable: boolean;
    readonly asNewSideEffectsAvailable: ITuple<
      [AccountId32, H256, Vec<T3rnTypesSideEffect>, Vec<H256>]
    >;
    readonly isCancelledSideEffects: boolean;
    readonly asCancelledSideEffects: ITuple<
      [AccountId32, H256, Vec<T3rnTypesSideEffect>]
    >;
    readonly isSideEffectsConfirmed: boolean;
    readonly asSideEffectsConfirmed: ITuple<
      [H256, Vec<Vec<T3rnPrimitivesSideEffectFullSideEffect>>]
    >;
    readonly isEscrowTransfer: boolean;
    readonly asEscrowTransfer: ITuple<[AccountId32, AccountId32, u128]>;
    readonly type:
      | "Transfer"
      | "TransferAssets"
      | "TransferORML"
      | "AddLiquidity"
      | "Swap"
      | "CallNative"
      | "CallEvm"
      | "CallWasm"
      | "CallCustom"
      | "Notification"
      | "Result"
      | "XTransactionReceivedForExec"
      | "SideEffectInsuranceReceived"
      | "SfxNewBidReceived"
      | "SideEffectConfirmed"
      | "XTransactionReadyForExec"
      | "XTransactionStepFinishedExec"
      | "XTransactionXtxFinishedExecAllSteps"
      | "XTransactionXtxRevertedAfterTimeOut"
      | "NewSideEffectsAvailable"
      | "CancelledSideEffects"
      | "SideEffectsConfirmed"
      | "EscrowTransfer";
  }

  /** @name PalletXbiPortalXbiFormatXbiNotificationKind (54) */
  interface PalletXbiPortalXbiFormatXbiNotificationKind extends Enum {
    readonly isSent: boolean;
    readonly isDelivered: boolean;
    readonly isExecuted: boolean;
    readonly type: "Sent" | "Delivered" | "Executed";
  }

  /** @name PalletXbiPortalXbiFormatXbiCheckOutStatus (55) */
  interface PalletXbiPortalXbiFormatXbiCheckOutStatus extends Enum {
    readonly isSuccessfullyExecuted: boolean;
    readonly isErrorFailedExecution: boolean;
    readonly isErrorFailedXCMDispatch: boolean;
    readonly isErrorDeliveryTimeout: boolean;
    readonly isErrorExecutionTimeout: boolean;
    readonly type:
      | "SuccessfullyExecuted"
      | "ErrorFailedExecution"
      | "ErrorFailedXCMDispatch"
      | "ErrorDeliveryTimeout"
      | "ErrorExecutionTimeout";
  }

  /** @name T3rnTypesSideEffect (57) */
  interface T3rnTypesSideEffect extends Struct {
    readonly target: U8aFixed;
    readonly maxFee: u128;
    readonly insurance: u128;
    readonly encodedAction: Bytes;
    readonly encodedArgs: Vec<Bytes>;
    readonly signature: Bytes;
    readonly requesterNonce: u32;
    readonly enforceExecutor: Option<AccountId32>;
  }

  /** @name T3rnPrimitivesSideEffectFullSideEffect (61) */
  interface T3rnPrimitivesSideEffectFullSideEffect extends Struct {
    readonly input: T3rnTypesSideEffect;
    readonly confirmed: Option<T3rnTypesSideEffectConfirmedSideEffect>;
    readonly securityLvl: T3rnTypesSideEffectSecurityLvl;
    readonly submissionTargetHeight: Bytes;
    readonly bestBid: Option<T3rnPrimitivesSideEffectSfxBid>;
  }

  /** @name T3rnTypesSideEffectConfirmedSideEffect (63) */
  interface T3rnTypesSideEffectConfirmedSideEffect extends Struct {
    readonly err: Option<T3rnTypesSideEffectConfirmationOutcome>;
    readonly output: Option<Bytes>;
    readonly inclusionData: Bytes;
    readonly executioner: AccountId32;
    readonly receivedAt: u32;
    readonly cost: Option<u128>;
  }

  /** @name T3rnTypesSideEffectConfirmationOutcome (65) */
  interface T3rnTypesSideEffectConfirmationOutcome extends Enum {
    readonly isSuccess: boolean;
    readonly isMisbehaviourMalformedValues: boolean;
    readonly asMisbehaviourMalformedValues: {
      readonly key: Bytes;
      readonly expected: Bytes;
      readonly received: Bytes;
    } & Struct;
    readonly isTimedOut: boolean;
    readonly type: "Success" | "MisbehaviourMalformedValues" | "TimedOut";
  }

  /** @name T3rnTypesSideEffectSecurityLvl (67) */
  interface T3rnTypesSideEffectSecurityLvl extends Enum {
    readonly isOptimistic: boolean;
    readonly isEscrow: boolean;
    readonly type: "Optimistic" | "Escrow";
  }

  /** @name T3rnPrimitivesSideEffectSfxBid (69) */
  interface T3rnPrimitivesSideEffectSfxBid extends Struct {
    readonly bid: u128;
    readonly insurance: u128;
    readonly reservedBond: Option<u128>;
    readonly executor: AccountId32;
    readonly requester: AccountId32;
  }

  /** @name PalletTreasuryEvent (70) */
  interface PalletTreasuryEvent extends Enum {
    readonly isNewRound: boolean;
    readonly asNewRound: {
      readonly round: u32;
      readonly head: u32;
    } & Struct;
    readonly isRoundTermChanged: boolean;
    readonly asRoundTermChanged: {
      readonly old: u32;
      readonly new_: u32;
      readonly roundMin: Perbill;
      readonly roundIdeal: Perbill;
      readonly roundMax: Perbill;
    } & Struct;
    readonly isInflationConfigChanged: boolean;
    readonly asInflationConfigChanged: {
      readonly annualMin: Perbill;
      readonly annualIdeal: Perbill;
      readonly annualMax: Perbill;
      readonly roundMin: Perbill;
      readonly roundIdeal: Perbill;
      readonly roundMax: Perbill;
    } & Struct;
    readonly isInflationAllocationChanged: boolean;
    readonly asInflationAllocationChanged: {
      readonly developer: Perbill;
      readonly executor: Perbill;
    } & Struct;
    readonly isRoundTokensIssued: boolean;
    readonly asRoundTokensIssued: ITuple<[u32, u128]>;
    readonly isBeneficiaryTokensIssued: boolean;
    readonly asBeneficiaryTokensIssued: ITuple<[AccountId32, u128]>;
    readonly isRewardsClaimed: boolean;
    readonly asRewardsClaimed: ITuple<[AccountId32, u128]>;
    readonly type:
      | "NewRound"
      | "RoundTermChanged"
      | "InflationConfigChanged"
      | "InflationAllocationChanged"
      | "RoundTokensIssued"
      | "BeneficiaryTokensIssued"
      | "RewardsClaimed";
  }

  /** @name PalletClockEvent (72) */
  type PalletClockEvent = Null;

  /** @name PalletXbiPortalEvent (73) */
  interface PalletXbiPortalEvent extends Enum {
    readonly isAbiInstructionExecuted: boolean;
    readonly type: "AbiInstructionExecuted";
  }

  /** @name Pallet3vmEvent (74) */
  interface Pallet3vmEvent extends Enum {
    readonly isSignalBounced: boolean;
    readonly asSignalBounced: ITuple<
      [u32, T3rnSdkPrimitivesSignalSignalKind, H256]
    >;
    readonly isExceededBounceThrehold: boolean;
    readonly asExceededBounceThrehold: ITuple<
      [u32, T3rnSdkPrimitivesSignalSignalKind, H256]
    >;
    readonly isModuleInstantiated: boolean;
    readonly asModuleInstantiated: ITuple<
      [H256, AccountId32, T3rnPrimitivesContractMetadataContractType, u32]
    >;
    readonly isAuthorStored: boolean;
    readonly asAuthorStored: ITuple<[AccountId32, AccountId32]>;
    readonly isAuthorRemoved: boolean;
    readonly asAuthorRemoved: AccountId32;
    readonly type:
      | "SignalBounced"
      | "ExceededBounceThrehold"
      | "ModuleInstantiated"
      | "AuthorStored"
      | "AuthorRemoved";
  }

  /** @name T3rnSdkPrimitivesSignalSignalKind (76) */
  interface T3rnSdkPrimitivesSignalSignalKind extends Enum {
    readonly isComplete: boolean;
    readonly isKill: boolean;
    readonly asKill: T3rnSdkPrimitivesSignalKillReason;
    readonly type: "Complete" | "Kill";
  }

  /** @name T3rnSdkPrimitivesSignalKillReason (77) */
  interface T3rnSdkPrimitivesSignalKillReason extends Enum {
    readonly isUnhandled: boolean;
    readonly isCodec: boolean;
    readonly isTimeout: boolean;
    readonly type: "Unhandled" | "Codec" | "Timeout";
  }

  /** @name T3rnPrimitivesContractMetadataContractType (79) */
  interface T3rnPrimitivesContractMetadataContractType extends Enum {
    readonly isSystem: boolean;
    readonly isVanillaEvm: boolean;
    readonly isVanillaWasm: boolean;
    readonly isVolatileEvm: boolean;
    readonly isVolatileWasm: boolean;
    readonly type:
      | "System"
      | "VanillaEvm"
      | "VanillaWasm"
      | "VolatileEvm"
      | "VolatileWasm";
  }

  /** @name PalletContractsEvent (81) */
  interface PalletContractsEvent extends Enum {
    readonly isInstantiated: boolean;
    readonly asInstantiated: {
      readonly deployer: AccountId32;
      readonly contract: AccountId32;
    } & Struct;
    readonly isTerminated: boolean;
    readonly asTerminated: {
      readonly contract: AccountId32;
      readonly beneficiary: AccountId32;
    } & Struct;
    readonly isCodeStored: boolean;
    readonly asCodeStored: {
      readonly codeHash: H256;
    } & Struct;
    readonly isContractEmitted: boolean;
    readonly asContractEmitted: {
      readonly contract: AccountId32;
      readonly data: Bytes;
    } & Struct;
    readonly isCodeRemoved: boolean;
    readonly asCodeRemoved: {
      readonly codeHash: H256;
    } & Struct;
    readonly isContractCodeUpdated: boolean;
    readonly asContractCodeUpdated: {
      readonly contract: AccountId32;
      readonly newCodeHash: H256;
      readonly oldCodeHash: H256;
    } & Struct;
    readonly type:
      | "Instantiated"
      | "Terminated"
      | "CodeStored"
      | "ContractEmitted"
      | "CodeRemoved"
      | "ContractCodeUpdated";
  }

  /** @name PalletEvmEvent (82) */
  interface PalletEvmEvent extends Enum {
    readonly isLog: boolean;
    readonly asLog: EthereumLog;
    readonly isCreated: boolean;
    readonly asCreated: H160;
    readonly isCreatedFailed: boolean;
    readonly asCreatedFailed: H160;
    readonly isExecuted: boolean;
    readonly asExecuted: H160;
    readonly isExecutedFailed: boolean;
    readonly asExecutedFailed: H160;
    readonly isBalanceDeposit: boolean;
    readonly asBalanceDeposit: ITuple<[AccountId32, H160, U256]>;
    readonly isBalanceWithdraw: boolean;
    readonly asBalanceWithdraw: ITuple<[AccountId32, H160, U256]>;
    readonly isClaimAccount: boolean;
    readonly asClaimAccount: {
      readonly accountId: AccountId32;
      readonly evmAddress: H160;
    } & Struct;
    readonly type:
      | "Log"
      | "Created"
      | "CreatedFailed"
      | "Executed"
      | "ExecutedFailed"
      | "BalanceDeposit"
      | "BalanceWithdraw"
      | "ClaimAccount";
  }

  /** @name EthereumLog (83) */
  interface EthereumLog extends Struct {
    readonly address: H160;
    readonly topics: Vec<H256>;
    readonly data: Bytes;
  }

  /** @name PalletAccountManagerEvent (84) */
  interface PalletAccountManagerEvent extends Enum {
    readonly isContractsRegistryExecutionFinalized: boolean;
    readonly asContractsRegistryExecutionFinalized: {
      readonly executionId: u64;
    } & Struct;
    readonly isIssued: boolean;
    readonly asIssued: {
      readonly recipient: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isDepositReceived: boolean;
    readonly asDepositReceived: {
      readonly chargeId: H256;
      readonly payee: AccountId32;
      readonly recipient: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly type:
      | "ContractsRegistryExecutionFinalized"
      | "Issued"
      | "DepositReceived";
  }

  /** @name PalletPortalEvent (85) */
  interface PalletPortalEvent extends Enum {
    readonly isGatewayRegistered: boolean;
    readonly asGatewayRegistered: U8aFixed;
    readonly isSetOwner: boolean;
    readonly asSetOwner: ITuple<[U8aFixed, Bytes]>;
    readonly isSetOperational: boolean;
    readonly asSetOperational: ITuple<[U8aFixed, bool]>;
    readonly isHeaderSubmitted: boolean;
    readonly asHeaderSubmitted: ITuple<[U8aFixed, Bytes]>;
    readonly type:
      | "GatewayRegistered"
      | "SetOwner"
      | "SetOperational"
      | "HeaderSubmitted";
  }

  /** @name FrameSystemPhase (86) */
  interface FrameSystemPhase extends Enum {
    readonly isApplyExtrinsic: boolean;
    readonly asApplyExtrinsic: u32;
    readonly isFinalization: boolean;
    readonly isInitialization: boolean;
    readonly type: "ApplyExtrinsic" | "Finalization" | "Initialization";
  }

  /** @name FrameSystemLastRuntimeUpgradeInfo (89) */
  interface FrameSystemLastRuntimeUpgradeInfo extends Struct {
    readonly specVersion: Compact<u32>;
    readonly specName: Text;
  }

  /** @name FrameSystemCall (92) */
  interface FrameSystemCall extends Enum {
    readonly isFillBlock: boolean;
    readonly asFillBlock: {
      readonly ratio: Perbill;
    } & Struct;
    readonly isRemark: boolean;
    readonly asRemark: {
      readonly remark: Bytes;
    } & Struct;
    readonly isSetHeapPages: boolean;
    readonly asSetHeapPages: {
      readonly pages: u64;
    } & Struct;
    readonly isSetCode: boolean;
    readonly asSetCode: {
      readonly code: Bytes;
    } & Struct;
    readonly isSetCodeWithoutChecks: boolean;
    readonly asSetCodeWithoutChecks: {
      readonly code: Bytes;
    } & Struct;
    readonly isSetStorage: boolean;
    readonly asSetStorage: {
      readonly items: Vec<ITuple<[Bytes, Bytes]>>;
    } & Struct;
    readonly isKillStorage: boolean;
    readonly asKillStorage: {
      readonly keys_: Vec<Bytes>;
    } & Struct;
    readonly isKillPrefix: boolean;
    readonly asKillPrefix: {
      readonly prefix: Bytes;
      readonly subkeys: u32;
    } & Struct;
    readonly isRemarkWithEvent: boolean;
    readonly asRemarkWithEvent: {
      readonly remark: Bytes;
    } & Struct;
    readonly type:
      | "FillBlock"
      | "Remark"
      | "SetHeapPages"
      | "SetCode"
      | "SetCodeWithoutChecks"
      | "SetStorage"
      | "KillStorage"
      | "KillPrefix"
      | "RemarkWithEvent";
  }

  /** @name FrameSystemLimitsBlockWeights (95) */
  interface FrameSystemLimitsBlockWeights extends Struct {
    readonly baseBlock: u64;
    readonly maxBlock: u64;
    readonly perClass: FrameSupportWeightsPerDispatchClassWeightsPerClass;
  }

  /** @name FrameSupportWeightsPerDispatchClassWeightsPerClass (96) */
  interface FrameSupportWeightsPerDispatchClassWeightsPerClass extends Struct {
    readonly normal: FrameSystemLimitsWeightsPerClass;
    readonly operational: FrameSystemLimitsWeightsPerClass;
    readonly mandatory: FrameSystemLimitsWeightsPerClass;
  }

  /** @name FrameSystemLimitsWeightsPerClass (97) */
  interface FrameSystemLimitsWeightsPerClass extends Struct {
    readonly baseExtrinsic: u64;
    readonly maxExtrinsic: Option<u64>;
    readonly maxTotal: Option<u64>;
    readonly reserved: Option<u64>;
  }

  /** @name FrameSystemLimitsBlockLength (99) */
  interface FrameSystemLimitsBlockLength extends Struct {
    readonly max: FrameSupportWeightsPerDispatchClassU32;
  }

  /** @name FrameSupportWeightsPerDispatchClassU32 (100) */
  interface FrameSupportWeightsPerDispatchClassU32 extends Struct {
    readonly normal: u32;
    readonly operational: u32;
    readonly mandatory: u32;
  }

  /** @name FrameSupportWeightsRuntimeDbWeight (101) */
  interface FrameSupportWeightsRuntimeDbWeight extends Struct {
    readonly read: u64;
    readonly write: u64;
  }

  /** @name SpVersionRuntimeVersion (102) */
  interface SpVersionRuntimeVersion extends Struct {
    readonly specName: Text;
    readonly implName: Text;
    readonly authoringVersion: u32;
    readonly specVersion: u32;
    readonly implVersion: u32;
    readonly apis: Vec<ITuple<[U8aFixed, u32]>>;
    readonly transactionVersion: u32;
    readonly stateVersion: u8;
  }

  /** @name FrameSystemError (108) */
  interface FrameSystemError extends Enum {
    readonly isInvalidSpecName: boolean;
    readonly isSpecVersionNeedsToIncrease: boolean;
    readonly isFailedToExtractRuntimeVersion: boolean;
    readonly isNonDefaultComposite: boolean;
    readonly isNonZeroRefCount: boolean;
    readonly isCallFiltered: boolean;
    readonly type:
      | "InvalidSpecName"
      | "SpecVersionNeedsToIncrease"
      | "FailedToExtractRuntimeVersion"
      | "NonDefaultComposite"
      | "NonZeroRefCount"
      | "CallFiltered";
  }

  /** @name PalletTimestampCall (110) */
  interface PalletTimestampCall extends Enum {
    readonly isSet: boolean;
    readonly asSet: {
      readonly now: Compact<u64>;
    } & Struct;
    readonly type: "Set";
  }

  /** @name SpConsensusAuraSr25519AppSr25519Public (113) */
  interface SpConsensusAuraSr25519AppSr25519Public
    extends SpCoreSr25519Public {}

  /** @name SpCoreSr25519Public (114) */
  interface SpCoreSr25519Public extends U8aFixed {}

  /** @name PalletGrandpaStoredState (117) */
  interface PalletGrandpaStoredState extends Enum {
    readonly isLive: boolean;
    readonly isPendingPause: boolean;
    readonly asPendingPause: {
      readonly scheduledAt: u32;
      readonly delay: u32;
    } & Struct;
    readonly isPaused: boolean;
    readonly isPendingResume: boolean;
    readonly asPendingResume: {
      readonly scheduledAt: u32;
      readonly delay: u32;
    } & Struct;
    readonly type: "Live" | "PendingPause" | "Paused" | "PendingResume";
  }

  /** @name PalletGrandpaStoredPendingChange (118) */
  interface PalletGrandpaStoredPendingChange extends Struct {
    readonly scheduledAt: u32;
    readonly delay: u32;
    readonly nextAuthorities: Vec<ITuple<[SpFinalityGrandpaAppPublic, u64]>>;
    readonly forced: Option<u32>;
  }

  /** @name PalletGrandpaCall (121) */
  interface PalletGrandpaCall extends Enum {
    readonly isReportEquivocation: boolean;
    readonly asReportEquivocation: {
      readonly equivocationProof: SpFinalityGrandpaEquivocationProof;
      readonly keyOwnerProof: SpCoreVoid;
    } & Struct;
    readonly isReportEquivocationUnsigned: boolean;
    readonly asReportEquivocationUnsigned: {
      readonly equivocationProof: SpFinalityGrandpaEquivocationProof;
      readonly keyOwnerProof: SpCoreVoid;
    } & Struct;
    readonly isNoteStalled: boolean;
    readonly asNoteStalled: {
      readonly delay: u32;
      readonly bestFinalizedBlockNumber: u32;
    } & Struct;
    readonly type:
      | "ReportEquivocation"
      | "ReportEquivocationUnsigned"
      | "NoteStalled";
  }

  /** @name SpFinalityGrandpaEquivocationProof (122) */
  interface SpFinalityGrandpaEquivocationProof extends Struct {
    readonly setId: u64;
    readonly equivocation: SpFinalityGrandpaEquivocation;
  }

  /** @name SpFinalityGrandpaEquivocation (123) */
  interface SpFinalityGrandpaEquivocation extends Enum {
    readonly isPrevote: boolean;
    readonly asPrevote: FinalityGrandpaEquivocationPrevote;
    readonly isPrecommit: boolean;
    readonly asPrecommit: FinalityGrandpaEquivocationPrecommit;
    readonly type: "Prevote" | "Precommit";
  }

  /** @name FinalityGrandpaEquivocationPrevote (124) */
  interface FinalityGrandpaEquivocationPrevote extends Struct {
    readonly roundNumber: u64;
    readonly identity: SpFinalityGrandpaAppPublic;
    readonly first: ITuple<
      [FinalityGrandpaPrevote, SpFinalityGrandpaAppSignature]
    >;
    readonly second: ITuple<
      [FinalityGrandpaPrevote, SpFinalityGrandpaAppSignature]
    >;
  }

  /** @name FinalityGrandpaPrevote (125) */
  interface FinalityGrandpaPrevote extends Struct {
    readonly targetHash: H256;
    readonly targetNumber: u32;
  }

  /** @name SpFinalityGrandpaAppSignature (126) */
  interface SpFinalityGrandpaAppSignature extends SpCoreEd25519Signature {}

  /** @name SpCoreEd25519Signature (127) */
  interface SpCoreEd25519Signature extends U8aFixed {}

  /** @name FinalityGrandpaEquivocationPrecommit (130) */
  interface FinalityGrandpaEquivocationPrecommit extends Struct {
    readonly roundNumber: u64;
    readonly identity: SpFinalityGrandpaAppPublic;
    readonly first: ITuple<
      [FinalityGrandpaPrecommit, SpFinalityGrandpaAppSignature]
    >;
    readonly second: ITuple<
      [FinalityGrandpaPrecommit, SpFinalityGrandpaAppSignature]
    >;
  }

  /** @name FinalityGrandpaPrecommit (131) */
  interface FinalityGrandpaPrecommit extends Struct {
    readonly targetHash: H256;
    readonly targetNumber: u32;
  }

  /** @name SpCoreVoid (133) */
  type SpCoreVoid = Null;

  /** @name PalletGrandpaError (134) */
  interface PalletGrandpaError extends Enum {
    readonly isPauseFailed: boolean;
    readonly isResumeFailed: boolean;
    readonly isChangePending: boolean;
    readonly isTooSoon: boolean;
    readonly isInvalidKeyOwnershipProof: boolean;
    readonly isInvalidEquivocationProof: boolean;
    readonly isDuplicateOffenceReport: boolean;
    readonly type:
      | "PauseFailed"
      | "ResumeFailed"
      | "ChangePending"
      | "TooSoon"
      | "InvalidKeyOwnershipProof"
      | "InvalidEquivocationProof"
      | "DuplicateOffenceReport";
  }

  /** @name PalletSudoCall (135) */
  interface PalletSudoCall extends Enum {
    readonly isSudo: boolean;
    readonly asSudo: {
      readonly call: Call;
    } & Struct;
    readonly isSudoUncheckedWeight: boolean;
    readonly asSudoUncheckedWeight: {
      readonly call: Call;
      readonly weight: u64;
    } & Struct;
    readonly isSetKey: boolean;
    readonly asSetKey: {
      readonly new_: MultiAddress;
    } & Struct;
    readonly isSudoAs: boolean;
    readonly asSudoAs: {
      readonly who: MultiAddress;
      readonly call: Call;
    } & Struct;
    readonly type: "Sudo" | "SudoUncheckedWeight" | "SetKey" | "SudoAs";
  }

  /** @name PalletUtilityCall (137) */
  interface PalletUtilityCall extends Enum {
    readonly isBatch: boolean;
    readonly asBatch: {
      readonly calls: Vec<Call>;
    } & Struct;
    readonly isAsDerivative: boolean;
    readonly asAsDerivative: {
      readonly index: u16;
      readonly call: Call;
    } & Struct;
    readonly isBatchAll: boolean;
    readonly asBatchAll: {
      readonly calls: Vec<Call>;
    } & Struct;
    readonly isDispatchAs: boolean;
    readonly asDispatchAs: {
      readonly asOrigin: CircuitStandaloneRuntimeOriginCaller;
      readonly call: Call;
    } & Struct;
    readonly isForceBatch: boolean;
    readonly asForceBatch: {
      readonly calls: Vec<Call>;
    } & Struct;
    readonly type:
      | "Batch"
      | "AsDerivative"
      | "BatchAll"
      | "DispatchAs"
      | "ForceBatch";
  }

  /** @name CircuitStandaloneRuntimeOriginCaller (139) */
  interface CircuitStandaloneRuntimeOriginCaller extends Enum {
    readonly isSystem: boolean;
    readonly asSystem: FrameSupportDispatchRawOrigin;
    readonly isVoid: boolean;
    readonly type: "System" | "Void";
  }

  /** @name FrameSupportDispatchRawOrigin (140) */
  interface FrameSupportDispatchRawOrigin extends Enum {
    readonly isRoot: boolean;
    readonly isSigned: boolean;
    readonly asSigned: AccountId32;
    readonly isNone: boolean;
    readonly type: "Root" | "Signed" | "None";
  }

  /** @name PalletBalancesCall (141) */
  interface PalletBalancesCall extends Enum {
    readonly isTransfer: boolean;
    readonly asTransfer: {
      readonly dest: MultiAddress;
      readonly value: Compact<u128>;
    } & Struct;
    readonly isSetBalance: boolean;
    readonly asSetBalance: {
      readonly who: MultiAddress;
      readonly newFree: Compact<u128>;
      readonly newReserved: Compact<u128>;
    } & Struct;
    readonly isForceTransfer: boolean;
    readonly asForceTransfer: {
      readonly source: MultiAddress;
      readonly dest: MultiAddress;
      readonly value: Compact<u128>;
    } & Struct;
    readonly isTransferKeepAlive: boolean;
    readonly asTransferKeepAlive: {
      readonly dest: MultiAddress;
      readonly value: Compact<u128>;
    } & Struct;
    readonly isTransferAll: boolean;
    readonly asTransferAll: {
      readonly dest: MultiAddress;
      readonly keepAlive: bool;
    } & Struct;
    readonly isForceUnreserve: boolean;
    readonly asForceUnreserve: {
      readonly who: MultiAddress;
      readonly amount: u128;
    } & Struct;
    readonly type:
      | "Transfer"
      | "SetBalance"
      | "ForceTransfer"
      | "TransferKeepAlive"
      | "TransferAll"
      | "ForceUnreserve";
  }

  /** @name PalletAssetsCall (145) */
  interface PalletAssetsCall extends Enum {
    readonly isCreate: boolean;
    readonly asCreate: {
      readonly id: Compact<u32>;
      readonly admin: MultiAddress;
      readonly minBalance: u128;
    } & Struct;
    readonly isForceCreate: boolean;
    readonly asForceCreate: {
      readonly id: Compact<u32>;
      readonly owner: MultiAddress;
      readonly isSufficient: bool;
      readonly minBalance: Compact<u128>;
    } & Struct;
    readonly isDestroy: boolean;
    readonly asDestroy: {
      readonly id: Compact<u32>;
      readonly witness: PalletAssetsDestroyWitness;
    } & Struct;
    readonly isMint: boolean;
    readonly asMint: {
      readonly id: Compact<u32>;
      readonly beneficiary: MultiAddress;
      readonly amount: Compact<u128>;
    } & Struct;
    readonly isBurn: boolean;
    readonly asBurn: {
      readonly id: Compact<u32>;
      readonly who: MultiAddress;
      readonly amount: Compact<u128>;
    } & Struct;
    readonly isTransfer: boolean;
    readonly asTransfer: {
      readonly id: Compact<u32>;
      readonly target: MultiAddress;
      readonly amount: Compact<u128>;
    } & Struct;
    readonly isTransferKeepAlive: boolean;
    readonly asTransferKeepAlive: {
      readonly id: Compact<u32>;
      readonly target: MultiAddress;
      readonly amount: Compact<u128>;
    } & Struct;
    readonly isForceTransfer: boolean;
    readonly asForceTransfer: {
      readonly id: Compact<u32>;
      readonly source: MultiAddress;
      readonly dest: MultiAddress;
      readonly amount: Compact<u128>;
    } & Struct;
    readonly isFreeze: boolean;
    readonly asFreeze: {
      readonly id: Compact<u32>;
      readonly who: MultiAddress;
    } & Struct;
    readonly isThaw: boolean;
    readonly asThaw: {
      readonly id: Compact<u32>;
      readonly who: MultiAddress;
    } & Struct;
    readonly isFreezeAsset: boolean;
    readonly asFreezeAsset: {
      readonly id: Compact<u32>;
    } & Struct;
    readonly isThawAsset: boolean;
    readonly asThawAsset: {
      readonly id: Compact<u32>;
    } & Struct;
    readonly isTransferOwnership: boolean;
    readonly asTransferOwnership: {
      readonly id: Compact<u32>;
      readonly owner: MultiAddress;
    } & Struct;
    readonly isSetTeam: boolean;
    readonly asSetTeam: {
      readonly id: Compact<u32>;
      readonly issuer: MultiAddress;
      readonly admin: MultiAddress;
      readonly freezer: MultiAddress;
    } & Struct;
    readonly isSetMetadata: boolean;
    readonly asSetMetadata: {
      readonly id: Compact<u32>;
      readonly name: Bytes;
      readonly symbol: Bytes;
      readonly decimals: u8;
    } & Struct;
    readonly isClearMetadata: boolean;
    readonly asClearMetadata: {
      readonly id: Compact<u32>;
    } & Struct;
    readonly isForceSetMetadata: boolean;
    readonly asForceSetMetadata: {
      readonly id: Compact<u32>;
      readonly name: Bytes;
      readonly symbol: Bytes;
      readonly decimals: u8;
      readonly isFrozen: bool;
    } & Struct;
    readonly isForceClearMetadata: boolean;
    readonly asForceClearMetadata: {
      readonly id: Compact<u32>;
    } & Struct;
    readonly isForceAssetStatus: boolean;
    readonly asForceAssetStatus: {
      readonly id: Compact<u32>;
      readonly owner: MultiAddress;
      readonly issuer: MultiAddress;
      readonly admin: MultiAddress;
      readonly freezer: MultiAddress;
      readonly minBalance: Compact<u128>;
      readonly isSufficient: bool;
      readonly isFrozen: bool;
    } & Struct;
    readonly isApproveTransfer: boolean;
    readonly asApproveTransfer: {
      readonly id: Compact<u32>;
      readonly delegate: MultiAddress;
      readonly amount: Compact<u128>;
    } & Struct;
    readonly isCancelApproval: boolean;
    readonly asCancelApproval: {
      readonly id: Compact<u32>;
      readonly delegate: MultiAddress;
    } & Struct;
    readonly isForceCancelApproval: boolean;
    readonly asForceCancelApproval: {
      readonly id: Compact<u32>;
      readonly owner: MultiAddress;
      readonly delegate: MultiAddress;
    } & Struct;
    readonly isTransferApproved: boolean;
    readonly asTransferApproved: {
      readonly id: Compact<u32>;
      readonly owner: MultiAddress;
      readonly destination: MultiAddress;
      readonly amount: Compact<u128>;
    } & Struct;
    readonly isTouch: boolean;
    readonly asTouch: {
      readonly id: Compact<u32>;
    } & Struct;
    readonly isRefund: boolean;
    readonly asRefund: {
      readonly id: Compact<u32>;
      readonly allowBurn: bool;
    } & Struct;
    readonly type:
      | "Create"
      | "ForceCreate"
      | "Destroy"
      | "Mint"
      | "Burn"
      | "Transfer"
      | "TransferKeepAlive"
      | "ForceTransfer"
      | "Freeze"
      | "Thaw"
      | "FreezeAsset"
      | "ThawAsset"
      | "TransferOwnership"
      | "SetTeam"
      | "SetMetadata"
      | "ClearMetadata"
      | "ForceSetMetadata"
      | "ForceClearMetadata"
      | "ForceAssetStatus"
      | "ApproveTransfer"
      | "CancelApproval"
      | "ForceCancelApproval"
      | "TransferApproved"
      | "Touch"
      | "Refund";
  }

  /** @name PalletAssetsDestroyWitness (146) */
  interface PalletAssetsDestroyWitness extends Struct {
    readonly accounts: Compact<u32>;
    readonly sufficients: Compact<u32>;
    readonly approvals: Compact<u32>;
  }

  /** @name PalletXdnsCall (147) */
  interface PalletXdnsCall extends Enum {
    readonly isAddSideEffect: boolean;
    readonly asAddSideEffect: {
      readonly id: U8aFixed;
      readonly name: Bytes;
      readonly argumentAbi: Vec<T3rnTypesAbiType>;
      readonly argumentToStateMapper: Vec<Bytes>;
      readonly confirmEvents: Vec<Bytes>;
      readonly escrowedEvents: Vec<Bytes>;
      readonly commitEvents: Vec<Bytes>;
      readonly revertEvents: Vec<Bytes>;
    } & Struct;
    readonly isUpdateTtl: boolean;
    readonly asUpdateTtl: {
      readonly gatewayId: U8aFixed;
      readonly lastFinalized: u64;
    } & Struct;
    readonly isPurgeXdnsRecord: boolean;
    readonly asPurgeXdnsRecord: {
      readonly requester: AccountId32;
      readonly xdnsRecordId: U8aFixed;
    } & Struct;
    readonly type: "AddSideEffect" | "UpdateTtl" | "PurgeXdnsRecord";
  }

  /** @name T3rnTypesAbiType (149) */
  interface T3rnTypesAbiType extends Enum {
    readonly isAddress: boolean;
    readonly asAddress: u16;
    readonly isDynamicAddress: boolean;
    readonly isBool: boolean;
    readonly isInt: boolean;
    readonly asInt: u16;
    readonly isUint: boolean;
    readonly asUint: u16;
    readonly isBytes: boolean;
    readonly asBytes: u8;
    readonly isDynamicBytes: boolean;
    readonly isString: boolean;
    readonly isEnum: boolean;
    readonly asEnum: u8;
    readonly isStruct: boolean;
    readonly asStruct: u8;
    readonly isMapping: boolean;
    readonly asMapping: ITuple<[T3rnTypesAbiType, T3rnTypesAbiType]>;
    readonly isContract: boolean;
    readonly isRef: boolean;
    readonly asRef: T3rnTypesAbiType;
    readonly isOption: boolean;
    readonly asOption: T3rnTypesAbiType;
    readonly isOptionalInsurance: boolean;
    readonly isOptionalReward: boolean;
    readonly isStorageRef: boolean;
    readonly asStorageRef: T3rnTypesAbiType;
    readonly isValue: boolean;
    readonly isSlice: boolean;
    readonly isHasher: boolean;
    readonly asHasher: ITuple<[T3rnTypesAbiHasherAlgo, u16]>;
    readonly isCrypto: boolean;
    readonly asCrypto: T3rnTypesAbiCryptoAlgo;
    readonly type:
      | "Address"
      | "DynamicAddress"
      | "Bool"
      | "Int"
      | "Uint"
      | "Bytes"
      | "DynamicBytes"
      | "String"
      | "Enum"
      | "Struct"
      | "Mapping"
      | "Contract"
      | "Ref"
      | "Option"
      | "OptionalInsurance"
      | "OptionalReward"
      | "StorageRef"
      | "Value"
      | "Slice"
      | "Hasher"
      | "Crypto";
  }

  /** @name T3rnTypesAbiHasherAlgo (150) */
  interface T3rnTypesAbiHasherAlgo extends Enum {
    readonly isBlake2: boolean;
    readonly isKeccak256: boolean;
    readonly type: "Blake2" | "Keccak256";
  }

  /** @name T3rnTypesAbiCryptoAlgo (151) */
  interface T3rnTypesAbiCryptoAlgo extends Enum {
    readonly isEd25519: boolean;
    readonly isSr25519: boolean;
    readonly isEcdsa: boolean;
    readonly type: "Ed25519" | "Sr25519" | "Ecdsa";
  }

  /** @name PalletContractsRegistryCall (152) */
  interface PalletContractsRegistryCall extends Enum {
    readonly isAddNewContract: boolean;
    readonly asAddNewContract: {
      readonly requester: AccountId32;
      readonly contract: T3rnPrimitivesContractsRegistryRegistryContract;
    } & Struct;
    readonly isPurge: boolean;
    readonly asPurge: {
      readonly requester: AccountId32;
      readonly contractId: H256;
    } & Struct;
    readonly type: "AddNewContract" | "Purge";
  }

  /** @name T3rnPrimitivesContractsRegistryRegistryContract (153) */
  interface T3rnPrimitivesContractsRegistryRegistryContract extends Struct {
    readonly codeTxt: Bytes;
    readonly bytes: Bytes;
    readonly author: T3rnPrimitivesContractsRegistryAuthorInfo;
    readonly abi: Option<Bytes>;
    readonly actionDescriptions: Vec<T3rnTypesAbiContractActionDesc>;
    readonly info: Option<T3rnPrimitivesStorageRawAliveContractInfo>;
    readonly meta: T3rnPrimitivesContractMetadata;
  }

  /** @name T3rnPrimitivesContractsRegistryAuthorInfo (154) */
  interface T3rnPrimitivesContractsRegistryAuthorInfo extends Struct {
    readonly account: AccountId32;
    readonly feesPerSingleUse: Option<u128>;
  }

  /** @name T3rnTypesAbiContractActionDesc (156) */
  interface T3rnTypesAbiContractActionDesc extends Struct {
    readonly actionId: H256;
    readonly targetId: Option<U8aFixed>;
    readonly to: Option<AccountId32>;
  }

  /** @name T3rnPrimitivesStorageRawAliveContractInfo (159) */
  interface T3rnPrimitivesStorageRawAliveContractInfo extends Struct {
    readonly trieId: Bytes;
    readonly storageSize: u32;
    readonly pairCount: u32;
    readonly codeHash: H256;
    readonly rentAllowance: u128;
    readonly rentPaid: u128;
    readonly deductBlock: u32;
    readonly lastWrite: Option<u32>;
    readonly reserved: Option<Null>;
  }

  /** @name T3rnPrimitivesContractMetadata (161) */
  interface T3rnPrimitivesContractMetadata extends Struct {
    readonly metadataVersion: Bytes;
    readonly name: Bytes;
    readonly contractType: T3rnPrimitivesContractMetadataContractType;
    readonly version: Bytes;
    readonly authors: Vec<Bytes>;
    readonly description: Option<Bytes>;
    readonly documentation: Option<Bytes>;
    readonly repository: Option<Bytes>;
    readonly homepage: Option<Bytes>;
    readonly license: Option<Bytes>;
  }

  /** @name PalletCircuitCall (162) */
  interface PalletCircuitCall extends Enum {
    readonly isOnLocalTrigger: boolean;
    readonly asOnLocalTrigger: {
      readonly trigger: Bytes;
    } & Struct;
    readonly isOnXcmTrigger: boolean;
    readonly isOnRemoteGatewayTrigger: boolean;
    readonly isOnExtrinsicTrigger: boolean;
    readonly asOnExtrinsicTrigger: {
      readonly sideEffects: Vec<T3rnTypesSideEffect>;
      readonly fee: u128;
      readonly sequential: bool;
    } & Struct;
    readonly isBidExecution: boolean;
    readonly asBidExecution: {
      readonly xtxId: H256;
      readonly sfxId: H256;
      readonly bidAmount: u128;
    } & Struct;
    readonly isExecuteSideEffectsWithXbi: boolean;
    readonly asExecuteSideEffectsWithXbi: {
      readonly xtxId: H256;
      readonly sideEffect: T3rnTypesSideEffect;
      readonly maxExecCost: u128;
      readonly maxNotificationsCost: u128;
    } & Struct;
    readonly isOnXbiSfxResolved: boolean;
    readonly asOnXbiSfxResolved: {
      readonly sfxId: H256;
    } & Struct;
    readonly isConfirmSideEffect: boolean;
    readonly asConfirmSideEffect: {
      readonly xtxId: H256;
      readonly sideEffect: T3rnTypesSideEffect;
      readonly confirmation: T3rnTypesSideEffectConfirmedSideEffect;
      readonly inclusionProof: Option<Vec<Bytes>>;
      readonly blockHash: Option<Bytes>;
    } & Struct;
    readonly type:
      | "OnLocalTrigger"
      | "OnXcmTrigger"
      | "OnRemoteGatewayTrigger"
      | "OnExtrinsicTrigger"
      | "BidExecution"
      | "ExecuteSideEffectsWithXbi"
      | "OnXbiSfxResolved"
      | "ConfirmSideEffect";
  }

  /** @name PalletTreasuryCall (164) */
  interface PalletTreasuryCall extends Enum {
    readonly isMintForRound: boolean;
    readonly asMintForRound: {
      readonly roundIndex: u32;
      readonly amount: Compact<u128>;
    } & Struct;
    readonly isClaimRewards: boolean;
    readonly isSetInflation: boolean;
    readonly asSetInflation: {
      readonly annualInflationConfig: {
        readonly min: Perbill;
        readonly ideal: Perbill;
        readonly max: Perbill;
      } & Struct;
    } & Struct;
    readonly isSetInflationAlloc: boolean;
    readonly asSetInflationAlloc: {
      readonly inflationAlloc: T3rnPrimitivesMonetaryInflationAllocation;
    } & Struct;
    readonly isSetRoundTerm: boolean;
    readonly asSetRoundTerm: {
      readonly new_: u32;
    } & Struct;
    readonly isAddBeneficiary: boolean;
    readonly asAddBeneficiary: {
      readonly beneficiary: AccountId32;
      readonly role: T3rnPrimitivesMonetaryBeneficiaryRole;
    } & Struct;
    readonly isRemoveBeneficiary: boolean;
    readonly asRemoveBeneficiary: {
      readonly beneficiary: AccountId32;
    } & Struct;
    readonly isSetTotalStakeExpectation: boolean;
    readonly asSetTotalStakeExpectation: {
      readonly expectations: {
        readonly min: u128;
        readonly ideal: u128;
        readonly max: u128;
      } & Struct;
    } & Struct;
    readonly type:
      | "MintForRound"
      | "ClaimRewards"
      | "SetInflation"
      | "SetInflationAlloc"
      | "SetRoundTerm"
      | "AddBeneficiary"
      | "RemoveBeneficiary"
      | "SetTotalStakeExpectation";
  }

  /** @name T3rnPrimitivesMonetaryInflationAllocation (166) */
  interface T3rnPrimitivesMonetaryInflationAllocation extends Struct {
    readonly developer: Perbill;
    readonly executor: Perbill;
  }

  /** @name T3rnPrimitivesMonetaryBeneficiaryRole (167) */
  interface T3rnPrimitivesMonetaryBeneficiaryRole extends Enum {
    readonly isDeveloper: boolean;
    readonly isExecutor: boolean;
    readonly type: "Developer" | "Executor";
  }

  /** @name PalletXbiPortalCall (169) */
  interface PalletXbiPortalCall extends Enum {
    readonly isExecuteXcm: boolean;
    readonly asExecuteXcm: {
      readonly xcm: XcmV2Xcm;
    } & Struct;
    readonly isCleanup: boolean;
    readonly isEnterCall: boolean;
    readonly asEnterCall: {
      readonly checkin: PalletXbiPortalXbiFormatXbiCheckIn;
      readonly xbiId: H256;
    } & Struct;
    readonly isCheckInXbi: boolean;
    readonly asCheckInXbi: {
      readonly xbi: PalletXbiPortalXbiFormat;
    } & Struct;
    readonly type: "ExecuteXcm" | "Cleanup" | "EnterCall" | "CheckInXbi";
  }

  /** @name XcmV2Xcm (170) */
  interface XcmV2Xcm extends Vec<XcmV2Instruction> {}

  /** @name XcmV2Instruction (172) */
  interface XcmV2Instruction extends Enum {
    readonly isWithdrawAsset: boolean;
    readonly asWithdrawAsset: XcmV1MultiassetMultiAssets;
    readonly isReserveAssetDeposited: boolean;
    readonly asReserveAssetDeposited: XcmV1MultiassetMultiAssets;
    readonly isReceiveTeleportedAsset: boolean;
    readonly asReceiveTeleportedAsset: XcmV1MultiassetMultiAssets;
    readonly isQueryResponse: boolean;
    readonly asQueryResponse: {
      readonly queryId: Compact<u64>;
      readonly response: XcmV2Response;
      readonly maxWeight: Compact<u64>;
    } & Struct;
    readonly isTransferAsset: boolean;
    readonly asTransferAsset: {
      readonly assets: XcmV1MultiassetMultiAssets;
      readonly beneficiary: XcmV1MultiLocation;
    } & Struct;
    readonly isTransferReserveAsset: boolean;
    readonly asTransferReserveAsset: {
      readonly assets: XcmV1MultiassetMultiAssets;
      readonly dest: XcmV1MultiLocation;
      readonly xcm: XcmV2Xcm;
    } & Struct;
    readonly isTransact: boolean;
    readonly asTransact: {
      readonly originType: XcmV0OriginKind;
      readonly requireWeightAtMost: Compact<u64>;
      readonly call: XcmDoubleEncoded;
    } & Struct;
    readonly isHrmpNewChannelOpenRequest: boolean;
    readonly asHrmpNewChannelOpenRequest: {
      readonly sender: Compact<u32>;
      readonly maxMessageSize: Compact<u32>;
      readonly maxCapacity: Compact<u32>;
    } & Struct;
    readonly isHrmpChannelAccepted: boolean;
    readonly asHrmpChannelAccepted: {
      readonly recipient: Compact<u32>;
    } & Struct;
    readonly isHrmpChannelClosing: boolean;
    readonly asHrmpChannelClosing: {
      readonly initiator: Compact<u32>;
      readonly sender: Compact<u32>;
      readonly recipient: Compact<u32>;
    } & Struct;
    readonly isClearOrigin: boolean;
    readonly isDescendOrigin: boolean;
    readonly asDescendOrigin: XcmV1MultilocationJunctions;
    readonly isReportError: boolean;
    readonly asReportError: {
      readonly queryId: Compact<u64>;
      readonly dest: XcmV1MultiLocation;
      readonly maxResponseWeight: Compact<u64>;
    } & Struct;
    readonly isDepositAsset: boolean;
    readonly asDepositAsset: {
      readonly assets: XcmV1MultiassetMultiAssetFilter;
      readonly maxAssets: Compact<u32>;
      readonly beneficiary: XcmV1MultiLocation;
    } & Struct;
    readonly isDepositReserveAsset: boolean;
    readonly asDepositReserveAsset: {
      readonly assets: XcmV1MultiassetMultiAssetFilter;
      readonly maxAssets: Compact<u32>;
      readonly dest: XcmV1MultiLocation;
      readonly xcm: XcmV2Xcm;
    } & Struct;
    readonly isExchangeAsset: boolean;
    readonly asExchangeAsset: {
      readonly give: XcmV1MultiassetMultiAssetFilter;
      readonly receive: XcmV1MultiassetMultiAssets;
    } & Struct;
    readonly isInitiateReserveWithdraw: boolean;
    readonly asInitiateReserveWithdraw: {
      readonly assets: XcmV1MultiassetMultiAssetFilter;
      readonly reserve: XcmV1MultiLocation;
      readonly xcm: XcmV2Xcm;
    } & Struct;
    readonly isInitiateTeleport: boolean;
    readonly asInitiateTeleport: {
      readonly assets: XcmV1MultiassetMultiAssetFilter;
      readonly dest: XcmV1MultiLocation;
      readonly xcm: XcmV2Xcm;
    } & Struct;
    readonly isQueryHolding: boolean;
    readonly asQueryHolding: {
      readonly queryId: Compact<u64>;
      readonly dest: XcmV1MultiLocation;
      readonly assets: XcmV1MultiassetMultiAssetFilter;
      readonly maxResponseWeight: Compact<u64>;
    } & Struct;
    readonly isBuyExecution: boolean;
    readonly asBuyExecution: {
      readonly fees: XcmV1MultiAsset;
      readonly weightLimit: XcmV2WeightLimit;
    } & Struct;
    readonly isRefundSurplus: boolean;
    readonly isSetErrorHandler: boolean;
    readonly asSetErrorHandler: XcmV2Xcm;
    readonly isSetAppendix: boolean;
    readonly asSetAppendix: XcmV2Xcm;
    readonly isClearError: boolean;
    readonly isClaimAsset: boolean;
    readonly asClaimAsset: {
      readonly assets: XcmV1MultiassetMultiAssets;
      readonly ticket: XcmV1MultiLocation;
    } & Struct;
    readonly isTrap: boolean;
    readonly asTrap: Compact<u64>;
    readonly isSubscribeVersion: boolean;
    readonly asSubscribeVersion: {
      readonly queryId: Compact<u64>;
      readonly maxResponseWeight: Compact<u64>;
    } & Struct;
    readonly isUnsubscribeVersion: boolean;
    readonly type:
      | "WithdrawAsset"
      | "ReserveAssetDeposited"
      | "ReceiveTeleportedAsset"
      | "QueryResponse"
      | "TransferAsset"
      | "TransferReserveAsset"
      | "Transact"
      | "HrmpNewChannelOpenRequest"
      | "HrmpChannelAccepted"
      | "HrmpChannelClosing"
      | "ClearOrigin"
      | "DescendOrigin"
      | "ReportError"
      | "DepositAsset"
      | "DepositReserveAsset"
      | "ExchangeAsset"
      | "InitiateReserveWithdraw"
      | "InitiateTeleport"
      | "QueryHolding"
      | "BuyExecution"
      | "RefundSurplus"
      | "SetErrorHandler"
      | "SetAppendix"
      | "ClearError"
      | "ClaimAsset"
      | "Trap"
      | "SubscribeVersion"
      | "UnsubscribeVersion";
  }

  /** @name XcmV1MultiassetMultiAssets (173) */
  interface XcmV1MultiassetMultiAssets extends Vec<XcmV1MultiAsset> {}

  /** @name XcmV1MultiAsset (175) */
  interface XcmV1MultiAsset extends Struct {
    readonly id: XcmV1MultiassetAssetId;
    readonly fun: XcmV1MultiassetFungibility;
  }

  /** @name XcmV1MultiassetAssetId (176) */
  interface XcmV1MultiassetAssetId extends Enum {
    readonly isConcrete: boolean;
    readonly asConcrete: XcmV1MultiLocation;
    readonly isAbstract: boolean;
    readonly asAbstract: Bytes;
    readonly type: "Concrete" | "Abstract";
  }

  /** @name XcmV1MultiLocation (177) */
  interface XcmV1MultiLocation extends Struct {
    readonly parents: u8;
    readonly interior: XcmV1MultilocationJunctions;
  }

  /** @name XcmV1MultilocationJunctions (178) */
  interface XcmV1MultilocationJunctions extends Enum {
    readonly isHere: boolean;
    readonly isX1: boolean;
    readonly asX1: XcmV1Junction;
    readonly isX2: boolean;
    readonly asX2: ITuple<[XcmV1Junction, XcmV1Junction]>;
    readonly isX3: boolean;
    readonly asX3: ITuple<[XcmV1Junction, XcmV1Junction, XcmV1Junction]>;
    readonly isX4: boolean;
    readonly asX4: ITuple<
      [XcmV1Junction, XcmV1Junction, XcmV1Junction, XcmV1Junction]
    >;
    readonly isX5: boolean;
    readonly asX5: ITuple<
      [
        XcmV1Junction,
        XcmV1Junction,
        XcmV1Junction,
        XcmV1Junction,
        XcmV1Junction
      ]
    >;
    readonly isX6: boolean;
    readonly asX6: ITuple<
      [
        XcmV1Junction,
        XcmV1Junction,
        XcmV1Junction,
        XcmV1Junction,
        XcmV1Junction,
        XcmV1Junction
      ]
    >;
    readonly isX7: boolean;
    readonly asX7: ITuple<
      [
        XcmV1Junction,
        XcmV1Junction,
        XcmV1Junction,
        XcmV1Junction,
        XcmV1Junction,
        XcmV1Junction,
        XcmV1Junction
      ]
    >;
    readonly isX8: boolean;
    readonly asX8: ITuple<
      [
        XcmV1Junction,
        XcmV1Junction,
        XcmV1Junction,
        XcmV1Junction,
        XcmV1Junction,
        XcmV1Junction,
        XcmV1Junction,
        XcmV1Junction
      ]
    >;
    readonly type:
      | "Here"
      | "X1"
      | "X2"
      | "X3"
      | "X4"
      | "X5"
      | "X6"
      | "X7"
      | "X8";
  }

  /** @name XcmV1Junction (179) */
  interface XcmV1Junction extends Enum {
    readonly isParachain: boolean;
    readonly asParachain: Compact<u32>;
    readonly isAccountId32: boolean;
    readonly asAccountId32: {
      readonly network: XcmV0JunctionNetworkId;
      readonly id: U8aFixed;
    } & Struct;
    readonly isAccountIndex64: boolean;
    readonly asAccountIndex64: {
      readonly network: XcmV0JunctionNetworkId;
      readonly index: Compact<u64>;
    } & Struct;
    readonly isAccountKey20: boolean;
    readonly asAccountKey20: {
      readonly network: XcmV0JunctionNetworkId;
      readonly key: U8aFixed;
    } & Struct;
    readonly isPalletInstance: boolean;
    readonly asPalletInstance: u8;
    readonly isGeneralIndex: boolean;
    readonly asGeneralIndex: Compact<u128>;
    readonly isGeneralKey: boolean;
    readonly asGeneralKey: Bytes;
    readonly isOnlyChild: boolean;
    readonly isPlurality: boolean;
    readonly asPlurality: {
      readonly id: XcmV0JunctionBodyId;
      readonly part: XcmV0JunctionBodyPart;
    } & Struct;
    readonly type:
      | "Parachain"
      | "AccountId32"
      | "AccountIndex64"
      | "AccountKey20"
      | "PalletInstance"
      | "GeneralIndex"
      | "GeneralKey"
      | "OnlyChild"
      | "Plurality";
  }

  /** @name XcmV0JunctionNetworkId (180) */
  interface XcmV0JunctionNetworkId extends Enum {
    readonly isAny: boolean;
    readonly isNamed: boolean;
    readonly asNamed: Bytes;
    readonly isPolkadot: boolean;
    readonly isKusama: boolean;
    readonly type: "Any" | "Named" | "Polkadot" | "Kusama";
  }

  /** @name XcmV0JunctionBodyId (182) */
  interface XcmV0JunctionBodyId extends Enum {
    readonly isUnit: boolean;
    readonly isNamed: boolean;
    readonly asNamed: Bytes;
    readonly isIndex: boolean;
    readonly asIndex: Compact<u32>;
    readonly isExecutive: boolean;
    readonly isTechnical: boolean;
    readonly isLegislative: boolean;
    readonly isJudicial: boolean;
    readonly type:
      | "Unit"
      | "Named"
      | "Index"
      | "Executive"
      | "Technical"
      | "Legislative"
      | "Judicial";
  }

  /** @name XcmV0JunctionBodyPart (183) */
  interface XcmV0JunctionBodyPart extends Enum {
    readonly isVoice: boolean;
    readonly isMembers: boolean;
    readonly asMembers: {
      readonly count: Compact<u32>;
    } & Struct;
    readonly isFraction: boolean;
    readonly asFraction: {
      readonly nom: Compact<u32>;
      readonly denom: Compact<u32>;
    } & Struct;
    readonly isAtLeastProportion: boolean;
    readonly asAtLeastProportion: {
      readonly nom: Compact<u32>;
      readonly denom: Compact<u32>;
    } & Struct;
    readonly isMoreThanProportion: boolean;
    readonly asMoreThanProportion: {
      readonly nom: Compact<u32>;
      readonly denom: Compact<u32>;
    } & Struct;
    readonly type:
      | "Voice"
      | "Members"
      | "Fraction"
      | "AtLeastProportion"
      | "MoreThanProportion";
  }

  /** @name XcmV1MultiassetFungibility (184) */
  interface XcmV1MultiassetFungibility extends Enum {
    readonly isFungible: boolean;
    readonly asFungible: Compact<u128>;
    readonly isNonFungible: boolean;
    readonly asNonFungible: XcmV1MultiassetAssetInstance;
    readonly type: "Fungible" | "NonFungible";
  }

  /** @name XcmV1MultiassetAssetInstance (185) */
  interface XcmV1MultiassetAssetInstance extends Enum {
    readonly isUndefined: boolean;
    readonly isIndex: boolean;
    readonly asIndex: Compact<u128>;
    readonly isArray4: boolean;
    readonly asArray4: U8aFixed;
    readonly isArray8: boolean;
    readonly asArray8: U8aFixed;
    readonly isArray16: boolean;
    readonly asArray16: U8aFixed;
    readonly isArray32: boolean;
    readonly asArray32: U8aFixed;
    readonly isBlob: boolean;
    readonly asBlob: Bytes;
    readonly type:
      | "Undefined"
      | "Index"
      | "Array4"
      | "Array8"
      | "Array16"
      | "Array32"
      | "Blob";
  }

  /** @name XcmV2Response (187) */
  interface XcmV2Response extends Enum {
    readonly isNull: boolean;
    readonly isAssets: boolean;
    readonly asAssets: XcmV1MultiassetMultiAssets;
    readonly isExecutionResult: boolean;
    readonly asExecutionResult: Option<ITuple<[u32, XcmV2TraitsError]>>;
    readonly isVersion: boolean;
    readonly asVersion: u32;
    readonly type: "Null" | "Assets" | "ExecutionResult" | "Version";
  }

  /** @name XcmV2TraitsError (190) */
  interface XcmV2TraitsError extends Enum {
    readonly isOverflow: boolean;
    readonly isUnimplemented: boolean;
    readonly isUntrustedReserveLocation: boolean;
    readonly isUntrustedTeleportLocation: boolean;
    readonly isMultiLocationFull: boolean;
    readonly isMultiLocationNotInvertible: boolean;
    readonly isBadOrigin: boolean;
    readonly isInvalidLocation: boolean;
    readonly isAssetNotFound: boolean;
    readonly isFailedToTransactAsset: boolean;
    readonly isNotWithdrawable: boolean;
    readonly isLocationCannotHold: boolean;
    readonly isExceedsMaxMessageSize: boolean;
    readonly isDestinationUnsupported: boolean;
    readonly isTransport: boolean;
    readonly isUnroutable: boolean;
    readonly isUnknownClaim: boolean;
    readonly isFailedToDecode: boolean;
    readonly isMaxWeightInvalid: boolean;
    readonly isNotHoldingFees: boolean;
    readonly isTooExpensive: boolean;
    readonly isTrap: boolean;
    readonly asTrap: u64;
    readonly isUnhandledXcmVersion: boolean;
    readonly isWeightLimitReached: boolean;
    readonly asWeightLimitReached: u64;
    readonly isBarrier: boolean;
    readonly isWeightNotComputable: boolean;
    readonly type:
      | "Overflow"
      | "Unimplemented"
      | "UntrustedReserveLocation"
      | "UntrustedTeleportLocation"
      | "MultiLocationFull"
      | "MultiLocationNotInvertible"
      | "BadOrigin"
      | "InvalidLocation"
      | "AssetNotFound"
      | "FailedToTransactAsset"
      | "NotWithdrawable"
      | "LocationCannotHold"
      | "ExceedsMaxMessageSize"
      | "DestinationUnsupported"
      | "Transport"
      | "Unroutable"
      | "UnknownClaim"
      | "FailedToDecode"
      | "MaxWeightInvalid"
      | "NotHoldingFees"
      | "TooExpensive"
      | "Trap"
      | "UnhandledXcmVersion"
      | "WeightLimitReached"
      | "Barrier"
      | "WeightNotComputable";
  }

  /** @name XcmV0OriginKind (194) */
  interface XcmV0OriginKind extends Enum {
    readonly isNative: boolean;
    readonly isSovereignAccount: boolean;
    readonly isSuperuser: boolean;
    readonly isXcm: boolean;
    readonly type: "Native" | "SovereignAccount" | "Superuser" | "Xcm";
  }

  /** @name XcmDoubleEncoded (195) */
  interface XcmDoubleEncoded extends Struct {
    readonly encoded: Bytes;
  }

  /** @name XcmV1MultiassetMultiAssetFilter (196) */
  interface XcmV1MultiassetMultiAssetFilter extends Enum {
    readonly isDefinite: boolean;
    readonly asDefinite: XcmV1MultiassetMultiAssets;
    readonly isWild: boolean;
    readonly asWild: XcmV1MultiassetWildMultiAsset;
    readonly type: "Definite" | "Wild";
  }

  /** @name XcmV1MultiassetWildMultiAsset (197) */
  interface XcmV1MultiassetWildMultiAsset extends Enum {
    readonly isAll: boolean;
    readonly isAllOf: boolean;
    readonly asAllOf: {
      readonly id: XcmV1MultiassetAssetId;
      readonly fun: XcmV1MultiassetWildFungibility;
    } & Struct;
    readonly type: "All" | "AllOf";
  }

  /** @name XcmV1MultiassetWildFungibility (198) */
  interface XcmV1MultiassetWildFungibility extends Enum {
    readonly isFungible: boolean;
    readonly isNonFungible: boolean;
    readonly type: "Fungible" | "NonFungible";
  }

  /** @name XcmV2WeightLimit (199) */
  interface XcmV2WeightLimit extends Enum {
    readonly isUnlimited: boolean;
    readonly isLimited: boolean;
    readonly asLimited: Compact<u64>;
    readonly type: "Unlimited" | "Limited";
  }

  /** @name PalletXbiPortalXbiFormatXbiCheckIn (201) */
  interface PalletXbiPortalXbiFormatXbiCheckIn extends Struct {
    readonly xbi: PalletXbiPortalXbiFormat;
    readonly notificationDeliveryTimeout: u32;
    readonly notificationExecutionTimeout: u32;
  }

  /** @name PalletXbiPortalXbiFormat (202) */
  interface PalletXbiPortalXbiFormat extends Struct {
    readonly instr: PalletXbiPortalXbiFormatXbiInstr;
    readonly metadata: PalletXbiPortalXbiFormatXbiMetadata;
  }

  /** @name PalletXbiPortalXbiFormatXbiInstr (203) */
  interface PalletXbiPortalXbiFormatXbiInstr extends Enum {
    readonly isUnknown: boolean;
    readonly asUnknown: {
      readonly identifier: u8;
      readonly params: Bytes;
    } & Struct;
    readonly isCallNative: boolean;
    readonly asCallNative: {
      readonly payload: Bytes;
    } & Struct;
    readonly isCallEvm: boolean;
    readonly asCallEvm: {
      readonly source: H160;
      readonly target: H160;
      readonly value: U256;
      readonly input: Bytes;
      readonly gasLimit: u64;
      readonly maxFeePerGas: U256;
      readonly maxPriorityFeePerGas: Option<U256>;
      readonly nonce: Option<U256>;
      readonly accessList: Vec<ITuple<[H160, Vec<H256>]>>;
    } & Struct;
    readonly isCallWasm: boolean;
    readonly asCallWasm: {
      readonly dest: AccountId32;
      readonly value: u128;
      readonly gasLimit: u64;
      readonly storageDepositLimit: Option<u128>;
      readonly data: Bytes;
    } & Struct;
    readonly isCallCustom: boolean;
    readonly asCallCustom: {
      readonly caller: AccountId32;
      readonly dest: AccountId32;
      readonly value: u128;
      readonly input: Bytes;
      readonly limit: u64;
      readonly additionalParams: Bytes;
    } & Struct;
    readonly isTransfer: boolean;
    readonly asTransfer: {
      readonly dest: AccountId32;
      readonly value: u128;
    } & Struct;
    readonly isTransferORML: boolean;
    readonly asTransferORML: {
      readonly currencyId: u64;
      readonly dest: AccountId32;
      readonly value: u128;
    } & Struct;
    readonly isTransferAssets: boolean;
    readonly asTransferAssets: {
      readonly currencyId: u64;
      readonly dest: AccountId32;
      readonly value: u128;
    } & Struct;
    readonly isResult: boolean;
    readonly asResult: {
      readonly outcome: PalletXbiPortalXbiFormatXbiCheckOutStatus;
      readonly output: Bytes;
      readonly witness: Bytes;
    } & Struct;
    readonly isNotification: boolean;
    readonly asNotification: {
      readonly kind: PalletXbiPortalXbiFormatXbiNotificationKind;
      readonly instructionId: Bytes;
      readonly extra: Bytes;
    } & Struct;
    readonly type:
      | "Unknown"
      | "CallNative"
      | "CallEvm"
      | "CallWasm"
      | "CallCustom"
      | "Transfer"
      | "TransferORML"
      | "TransferAssets"
      | "Result"
      | "Notification";
  }

  /** @name PalletXbiPortalXbiFormatXbiMetadata (204) */
  interface PalletXbiPortalXbiFormatXbiMetadata extends Struct {
    readonly id: H256;
    readonly destParaId: u32;
    readonly srcParaId: u32;
    readonly sent: PalletXbiPortalXbiFormatActionNotificationTimeouts;
    readonly delivered: PalletXbiPortalXbiFormatActionNotificationTimeouts;
    readonly executed: PalletXbiPortalXbiFormatActionNotificationTimeouts;
    readonly maxExecCost: u128;
    readonly maxNotificationsCost: u128;
    readonly actualAggregatedCost: Option<u128>;
    readonly maybeKnownOrigin: Option<AccountId32>;
  }

  /** @name PalletXbiPortalXbiFormatActionNotificationTimeouts (205) */
  interface PalletXbiPortalXbiFormatActionNotificationTimeouts extends Struct {
    readonly action: u32;
    readonly notification: u32;
  }

  /** @name Pallet3vmCall (206) */
  type Pallet3vmCall = Null;

  /** @name PalletContractsCall (207) */
  interface PalletContractsCall extends Enum {
    readonly isCall: boolean;
    readonly asCall: {
      readonly dest: MultiAddress;
      readonly value: Compact<u128>;
      readonly gasLimit: Compact<u64>;
      readonly storageDepositLimit: Option<Compact<u128>>;
      readonly data: Bytes;
    } & Struct;
    readonly isInstantiateWithCode: boolean;
    readonly asInstantiateWithCode: {
      readonly value: Compact<u128>;
      readonly gasLimit: Compact<u64>;
      readonly storageDepositLimit: Option<Compact<u128>>;
      readonly code: Bytes;
      readonly data: Bytes;
      readonly salt: Bytes;
    } & Struct;
    readonly isInstantiate: boolean;
    readonly asInstantiate: {
      readonly value: Compact<u128>;
      readonly gasLimit: Compact<u64>;
      readonly storageDepositLimit: Option<Compact<u128>>;
      readonly codeHash: H256;
      readonly data: Bytes;
      readonly salt: Bytes;
    } & Struct;
    readonly isUploadCode: boolean;
    readonly asUploadCode: {
      readonly code: Bytes;
      readonly storageDepositLimit: Option<Compact<u128>>;
    } & Struct;
    readonly isRemoveCode: boolean;
    readonly asRemoveCode: {
      readonly codeHash: H256;
    } & Struct;
    readonly type:
      | "Call"
      | "InstantiateWithCode"
      | "Instantiate"
      | "UploadCode"
      | "RemoveCode";
  }

  /** @name PalletEvmCall (209) */
  interface PalletEvmCall extends Enum {
    readonly isWithdraw: boolean;
    readonly asWithdraw: {
      readonly address: H160;
      readonly value: u128;
    } & Struct;
    readonly isCall: boolean;
    readonly asCall: {
      readonly target: H160;
      readonly input: Bytes;
      readonly value: U256;
      readonly gasLimit: u64;
      readonly maxFeePerGas: U256;
      readonly maxPriorityFeePerGas: Option<U256>;
      readonly nonce: Option<U256>;
      readonly accessList: Vec<ITuple<[H160, Vec<H256>]>>;
    } & Struct;
    readonly isCreate: boolean;
    readonly asCreate: {
      readonly init: Bytes;
      readonly value: U256;
      readonly gasLimit: u64;
      readonly maxFeePerGas: U256;
      readonly maxPriorityFeePerGas: Option<U256>;
      readonly nonce: Option<U256>;
      readonly accessList: Vec<ITuple<[H160, Vec<H256>]>>;
    } & Struct;
    readonly isCreate2: boolean;
    readonly asCreate2: {
      readonly init: Bytes;
      readonly salt: H256;
      readonly value: U256;
      readonly gasLimit: u64;
      readonly maxFeePerGas: U256;
      readonly maxPriorityFeePerGas: Option<U256>;
      readonly nonce: Option<U256>;
      readonly accessList: Vec<ITuple<[H160, Vec<H256>]>>;
    } & Struct;
    readonly isClaim: boolean;
    readonly type: "Withdraw" | "Call" | "Create" | "Create2" | "Claim";
  }

  /** @name PalletAccountManagerCall (210) */
  interface PalletAccountManagerCall extends Enum {
    readonly isDeposit: boolean;
    readonly asDeposit: {
      readonly chargeId: H256;
      readonly payee: AccountId32;
      readonly chargeFee: u128;
      readonly offeredReward: u128;
      readonly source: T3rnPrimitivesClaimableBenefitSource;
      readonly role: T3rnPrimitivesClaimableCircuitRole;
      readonly maybeRecipient: Option<AccountId32>;
    } & Struct;
    readonly isFinalize: boolean;
    readonly asFinalize: {
      readonly chargeId: H256;
      readonly outcome: T3rnPrimitivesAccountManagerOutcome;
      readonly maybeRecipient: Option<AccountId32>;
      readonly maybeActualFees: Option<u128>;
    } & Struct;
    readonly type: "Deposit" | "Finalize";
  }

  /** @name T3rnPrimitivesClaimableBenefitSource (211) */
  interface T3rnPrimitivesClaimableBenefitSource extends Enum {
    readonly isTrafficFees: boolean;
    readonly isTrafficRewards: boolean;
    readonly isBootstrapPool: boolean;
    readonly isInflation: boolean;
    readonly type:
      | "TrafficFees"
      | "TrafficRewards"
      | "BootstrapPool"
      | "Inflation";
  }

  /** @name T3rnPrimitivesClaimableCircuitRole (212) */
  interface T3rnPrimitivesClaimableCircuitRole extends Enum {
    readonly isAmbassador: boolean;
    readonly isExecutor: boolean;
    readonly isAttester: boolean;
    readonly isStaker: boolean;
    readonly isCollator: boolean;
    readonly isContractAuthor: boolean;
    readonly isRelayer: boolean;
    readonly isRequester: boolean;
    readonly isLocal: boolean;
    readonly type:
      | "Ambassador"
      | "Executor"
      | "Attester"
      | "Staker"
      | "Collator"
      | "ContractAuthor"
      | "Relayer"
      | "Requester"
      | "Local";
  }

  /** @name T3rnPrimitivesAccountManagerOutcome (213) */
  interface T3rnPrimitivesAccountManagerOutcome extends Enum {
    readonly isUnexpectedFailure: boolean;
    readonly isRevert: boolean;
    readonly isCommit: boolean;
    readonly type: "UnexpectedFailure" | "Revert" | "Commit";
  }

  /** @name PalletPortalCall (214) */
  interface PalletPortalCall extends Enum {
    readonly isRegisterGateway: boolean;
    readonly asRegisterGateway: {
      readonly url: Bytes;
      readonly gatewayId: U8aFixed;
      readonly gatewayAbi: T3rnTypesAbiGatewayABIConfig;
      readonly gatewayVendor: T3rnPrimitivesGatewayVendor;
      readonly gatewayType: T3rnPrimitivesGatewayType;
      readonly gatewayGenesis: T3rnPrimitivesGatewayGenesisConfig;
      readonly gatewaySysProps: T3rnPrimitivesGatewaySysProps;
      readonly allowedSideEffects: Vec<U8aFixed>;
      readonly encodedRegistrationData: Bytes;
    } & Struct;
    readonly isSetOwner: boolean;
    readonly asSetOwner: {
      readonly gatewayId: U8aFixed;
      readonly encodedNewOwner: Bytes;
    } & Struct;
    readonly isSetOperational: boolean;
    readonly asSetOperational: {
      readonly gatewayId: U8aFixed;
      readonly operational: bool;
    } & Struct;
    readonly isSubmitHeaders: boolean;
    readonly asSubmitHeaders: {
      readonly gatewayId: U8aFixed;
      readonly encodedHeaderData: Bytes;
    } & Struct;
    readonly type:
      | "RegisterGateway"
      | "SetOwner"
      | "SetOperational"
      | "SubmitHeaders";
  }

  /** @name T3rnTypesAbiGatewayABIConfig (215) */
  interface T3rnTypesAbiGatewayABIConfig extends Struct {
    readonly blockNumberTypeSize: u16;
    readonly hashSize: u16;
    readonly hasher: T3rnTypesAbiHasherAlgo;
    readonly crypto: T3rnTypesAbiCryptoAlgo;
    readonly addressLength: u16;
    readonly valueTypeSize: u16;
    readonly decimals: u16;
    readonly structs: Vec<T3rnTypesAbiStructDecl>;
  }

  /** @name T3rnTypesAbiStructDecl (217) */
  interface T3rnTypesAbiStructDecl extends Struct {
    readonly name: T3rnTypesAbiType;
    readonly fields: Vec<T3rnTypesAbiParameter>;
    readonly offsets: Vec<u16>;
  }

  /** @name T3rnTypesAbiParameter (219) */
  interface T3rnTypesAbiParameter extends Struct {
    readonly name: Option<Bytes>;
    readonly ty: T3rnTypesAbiType;
    readonly no: u32;
    readonly indexed: Option<bool>;
  }

  /** @name T3rnPrimitivesGatewayVendor (222) */
  interface T3rnPrimitivesGatewayVendor extends Enum {
    readonly isPolkadot: boolean;
    readonly isKusama: boolean;
    readonly isRococo: boolean;
    readonly isEthereum: boolean;
    readonly type: "Polkadot" | "Kusama" | "Rococo" | "Ethereum";
  }

  /** @name T3rnPrimitivesGatewayType (223) */
  interface T3rnPrimitivesGatewayType extends Enum {
    readonly isProgrammableInternal: boolean;
    readonly asProgrammableInternal: u32;
    readonly isProgrammableExternal: boolean;
    readonly asProgrammableExternal: u32;
    readonly isTxOnly: boolean;
    readonly asTxOnly: u32;
    readonly isOnCircuit: boolean;
    readonly asOnCircuit: u32;
    readonly type:
      | "ProgrammableInternal"
      | "ProgrammableExternal"
      | "TxOnly"
      | "OnCircuit";
  }

  /** @name T3rnPrimitivesGatewayGenesisConfig (224) */
  interface T3rnPrimitivesGatewayGenesisConfig extends Struct {
    readonly modulesEncoded: Option<Bytes>;
    readonly extrinsicsVersion: u8;
    readonly genesisHash: Bytes;
  }

  /** @name T3rnPrimitivesGatewaySysProps (225) */
  interface T3rnPrimitivesGatewaySysProps extends Struct {
    readonly ss58Format: u16;
    readonly tokenSymbol: Bytes;
    readonly tokenDecimals: u8;
  }

  /** @name PalletSudoError (227) */
  interface PalletSudoError extends Enum {
    readonly isRequireSudo: boolean;
    readonly type: "RequireSudo";
  }

  /** @name PalletUtilityError (228) */
  interface PalletUtilityError extends Enum {
    readonly isTooManyCalls: boolean;
    readonly type: "TooManyCalls";
  }

  /** @name PalletBalancesBalanceLock (230) */
  interface PalletBalancesBalanceLock extends Struct {
    readonly id: U8aFixed;
    readonly amount: u128;
    readonly reasons: PalletBalancesReasons;
  }

  /** @name PalletBalancesReasons (231) */
  interface PalletBalancesReasons extends Enum {
    readonly isFee: boolean;
    readonly isMisc: boolean;
    readonly isAll: boolean;
    readonly type: "Fee" | "Misc" | "All";
  }

  /** @name PalletBalancesReserveData (234) */
  interface PalletBalancesReserveData extends Struct {
    readonly id: U8aFixed;
    readonly amount: u128;
  }

  /** @name PalletBalancesReleases (236) */
  interface PalletBalancesReleases extends Enum {
    readonly isV100: boolean;
    readonly isV200: boolean;
    readonly type: "V100" | "V200";
  }

  /** @name PalletBalancesError (237) */
  interface PalletBalancesError extends Enum {
    readonly isVestingBalance: boolean;
    readonly isLiquidityRestrictions: boolean;
    readonly isInsufficientBalance: boolean;
    readonly isExistentialDeposit: boolean;
    readonly isKeepAlive: boolean;
    readonly isExistingVestingSchedule: boolean;
    readonly isDeadAccount: boolean;
    readonly isTooManyReserves: boolean;
    readonly type:
      | "VestingBalance"
      | "LiquidityRestrictions"
      | "InsufficientBalance"
      | "ExistentialDeposit"
      | "KeepAlive"
      | "ExistingVestingSchedule"
      | "DeadAccount"
      | "TooManyReserves";
  }

  /** @name PalletTransactionPaymentReleases (239) */
  interface PalletTransactionPaymentReleases extends Enum {
    readonly isV1Ancient: boolean;
    readonly isV2: boolean;
    readonly type: "V1Ancient" | "V2";
  }

  /** @name PalletAssetsAssetDetails (240) */
  interface PalletAssetsAssetDetails extends Struct {
    readonly owner: AccountId32;
    readonly issuer: AccountId32;
    readonly admin: AccountId32;
    readonly freezer: AccountId32;
    readonly supply: u128;
    readonly deposit: u128;
    readonly minBalance: u128;
    readonly isSufficient: bool;
    readonly accounts: u32;
    readonly sufficients: u32;
    readonly approvals: u32;
    readonly isFrozen: bool;
  }

  /** @name PalletAssetsAssetAccount (242) */
  interface PalletAssetsAssetAccount extends Struct {
    readonly balance: u128;
    readonly isFrozen: bool;
    readonly reason: PalletAssetsExistenceReason;
    readonly extra: Null;
  }

  /** @name PalletAssetsExistenceReason (243) */
  interface PalletAssetsExistenceReason extends Enum {
    readonly isConsumer: boolean;
    readonly isSufficient: boolean;
    readonly isDepositHeld: boolean;
    readonly asDepositHeld: u128;
    readonly isDepositRefunded: boolean;
    readonly type:
      | "Consumer"
      | "Sufficient"
      | "DepositHeld"
      | "DepositRefunded";
  }

  /** @name PalletAssetsApproval (245) */
  interface PalletAssetsApproval extends Struct {
    readonly amount: u128;
    readonly deposit: u128;
  }

  /** @name PalletAssetsAssetMetadata (246) */
  interface PalletAssetsAssetMetadata extends Struct {
    readonly deposit: u128;
    readonly name: Bytes;
    readonly symbol: Bytes;
    readonly decimals: u8;
    readonly isFrozen: bool;
  }

  /** @name PalletAssetsError (248) */
  interface PalletAssetsError extends Enum {
    readonly isBalanceLow: boolean;
    readonly isNoAccount: boolean;
    readonly isNoPermission: boolean;
    readonly isUnknown: boolean;
    readonly isFrozen: boolean;
    readonly isInUse: boolean;
    readonly isBadWitness: boolean;
    readonly isMinBalanceZero: boolean;
    readonly isNoProvider: boolean;
    readonly isBadMetadata: boolean;
    readonly isUnapproved: boolean;
    readonly isWouldDie: boolean;
    readonly isAlreadyExists: boolean;
    readonly isNoDeposit: boolean;
    readonly isWouldBurn: boolean;
    readonly type:
      | "BalanceLow"
      | "NoAccount"
      | "NoPermission"
      | "Unknown"
      | "Frozen"
      | "InUse"
      | "BadWitness"
      | "MinBalanceZero"
      | "NoProvider"
      | "BadMetadata"
      | "Unapproved"
      | "WouldDie"
      | "AlreadyExists"
      | "NoDeposit"
      | "WouldBurn";
  }

  /** @name T3rnPrimitivesSideEffectInterfaceSideEffectInterface (249) */
  interface T3rnPrimitivesSideEffectInterfaceSideEffectInterface
    extends Struct {
    readonly id: U8aFixed;
    readonly name: Bytes;
    readonly argumentAbi: Vec<T3rnTypesAbiType>;
    readonly argumentToStateMapper: Vec<Bytes>;
    readonly confirmEvents: Vec<Bytes>;
    readonly escrowedEvents: Vec<Bytes>;
    readonly commitEvents: Vec<Bytes>;
    readonly revertEvents: Vec<Bytes>;
  }

  /** @name T3rnPrimitivesXdnsXdnsRecord (250) */
  interface T3rnPrimitivesXdnsXdnsRecord extends Struct {
    readonly url: Bytes;
    readonly gatewayAbi: T3rnTypesAbiGatewayABIConfig;
    readonly gatewayGenesis: T3rnPrimitivesGatewayGenesisConfig;
    readonly gatewayVendor: T3rnPrimitivesGatewayVendor;
    readonly gatewayType: T3rnPrimitivesGatewayType;
    readonly gatewayId: U8aFixed;
    readonly parachain: Option<T3rnPrimitivesXdnsParachain>;
    readonly gatewaySysProps: T3rnPrimitivesGatewaySysProps;
    readonly registrant: Option<AccountId32>;
    readonly securityCoordinates: Bytes;
    readonly lastFinalized: Option<u64>;
    readonly allowedSideEffects: Vec<U8aFixed>;
  }

  /** @name T3rnPrimitivesXdnsParachain (252) */
  interface T3rnPrimitivesXdnsParachain extends Struct {
    readonly relayChainId: U8aFixed;
    readonly id: u32;
  }

  /** @name PalletXdnsError (253) */
  interface PalletXdnsError extends Enum {
    readonly isXdnsRecordAlreadyExists: boolean;
    readonly isUnknownXdnsRecord: boolean;
    readonly isXdnsRecordNotFound: boolean;
    readonly isSideEffectInterfaceAlreadyExists: boolean;
    readonly isSideEffectInterfaceNotFound: boolean;
    readonly isNoParachainInfoFound: boolean;
    readonly type:
      | "XdnsRecordAlreadyExists"
      | "UnknownXdnsRecord"
      | "XdnsRecordNotFound"
      | "SideEffectInterfaceAlreadyExists"
      | "SideEffectInterfaceNotFound"
      | "NoParachainInfoFound";
  }

  /** @name PalletContractsRegistryError (254) */
  interface PalletContractsRegistryError extends Enum {
    readonly isContractAlreadyExists: boolean;
    readonly isUnknownContract: boolean;
    readonly type: "ContractAlreadyExists" | "UnknownContract";
  }

  /** @name PalletCircuitStateXExecSignal (256) */
  interface PalletCircuitStateXExecSignal extends Struct {
    readonly requester: AccountId32;
    readonly requesterNonce: u32;
    readonly timeoutsAt: u32;
    readonly delayStepsAt: Option<Vec<u32>>;
    readonly status: PalletCircuitStateCircuitStatus;
    readonly stepsCnt: ITuple<[u32, u32]>;
    readonly totalReward: Option<u128>;
  }

  /** @name PalletCircuitStateCircuitStatus (259) */
  interface PalletCircuitStateCircuitStatus extends Enum {
    readonly isRequested: boolean;
    readonly isPendingBidding: boolean;
    readonly isReady: boolean;
    readonly isPendingExecution: boolean;
    readonly isFinished: boolean;
    readonly isFinishedAllSteps: boolean;
    readonly isCommitted: boolean;
    readonly isDroppedAtBidding: boolean;
    readonly isReverted: boolean;
    readonly isRevertTimedOut: boolean;
    readonly isRevertKill: boolean;
    readonly isRevertMisbehaviour: boolean;
    readonly type:
      | "Requested"
      | "PendingBidding"
      | "Ready"
      | "PendingExecution"
      | "Finished"
      | "FinishedAllSteps"
      | "Committed"
      | "DroppedAtBidding"
      | "Reverted"
      | "RevertTimedOut"
      | "RevertKill"
      | "RevertMisbehaviour";
  }

  /** @name T3rnPrimitivesVolatileLocalState (260) */
  interface T3rnPrimitivesVolatileLocalState extends Struct {
    readonly state: BTreeMap<U8aFixed, Bytes>;
  }

  /** @name T3rnSdkPrimitivesSignalExecutionSignal (266) */
  interface T3rnSdkPrimitivesSignalExecutionSignal extends Struct {
    readonly step: u32;
    readonly kind: T3rnSdkPrimitivesSignalSignalKind;
    readonly executionId: H256;
  }

  /** @name PalletCircuitError (268) */
  interface PalletCircuitError extends Enum {
    readonly isUpdateXtxTriggeredWithUnexpectedStatus: boolean;
    readonly isApplyTriggeredWithUnexpectedStatus: boolean;
    readonly isRequesterNotEnoughBalance: boolean;
    readonly isContractXtxKilledRunOutOfFunds: boolean;
    readonly isChargingTransferFailed: boolean;
    readonly isChargingTransferFailedAtPendingExecution: boolean;
    readonly isXtxChargeFailedRequesterBalanceTooLow: boolean;
    readonly isXtxChargeBondDepositFailedCantAccessBid: boolean;
    readonly isFinalizeSquareUpFailed: boolean;
    readonly isCriticalStateSquareUpCalledToFinishWithoutFsxConfirmed: boolean;
    readonly isRewardTransferFailed: boolean;
    readonly isRefundTransferFailed: boolean;
    readonly isSideEffectsValidationFailed: boolean;
    readonly isInsuranceBondNotRequired: boolean;
    readonly isBiddingInactive: boolean;
    readonly isBiddingRejectedBidBelowDust: boolean;
    readonly isBiddingRejectedExecutorNotEnoughBalance: boolean;
    readonly isBiddingRejectedBidTooHigh: boolean;
    readonly isBiddingRejectedBetterBidFound: boolean;
    readonly isBiddingFailedExecutorsBalanceTooLowToReserve: boolean;
    readonly isInsuranceBondAlreadyDeposited: boolean;
    readonly isSetupFailed: boolean;
    readonly isSetupFailedXtxNotFound: boolean;
    readonly isSetupFailedXtxStorageArtifactsNotFound: boolean;
    readonly isSetupFailedIncorrectXtxStatus: boolean;
    readonly isSetupFailedDuplicatedXtx: boolean;
    readonly isSetupFailedEmptyXtx: boolean;
    readonly isSetupFailedXtxAlreadyFinished: boolean;
    readonly isSetupFailedXtxWasDroppedAtBidding: boolean;
    readonly isSetupFailedXtxReverted: boolean;
    readonly isSetupFailedXtxRevertedTimeout: boolean;
    readonly isSetupFailedUnknownXtx: boolean;
    readonly isInvalidFSXBidStateLocated: boolean;
    readonly isEnactSideEffectsCanOnlyBeCalledWithMin1StepFinished: boolean;
    readonly isFatalXtxTimeoutXtxIdNotMatched: boolean;
    readonly isRelayEscrowedFailedNothingToConfirm: boolean;
    readonly isFatalCommitSideEffectWithoutConfirmationAttempt: boolean;
    readonly isFatalErroredCommitSideEffectConfirmationAttempt: boolean;
    readonly isFatalErroredRevertSideEffectConfirmationAttempt: boolean;
    readonly isFailedToHardenFullSideEffect: boolean;
    readonly isApplyFailed: boolean;
    readonly isDeterminedForbiddenXtxStatus: boolean;
    readonly isSideEffectIsAlreadyScheduledToExecuteOverXBI: boolean;
    readonly isLocalSideEffectExecutionNotApplicable: boolean;
    readonly isLocalExecutionUnauthorized: boolean;
    readonly isFailedToConvertSFX2XBI: boolean;
    readonly isFailedToCheckInOverXBI: boolean;
    readonly isFailedToCreateXBIMetadataDueToWrongAccountConversion: boolean;
    readonly isFailedToConvertXBIResult2SFXConfirmation: boolean;
    readonly isFailedToEnterXBIPortal: boolean;
    readonly isFailedToExitXBIPortal: boolean;
    readonly isXbiExitFailedOnSFXConfirmation: boolean;
    readonly isUnsupportedRole: boolean;
    readonly isInvalidLocalTrigger: boolean;
    readonly isSignalQueueFull: boolean;
    readonly type:
      | "UpdateXtxTriggeredWithUnexpectedStatus"
      | "ApplyTriggeredWithUnexpectedStatus"
      | "RequesterNotEnoughBalance"
      | "ContractXtxKilledRunOutOfFunds"
      | "ChargingTransferFailed"
      | "ChargingTransferFailedAtPendingExecution"
      | "XtxChargeFailedRequesterBalanceTooLow"
      | "XtxChargeBondDepositFailedCantAccessBid"
      | "FinalizeSquareUpFailed"
      | "CriticalStateSquareUpCalledToFinishWithoutFsxConfirmed"
      | "RewardTransferFailed"
      | "RefundTransferFailed"
      | "SideEffectsValidationFailed"
      | "InsuranceBondNotRequired"
      | "BiddingInactive"
      | "BiddingRejectedBidBelowDust"
      | "BiddingRejectedExecutorNotEnoughBalance"
      | "BiddingRejectedBidTooHigh"
      | "BiddingRejectedBetterBidFound"
      | "BiddingFailedExecutorsBalanceTooLowToReserve"
      | "InsuranceBondAlreadyDeposited"
      | "SetupFailed"
      | "SetupFailedXtxNotFound"
      | "SetupFailedXtxStorageArtifactsNotFound"
      | "SetupFailedIncorrectXtxStatus"
      | "SetupFailedDuplicatedXtx"
      | "SetupFailedEmptyXtx"
      | "SetupFailedXtxAlreadyFinished"
      | "SetupFailedXtxWasDroppedAtBidding"
      | "SetupFailedXtxReverted"
      | "SetupFailedXtxRevertedTimeout"
      | "SetupFailedUnknownXtx"
      | "InvalidFSXBidStateLocated"
      | "EnactSideEffectsCanOnlyBeCalledWithMin1StepFinished"
      | "FatalXtxTimeoutXtxIdNotMatched"
      | "RelayEscrowedFailedNothingToConfirm"
      | "FatalCommitSideEffectWithoutConfirmationAttempt"
      | "FatalErroredCommitSideEffectConfirmationAttempt"
      | "FatalErroredRevertSideEffectConfirmationAttempt"
      | "FailedToHardenFullSideEffect"
      | "ApplyFailed"
      | "DeterminedForbiddenXtxStatus"
      | "SideEffectIsAlreadyScheduledToExecuteOverXBI"
      | "LocalSideEffectExecutionNotApplicable"
      | "LocalExecutionUnauthorized"
      | "FailedToConvertSFX2XBI"
      | "FailedToCheckInOverXBI"
      | "FailedToCreateXBIMetadataDueToWrongAccountConversion"
      | "FailedToConvertXBIResult2SFXConfirmation"
      | "FailedToEnterXBIPortal"
      | "FailedToExitXBIPortal"
      | "XbiExitFailedOnSFXConfirmation"
      | "UnsupportedRole"
      | "InvalidLocalTrigger"
      | "SignalQueueFull";
  }

  /** @name PalletTreasuryInflationInflationInfo (269) */
  interface PalletTreasuryInflationInflationInfo extends Struct {
    readonly annual: {
      readonly min: Perbill;
      readonly ideal: Perbill;
      readonly max: Perbill;
    } & Struct;
    readonly round: {
      readonly min: Perbill;
      readonly ideal: Perbill;
      readonly max: Perbill;
    } & Struct;
  }

  /** @name T3rnPrimitivesCommonRoundInfo (270) */
  interface T3rnPrimitivesCommonRoundInfo extends Struct {
    readonly index: u32;
    readonly head: u32;
    readonly term: u32;
  }

  /** @name PalletTreasuryError (273) */
  interface PalletTreasuryError extends Enum {
    readonly isInvalidInflationConfig: boolean;
    readonly isInvalidInflationAllocation: boolean;
    readonly isValueNotChanged: boolean;
    readonly isRoundTermTooShort: boolean;
    readonly isNotBeneficiary: boolean;
    readonly isNoRewardsAvailable: boolean;
    readonly type:
      | "InvalidInflationConfig"
      | "InvalidInflationAllocation"
      | "ValueNotChanged"
      | "RoundTermTooShort"
      | "NotBeneficiary"
      | "NoRewardsAvailable";
  }

  /** @name T3rnPrimitivesClaimableClaimableArtifacts (275) */
  interface T3rnPrimitivesClaimableClaimableArtifacts extends Struct {
    readonly beneficiary: AccountId32;
    readonly role: T3rnPrimitivesClaimableCircuitRole;
    readonly totalRoundClaim: u128;
    readonly benefitSource: T3rnPrimitivesClaimableBenefitSource;
  }

  /** @name PalletClockError (276) */
  type PalletClockError = Null;

  /** @name PalletXbiPortalXbiFormatXbiCheckOut (277) */
  interface PalletXbiPortalXbiFormatXbiCheckOut extends Struct {
    readonly xbi: PalletXbiPortalXbiFormatXbiInstr;
    readonly resolutionStatus: PalletXbiPortalXbiFormatXbiCheckOutStatus;
    readonly checkoutTimeout: u32;
    readonly actualExecutionCost: u128;
    readonly actualDeliveryCost: u128;
    readonly actualAggregatedCost: u128;
  }

  /** @name PalletXbiPortalError (278) */
  interface PalletXbiPortalError extends Enum {
    readonly isEnterFailedOnXcmSend: boolean;
    readonly isEnterFailedOnMultiLocationTransform: boolean;
    readonly isExitUnhandled: boolean;
    readonly isXbiabiFailedToCastBetweenTypesValue: boolean;
    readonly isXbiabiFailedToCastBetweenTypesAddress: boolean;
    readonly isXbiInstructionNotAllowedHere: boolean;
    readonly isXbiAlreadyCheckedIn: boolean;
    readonly isXbiNotificationTimeOutDelivery: boolean;
    readonly isXbiNotificationTimeOutExecution: boolean;
    readonly isNoXBICallbackSupported: boolean;
    readonly isNoEVMSupportedAtDest: boolean;
    readonly isNoWASMSupportedAtDest: boolean;
    readonly isNo3VMSupportedAtDest: boolean;
    readonly isNoTransferSupportedAtDest: boolean;
    readonly isNoTransferAssetsSupportedAtDest: boolean;
    readonly isNoTransferORMLSupportedAtDest: boolean;
    readonly isNoTransferEscrowSupportedAtDest: boolean;
    readonly isNoTransferMultiEscrowSupportedAtDest: boolean;
    readonly isNoSwapSupportedAtDest: boolean;
    readonly isNoAddLiquiditySupportedAtDest: boolean;
    readonly type:
      | "EnterFailedOnXcmSend"
      | "EnterFailedOnMultiLocationTransform"
      | "ExitUnhandled"
      | "XbiabiFailedToCastBetweenTypesValue"
      | "XbiabiFailedToCastBetweenTypesAddress"
      | "XbiInstructionNotAllowedHere"
      | "XbiAlreadyCheckedIn"
      | "XbiNotificationTimeOutDelivery"
      | "XbiNotificationTimeOutExecution"
      | "NoXBICallbackSupported"
      | "NoEVMSupportedAtDest"
      | "NoWASMSupportedAtDest"
      | "No3VMSupportedAtDest"
      | "NoTransferSupportedAtDest"
      | "NoTransferAssetsSupportedAtDest"
      | "NoTransferORMLSupportedAtDest"
      | "NoTransferEscrowSupportedAtDest"
      | "NoTransferMultiEscrowSupportedAtDest"
      | "NoSwapSupportedAtDest"
      | "NoAddLiquiditySupportedAtDest";
  }

  /** @name Pallet3vmError (280) */
  interface Pallet3vmError extends Enum {
    readonly isExceededSignalBounceThreshold: boolean;
    readonly isCannotTriggerWithoutSideEffects: boolean;
    readonly isContractNotFound: boolean;
    readonly isInvalidOrigin: boolean;
    readonly isCannotInstantiateContract: boolean;
    readonly isContractCannotRemunerate: boolean;
    readonly isContractCannotHaveStorage: boolean;
    readonly isContractCannotGenerateSideEffects: boolean;
    readonly isInvalidPrecompilePointer: boolean;
    readonly isInvalidPrecompileArgs: boolean;
    readonly type:
      | "ExceededSignalBounceThreshold"
      | "CannotTriggerWithoutSideEffects"
      | "ContractNotFound"
      | "InvalidOrigin"
      | "CannotInstantiateContract"
      | "ContractCannotRemunerate"
      | "ContractCannotHaveStorage"
      | "ContractCannotGenerateSideEffects"
      | "InvalidPrecompilePointer"
      | "InvalidPrecompileArgs";
  }

  /** @name PalletContractsWasmPrefabWasmModule (281) */
  interface PalletContractsWasmPrefabWasmModule extends Struct {
    readonly instructionWeightsVersion: Compact<u32>;
    readonly initial: Compact<u32>;
    readonly maximum: Compact<u32>;
    readonly code: Bytes;
    readonly author: Option<T3rnPrimitivesContractsRegistryAuthorInfo>;
    readonly kind: T3rnPrimitivesContractMetadataContractType;
  }

  /** @name PalletContractsWasmOwnerInfo (283) */
  interface PalletContractsWasmOwnerInfo extends Struct {
    readonly owner: AccountId32;
    readonly deposit: Compact<u128>;
    readonly refcount: Compact<u64>;
  }

  /** @name PalletContractsStorageRawContractInfo (284) */
  interface PalletContractsStorageRawContractInfo extends Struct {
    readonly trieId: Bytes;
    readonly codeHash: H256;
    readonly storageDeposit: u128;
  }

  /** @name PalletContractsStorageDeletedContract (286) */
  interface PalletContractsStorageDeletedContract extends Struct {
    readonly trieId: Bytes;
  }

  /** @name PalletContractsSchedule (287) */
  interface PalletContractsSchedule extends Struct {
    readonly limits: PalletContractsScheduleLimits;
    readonly instructionWeights: PalletContractsScheduleInstructionWeights;
    readonly hostFnWeights: PalletContractsScheduleHostFnWeights;
  }

  /** @name PalletContractsScheduleLimits (288) */
  interface PalletContractsScheduleLimits extends Struct {
    readonly eventTopics: u32;
    readonly stackHeight: Option<u32>;
    readonly globals: u32;
    readonly parameters: u32;
    readonly memoryPages: u32;
    readonly tableSize: u32;
    readonly brTableSize: u32;
    readonly subjectLen: u32;
    readonly callDepth: u32;
    readonly payloadLen: u32;
    readonly codeLen: u32;
  }

  /** @name PalletContractsScheduleInstructionWeights (289) */
  interface PalletContractsScheduleInstructionWeights extends Struct {
    readonly version: u32;
    readonly i64const: u32;
    readonly i64load: u32;
    readonly i64store: u32;
    readonly select: u32;
    readonly r_if: u32;
    readonly br: u32;
    readonly brIf: u32;
    readonly brTable: u32;
    readonly brTablePerEntry: u32;
    readonly call: u32;
    readonly callIndirect: u32;
    readonly callIndirectPerParam: u32;
    readonly localGet: u32;
    readonly localSet: u32;
    readonly localTee: u32;
    readonly globalGet: u32;
    readonly globalSet: u32;
    readonly memoryCurrent: u32;
    readonly memoryGrow: u32;
    readonly i64clz: u32;
    readonly i64ctz: u32;
    readonly i64popcnt: u32;
    readonly i64eqz: u32;
    readonly i64extendsi32: u32;
    readonly i64extendui32: u32;
    readonly i32wrapi64: u32;
    readonly i64eq: u32;
    readonly i64ne: u32;
    readonly i64lts: u32;
    readonly i64ltu: u32;
    readonly i64gts: u32;
    readonly i64gtu: u32;
    readonly i64les: u32;
    readonly i64leu: u32;
    readonly i64ges: u32;
    readonly i64geu: u32;
    readonly i64add: u32;
    readonly i64sub: u32;
    readonly i64mul: u32;
    readonly i64divs: u32;
    readonly i64divu: u32;
    readonly i64rems: u32;
    readonly i64remu: u32;
    readonly i64and: u32;
    readonly i64or: u32;
    readonly i64xor: u32;
    readonly i64shl: u32;
    readonly i64shrs: u32;
    readonly i64shru: u32;
    readonly i64rotl: u32;
    readonly i64rotr: u32;
  }

  /** @name PalletContractsScheduleHostFnWeights (290) */
  interface PalletContractsScheduleHostFnWeights extends Struct {
    readonly caller: u64;
    readonly isContract: u64;
    readonly codeHash: u64;
    readonly ownCodeHash: u64;
    readonly callerIsOrigin: u64;
    readonly address: u64;
    readonly gasLeft: u64;
    readonly balance: u64;
    readonly valueTransferred: u64;
    readonly minimumBalance: u64;
    readonly blockNumber: u64;
    readonly now: u64;
    readonly weightToFee: u64;
    readonly gas: u64;
    readonly input: u64;
    readonly inputPerByte: u64;
    readonly r_return: u64;
    readonly returnPerByte: u64;
    readonly terminate: u64;
    readonly random: u64;
    readonly depositEvent: u64;
    readonly depositEventPerTopic: u64;
    readonly depositEventPerByte: u64;
    readonly debugMessage: u64;
    readonly setStorage: u64;
    readonly setStoragePerNewByte: u64;
    readonly setStoragePerOldByte: u64;
    readonly setCodeHash: u64;
    readonly clearStorage: u64;
    readonly clearStoragePerByte: u64;
    readonly containsStorage: u64;
    readonly containsStoragePerByte: u64;
    readonly getStorage: u64;
    readonly getStoragePerByte: u64;
    readonly takeStorage: u64;
    readonly takeStoragePerByte: u64;
    readonly transfer: u64;
    readonly call: u64;
    readonly delegateCall: u64;
    readonly callTransferSurcharge: u64;
    readonly callPerClonedByte: u64;
    readonly instantiate: u64;
    readonly instantiateTransferSurcharge: u64;
    readonly instantiatePerSaltByte: u64;
    readonly hashSha2256: u64;
    readonly hashSha2256PerByte: u64;
    readonly hashKeccak256: u64;
    readonly hashKeccak256PerByte: u64;
    readonly hashBlake2256: u64;
    readonly hashBlake2256PerByte: u64;
    readonly hashBlake2128: u64;
    readonly hashBlake2128PerByte: u64;
    readonly ecdsaRecover: u64;
  }

  /** @name PalletContractsError (291) */
  interface PalletContractsError extends Enum {
    readonly isInvalidScheduleVersion: boolean;
    readonly isInvalidCallFlags: boolean;
    readonly isOutOfGas: boolean;
    readonly isOutputBufferTooSmall: boolean;
    readonly isTransferFailed: boolean;
    readonly isMaxCallDepthReached: boolean;
    readonly isContractNotFound: boolean;
    readonly isCodeTooLarge: boolean;
    readonly isCodeNotFound: boolean;
    readonly isOutOfBounds: boolean;
    readonly isDecodingFailed: boolean;
    readonly isContractTrapped: boolean;
    readonly isValueTooLarge: boolean;
    readonly isTerminatedWhileReentrant: boolean;
    readonly isInputForwarded: boolean;
    readonly isRandomSubjectTooLong: boolean;
    readonly isTooManyTopics: boolean;
    readonly isDuplicateTopics: boolean;
    readonly isNoChainExtension: boolean;
    readonly isDeletionQueueFull: boolean;
    readonly isDuplicateContract: boolean;
    readonly isTerminatedInConstructor: boolean;
    readonly isDebugMessageInvalidUTF8: boolean;
    readonly isReentranceDenied: boolean;
    readonly isStorageDepositNotEnoughFunds: boolean;
    readonly isStorageDepositLimitExhausted: boolean;
    readonly isCodeInUse: boolean;
    readonly isContractReverted: boolean;
    readonly isCodeRejected: boolean;
    readonly isNoStateReturned: boolean;
    readonly type:
      | "InvalidScheduleVersion"
      | "InvalidCallFlags"
      | "OutOfGas"
      | "OutputBufferTooSmall"
      | "TransferFailed"
      | "MaxCallDepthReached"
      | "ContractNotFound"
      | "CodeTooLarge"
      | "CodeNotFound"
      | "OutOfBounds"
      | "DecodingFailed"
      | "ContractTrapped"
      | "ValueTooLarge"
      | "TerminatedWhileReentrant"
      | "InputForwarded"
      | "RandomSubjectTooLong"
      | "TooManyTopics"
      | "DuplicateTopics"
      | "NoChainExtension"
      | "DeletionQueueFull"
      | "DuplicateContract"
      | "TerminatedInConstructor"
      | "DebugMessageInvalidUTF8"
      | "ReentranceDenied"
      | "StorageDepositNotEnoughFunds"
      | "StorageDepositLimitExhausted"
      | "CodeInUse"
      | "ContractReverted"
      | "CodeRejected"
      | "NoStateReturned";
  }

  /** @name PalletEvmThreeVmInfo (293) */
  interface PalletEvmThreeVmInfo extends Struct {
    readonly author: T3rnPrimitivesContractsRegistryAuthorInfo;
    readonly kind: T3rnPrimitivesContractMetadataContractType;
  }

  /** @name PalletEvmError (294) */
  interface PalletEvmError extends Enum {
    readonly isBalanceLow: boolean;
    readonly isFeeOverflow: boolean;
    readonly isPaymentOverflow: boolean;
    readonly isWithdrawFailed: boolean;
    readonly isGasPriceTooLow: boolean;
    readonly isInvalidNonce: boolean;
    readonly isInvalidRegistryHash: boolean;
    readonly isRemunerateAuthor: boolean;
    readonly isExecutedFailed: boolean;
    readonly isCreatedFailed: boolean;
    readonly type:
      | "BalanceLow"
      | "FeeOverflow"
      | "PaymentOverflow"
      | "WithdrawFailed"
      | "GasPriceTooLow"
      | "InvalidNonce"
      | "InvalidRegistryHash"
      | "RemunerateAuthor"
      | "ExecutedFailed"
      | "CreatedFailed";
  }

  /** @name T3rnPrimitivesAccountManagerRequestCharge (296) */
  interface T3rnPrimitivesAccountManagerRequestCharge extends Struct {
    readonly payee: AccountId32;
    readonly offeredReward: u128;
    readonly chargeFee: u128;
    readonly recipient: AccountId32;
    readonly source: T3rnPrimitivesClaimableBenefitSource;
    readonly role: T3rnPrimitivesClaimableCircuitRole;
  }

  /** @name T3rnPrimitivesAccountManagerSettlement (297) */
  interface T3rnPrimitivesAccountManagerSettlement extends Struct {
    readonly requester: AccountId32;
    readonly recipient: AccountId32;
    readonly settlementAmount: u128;
    readonly outcome: T3rnPrimitivesAccountManagerOutcome;
    readonly source: T3rnPrimitivesClaimableBenefitSource;
    readonly role: T3rnPrimitivesClaimableCircuitRole;
  }

  /** @name PalletAccountManagerError (298) */
  interface PalletAccountManagerError extends Enum {
    readonly isPendingChargeNotFoundAtCommit: boolean;
    readonly isPendingChargeNotFoundAtRefund: boolean;
    readonly isExecutionNotRegistered: boolean;
    readonly isExecutionAlreadyRegistered: boolean;
    readonly isSkippingEmptyCharges: boolean;
    readonly isNoChargeOfGivenIdRegistered: boolean;
    readonly isChargeAlreadyRegistered: boolean;
    readonly isChargeOrSettlementCalculationOverflow: boolean;
    readonly isDecodingExecutionIDFailed: boolean;
    readonly type:
      | "PendingChargeNotFoundAtCommit"
      | "PendingChargeNotFoundAtRefund"
      | "ExecutionNotRegistered"
      | "ExecutionAlreadyRegistered"
      | "SkippingEmptyCharges"
      | "NoChargeOfGivenIdRegistered"
      | "ChargeAlreadyRegistered"
      | "ChargeOrSettlementCalculationOverflow"
      | "DecodingExecutionIDFailed";
  }

  /** @name PalletPortalError (299) */
  interface PalletPortalError extends Enum {
    readonly isXdnsRecordCreationFailed: boolean;
    readonly isUnimplementedGatewayVendor: boolean;
    readonly isRegistrationError: boolean;
    readonly isGatewayVendorNotFound: boolean;
    readonly isSetOwnerError: boolean;
    readonly isSetOperationalError: boolean;
    readonly isSubmitHeaderError: boolean;
    readonly isNoGatewayHeightAvailable: boolean;
    readonly isSideEffectConfirmationFailed: boolean;
    readonly type:
      | "XdnsRecordCreationFailed"
      | "UnimplementedGatewayVendor"
      | "RegistrationError"
      | "GatewayVendorNotFound"
      | "SetOwnerError"
      | "SetOperationalError"
      | "SubmitHeaderError"
      | "NoGatewayHeightAvailable"
      | "SideEffectConfirmationFailed";
  }

  /** @name SpRuntimeHeader (302) */
  interface SpRuntimeHeader extends Struct {
    readonly parentHash: H256;
    readonly number: Compact<u32>;
    readonly stateRoot: H256;
    readonly extrinsicsRoot: H256;
    readonly digest: SpRuntimeDigest;
  }

  /** @name SpRuntimeBlakeTwo256 (303) */
  type SpRuntimeBlakeTwo256 = Null;

  /** @name PalletGrandpaFinalityVerifierBridgesHeaderChainAuthoritySet (304) */
  interface PalletGrandpaFinalityVerifierBridgesHeaderChainAuthoritySet
    extends Struct {
    readonly authorities: Vec<ITuple<[SpFinalityGrandpaAppPublic, u64]>>;
    readonly setId: u64;
  }

  /** @name PalletGrandpaFinalityVerifierParachain (305) */
  interface PalletGrandpaFinalityVerifierParachain extends Struct {
    readonly relayChainId: U8aFixed;
    readonly id: u32;
  }

  /** @name PalletGrandpaFinalityVerifierError (306) */
  interface PalletGrandpaFinalityVerifierError extends Enum {
    readonly isEmptyRangeSubmitted: boolean;
    readonly isRangeToLarge: boolean;
    readonly isNoFinalizedHeader: boolean;
    readonly isInvalidAuthoritySet: boolean;
    readonly isInvalidGrandpaJustification: boolean;
    readonly isInvalidRangeLinkage: boolean;
    readonly isInvalidJustificationLinkage: boolean;
    readonly isParachainEntryNotFound: boolean;
    readonly isStorageRootNotFound: boolean;
    readonly isInclusionDataDecodeError: boolean;
    readonly isInvalidStorageProof: boolean;
    readonly isEventNotIncluded: boolean;
    readonly isHeaderDecodingError: boolean;
    readonly isHeaderDataDecodingError: boolean;
    readonly isStorageRootMismatch: boolean;
    readonly isUnknownHeader: boolean;
    readonly isEventDecodingFailed: boolean;
    readonly isUnkownSideEffect: boolean;
    readonly isUnsupportedScheduledChange: boolean;
    readonly isHalted: boolean;
    readonly isBlockHeightConversionError: boolean;
    readonly type:
      | "EmptyRangeSubmitted"
      | "RangeToLarge"
      | "NoFinalizedHeader"
      | "InvalidAuthoritySet"
      | "InvalidGrandpaJustification"
      | "InvalidRangeLinkage"
      | "InvalidJustificationLinkage"
      | "ParachainEntryNotFound"
      | "StorageRootNotFound"
      | "InclusionDataDecodeError"
      | "InvalidStorageProof"
      | "EventNotIncluded"
      | "HeaderDecodingError"
      | "HeaderDataDecodingError"
      | "StorageRootMismatch"
      | "UnknownHeader"
      | "EventDecodingFailed"
      | "UnkownSideEffect"
      | "UnsupportedScheduledChange"
      | "Halted"
      | "BlockHeightConversionError";
  }

  /** @name SpRuntimeMultiSignature (308) */
  interface SpRuntimeMultiSignature extends Enum {
    readonly isEd25519: boolean;
    readonly asEd25519: SpCoreEd25519Signature;
    readonly isSr25519: boolean;
    readonly asSr25519: SpCoreSr25519Signature;
    readonly isEcdsa: boolean;
    readonly asEcdsa: SpCoreEcdsaSignature;
    readonly type: "Ed25519" | "Sr25519" | "Ecdsa";
  }

  /** @name SpCoreSr25519Signature (309) */
  interface SpCoreSr25519Signature extends U8aFixed {}

  /** @name SpCoreEcdsaSignature (310) */
  interface SpCoreEcdsaSignature extends U8aFixed {}

  /** @name FrameSystemExtensionsCheckNonZeroSender (313) */
  type FrameSystemExtensionsCheckNonZeroSender = Null;

  /** @name FrameSystemExtensionsCheckSpecVersion (314) */
  type FrameSystemExtensionsCheckSpecVersion = Null;

  /** @name FrameSystemExtensionsCheckTxVersion (315) */
  type FrameSystemExtensionsCheckTxVersion = Null;

  /** @name FrameSystemExtensionsCheckGenesis (316) */
  type FrameSystemExtensionsCheckGenesis = Null;

  /** @name FrameSystemExtensionsCheckNonce (319) */
  interface FrameSystemExtensionsCheckNonce extends Compact<u32> {}

  /** @name FrameSystemExtensionsCheckWeight (320) */
  type FrameSystemExtensionsCheckWeight = Null;

  /** @name PalletTransactionPaymentChargeTransactionPayment (321) */
  interface PalletTransactionPaymentChargeTransactionPayment
    extends Compact<u128> {}

  /** @name CircuitStandaloneRuntimeRuntime (322) */
  type CircuitStandaloneRuntimeRuntime = Null;
} // declare module
