// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/types/lookup';

import type { Data } from '@polkadot/types';
import type { BTreeMap, BTreeSet, Bytes, Compact, Enum, Null, Option, Result, Set, Struct, Text, U256, U8aFixed, Vec, bool, u128, u16, u32, u64, u8 } from '@polkadot/types-codec';
import type { ITuple } from '@polkadot/types-codec/types';
import type { AccountId32, Call, H160, H256, MultiAddress, Perbill, Percent } from '@polkadot/types/interfaces/runtime';
import type { Event } from '@polkadot/types/interfaces/system';

declare module '@polkadot/types/lookup' {
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
    readonly type: 'Other' | 'Consensus' | 'Seal' | 'PreRuntime' | 'RuntimeEnvironmentUpdated';
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
    readonly type: 'ExtrinsicSuccess' | 'ExtrinsicFailed' | 'CodeUpdated' | 'NewAccount' | 'KilledAccount' | 'Remarked';
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
    readonly type: 'Normal' | 'Operational' | 'Mandatory';
  }

  /** @name FrameSupportWeightsPays (21) */
  interface FrameSupportWeightsPays extends Enum {
    readonly isYes: boolean;
    readonly isNo: boolean;
    readonly type: 'Yes' | 'No';
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
    readonly type: 'Other' | 'CannotLookup' | 'BadOrigin' | 'Module' | 'ConsumerRemaining' | 'NoProviders' | 'TooManyConsumers' | 'Token' | 'Arithmetic' | 'Transactional';
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
    readonly type: 'NoFunds' | 'WouldDie' | 'BelowMinimum' | 'CannotCreate' | 'UnknownAsset' | 'Frozen' | 'Unsupported';
  }

  /** @name SpRuntimeArithmeticError (25) */
  interface SpRuntimeArithmeticError extends Enum {
    readonly isUnderflow: boolean;
    readonly isOverflow: boolean;
    readonly isDivisionByZero: boolean;
    readonly type: 'Underflow' | 'Overflow' | 'DivisionByZero';
  }

  /** @name SpRuntimeTransactionalError (26) */
  interface SpRuntimeTransactionalError extends Enum {
    readonly isLimitReached: boolean;
    readonly isNoLayer: boolean;
    readonly type: 'LimitReached' | 'NoLayer';
  }

  /** @name CumulusPalletParachainSystemEvent (27) */
  interface CumulusPalletParachainSystemEvent extends Enum {
    readonly isValidationFunctionStored: boolean;
    readonly isValidationFunctionApplied: boolean;
    readonly asValidationFunctionApplied: {
      readonly relayChainBlockNum: u32;
    } & Struct;
    readonly isValidationFunctionDiscarded: boolean;
    readonly isUpgradeAuthorized: boolean;
    readonly asUpgradeAuthorized: {
      readonly codeHash: H256;
    } & Struct;
    readonly isDownwardMessagesReceived: boolean;
    readonly asDownwardMessagesReceived: {
      readonly count: u32;
    } & Struct;
    readonly isDownwardMessagesProcessed: boolean;
    readonly asDownwardMessagesProcessed: {
      readonly weightUsed: u64;
      readonly dmqHead: H256;
    } & Struct;
    readonly type: 'ValidationFunctionStored' | 'ValidationFunctionApplied' | 'ValidationFunctionDiscarded' | 'UpgradeAuthorized' | 'DownwardMessagesReceived' | 'DownwardMessagesProcessed';
  }

  /** @name PalletPreimageEvent (28) */
  interface PalletPreimageEvent extends Enum {
    readonly isNoted: boolean;
    readonly asNoted: {
      readonly hash_: H256;
    } & Struct;
    readonly isRequested: boolean;
    readonly asRequested: {
      readonly hash_: H256;
    } & Struct;
    readonly isCleared: boolean;
    readonly asCleared: {
      readonly hash_: H256;
    } & Struct;
    readonly type: 'Noted' | 'Requested' | 'Cleared';
  }

  /** @name PalletSchedulerEvent (29) */
  interface PalletSchedulerEvent extends Enum {
    readonly isScheduled: boolean;
    readonly asScheduled: {
      readonly when: u32;
      readonly index: u32;
    } & Struct;
    readonly isCanceled: boolean;
    readonly asCanceled: {
      readonly when: u32;
      readonly index: u32;
    } & Struct;
    readonly isDispatched: boolean;
    readonly asDispatched: {
      readonly task: ITuple<[u32, u32]>;
      readonly id: Option<Bytes>;
      readonly result: Result<Null, SpRuntimeDispatchError>;
    } & Struct;
    readonly isCallLookupFailed: boolean;
    readonly asCallLookupFailed: {
      readonly task: ITuple<[u32, u32]>;
      readonly id: Option<Bytes>;
      readonly error: FrameSupportScheduleLookupError;
    } & Struct;
    readonly type: 'Scheduled' | 'Canceled' | 'Dispatched' | 'CallLookupFailed';
  }

  /** @name FrameSupportScheduleLookupError (34) */
  interface FrameSupportScheduleLookupError extends Enum {
    readonly isUnknown: boolean;
    readonly isBadFormat: boolean;
    readonly type: 'Unknown' | 'BadFormat';
  }

  /** @name PalletUtilityEvent (35) */
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
    readonly type: 'BatchInterrupted' | 'BatchCompleted' | 'BatchCompletedWithErrors' | 'ItemCompleted' | 'ItemFailed' | 'DispatchedAs';
  }

  /** @name PalletIdentityEvent (36) */
  interface PalletIdentityEvent extends Enum {
    readonly isIdentitySet: boolean;
    readonly asIdentitySet: {
      readonly who: AccountId32;
    } & Struct;
    readonly isIdentityCleared: boolean;
    readonly asIdentityCleared: {
      readonly who: AccountId32;
      readonly deposit: u128;
    } & Struct;
    readonly isIdentityKilled: boolean;
    readonly asIdentityKilled: {
      readonly who: AccountId32;
      readonly deposit: u128;
    } & Struct;
    readonly isJudgementRequested: boolean;
    readonly asJudgementRequested: {
      readonly who: AccountId32;
      readonly registrarIndex: u32;
    } & Struct;
    readonly isJudgementUnrequested: boolean;
    readonly asJudgementUnrequested: {
      readonly who: AccountId32;
      readonly registrarIndex: u32;
    } & Struct;
    readonly isJudgementGiven: boolean;
    readonly asJudgementGiven: {
      readonly target: AccountId32;
      readonly registrarIndex: u32;
    } & Struct;
    readonly isRegistrarAdded: boolean;
    readonly asRegistrarAdded: {
      readonly registrarIndex: u32;
    } & Struct;
    readonly isSubIdentityAdded: boolean;
    readonly asSubIdentityAdded: {
      readonly sub: AccountId32;
      readonly main: AccountId32;
      readonly deposit: u128;
    } & Struct;
    readonly isSubIdentityRemoved: boolean;
    readonly asSubIdentityRemoved: {
      readonly sub: AccountId32;
      readonly main: AccountId32;
      readonly deposit: u128;
    } & Struct;
    readonly isSubIdentityRevoked: boolean;
    readonly asSubIdentityRevoked: {
      readonly sub: AccountId32;
      readonly main: AccountId32;
      readonly deposit: u128;
    } & Struct;
    readonly type: 'IdentitySet' | 'IdentityCleared' | 'IdentityKilled' | 'JudgementRequested' | 'JudgementUnrequested' | 'JudgementGiven' | 'RegistrarAdded' | 'SubIdentityAdded' | 'SubIdentityRemoved' | 'SubIdentityRevoked';
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
    readonly type: 'Endowed' | 'DustLost' | 'Transfer' | 'BalanceSet' | 'Reserved' | 'Unreserved' | 'ReserveRepatriated' | 'Deposit' | 'Withdraw' | 'Slashed';
  }

  /** @name FrameSupportTokensMiscBalanceStatus (38) */
  interface FrameSupportTokensMiscBalanceStatus extends Enum {
    readonly isFree: boolean;
    readonly isReserved: boolean;
    readonly type: 'Free' | 'Reserved';
  }

  /** @name PalletTransactionPaymentEvent (39) */
  interface PalletTransactionPaymentEvent extends Enum {
    readonly isTransactionFeePaid: boolean;
    readonly asTransactionFeePaid: {
      readonly who: AccountId32;
      readonly actualFee: u128;
      readonly tip: u128;
    } & Struct;
    readonly type: 'TransactionFeePaid';
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
    readonly type: 'Created' | 'Issued' | 'Transferred' | 'Burned' | 'TeamChanged' | 'OwnerChanged' | 'Frozen' | 'Thawed' | 'AssetFrozen' | 'AssetThawed' | 'Destroyed' | 'ForceCreated' | 'MetadataSet' | 'MetadataCleared' | 'ApprovedTransfer' | 'ApprovalCancelled' | 'TransferredApproved' | 'AssetStatusChanged';
  }

  /** @name PalletAccountManagerEvent (42) */
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
      readonly recipient: Option<AccountId32>;
      readonly amount: u128;
    } & Struct;
    readonly type: 'ContractsRegistryExecutionFinalized' | 'Issued' | 'DepositReceived';
  }

  /** @name PalletTreasuryEvent (44) */
  interface PalletTreasuryEvent extends Enum {
    readonly isProposed: boolean;
    readonly asProposed: {
      readonly proposalIndex: u32;
    } & Struct;
    readonly isSpending: boolean;
    readonly asSpending: {
      readonly budgetRemaining: u128;
    } & Struct;
    readonly isAwarded: boolean;
    readonly asAwarded: {
      readonly proposalIndex: u32;
      readonly award: u128;
      readonly account: AccountId32;
    } & Struct;
    readonly isRejected: boolean;
    readonly asRejected: {
      readonly proposalIndex: u32;
      readonly slashed: u128;
    } & Struct;
    readonly isBurnt: boolean;
    readonly asBurnt: {
      readonly burntFunds: u128;
    } & Struct;
    readonly isRollover: boolean;
    readonly asRollover: {
      readonly rolloverBalance: u128;
    } & Struct;
    readonly isDeposit: boolean;
    readonly asDeposit: {
      readonly value: u128;
    } & Struct;
    readonly isSpendApproved: boolean;
    readonly asSpendApproved: {
      readonly proposalIndex: u32;
      readonly amount: u128;
      readonly beneficiary: AccountId32;
    } & Struct;
    readonly type: 'Proposed' | 'Spending' | 'Awarded' | 'Rejected' | 'Burnt' | 'Rollover' | 'Deposit' | 'SpendApproved';
  }

  /** @name PalletCollatorSelectionEvent (49) */
  interface PalletCollatorSelectionEvent extends Enum {
    readonly isNewInvulnerables: boolean;
    readonly asNewInvulnerables: {
      readonly invulnerables: Vec<AccountId32>;
    } & Struct;
    readonly isNewDesiredCandidates: boolean;
    readonly asNewDesiredCandidates: {
      readonly desiredCandidates: u32;
    } & Struct;
    readonly isNewCandidacyBond: boolean;
    readonly asNewCandidacyBond: {
      readonly bondAmount: u128;
    } & Struct;
    readonly isCandidateAdded: boolean;
    readonly asCandidateAdded: {
      readonly accountId: AccountId32;
      readonly deposit: u128;
    } & Struct;
    readonly isCandidateRemoved: boolean;
    readonly asCandidateRemoved: {
      readonly accountId: AccountId32;
    } & Struct;
    readonly type: 'NewInvulnerables' | 'NewDesiredCandidates' | 'NewCandidacyBond' | 'CandidateAdded' | 'CandidateRemoved';
  }

  /** @name PalletSessionEvent (51) */
  interface PalletSessionEvent extends Enum {
    readonly isNewSession: boolean;
    readonly asNewSession: {
      readonly sessionIndex: u32;
    } & Struct;
    readonly type: 'NewSession';
  }

  /** @name CumulusPalletXcmpQueueEvent (52) */
  interface CumulusPalletXcmpQueueEvent extends Enum {
    readonly isSuccess: boolean;
    readonly asSuccess: {
      readonly messageHash: Option<H256>;
      readonly weight: u64;
    } & Struct;
    readonly isFail: boolean;
    readonly asFail: {
      readonly messageHash: Option<H256>;
      readonly error: XcmV2TraitsError;
      readonly weight: u64;
    } & Struct;
    readonly isBadVersion: boolean;
    readonly asBadVersion: {
      readonly messageHash: Option<H256>;
    } & Struct;
    readonly isBadFormat: boolean;
    readonly asBadFormat: {
      readonly messageHash: Option<H256>;
    } & Struct;
    readonly isUpwardMessageSent: boolean;
    readonly asUpwardMessageSent: {
      readonly messageHash: Option<H256>;
    } & Struct;
    readonly isXcmpMessageSent: boolean;
    readonly asXcmpMessageSent: {
      readonly messageHash: Option<H256>;
    } & Struct;
    readonly isOverweightEnqueued: boolean;
    readonly asOverweightEnqueued: {
      readonly sender: u32;
      readonly sentAt: u32;
      readonly index: u64;
      readonly required: u64;
    } & Struct;
    readonly isOverweightServiced: boolean;
    readonly asOverweightServiced: {
      readonly index: u64;
      readonly used: u64;
    } & Struct;
    readonly type: 'Success' | 'Fail' | 'BadVersion' | 'BadFormat' | 'UpwardMessageSent' | 'XcmpMessageSent' | 'OverweightEnqueued' | 'OverweightServiced';
  }

  /** @name XcmV2TraitsError (54) */
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
    readonly type: 'Overflow' | 'Unimplemented' | 'UntrustedReserveLocation' | 'UntrustedTeleportLocation' | 'MultiLocationFull' | 'MultiLocationNotInvertible' | 'BadOrigin' | 'InvalidLocation' | 'AssetNotFound' | 'FailedToTransactAsset' | 'NotWithdrawable' | 'LocationCannotHold' | 'ExceedsMaxMessageSize' | 'DestinationUnsupported' | 'Transport' | 'Unroutable' | 'UnknownClaim' | 'FailedToDecode' | 'MaxWeightInvalid' | 'NotHoldingFees' | 'TooExpensive' | 'Trap' | 'UnhandledXcmVersion' | 'WeightLimitReached' | 'Barrier' | 'WeightNotComputable';
  }

  /** @name PalletXcmEvent (56) */
  interface PalletXcmEvent extends Enum {
    readonly isAttempted: boolean;
    readonly asAttempted: XcmV2TraitsOutcome;
    readonly isSent: boolean;
    readonly asSent: ITuple<[XcmV1MultiLocation, XcmV1MultiLocation, XcmV2Xcm]>;
    readonly isUnexpectedResponse: boolean;
    readonly asUnexpectedResponse: ITuple<[XcmV1MultiLocation, u64]>;
    readonly isResponseReady: boolean;
    readonly asResponseReady: ITuple<[u64, XcmV2Response]>;
    readonly isNotified: boolean;
    readonly asNotified: ITuple<[u64, u8, u8]>;
    readonly isNotifyOverweight: boolean;
    readonly asNotifyOverweight: ITuple<[u64, u8, u8, u64, u64]>;
    readonly isNotifyDispatchError: boolean;
    readonly asNotifyDispatchError: ITuple<[u64, u8, u8]>;
    readonly isNotifyDecodeFailed: boolean;
    readonly asNotifyDecodeFailed: ITuple<[u64, u8, u8]>;
    readonly isInvalidResponder: boolean;
    readonly asInvalidResponder: ITuple<[XcmV1MultiLocation, u64, Option<XcmV1MultiLocation>]>;
    readonly isInvalidResponderVersion: boolean;
    readonly asInvalidResponderVersion: ITuple<[XcmV1MultiLocation, u64]>;
    readonly isResponseTaken: boolean;
    readonly asResponseTaken: u64;
    readonly isAssetsTrapped: boolean;
    readonly asAssetsTrapped: ITuple<[H256, XcmV1MultiLocation, XcmVersionedMultiAssets]>;
    readonly isVersionChangeNotified: boolean;
    readonly asVersionChangeNotified: ITuple<[XcmV1MultiLocation, u32]>;
    readonly isSupportedVersionChanged: boolean;
    readonly asSupportedVersionChanged: ITuple<[XcmV1MultiLocation, u32]>;
    readonly isNotifyTargetSendFail: boolean;
    readonly asNotifyTargetSendFail: ITuple<[XcmV1MultiLocation, u64, XcmV2TraitsError]>;
    readonly isNotifyTargetMigrationFail: boolean;
    readonly asNotifyTargetMigrationFail: ITuple<[XcmVersionedMultiLocation, u64]>;
    readonly type: 'Attempted' | 'Sent' | 'UnexpectedResponse' | 'ResponseReady' | 'Notified' | 'NotifyOverweight' | 'NotifyDispatchError' | 'NotifyDecodeFailed' | 'InvalidResponder' | 'InvalidResponderVersion' | 'ResponseTaken' | 'AssetsTrapped' | 'VersionChangeNotified' | 'SupportedVersionChanged' | 'NotifyTargetSendFail' | 'NotifyTargetMigrationFail';
  }

  /** @name XcmV2TraitsOutcome (57) */
  interface XcmV2TraitsOutcome extends Enum {
    readonly isComplete: boolean;
    readonly asComplete: u64;
    readonly isIncomplete: boolean;
    readonly asIncomplete: ITuple<[u64, XcmV2TraitsError]>;
    readonly isError: boolean;
    readonly asError: XcmV2TraitsError;
    readonly type: 'Complete' | 'Incomplete' | 'Error';
  }

  /** @name XcmV1MultiLocation (58) */
  interface XcmV1MultiLocation extends Struct {
    readonly parents: u8;
    readonly interior: XcmV1MultilocationJunctions;
  }

  /** @name XcmV1MultilocationJunctions (59) */
  interface XcmV1MultilocationJunctions extends Enum {
    readonly isHere: boolean;
    readonly isX1: boolean;
    readonly asX1: XcmV1Junction;
    readonly isX2: boolean;
    readonly asX2: ITuple<[XcmV1Junction, XcmV1Junction]>;
    readonly isX3: boolean;
    readonly asX3: ITuple<[XcmV1Junction, XcmV1Junction, XcmV1Junction]>;
    readonly isX4: boolean;
    readonly asX4: ITuple<[XcmV1Junction, XcmV1Junction, XcmV1Junction, XcmV1Junction]>;
    readonly isX5: boolean;
    readonly asX5: ITuple<[XcmV1Junction, XcmV1Junction, XcmV1Junction, XcmV1Junction, XcmV1Junction]>;
    readonly isX6: boolean;
    readonly asX6: ITuple<[XcmV1Junction, XcmV1Junction, XcmV1Junction, XcmV1Junction, XcmV1Junction, XcmV1Junction]>;
    readonly isX7: boolean;
    readonly asX7: ITuple<[XcmV1Junction, XcmV1Junction, XcmV1Junction, XcmV1Junction, XcmV1Junction, XcmV1Junction, XcmV1Junction]>;
    readonly isX8: boolean;
    readonly asX8: ITuple<[XcmV1Junction, XcmV1Junction, XcmV1Junction, XcmV1Junction, XcmV1Junction, XcmV1Junction, XcmV1Junction, XcmV1Junction]>;
    readonly type: 'Here' | 'X1' | 'X2' | 'X3' | 'X4' | 'X5' | 'X6' | 'X7' | 'X8';
  }

  /** @name XcmV1Junction (60) */
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
    readonly type: 'Parachain' | 'AccountId32' | 'AccountIndex64' | 'AccountKey20' | 'PalletInstance' | 'GeneralIndex' | 'GeneralKey' | 'OnlyChild' | 'Plurality';
  }

  /** @name XcmV0JunctionNetworkId (62) */
  interface XcmV0JunctionNetworkId extends Enum {
    readonly isAny: boolean;
    readonly isNamed: boolean;
    readonly asNamed: Bytes;
    readonly isPolkadot: boolean;
    readonly isKusama: boolean;
    readonly type: 'Any' | 'Named' | 'Polkadot' | 'Kusama';
  }

  /** @name XcmV0JunctionBodyId (67) */
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
    readonly type: 'Unit' | 'Named' | 'Index' | 'Executive' | 'Technical' | 'Legislative' | 'Judicial';
  }

  /** @name XcmV0JunctionBodyPart (68) */
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
    readonly type: 'Voice' | 'Members' | 'Fraction' | 'AtLeastProportion' | 'MoreThanProportion';
  }

  /** @name XcmV2Xcm (69) */
  interface XcmV2Xcm extends Vec<XcmV2Instruction> {}

  /** @name XcmV2Instruction (71) */
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
    readonly type: 'WithdrawAsset' | 'ReserveAssetDeposited' | 'ReceiveTeleportedAsset' | 'QueryResponse' | 'TransferAsset' | 'TransferReserveAsset' | 'Transact' | 'HrmpNewChannelOpenRequest' | 'HrmpChannelAccepted' | 'HrmpChannelClosing' | 'ClearOrigin' | 'DescendOrigin' | 'ReportError' | 'DepositAsset' | 'DepositReserveAsset' | 'ExchangeAsset' | 'InitiateReserveWithdraw' | 'InitiateTeleport' | 'QueryHolding' | 'BuyExecution' | 'RefundSurplus' | 'SetErrorHandler' | 'SetAppendix' | 'ClearError' | 'ClaimAsset' | 'Trap' | 'SubscribeVersion' | 'UnsubscribeVersion';
  }

  /** @name XcmV1MultiassetMultiAssets (72) */
  interface XcmV1MultiassetMultiAssets extends Vec<XcmV1MultiAsset> {}

  /** @name XcmV1MultiAsset (74) */
  interface XcmV1MultiAsset extends Struct {
    readonly id: XcmV1MultiassetAssetId;
    readonly fun: XcmV1MultiassetFungibility;
  }

  /** @name XcmV1MultiassetAssetId (75) */
  interface XcmV1MultiassetAssetId extends Enum {
    readonly isConcrete: boolean;
    readonly asConcrete: XcmV1MultiLocation;
    readonly isAbstract: boolean;
    readonly asAbstract: Bytes;
    readonly type: 'Concrete' | 'Abstract';
  }

  /** @name XcmV1MultiassetFungibility (76) */
  interface XcmV1MultiassetFungibility extends Enum {
    readonly isFungible: boolean;
    readonly asFungible: Compact<u128>;
    readonly isNonFungible: boolean;
    readonly asNonFungible: XcmV1MultiassetAssetInstance;
    readonly type: 'Fungible' | 'NonFungible';
  }

  /** @name XcmV1MultiassetAssetInstance (77) */
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
    readonly type: 'Undefined' | 'Index' | 'Array4' | 'Array8' | 'Array16' | 'Array32' | 'Blob';
  }

  /** @name XcmV2Response (80) */
  interface XcmV2Response extends Enum {
    readonly isNull: boolean;
    readonly isAssets: boolean;
    readonly asAssets: XcmV1MultiassetMultiAssets;
    readonly isExecutionResult: boolean;
    readonly asExecutionResult: Option<ITuple<[u32, XcmV2TraitsError]>>;
    readonly isVersion: boolean;
    readonly asVersion: u32;
    readonly type: 'Null' | 'Assets' | 'ExecutionResult' | 'Version';
  }

  /** @name XcmV0OriginKind (83) */
  interface XcmV0OriginKind extends Enum {
    readonly isNative: boolean;
    readonly isSovereignAccount: boolean;
    readonly isSuperuser: boolean;
    readonly isXcm: boolean;
    readonly type: 'Native' | 'SovereignAccount' | 'Superuser' | 'Xcm';
  }

  /** @name XcmDoubleEncoded (84) */
  interface XcmDoubleEncoded extends Struct {
    readonly encoded: Bytes;
  }

  /** @name XcmV1MultiassetMultiAssetFilter (85) */
  interface XcmV1MultiassetMultiAssetFilter extends Enum {
    readonly isDefinite: boolean;
    readonly asDefinite: XcmV1MultiassetMultiAssets;
    readonly isWild: boolean;
    readonly asWild: XcmV1MultiassetWildMultiAsset;
    readonly type: 'Definite' | 'Wild';
  }

  /** @name XcmV1MultiassetWildMultiAsset (86) */
  interface XcmV1MultiassetWildMultiAsset extends Enum {
    readonly isAll: boolean;
    readonly isAllOf: boolean;
    readonly asAllOf: {
      readonly id: XcmV1MultiassetAssetId;
      readonly fun: XcmV1MultiassetWildFungibility;
    } & Struct;
    readonly type: 'All' | 'AllOf';
  }

  /** @name XcmV1MultiassetWildFungibility (87) */
  interface XcmV1MultiassetWildFungibility extends Enum {
    readonly isFungible: boolean;
    readonly isNonFungible: boolean;
    readonly type: 'Fungible' | 'NonFungible';
  }

  /** @name XcmV2WeightLimit (88) */
  interface XcmV2WeightLimit extends Enum {
    readonly isUnlimited: boolean;
    readonly isLimited: boolean;
    readonly asLimited: Compact<u64>;
    readonly type: 'Unlimited' | 'Limited';
  }

  /** @name XcmVersionedMultiAssets (90) */
  interface XcmVersionedMultiAssets extends Enum {
    readonly isV0: boolean;
    readonly asV0: Vec<XcmV0MultiAsset>;
    readonly isV1: boolean;
    readonly asV1: XcmV1MultiassetMultiAssets;
    readonly type: 'V0' | 'V1';
  }

  /** @name XcmV0MultiAsset (92) */
  interface XcmV0MultiAsset extends Enum {
    readonly isNone: boolean;
    readonly isAll: boolean;
    readonly isAllFungible: boolean;
    readonly isAllNonFungible: boolean;
    readonly isAllAbstractFungible: boolean;
    readonly asAllAbstractFungible: {
      readonly id: Bytes;
    } & Struct;
    readonly isAllAbstractNonFungible: boolean;
    readonly asAllAbstractNonFungible: {
      readonly class: Bytes;
    } & Struct;
    readonly isAllConcreteFungible: boolean;
    readonly asAllConcreteFungible: {
      readonly id: XcmV0MultiLocation;
    } & Struct;
    readonly isAllConcreteNonFungible: boolean;
    readonly asAllConcreteNonFungible: {
      readonly class: XcmV0MultiLocation;
    } & Struct;
    readonly isAbstractFungible: boolean;
    readonly asAbstractFungible: {
      readonly id: Bytes;
      readonly amount: Compact<u128>;
    } & Struct;
    readonly isAbstractNonFungible: boolean;
    readonly asAbstractNonFungible: {
      readonly class: Bytes;
      readonly instance: XcmV1MultiassetAssetInstance;
    } & Struct;
    readonly isConcreteFungible: boolean;
    readonly asConcreteFungible: {
      readonly id: XcmV0MultiLocation;
      readonly amount: Compact<u128>;
    } & Struct;
    readonly isConcreteNonFungible: boolean;
    readonly asConcreteNonFungible: {
      readonly class: XcmV0MultiLocation;
      readonly instance: XcmV1MultiassetAssetInstance;
    } & Struct;
    readonly type: 'None' | 'All' | 'AllFungible' | 'AllNonFungible' | 'AllAbstractFungible' | 'AllAbstractNonFungible' | 'AllConcreteFungible' | 'AllConcreteNonFungible' | 'AbstractFungible' | 'AbstractNonFungible' | 'ConcreteFungible' | 'ConcreteNonFungible';
  }

  /** @name XcmV0MultiLocation (93) */
  interface XcmV0MultiLocation extends Enum {
    readonly isNull: boolean;
    readonly isX1: boolean;
    readonly asX1: XcmV0Junction;
    readonly isX2: boolean;
    readonly asX2: ITuple<[XcmV0Junction, XcmV0Junction]>;
    readonly isX3: boolean;
    readonly asX3: ITuple<[XcmV0Junction, XcmV0Junction, XcmV0Junction]>;
    readonly isX4: boolean;
    readonly asX4: ITuple<[XcmV0Junction, XcmV0Junction, XcmV0Junction, XcmV0Junction]>;
    readonly isX5: boolean;
    readonly asX5: ITuple<[XcmV0Junction, XcmV0Junction, XcmV0Junction, XcmV0Junction, XcmV0Junction]>;
    readonly isX6: boolean;
    readonly asX6: ITuple<[XcmV0Junction, XcmV0Junction, XcmV0Junction, XcmV0Junction, XcmV0Junction, XcmV0Junction]>;
    readonly isX7: boolean;
    readonly asX7: ITuple<[XcmV0Junction, XcmV0Junction, XcmV0Junction, XcmV0Junction, XcmV0Junction, XcmV0Junction, XcmV0Junction]>;
    readonly isX8: boolean;
    readonly asX8: ITuple<[XcmV0Junction, XcmV0Junction, XcmV0Junction, XcmV0Junction, XcmV0Junction, XcmV0Junction, XcmV0Junction, XcmV0Junction]>;
    readonly type: 'Null' | 'X1' | 'X2' | 'X3' | 'X4' | 'X5' | 'X6' | 'X7' | 'X8';
  }

  /** @name XcmV0Junction (94) */
  interface XcmV0Junction extends Enum {
    readonly isParent: boolean;
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
    readonly type: 'Parent' | 'Parachain' | 'AccountId32' | 'AccountIndex64' | 'AccountKey20' | 'PalletInstance' | 'GeneralIndex' | 'GeneralKey' | 'OnlyChild' | 'Plurality';
  }

  /** @name XcmVersionedMultiLocation (95) */
  interface XcmVersionedMultiLocation extends Enum {
    readonly isV0: boolean;
    readonly asV0: XcmV0MultiLocation;
    readonly isV1: boolean;
    readonly asV1: XcmV1MultiLocation;
    readonly type: 'V0' | 'V1';
  }

  /** @name CumulusPalletXcmEvent (96) */
  interface CumulusPalletXcmEvent extends Enum {
    readonly isInvalidFormat: boolean;
    readonly asInvalidFormat: U8aFixed;
    readonly isUnsupportedVersion: boolean;
    readonly asUnsupportedVersion: U8aFixed;
    readonly isExecutedDownward: boolean;
    readonly asExecutedDownward: ITuple<[U8aFixed, XcmV2TraitsOutcome]>;
    readonly type: 'InvalidFormat' | 'UnsupportedVersion' | 'ExecutedDownward';
  }

  /** @name CumulusPalletDmpQueueEvent (97) */
  interface CumulusPalletDmpQueueEvent extends Enum {
    readonly isInvalidFormat: boolean;
    readonly asInvalidFormat: {
      readonly messageId: U8aFixed;
    } & Struct;
    readonly isUnsupportedVersion: boolean;
    readonly asUnsupportedVersion: {
      readonly messageId: U8aFixed;
    } & Struct;
    readonly isExecutedDownward: boolean;
    readonly asExecutedDownward: {
      readonly messageId: U8aFixed;
      readonly outcome: XcmV2TraitsOutcome;
    } & Struct;
    readonly isWeightExhausted: boolean;
    readonly asWeightExhausted: {
      readonly messageId: U8aFixed;
      readonly remainingWeight: u64;
      readonly requiredWeight: u64;
    } & Struct;
    readonly isOverweightEnqueued: boolean;
    readonly asOverweightEnqueued: {
      readonly messageId: U8aFixed;
      readonly overweightIndex: u64;
      readonly requiredWeight: u64;
    } & Struct;
    readonly isOverweightServiced: boolean;
    readonly asOverweightServiced: {
      readonly overweightIndex: u64;
      readonly weightUsed: u64;
    } & Struct;
    readonly type: 'InvalidFormat' | 'UnsupportedVersion' | 'ExecutedDownward' | 'WeightExhausted' | 'OverweightEnqueued' | 'OverweightServiced';
  }

  /** @name PalletXbiPortalEvent (98) */
  interface PalletXbiPortalEvent extends Enum {
    readonly isXbiMessageReceived: boolean;
    readonly asXbiMessageReceived: {
      readonly request: Option<XpFormatXbiFormat>;
      readonly response: Option<XpFormatXbiResult>;
    } & Struct;
    readonly isXbiMessageSent: boolean;
    readonly asXbiMessageSent: {
      readonly msg: XpChannelMessage;
    } & Struct;
    readonly isXbiRequestHandled: boolean;
    readonly asXbiRequestHandled: {
      readonly result: XpFormatXbiResult;
      readonly metadata: XpFormatXbiMetadata;
      readonly weight: u64;
    } & Struct;
    readonly isXbiInstructionHandled: boolean;
    readonly asXbiInstructionHandled: {
      readonly msg: XpFormatXbiFormat;
      readonly weight: u64;
    } & Struct;
    readonly isQueueEmpty: boolean;
    readonly isQueuePopped: boolean;
    readonly asQueuePopped: {
      readonly signal: XpChannelQueueQueueSignal;
      readonly msg: XpChannelMessage;
    } & Struct;
    readonly isQueuePushed: boolean;
    readonly asQueuePushed: {
      readonly signal: XpChannelQueueQueueSignal;
      readonly msg: XpChannelMessage;
    } & Struct;
    readonly isResponseStored: boolean;
    readonly asResponseStored: {
      readonly hash_: H256;
      readonly result: XpFormatXbiResult;
    } & Struct;
    readonly type: 'XbiMessageReceived' | 'XbiMessageSent' | 'XbiRequestHandled' | 'XbiInstructionHandled' | 'QueueEmpty' | 'QueuePopped' | 'QueuePushed' | 'ResponseStored';
  }

  /** @name XpFormatXbiFormat (100) */
  interface XpFormatXbiFormat extends Struct {
    readonly instr: XpFormatXbiInstruction;
    readonly metadata: XpFormatXbiMetadata;
  }

  /** @name XpFormatXbiInstruction (101) */
  interface XpFormatXbiInstruction extends Enum {
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
    readonly isTransferAssets: boolean;
    readonly asTransferAssets: {
      readonly currencyId: u32;
      readonly dest: AccountId32;
      readonly value: u128;
    } & Struct;
    readonly isSwap: boolean;
    readonly asSwap: {
      readonly assetOut: u32;
      readonly assetIn: u32;
      readonly amount: u128;
      readonly maxLimit: u128;
      readonly discount: bool;
    } & Struct;
    readonly isAddLiquidity: boolean;
    readonly asAddLiquidity: {
      readonly assetA: u32;
      readonly assetB: u32;
      readonly amountA: u128;
      readonly amountBMaxLimit: u128;
    } & Struct;
    readonly isRemoveLiquidity: boolean;
    readonly asRemoveLiquidity: {
      readonly assetA: u32;
      readonly assetB: u32;
      readonly liquidityAmount: u128;
    } & Struct;
    readonly isGetPrice: boolean;
    readonly asGetPrice: {
      readonly assetA: u32;
      readonly assetB: u32;
      readonly amount: u128;
    } & Struct;
    readonly type: 'Unknown' | 'CallNative' | 'CallEvm' | 'CallWasm' | 'CallCustom' | 'Transfer' | 'TransferAssets' | 'Swap' | 'AddLiquidity' | 'RemoveLiquidity' | 'GetPrice';
  }

  /** @name XpFormatXbiMetadata (110) */
  interface XpFormatXbiMetadata extends Struct {
    readonly id: H256;
    readonly destParaId: u32;
    readonly srcParaId: u32;
    readonly timeouts: XpFormatTimeouts;
    readonly timesheet: XpFormatXbiTimeSheet;
    readonly fees: XpFormatFees;
    readonly origin: Option<AccountId32>;
  }

  /** @name XpFormatTimeouts (111) */
  interface XpFormatTimeouts extends Struct {
    readonly sent: XpFormatActionNotificationTimeouts;
    readonly delivered: XpFormatActionNotificationTimeouts;
    readonly executed: XpFormatActionNotificationTimeouts;
    readonly responded: XpFormatActionNotificationTimeouts;
  }

  /** @name XpFormatActionNotificationTimeouts (112) */
  interface XpFormatActionNotificationTimeouts extends Struct {
    readonly action: u32;
    readonly notification: u32;
  }

  /** @name XpFormatXbiTimeSheet (113) */
  interface XpFormatXbiTimeSheet extends Struct {
    readonly submitted: Option<u32>;
    readonly sent: Option<u32>;
    readonly delivered: Option<u32>;
    readonly executed: Option<u32>;
    readonly responded: Option<u32>;
    readonly received: Option<u32>;
  }

  /** @name XpFormatFees (115) */
  interface XpFormatFees extends Struct {
    readonly asset: Option<u32>;
    readonly executionCostLimit: u128;
    readonly notificationCostLimit: u128;
    readonly aggregatedCost: u128;
  }

  /** @name XpFormatXbiResult (117) */
  interface XpFormatXbiResult extends Struct {
    readonly status: XpFormatStatus;
    readonly output: Bytes;
    readonly witness: Bytes;
  }

  /** @name XpFormatStatus (118) */
  interface XpFormatStatus extends Enum {
    readonly isSuccess: boolean;
    readonly isFailedExecution: boolean;
    readonly isDispatchFailed: boolean;
    readonly isExecutionLimitExceeded: boolean;
    readonly isNotificationLimitExceeded: boolean;
    readonly isSendTimeout: boolean;
    readonly isDeliveryTimeout: boolean;
    readonly isExecutionTimeout: boolean;
    readonly type: 'Success' | 'FailedExecution' | 'DispatchFailed' | 'ExecutionLimitExceeded' | 'NotificationLimitExceeded' | 'SendTimeout' | 'DeliveryTimeout' | 'ExecutionTimeout';
  }

  /** @name XpChannelMessage (119) */
  interface XpChannelMessage extends Enum {
    readonly isRequest: boolean;
    readonly asRequest: XpFormatXbiFormat;
    readonly isResponse: boolean;
    readonly asResponse: ITuple<[XpFormatXbiResult, XpFormatXbiMetadata]>;
    readonly type: 'Request' | 'Response';
  }

  /** @name XpChannelQueueQueueSignal (120) */
  interface XpChannelQueueQueueSignal extends Enum {
    readonly isPendingRequest: boolean;
    readonly isPendingExecution: boolean;
    readonly isPendingResponse: boolean;
    readonly isPendingResult: boolean;
    readonly isProtocolError: boolean;
    readonly asProtocolError: XpFormatStatus;
    readonly type: 'PendingRequest' | 'PendingExecution' | 'PendingResponse' | 'PendingResult' | 'ProtocolError';
  }

  /** @name PalletAssetRegistryEvent (121) */
  interface PalletAssetRegistryEvent extends Enum {
    readonly isRegistered: boolean;
    readonly asRegistered: {
      readonly assetId: u32;
      readonly location: XcmV1MultiLocation;
    } & Struct;
    readonly isInfo: boolean;
    readonly asInfo: {
      readonly assetId: u32;
      readonly location: XcmV1MultiLocation;
    } & Struct;
    readonly type: 'Registered' | 'Info';
  }

  /** @name PalletXdnsEvent (122) */
  interface PalletXdnsEvent extends Enum {
    readonly isGatewayRecordStored: boolean;
    readonly asGatewayRecordStored: U8aFixed;
    readonly isNewTokenLinkedToGateway: boolean;
    readonly asNewTokenLinkedToGateway: ITuple<[u32, U8aFixed]>;
    readonly isNewTokenAssetRegistered: boolean;
    readonly asNewTokenAssetRegistered: ITuple<[u32, U8aFixed]>;
    readonly isGatewayRecordPurged: boolean;
    readonly asGatewayRecordPurged: ITuple<[AccountId32, U8aFixed]>;
    readonly isXdnsRecordPurged: boolean;
    readonly asXdnsRecordPurged: ITuple<[AccountId32, U8aFixed]>;
    readonly isXdnsRecordUpdated: boolean;
    readonly asXdnsRecordUpdated: U8aFixed;
    readonly type: 'GatewayRecordStored' | 'NewTokenLinkedToGateway' | 'NewTokenAssetRegistered' | 'GatewayRecordPurged' | 'XdnsRecordPurged' | 'XdnsRecordUpdated';
  }

  /** @name PalletAttestersEvent (123) */
  interface PalletAttestersEvent extends Enum {
    readonly isAttesterRegistered: boolean;
    readonly asAttesterRegistered: AccountId32;
    readonly isAttesterDeregistrationScheduled: boolean;
    readonly asAttesterDeregistrationScheduled: ITuple<[AccountId32, u32]>;
    readonly isAttesterDeregistered: boolean;
    readonly asAttesterDeregistered: AccountId32;
    readonly isAttestationSubmitted: boolean;
    readonly asAttestationSubmitted: AccountId32;
    readonly isNewAttestationBatch: boolean;
    readonly asNewAttestationBatch: ITuple<[U8aFixed, PalletAttestersBatchMessage]>;
    readonly isNewAttestationMessageHash: boolean;
    readonly asNewAttestationMessageHash: ITuple<[U8aFixed, H256, T3rnPrimitivesExecutionVendor]>;
    readonly isNewConfirmationBatch: boolean;
    readonly asNewConfirmationBatch: ITuple<[U8aFixed, PalletAttestersBatchMessage, Bytes, H256]>;
    readonly isNominated: boolean;
    readonly asNominated: ITuple<[AccountId32, AccountId32, u128]>;
    readonly isNewTargetActivated: boolean;
    readonly asNewTargetActivated: U8aFixed;
    readonly isNewTargetProposed: boolean;
    readonly asNewTargetProposed: U8aFixed;
    readonly isAttesterAgreedToNewTarget: boolean;
    readonly asAttesterAgreedToNewTarget: ITuple<[AccountId32, U8aFixed, Bytes]>;
    readonly isCurrentPendingAttestationBatches: boolean;
    readonly asCurrentPendingAttestationBatches: ITuple<[U8aFixed, Vec<ITuple<[u32, H256]>>]>;
    readonly type: 'AttesterRegistered' | 'AttesterDeregistrationScheduled' | 'AttesterDeregistered' | 'AttestationSubmitted' | 'NewAttestationBatch' | 'NewAttestationMessageHash' | 'NewConfirmationBatch' | 'Nominated' | 'NewTargetActivated' | 'NewTargetProposed' | 'AttesterAgreedToNewTarget' | 'CurrentPendingAttestationBatches';
  }

  /** @name PalletAttestersBatchMessage (124) */
  interface PalletAttestersBatchMessage extends Struct {
    readonly committedSfx: Option<Vec<H256>>;
    readonly revertedSfx: Option<Vec<H256>>;
    readonly nextCommittee: Option<Vec<Bytes>>;
    readonly bannedCommittee: Option<Vec<Bytes>>;
    readonly index: u32;
    readonly signatures: Vec<ITuple<[u32, U8aFixed]>>;
    readonly created: u32;
    readonly status: PalletAttestersBatchStatus;
    readonly latency: T3rnPrimitivesAttestersLatencyStatus;
  }

  /** @name PalletAttestersBatchStatus (131) */
  interface PalletAttestersBatchStatus extends Enum {
    readonly isPendingMessage: boolean;
    readonly isPendingAttestation: boolean;
    readonly isReadyForSubmissionByMajority: boolean;
    readonly isReadyForSubmissionFullyApproved: boolean;
    readonly isRepatriated: boolean;
    readonly isExpired: boolean;
    readonly isCommitted: boolean;
    readonly type: 'PendingMessage' | 'PendingAttestation' | 'ReadyForSubmissionByMajority' | 'ReadyForSubmissionFullyApproved' | 'Repatriated' | 'Expired' | 'Committed';
  }

  /** @name T3rnPrimitivesAttestersLatencyStatus (132) */
  interface T3rnPrimitivesAttestersLatencyStatus extends Enum {
    readonly isOnTime: boolean;
    readonly isLate: boolean;
    readonly asLate: ITuple<[u32, u32]>;
    readonly type: 'OnTime' | 'Late';
  }

  /** @name T3rnPrimitivesExecutionVendor (133) */
  interface T3rnPrimitivesExecutionVendor extends Enum {
    readonly isSubstrate: boolean;
    readonly isEvm: boolean;
    readonly type: 'Substrate' | 'Evm';
  }

  /** @name PalletRewardsEvent (136) */
  interface PalletRewardsEvent extends Enum {
    readonly isAttesterRewarded: boolean;
    readonly asAttesterRewarded: ITuple<[AccountId32, u128]>;
    readonly isCollatorRewarded: boolean;
    readonly asCollatorRewarded: ITuple<[AccountId32, u128]>;
    readonly isExecutorRewarded: boolean;
    readonly asExecutorRewarded: ITuple<[AccountId32, u128]>;
    readonly isNewMaxRewardExecutorsKickbackSet: boolean;
    readonly asNewMaxRewardExecutorsKickbackSet: ITuple<[Percent, Percent]>;
    readonly isClaimed: boolean;
    readonly asClaimed: ITuple<[AccountId32, u128]>;
    readonly isPendingClaim: boolean;
    readonly asPendingClaim: ITuple<[AccountId32, u128]>;
    readonly type: 'AttesterRewarded' | 'CollatorRewarded' | 'ExecutorRewarded' | 'NewMaxRewardExecutorsKickbackSet' | 'Claimed' | 'PendingClaim';
  }

  /** @name PalletContractsRegistryEvent (138) */
  interface PalletContractsRegistryEvent extends Enum {
    readonly isContractStored: boolean;
    readonly asContractStored: ITuple<[AccountId32, H256]>;
    readonly isContractPurged: boolean;
    readonly asContractPurged: ITuple<[AccountId32, H256]>;
    readonly type: 'ContractStored' | 'ContractPurged';
  }

  /** @name PalletCircuitEvent (139) */
  interface PalletCircuitEvent extends Enum {
    readonly isTransfer: boolean;
    readonly asTransfer: ITuple<[AccountId32, AccountId32, AccountId32, u128]>;
    readonly isTransferAssets: boolean;
    readonly asTransferAssets: ITuple<[AccountId32, u32, AccountId32, AccountId32, u128]>;
    readonly isTransferORML: boolean;
    readonly asTransferORML: ITuple<[AccountId32, u32, AccountId32, AccountId32, u128]>;
    readonly isAddLiquidity: boolean;
    readonly asAddLiquidity: ITuple<[AccountId32, u32, u32, u128, u128, u128]>;
    readonly isSwap: boolean;
    readonly asSwap: ITuple<[AccountId32, u32, u32, u128, u128, u128]>;
    readonly isCallNative: boolean;
    readonly asCallNative: ITuple<[AccountId32, Bytes]>;
    readonly isCallEvm: boolean;
    readonly asCallEvm: ITuple<[AccountId32, H160, H160, U256, Bytes, u64, U256, Option<U256>, Option<U256>, Vec<ITuple<[H160, Vec<H256>]>>]>;
    readonly isCallWasm: boolean;
    readonly asCallWasm: ITuple<[AccountId32, AccountId32, u128, u64, Option<u128>, Bytes]>;
    readonly isCallCustom: boolean;
    readonly asCallCustom: ITuple<[AccountId32, AccountId32, AccountId32, u128, Bytes, u64, Bytes]>;
    readonly isResult: boolean;
    readonly asResult: ITuple<[AccountId32, AccountId32, XpFormatXbiResult, Bytes, Bytes]>;
    readonly isXTransactionReceivedForExec: boolean;
    readonly asXTransactionReceivedForExec: H256;
    readonly isSfxNewBidReceived: boolean;
    readonly asSfxNewBidReceived: ITuple<[H256, AccountId32, u128]>;
    readonly isSideEffectConfirmed: boolean;
    readonly asSideEffectConfirmed: H256;
    readonly isXTransactionReadyForExec: boolean;
    readonly asXTransactionReadyForExec: H256;
    readonly isXTransactionStepFinishedExec: boolean;
    readonly asXTransactionStepFinishedExec: H256;
    readonly isXTransactionXtxFinishedExecAllSteps: boolean;
    readonly asXTransactionXtxFinishedExecAllSteps: H256;
    readonly isXTransactionFSXCommitted: boolean;
    readonly asXTransactionFSXCommitted: H256;
    readonly isXTransactionXtxCommitted: boolean;
    readonly asXTransactionXtxCommitted: H256;
    readonly isXTransactionXtxRevertedAfterTimeOut: boolean;
    readonly asXTransactionXtxRevertedAfterTimeOut: H256;
    readonly isXTransactionXtxDroppedAtBidding: boolean;
    readonly asXTransactionXtxDroppedAtBidding: H256;
    readonly isNewSideEffectsAvailable: boolean;
    readonly asNewSideEffectsAvailable: ITuple<[AccountId32, H256, Vec<T3rnTypesSfxSideEffect>, Vec<H256>]>;
    readonly isCancelledSideEffects: boolean;
    readonly asCancelledSideEffects: ITuple<[AccountId32, H256, Vec<T3rnTypesSfxSideEffect>]>;
    readonly isSideEffectsConfirmed: boolean;
    readonly asSideEffectsConfirmed: ITuple<[H256, Vec<Vec<T3rnTypesFsxFullSideEffect>>]>;
    readonly isEscrowTransfer: boolean;
    readonly asEscrowTransfer: ITuple<[AccountId32, AccountId32, u128]>;
    readonly isSuccessfulFSXCommitAttestationRequest: boolean;
    readonly asSuccessfulFSXCommitAttestationRequest: H256;
    readonly isUnsuccessfulFSXCommitAttestationRequest: boolean;
    readonly asUnsuccessfulFSXCommitAttestationRequest: H256;
    readonly isSuccessfulFSXRevertAttestationRequest: boolean;
    readonly asSuccessfulFSXRevertAttestationRequest: H256;
    readonly isUnsuccessfulFSXRevertAttestationRequest: boolean;
    readonly asUnsuccessfulFSXRevertAttestationRequest: H256;
    readonly type: 'Transfer' | 'TransferAssets' | 'TransferORML' | 'AddLiquidity' | 'Swap' | 'CallNative' | 'CallEvm' | 'CallWasm' | 'CallCustom' | 'Result' | 'XTransactionReceivedForExec' | 'SfxNewBidReceived' | 'SideEffectConfirmed' | 'XTransactionReadyForExec' | 'XTransactionStepFinishedExec' | 'XTransactionXtxFinishedExecAllSteps' | 'XTransactionFSXCommitted' | 'XTransactionXtxCommitted' | 'XTransactionXtxRevertedAfterTimeOut' | 'XTransactionXtxDroppedAtBidding' | 'NewSideEffectsAvailable' | 'CancelledSideEffects' | 'SideEffectsConfirmed' | 'EscrowTransfer' | 'SuccessfulFSXCommitAttestationRequest' | 'UnsuccessfulFSXCommitAttestationRequest' | 'SuccessfulFSXRevertAttestationRequest' | 'UnsuccessfulFSXRevertAttestationRequest';
  }

  /** @name T3rnTypesSfxSideEffect (141) */
  interface T3rnTypesSfxSideEffect extends Struct {
    readonly target: U8aFixed;
    readonly maxReward: u128;
    readonly insurance: u128;
    readonly action: U8aFixed;
    readonly encodedArgs: Vec<Bytes>;
    readonly signature: Bytes;
    readonly enforceExecutor: Option<AccountId32>;
    readonly rewardAssetId: Option<u32>;
  }

  /** @name T3rnTypesFsxFullSideEffect (144) */
  interface T3rnTypesFsxFullSideEffect extends Struct {
    readonly input: T3rnTypesSfxSideEffect;
    readonly confirmed: Option<T3rnTypesSfxConfirmedSideEffect>;
    readonly securityLvl: T3rnTypesSfxSecurityLvl;
    readonly submissionTargetHeight: u32;
    readonly bestBid: Option<T3rnTypesBidSfxBid>;
    readonly index: u32;
  }

  /** @name T3rnTypesSfxConfirmedSideEffect (146) */
  interface T3rnTypesSfxConfirmedSideEffect extends Struct {
    readonly err: Option<T3rnTypesSfxConfirmationOutcome>;
    readonly output: Option<Bytes>;
    readonly inclusionData: Bytes;
    readonly executioner: AccountId32;
    readonly receivedAt: u32;
    readonly cost: Option<u128>;
  }

  /** @name T3rnTypesSfxConfirmationOutcome (148) */
  interface T3rnTypesSfxConfirmationOutcome extends Enum {
    readonly isSuccess: boolean;
    readonly isMisbehaviourMalformedValues: boolean;
    readonly asMisbehaviourMalformedValues: {
      readonly key: Bytes;
      readonly expected: Bytes;
      readonly received: Bytes;
    } & Struct;
    readonly isTimedOut: boolean;
    readonly type: 'Success' | 'MisbehaviourMalformedValues' | 'TimedOut';
  }

  /** @name T3rnTypesSfxSecurityLvl (149) */
  interface T3rnTypesSfxSecurityLvl extends Enum {
    readonly isOptimistic: boolean;
    readonly isEscrow: boolean;
    readonly type: 'Optimistic' | 'Escrow';
  }

  /** @name T3rnTypesBidSfxBid (151) */
  interface T3rnTypesBidSfxBid extends Struct {
    readonly amount: u128;
    readonly insurance: u128;
    readonly reservedBond: Option<u128>;
    readonly rewardAssetId: Option<u32>;
    readonly executor: AccountId32;
    readonly requester: AccountId32;
  }

  /** @name PalletClockEvent (152) */
  interface PalletClockEvent extends Enum {
    readonly isNewRound: boolean;
    readonly asNewRound: {
      readonly index: u32;
      readonly head: u32;
      readonly term: u32;
    } & Struct;
    readonly type: 'NewRound';
  }

  /** @name Pallet3vmEvent (153) */
  interface Pallet3vmEvent extends Enum {
    readonly isSignalBounced: boolean;
    readonly asSignalBounced: ITuple<[u32, T3rnSdkPrimitivesSignalSignalKind, H256]>;
    readonly isExceededBounceThrehold: boolean;
    readonly asExceededBounceThrehold: ITuple<[u32, T3rnSdkPrimitivesSignalSignalKind, H256]>;
    readonly isModuleInstantiated: boolean;
    readonly asModuleInstantiated: ITuple<[H256, AccountId32, T3rnPrimitivesContractMetadataContractType, u32]>;
    readonly isAuthorStored: boolean;
    readonly asAuthorStored: ITuple<[AccountId32, AccountId32]>;
    readonly isAuthorRemoved: boolean;
    readonly asAuthorRemoved: AccountId32;
    readonly type: 'SignalBounced' | 'ExceededBounceThrehold' | 'ModuleInstantiated' | 'AuthorStored' | 'AuthorRemoved';
  }

  /** @name T3rnSdkPrimitivesSignalSignalKind (155) */
  interface T3rnSdkPrimitivesSignalSignalKind extends Enum {
    readonly isComplete: boolean;
    readonly isKill: boolean;
    readonly asKill: T3rnSdkPrimitivesSignalKillReason;
    readonly type: 'Complete' | 'Kill';
  }

  /** @name T3rnSdkPrimitivesSignalKillReason (156) */
  interface T3rnSdkPrimitivesSignalKillReason extends Enum {
    readonly isUnhandled: boolean;
    readonly isCodec: boolean;
    readonly isTimeout: boolean;
    readonly type: 'Unhandled' | 'Codec' | 'Timeout';
  }

  /** @name T3rnPrimitivesContractMetadataContractType (158) */
  interface T3rnPrimitivesContractMetadataContractType extends Enum {
    readonly isSystem: boolean;
    readonly isVanillaEvm: boolean;
    readonly isVanillaWasm: boolean;
    readonly isVolatileEvm: boolean;
    readonly isVolatileWasm: boolean;
    readonly type: 'System' | 'VanillaEvm' | 'VanillaWasm' | 'VolatileEvm' | 'VolatileWasm';
  }

  /** @name PalletContractsEvent (160) */
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
    readonly type: 'Instantiated' | 'Terminated' | 'CodeStored' | 'ContractEmitted' | 'CodeRemoved' | 'ContractCodeUpdated';
  }

  /** @name PalletEvmEvent (161) */
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
    readonly type: 'Log' | 'Created' | 'CreatedFailed' | 'Executed' | 'ExecutedFailed' | 'BalanceDeposit' | 'BalanceWithdraw' | 'ClaimAccount';
  }

  /** @name EthereumLog (162) */
  interface EthereumLog extends Struct {
    readonly address: H160;
    readonly topics: Vec<H256>;
    readonly data: Bytes;
  }

  /** @name PalletPortalEvent (163) */
  interface PalletPortalEvent extends Enum {
    readonly isGatewayRegistered: boolean;
    readonly asGatewayRegistered: U8aFixed;
    readonly isSetOwner: boolean;
    readonly asSetOwner: ITuple<[U8aFixed, Bytes]>;
    readonly isSetOperational: boolean;
    readonly asSetOperational: ITuple<[U8aFixed, bool]>;
    readonly isHeaderSubmitted: boolean;
    readonly asHeaderSubmitted: ITuple<[T3rnPrimitivesGatewayVendor, Bytes]>;
    readonly type: 'GatewayRegistered' | 'SetOwner' | 'SetOperational' | 'HeaderSubmitted';
  }

  /** @name T3rnPrimitivesGatewayVendor (164) */
  interface T3rnPrimitivesGatewayVendor extends Enum {
    readonly isPolkadot: boolean;
    readonly isKusama: boolean;
    readonly isRococo: boolean;
    readonly isEthereum: boolean;
    readonly type: 'Polkadot' | 'Kusama' | 'Rococo' | 'Ethereum';
  }

  /** @name PalletGrandpaFinalityVerifierEvent (165) */
  interface PalletGrandpaFinalityVerifierEvent extends Enum {
    readonly isHeadersAdded: boolean;
    readonly asHeadersAdded: u32;
    readonly type: 'HeadersAdded';
  }

  /** @name PalletMaintenanceModeEvent (168) */
  interface PalletMaintenanceModeEvent extends Enum {
    readonly isEnteredMaintenanceMode: boolean;
    readonly isNormalOperationResumed: boolean;
    readonly isFailedToSuspendIdleXcmExecution: boolean;
    readonly asFailedToSuspendIdleXcmExecution: {
      readonly error: SpRuntimeDispatchError;
    } & Struct;
    readonly isFailedToResumeIdleXcmExecution: boolean;
    readonly asFailedToResumeIdleXcmExecution: {
      readonly error: SpRuntimeDispatchError;
    } & Struct;
    readonly type: 'EnteredMaintenanceMode' | 'NormalOperationResumed' | 'FailedToSuspendIdleXcmExecution' | 'FailedToResumeIdleXcmExecution';
  }

  /** @name PalletSudoEvent (169) */
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
    readonly type: 'Sudid' | 'KeyChanged' | 'SudoAsDone';
  }

  /** @name FrameSystemPhase (170) */
  interface FrameSystemPhase extends Enum {
    readonly isApplyExtrinsic: boolean;
    readonly asApplyExtrinsic: u32;
    readonly isFinalization: boolean;
    readonly isInitialization: boolean;
    readonly type: 'ApplyExtrinsic' | 'Finalization' | 'Initialization';
  }

  /** @name FrameSystemLastRuntimeUpgradeInfo (172) */
  interface FrameSystemLastRuntimeUpgradeInfo extends Struct {
    readonly specVersion: Compact<u32>;
    readonly specName: Text;
  }

  /** @name FrameSystemCall (174) */
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
    readonly type: 'FillBlock' | 'Remark' | 'SetHeapPages' | 'SetCode' | 'SetCodeWithoutChecks' | 'SetStorage' | 'KillStorage' | 'KillPrefix' | 'RemarkWithEvent';
  }

  /** @name FrameSystemLimitsBlockWeights (178) */
  interface FrameSystemLimitsBlockWeights extends Struct {
    readonly baseBlock: u64;
    readonly maxBlock: u64;
    readonly perClass: FrameSupportWeightsPerDispatchClassWeightsPerClass;
  }

  /** @name FrameSupportWeightsPerDispatchClassWeightsPerClass (179) */
  interface FrameSupportWeightsPerDispatchClassWeightsPerClass extends Struct {
    readonly normal: FrameSystemLimitsWeightsPerClass;
    readonly operational: FrameSystemLimitsWeightsPerClass;
    readonly mandatory: FrameSystemLimitsWeightsPerClass;
  }

  /** @name FrameSystemLimitsWeightsPerClass (180) */
  interface FrameSystemLimitsWeightsPerClass extends Struct {
    readonly baseExtrinsic: u64;
    readonly maxExtrinsic: Option<u64>;
    readonly maxTotal: Option<u64>;
    readonly reserved: Option<u64>;
  }

  /** @name FrameSystemLimitsBlockLength (182) */
  interface FrameSystemLimitsBlockLength extends Struct {
    readonly max: FrameSupportWeightsPerDispatchClassU32;
  }

  /** @name FrameSupportWeightsPerDispatchClassU32 (183) */
  interface FrameSupportWeightsPerDispatchClassU32 extends Struct {
    readonly normal: u32;
    readonly operational: u32;
    readonly mandatory: u32;
  }

  /** @name FrameSupportWeightsRuntimeDbWeight (184) */
  interface FrameSupportWeightsRuntimeDbWeight extends Struct {
    readonly read: u64;
    readonly write: u64;
  }

  /** @name SpVersionRuntimeVersion (185) */
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

  /** @name FrameSystemError (190) */
  interface FrameSystemError extends Enum {
    readonly isInvalidSpecName: boolean;
    readonly isSpecVersionNeedsToIncrease: boolean;
    readonly isFailedToExtractRuntimeVersion: boolean;
    readonly isNonDefaultComposite: boolean;
    readonly isNonZeroRefCount: boolean;
    readonly isCallFiltered: boolean;
    readonly type: 'InvalidSpecName' | 'SpecVersionNeedsToIncrease' | 'FailedToExtractRuntimeVersion' | 'NonDefaultComposite' | 'NonZeroRefCount' | 'CallFiltered';
  }

  /** @name PolkadotPrimitivesV2PersistedValidationData (191) */
  interface PolkadotPrimitivesV2PersistedValidationData extends Struct {
    readonly parentHead: Bytes;
    readonly relayParentNumber: u32;
    readonly relayParentStorageRoot: H256;
    readonly maxPovSize: u32;
  }

  /** @name PolkadotPrimitivesV2UpgradeRestriction (194) */
  interface PolkadotPrimitivesV2UpgradeRestriction extends Enum {
    readonly isPresent: boolean;
    readonly type: 'Present';
  }

  /** @name SpTrieStorageProof (195) */
  interface SpTrieStorageProof extends Struct {
    readonly trieNodes: BTreeSet<Bytes>;
  }

  /** @name CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot (197) */
  interface CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot extends Struct {
    readonly dmqMqcHead: H256;
    readonly relayDispatchQueueSize: ITuple<[u32, u32]>;
    readonly ingressChannels: Vec<ITuple<[u32, PolkadotPrimitivesV2AbridgedHrmpChannel]>>;
    readonly egressChannels: Vec<ITuple<[u32, PolkadotPrimitivesV2AbridgedHrmpChannel]>>;
  }

  /** @name PolkadotPrimitivesV2AbridgedHrmpChannel (200) */
  interface PolkadotPrimitivesV2AbridgedHrmpChannel extends Struct {
    readonly maxCapacity: u32;
    readonly maxTotalSize: u32;
    readonly maxMessageSize: u32;
    readonly msgCount: u32;
    readonly totalSize: u32;
    readonly mqcHead: Option<H256>;
  }

  /** @name PolkadotPrimitivesV2AbridgedHostConfiguration (201) */
  interface PolkadotPrimitivesV2AbridgedHostConfiguration extends Struct {
    readonly maxCodeSize: u32;
    readonly maxHeadDataSize: u32;
    readonly maxUpwardQueueCount: u32;
    readonly maxUpwardQueueSize: u32;
    readonly maxUpwardMessageSize: u32;
    readonly maxUpwardMessageNumPerCandidate: u32;
    readonly hrmpMaxMessageNumPerCandidate: u32;
    readonly validationUpgradeCooldown: u32;
    readonly validationUpgradeDelay: u32;
  }

  /** @name PolkadotCorePrimitivesOutboundHrmpMessage (207) */
  interface PolkadotCorePrimitivesOutboundHrmpMessage extends Struct {
    readonly recipient: u32;
    readonly data: Bytes;
  }

  /** @name CumulusPalletParachainSystemCall (208) */
  interface CumulusPalletParachainSystemCall extends Enum {
    readonly isSetValidationData: boolean;
    readonly asSetValidationData: {
      readonly data: CumulusPrimitivesParachainInherentParachainInherentData;
    } & Struct;
    readonly isSudoSendUpwardMessage: boolean;
    readonly asSudoSendUpwardMessage: {
      readonly message: Bytes;
    } & Struct;
    readonly isAuthorizeUpgrade: boolean;
    readonly asAuthorizeUpgrade: {
      readonly codeHash: H256;
    } & Struct;
    readonly isEnactAuthorizedUpgrade: boolean;
    readonly asEnactAuthorizedUpgrade: {
      readonly code: Bytes;
    } & Struct;
    readonly type: 'SetValidationData' | 'SudoSendUpwardMessage' | 'AuthorizeUpgrade' | 'EnactAuthorizedUpgrade';
  }

  /** @name CumulusPrimitivesParachainInherentParachainInherentData (209) */
  interface CumulusPrimitivesParachainInherentParachainInherentData extends Struct {
    readonly validationData: PolkadotPrimitivesV2PersistedValidationData;
    readonly relayChainState: SpTrieStorageProof;
    readonly downwardMessages: Vec<PolkadotCorePrimitivesInboundDownwardMessage>;
    readonly horizontalMessages: BTreeMap<u32, Vec<PolkadotCorePrimitivesInboundHrmpMessage>>;
  }

  /** @name PolkadotCorePrimitivesInboundDownwardMessage (211) */
  interface PolkadotCorePrimitivesInboundDownwardMessage extends Struct {
    readonly sentAt: u32;
    readonly msg: Bytes;
  }

  /** @name PolkadotCorePrimitivesInboundHrmpMessage (214) */
  interface PolkadotCorePrimitivesInboundHrmpMessage extends Struct {
    readonly sentAt: u32;
    readonly data: Bytes;
  }

  /** @name CumulusPalletParachainSystemError (217) */
  interface CumulusPalletParachainSystemError extends Enum {
    readonly isOverlappingUpgrades: boolean;
    readonly isProhibitedByPolkadot: boolean;
    readonly isTooBig: boolean;
    readonly isValidationDataNotAvailable: boolean;
    readonly isHostConfigurationNotAvailable: boolean;
    readonly isNotScheduled: boolean;
    readonly isNothingAuthorized: boolean;
    readonly isUnauthorized: boolean;
    readonly type: 'OverlappingUpgrades' | 'ProhibitedByPolkadot' | 'TooBig' | 'ValidationDataNotAvailable' | 'HostConfigurationNotAvailable' | 'NotScheduled' | 'NothingAuthorized' | 'Unauthorized';
  }

  /** @name PalletTimestampCall (218) */
  interface PalletTimestampCall extends Enum {
    readonly isSet: boolean;
    readonly asSet: {
      readonly now: Compact<u64>;
    } & Struct;
    readonly type: 'Set';
  }

  /** @name PalletPreimageRequestStatus (219) */
  interface PalletPreimageRequestStatus extends Enum {
    readonly isUnrequested: boolean;
    readonly asUnrequested: Option<ITuple<[AccountId32, u128]>>;
    readonly isRequested: boolean;
    readonly asRequested: u32;
    readonly type: 'Unrequested' | 'Requested';
  }

  /** @name PalletPreimageCall (223) */
  interface PalletPreimageCall extends Enum {
    readonly isNotePreimage: boolean;
    readonly asNotePreimage: {
      readonly bytes: Bytes;
    } & Struct;
    readonly isUnnotePreimage: boolean;
    readonly asUnnotePreimage: {
      readonly hash_: H256;
    } & Struct;
    readonly isRequestPreimage: boolean;
    readonly asRequestPreimage: {
      readonly hash_: H256;
    } & Struct;
    readonly isUnrequestPreimage: boolean;
    readonly asUnrequestPreimage: {
      readonly hash_: H256;
    } & Struct;
    readonly type: 'NotePreimage' | 'UnnotePreimage' | 'RequestPreimage' | 'UnrequestPreimage';
  }

  /** @name PalletPreimageError (224) */
  interface PalletPreimageError extends Enum {
    readonly isTooLarge: boolean;
    readonly isAlreadyNoted: boolean;
    readonly isNotAuthorized: boolean;
    readonly isNotNoted: boolean;
    readonly isRequested: boolean;
    readonly isNotRequested: boolean;
    readonly type: 'TooLarge' | 'AlreadyNoted' | 'NotAuthorized' | 'NotNoted' | 'Requested' | 'NotRequested';
  }

  /** @name PalletSchedulerScheduledV3 (227) */
  interface PalletSchedulerScheduledV3 extends Struct {
    readonly maybeId: Option<Bytes>;
    readonly priority: u8;
    readonly call: FrameSupportScheduleMaybeHashed;
    readonly maybePeriodic: Option<ITuple<[u32, u32]>>;
    readonly origin: T0rnParachainRuntimeOriginCaller;
  }

  /** @name FrameSupportScheduleMaybeHashed (228) */
  interface FrameSupportScheduleMaybeHashed extends Enum {
    readonly isValue: boolean;
    readonly asValue: Call;
    readonly isHash: boolean;
    readonly asHash: H256;
    readonly type: 'Value' | 'Hash';
  }

  /** @name PalletSchedulerCall (230) */
  interface PalletSchedulerCall extends Enum {
    readonly isSchedule: boolean;
    readonly asSchedule: {
      readonly when: u32;
      readonly maybePeriodic: Option<ITuple<[u32, u32]>>;
      readonly priority: u8;
      readonly call: FrameSupportScheduleMaybeHashed;
    } & Struct;
    readonly isCancel: boolean;
    readonly asCancel: {
      readonly when: u32;
      readonly index: u32;
    } & Struct;
    readonly isScheduleNamed: boolean;
    readonly asScheduleNamed: {
      readonly id: Bytes;
      readonly when: u32;
      readonly maybePeriodic: Option<ITuple<[u32, u32]>>;
      readonly priority: u8;
      readonly call: FrameSupportScheduleMaybeHashed;
    } & Struct;
    readonly isCancelNamed: boolean;
    readonly asCancelNamed: {
      readonly id: Bytes;
    } & Struct;
    readonly isScheduleAfter: boolean;
    readonly asScheduleAfter: {
      readonly after: u32;
      readonly maybePeriodic: Option<ITuple<[u32, u32]>>;
      readonly priority: u8;
      readonly call: FrameSupportScheduleMaybeHashed;
    } & Struct;
    readonly isScheduleNamedAfter: boolean;
    readonly asScheduleNamedAfter: {
      readonly id: Bytes;
      readonly after: u32;
      readonly maybePeriodic: Option<ITuple<[u32, u32]>>;
      readonly priority: u8;
      readonly call: FrameSupportScheduleMaybeHashed;
    } & Struct;
    readonly type: 'Schedule' | 'Cancel' | 'ScheduleNamed' | 'CancelNamed' | 'ScheduleAfter' | 'ScheduleNamedAfter';
  }

  /** @name PalletUtilityCall (232) */
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
      readonly asOrigin: T0rnParachainRuntimeOriginCaller;
      readonly call: Call;
    } & Struct;
    readonly isForceBatch: boolean;
    readonly asForceBatch: {
      readonly calls: Vec<Call>;
    } & Struct;
    readonly type: 'Batch' | 'AsDerivative' | 'BatchAll' | 'DispatchAs' | 'ForceBatch';
  }

  /** @name T0rnParachainRuntimeOriginCaller (234) */
  interface T0rnParachainRuntimeOriginCaller extends Enum {
    readonly isSystem: boolean;
    readonly asSystem: FrameSupportDispatchRawOrigin;
    readonly isVoid: boolean;
    readonly isPolkadotXcm: boolean;
    readonly asPolkadotXcm: PalletXcmOrigin;
    readonly isCumulusXcm: boolean;
    readonly asCumulusXcm: CumulusPalletXcmOrigin;
    readonly type: 'System' | 'Void' | 'PolkadotXcm' | 'CumulusXcm';
  }

  /** @name FrameSupportDispatchRawOrigin (235) */
  interface FrameSupportDispatchRawOrigin extends Enum {
    readonly isRoot: boolean;
    readonly isSigned: boolean;
    readonly asSigned: AccountId32;
    readonly isNone: boolean;
    readonly type: 'Root' | 'Signed' | 'None';
  }

  /** @name PalletXcmOrigin (236) */
  interface PalletXcmOrigin extends Enum {
    readonly isXcm: boolean;
    readonly asXcm: XcmV1MultiLocation;
    readonly isResponse: boolean;
    readonly asResponse: XcmV1MultiLocation;
    readonly type: 'Xcm' | 'Response';
  }

  /** @name CumulusPalletXcmOrigin (237) */
  interface CumulusPalletXcmOrigin extends Enum {
    readonly isRelay: boolean;
    readonly isSiblingParachain: boolean;
    readonly asSiblingParachain: u32;
    readonly type: 'Relay' | 'SiblingParachain';
  }

  /** @name SpCoreVoid (238) */
  type SpCoreVoid = Null;

  /** @name PalletIdentityCall (239) */
  interface PalletIdentityCall extends Enum {
    readonly isAddRegistrar: boolean;
    readonly asAddRegistrar: {
      readonly account: AccountId32;
    } & Struct;
    readonly isSetIdentity: boolean;
    readonly asSetIdentity: {
      readonly info: PalletIdentityIdentityInfo;
    } & Struct;
    readonly isSetSubs: boolean;
    readonly asSetSubs: {
      readonly subs: Vec<ITuple<[AccountId32, Data]>>;
    } & Struct;
    readonly isClearIdentity: boolean;
    readonly isRequestJudgement: boolean;
    readonly asRequestJudgement: {
      readonly regIndex: Compact<u32>;
      readonly maxFee: Compact<u128>;
    } & Struct;
    readonly isCancelRequest: boolean;
    readonly asCancelRequest: {
      readonly regIndex: u32;
    } & Struct;
    readonly isSetFee: boolean;
    readonly asSetFee: {
      readonly index: Compact<u32>;
      readonly fee: Compact<u128>;
    } & Struct;
    readonly isSetAccountId: boolean;
    readonly asSetAccountId: {
      readonly index: Compact<u32>;
      readonly new_: AccountId32;
    } & Struct;
    readonly isSetFields: boolean;
    readonly asSetFields: {
      readonly index: Compact<u32>;
      readonly fields: PalletIdentityBitFlags;
    } & Struct;
    readonly isProvideJudgement: boolean;
    readonly asProvideJudgement: {
      readonly regIndex: Compact<u32>;
      readonly target: MultiAddress;
      readonly judgement: PalletIdentityJudgement;
    } & Struct;
    readonly isKillIdentity: boolean;
    readonly asKillIdentity: {
      readonly target: MultiAddress;
    } & Struct;
    readonly isAddSub: boolean;
    readonly asAddSub: {
      readonly sub: MultiAddress;
      readonly data: Data;
    } & Struct;
    readonly isRenameSub: boolean;
    readonly asRenameSub: {
      readonly sub: MultiAddress;
      readonly data: Data;
    } & Struct;
    readonly isRemoveSub: boolean;
    readonly asRemoveSub: {
      readonly sub: MultiAddress;
    } & Struct;
    readonly isQuitSub: boolean;
    readonly type: 'AddRegistrar' | 'SetIdentity' | 'SetSubs' | 'ClearIdentity' | 'RequestJudgement' | 'CancelRequest' | 'SetFee' | 'SetAccountId' | 'SetFields' | 'ProvideJudgement' | 'KillIdentity' | 'AddSub' | 'RenameSub' | 'RemoveSub' | 'QuitSub';
  }

  /** @name PalletIdentityIdentityInfo (240) */
  interface PalletIdentityIdentityInfo extends Struct {
    readonly additional: Vec<ITuple<[Data, Data]>>;
    readonly display: Data;
    readonly legal: Data;
    readonly web: Data;
    readonly riot: Data;
    readonly email: Data;
    readonly pgpFingerprint: Option<U8aFixed>;
    readonly image: Data;
    readonly twitter: Data;
  }

  /** @name PalletIdentityBitFlags (276) */
  interface PalletIdentityBitFlags extends Set {
    readonly isDisplay: boolean;
    readonly isLegal: boolean;
    readonly isWeb: boolean;
    readonly isRiot: boolean;
    readonly isEmail: boolean;
    readonly isPgpFingerprint: boolean;
    readonly isImage: boolean;
    readonly isTwitter: boolean;
  }

  /** @name PalletIdentityIdentityField (277) */
  interface PalletIdentityIdentityField extends Enum {
    readonly isDisplay: boolean;
    readonly isLegal: boolean;
    readonly isWeb: boolean;
    readonly isRiot: boolean;
    readonly isEmail: boolean;
    readonly isPgpFingerprint: boolean;
    readonly isImage: boolean;
    readonly isTwitter: boolean;
    readonly type: 'Display' | 'Legal' | 'Web' | 'Riot' | 'Email' | 'PgpFingerprint' | 'Image' | 'Twitter';
  }

  /** @name PalletIdentityJudgement (280) */
  interface PalletIdentityJudgement extends Enum {
    readonly isUnknown: boolean;
    readonly isFeePaid: boolean;
    readonly asFeePaid: u128;
    readonly isReasonable: boolean;
    readonly isKnownGood: boolean;
    readonly isOutOfDate: boolean;
    readonly isLowQuality: boolean;
    readonly isErroneous: boolean;
    readonly type: 'Unknown' | 'FeePaid' | 'Reasonable' | 'KnownGood' | 'OutOfDate' | 'LowQuality' | 'Erroneous';
  }

  /** @name PalletBalancesCall (281) */
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
    readonly type: 'Transfer' | 'SetBalance' | 'ForceTransfer' | 'TransferKeepAlive' | 'TransferAll' | 'ForceUnreserve';
  }

  /** @name PalletAssetsCall (282) */
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
    readonly type: 'Create' | 'ForceCreate' | 'Destroy' | 'Mint' | 'Burn' | 'Transfer' | 'TransferKeepAlive' | 'ForceTransfer' | 'Freeze' | 'Thaw' | 'FreezeAsset' | 'ThawAsset' | 'TransferOwnership' | 'SetTeam' | 'SetMetadata' | 'ClearMetadata' | 'ForceSetMetadata' | 'ForceClearMetadata' | 'ForceAssetStatus' | 'ApproveTransfer' | 'CancelApproval' | 'ForceCancelApproval' | 'TransferApproved' | 'Touch' | 'Refund';
  }

  /** @name PalletAssetsDestroyWitness (283) */
  interface PalletAssetsDestroyWitness extends Struct {
    readonly accounts: Compact<u32>;
    readonly sufficients: Compact<u32>;
    readonly approvals: Compact<u32>;
  }

  /** @name PalletAccountManagerCall (284) */
  interface PalletAccountManagerCall extends Enum {
    readonly isDeposit: boolean;
    readonly asDeposit: {
      readonly chargeId: H256;
      readonly payee: AccountId32;
      readonly chargeFee: u128;
      readonly offeredReward: u128;
      readonly source: T3rnPrimitivesClaimableBenefitSource;
      readonly role: T3rnPrimitivesClaimableCircuitRole;
      readonly recipient: Option<AccountId32>;
      readonly maybeAssetId: Option<u32>;
    } & Struct;
    readonly isFinalize: boolean;
    readonly asFinalize: {
      readonly chargeId: H256;
      readonly outcome: T3rnPrimitivesAccountManagerOutcome;
      readonly maybeRecipient: Option<AccountId32>;
      readonly maybeActualFees: Option<u128>;
    } & Struct;
    readonly type: 'Deposit' | 'Finalize';
  }

  /** @name T3rnPrimitivesClaimableBenefitSource (285) */
  interface T3rnPrimitivesClaimableBenefitSource extends Enum {
    readonly isBootstrapPool: boolean;
    readonly isInflation: boolean;
    readonly isTrafficFees: boolean;
    readonly isTrafficRewards: boolean;
    readonly isUnsettled: boolean;
    readonly isSlashTreasury: boolean;
    readonly type: 'BootstrapPool' | 'Inflation' | 'TrafficFees' | 'TrafficRewards' | 'Unsettled' | 'SlashTreasury';
  }

  /** @name T3rnPrimitivesClaimableCircuitRole (286) */
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
    readonly type: 'Ambassador' | 'Executor' | 'Attester' | 'Staker' | 'Collator' | 'ContractAuthor' | 'Relayer' | 'Requester' | 'Local';
  }

  /** @name T3rnPrimitivesAccountManagerOutcome (287) */
  interface T3rnPrimitivesAccountManagerOutcome extends Enum {
    readonly isUnexpectedFailure: boolean;
    readonly isRevert: boolean;
    readonly isCommit: boolean;
    readonly isSlash: boolean;
    readonly type: 'UnexpectedFailure' | 'Revert' | 'Commit' | 'Slash';
  }

  /** @name PalletTreasuryCall (288) */
  interface PalletTreasuryCall extends Enum {
    readonly isProposeSpend: boolean;
    readonly asProposeSpend: {
      readonly value: Compact<u128>;
      readonly beneficiary: MultiAddress;
    } & Struct;
    readonly isRejectProposal: boolean;
    readonly asRejectProposal: {
      readonly proposalId: Compact<u32>;
    } & Struct;
    readonly isApproveProposal: boolean;
    readonly asApproveProposal: {
      readonly proposalId: Compact<u32>;
    } & Struct;
    readonly isSpend: boolean;
    readonly asSpend: {
      readonly amount: Compact<u128>;
      readonly beneficiary: MultiAddress;
    } & Struct;
    readonly isRemoveApproval: boolean;
    readonly asRemoveApproval: {
      readonly proposalId: Compact<u32>;
    } & Struct;
    readonly type: 'ProposeSpend' | 'RejectProposal' | 'ApproveProposal' | 'Spend' | 'RemoveApproval';
  }

  /** @name PalletAuthorshipCall (293) */
  interface PalletAuthorshipCall extends Enum {
    readonly isSetUncles: boolean;
    readonly asSetUncles: {
      readonly newUncles: Vec<SpRuntimeHeader>;
    } & Struct;
    readonly type: 'SetUncles';
  }

  /** @name SpRuntimeHeader (295) */
  interface SpRuntimeHeader extends Struct {
    readonly parentHash: H256;
    readonly number: Compact<u32>;
    readonly stateRoot: H256;
    readonly extrinsicsRoot: H256;
    readonly digest: SpRuntimeDigest;
  }

  /** @name SpRuntimeBlakeTwo256 (296) */
  type SpRuntimeBlakeTwo256 = Null;

  /** @name PalletCollatorSelectionCall (297) */
  interface PalletCollatorSelectionCall extends Enum {
    readonly isSetInvulnerables: boolean;
    readonly asSetInvulnerables: {
      readonly new_: Vec<AccountId32>;
    } & Struct;
    readonly isSetDesiredCandidates: boolean;
    readonly asSetDesiredCandidates: {
      readonly max: u32;
    } & Struct;
    readonly isSetCandidacyBond: boolean;
    readonly asSetCandidacyBond: {
      readonly bond: u128;
    } & Struct;
    readonly isRegisterAsCandidate: boolean;
    readonly isLeaveIntent: boolean;
    readonly type: 'SetInvulnerables' | 'SetDesiredCandidates' | 'SetCandidacyBond' | 'RegisterAsCandidate' | 'LeaveIntent';
  }

  /** @name PalletSessionCall (298) */
  interface PalletSessionCall extends Enum {
    readonly isSetKeys: boolean;
    readonly asSetKeys: {
      readonly keys_: T0rnParachainRuntimeParachainConfigSessionKeys;
      readonly proof: Bytes;
    } & Struct;
    readonly isPurgeKeys: boolean;
    readonly type: 'SetKeys' | 'PurgeKeys';
  }

  /** @name T0rnParachainRuntimeParachainConfigSessionKeys (299) */
  interface T0rnParachainRuntimeParachainConfigSessionKeys extends Struct {
    readonly aura: SpConsensusAuraSr25519AppSr25519Public;
  }

  /** @name SpConsensusAuraSr25519AppSr25519Public (300) */
  interface SpConsensusAuraSr25519AppSr25519Public extends SpCoreSr25519Public {}

  /** @name SpCoreSr25519Public (301) */
  interface SpCoreSr25519Public extends U8aFixed {}

  /** @name CumulusPalletXcmpQueueCall (302) */
  interface CumulusPalletXcmpQueueCall extends Enum {
    readonly isServiceOverweight: boolean;
    readonly asServiceOverweight: {
      readonly index: u64;
      readonly weightLimit: u64;
    } & Struct;
    readonly isSuspendXcmExecution: boolean;
    readonly isResumeXcmExecution: boolean;
    readonly isUpdateSuspendThreshold: boolean;
    readonly asUpdateSuspendThreshold: {
      readonly new_: u32;
    } & Struct;
    readonly isUpdateDropThreshold: boolean;
    readonly asUpdateDropThreshold: {
      readonly new_: u32;
    } & Struct;
    readonly isUpdateResumeThreshold: boolean;
    readonly asUpdateResumeThreshold: {
      readonly new_: u32;
    } & Struct;
    readonly isUpdateThresholdWeight: boolean;
    readonly asUpdateThresholdWeight: {
      readonly new_: u64;
    } & Struct;
    readonly isUpdateWeightRestrictDecay: boolean;
    readonly asUpdateWeightRestrictDecay: {
      readonly new_: u64;
    } & Struct;
    readonly isUpdateXcmpMaxIndividualWeight: boolean;
    readonly asUpdateXcmpMaxIndividualWeight: {
      readonly new_: u64;
    } & Struct;
    readonly type: 'ServiceOverweight' | 'SuspendXcmExecution' | 'ResumeXcmExecution' | 'UpdateSuspendThreshold' | 'UpdateDropThreshold' | 'UpdateResumeThreshold' | 'UpdateThresholdWeight' | 'UpdateWeightRestrictDecay' | 'UpdateXcmpMaxIndividualWeight';
  }

  /** @name PalletXcmCall (303) */
  interface PalletXcmCall extends Enum {
    readonly isSend: boolean;
    readonly asSend: {
      readonly dest: XcmVersionedMultiLocation;
      readonly message: XcmVersionedXcm;
    } & Struct;
    readonly isTeleportAssets: boolean;
    readonly asTeleportAssets: {
      readonly dest: XcmVersionedMultiLocation;
      readonly beneficiary: XcmVersionedMultiLocation;
      readonly assets: XcmVersionedMultiAssets;
      readonly feeAssetItem: u32;
    } & Struct;
    readonly isReserveTransferAssets: boolean;
    readonly asReserveTransferAssets: {
      readonly dest: XcmVersionedMultiLocation;
      readonly beneficiary: XcmVersionedMultiLocation;
      readonly assets: XcmVersionedMultiAssets;
      readonly feeAssetItem: u32;
    } & Struct;
    readonly isExecute: boolean;
    readonly asExecute: {
      readonly message: XcmVersionedXcm;
      readonly maxWeight: u64;
    } & Struct;
    readonly isForceXcmVersion: boolean;
    readonly asForceXcmVersion: {
      readonly location: XcmV1MultiLocation;
      readonly xcmVersion: u32;
    } & Struct;
    readonly isForceDefaultXcmVersion: boolean;
    readonly asForceDefaultXcmVersion: {
      readonly maybeXcmVersion: Option<u32>;
    } & Struct;
    readonly isForceSubscribeVersionNotify: boolean;
    readonly asForceSubscribeVersionNotify: {
      readonly location: XcmVersionedMultiLocation;
    } & Struct;
    readonly isForceUnsubscribeVersionNotify: boolean;
    readonly asForceUnsubscribeVersionNotify: {
      readonly location: XcmVersionedMultiLocation;
    } & Struct;
    readonly isLimitedReserveTransferAssets: boolean;
    readonly asLimitedReserveTransferAssets: {
      readonly dest: XcmVersionedMultiLocation;
      readonly beneficiary: XcmVersionedMultiLocation;
      readonly assets: XcmVersionedMultiAssets;
      readonly feeAssetItem: u32;
      readonly weightLimit: XcmV2WeightLimit;
    } & Struct;
    readonly isLimitedTeleportAssets: boolean;
    readonly asLimitedTeleportAssets: {
      readonly dest: XcmVersionedMultiLocation;
      readonly beneficiary: XcmVersionedMultiLocation;
      readonly assets: XcmVersionedMultiAssets;
      readonly feeAssetItem: u32;
      readonly weightLimit: XcmV2WeightLimit;
    } & Struct;
    readonly type: 'Send' | 'TeleportAssets' | 'ReserveTransferAssets' | 'Execute' | 'ForceXcmVersion' | 'ForceDefaultXcmVersion' | 'ForceSubscribeVersionNotify' | 'ForceUnsubscribeVersionNotify' | 'LimitedReserveTransferAssets' | 'LimitedTeleportAssets';
  }

  /** @name XcmVersionedXcm (304) */
  interface XcmVersionedXcm extends Enum {
    readonly isV0: boolean;
    readonly asV0: XcmV0Xcm;
    readonly isV1: boolean;
    readonly asV1: XcmV1Xcm;
    readonly isV2: boolean;
    readonly asV2: XcmV2Xcm;
    readonly type: 'V0' | 'V1' | 'V2';
  }

  /** @name XcmV0Xcm (305) */
  interface XcmV0Xcm extends Enum {
    readonly isWithdrawAsset: boolean;
    readonly asWithdrawAsset: {
      readonly assets: Vec<XcmV0MultiAsset>;
      readonly effects: Vec<XcmV0Order>;
    } & Struct;
    readonly isReserveAssetDeposit: boolean;
    readonly asReserveAssetDeposit: {
      readonly assets: Vec<XcmV0MultiAsset>;
      readonly effects: Vec<XcmV0Order>;
    } & Struct;
    readonly isTeleportAsset: boolean;
    readonly asTeleportAsset: {
      readonly assets: Vec<XcmV0MultiAsset>;
      readonly effects: Vec<XcmV0Order>;
    } & Struct;
    readonly isQueryResponse: boolean;
    readonly asQueryResponse: {
      readonly queryId: Compact<u64>;
      readonly response: XcmV0Response;
    } & Struct;
    readonly isTransferAsset: boolean;
    readonly asTransferAsset: {
      readonly assets: Vec<XcmV0MultiAsset>;
      readonly dest: XcmV0MultiLocation;
    } & Struct;
    readonly isTransferReserveAsset: boolean;
    readonly asTransferReserveAsset: {
      readonly assets: Vec<XcmV0MultiAsset>;
      readonly dest: XcmV0MultiLocation;
      readonly effects: Vec<XcmV0Order>;
    } & Struct;
    readonly isTransact: boolean;
    readonly asTransact: {
      readonly originType: XcmV0OriginKind;
      readonly requireWeightAtMost: u64;
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
    readonly isRelayedFrom: boolean;
    readonly asRelayedFrom: {
      readonly who: XcmV0MultiLocation;
      readonly message: XcmV0Xcm;
    } & Struct;
    readonly type: 'WithdrawAsset' | 'ReserveAssetDeposit' | 'TeleportAsset' | 'QueryResponse' | 'TransferAsset' | 'TransferReserveAsset' | 'Transact' | 'HrmpNewChannelOpenRequest' | 'HrmpChannelAccepted' | 'HrmpChannelClosing' | 'RelayedFrom';
  }

  /** @name XcmV0Order (307) */
  interface XcmV0Order extends Enum {
    readonly isNull: boolean;
    readonly isDepositAsset: boolean;
    readonly asDepositAsset: {
      readonly assets: Vec<XcmV0MultiAsset>;
      readonly dest: XcmV0MultiLocation;
    } & Struct;
    readonly isDepositReserveAsset: boolean;
    readonly asDepositReserveAsset: {
      readonly assets: Vec<XcmV0MultiAsset>;
      readonly dest: XcmV0MultiLocation;
      readonly effects: Vec<XcmV0Order>;
    } & Struct;
    readonly isExchangeAsset: boolean;
    readonly asExchangeAsset: {
      readonly give: Vec<XcmV0MultiAsset>;
      readonly receive: Vec<XcmV0MultiAsset>;
    } & Struct;
    readonly isInitiateReserveWithdraw: boolean;
    readonly asInitiateReserveWithdraw: {
      readonly assets: Vec<XcmV0MultiAsset>;
      readonly reserve: XcmV0MultiLocation;
      readonly effects: Vec<XcmV0Order>;
    } & Struct;
    readonly isInitiateTeleport: boolean;
    readonly asInitiateTeleport: {
      readonly assets: Vec<XcmV0MultiAsset>;
      readonly dest: XcmV0MultiLocation;
      readonly effects: Vec<XcmV0Order>;
    } & Struct;
    readonly isQueryHolding: boolean;
    readonly asQueryHolding: {
      readonly queryId: Compact<u64>;
      readonly dest: XcmV0MultiLocation;
      readonly assets: Vec<XcmV0MultiAsset>;
    } & Struct;
    readonly isBuyExecution: boolean;
    readonly asBuyExecution: {
      readonly fees: XcmV0MultiAsset;
      readonly weight: u64;
      readonly debt: u64;
      readonly haltOnError: bool;
      readonly xcm: Vec<XcmV0Xcm>;
    } & Struct;
    readonly type: 'Null' | 'DepositAsset' | 'DepositReserveAsset' | 'ExchangeAsset' | 'InitiateReserveWithdraw' | 'InitiateTeleport' | 'QueryHolding' | 'BuyExecution';
  }

  /** @name XcmV0Response (309) */
  interface XcmV0Response extends Enum {
    readonly isAssets: boolean;
    readonly asAssets: Vec<XcmV0MultiAsset>;
    readonly type: 'Assets';
  }

  /** @name XcmV1Xcm (310) */
  interface XcmV1Xcm extends Enum {
    readonly isWithdrawAsset: boolean;
    readonly asWithdrawAsset: {
      readonly assets: XcmV1MultiassetMultiAssets;
      readonly effects: Vec<XcmV1Order>;
    } & Struct;
    readonly isReserveAssetDeposited: boolean;
    readonly asReserveAssetDeposited: {
      readonly assets: XcmV1MultiassetMultiAssets;
      readonly effects: Vec<XcmV1Order>;
    } & Struct;
    readonly isReceiveTeleportedAsset: boolean;
    readonly asReceiveTeleportedAsset: {
      readonly assets: XcmV1MultiassetMultiAssets;
      readonly effects: Vec<XcmV1Order>;
    } & Struct;
    readonly isQueryResponse: boolean;
    readonly asQueryResponse: {
      readonly queryId: Compact<u64>;
      readonly response: XcmV1Response;
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
      readonly effects: Vec<XcmV1Order>;
    } & Struct;
    readonly isTransact: boolean;
    readonly asTransact: {
      readonly originType: XcmV0OriginKind;
      readonly requireWeightAtMost: u64;
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
    readonly isRelayedFrom: boolean;
    readonly asRelayedFrom: {
      readonly who: XcmV1MultilocationJunctions;
      readonly message: XcmV1Xcm;
    } & Struct;
    readonly isSubscribeVersion: boolean;
    readonly asSubscribeVersion: {
      readonly queryId: Compact<u64>;
      readonly maxResponseWeight: Compact<u64>;
    } & Struct;
    readonly isUnsubscribeVersion: boolean;
    readonly type: 'WithdrawAsset' | 'ReserveAssetDeposited' | 'ReceiveTeleportedAsset' | 'QueryResponse' | 'TransferAsset' | 'TransferReserveAsset' | 'Transact' | 'HrmpNewChannelOpenRequest' | 'HrmpChannelAccepted' | 'HrmpChannelClosing' | 'RelayedFrom' | 'SubscribeVersion' | 'UnsubscribeVersion';
  }

  /** @name XcmV1Order (312) */
  interface XcmV1Order extends Enum {
    readonly isNoop: boolean;
    readonly isDepositAsset: boolean;
    readonly asDepositAsset: {
      readonly assets: XcmV1MultiassetMultiAssetFilter;
      readonly maxAssets: u32;
      readonly beneficiary: XcmV1MultiLocation;
    } & Struct;
    readonly isDepositReserveAsset: boolean;
    readonly asDepositReserveAsset: {
      readonly assets: XcmV1MultiassetMultiAssetFilter;
      readonly maxAssets: u32;
      readonly dest: XcmV1MultiLocation;
      readonly effects: Vec<XcmV1Order>;
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
      readonly effects: Vec<XcmV1Order>;
    } & Struct;
    readonly isInitiateTeleport: boolean;
    readonly asInitiateTeleport: {
      readonly assets: XcmV1MultiassetMultiAssetFilter;
      readonly dest: XcmV1MultiLocation;
      readonly effects: Vec<XcmV1Order>;
    } & Struct;
    readonly isQueryHolding: boolean;
    readonly asQueryHolding: {
      readonly queryId: Compact<u64>;
      readonly dest: XcmV1MultiLocation;
      readonly assets: XcmV1MultiassetMultiAssetFilter;
    } & Struct;
    readonly isBuyExecution: boolean;
    readonly asBuyExecution: {
      readonly fees: XcmV1MultiAsset;
      readonly weight: u64;
      readonly debt: u64;
      readonly haltOnError: bool;
      readonly instructions: Vec<XcmV1Xcm>;
    } & Struct;
    readonly type: 'Noop' | 'DepositAsset' | 'DepositReserveAsset' | 'ExchangeAsset' | 'InitiateReserveWithdraw' | 'InitiateTeleport' | 'QueryHolding' | 'BuyExecution';
  }

  /** @name XcmV1Response (314) */
  interface XcmV1Response extends Enum {
    readonly isAssets: boolean;
    readonly asAssets: XcmV1MultiassetMultiAssets;
    readonly isVersion: boolean;
    readonly asVersion: u32;
    readonly type: 'Assets' | 'Version';
  }

  /** @name CumulusPalletDmpQueueCall (328) */
  interface CumulusPalletDmpQueueCall extends Enum {
    readonly isServiceOverweight: boolean;
    readonly asServiceOverweight: {
      readonly index: u64;
      readonly weightLimit: u64;
    } & Struct;
    readonly type: 'ServiceOverweight';
  }

  /** @name PalletXbiPortalCall (329) */
  interface PalletXbiPortalCall extends Enum {
    readonly isSend: boolean;
    readonly asSend: {
      readonly kind: XpChannelExecutionType;
      readonly msg: XpFormatXbiFormat;
    } & Struct;
    readonly isReceive: boolean;
    readonly asReceive: {
      readonly msg: XpChannelMessage;
    } & Struct;
    readonly isProcessQueue: boolean;
    readonly type: 'Send' | 'Receive' | 'ProcessQueue';
  }

  /** @name XpChannelExecutionType (330) */
  interface XpChannelExecutionType extends Enum {
    readonly isSync: boolean;
    readonly isAsync: boolean;
    readonly type: 'Sync' | 'Async';
  }

  /** @name PalletAssetRegistryCall (331) */
  interface PalletAssetRegistryCall extends Enum {
    readonly isRegister: boolean;
    readonly asRegister: {
      readonly location: XcmV1MultiLocation;
      readonly id: u32;
    } & Struct;
    readonly isRegisterInfo: boolean;
    readonly asRegisterInfo: {
      readonly info: PalletAssetRegistryAssetInfo;
    } & Struct;
    readonly type: 'Register' | 'RegisterInfo';
  }

  /** @name PalletAssetRegistryAssetInfo (332) */
  interface PalletAssetRegistryAssetInfo extends Struct {
    readonly id: u32;
    readonly capabilities: Vec<PalletAssetRegistryCapability>;
    readonly location: XcmV1MultiLocation;
  }

  /** @name PalletAssetRegistryCapability (334) */
  interface PalletAssetRegistryCapability extends Enum {
    readonly isTeleport: boolean;
    readonly asTeleport: Option<AccountId32>;
    readonly isReserve: boolean;
    readonly asReserve: Option<AccountId32>;
    readonly isPayable: boolean;
    readonly asPayable: {
      readonly feesPerWeight: Option<u128>;
    } & Struct;
    readonly type: 'Teleport' | 'Reserve' | 'Payable';
  }

  /** @name PalletXdnsCall (335) */
  interface PalletXdnsCall extends Enum {
    readonly isRebootSelfGateway: boolean;
    readonly asRebootSelfGateway: {
      readonly vendor: T3rnPrimitivesGatewayVendor;
    } & Struct;
    readonly isPurgeGatewayRecord: boolean;
    readonly asPurgeGatewayRecord: {
      readonly requester: AccountId32;
      readonly gatewayId: U8aFixed;
    } & Struct;
    readonly isUnlinkToken: boolean;
    readonly asUnlinkToken: {
      readonly gatewayId: U8aFixed;
      readonly tokenId: u32;
    } & Struct;
    readonly isPurgeTokenRecord: boolean;
    readonly asPurgeTokenRecord: {
      readonly tokenId: u32;
    } & Struct;
    readonly type: 'RebootSelfGateway' | 'PurgeGatewayRecord' | 'UnlinkToken' | 'PurgeTokenRecord';
  }

  /** @name PalletAttestersCall (336) */
  interface PalletAttestersCall extends Enum {
    readonly isRegisterAttester: boolean;
    readonly asRegisterAttester: {
      readonly selfNominateAmount: u128;
      readonly ecdsaKey: U8aFixed;
      readonly ed25519Key: U8aFixed;
      readonly sr25519Key: U8aFixed;
      readonly customCommission: Option<Percent>;
    } & Struct;
    readonly isDeregisterAttester: boolean;
    readonly isRemoveAttestationTarget: boolean;
    readonly asRemoveAttestationTarget: {
      readonly target: U8aFixed;
    } & Struct;
    readonly isAgreeToNewAttestationTarget: boolean;
    readonly asAgreeToNewAttestationTarget: {
      readonly target: U8aFixed;
      readonly recoverable: Bytes;
    } & Struct;
    readonly isForceActivateTarget: boolean;
    readonly asForceActivateTarget: {
      readonly target: U8aFixed;
    } & Struct;
    readonly isAddAttestationTarget: boolean;
    readonly asAddAttestationTarget: {
      readonly target: U8aFixed;
    } & Struct;
    readonly isSubmitAttestation: boolean;
    readonly asSubmitAttestation: {
      readonly message: H256;
      readonly signature: Bytes;
      readonly target: U8aFixed;
    } & Struct;
    readonly isCommitBatch: boolean;
    readonly asCommitBatch: {
      readonly target: U8aFixed;
      readonly targetInclusionProofEncoded: Bytes;
    } & Struct;
    readonly isSetConfirmationCost: boolean;
    readonly asSetConfirmationCost: {
      readonly target: U8aFixed;
      readonly cost: u128;
    } & Struct;
    readonly isNominate: boolean;
    readonly asNominate: {
      readonly attester: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isUnnominate: boolean;
    readonly asUnnominate: {
      readonly attester: AccountId32;
    } & Struct;
    readonly type: 'RegisterAttester' | 'DeregisterAttester' | 'RemoveAttestationTarget' | 'AgreeToNewAttestationTarget' | 'ForceActivateTarget' | 'AddAttestationTarget' | 'SubmitAttestation' | 'CommitBatch' | 'SetConfirmationCost' | 'Nominate' | 'Unnominate';
  }

  /** @name PalletRewardsCall (339) */
  interface PalletRewardsCall extends Enum {
    readonly isSetMaxRewardsExecutorsKickback: boolean;
    readonly asSetMaxRewardsExecutorsKickback: {
      readonly newKickback: Percent;
    } & Struct;
    readonly isTriggerDistribution: boolean;
    readonly isTurnOnOffDistribution: boolean;
    readonly isTurnOnOffClaims: boolean;
    readonly isTurnOnOffSettlementAccumulation: boolean;
    readonly isClaim: boolean;
    readonly asClaim: {
      readonly roleToClaim: Option<T3rnPrimitivesClaimableCircuitRole>;
    } & Struct;
    readonly type: 'SetMaxRewardsExecutorsKickback' | 'TriggerDistribution' | 'TurnOnOffDistribution' | 'TurnOnOffClaims' | 'TurnOnOffSettlementAccumulation' | 'Claim';
  }

  /** @name PalletContractsRegistryCall (341) */
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
    readonly type: 'AddNewContract' | 'Purge';
  }

  /** @name T3rnPrimitivesContractsRegistryRegistryContract (342) */
  interface T3rnPrimitivesContractsRegistryRegistryContract extends Struct {
    readonly codeTxt: Bytes;
    readonly bytes: Bytes;
    readonly author: T3rnPrimitivesContractsRegistryAuthorInfo;
    readonly abi: Option<Bytes>;
    readonly actionDescriptions: Vec<T3rnTypesGatewayContractActionDesc>;
    readonly info: Option<T3rnPrimitivesStorageRawAliveContractInfo>;
    readonly meta: T3rnPrimitivesContractMetadata;
  }

  /** @name T3rnPrimitivesContractsRegistryAuthorInfo (343) */
  interface T3rnPrimitivesContractsRegistryAuthorInfo extends Struct {
    readonly account: AccountId32;
    readonly feesPerSingleUse: Option<u128>;
  }

  /** @name T3rnTypesGatewayContractActionDesc (345) */
  interface T3rnTypesGatewayContractActionDesc extends Struct {
    readonly actionId: H256;
    readonly targetId: Option<U8aFixed>;
    readonly to: Option<AccountId32>;
  }

  /** @name T3rnPrimitivesStorageRawAliveContractInfo (348) */
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

  /** @name T3rnPrimitivesContractMetadata (350) */
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

  /** @name PalletCircuitCall (351) */
  interface PalletCircuitCall extends Enum {
    readonly isOnLocalTrigger: boolean;
    readonly asOnLocalTrigger: {
      readonly trigger: Bytes;
    } & Struct;
    readonly isOnXcmTrigger: boolean;
    readonly isOnRemoteGatewayTrigger: boolean;
    readonly isCancelXtx: boolean;
    readonly asCancelXtx: {
      readonly xtxId: H256;
    } & Struct;
    readonly isRevert: boolean;
    readonly asRevert: {
      readonly xtxId: H256;
    } & Struct;
    readonly isOnExtrinsicTrigger: boolean;
    readonly asOnExtrinsicTrigger: {
      readonly sideEffects: Vec<T3rnTypesSfxSideEffect>;
      readonly speedMode: T3rnPrimitivesSpeedMode;
    } & Struct;
    readonly isBidSfx: boolean;
    readonly asBidSfx: {
      readonly sfxId: H256;
      readonly bidAmount: u128;
    } & Struct;
    readonly isConfirmSideEffect: boolean;
    readonly asConfirmSideEffect: {
      readonly sfxId: H256;
      readonly confirmation: T3rnTypesSfxConfirmedSideEffect;
    } & Struct;
    readonly type: 'OnLocalTrigger' | 'OnXcmTrigger' | 'OnRemoteGatewayTrigger' | 'CancelXtx' | 'Revert' | 'OnExtrinsicTrigger' | 'BidSfx' | 'ConfirmSideEffect';
  }

  /** @name T3rnPrimitivesSpeedMode (352) */
  interface T3rnPrimitivesSpeedMode extends Enum {
    readonly isFast: boolean;
    readonly isRational: boolean;
    readonly isFinalized: boolean;
    readonly type: 'Fast' | 'Rational' | 'Finalized';
  }

  /** @name Pallet3vmCall (353) */
  type Pallet3vmCall = Null;

  /** @name PalletContractsCall (354) */
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
    readonly type: 'Call' | 'InstantiateWithCode' | 'Instantiate' | 'UploadCode' | 'RemoveCode';
  }

  /** @name PalletEvmCall (356) */
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
    readonly type: 'Withdraw' | 'Call' | 'Create' | 'Create2' | 'Claim';
  }

  /** @name PalletPortalCall (357) */
  interface PalletPortalCall extends Enum {
    readonly isRegisterGateway: boolean;
    readonly asRegisterGateway: {
      readonly gatewayId: U8aFixed;
      readonly tokenId: U8aFixed;
      readonly verificationVendor: T3rnPrimitivesGatewayVendor;
      readonly executionVendor: T3rnPrimitivesExecutionVendor;
      readonly codec: T3rnAbiRecodeCodec;
      readonly registrant: Option<AccountId32>;
      readonly escrowAccount: Option<AccountId32>;
      readonly allowedSideEffects: Vec<ITuple<[U8aFixed, Option<u8>]>>;
      readonly tokenProps: T3rnPrimitivesTokenInfo;
      readonly encodedRegistrationData: Bytes;
    } & Struct;
    readonly type: 'RegisterGateway';
  }

  /** @name T3rnAbiRecodeCodec (358) */
  interface T3rnAbiRecodeCodec extends Enum {
    readonly isScale: boolean;
    readonly isRlp: boolean;
    readonly type: 'Scale' | 'Rlp';
  }

  /** @name T3rnPrimitivesTokenInfo (362) */
  interface T3rnPrimitivesTokenInfo extends Enum {
    readonly isSubstrate: boolean;
    readonly asSubstrate: T3rnPrimitivesSubstrateToken;
    readonly isEthereum: boolean;
    readonly asEthereum: T3rnPrimitivesEthereumToken;
    readonly type: 'Substrate' | 'Ethereum';
  }

  /** @name T3rnPrimitivesSubstrateToken (363) */
  interface T3rnPrimitivesSubstrateToken extends Struct {
    readonly id: u32;
    readonly symbol: Bytes;
    readonly decimals: u8;
  }

  /** @name T3rnPrimitivesEthereumToken (364) */
  interface T3rnPrimitivesEthereumToken extends Struct {
    readonly symbol: Bytes;
    readonly decimals: u8;
    readonly address: Option<U8aFixed>;
  }

  /** @name PalletGrandpaFinalityVerifierCall (365) */
  interface PalletGrandpaFinalityVerifierCall extends Enum {
    readonly isSubmitHeaders: boolean;
    readonly asSubmitHeaders: {
      readonly range: Vec<SpRuntimeHeader>;
      readonly signedHeader: SpRuntimeHeader;
      readonly justification: PalletGrandpaFinalityVerifierBridgesHeaderChainJustificationGrandpaJustification;
    } & Struct;
    readonly isReset: boolean;
    readonly type: 'SubmitHeaders' | 'Reset';
  }

  /** @name PalletGrandpaFinalityVerifierBridgesHeaderChainJustificationGrandpaJustification (366) */
  interface PalletGrandpaFinalityVerifierBridgesHeaderChainJustificationGrandpaJustification extends Struct {
    readonly round: u64;
    readonly commit: FinalityGrandpaCommit;
    readonly votesAncestries: Vec<SpRuntimeHeader>;
  }

  /** @name FinalityGrandpaCommit (367) */
  interface FinalityGrandpaCommit extends Struct {
    readonly targetHash: H256;
    readonly targetNumber: u32;
    readonly precommits: Vec<FinalityGrandpaSignedPrecommit>;
  }

  /** @name SpFinalityGrandpaAppSignature (368) */
  interface SpFinalityGrandpaAppSignature extends SpCoreEd25519Signature {}

  /** @name SpCoreEd25519Signature (369) */
  interface SpCoreEd25519Signature extends U8aFixed {}

  /** @name SpFinalityGrandpaAppPublic (371) */
  interface SpFinalityGrandpaAppPublic extends SpCoreEd25519Public {}

  /** @name SpCoreEd25519Public (372) */
  interface SpCoreEd25519Public extends U8aFixed {}

  /** @name FinalityGrandpaSignedPrecommit (374) */
  interface FinalityGrandpaSignedPrecommit extends Struct {
    readonly precommit: FinalityGrandpaPrecommit;
    readonly signature: SpFinalityGrandpaAppSignature;
    readonly id: SpFinalityGrandpaAppPublic;
  }

  /** @name FinalityGrandpaPrecommit (375) */
  interface FinalityGrandpaPrecommit extends Struct {
    readonly targetHash: H256;
    readonly targetNumber: u32;
  }

  /** @name PalletEth2FinalityVerifierCall (378) */
  interface PalletEth2FinalityVerifierCall extends Enum {
    readonly isSubmitEpoch: boolean;
    readonly asSubmitEpoch: {
      readonly update: PalletEth2FinalityVerifierEpochUpdate;
    } & Struct;
    readonly isSubmitFork: boolean;
    readonly asSubmitFork: {
      readonly encodedNewUpdate: Bytes;
      readonly encodedOldUpdate: Bytes;
    } & Struct;
    readonly isAddNextSyncCommittee: boolean;
    readonly asAddNextSyncCommittee: {
      readonly nextSyncCommittee: PalletEth2FinalityVerifierSyncCommittee;
      readonly proof: PalletEth2FinalityVerifierMerkleProof;
      readonly proofSlot: u64;
    } & Struct;
    readonly isVerifyReceiptInclusion: boolean;
    readonly asVerifyReceiptInclusion: {
      readonly proof: PalletEth2FinalityVerifierEthereumReceiptInclusionProof;
      readonly submissionTargetHeight: Option<u32>;
    } & Struct;
    readonly isVerifyEventInclusion: boolean;
    readonly asVerifyEventInclusion: {
      readonly proof: PalletEth2FinalityVerifierEthereumEventInclusionProof;
      readonly submissionTargetHeight: Option<u32>;
    } & Struct;
    readonly type: 'SubmitEpoch' | 'SubmitFork' | 'AddNextSyncCommittee' | 'VerifyReceiptInclusion' | 'VerifyEventInclusion';
  }

  /** @name PalletEth2FinalityVerifierEpochUpdate (379) */
  interface PalletEth2FinalityVerifierEpochUpdate extends Struct {
    readonly attestedBeaconHeader: PalletEth2FinalityVerifierBeaconBlockHeader;
    readonly signature: U8aFixed;
    readonly signerBits: Vec<bool>;
    readonly justifiedProof: PalletEth2FinalityVerifierMerkleProof;
    readonly finalizedProof: PalletEth2FinalityVerifierMerkleProof;
    readonly executionHeader: PalletEth2FinalityVerifierExecutionHeader;
    readonly executionProof: PalletEth2FinalityVerifierMerkleProof;
    readonly executionRange: Vec<PalletEth2FinalityVerifierExecutionHeader>;
  }

  /** @name PalletEth2FinalityVerifierBeaconBlockHeader (380) */
  interface PalletEth2FinalityVerifierBeaconBlockHeader extends Struct {
    readonly slot: u64;
    readonly proposerIndex: u64;
    readonly parentRoot: U8aFixed;
    readonly stateRoot: U8aFixed;
    readonly bodyRoot: U8aFixed;
  }

  /** @name PalletEth2FinalityVerifierMerkleProof (383) */
  interface PalletEth2FinalityVerifierMerkleProof extends Struct {
    readonly gIndex: u64;
    readonly witness: Vec<U8aFixed>;
  }

  /** @name PalletEth2FinalityVerifierExecutionHeader (385) */
  interface PalletEth2FinalityVerifierExecutionHeader extends Struct {
    readonly parentHash: U8aFixed;
    readonly feeRecipient: U8aFixed;
    readonly stateRoot: U8aFixed;
    readonly receiptsRoot: U8aFixed;
    readonly logsBloom: EthbloomBloom;
    readonly prevRandao: U8aFixed;
    readonly blockNumber: u64;
    readonly gasLimit: u64;
    readonly gasUsed: u64;
    readonly timestamp: u64;
    readonly extraData: Bytes;
    readonly baseFeePerGas: U256;
    readonly blockHash: U8aFixed;
    readonly transactionsRoot: U8aFixed;
    readonly withdrawalsRoot: U8aFixed;
  }

  /** @name EthbloomBloom (386) */
  interface EthbloomBloom extends U8aFixed {}

  /** @name PalletEth2FinalityVerifierSyncCommittee (390) */
  interface PalletEth2FinalityVerifierSyncCommittee extends Struct {
    readonly pubs: Vec<U8aFixed>;
    readonly aggr: U8aFixed;
  }

  /** @name PalletEth2FinalityVerifierEthereumReceiptInclusionProof (393) */
  interface PalletEth2FinalityVerifierEthereumReceiptInclusionProof extends Struct {
    readonly blockNumber: u64;
    readonly witness: Vec<Bytes>;
    readonly index: Bytes;
  }

  /** @name PalletEth2FinalityVerifierEthereumEventInclusionProof (394) */
  interface PalletEth2FinalityVerifierEthereumEventInclusionProof extends Struct {
    readonly blockNumber: u64;
    readonly witness: Vec<Bytes>;
    readonly index: Bytes;
    readonly event: Bytes;
  }

  /** @name PalletMaintenanceModeCall (395) */
  interface PalletMaintenanceModeCall extends Enum {
    readonly isEnterMaintenanceMode: boolean;
    readonly isResumeNormalOperation: boolean;
    readonly type: 'EnterMaintenanceMode' | 'ResumeNormalOperation';
  }

  /** @name PalletSudoCall (396) */
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
    readonly type: 'Sudo' | 'SudoUncheckedWeight' | 'SetKey' | 'SudoAs';
  }

  /** @name PalletSchedulerError (397) */
  interface PalletSchedulerError extends Enum {
    readonly isFailedToSchedule: boolean;
    readonly isNotFound: boolean;
    readonly isTargetBlockNumberInPast: boolean;
    readonly isRescheduleNoChange: boolean;
    readonly type: 'FailedToSchedule' | 'NotFound' | 'TargetBlockNumberInPast' | 'RescheduleNoChange';
  }

  /** @name PalletUtilityError (398) */
  interface PalletUtilityError extends Enum {
    readonly isTooManyCalls: boolean;
    readonly type: 'TooManyCalls';
  }

  /** @name PalletIdentityRegistration (399) */
  interface PalletIdentityRegistration extends Struct {
    readonly judgements: Vec<ITuple<[u32, PalletIdentityJudgement]>>;
    readonly deposit: u128;
    readonly info: PalletIdentityIdentityInfo;
  }

  /** @name PalletIdentityRegistrarInfo (407) */
  interface PalletIdentityRegistrarInfo extends Struct {
    readonly account: AccountId32;
    readonly fee: u128;
    readonly fields: PalletIdentityBitFlags;
  }

  /** @name PalletIdentityError (409) */
  interface PalletIdentityError extends Enum {
    readonly isTooManySubAccounts: boolean;
    readonly isNotFound: boolean;
    readonly isNotNamed: boolean;
    readonly isEmptyIndex: boolean;
    readonly isFeeChanged: boolean;
    readonly isNoIdentity: boolean;
    readonly isStickyJudgement: boolean;
    readonly isJudgementGiven: boolean;
    readonly isInvalidJudgement: boolean;
    readonly isInvalidIndex: boolean;
    readonly isInvalidTarget: boolean;
    readonly isTooManyFields: boolean;
    readonly isTooManyRegistrars: boolean;
    readonly isAlreadyClaimed: boolean;
    readonly isNotSub: boolean;
    readonly isNotOwned: boolean;
    readonly type: 'TooManySubAccounts' | 'NotFound' | 'NotNamed' | 'EmptyIndex' | 'FeeChanged' | 'NoIdentity' | 'StickyJudgement' | 'JudgementGiven' | 'InvalidJudgement' | 'InvalidIndex' | 'InvalidTarget' | 'TooManyFields' | 'TooManyRegistrars' | 'AlreadyClaimed' | 'NotSub' | 'NotOwned';
  }

  /** @name PalletBalancesBalanceLock (412) */
  interface PalletBalancesBalanceLock extends Struct {
    readonly id: U8aFixed;
    readonly amount: u128;
    readonly reasons: PalletBalancesReasons;
  }

  /** @name PalletBalancesReasons (413) */
  interface PalletBalancesReasons extends Enum {
    readonly isFee: boolean;
    readonly isMisc: boolean;
    readonly isAll: boolean;
    readonly type: 'Fee' | 'Misc' | 'All';
  }

  /** @name PalletBalancesReserveData (416) */
  interface PalletBalancesReserveData extends Struct {
    readonly id: U8aFixed;
    readonly amount: u128;
  }

  /** @name PalletBalancesReleases (418) */
  interface PalletBalancesReleases extends Enum {
    readonly isV100: boolean;
    readonly isV200: boolean;
    readonly type: 'V100' | 'V200';
  }

  /** @name PalletBalancesError (419) */
  interface PalletBalancesError extends Enum {
    readonly isVestingBalance: boolean;
    readonly isLiquidityRestrictions: boolean;
    readonly isInsufficientBalance: boolean;
    readonly isExistentialDeposit: boolean;
    readonly isKeepAlive: boolean;
    readonly isExistingVestingSchedule: boolean;
    readonly isDeadAccount: boolean;
    readonly isTooManyReserves: boolean;
    readonly type: 'VestingBalance' | 'LiquidityRestrictions' | 'InsufficientBalance' | 'ExistentialDeposit' | 'KeepAlive' | 'ExistingVestingSchedule' | 'DeadAccount' | 'TooManyReserves';
  }

  /** @name PalletTransactionPaymentReleases (421) */
  interface PalletTransactionPaymentReleases extends Enum {
    readonly isV1Ancient: boolean;
    readonly isV2: boolean;
    readonly type: 'V1Ancient' | 'V2';
  }

  /** @name PalletAssetsAssetDetails (422) */
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

  /** @name PalletAssetsAssetAccount (424) */
  interface PalletAssetsAssetAccount extends Struct {
    readonly balance: u128;
    readonly isFrozen: bool;
    readonly reason: PalletAssetsExistenceReason;
    readonly extra: Null;
  }

  /** @name PalletAssetsExistenceReason (425) */
  interface PalletAssetsExistenceReason extends Enum {
    readonly isConsumer: boolean;
    readonly isSufficient: boolean;
    readonly isDepositHeld: boolean;
    readonly asDepositHeld: u128;
    readonly isDepositRefunded: boolean;
    readonly type: 'Consumer' | 'Sufficient' | 'DepositHeld' | 'DepositRefunded';
  }

  /** @name PalletAssetsApproval (427) */
  interface PalletAssetsApproval extends Struct {
    readonly amount: u128;
    readonly deposit: u128;
  }

  /** @name PalletAssetsAssetMetadata (428) */
  interface PalletAssetsAssetMetadata extends Struct {
    readonly deposit: u128;
    readonly name: Bytes;
    readonly symbol: Bytes;
    readonly decimals: u8;
    readonly isFrozen: bool;
  }

  /** @name PalletAssetsError (430) */
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
    readonly type: 'BalanceLow' | 'NoAccount' | 'NoPermission' | 'Unknown' | 'Frozen' | 'InUse' | 'BadWitness' | 'MinBalanceZero' | 'NoProvider' | 'BadMetadata' | 'Unapproved' | 'WouldDie' | 'AlreadyExists' | 'NoDeposit' | 'WouldBurn';
  }

  /** @name T3rnPrimitivesAccountManagerRequestCharge (431) */
  interface T3rnPrimitivesAccountManagerRequestCharge extends Struct {
    readonly payee: AccountId32;
    readonly offeredReward: u128;
    readonly maybeAssetId: Option<u32>;
    readonly chargeFee: u128;
    readonly recipient: Option<AccountId32>;
    readonly source: T3rnPrimitivesClaimableBenefitSource;
    readonly role: T3rnPrimitivesClaimableCircuitRole;
  }

  /** @name T3rnPrimitivesCommonRoundInfo (433) */
  interface T3rnPrimitivesCommonRoundInfo extends Struct {
    readonly index: u32;
    readonly head: u32;
    readonly term: u32;
  }

  /** @name T3rnPrimitivesAccountManagerSettlement (434) */
  interface T3rnPrimitivesAccountManagerSettlement extends Struct {
    readonly requester: AccountId32;
    readonly recipient: AccountId32;
    readonly settlementAmount: u128;
    readonly outcome: T3rnPrimitivesAccountManagerOutcome;
    readonly source: T3rnPrimitivesClaimableBenefitSource;
    readonly role: T3rnPrimitivesClaimableCircuitRole;
  }

  /** @name PalletAccountManagerError (435) */
  interface PalletAccountManagerError extends Enum {
    readonly isPendingChargeNotFoundAtCommit: boolean;
    readonly isPendingChargeNotFoundAtRefund: boolean;
    readonly isExecutionNotRegistered: boolean;
    readonly isExecutionAlreadyRegistered: boolean;
    readonly isSkippingEmptyCharges: boolean;
    readonly isNoChargeOfGivenIdRegistered: boolean;
    readonly isChargeAlreadyRegistered: boolean;
    readonly isChargeOrSettlementCalculationOverflow: boolean;
    readonly isChargeOrSettlementActualFeesOutgrowReserved: boolean;
    readonly isDecodingExecutionIDFailed: boolean;
    readonly isTransferDepositFailedOldChargeNotFound: boolean;
    readonly isTransferDepositFailedToReleasePreviousCharge: boolean;
    readonly type: 'PendingChargeNotFoundAtCommit' | 'PendingChargeNotFoundAtRefund' | 'ExecutionNotRegistered' | 'ExecutionAlreadyRegistered' | 'SkippingEmptyCharges' | 'NoChargeOfGivenIdRegistered' | 'ChargeAlreadyRegistered' | 'ChargeOrSettlementCalculationOverflow' | 'ChargeOrSettlementActualFeesOutgrowReserved' | 'DecodingExecutionIDFailed' | 'TransferDepositFailedOldChargeNotFound' | 'TransferDepositFailedToReleasePreviousCharge';
  }

  /** @name PalletTreasuryProposal (436) */
  interface PalletTreasuryProposal extends Struct {
    readonly proposer: AccountId32;
    readonly value: u128;
    readonly beneficiary: AccountId32;
    readonly bond: u128;
  }

  /** @name FrameSupportPalletId (440) */
  interface FrameSupportPalletId extends U8aFixed {}

  /** @name PalletTreasuryError (441) */
  interface PalletTreasuryError extends Enum {
    readonly isInsufficientProposersBalance: boolean;
    readonly isInvalidIndex: boolean;
    readonly isTooManyApprovals: boolean;
    readonly isInsufficientPermission: boolean;
    readonly isProposalNotApproved: boolean;
    readonly type: 'InsufficientProposersBalance' | 'InvalidIndex' | 'TooManyApprovals' | 'InsufficientPermission' | 'ProposalNotApproved';
  }

  /** @name PalletAuthorshipUncleEntryItem (447) */
  interface PalletAuthorshipUncleEntryItem extends Enum {
    readonly isInclusionHeight: boolean;
    readonly asInclusionHeight: u32;
    readonly isUncle: boolean;
    readonly asUncle: ITuple<[H256, Option<AccountId32>]>;
    readonly type: 'InclusionHeight' | 'Uncle';
  }

  /** @name PalletAuthorshipError (449) */
  interface PalletAuthorshipError extends Enum {
    readonly isInvalidUncleParent: boolean;
    readonly isUnclesAlreadySet: boolean;
    readonly isTooManyUncles: boolean;
    readonly isGenesisUncle: boolean;
    readonly isTooHighUncle: boolean;
    readonly isUncleAlreadyIncluded: boolean;
    readonly isOldUncle: boolean;
    readonly type: 'InvalidUncleParent' | 'UnclesAlreadySet' | 'TooManyUncles' | 'GenesisUncle' | 'TooHighUncle' | 'UncleAlreadyIncluded' | 'OldUncle';
  }

  /** @name PalletCollatorSelectionCandidateInfo (452) */
  interface PalletCollatorSelectionCandidateInfo extends Struct {
    readonly who: AccountId32;
    readonly deposit: u128;
  }

  /** @name PalletCollatorSelectionError (454) */
  interface PalletCollatorSelectionError extends Enum {
    readonly isTooManyCandidates: boolean;
    readonly isTooFewCandidates: boolean;
    readonly isUnknown: boolean;
    readonly isPermission: boolean;
    readonly isAlreadyCandidate: boolean;
    readonly isNotCandidate: boolean;
    readonly isTooManyInvulnerables: boolean;
    readonly isAlreadyInvulnerable: boolean;
    readonly isNoAssociatedValidatorId: boolean;
    readonly isValidatorNotRegistered: boolean;
    readonly type: 'TooManyCandidates' | 'TooFewCandidates' | 'Unknown' | 'Permission' | 'AlreadyCandidate' | 'NotCandidate' | 'TooManyInvulnerables' | 'AlreadyInvulnerable' | 'NoAssociatedValidatorId' | 'ValidatorNotRegistered';
  }

  /** @name SpCoreCryptoKeyTypeId (458) */
  interface SpCoreCryptoKeyTypeId extends U8aFixed {}

  /** @name PalletSessionError (459) */
  interface PalletSessionError extends Enum {
    readonly isInvalidProof: boolean;
    readonly isNoAssociatedValidatorId: boolean;
    readonly isDuplicatedKey: boolean;
    readonly isNoKeys: boolean;
    readonly isNoAccount: boolean;
    readonly type: 'InvalidProof' | 'NoAssociatedValidatorId' | 'DuplicatedKey' | 'NoKeys' | 'NoAccount';
  }

  /** @name CumulusPalletXcmpQueueInboundChannelDetails (464) */
  interface CumulusPalletXcmpQueueInboundChannelDetails extends Struct {
    readonly sender: u32;
    readonly state: CumulusPalletXcmpQueueInboundState;
    readonly messageMetadata: Vec<ITuple<[u32, PolkadotParachainPrimitivesXcmpMessageFormat]>>;
  }

  /** @name CumulusPalletXcmpQueueInboundState (465) */
  interface CumulusPalletXcmpQueueInboundState extends Enum {
    readonly isOk: boolean;
    readonly isSuspended: boolean;
    readonly type: 'Ok' | 'Suspended';
  }

  /** @name PolkadotParachainPrimitivesXcmpMessageFormat (468) */
  interface PolkadotParachainPrimitivesXcmpMessageFormat extends Enum {
    readonly isConcatenatedVersionedXcm: boolean;
    readonly isConcatenatedEncodedBlob: boolean;
    readonly isSignals: boolean;
    readonly type: 'ConcatenatedVersionedXcm' | 'ConcatenatedEncodedBlob' | 'Signals';
  }

  /** @name CumulusPalletXcmpQueueOutboundChannelDetails (471) */
  interface CumulusPalletXcmpQueueOutboundChannelDetails extends Struct {
    readonly recipient: u32;
    readonly state: CumulusPalletXcmpQueueOutboundState;
    readonly signalsExist: bool;
    readonly firstIndex: u16;
    readonly lastIndex: u16;
  }

  /** @name CumulusPalletXcmpQueueOutboundState (472) */
  interface CumulusPalletXcmpQueueOutboundState extends Enum {
    readonly isOk: boolean;
    readonly isSuspended: boolean;
    readonly type: 'Ok' | 'Suspended';
  }

  /** @name CumulusPalletXcmpQueueQueueConfigData (474) */
  interface CumulusPalletXcmpQueueQueueConfigData extends Struct {
    readonly suspendThreshold: u32;
    readonly dropThreshold: u32;
    readonly resumeThreshold: u32;
    readonly thresholdWeight: u64;
    readonly weightRestrictDecay: u64;
    readonly xcmpMaxIndividualWeight: u64;
  }

  /** @name CumulusPalletXcmpQueueError (476) */
  interface CumulusPalletXcmpQueueError extends Enum {
    readonly isFailedToSend: boolean;
    readonly isBadXcmOrigin: boolean;
    readonly isBadXcm: boolean;
    readonly isBadOverweightIndex: boolean;
    readonly isWeightOverLimit: boolean;
    readonly type: 'FailedToSend' | 'BadXcmOrigin' | 'BadXcm' | 'BadOverweightIndex' | 'WeightOverLimit';
  }

  /** @name PalletXcmError (477) */
  interface PalletXcmError extends Enum {
    readonly isUnreachable: boolean;
    readonly isSendFailure: boolean;
    readonly isFiltered: boolean;
    readonly isUnweighableMessage: boolean;
    readonly isDestinationNotInvertible: boolean;
    readonly isEmpty: boolean;
    readonly isCannotReanchor: boolean;
    readonly isTooManyAssets: boolean;
    readonly isInvalidOrigin: boolean;
    readonly isBadVersion: boolean;
    readonly isBadLocation: boolean;
    readonly isNoSubscription: boolean;
    readonly isAlreadySubscribed: boolean;
    readonly type: 'Unreachable' | 'SendFailure' | 'Filtered' | 'UnweighableMessage' | 'DestinationNotInvertible' | 'Empty' | 'CannotReanchor' | 'TooManyAssets' | 'InvalidOrigin' | 'BadVersion' | 'BadLocation' | 'NoSubscription' | 'AlreadySubscribed';
  }

  /** @name CumulusPalletXcmError (478) */
  type CumulusPalletXcmError = Null;

  /** @name CumulusPalletDmpQueueConfigData (479) */
  interface CumulusPalletDmpQueueConfigData extends Struct {
    readonly maxIndividual: u64;
  }

  /** @name CumulusPalletDmpQueuePageIndexData (480) */
  interface CumulusPalletDmpQueuePageIndexData extends Struct {
    readonly beginUsed: u32;
    readonly endUsed: u32;
    readonly overweightCount: u64;
  }

  /** @name CumulusPalletDmpQueueError (483) */
  interface CumulusPalletDmpQueueError extends Enum {
    readonly isUnknown: boolean;
    readonly isOverLimit: boolean;
    readonly type: 'Unknown' | 'OverLimit';
  }

  /** @name PalletXbiPortalError (486) */
  interface PalletXbiPortalError extends Enum {
    readonly isFailedToCastValue: boolean;
    readonly isFailedToCastAddress: boolean;
    readonly isFailedToCastHash: boolean;
    readonly isInstructionuctionNotAllowedHere: boolean;
    readonly isAlreadyCheckedIn: boolean;
    readonly isNotificationTimeoutDelivery: boolean;
    readonly isNotificationTimeoutExecution: boolean;
    readonly isCallbackUnsupported: boolean;
    readonly isEvmUnsupported: boolean;
    readonly isWasmUnsupported: boolean;
    readonly isCallNativeUnsupported: boolean;
    readonly isCallCustomUnsupported: boolean;
    readonly isTransferUnsupported: boolean;
    readonly isAssetsUnsupported: boolean;
    readonly isDefiUnsupported: boolean;
    readonly isArithmeticErrorOverflow: boolean;
    readonly isTransferFailed: boolean;
    readonly isResponseAlreadyStored: boolean;
    readonly type: 'FailedToCastValue' | 'FailedToCastAddress' | 'FailedToCastHash' | 'InstructionuctionNotAllowedHere' | 'AlreadyCheckedIn' | 'NotificationTimeoutDelivery' | 'NotificationTimeoutExecution' | 'CallbackUnsupported' | 'EvmUnsupported' | 'WasmUnsupported' | 'CallNativeUnsupported' | 'CallCustomUnsupported' | 'TransferUnsupported' | 'AssetsUnsupported' | 'DefiUnsupported' | 'ArithmeticErrorOverflow' | 'TransferFailed' | 'ResponseAlreadyStored';
  }

  /** @name PalletAssetRegistryError (487) */
  interface PalletAssetRegistryError extends Enum {
    readonly isNotFound: boolean;
    readonly isLocationUnallowed: boolean;
    readonly isCapabilitiesNotPermitted: boolean;
    readonly isShouldntExecuteMessage: boolean;
    readonly type: 'NotFound' | 'LocationUnallowed' | 'CapabilitiesNotPermitted' | 'ShouldntExecuteMessage';
  }

  /** @name T3rnAbiSfxAbi (488) */
  interface T3rnAbiSfxAbi extends Struct {
    readonly argsNames: Vec<ITuple<[Bytes, bool]>>;
    readonly maybePrefixMemo: Option<u8>;
    readonly egressAbiDescriptors: T3rnAbiSfxAbiPerCodecAbiDescriptors;
    readonly ingressAbiDescriptors: T3rnAbiSfxAbiPerCodecAbiDescriptors;
  }

  /** @name T3rnAbiSfxAbiPerCodecAbiDescriptors (491) */
  interface T3rnAbiSfxAbiPerCodecAbiDescriptors extends Struct {
    readonly forRlp: Bytes;
    readonly forScale: Bytes;
  }

  /** @name T3rnPrimitivesXdnsGatewayRecord (493) */
  interface T3rnPrimitivesXdnsGatewayRecord extends Struct {
    readonly gatewayId: U8aFixed;
    readonly verificationVendor: T3rnPrimitivesGatewayVendor;
    readonly executionVendor: T3rnPrimitivesExecutionVendor;
    readonly codec: T3rnAbiRecodeCodec;
    readonly registrant: Option<AccountId32>;
    readonly escrowAccount: Option<AccountId32>;
    readonly allowedSideEffects: Vec<ITuple<[U8aFixed, Option<u8>]>>;
  }

  /** @name T3rnPrimitivesXdnsTokenRecord (495) */
  interface T3rnPrimitivesXdnsTokenRecord extends Struct {
    readonly tokenId: u32;
    readonly gatewayId: U8aFixed;
    readonly tokenProps: T3rnPrimitivesTokenInfo;
  }

  /** @name T3rnPrimitivesGatewayActivity (499) */
  interface T3rnPrimitivesGatewayActivity extends Struct {
    readonly gatewayId: U8aFixed;
    readonly reportedAt: u32;
    readonly justifiedHeight: u32;
    readonly finalizedHeight: u32;
    readonly updatedHeight: u32;
    readonly attestationLatency: Option<T3rnPrimitivesAttestersLatencyStatus>;
    readonly securityLvl: T3rnTypesSfxSecurityLvl;
    readonly isActive: bool;
  }

  /** @name PalletXdnsError (501) */
  interface PalletXdnsError extends Enum {
    readonly isGatewayRecordAlreadyExists: boolean;
    readonly isXdnsRecordNotFound: boolean;
    readonly isTokenRecordAlreadyExists: boolean;
    readonly isTokenRecordNotFoundInAssetsOverlay: boolean;
    readonly isGatewayRecordNotFound: boolean;
    readonly isSideEffectABIAlreadyExists: boolean;
    readonly isSideEffectABINotFound: boolean;
    readonly isNoParachainInfoFound: boolean;
    readonly isTokenExecutionVendorMismatch: boolean;
    readonly isGatewayNotActive: boolean;
    readonly type: 'GatewayRecordAlreadyExists' | 'XdnsRecordNotFound' | 'TokenRecordAlreadyExists' | 'TokenRecordNotFoundInAssetsOverlay' | 'GatewayRecordNotFound' | 'SideEffectABIAlreadyExists' | 'SideEffectABINotFound' | 'NoParachainInfoFound' | 'TokenExecutionVendorMismatch' | 'GatewayNotActive';
  }

  /** @name T3rnPrimitivesAttestersAttesterInfo (502) */
  interface T3rnPrimitivesAttestersAttesterInfo extends Struct {
    readonly keyEd: U8aFixed;
    readonly keyEc: U8aFixed;
    readonly keySr: U8aFixed;
    readonly commission: Percent;
    readonly index: u32;
  }

  /** @name PalletAttestersError (508) */
  interface PalletAttestersError extends Enum {
    readonly isAttesterNotFound: boolean;
    readonly isArithmeticOverflow: boolean;
    readonly isInvalidSignature: boolean;
    readonly isInvalidMessage: boolean;
    readonly isInvalidTargetInclusionProof: boolean;
    readonly isAlreadyRegistered: boolean;
    readonly isPublicKeyMissing: boolean;
    readonly isAttestationSignatureInvalid: boolean;
    readonly isAttestationDoubleSignAttempt: boolean;
    readonly isNotActiveSet: boolean;
    readonly isNotInCurrentCommittee: boolean;
    readonly isAttesterDidNotAgreeToNewTarget: boolean;
    readonly isNotRegistered: boolean;
    readonly isNoNominationFound: boolean;
    readonly isAlreadyNominated: boolean;
    readonly isNominatorNotEnoughBalance: boolean;
    readonly isNominatorBondTooSmall: boolean;
    readonly isAttesterBondTooSmall: boolean;
    readonly isMissingNominations: boolean;
    readonly isBatchHashMismatch: boolean;
    readonly isBatchNotFound: boolean;
    readonly isCollusionWithPermanentSlashDetected: boolean;
    readonly isBatchFoundWithUnsignableStatus: boolean;
    readonly isRejectingFromSlashedAttester: boolean;
    readonly isTargetAlreadyActive: boolean;
    readonly isTargetNotActive: boolean;
    readonly isXdnsTargetNotActive: boolean;
    readonly isXdnsGatewayDoesNotHaveEscrowAddressRegistered: boolean;
    readonly isSfxAlreadyRequested: boolean;
    readonly isAddAttesterAlreadyRequested: boolean;
    readonly isRemoveAttesterAlreadyRequested: boolean;
    readonly isNextCommitteeAlreadyRequested: boolean;
    readonly isBanAttesterAlreadyRequested: boolean;
    readonly isBatchAlreadyCommitted: boolean;
    readonly isCommitteeSizeTooLarge: boolean;
    readonly type: 'AttesterNotFound' | 'ArithmeticOverflow' | 'InvalidSignature' | 'InvalidMessage' | 'InvalidTargetInclusionProof' | 'AlreadyRegistered' | 'PublicKeyMissing' | 'AttestationSignatureInvalid' | 'AttestationDoubleSignAttempt' | 'NotActiveSet' | 'NotInCurrentCommittee' | 'AttesterDidNotAgreeToNewTarget' | 'NotRegistered' | 'NoNominationFound' | 'AlreadyNominated' | 'NominatorNotEnoughBalance' | 'NominatorBondTooSmall' | 'AttesterBondTooSmall' | 'MissingNominations' | 'BatchHashMismatch' | 'BatchNotFound' | 'CollusionWithPermanentSlashDetected' | 'BatchFoundWithUnsignableStatus' | 'RejectingFromSlashedAttester' | 'TargetAlreadyActive' | 'TargetNotActive' | 'XdnsTargetNotActive' | 'XdnsGatewayDoesNotHaveEscrowAddressRegistered' | 'SfxAlreadyRequested' | 'AddAttesterAlreadyRequested' | 'RemoveAttesterAlreadyRequested' | 'NextCommitteeAlreadyRequested' | 'BanAttesterAlreadyRequested' | 'BatchAlreadyCommitted' | 'CommitteeSizeTooLarge';
  }

  /** @name PalletRewardsTreasuryBalanceSheet (512) */
  interface PalletRewardsTreasuryBalanceSheet extends Struct {
    readonly treasury: u128;
    readonly escrow: u128;
    readonly fee: u128;
    readonly slash: u128;
    readonly parachain: u128;
  }

  /** @name PalletRewardsDistributionRecord (514) */
  interface PalletRewardsDistributionRecord extends Struct {
    readonly blockNumber: u32;
    readonly attesterRewards: u128;
    readonly collatorRewards: u128;
    readonly executorRewards: u128;
    readonly treasuryRewards: u128;
    readonly available: u128;
    readonly distributed: u128;
  }

  /** @name T3rnPrimitivesClaimableClaimableArtifacts (516) */
  interface T3rnPrimitivesClaimableClaimableArtifacts extends Struct {
    readonly beneficiary: AccountId32;
    readonly role: T3rnPrimitivesClaimableCircuitRole;
    readonly totalRoundClaim: u128;
    readonly benefitSource: T3rnPrimitivesClaimableBenefitSource;
  }

  /** @name PalletRewardsError (517) */
  interface PalletRewardsError extends Enum {
    readonly isDistributionPeriodNotElapsed: boolean;
    readonly isNoPendingClaims: boolean;
    readonly isArithmeticOverflow: boolean;
    readonly isAttesterNotFound: boolean;
    readonly isTryIntoConversionU128ToBalanceFailed: boolean;
    readonly isHalted: boolean;
    readonly type: 'DistributionPeriodNotElapsed' | 'NoPendingClaims' | 'ArithmeticOverflow' | 'AttesterNotFound' | 'TryIntoConversionU128ToBalanceFailed' | 'Halted';
  }

  /** @name PalletContractsRegistryError (518) */
  interface PalletContractsRegistryError extends Enum {
    readonly isContractAlreadyExists: boolean;
    readonly isUnknownContract: boolean;
    readonly type: 'ContractAlreadyExists' | 'UnknownContract';
  }

  /** @name T3rnPrimitivesCircuitTypesXExecSignal (519) */
  interface T3rnPrimitivesCircuitTypesXExecSignal extends Struct {
    readonly requester: AccountId32;
    readonly requesterNonce: u32;
    readonly timeoutsAt: u32;
    readonly speedMode: T3rnPrimitivesSpeedMode;
    readonly delayStepsAt: Option<Vec<u32>>;
    readonly status: T3rnPrimitivesCircuitTypesCircuitStatus;
    readonly stepsCnt: ITuple<[u32, u32]>;
  }

  /** @name T3rnPrimitivesCircuitTypesCircuitStatus (521) */
  interface T3rnPrimitivesCircuitTypesCircuitStatus extends Enum {
    readonly isRequested: boolean;
    readonly isReserved: boolean;
    readonly isPendingBidding: boolean;
    readonly isInBidding: boolean;
    readonly isKilled: boolean;
    readonly asKilled: T3rnPrimitivesCircuitTypesCause;
    readonly isReady: boolean;
    readonly isPendingExecution: boolean;
    readonly isFinished: boolean;
    readonly isFinishedAllSteps: boolean;
    readonly isReverted: boolean;
    readonly asReverted: T3rnPrimitivesCircuitTypesCause;
    readonly isCommitted: boolean;
    readonly type: 'Requested' | 'Reserved' | 'PendingBidding' | 'InBidding' | 'Killed' | 'Ready' | 'PendingExecution' | 'Finished' | 'FinishedAllSteps' | 'Reverted' | 'Committed';
  }

  /** @name T3rnPrimitivesCircuitTypesCause (522) */
  interface T3rnPrimitivesCircuitTypesCause extends Enum {
    readonly isTimeout: boolean;
    readonly isIntentionalKill: boolean;
    readonly type: 'Timeout' | 'IntentionalKill';
  }

  /** @name T3rnPrimitivesVolatileLocalState (523) */
  interface T3rnPrimitivesVolatileLocalState extends Struct {
    readonly state: BTreeMap<U8aFixed, Bytes>;
  }

  /** @name T3rnSdkPrimitivesSignalExecutionSignal (529) */
  interface T3rnSdkPrimitivesSignalExecutionSignal extends Struct {
    readonly step: u32;
    readonly kind: T3rnSdkPrimitivesSignalSignalKind;
    readonly executionId: H256;
  }

  /** @name PalletCircuitError (531) */
  interface PalletCircuitError extends Enum {
    readonly isUpdateAttemptDoubleRevert: boolean;
    readonly isUpdateAttemptDoubleKill: boolean;
    readonly isUpdateStateTransitionDisallowed: boolean;
    readonly isUpdateForcedStateTransitionDisallowed: boolean;
    readonly isUpdateXtxTriggeredWithUnexpectedStatus: boolean;
    readonly isConfirmationFailed: boolean;
    readonly isApplyTriggeredWithUnexpectedStatus: boolean;
    readonly isBidderNotEnoughBalance: boolean;
    readonly isRequesterNotEnoughBalance: boolean;
    readonly isSanityAfterCreatingSFXDepositsFailed: boolean;
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
    readonly isBiddingRejectedBidTooHigh: boolean;
    readonly isBiddingRejectedInsuranceTooLow: boolean;
    readonly isBiddingRejectedBetterBidFound: boolean;
    readonly isBiddingRejectedFailedToDepositBidderBond: boolean;
    readonly isBiddingFailedExecutorsBalanceTooLowToReserve: boolean;
    readonly isInsuranceBondAlreadyDeposited: boolean;
    readonly isInvalidFTXStateEmptyBidForReadyXtx: boolean;
    readonly isInvalidFTXStateEmptyConfirmationForFinishedXtx: boolean;
    readonly isInvalidFTXStateUnassignedExecutorForReadySFX: boolean;
    readonly isInvalidFTXStateIncorrectExecutorForReadySFX: boolean;
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
    readonly isXtxDoesNotExist: boolean;
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
    readonly isFsxNotFoundById: boolean;
    readonly isXtxNotFound: boolean;
    readonly isLocalSideEffectExecutionNotApplicable: boolean;
    readonly isLocalExecutionUnauthorized: boolean;
    readonly isOnLocalTriggerFailedToSetupXtx: boolean;
    readonly isUnauthorizedCancellation: boolean;
    readonly isFailedToConvertSFX2XBI: boolean;
    readonly isFailedToCheckInOverXBI: boolean;
    readonly isFailedToCreateXBIMetadataDueToWrongAccountConversion: boolean;
    readonly isFailedToConvertXBIResult2SFXConfirmation: boolean;
    readonly isFailedToEnterXBIPortal: boolean;
    readonly isFailedToExitXBIPortal: boolean;
    readonly isFailedToCommitFSX: boolean;
    readonly isXbiExitFailedOnSFXConfirmation: boolean;
    readonly isUnsupportedRole: boolean;
    readonly isInvalidLocalTrigger: boolean;
    readonly isSignalQueueFull: boolean;
    readonly isArithmeticErrorOverflow: boolean;
    readonly isArithmeticErrorUnderflow: boolean;
    readonly isArithmeticErrorDivisionByZero: boolean;
    readonly type: 'UpdateAttemptDoubleRevert' | 'UpdateAttemptDoubleKill' | 'UpdateStateTransitionDisallowed' | 'UpdateForcedStateTransitionDisallowed' | 'UpdateXtxTriggeredWithUnexpectedStatus' | 'ConfirmationFailed' | 'ApplyTriggeredWithUnexpectedStatus' | 'BidderNotEnoughBalance' | 'RequesterNotEnoughBalance' | 'SanityAfterCreatingSFXDepositsFailed' | 'ContractXtxKilledRunOutOfFunds' | 'ChargingTransferFailed' | 'ChargingTransferFailedAtPendingExecution' | 'XtxChargeFailedRequesterBalanceTooLow' | 'XtxChargeBondDepositFailedCantAccessBid' | 'FinalizeSquareUpFailed' | 'CriticalStateSquareUpCalledToFinishWithoutFsxConfirmed' | 'RewardTransferFailed' | 'RefundTransferFailed' | 'SideEffectsValidationFailed' | 'InsuranceBondNotRequired' | 'BiddingInactive' | 'BiddingRejectedBidBelowDust' | 'BiddingRejectedBidTooHigh' | 'BiddingRejectedInsuranceTooLow' | 'BiddingRejectedBetterBidFound' | 'BiddingRejectedFailedToDepositBidderBond' | 'BiddingFailedExecutorsBalanceTooLowToReserve' | 'InsuranceBondAlreadyDeposited' | 'InvalidFTXStateEmptyBidForReadyXtx' | 'InvalidFTXStateEmptyConfirmationForFinishedXtx' | 'InvalidFTXStateUnassignedExecutorForReadySFX' | 'InvalidFTXStateIncorrectExecutorForReadySFX' | 'SetupFailed' | 'SetupFailedXtxNotFound' | 'SetupFailedXtxStorageArtifactsNotFound' | 'SetupFailedIncorrectXtxStatus' | 'SetupFailedDuplicatedXtx' | 'SetupFailedEmptyXtx' | 'SetupFailedXtxAlreadyFinished' | 'SetupFailedXtxWasDroppedAtBidding' | 'SetupFailedXtxReverted' | 'SetupFailedXtxRevertedTimeout' | 'XtxDoesNotExist' | 'InvalidFSXBidStateLocated' | 'EnactSideEffectsCanOnlyBeCalledWithMin1StepFinished' | 'FatalXtxTimeoutXtxIdNotMatched' | 'RelayEscrowedFailedNothingToConfirm' | 'FatalCommitSideEffectWithoutConfirmationAttempt' | 'FatalErroredCommitSideEffectConfirmationAttempt' | 'FatalErroredRevertSideEffectConfirmationAttempt' | 'FailedToHardenFullSideEffect' | 'ApplyFailed' | 'DeterminedForbiddenXtxStatus' | 'SideEffectIsAlreadyScheduledToExecuteOverXBI' | 'FsxNotFoundById' | 'XtxNotFound' | 'LocalSideEffectExecutionNotApplicable' | 'LocalExecutionUnauthorized' | 'OnLocalTriggerFailedToSetupXtx' | 'UnauthorizedCancellation' | 'FailedToConvertSFX2XBI' | 'FailedToCheckInOverXBI' | 'FailedToCreateXBIMetadataDueToWrongAccountConversion' | 'FailedToConvertXBIResult2SFXConfirmation' | 'FailedToEnterXBIPortal' | 'FailedToExitXBIPortal' | 'FailedToCommitFSX' | 'XbiExitFailedOnSFXConfirmation' | 'UnsupportedRole' | 'InvalidLocalTrigger' | 'SignalQueueFull' | 'ArithmeticErrorOverflow' | 'ArithmeticErrorUnderflow' | 'ArithmeticErrorDivisionByZero';
  }

  /** @name PalletClockError (532) */
  type PalletClockError = Null;

  /** @name Pallet3vmError (534) */
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
    readonly isInvalidArithmeticOverflow: boolean;
    readonly isDownstreamCircuit: boolean;
    readonly type: 'ExceededSignalBounceThreshold' | 'CannotTriggerWithoutSideEffects' | 'ContractNotFound' | 'InvalidOrigin' | 'CannotInstantiateContract' | 'ContractCannotRemunerate' | 'ContractCannotHaveStorage' | 'ContractCannotGenerateSideEffects' | 'InvalidPrecompilePointer' | 'InvalidPrecompileArgs' | 'InvalidArithmeticOverflow' | 'DownstreamCircuit';
  }

  /** @name PalletContractsWasmPrefabWasmModule (535) */
  interface PalletContractsWasmPrefabWasmModule extends Struct {
    readonly instructionWeightsVersion: Compact<u32>;
    readonly initial: Compact<u32>;
    readonly maximum: Compact<u32>;
    readonly code: Bytes;
    readonly author: Option<T3rnPrimitivesContractsRegistryAuthorInfo>;
    readonly kind: T3rnPrimitivesContractMetadataContractType;
  }

  /** @name PalletContractsWasmOwnerInfo (537) */
  interface PalletContractsWasmOwnerInfo extends Struct {
    readonly owner: AccountId32;
    readonly deposit: Compact<u128>;
    readonly refcount: Compact<u64>;
  }

  /** @name PalletContractsStorageRawContractInfo (538) */
  interface PalletContractsStorageRawContractInfo extends Struct {
    readonly trieId: Bytes;
    readonly codeHash: H256;
    readonly storageDeposit: u128;
  }

  /** @name PalletContractsStorageDeletedContract (540) */
  interface PalletContractsStorageDeletedContract extends Struct {
    readonly trieId: Bytes;
  }

  /** @name PalletContractsSchedule (541) */
  interface PalletContractsSchedule extends Struct {
    readonly limits: PalletContractsScheduleLimits;
    readonly instructionWeights: PalletContractsScheduleInstructionWeights;
    readonly hostFnWeights: PalletContractsScheduleHostFnWeights;
  }

  /** @name PalletContractsScheduleLimits (542) */
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

  /** @name PalletContractsScheduleInstructionWeights (543) */
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

  /** @name PalletContractsScheduleHostFnWeights (544) */
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

  /** @name PalletContractsError (545) */
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
    readonly type: 'InvalidScheduleVersion' | 'InvalidCallFlags' | 'OutOfGas' | 'OutputBufferTooSmall' | 'TransferFailed' | 'MaxCallDepthReached' | 'ContractNotFound' | 'CodeTooLarge' | 'CodeNotFound' | 'OutOfBounds' | 'DecodingFailed' | 'ContractTrapped' | 'ValueTooLarge' | 'TerminatedWhileReentrant' | 'InputForwarded' | 'RandomSubjectTooLong' | 'TooManyTopics' | 'DuplicateTopics' | 'NoChainExtension' | 'DeletionQueueFull' | 'DuplicateContract' | 'TerminatedInConstructor' | 'DebugMessageInvalidUTF8' | 'ReentranceDenied' | 'StorageDepositNotEnoughFunds' | 'StorageDepositLimitExhausted' | 'CodeInUse' | 'ContractReverted' | 'CodeRejected' | 'NoStateReturned';
  }

  /** @name PalletEvmThreeVmInfo (547) */
  interface PalletEvmThreeVmInfo extends Struct {
    readonly author: T3rnPrimitivesContractsRegistryAuthorInfo;
    readonly kind: T3rnPrimitivesContractMetadataContractType;
  }

  /** @name PalletEvmError (548) */
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
    readonly type: 'BalanceLow' | 'FeeOverflow' | 'PaymentOverflow' | 'WithdrawFailed' | 'GasPriceTooLow' | 'InvalidNonce' | 'InvalidRegistryHash' | 'RemunerateAuthor' | 'ExecutedFailed' | 'CreatedFailed';
  }

  /** @name PalletPortalError (549) */
  interface PalletPortalError extends Enum {
    readonly isXdnsRecordCreationFailed: boolean;
    readonly isUnimplementedGatewayVendor: boolean;
    readonly isLightClientNotFoundByVendor: boolean;
    readonly isRegistrationError: boolean;
    readonly isGatewayVendorNotFound: boolean;
    readonly isSetOwnerError: boolean;
    readonly isSetOperationalError: boolean;
    readonly isSubmitHeaderError: boolean;
    readonly isNoGatewayHeightAvailable: boolean;
    readonly isSideEffectConfirmationFailed: boolean;
    readonly isSfxRecodeError: boolean;
    readonly type: 'XdnsRecordCreationFailed' | 'UnimplementedGatewayVendor' | 'LightClientNotFoundByVendor' | 'RegistrationError' | 'GatewayVendorNotFound' | 'SetOwnerError' | 'SetOperationalError' | 'SubmitHeaderError' | 'NoGatewayHeightAvailable' | 'SideEffectConfirmationFailed' | 'SfxRecodeError';
  }

  /** @name PalletGrandpaFinalityVerifierBridgesHeaderChainAuthoritySet (550) */
  interface PalletGrandpaFinalityVerifierBridgesHeaderChainAuthoritySet extends Struct {
    readonly authorities: Vec<ITuple<[SpFinalityGrandpaAppPublic, u64]>>;
    readonly setId: u64;
  }

  /** @name PalletGrandpaFinalityVerifierParachainRegistrationData (553) */
  interface PalletGrandpaFinalityVerifierParachainRegistrationData extends Struct {
    readonly relayGatewayId: U8aFixed;
    readonly id: u32;
  }

  /** @name PalletGrandpaFinalityVerifierError (554) */
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
    readonly type: 'EmptyRangeSubmitted' | 'RangeToLarge' | 'NoFinalizedHeader' | 'InvalidAuthoritySet' | 'InvalidGrandpaJustification' | 'InvalidRangeLinkage' | 'InvalidJustificationLinkage' | 'ParachainEntryNotFound' | 'StorageRootNotFound' | 'InclusionDataDecodeError' | 'InvalidStorageProof' | 'EventNotIncluded' | 'HeaderDecodingError' | 'HeaderDataDecodingError' | 'StorageRootMismatch' | 'UnknownHeader' | 'EventDecodingFailed' | 'UnkownSideEffect' | 'UnsupportedScheduledChange' | 'Halted' | 'BlockHeightConversionError';
  }

  /** @name PalletEth2FinalityVerifierCheckpoint (557) */
  interface PalletEth2FinalityVerifierCheckpoint extends Struct {
    readonly attestedBeacon: PalletEth2FinalityVerifierBeaconCheckpoint;
    readonly attestedExecution: PalletEth2FinalityVerifierExecutionCheckpoint;
    readonly justifiedBeacon: PalletEth2FinalityVerifierBeaconCheckpoint;
    readonly justifiedExecution: PalletEth2FinalityVerifierExecutionCheckpoint;
    readonly finalizedBeacon: PalletEth2FinalityVerifierBeaconCheckpoint;
    readonly finalizedExecution: PalletEth2FinalityVerifierExecutionCheckpoint;
  }

  /** @name PalletEth2FinalityVerifierBeaconCheckpoint (558) */
  interface PalletEth2FinalityVerifierBeaconCheckpoint extends Struct {
    readonly epoch: u64;
    readonly root: U8aFixed;
  }

  /** @name PalletEth2FinalityVerifierExecutionCheckpoint (559) */
  interface PalletEth2FinalityVerifierExecutionCheckpoint extends Struct {
    readonly height: u64;
    readonly root: U8aFixed;
  }

  /** @name PalletEth2FinalityVerifierError (560) */
  interface PalletEth2FinalityVerifierError extends Enum {
    readonly isHalted: boolean;
    readonly isAlreadyInitialized: boolean;
    readonly isInvalidInitializationData: boolean;
    readonly isSszForkDataHashTreeRootFailed: boolean;
    readonly isSszSigningDataHashTreeRootFailed: boolean;
    readonly isBlsPubkeyAggregationFaild: boolean;
    readonly isInvalidBLSPublicKeyUsedForVerification: boolean;
    readonly isInvalidInclusionProof: boolean;
    readonly isForkNotDetected: boolean;
    readonly isValidSyncCommitteeNotAvailable: boolean;
    readonly isSubmittedHeaderToOld: boolean;
    readonly isInvalidBLSSignature: boolean;
    readonly isInvalidMerkleProof: boolean;
    readonly isBeaconHeaderHashTreeRootFailed: boolean;
    readonly isBeaconHeaderNotFound: boolean;
    readonly isBeaconHeaderNotFinalized: boolean;
    readonly isExecutionHeaderHashTreeRootFailed: boolean;
    readonly isInvalidExecutionRangeLinkage: boolean;
    readonly isInvalidExecutionRange: boolean;
    readonly isSyncCommitteeParticipantsNotSupermajority: boolean;
    readonly isSyncCommitteeInvalid: boolean;
    readonly isNotPeriodsFirstEpoch: boolean;
    readonly isInvalidCheckpoint: boolean;
    readonly isExecutionHeaderNotFound: boolean;
    readonly isEventNotInReceipt: boolean;
    readonly isInvalidEncodedEpochUpdate: boolean;
    readonly isInvalidSyncCommitteePeriod: boolean;
    readonly isMathError: boolean;
    readonly isCurrentSyncCommitteePeriodNotAvailable: boolean;
    readonly isBeaconCheckpointHashTreeRootFailed: boolean;
    readonly isInvalidFork: boolean;
    readonly type: 'Halted' | 'AlreadyInitialized' | 'InvalidInitializationData' | 'SszForkDataHashTreeRootFailed' | 'SszSigningDataHashTreeRootFailed' | 'BlsPubkeyAggregationFaild' | 'InvalidBLSPublicKeyUsedForVerification' | 'InvalidInclusionProof' | 'ForkNotDetected' | 'ValidSyncCommitteeNotAvailable' | 'SubmittedHeaderToOld' | 'InvalidBLSSignature' | 'InvalidMerkleProof' | 'BeaconHeaderHashTreeRootFailed' | 'BeaconHeaderNotFound' | 'BeaconHeaderNotFinalized' | 'ExecutionHeaderHashTreeRootFailed' | 'InvalidExecutionRangeLinkage' | 'InvalidExecutionRange' | 'SyncCommitteeParticipantsNotSupermajority' | 'SyncCommitteeInvalid' | 'NotPeriodsFirstEpoch' | 'InvalidCheckpoint' | 'ExecutionHeaderNotFound' | 'EventNotInReceipt' | 'InvalidEncodedEpochUpdate' | 'InvalidSyncCommitteePeriod' | 'MathError' | 'CurrentSyncCommitteePeriodNotAvailable' | 'BeaconCheckpointHashTreeRootFailed' | 'InvalidFork';
  }

  /** @name PalletMaintenanceModeError (561) */
  interface PalletMaintenanceModeError extends Enum {
    readonly isAlreadyInMaintenanceMode: boolean;
    readonly isNotInMaintenanceMode: boolean;
    readonly type: 'AlreadyInMaintenanceMode' | 'NotInMaintenanceMode';
  }

  /** @name PalletSudoError (562) */
  interface PalletSudoError extends Enum {
    readonly isRequireSudo: boolean;
    readonly type: 'RequireSudo';
  }

  /** @name SpRuntimeMultiSignature (564) */
  interface SpRuntimeMultiSignature extends Enum {
    readonly isEd25519: boolean;
    readonly asEd25519: SpCoreEd25519Signature;
    readonly isSr25519: boolean;
    readonly asSr25519: SpCoreSr25519Signature;
    readonly isEcdsa: boolean;
    readonly asEcdsa: SpCoreEcdsaSignature;
    readonly type: 'Ed25519' | 'Sr25519' | 'Ecdsa';
  }

  /** @name SpCoreSr25519Signature (565) */
  interface SpCoreSr25519Signature extends U8aFixed {}

  /** @name SpCoreEcdsaSignature (566) */
  interface SpCoreEcdsaSignature extends U8aFixed {}

  /** @name FrameSystemExtensionsCheckNonZeroSender (568) */
  type FrameSystemExtensionsCheckNonZeroSender = Null;

  /** @name FrameSystemExtensionsCheckSpecVersion (569) */
  type FrameSystemExtensionsCheckSpecVersion = Null;

  /** @name FrameSystemExtensionsCheckTxVersion (570) */
  type FrameSystemExtensionsCheckTxVersion = Null;

  /** @name FrameSystemExtensionsCheckGenesis (571) */
  type FrameSystemExtensionsCheckGenesis = Null;

  /** @name FrameSystemExtensionsCheckNonce (574) */
  interface FrameSystemExtensionsCheckNonce extends Compact<u32> {}

  /** @name FrameSystemExtensionsCheckWeight (575) */
  type FrameSystemExtensionsCheckWeight = Null;

  /** @name PalletAssetTxPaymentChargeAssetTxPayment (576) */
  interface PalletAssetTxPaymentChargeAssetTxPayment extends Struct {
    readonly tip: Compact<u128>;
    readonly assetId: Option<u32>;
  }

  /** @name T0rnParachainRuntimeRuntime (577) */
  type T0rnParachainRuntimeRuntime = Null;

} // declare module
