// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/types/lookup';

import type { Data } from '@polkadot/types';
import type { BTreeMap, BTreeSet, Bytes, Compact, Enum, Null, Option, Result, Set, Struct, Text, U256, U8aFixed, Vec, bool, u128, u16, u32, u64, u8 } from '@polkadot/types-codec';
import type { ITuple } from '@polkadot/types-codec/types';
import type { AccountId32, Call, H160, H256, MultiAddress, Percent } from '@polkadot/types/interfaces/runtime';
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
    readonly frozen: u128;
    readonly flags: u128;
  }

  /** @name FrameSupportDispatchPerDispatchClassWeight (8) */
  interface FrameSupportDispatchPerDispatchClassWeight extends Struct {
    readonly normal: SpWeightsWeightV2Weight;
    readonly operational: SpWeightsWeightV2Weight;
    readonly mandatory: SpWeightsWeightV2Weight;
  }

  /** @name SpWeightsWeightV2Weight (9) */
  interface SpWeightsWeightV2Weight extends Struct {
    readonly refTime: Compact<u64>;
    readonly proofSize: Compact<u64>;
  }

  /** @name SpRuntimeDigest (14) */
  interface SpRuntimeDigest extends Struct {
    readonly logs: Vec<SpRuntimeDigestDigestItem>;
  }

  /** @name SpRuntimeDigestDigestItem (16) */
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

  /** @name FrameSystemEventRecord (19) */
  interface FrameSystemEventRecord extends Struct {
    readonly phase: FrameSystemPhase;
    readonly event: Event;
    readonly topics: Vec<H256>;
  }

  /** @name FrameSystemEvent (21) */
  interface FrameSystemEvent extends Enum {
    readonly isExtrinsicSuccess: boolean;
    readonly asExtrinsicSuccess: {
      readonly dispatchInfo: FrameSupportDispatchDispatchInfo;
    } & Struct;
    readonly isExtrinsicFailed: boolean;
    readonly asExtrinsicFailed: {
      readonly dispatchError: SpRuntimeDispatchError;
      readonly dispatchInfo: FrameSupportDispatchDispatchInfo;
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

  /** @name FrameSupportDispatchDispatchInfo (22) */
  interface FrameSupportDispatchDispatchInfo extends Struct {
    readonly weight: SpWeightsWeightV2Weight;
    readonly class: FrameSupportDispatchDispatchClass;
    readonly paysFee: FrameSupportDispatchPays;
  }

  /** @name FrameSupportDispatchDispatchClass (23) */
  interface FrameSupportDispatchDispatchClass extends Enum {
    readonly isNormal: boolean;
    readonly isOperational: boolean;
    readonly isMandatory: boolean;
    readonly type: 'Normal' | 'Operational' | 'Mandatory';
  }

  /** @name FrameSupportDispatchPays (24) */
  interface FrameSupportDispatchPays extends Enum {
    readonly isYes: boolean;
    readonly isNo: boolean;
    readonly type: 'Yes' | 'No';
  }

  /** @name SpRuntimeDispatchError (25) */
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
    readonly asArithmetic: SpArithmeticArithmeticError;
    readonly isTransactional: boolean;
    readonly asTransactional: SpRuntimeTransactionalError;
    readonly isExhausted: boolean;
    readonly isCorruption: boolean;
    readonly isUnavailable: boolean;
    readonly isRootNotAllowed: boolean;
    readonly type: 'Other' | 'CannotLookup' | 'BadOrigin' | 'Module' | 'ConsumerRemaining' | 'NoProviders' | 'TooManyConsumers' | 'Token' | 'Arithmetic' | 'Transactional' | 'Exhausted' | 'Corruption' | 'Unavailable' | 'RootNotAllowed';
  }

  /** @name SpRuntimeModuleError (26) */
  interface SpRuntimeModuleError extends Struct {
    readonly index: u8;
    readonly error: U8aFixed;
  }

  /** @name SpRuntimeTokenError (27) */
  interface SpRuntimeTokenError extends Enum {
    readonly isFundsUnavailable: boolean;
    readonly isOnlyProvider: boolean;
    readonly isBelowMinimum: boolean;
    readonly isCannotCreate: boolean;
    readonly isUnknownAsset: boolean;
    readonly isFrozen: boolean;
    readonly isUnsupported: boolean;
    readonly isCannotCreateHold: boolean;
    readonly isNotExpendable: boolean;
    readonly isBlocked: boolean;
    readonly type: 'FundsUnavailable' | 'OnlyProvider' | 'BelowMinimum' | 'CannotCreate' | 'UnknownAsset' | 'Frozen' | 'Unsupported' | 'CannotCreateHold' | 'NotExpendable' | 'Blocked';
  }

  /** @name SpArithmeticArithmeticError (28) */
  interface SpArithmeticArithmeticError extends Enum {
    readonly isUnderflow: boolean;
    readonly isOverflow: boolean;
    readonly isDivisionByZero: boolean;
    readonly type: 'Underflow' | 'Overflow' | 'DivisionByZero';
  }

  /** @name SpRuntimeTransactionalError (29) */
  interface SpRuntimeTransactionalError extends Enum {
    readonly isLimitReached: boolean;
    readonly isNoLayer: boolean;
    readonly type: 'LimitReached' | 'NoLayer';
  }

  /** @name CumulusPalletParachainSystemEvent (30) */
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
      readonly weightUsed: SpWeightsWeightV2Weight;
      readonly dmqHead: H256;
    } & Struct;
    readonly isUpwardMessageSent: boolean;
    readonly asUpwardMessageSent: {
      readonly messageHash: Option<U8aFixed>;
    } & Struct;
    readonly type: 'ValidationFunctionStored' | 'ValidationFunctionApplied' | 'ValidationFunctionDiscarded' | 'UpgradeAuthorized' | 'DownwardMessagesReceived' | 'DownwardMessagesProcessed' | 'UpwardMessageSent';
  }

  /** @name PalletPreimageEvent (32) */
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

  /** @name PalletSchedulerEvent (33) */
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
      readonly id: Option<U8aFixed>;
      readonly result: Result<Null, SpRuntimeDispatchError>;
    } & Struct;
    readonly isCallUnavailable: boolean;
    readonly asCallUnavailable: {
      readonly task: ITuple<[u32, u32]>;
      readonly id: Option<U8aFixed>;
    } & Struct;
    readonly isPeriodicFailed: boolean;
    readonly asPeriodicFailed: {
      readonly task: ITuple<[u32, u32]>;
      readonly id: Option<U8aFixed>;
    } & Struct;
    readonly isPermanentlyOverweight: boolean;
    readonly asPermanentlyOverweight: {
      readonly task: ITuple<[u32, u32]>;
      readonly id: Option<U8aFixed>;
    } & Struct;
    readonly type: 'Scheduled' | 'Canceled' | 'Dispatched' | 'CallUnavailable' | 'PeriodicFailed' | 'PermanentlyOverweight';
  }

  /** @name PalletUtilityEvent (37) */
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

  /** @name PalletIdentityEvent (38) */
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

  /** @name PalletBalancesEvent (39) */
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
    readonly isMinted: boolean;
    readonly asMinted: {
      readonly who: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isBurned: boolean;
    readonly asBurned: {
      readonly who: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isSuspended: boolean;
    readonly asSuspended: {
      readonly who: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isRestored: boolean;
    readonly asRestored: {
      readonly who: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isUpgraded: boolean;
    readonly asUpgraded: {
      readonly who: AccountId32;
    } & Struct;
    readonly isIssued: boolean;
    readonly asIssued: {
      readonly amount: u128;
    } & Struct;
    readonly isRescinded: boolean;
    readonly asRescinded: {
      readonly amount: u128;
    } & Struct;
    readonly isLocked: boolean;
    readonly asLocked: {
      readonly who: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isUnlocked: boolean;
    readonly asUnlocked: {
      readonly who: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isFrozen: boolean;
    readonly asFrozen: {
      readonly who: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isThawed: boolean;
    readonly asThawed: {
      readonly who: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly type: 'Endowed' | 'DustLost' | 'Transfer' | 'BalanceSet' | 'Reserved' | 'Unreserved' | 'ReserveRepatriated' | 'Deposit' | 'Withdraw' | 'Slashed' | 'Minted' | 'Burned' | 'Suspended' | 'Restored' | 'Upgraded' | 'Issued' | 'Rescinded' | 'Locked' | 'Unlocked' | 'Frozen' | 'Thawed';
  }

  /** @name FrameSupportTokensMiscBalanceStatus (40) */
  interface FrameSupportTokensMiscBalanceStatus extends Enum {
    readonly isFree: boolean;
    readonly isReserved: boolean;
    readonly type: 'Free' | 'Reserved';
  }

  /** @name PalletTransactionPaymentEvent (41) */
  interface PalletTransactionPaymentEvent extends Enum {
    readonly isTransactionFeePaid: boolean;
    readonly asTransactionFeePaid: {
      readonly who: AccountId32;
      readonly actualFee: u128;
      readonly tip: u128;
    } & Struct;
    readonly type: 'TransactionFeePaid';
  }

  /** @name PalletAssetsEvent (42) */
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
      readonly amount: u128;
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
    readonly isAccountsDestroyed: boolean;
    readonly asAccountsDestroyed: {
      readonly assetId: u32;
      readonly accountsDestroyed: u32;
      readonly accountsRemaining: u32;
    } & Struct;
    readonly isApprovalsDestroyed: boolean;
    readonly asApprovalsDestroyed: {
      readonly assetId: u32;
      readonly approvalsDestroyed: u32;
      readonly approvalsRemaining: u32;
    } & Struct;
    readonly isDestructionStarted: boolean;
    readonly asDestructionStarted: {
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
    readonly isAssetMinBalanceChanged: boolean;
    readonly asAssetMinBalanceChanged: {
      readonly assetId: u32;
      readonly newMinBalance: u128;
    } & Struct;
    readonly isTouched: boolean;
    readonly asTouched: {
      readonly assetId: u32;
      readonly who: AccountId32;
      readonly depositor: AccountId32;
    } & Struct;
    readonly isBlocked: boolean;
    readonly asBlocked: {
      readonly assetId: u32;
      readonly who: AccountId32;
    } & Struct;
    readonly type: 'Created' | 'Issued' | 'Transferred' | 'Burned' | 'TeamChanged' | 'OwnerChanged' | 'Frozen' | 'Thawed' | 'AssetFrozen' | 'AssetThawed' | 'AccountsDestroyed' | 'ApprovalsDestroyed' | 'DestructionStarted' | 'Destroyed' | 'ForceCreated' | 'MetadataSet' | 'MetadataCleared' | 'ApprovedTransfer' | 'ApprovalCancelled' | 'TransferredApproved' | 'AssetStatusChanged' | 'AssetMinBalanceChanged' | 'Touched' | 'Blocked';
  }

  /** @name PalletAccountManagerEvent (44) */
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

  /** @name PalletAssetTxPaymentEvent (46) */
  interface PalletAssetTxPaymentEvent extends Enum {
    readonly isAssetTxFeePaid: boolean;
    readonly asAssetTxFeePaid: {
      readonly who: AccountId32;
      readonly actualFee: u128;
      readonly tip: u128;
      readonly assetId: Option<u32>;
    } & Struct;
    readonly type: 'AssetTxFeePaid';
  }

  /** @name PalletTreasuryEvent (48) */
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
    readonly isUpdatedInactive: boolean;
    readonly asUpdatedInactive: {
      readonly reactivated: u128;
      readonly deactivated: u128;
    } & Struct;
    readonly type: 'Proposed' | 'Spending' | 'Awarded' | 'Rejected' | 'Burnt' | 'Rollover' | 'Deposit' | 'SpendApproved' | 'UpdatedInactive';
  }

  /** @name PalletCollatorSelectionEvent (53) */
  interface PalletCollatorSelectionEvent extends Enum {
    readonly isNewInvulnerables: boolean;
    readonly asNewInvulnerables: {
      readonly invulnerables: Vec<AccountId32>;
    } & Struct;
    readonly isInvulnerableAdded: boolean;
    readonly asInvulnerableAdded: {
      readonly accountId: AccountId32;
    } & Struct;
    readonly isInvulnerableRemoved: boolean;
    readonly asInvulnerableRemoved: {
      readonly accountId: AccountId32;
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
    readonly isInvalidInvulnerableSkipped: boolean;
    readonly asInvalidInvulnerableSkipped: {
      readonly accountId: AccountId32;
    } & Struct;
    readonly type: 'NewInvulnerables' | 'InvulnerableAdded' | 'InvulnerableRemoved' | 'NewDesiredCandidates' | 'NewCandidacyBond' | 'CandidateAdded' | 'CandidateRemoved' | 'InvalidInvulnerableSkipped';
  }

  /** @name PalletSessionEvent (55) */
  interface PalletSessionEvent extends Enum {
    readonly isNewSession: boolean;
    readonly asNewSession: {
      readonly sessionIndex: u32;
    } & Struct;
    readonly type: 'NewSession';
  }

  /** @name CumulusPalletXcmpQueueEvent (56) */
  interface CumulusPalletXcmpQueueEvent extends Enum {
    readonly isSuccess: boolean;
    readonly asSuccess: {
      readonly messageHash: U8aFixed;
      readonly messageId: U8aFixed;
      readonly weight: SpWeightsWeightV2Weight;
    } & Struct;
    readonly isFail: boolean;
    readonly asFail: {
      readonly messageHash: U8aFixed;
      readonly messageId: U8aFixed;
      readonly error: XcmV3TraitsError;
      readonly weight: SpWeightsWeightV2Weight;
    } & Struct;
    readonly isBadVersion: boolean;
    readonly asBadVersion: {
      readonly messageHash: U8aFixed;
    } & Struct;
    readonly isBadFormat: boolean;
    readonly asBadFormat: {
      readonly messageHash: U8aFixed;
    } & Struct;
    readonly isXcmpMessageSent: boolean;
    readonly asXcmpMessageSent: {
      readonly messageHash: U8aFixed;
    } & Struct;
    readonly isOverweightEnqueued: boolean;
    readonly asOverweightEnqueued: {
      readonly sender: u32;
      readonly sentAt: u32;
      readonly index: u64;
      readonly required: SpWeightsWeightV2Weight;
    } & Struct;
    readonly isOverweightServiced: boolean;
    readonly asOverweightServiced: {
      readonly index: u64;
      readonly used: SpWeightsWeightV2Weight;
    } & Struct;
    readonly type: 'Success' | 'Fail' | 'BadVersion' | 'BadFormat' | 'XcmpMessageSent' | 'OverweightEnqueued' | 'OverweightServiced';
  }

  /** @name XcmV3TraitsError (57) */
  interface XcmV3TraitsError extends Enum {
    readonly isOverflow: boolean;
    readonly isUnimplemented: boolean;
    readonly isUntrustedReserveLocation: boolean;
    readonly isUntrustedTeleportLocation: boolean;
    readonly isLocationFull: boolean;
    readonly isLocationNotInvertible: boolean;
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
    readonly isExpectationFalse: boolean;
    readonly isPalletNotFound: boolean;
    readonly isNameMismatch: boolean;
    readonly isVersionIncompatible: boolean;
    readonly isHoldingWouldOverflow: boolean;
    readonly isExportError: boolean;
    readonly isReanchorFailed: boolean;
    readonly isNoDeal: boolean;
    readonly isFeesNotMet: boolean;
    readonly isLockError: boolean;
    readonly isNoPermission: boolean;
    readonly isUnanchored: boolean;
    readonly isNotDepositable: boolean;
    readonly isUnhandledXcmVersion: boolean;
    readonly isWeightLimitReached: boolean;
    readonly asWeightLimitReached: SpWeightsWeightV2Weight;
    readonly isBarrier: boolean;
    readonly isWeightNotComputable: boolean;
    readonly isExceedsStackLimit: boolean;
    readonly type: 'Overflow' | 'Unimplemented' | 'UntrustedReserveLocation' | 'UntrustedTeleportLocation' | 'LocationFull' | 'LocationNotInvertible' | 'BadOrigin' | 'InvalidLocation' | 'AssetNotFound' | 'FailedToTransactAsset' | 'NotWithdrawable' | 'LocationCannotHold' | 'ExceedsMaxMessageSize' | 'DestinationUnsupported' | 'Transport' | 'Unroutable' | 'UnknownClaim' | 'FailedToDecode' | 'MaxWeightInvalid' | 'NotHoldingFees' | 'TooExpensive' | 'Trap' | 'ExpectationFalse' | 'PalletNotFound' | 'NameMismatch' | 'VersionIncompatible' | 'HoldingWouldOverflow' | 'ExportError' | 'ReanchorFailed' | 'NoDeal' | 'FeesNotMet' | 'LockError' | 'NoPermission' | 'Unanchored' | 'NotDepositable' | 'UnhandledXcmVersion' | 'WeightLimitReached' | 'Barrier' | 'WeightNotComputable' | 'ExceedsStackLimit';
  }

  /** @name PalletXcmEvent (59) */
  interface PalletXcmEvent extends Enum {
    readonly isAttempted: boolean;
    readonly asAttempted: {
      readonly outcome: XcmV3TraitsOutcome;
    } & Struct;
    readonly isSent: boolean;
    readonly asSent: {
      readonly origin: XcmV3MultiLocation;
      readonly destination: XcmV3MultiLocation;
      readonly message: XcmV3Xcm;
      readonly messageId: U8aFixed;
    } & Struct;
    readonly isUnexpectedResponse: boolean;
    readonly asUnexpectedResponse: {
      readonly origin: XcmV3MultiLocation;
      readonly queryId: u64;
    } & Struct;
    readonly isResponseReady: boolean;
    readonly asResponseReady: {
      readonly queryId: u64;
      readonly response: XcmV3Response;
    } & Struct;
    readonly isNotified: boolean;
    readonly asNotified: {
      readonly queryId: u64;
      readonly palletIndex: u8;
      readonly callIndex: u8;
    } & Struct;
    readonly isNotifyOverweight: boolean;
    readonly asNotifyOverweight: {
      readonly queryId: u64;
      readonly palletIndex: u8;
      readonly callIndex: u8;
      readonly actualWeight: SpWeightsWeightV2Weight;
      readonly maxBudgetedWeight: SpWeightsWeightV2Weight;
    } & Struct;
    readonly isNotifyDispatchError: boolean;
    readonly asNotifyDispatchError: {
      readonly queryId: u64;
      readonly palletIndex: u8;
      readonly callIndex: u8;
    } & Struct;
    readonly isNotifyDecodeFailed: boolean;
    readonly asNotifyDecodeFailed: {
      readonly queryId: u64;
      readonly palletIndex: u8;
      readonly callIndex: u8;
    } & Struct;
    readonly isInvalidResponder: boolean;
    readonly asInvalidResponder: {
      readonly origin: XcmV3MultiLocation;
      readonly queryId: u64;
      readonly expectedLocation: Option<XcmV3MultiLocation>;
    } & Struct;
    readonly isInvalidResponderVersion: boolean;
    readonly asInvalidResponderVersion: {
      readonly origin: XcmV3MultiLocation;
      readonly queryId: u64;
    } & Struct;
    readonly isResponseTaken: boolean;
    readonly asResponseTaken: {
      readonly queryId: u64;
    } & Struct;
    readonly isAssetsTrapped: boolean;
    readonly asAssetsTrapped: {
      readonly hash_: H256;
      readonly origin: XcmV3MultiLocation;
      readonly assets: XcmVersionedMultiAssets;
    } & Struct;
    readonly isVersionChangeNotified: boolean;
    readonly asVersionChangeNotified: {
      readonly destination: XcmV3MultiLocation;
      readonly result: u32;
      readonly cost: XcmV3MultiassetMultiAssets;
      readonly messageId: U8aFixed;
    } & Struct;
    readonly isSupportedVersionChanged: boolean;
    readonly asSupportedVersionChanged: {
      readonly location: XcmV3MultiLocation;
      readonly version: u32;
    } & Struct;
    readonly isNotifyTargetSendFail: boolean;
    readonly asNotifyTargetSendFail: {
      readonly location: XcmV3MultiLocation;
      readonly queryId: u64;
      readonly error: XcmV3TraitsError;
    } & Struct;
    readonly isNotifyTargetMigrationFail: boolean;
    readonly asNotifyTargetMigrationFail: {
      readonly location: XcmVersionedMultiLocation;
      readonly queryId: u64;
    } & Struct;
    readonly isInvalidQuerierVersion: boolean;
    readonly asInvalidQuerierVersion: {
      readonly origin: XcmV3MultiLocation;
      readonly queryId: u64;
    } & Struct;
    readonly isInvalidQuerier: boolean;
    readonly asInvalidQuerier: {
      readonly origin: XcmV3MultiLocation;
      readonly queryId: u64;
      readonly expectedQuerier: XcmV3MultiLocation;
      readonly maybeActualQuerier: Option<XcmV3MultiLocation>;
    } & Struct;
    readonly isVersionNotifyStarted: boolean;
    readonly asVersionNotifyStarted: {
      readonly destination: XcmV3MultiLocation;
      readonly cost: XcmV3MultiassetMultiAssets;
      readonly messageId: U8aFixed;
    } & Struct;
    readonly isVersionNotifyRequested: boolean;
    readonly asVersionNotifyRequested: {
      readonly destination: XcmV3MultiLocation;
      readonly cost: XcmV3MultiassetMultiAssets;
      readonly messageId: U8aFixed;
    } & Struct;
    readonly isVersionNotifyUnrequested: boolean;
    readonly asVersionNotifyUnrequested: {
      readonly destination: XcmV3MultiLocation;
      readonly cost: XcmV3MultiassetMultiAssets;
      readonly messageId: U8aFixed;
    } & Struct;
    readonly isFeesPaid: boolean;
    readonly asFeesPaid: {
      readonly paying: XcmV3MultiLocation;
      readonly fees: XcmV3MultiassetMultiAssets;
    } & Struct;
    readonly isAssetsClaimed: boolean;
    readonly asAssetsClaimed: {
      readonly hash_: H256;
      readonly origin: XcmV3MultiLocation;
      readonly assets: XcmVersionedMultiAssets;
    } & Struct;
    readonly type: 'Attempted' | 'Sent' | 'UnexpectedResponse' | 'ResponseReady' | 'Notified' | 'NotifyOverweight' | 'NotifyDispatchError' | 'NotifyDecodeFailed' | 'InvalidResponder' | 'InvalidResponderVersion' | 'ResponseTaken' | 'AssetsTrapped' | 'VersionChangeNotified' | 'SupportedVersionChanged' | 'NotifyTargetSendFail' | 'NotifyTargetMigrationFail' | 'InvalidQuerierVersion' | 'InvalidQuerier' | 'VersionNotifyStarted' | 'VersionNotifyRequested' | 'VersionNotifyUnrequested' | 'FeesPaid' | 'AssetsClaimed';
  }

  /** @name XcmV3TraitsOutcome (60) */
  interface XcmV3TraitsOutcome extends Enum {
    readonly isComplete: boolean;
    readonly asComplete: SpWeightsWeightV2Weight;
    readonly isIncomplete: boolean;
    readonly asIncomplete: ITuple<[SpWeightsWeightV2Weight, XcmV3TraitsError]>;
    readonly isError: boolean;
    readonly asError: XcmV3TraitsError;
    readonly type: 'Complete' | 'Incomplete' | 'Error';
  }

  /** @name XcmV3MultiLocation (61) */
  interface XcmV3MultiLocation extends Struct {
    readonly parents: u8;
    readonly interior: XcmV3Junctions;
  }

  /** @name XcmV3Junctions (62) */
  interface XcmV3Junctions extends Enum {
    readonly isHere: boolean;
    readonly isX1: boolean;
    readonly asX1: XcmV3Junction;
    readonly isX2: boolean;
    readonly asX2: ITuple<[XcmV3Junction, XcmV3Junction]>;
    readonly isX3: boolean;
    readonly asX3: ITuple<[XcmV3Junction, XcmV3Junction, XcmV3Junction]>;
    readonly isX4: boolean;
    readonly asX4: ITuple<[XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction]>;
    readonly isX5: boolean;
    readonly asX5: ITuple<[XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction]>;
    readonly isX6: boolean;
    readonly asX6: ITuple<[XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction]>;
    readonly isX7: boolean;
    readonly asX7: ITuple<[XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction]>;
    readonly isX8: boolean;
    readonly asX8: ITuple<[XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction]>;
    readonly type: 'Here' | 'X1' | 'X2' | 'X3' | 'X4' | 'X5' | 'X6' | 'X7' | 'X8';
  }

  /** @name XcmV3Junction (63) */
  interface XcmV3Junction extends Enum {
    readonly isParachain: boolean;
    readonly asParachain: Compact<u32>;
    readonly isAccountId32: boolean;
    readonly asAccountId32: {
      readonly network: Option<XcmV3JunctionNetworkId>;
      readonly id: U8aFixed;
    } & Struct;
    readonly isAccountIndex64: boolean;
    readonly asAccountIndex64: {
      readonly network: Option<XcmV3JunctionNetworkId>;
      readonly index: Compact<u64>;
    } & Struct;
    readonly isAccountKey20: boolean;
    readonly asAccountKey20: {
      readonly network: Option<XcmV3JunctionNetworkId>;
      readonly key: U8aFixed;
    } & Struct;
    readonly isPalletInstance: boolean;
    readonly asPalletInstance: u8;
    readonly isGeneralIndex: boolean;
    readonly asGeneralIndex: Compact<u128>;
    readonly isGeneralKey: boolean;
    readonly asGeneralKey: {
      readonly length: u8;
      readonly data: U8aFixed;
    } & Struct;
    readonly isOnlyChild: boolean;
    readonly isPlurality: boolean;
    readonly asPlurality: {
      readonly id: XcmV3JunctionBodyId;
      readonly part: XcmV3JunctionBodyPart;
    } & Struct;
    readonly isGlobalConsensus: boolean;
    readonly asGlobalConsensus: XcmV3JunctionNetworkId;
    readonly type: 'Parachain' | 'AccountId32' | 'AccountIndex64' | 'AccountKey20' | 'PalletInstance' | 'GeneralIndex' | 'GeneralKey' | 'OnlyChild' | 'Plurality' | 'GlobalConsensus';
  }

  /** @name XcmV3JunctionNetworkId (66) */
  interface XcmV3JunctionNetworkId extends Enum {
    readonly isByGenesis: boolean;
    readonly asByGenesis: U8aFixed;
    readonly isByFork: boolean;
    readonly asByFork: {
      readonly blockNumber: u64;
      readonly blockHash: U8aFixed;
    } & Struct;
    readonly isPolkadot: boolean;
    readonly isKusama: boolean;
    readonly isWestend: boolean;
    readonly isRococo: boolean;
    readonly isWococo: boolean;
    readonly isEthereum: boolean;
    readonly asEthereum: {
      readonly chainId: Compact<u64>;
    } & Struct;
    readonly isBitcoinCore: boolean;
    readonly isBitcoinCash: boolean;
    readonly type: 'ByGenesis' | 'ByFork' | 'Polkadot' | 'Kusama' | 'Westend' | 'Rococo' | 'Wococo' | 'Ethereum' | 'BitcoinCore' | 'BitcoinCash';
  }

  /** @name XcmV3JunctionBodyId (69) */
  interface XcmV3JunctionBodyId extends Enum {
    readonly isUnit: boolean;
    readonly isMoniker: boolean;
    readonly asMoniker: U8aFixed;
    readonly isIndex: boolean;
    readonly asIndex: Compact<u32>;
    readonly isExecutive: boolean;
    readonly isTechnical: boolean;
    readonly isLegislative: boolean;
    readonly isJudicial: boolean;
    readonly isDefense: boolean;
    readonly isAdministration: boolean;
    readonly isTreasury: boolean;
    readonly type: 'Unit' | 'Moniker' | 'Index' | 'Executive' | 'Technical' | 'Legislative' | 'Judicial' | 'Defense' | 'Administration' | 'Treasury';
  }

  /** @name XcmV3JunctionBodyPart (70) */
  interface XcmV3JunctionBodyPart extends Enum {
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

  /** @name XcmV3Xcm (71) */
  interface XcmV3Xcm extends Vec<XcmV3Instruction> {}

  /** @name XcmV3Instruction (73) */
  interface XcmV3Instruction extends Enum {
    readonly isWithdrawAsset: boolean;
    readonly asWithdrawAsset: XcmV3MultiassetMultiAssets;
    readonly isReserveAssetDeposited: boolean;
    readonly asReserveAssetDeposited: XcmV3MultiassetMultiAssets;
    readonly isReceiveTeleportedAsset: boolean;
    readonly asReceiveTeleportedAsset: XcmV3MultiassetMultiAssets;
    readonly isQueryResponse: boolean;
    readonly asQueryResponse: {
      readonly queryId: Compact<u64>;
      readonly response: XcmV3Response;
      readonly maxWeight: SpWeightsWeightV2Weight;
      readonly querier: Option<XcmV3MultiLocation>;
    } & Struct;
    readonly isTransferAsset: boolean;
    readonly asTransferAsset: {
      readonly assets: XcmV3MultiassetMultiAssets;
      readonly beneficiary: XcmV3MultiLocation;
    } & Struct;
    readonly isTransferReserveAsset: boolean;
    readonly asTransferReserveAsset: {
      readonly assets: XcmV3MultiassetMultiAssets;
      readonly dest: XcmV3MultiLocation;
      readonly xcm: XcmV3Xcm;
    } & Struct;
    readonly isTransact: boolean;
    readonly asTransact: {
      readonly originKind: XcmV2OriginKind;
      readonly requireWeightAtMost: SpWeightsWeightV2Weight;
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
    readonly asDescendOrigin: XcmV3Junctions;
    readonly isReportError: boolean;
    readonly asReportError: XcmV3QueryResponseInfo;
    readonly isDepositAsset: boolean;
    readonly asDepositAsset: {
      readonly assets: XcmV3MultiassetMultiAssetFilter;
      readonly beneficiary: XcmV3MultiLocation;
    } & Struct;
    readonly isDepositReserveAsset: boolean;
    readonly asDepositReserveAsset: {
      readonly assets: XcmV3MultiassetMultiAssetFilter;
      readonly dest: XcmV3MultiLocation;
      readonly xcm: XcmV3Xcm;
    } & Struct;
    readonly isExchangeAsset: boolean;
    readonly asExchangeAsset: {
      readonly give: XcmV3MultiassetMultiAssetFilter;
      readonly want: XcmV3MultiassetMultiAssets;
      readonly maximal: bool;
    } & Struct;
    readonly isInitiateReserveWithdraw: boolean;
    readonly asInitiateReserveWithdraw: {
      readonly assets: XcmV3MultiassetMultiAssetFilter;
      readonly reserve: XcmV3MultiLocation;
      readonly xcm: XcmV3Xcm;
    } & Struct;
    readonly isInitiateTeleport: boolean;
    readonly asInitiateTeleport: {
      readonly assets: XcmV3MultiassetMultiAssetFilter;
      readonly dest: XcmV3MultiLocation;
      readonly xcm: XcmV3Xcm;
    } & Struct;
    readonly isReportHolding: boolean;
    readonly asReportHolding: {
      readonly responseInfo: XcmV3QueryResponseInfo;
      readonly assets: XcmV3MultiassetMultiAssetFilter;
    } & Struct;
    readonly isBuyExecution: boolean;
    readonly asBuyExecution: {
      readonly fees: XcmV3MultiAsset;
      readonly weightLimit: XcmV3WeightLimit;
    } & Struct;
    readonly isRefundSurplus: boolean;
    readonly isSetErrorHandler: boolean;
    readonly asSetErrorHandler: XcmV3Xcm;
    readonly isSetAppendix: boolean;
    readonly asSetAppendix: XcmV3Xcm;
    readonly isClearError: boolean;
    readonly isClaimAsset: boolean;
    readonly asClaimAsset: {
      readonly assets: XcmV3MultiassetMultiAssets;
      readonly ticket: XcmV3MultiLocation;
    } & Struct;
    readonly isTrap: boolean;
    readonly asTrap: Compact<u64>;
    readonly isSubscribeVersion: boolean;
    readonly asSubscribeVersion: {
      readonly queryId: Compact<u64>;
      readonly maxResponseWeight: SpWeightsWeightV2Weight;
    } & Struct;
    readonly isUnsubscribeVersion: boolean;
    readonly isBurnAsset: boolean;
    readonly asBurnAsset: XcmV3MultiassetMultiAssets;
    readonly isExpectAsset: boolean;
    readonly asExpectAsset: XcmV3MultiassetMultiAssets;
    readonly isExpectOrigin: boolean;
    readonly asExpectOrigin: Option<XcmV3MultiLocation>;
    readonly isExpectError: boolean;
    readonly asExpectError: Option<ITuple<[u32, XcmV3TraitsError]>>;
    readonly isExpectTransactStatus: boolean;
    readonly asExpectTransactStatus: XcmV3MaybeErrorCode;
    readonly isQueryPallet: boolean;
    readonly asQueryPallet: {
      readonly moduleName: Bytes;
      readonly responseInfo: XcmV3QueryResponseInfo;
    } & Struct;
    readonly isExpectPallet: boolean;
    readonly asExpectPallet: {
      readonly index: Compact<u32>;
      readonly name: Bytes;
      readonly moduleName: Bytes;
      readonly crateMajor: Compact<u32>;
      readonly minCrateMinor: Compact<u32>;
    } & Struct;
    readonly isReportTransactStatus: boolean;
    readonly asReportTransactStatus: XcmV3QueryResponseInfo;
    readonly isClearTransactStatus: boolean;
    readonly isUniversalOrigin: boolean;
    readonly asUniversalOrigin: XcmV3Junction;
    readonly isExportMessage: boolean;
    readonly asExportMessage: {
      readonly network: XcmV3JunctionNetworkId;
      readonly destination: XcmV3Junctions;
      readonly xcm: XcmV3Xcm;
    } & Struct;
    readonly isLockAsset: boolean;
    readonly asLockAsset: {
      readonly asset: XcmV3MultiAsset;
      readonly unlocker: XcmV3MultiLocation;
    } & Struct;
    readonly isUnlockAsset: boolean;
    readonly asUnlockAsset: {
      readonly asset: XcmV3MultiAsset;
      readonly target: XcmV3MultiLocation;
    } & Struct;
    readonly isNoteUnlockable: boolean;
    readonly asNoteUnlockable: {
      readonly asset: XcmV3MultiAsset;
      readonly owner: XcmV3MultiLocation;
    } & Struct;
    readonly isRequestUnlock: boolean;
    readonly asRequestUnlock: {
      readonly asset: XcmV3MultiAsset;
      readonly locker: XcmV3MultiLocation;
    } & Struct;
    readonly isSetFeesMode: boolean;
    readonly asSetFeesMode: {
      readonly jitWithdraw: bool;
    } & Struct;
    readonly isSetTopic: boolean;
    readonly asSetTopic: U8aFixed;
    readonly isClearTopic: boolean;
    readonly isAliasOrigin: boolean;
    readonly asAliasOrigin: XcmV3MultiLocation;
    readonly isUnpaidExecution: boolean;
    readonly asUnpaidExecution: {
      readonly weightLimit: XcmV3WeightLimit;
      readonly checkOrigin: Option<XcmV3MultiLocation>;
    } & Struct;
    readonly type: 'WithdrawAsset' | 'ReserveAssetDeposited' | 'ReceiveTeleportedAsset' | 'QueryResponse' | 'TransferAsset' | 'TransferReserveAsset' | 'Transact' | 'HrmpNewChannelOpenRequest' | 'HrmpChannelAccepted' | 'HrmpChannelClosing' | 'ClearOrigin' | 'DescendOrigin' | 'ReportError' | 'DepositAsset' | 'DepositReserveAsset' | 'ExchangeAsset' | 'InitiateReserveWithdraw' | 'InitiateTeleport' | 'ReportHolding' | 'BuyExecution' | 'RefundSurplus' | 'SetErrorHandler' | 'SetAppendix' | 'ClearError' | 'ClaimAsset' | 'Trap' | 'SubscribeVersion' | 'UnsubscribeVersion' | 'BurnAsset' | 'ExpectAsset' | 'ExpectOrigin' | 'ExpectError' | 'ExpectTransactStatus' | 'QueryPallet' | 'ExpectPallet' | 'ReportTransactStatus' | 'ClearTransactStatus' | 'UniversalOrigin' | 'ExportMessage' | 'LockAsset' | 'UnlockAsset' | 'NoteUnlockable' | 'RequestUnlock' | 'SetFeesMode' | 'SetTopic' | 'ClearTopic' | 'AliasOrigin' | 'UnpaidExecution';
  }

  /** @name XcmV3MultiassetMultiAssets (74) */
  interface XcmV3MultiassetMultiAssets extends Vec<XcmV3MultiAsset> {}

  /** @name XcmV3MultiAsset (76) */
  interface XcmV3MultiAsset extends Struct {
    readonly id: XcmV3MultiassetAssetId;
    readonly fun: XcmV3MultiassetFungibility;
  }

  /** @name XcmV3MultiassetAssetId (77) */
  interface XcmV3MultiassetAssetId extends Enum {
    readonly isConcrete: boolean;
    readonly asConcrete: XcmV3MultiLocation;
    readonly isAbstract: boolean;
    readonly asAbstract: U8aFixed;
    readonly type: 'Concrete' | 'Abstract';
  }

  /** @name XcmV3MultiassetFungibility (78) */
  interface XcmV3MultiassetFungibility extends Enum {
    readonly isFungible: boolean;
    readonly asFungible: Compact<u128>;
    readonly isNonFungible: boolean;
    readonly asNonFungible: XcmV3MultiassetAssetInstance;
    readonly type: 'Fungible' | 'NonFungible';
  }

  /** @name XcmV3MultiassetAssetInstance (79) */
  interface XcmV3MultiassetAssetInstance extends Enum {
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
    readonly type: 'Undefined' | 'Index' | 'Array4' | 'Array8' | 'Array16' | 'Array32';
  }

  /** @name XcmV3Response (82) */
  interface XcmV3Response extends Enum {
    readonly isNull: boolean;
    readonly isAssets: boolean;
    readonly asAssets: XcmV3MultiassetMultiAssets;
    readonly isExecutionResult: boolean;
    readonly asExecutionResult: Option<ITuple<[u32, XcmV3TraitsError]>>;
    readonly isVersion: boolean;
    readonly asVersion: u32;
    readonly isPalletsInfo: boolean;
    readonly asPalletsInfo: Vec<XcmV3PalletInfo>;
    readonly isDispatchResult: boolean;
    readonly asDispatchResult: XcmV3MaybeErrorCode;
    readonly type: 'Null' | 'Assets' | 'ExecutionResult' | 'Version' | 'PalletsInfo' | 'DispatchResult';
  }

  /** @name XcmV3PalletInfo (86) */
  interface XcmV3PalletInfo extends Struct {
    readonly index: Compact<u32>;
    readonly name: Bytes;
    readonly moduleName: Bytes;
    readonly major: Compact<u32>;
    readonly minor: Compact<u32>;
    readonly patch: Compact<u32>;
  }

  /** @name XcmV3MaybeErrorCode (89) */
  interface XcmV3MaybeErrorCode extends Enum {
    readonly isSuccess: boolean;
    readonly isError: boolean;
    readonly asError: Bytes;
    readonly isTruncatedError: boolean;
    readonly asTruncatedError: Bytes;
    readonly type: 'Success' | 'Error' | 'TruncatedError';
  }

  /** @name XcmV2OriginKind (92) */
  interface XcmV2OriginKind extends Enum {
    readonly isNative: boolean;
    readonly isSovereignAccount: boolean;
    readonly isSuperuser: boolean;
    readonly isXcm: boolean;
    readonly type: 'Native' | 'SovereignAccount' | 'Superuser' | 'Xcm';
  }

  /** @name XcmDoubleEncoded (93) */
  interface XcmDoubleEncoded extends Struct {
    readonly encoded: Bytes;
  }

  /** @name XcmV3QueryResponseInfo (94) */
  interface XcmV3QueryResponseInfo extends Struct {
    readonly destination: XcmV3MultiLocation;
    readonly queryId: Compact<u64>;
    readonly maxWeight: SpWeightsWeightV2Weight;
  }

  /** @name XcmV3MultiassetMultiAssetFilter (95) */
  interface XcmV3MultiassetMultiAssetFilter extends Enum {
    readonly isDefinite: boolean;
    readonly asDefinite: XcmV3MultiassetMultiAssets;
    readonly isWild: boolean;
    readonly asWild: XcmV3MultiassetWildMultiAsset;
    readonly type: 'Definite' | 'Wild';
  }

  /** @name XcmV3MultiassetWildMultiAsset (96) */
  interface XcmV3MultiassetWildMultiAsset extends Enum {
    readonly isAll: boolean;
    readonly isAllOf: boolean;
    readonly asAllOf: {
      readonly id: XcmV3MultiassetAssetId;
      readonly fun: XcmV3MultiassetWildFungibility;
    } & Struct;
    readonly isAllCounted: boolean;
    readonly asAllCounted: Compact<u32>;
    readonly isAllOfCounted: boolean;
    readonly asAllOfCounted: {
      readonly id: XcmV3MultiassetAssetId;
      readonly fun: XcmV3MultiassetWildFungibility;
      readonly count: Compact<u32>;
    } & Struct;
    readonly type: 'All' | 'AllOf' | 'AllCounted' | 'AllOfCounted';
  }

  /** @name XcmV3MultiassetWildFungibility (97) */
  interface XcmV3MultiassetWildFungibility extends Enum {
    readonly isFungible: boolean;
    readonly isNonFungible: boolean;
    readonly type: 'Fungible' | 'NonFungible';
  }

  /** @name XcmV3WeightLimit (98) */
  interface XcmV3WeightLimit extends Enum {
    readonly isUnlimited: boolean;
    readonly isLimited: boolean;
    readonly asLimited: SpWeightsWeightV2Weight;
    readonly type: 'Unlimited' | 'Limited';
  }

  /** @name XcmVersionedMultiAssets (99) */
  interface XcmVersionedMultiAssets extends Enum {
    readonly isV2: boolean;
    readonly asV2: XcmV2MultiassetMultiAssets;
    readonly isV3: boolean;
    readonly asV3: XcmV3MultiassetMultiAssets;
    readonly type: 'V2' | 'V3';
  }

  /** @name XcmV2MultiassetMultiAssets (100) */
  interface XcmV2MultiassetMultiAssets extends Vec<XcmV2MultiAsset> {}

  /** @name XcmV2MultiAsset (102) */
  interface XcmV2MultiAsset extends Struct {
    readonly id: XcmV2MultiassetAssetId;
    readonly fun: XcmV2MultiassetFungibility;
  }

  /** @name XcmV2MultiassetAssetId (103) */
  interface XcmV2MultiassetAssetId extends Enum {
    readonly isConcrete: boolean;
    readonly asConcrete: XcmV2MultiLocation;
    readonly isAbstract: boolean;
    readonly asAbstract: Bytes;
    readonly type: 'Concrete' | 'Abstract';
  }

  /** @name XcmV2MultiLocation (104) */
  interface XcmV2MultiLocation extends Struct {
    readonly parents: u8;
    readonly interior: XcmV2MultilocationJunctions;
  }

  /** @name XcmV2MultilocationJunctions (105) */
  interface XcmV2MultilocationJunctions extends Enum {
    readonly isHere: boolean;
    readonly isX1: boolean;
    readonly asX1: XcmV2Junction;
    readonly isX2: boolean;
    readonly asX2: ITuple<[XcmV2Junction, XcmV2Junction]>;
    readonly isX3: boolean;
    readonly asX3: ITuple<[XcmV2Junction, XcmV2Junction, XcmV2Junction]>;
    readonly isX4: boolean;
    readonly asX4: ITuple<[XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction]>;
    readonly isX5: boolean;
    readonly asX5: ITuple<[XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction]>;
    readonly isX6: boolean;
    readonly asX6: ITuple<[XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction]>;
    readonly isX7: boolean;
    readonly asX7: ITuple<[XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction]>;
    readonly isX8: boolean;
    readonly asX8: ITuple<[XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction]>;
    readonly type: 'Here' | 'X1' | 'X2' | 'X3' | 'X4' | 'X5' | 'X6' | 'X7' | 'X8';
  }

  /** @name XcmV2Junction (106) */
  interface XcmV2Junction extends Enum {
    readonly isParachain: boolean;
    readonly asParachain: Compact<u32>;
    readonly isAccountId32: boolean;
    readonly asAccountId32: {
      readonly network: XcmV2NetworkId;
      readonly id: U8aFixed;
    } & Struct;
    readonly isAccountIndex64: boolean;
    readonly asAccountIndex64: {
      readonly network: XcmV2NetworkId;
      readonly index: Compact<u64>;
    } & Struct;
    readonly isAccountKey20: boolean;
    readonly asAccountKey20: {
      readonly network: XcmV2NetworkId;
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
      readonly id: XcmV2BodyId;
      readonly part: XcmV2BodyPart;
    } & Struct;
    readonly type: 'Parachain' | 'AccountId32' | 'AccountIndex64' | 'AccountKey20' | 'PalletInstance' | 'GeneralIndex' | 'GeneralKey' | 'OnlyChild' | 'Plurality';
  }

  /** @name XcmV2NetworkId (107) */
  interface XcmV2NetworkId extends Enum {
    readonly isAny: boolean;
    readonly isNamed: boolean;
    readonly asNamed: Bytes;
    readonly isPolkadot: boolean;
    readonly isKusama: boolean;
    readonly type: 'Any' | 'Named' | 'Polkadot' | 'Kusama';
  }

  /** @name XcmV2BodyId (109) */
  interface XcmV2BodyId extends Enum {
    readonly isUnit: boolean;
    readonly isNamed: boolean;
    readonly asNamed: Bytes;
    readonly isIndex: boolean;
    readonly asIndex: Compact<u32>;
    readonly isExecutive: boolean;
    readonly isTechnical: boolean;
    readonly isLegislative: boolean;
    readonly isJudicial: boolean;
    readonly isDefense: boolean;
    readonly isAdministration: boolean;
    readonly isTreasury: boolean;
    readonly type: 'Unit' | 'Named' | 'Index' | 'Executive' | 'Technical' | 'Legislative' | 'Judicial' | 'Defense' | 'Administration' | 'Treasury';
  }

  /** @name XcmV2BodyPart (110) */
  interface XcmV2BodyPart extends Enum {
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

  /** @name XcmV2MultiassetFungibility (111) */
  interface XcmV2MultiassetFungibility extends Enum {
    readonly isFungible: boolean;
    readonly asFungible: Compact<u128>;
    readonly isNonFungible: boolean;
    readonly asNonFungible: XcmV2MultiassetAssetInstance;
    readonly type: 'Fungible' | 'NonFungible';
  }

  /** @name XcmV2MultiassetAssetInstance (112) */
  interface XcmV2MultiassetAssetInstance extends Enum {
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

  /** @name XcmVersionedMultiLocation (113) */
  interface XcmVersionedMultiLocation extends Enum {
    readonly isV2: boolean;
    readonly asV2: XcmV2MultiLocation;
    readonly isV3: boolean;
    readonly asV3: XcmV3MultiLocation;
    readonly type: 'V2' | 'V3';
  }

  /** @name CumulusPalletXcmEvent (114) */
  interface CumulusPalletXcmEvent extends Enum {
    readonly isInvalidFormat: boolean;
    readonly asInvalidFormat: U8aFixed;
    readonly isUnsupportedVersion: boolean;
    readonly asUnsupportedVersion: U8aFixed;
    readonly isExecutedDownward: boolean;
    readonly asExecutedDownward: ITuple<[U8aFixed, XcmV3TraitsOutcome]>;
    readonly type: 'InvalidFormat' | 'UnsupportedVersion' | 'ExecutedDownward';
  }

  /** @name CumulusPalletDmpQueueEvent (115) */
  interface CumulusPalletDmpQueueEvent extends Enum {
    readonly isInvalidFormat: boolean;
    readonly asInvalidFormat: {
      readonly messageHash: U8aFixed;
    } & Struct;
    readonly isUnsupportedVersion: boolean;
    readonly asUnsupportedVersion: {
      readonly messageHash: U8aFixed;
    } & Struct;
    readonly isExecutedDownward: boolean;
    readonly asExecutedDownward: {
      readonly messageHash: U8aFixed;
      readonly messageId: U8aFixed;
      readonly outcome: XcmV3TraitsOutcome;
    } & Struct;
    readonly isWeightExhausted: boolean;
    readonly asWeightExhausted: {
      readonly messageHash: U8aFixed;
      readonly messageId: U8aFixed;
      readonly remainingWeight: SpWeightsWeightV2Weight;
      readonly requiredWeight: SpWeightsWeightV2Weight;
    } & Struct;
    readonly isOverweightEnqueued: boolean;
    readonly asOverweightEnqueued: {
      readonly messageHash: U8aFixed;
      readonly messageId: U8aFixed;
      readonly overweightIndex: u64;
      readonly requiredWeight: SpWeightsWeightV2Weight;
    } & Struct;
    readonly isOverweightServiced: boolean;
    readonly asOverweightServiced: {
      readonly overweightIndex: u64;
      readonly weightUsed: SpWeightsWeightV2Weight;
    } & Struct;
    readonly isMaxMessagesExhausted: boolean;
    readonly asMaxMessagesExhausted: {
      readonly messageHash: U8aFixed;
    } & Struct;
    readonly type: 'InvalidFormat' | 'UnsupportedVersion' | 'ExecutedDownward' | 'WeightExhausted' | 'OverweightEnqueued' | 'OverweightServiced' | 'MaxMessagesExhausted';
  }

  /** @name PalletAssetRegistryEvent (116) */
  interface PalletAssetRegistryEvent extends Enum {
    readonly isRegistered: boolean;
    readonly asRegistered: {
      readonly assetId: u32;
      readonly location: XcmV3MultiLocation;
    } & Struct;
    readonly isInfo: boolean;
    readonly asInfo: {
      readonly assetId: u32;
      readonly location: XcmV3MultiLocation;
    } & Struct;
    readonly type: 'Registered' | 'Info';
  }

  /** @name PalletXdnsEvent (117) */
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

  /** @name PalletAttestersEvent (118) */
  interface PalletAttestersEvent extends Enum {
    readonly isAttesterRegistered: boolean;
    readonly asAttesterRegistered: AccountId32;
    readonly isAttesterDeregistrationScheduled: boolean;
    readonly asAttesterDeregistrationScheduled: ITuple<[AccountId32, u32]>;
    readonly isAttesterDeregistered: boolean;
    readonly asAttesterDeregistered: AccountId32;
    readonly isAttestationSubmitted: boolean;
    readonly asAttestationSubmitted: AccountId32;
    readonly isBatchingFactorRead: boolean;
    readonly asBatchingFactorRead: Vec<ITuple<[U8aFixed, Option<T3rnPrimitivesAttestersBatchingFactor>]>>;
    readonly isBatchCommitted: boolean;
    readonly asBatchCommitted: ITuple<[U8aFixed, PalletAttestersBatchMessage, Bytes, H256, u128]>;
    readonly isConfirmationRewardCalculated: boolean;
    readonly asConfirmationRewardCalculated: ITuple<[U8aFixed, u32, u128, Percent, Percent]>;
    readonly isCollusionWithPermanentSlashDetected: boolean;
    readonly asCollusionWithPermanentSlashDetected: ITuple<[U8aFixed, H256]>;
    readonly isUserFinalityFeeEstimated: boolean;
    readonly asUserFinalityFeeEstimated: ITuple<[U8aFixed, u128]>;
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
    readonly isAttestationsRemovedFromLateBatches: boolean;
    readonly asAttestationsRemovedFromLateBatches: Vec<u32>;
    readonly isAttestationTargetRemoved: boolean;
    readonly asAttestationTargetRemoved: ITuple<[U8aFixed, Vec<U8aFixed>]>;
    readonly isShufflingCompleted: boolean;
    readonly asShufflingCompleted: ITuple<[Vec<AccountId32>, Vec<AccountId32>, Vec<AccountId32>]>;
    readonly type: 'AttesterRegistered' | 'AttesterDeregistrationScheduled' | 'AttesterDeregistered' | 'AttestationSubmitted' | 'BatchingFactorRead' | 'BatchCommitted' | 'ConfirmationRewardCalculated' | 'CollusionWithPermanentSlashDetected' | 'UserFinalityFeeEstimated' | 'NewAttestationBatch' | 'NewAttestationMessageHash' | 'NewConfirmationBatch' | 'Nominated' | 'NewTargetActivated' | 'NewTargetProposed' | 'AttesterAgreedToNewTarget' | 'CurrentPendingAttestationBatches' | 'AttestationsRemovedFromLateBatches' | 'AttestationTargetRemoved' | 'ShufflingCompleted';
  }

  /** @name T3rnPrimitivesAttestersBatchingFactor (122) */
  interface T3rnPrimitivesAttestersBatchingFactor extends Struct {
    readonly latestConfirmed: u16;
    readonly latestSigned: u16;
    readonly currentNext: u16;
    readonly upToLast10Confirmed: Vec<u16>;
  }

  /** @name PalletAttestersBatchMessage (125) */
  interface PalletAttestersBatchMessage extends Struct {
    readonly availableToCommitAt: u32;
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

  /** @name PalletAttestersBatchStatus (133) */
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

  /** @name T3rnPrimitivesAttestersLatencyStatus (134) */
  interface T3rnPrimitivesAttestersLatencyStatus extends Enum {
    readonly isOnTime: boolean;
    readonly isLate: boolean;
    readonly asLate: ITuple<[u32, u32]>;
    readonly type: 'OnTime' | 'Late';
  }

  /** @name T3rnPrimitivesExecutionVendor (136) */
  interface T3rnPrimitivesExecutionVendor extends Enum {
    readonly isSubstrate: boolean;
    readonly isEvm: boolean;
    readonly type: 'Substrate' | 'Evm';
  }

  /** @name PalletRewardsEvent (141) */
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
    readonly asClaimed: ITuple<[AccountId32, Vec<ITuple<[u128, Option<u32>]>>]>;
    readonly isPendingClaim: boolean;
    readonly asPendingClaim: ITuple<[AccountId32, u128]>;
    readonly type: 'AttesterRewarded' | 'CollatorRewarded' | 'ExecutorRewarded' | 'NewMaxRewardExecutorsKickbackSet' | 'Claimed' | 'PendingClaim';
  }

  /** @name PalletContractsRegistryEvent (144) */
  interface PalletContractsRegistryEvent extends Enum {
    readonly isContractStored: boolean;
    readonly asContractStored: ITuple<[AccountId32, H256]>;
    readonly isContractPurged: boolean;
    readonly asContractPurged: ITuple<[AccountId32, H256]>;
    readonly type: 'ContractStored' | 'ContractPurged';
  }

  /** @name PalletCircuitEvent (145) */
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

  /** @name XpFormatXbiResult (153) */
  interface XpFormatXbiResult extends Struct {
    readonly status: XpFormatStatus;
    readonly output: Bytes;
    readonly witness: Bytes;
  }

  /** @name XpFormatStatus (154) */
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

  /** @name T3rnTypesSfxSideEffect (156) */
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

  /** @name T3rnTypesFsxFullSideEffect (159) */
  interface T3rnTypesFsxFullSideEffect extends Struct {
    readonly input: T3rnTypesSfxSideEffect;
    readonly confirmed: Option<T3rnTypesSfxConfirmedSideEffect>;
    readonly securityLvl: T3rnTypesSfxSecurityLvl;
    readonly submissionTargetHeight: u32;
    readonly bestBid: Option<T3rnTypesBidSfxBid>;
    readonly index: u32;
  }

  /** @name T3rnTypesSfxConfirmedSideEffect (161) */
  interface T3rnTypesSfxConfirmedSideEffect extends Struct {
    readonly err: Option<T3rnTypesSfxConfirmationOutcome>;
    readonly output: Option<Bytes>;
    readonly inclusionData: Bytes;
    readonly executioner: AccountId32;
    readonly receivedAt: u32;
    readonly cost: Option<u128>;
  }

  /** @name T3rnTypesSfxConfirmationOutcome (163) */
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

  /** @name T3rnTypesSfxSecurityLvl (165) */
  interface T3rnTypesSfxSecurityLvl extends Enum {
    readonly isOptimistic: boolean;
    readonly isEscrow: boolean;
    readonly type: 'Optimistic' | 'Escrow';
  }

  /** @name T3rnTypesBidSfxBid (167) */
  interface T3rnTypesBidSfxBid extends Struct {
    readonly amount: u128;
    readonly insurance: u128;
    readonly reservedBond: Option<u128>;
    readonly rewardAssetId: Option<u32>;
    readonly executor: AccountId32;
    readonly requester: AccountId32;
  }

  /** @name PalletClockEvent (168) */
  interface PalletClockEvent extends Enum {
    readonly isNewRound: boolean;
    readonly asNewRound: {
      readonly index: u32;
      readonly head: u32;
      readonly term: u32;
    } & Struct;
    readonly type: 'NewRound';
  }

  /** @name PalletCircuitVacuumEvent (169) */
  interface PalletCircuitVacuumEvent extends Enum {
    readonly isOrderStatusRead: boolean;
    readonly asOrderStatusRead: PalletCircuitVacuumOrderStatusRead;
    readonly type: 'OrderStatusRead';
  }

  /** @name PalletCircuitVacuumOrderStatusRead (170) */
  interface PalletCircuitVacuumOrderStatusRead extends Struct {
    readonly xtxId: H256;
    readonly status: T3rnPrimitivesCircuitTypesCircuitStatus;
    readonly allIncludedSfx: Vec<ITuple<[H256, T3rnPrimitivesCircuitTypesCircuitStatus]>>;
    readonly timeoutsAt: T3rnPrimitivesCircuitTypesAdaptiveTimeout;
  }

  /** @name T3rnPrimitivesCircuitTypesCircuitStatus (171) */
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

  /** @name T3rnPrimitivesCircuitTypesCause (172) */
  interface T3rnPrimitivesCircuitTypesCause extends Enum {
    readonly isTimeout: boolean;
    readonly isIntentionalKill: boolean;
    readonly type: 'Timeout' | 'IntentionalKill';
  }

  /** @name T3rnPrimitivesCircuitTypesAdaptiveTimeout (175) */
  interface T3rnPrimitivesCircuitTypesAdaptiveTimeout extends Struct {
    readonly estimatedHeightHere: u32;
    readonly estimatedHeightThere: u32;
    readonly submitByHeightHere: u32;
    readonly submitByHeightThere: u32;
    readonly emergencyTimeoutHere: u32;
    readonly there: U8aFixed;
    readonly dlq: Option<u32>;
  }

  /** @name Pallet3vmEvent (176) */
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

  /** @name T3rnSdkPrimitivesSignalSignalKind (178) */
  interface T3rnSdkPrimitivesSignalSignalKind extends Enum {
    readonly isComplete: boolean;
    readonly isKill: boolean;
    readonly asKill: T3rnSdkPrimitivesSignalKillReason;
    readonly type: 'Complete' | 'Kill';
  }

  /** @name T3rnSdkPrimitivesSignalKillReason (179) */
  interface T3rnSdkPrimitivesSignalKillReason extends Enum {
    readonly isUnhandled: boolean;
    readonly isCodec: boolean;
    readonly isTimeout: boolean;
    readonly type: 'Unhandled' | 'Codec' | 'Timeout';
  }

  /** @name T3rnPrimitivesContractMetadataContractType (181) */
  interface T3rnPrimitivesContractMetadataContractType extends Enum {
    readonly isSystem: boolean;
    readonly isVanillaEvm: boolean;
    readonly isVanillaWasm: boolean;
    readonly isVolatileEvm: boolean;
    readonly isVolatileWasm: boolean;
    readonly type: 'System' | 'VanillaEvm' | 'VanillaWasm' | 'VolatileEvm' | 'VolatileWasm';
  }

  /** @name PalletContractsEvent (183) */
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
    readonly isCalled: boolean;
    readonly asCalled: {
      readonly caller: PalletContractsOrigin;
      readonly contract: AccountId32;
    } & Struct;
    readonly isDelegateCalled: boolean;
    readonly asDelegateCalled: {
      readonly contract: AccountId32;
      readonly codeHash: H256;
    } & Struct;
    readonly type: 'Instantiated' | 'Terminated' | 'CodeStored' | 'ContractEmitted' | 'CodeRemoved' | 'ContractCodeUpdated' | 'Called' | 'DelegateCalled';
  }

  /** @name PalletContractsOrigin (184) */
  interface PalletContractsOrigin extends Enum {
    readonly isRoot: boolean;
    readonly isSigned: boolean;
    readonly asSigned: AccountId32;
    readonly type: 'Root' | 'Signed';
  }

  /** @name T0rnParachainRuntimeRuntime (185) */
  type T0rnParachainRuntimeRuntime = Null;

  /** @name PalletEvmEvent (186) */
  interface PalletEvmEvent extends Enum {
    readonly isLog: boolean;
    readonly asLog: {
      readonly log: EthereumLog;
    } & Struct;
    readonly isCreated: boolean;
    readonly asCreated: {
      readonly address: H160;
    } & Struct;
    readonly isCreatedFailed: boolean;
    readonly asCreatedFailed: {
      readonly address: H160;
    } & Struct;
    readonly isExecuted: boolean;
    readonly asExecuted: {
      readonly address: H160;
    } & Struct;
    readonly isExecutedFailed: boolean;
    readonly asExecutedFailed: {
      readonly address: H160;
    } & Struct;
    readonly type: 'Log' | 'Created' | 'CreatedFailed' | 'Executed' | 'ExecutedFailed';
  }

  /** @name EthereumLog (187) */
  interface EthereumLog extends Struct {
    readonly address: H160;
    readonly topics: Vec<H256>;
    readonly data: Bytes;
  }

  /** @name PalletPortalEvent (188) */
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

  /** @name T3rnPrimitivesGatewayVendor (189) */
  interface T3rnPrimitivesGatewayVendor extends Enum {
    readonly isPolkadot: boolean;
    readonly isKusama: boolean;
    readonly isRococo: boolean;
    readonly isEthereum: boolean;
    readonly isSepolia: boolean;
    readonly isXbi: boolean;
    readonly type: 'Polkadot' | 'Kusama' | 'Rococo' | 'Ethereum' | 'Sepolia' | 'Xbi';
  }

  /** @name PalletGrandpaFinalityVerifierEvent (190) */
  interface PalletGrandpaFinalityVerifierEvent extends Enum {
    readonly isHeadersAdded: boolean;
    readonly asHeadersAdded: u32;
    readonly type: 'HeadersAdded';
  }

  /** @name PalletEth2FinalityVerifierEvent (193) */
  interface PalletEth2FinalityVerifierEvent extends Enum {
    readonly isEpochUpdate: boolean;
    readonly asEpochUpdate: PalletEth2FinalityVerifierEpochSubmitted;
    readonly type: 'EpochUpdate';
  }

  /** @name PalletEth2FinalityVerifierEpochSubmitted (194) */
  interface PalletEth2FinalityVerifierEpochSubmitted extends Struct {
    readonly epoch: u64;
    readonly beaconHeight: u64;
    readonly executionHeight: u64;
  }

  /** @name PalletSepoliaFinalityVerifierEvent (195) */
  interface PalletSepoliaFinalityVerifierEvent extends Enum {
    readonly isEpochUpdate: boolean;
    readonly asEpochUpdate: PalletSepoliaFinalityVerifierEpochSubmitted;
    readonly type: 'EpochUpdate';
  }

  /** @name PalletSepoliaFinalityVerifierEpochSubmitted (196) */
  interface PalletSepoliaFinalityVerifierEpochSubmitted extends Struct {
    readonly epoch: u64;
    readonly beaconHeight: u64;
    readonly executionHeight: u64;
  }

  /** @name PalletMaintenanceModeEvent (197) */
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

  /** @name PalletSudoEvent (198) */
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

  /** @name FrameSystemPhase (199) */
  interface FrameSystemPhase extends Enum {
    readonly isApplyExtrinsic: boolean;
    readonly asApplyExtrinsic: u32;
    readonly isFinalization: boolean;
    readonly isInitialization: boolean;
    readonly type: 'ApplyExtrinsic' | 'Finalization' | 'Initialization';
  }

  /** @name FrameSystemLastRuntimeUpgradeInfo (201) */
  interface FrameSystemLastRuntimeUpgradeInfo extends Struct {
    readonly specVersion: Compact<u32>;
    readonly specName: Text;
  }

  /** @name FrameSystemCall (203) */
  interface FrameSystemCall extends Enum {
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
    readonly type: 'Remark' | 'SetHeapPages' | 'SetCode' | 'SetCodeWithoutChecks' | 'SetStorage' | 'KillStorage' | 'KillPrefix' | 'RemarkWithEvent';
  }

  /** @name FrameSystemLimitsBlockWeights (206) */
  interface FrameSystemLimitsBlockWeights extends Struct {
    readonly baseBlock: SpWeightsWeightV2Weight;
    readonly maxBlock: SpWeightsWeightV2Weight;
    readonly perClass: FrameSupportDispatchPerDispatchClassWeightsPerClass;
  }

  /** @name FrameSupportDispatchPerDispatchClassWeightsPerClass (207) */
  interface FrameSupportDispatchPerDispatchClassWeightsPerClass extends Struct {
    readonly normal: FrameSystemLimitsWeightsPerClass;
    readonly operational: FrameSystemLimitsWeightsPerClass;
    readonly mandatory: FrameSystemLimitsWeightsPerClass;
  }

  /** @name FrameSystemLimitsWeightsPerClass (208) */
  interface FrameSystemLimitsWeightsPerClass extends Struct {
    readonly baseExtrinsic: SpWeightsWeightV2Weight;
    readonly maxExtrinsic: Option<SpWeightsWeightV2Weight>;
    readonly maxTotal: Option<SpWeightsWeightV2Weight>;
    readonly reserved: Option<SpWeightsWeightV2Weight>;
  }

  /** @name FrameSystemLimitsBlockLength (210) */
  interface FrameSystemLimitsBlockLength extends Struct {
    readonly max: FrameSupportDispatchPerDispatchClassU32;
  }

  /** @name FrameSupportDispatchPerDispatchClassU32 (211) */
  interface FrameSupportDispatchPerDispatchClassU32 extends Struct {
    readonly normal: u32;
    readonly operational: u32;
    readonly mandatory: u32;
  }

  /** @name SpWeightsRuntimeDbWeight (212) */
  interface SpWeightsRuntimeDbWeight extends Struct {
    readonly read: u64;
    readonly write: u64;
  }

  /** @name SpVersionRuntimeVersion (213) */
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

  /** @name FrameSystemError (217) */
  interface FrameSystemError extends Enum {
    readonly isInvalidSpecName: boolean;
    readonly isSpecVersionNeedsToIncrease: boolean;
    readonly isFailedToExtractRuntimeVersion: boolean;
    readonly isNonDefaultComposite: boolean;
    readonly isNonZeroRefCount: boolean;
    readonly isCallFiltered: boolean;
    readonly type: 'InvalidSpecName' | 'SpecVersionNeedsToIncrease' | 'FailedToExtractRuntimeVersion' | 'NonDefaultComposite' | 'NonZeroRefCount' | 'CallFiltered';
  }

  /** @name PolkadotPrimitivesV5PersistedValidationData (218) */
  interface PolkadotPrimitivesV5PersistedValidationData extends Struct {
    readonly parentHead: Bytes;
    readonly relayParentNumber: u32;
    readonly relayParentStorageRoot: H256;
    readonly maxPovSize: u32;
  }

  /** @name PolkadotPrimitivesV5UpgradeRestriction (221) */
  interface PolkadotPrimitivesV5UpgradeRestriction extends Enum {
    readonly isPresent: boolean;
    readonly type: 'Present';
  }

  /** @name SpTrieStorageProof (222) */
  interface SpTrieStorageProof extends Struct {
    readonly trieNodes: BTreeSet<Bytes>;
  }

  /** @name CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot (224) */
  interface CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot extends Struct {
    readonly dmqMqcHead: H256;
    readonly relayDispatchQueueSize: CumulusPalletParachainSystemRelayStateSnapshotRelayDispachQueueSize;
    readonly ingressChannels: Vec<ITuple<[u32, PolkadotPrimitivesV5AbridgedHrmpChannel]>>;
    readonly egressChannels: Vec<ITuple<[u32, PolkadotPrimitivesV5AbridgedHrmpChannel]>>;
  }

  /** @name CumulusPalletParachainSystemRelayStateSnapshotRelayDispachQueueSize (225) */
  interface CumulusPalletParachainSystemRelayStateSnapshotRelayDispachQueueSize extends Struct {
    readonly remainingCount: u32;
    readonly remainingSize: u32;
  }

  /** @name PolkadotPrimitivesV5AbridgedHrmpChannel (228) */
  interface PolkadotPrimitivesV5AbridgedHrmpChannel extends Struct {
    readonly maxCapacity: u32;
    readonly maxTotalSize: u32;
    readonly maxMessageSize: u32;
    readonly msgCount: u32;
    readonly totalSize: u32;
    readonly mqcHead: Option<H256>;
  }

  /** @name PolkadotPrimitivesV5AbridgedHostConfiguration (230) */
  interface PolkadotPrimitivesV5AbridgedHostConfiguration extends Struct {
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

  /** @name PolkadotCorePrimitivesOutboundHrmpMessage (236) */
  interface PolkadotCorePrimitivesOutboundHrmpMessage extends Struct {
    readonly recipient: u32;
    readonly data: Bytes;
  }

  /** @name CumulusPalletParachainSystemCodeUpgradeAuthorization (237) */
  interface CumulusPalletParachainSystemCodeUpgradeAuthorization extends Struct {
    readonly codeHash: H256;
    readonly checkVersion: bool;
  }

  /** @name CumulusPalletParachainSystemCall (238) */
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
      readonly checkVersion: bool;
    } & Struct;
    readonly isEnactAuthorizedUpgrade: boolean;
    readonly asEnactAuthorizedUpgrade: {
      readonly code: Bytes;
    } & Struct;
    readonly type: 'SetValidationData' | 'SudoSendUpwardMessage' | 'AuthorizeUpgrade' | 'EnactAuthorizedUpgrade';
  }

  /** @name CumulusPrimitivesParachainInherentParachainInherentData (239) */
  interface CumulusPrimitivesParachainInherentParachainInherentData extends Struct {
    readonly validationData: PolkadotPrimitivesV5PersistedValidationData;
    readonly relayChainState: SpTrieStorageProof;
    readonly downwardMessages: Vec<PolkadotCorePrimitivesInboundDownwardMessage>;
    readonly horizontalMessages: BTreeMap<u32, Vec<PolkadotCorePrimitivesInboundHrmpMessage>>;
  }

  /** @name PolkadotCorePrimitivesInboundDownwardMessage (241) */
  interface PolkadotCorePrimitivesInboundDownwardMessage extends Struct {
    readonly sentAt: u32;
    readonly msg: Bytes;
  }

  /** @name PolkadotCorePrimitivesInboundHrmpMessage (244) */
  interface PolkadotCorePrimitivesInboundHrmpMessage extends Struct {
    readonly sentAt: u32;
    readonly data: Bytes;
  }

  /** @name CumulusPalletParachainSystemError (247) */
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

  /** @name PalletTimestampCall (248) */
  interface PalletTimestampCall extends Enum {
    readonly isSet: boolean;
    readonly asSet: {
      readonly now: Compact<u64>;
    } & Struct;
    readonly type: 'Set';
  }

  /** @name ParachainInfoCall (249) */
  type ParachainInfoCall = Null;

  /** @name PalletPreimageRequestStatus (250) */
  interface PalletPreimageRequestStatus extends Enum {
    readonly isUnrequested: boolean;
    readonly asUnrequested: {
      readonly deposit: ITuple<[AccountId32, u128]>;
      readonly len: u32;
    } & Struct;
    readonly isRequested: boolean;
    readonly asRequested: {
      readonly deposit: Option<ITuple<[AccountId32, u128]>>;
      readonly count: u32;
      readonly len: Option<u32>;
    } & Struct;
    readonly type: 'Unrequested' | 'Requested';
  }

  /** @name PalletPreimageCall (255) */
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

  /** @name PalletPreimageError (256) */
  interface PalletPreimageError extends Enum {
    readonly isTooBig: boolean;
    readonly isAlreadyNoted: boolean;
    readonly isNotAuthorized: boolean;
    readonly isNotNoted: boolean;
    readonly isRequested: boolean;
    readonly isNotRequested: boolean;
    readonly type: 'TooBig' | 'AlreadyNoted' | 'NotAuthorized' | 'NotNoted' | 'Requested' | 'NotRequested';
  }

  /** @name PalletSchedulerScheduled (259) */
  interface PalletSchedulerScheduled extends Struct {
    readonly maybeId: Option<U8aFixed>;
    readonly priority: u8;
    readonly call: FrameSupportPreimagesBounded;
    readonly maybePeriodic: Option<ITuple<[u32, u32]>>;
    readonly origin: T0rnParachainRuntimeOriginCaller;
  }

  /** @name FrameSupportPreimagesBounded (260) */
  interface FrameSupportPreimagesBounded extends Enum {
    readonly isLegacy: boolean;
    readonly asLegacy: {
      readonly hash_: H256;
    } & Struct;
    readonly isInline: boolean;
    readonly asInline: Bytes;
    readonly isLookup: boolean;
    readonly asLookup: {
      readonly hash_: H256;
      readonly len: u32;
    } & Struct;
    readonly type: 'Legacy' | 'Inline' | 'Lookup';
  }

  /** @name PalletSchedulerCall (262) */
  interface PalletSchedulerCall extends Enum {
    readonly isSchedule: boolean;
    readonly asSchedule: {
      readonly when: u32;
      readonly maybePeriodic: Option<ITuple<[u32, u32]>>;
      readonly priority: u8;
      readonly call: Call;
    } & Struct;
    readonly isCancel: boolean;
    readonly asCancel: {
      readonly when: u32;
      readonly index: u32;
    } & Struct;
    readonly isScheduleNamed: boolean;
    readonly asScheduleNamed: {
      readonly id: U8aFixed;
      readonly when: u32;
      readonly maybePeriodic: Option<ITuple<[u32, u32]>>;
      readonly priority: u8;
      readonly call: Call;
    } & Struct;
    readonly isCancelNamed: boolean;
    readonly asCancelNamed: {
      readonly id: U8aFixed;
    } & Struct;
    readonly isScheduleAfter: boolean;
    readonly asScheduleAfter: {
      readonly after: u32;
      readonly maybePeriodic: Option<ITuple<[u32, u32]>>;
      readonly priority: u8;
      readonly call: Call;
    } & Struct;
    readonly isScheduleNamedAfter: boolean;
    readonly asScheduleNamedAfter: {
      readonly id: U8aFixed;
      readonly after: u32;
      readonly maybePeriodic: Option<ITuple<[u32, u32]>>;
      readonly priority: u8;
      readonly call: Call;
    } & Struct;
    readonly type: 'Schedule' | 'Cancel' | 'ScheduleNamed' | 'CancelNamed' | 'ScheduleAfter' | 'ScheduleNamedAfter';
  }

  /** @name PalletUtilityCall (264) */
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
    readonly isWithWeight: boolean;
    readonly asWithWeight: {
      readonly call: Call;
      readonly weight: SpWeightsWeightV2Weight;
    } & Struct;
    readonly type: 'Batch' | 'AsDerivative' | 'BatchAll' | 'DispatchAs' | 'ForceBatch' | 'WithWeight';
  }

  /** @name T0rnParachainRuntimeOriginCaller (266) */
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

  /** @name FrameSupportDispatchRawOrigin (267) */
  interface FrameSupportDispatchRawOrigin extends Enum {
    readonly isRoot: boolean;
    readonly isSigned: boolean;
    readonly asSigned: AccountId32;
    readonly isNone: boolean;
    readonly type: 'Root' | 'Signed' | 'None';
  }

  /** @name PalletXcmOrigin (268) */
  interface PalletXcmOrigin extends Enum {
    readonly isXcm: boolean;
    readonly asXcm: XcmV3MultiLocation;
    readonly isResponse: boolean;
    readonly asResponse: XcmV3MultiLocation;
    readonly type: 'Xcm' | 'Response';
  }

  /** @name CumulusPalletXcmOrigin (269) */
  interface CumulusPalletXcmOrigin extends Enum {
    readonly isRelay: boolean;
    readonly isSiblingParachain: boolean;
    readonly asSiblingParachain: u32;
    readonly type: 'Relay' | 'SiblingParachain';
  }

  /** @name SpCoreVoid (270) */
  type SpCoreVoid = Null;

  /** @name PalletIdentityCall (271) */
  interface PalletIdentityCall extends Enum {
    readonly isAddRegistrar: boolean;
    readonly asAddRegistrar: {
      readonly account: MultiAddress;
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
      readonly new_: MultiAddress;
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
      readonly identity: H256;
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

  /** @name PalletIdentityIdentityInfo (274) */
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

  /** @name PalletIdentityBitFlags (310) */
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

  /** @name PalletIdentityIdentityField (311) */
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

  /** @name PalletIdentityJudgement (312) */
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

  /** @name PalletBalancesCall (313) */
  interface PalletBalancesCall extends Enum {
    readonly isTransferAllowDeath: boolean;
    readonly asTransferAllowDeath: {
      readonly dest: MultiAddress;
      readonly value: Compact<u128>;
    } & Struct;
    readonly isSetBalanceDeprecated: boolean;
    readonly asSetBalanceDeprecated: {
      readonly who: MultiAddress;
      readonly newFree: Compact<u128>;
      readonly oldReserved: Compact<u128>;
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
    readonly isUpgradeAccounts: boolean;
    readonly asUpgradeAccounts: {
      readonly who: Vec<AccountId32>;
    } & Struct;
    readonly isTransfer: boolean;
    readonly asTransfer: {
      readonly dest: MultiAddress;
      readonly value: Compact<u128>;
    } & Struct;
    readonly isForceSetBalance: boolean;
    readonly asForceSetBalance: {
      readonly who: MultiAddress;
      readonly newFree: Compact<u128>;
    } & Struct;
    readonly type: 'TransferAllowDeath' | 'SetBalanceDeprecated' | 'ForceTransfer' | 'TransferKeepAlive' | 'TransferAll' | 'ForceUnreserve' | 'UpgradeAccounts' | 'Transfer' | 'ForceSetBalance';
  }

  /** @name PalletAssetsCall (314) */
  interface PalletAssetsCall extends Enum {
    readonly isCreate: boolean;
    readonly asCreate: {
      readonly id: u32;
      readonly admin: MultiAddress;
      readonly minBalance: u128;
    } & Struct;
    readonly isForceCreate: boolean;
    readonly asForceCreate: {
      readonly id: u32;
      readonly owner: MultiAddress;
      readonly isSufficient: bool;
      readonly minBalance: Compact<u128>;
    } & Struct;
    readonly isStartDestroy: boolean;
    readonly asStartDestroy: {
      readonly id: u32;
    } & Struct;
    readonly isDestroyAccounts: boolean;
    readonly asDestroyAccounts: {
      readonly id: u32;
    } & Struct;
    readonly isDestroyApprovals: boolean;
    readonly asDestroyApprovals: {
      readonly id: u32;
    } & Struct;
    readonly isFinishDestroy: boolean;
    readonly asFinishDestroy: {
      readonly id: u32;
    } & Struct;
    readonly isMint: boolean;
    readonly asMint: {
      readonly id: u32;
      readonly beneficiary: MultiAddress;
      readonly amount: Compact<u128>;
    } & Struct;
    readonly isBurn: boolean;
    readonly asBurn: {
      readonly id: u32;
      readonly who: MultiAddress;
      readonly amount: Compact<u128>;
    } & Struct;
    readonly isTransfer: boolean;
    readonly asTransfer: {
      readonly id: u32;
      readonly target: MultiAddress;
      readonly amount: Compact<u128>;
    } & Struct;
    readonly isTransferKeepAlive: boolean;
    readonly asTransferKeepAlive: {
      readonly id: u32;
      readonly target: MultiAddress;
      readonly amount: Compact<u128>;
    } & Struct;
    readonly isForceTransfer: boolean;
    readonly asForceTransfer: {
      readonly id: u32;
      readonly source: MultiAddress;
      readonly dest: MultiAddress;
      readonly amount: Compact<u128>;
    } & Struct;
    readonly isFreeze: boolean;
    readonly asFreeze: {
      readonly id: u32;
      readonly who: MultiAddress;
    } & Struct;
    readonly isThaw: boolean;
    readonly asThaw: {
      readonly id: u32;
      readonly who: MultiAddress;
    } & Struct;
    readonly isFreezeAsset: boolean;
    readonly asFreezeAsset: {
      readonly id: u32;
    } & Struct;
    readonly isThawAsset: boolean;
    readonly asThawAsset: {
      readonly id: u32;
    } & Struct;
    readonly isTransferOwnership: boolean;
    readonly asTransferOwnership: {
      readonly id: u32;
      readonly owner: MultiAddress;
    } & Struct;
    readonly isSetTeam: boolean;
    readonly asSetTeam: {
      readonly id: u32;
      readonly issuer: MultiAddress;
      readonly admin: MultiAddress;
      readonly freezer: MultiAddress;
    } & Struct;
    readonly isSetMetadata: boolean;
    readonly asSetMetadata: {
      readonly id: u32;
      readonly name: Bytes;
      readonly symbol: Bytes;
      readonly decimals: u8;
    } & Struct;
    readonly isClearMetadata: boolean;
    readonly asClearMetadata: {
      readonly id: u32;
    } & Struct;
    readonly isForceSetMetadata: boolean;
    readonly asForceSetMetadata: {
      readonly id: u32;
      readonly name: Bytes;
      readonly symbol: Bytes;
      readonly decimals: u8;
      readonly isFrozen: bool;
    } & Struct;
    readonly isForceClearMetadata: boolean;
    readonly asForceClearMetadata: {
      readonly id: u32;
    } & Struct;
    readonly isForceAssetStatus: boolean;
    readonly asForceAssetStatus: {
      readonly id: u32;
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
      readonly id: u32;
      readonly delegate: MultiAddress;
      readonly amount: Compact<u128>;
    } & Struct;
    readonly isCancelApproval: boolean;
    readonly asCancelApproval: {
      readonly id: u32;
      readonly delegate: MultiAddress;
    } & Struct;
    readonly isForceCancelApproval: boolean;
    readonly asForceCancelApproval: {
      readonly id: u32;
      readonly owner: MultiAddress;
      readonly delegate: MultiAddress;
    } & Struct;
    readonly isTransferApproved: boolean;
    readonly asTransferApproved: {
      readonly id: u32;
      readonly owner: MultiAddress;
      readonly destination: MultiAddress;
      readonly amount: Compact<u128>;
    } & Struct;
    readonly isTouch: boolean;
    readonly asTouch: {
      readonly id: u32;
    } & Struct;
    readonly isRefund: boolean;
    readonly asRefund: {
      readonly id: u32;
      readonly allowBurn: bool;
    } & Struct;
    readonly isSetMinBalance: boolean;
    readonly asSetMinBalance: {
      readonly id: u32;
      readonly minBalance: u128;
    } & Struct;
    readonly isTouchOther: boolean;
    readonly asTouchOther: {
      readonly id: u32;
      readonly who: MultiAddress;
    } & Struct;
    readonly isRefundOther: boolean;
    readonly asRefundOther: {
      readonly id: u32;
      readonly who: MultiAddress;
    } & Struct;
    readonly isBlock: boolean;
    readonly asBlock: {
      readonly id: u32;
      readonly who: MultiAddress;
    } & Struct;
    readonly type: 'Create' | 'ForceCreate' | 'StartDestroy' | 'DestroyAccounts' | 'DestroyApprovals' | 'FinishDestroy' | 'Mint' | 'Burn' | 'Transfer' | 'TransferKeepAlive' | 'ForceTransfer' | 'Freeze' | 'Thaw' | 'FreezeAsset' | 'ThawAsset' | 'TransferOwnership' | 'SetTeam' | 'SetMetadata' | 'ClearMetadata' | 'ForceSetMetadata' | 'ForceClearMetadata' | 'ForceAssetStatus' | 'ApproveTransfer' | 'CancelApproval' | 'ForceCancelApproval' | 'TransferApproved' | 'Touch' | 'Refund' | 'SetMinBalance' | 'TouchOther' | 'RefundOther' | 'Block';
  }

  /** @name PalletAccountManagerCall (315) */
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

  /** @name T3rnPrimitivesClaimableBenefitSource (316) */
  interface T3rnPrimitivesClaimableBenefitSource extends Enum {
    readonly isBootstrapPool: boolean;
    readonly isInflation: boolean;
    readonly isTrafficFees: boolean;
    readonly isTrafficRewards: boolean;
    readonly isUnsettled: boolean;
    readonly isSlashTreasury: boolean;
    readonly type: 'BootstrapPool' | 'Inflation' | 'TrafficFees' | 'TrafficRewards' | 'Unsettled' | 'SlashTreasury';
  }

  /** @name T3rnPrimitivesClaimableCircuitRole (317) */
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

  /** @name T3rnPrimitivesAccountManagerOutcome (318) */
  interface T3rnPrimitivesAccountManagerOutcome extends Enum {
    readonly isUnexpectedFailure: boolean;
    readonly isRevert: boolean;
    readonly isCommit: boolean;
    readonly isSlash: boolean;
    readonly type: 'UnexpectedFailure' | 'Revert' | 'Commit' | 'Slash';
  }

  /** @name PalletTreasuryCall (319) */
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

  /** @name PalletCollatorSelectionCall (324) */
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
    readonly isAddInvulnerable: boolean;
    readonly asAddInvulnerable: {
      readonly who: AccountId32;
    } & Struct;
    readonly isRemoveInvulnerable: boolean;
    readonly asRemoveInvulnerable: {
      readonly who: AccountId32;
    } & Struct;
    readonly type: 'SetInvulnerables' | 'SetDesiredCandidates' | 'SetCandidacyBond' | 'RegisterAsCandidate' | 'LeaveIntent' | 'AddInvulnerable' | 'RemoveInvulnerable';
  }

  /** @name PalletSessionCall (325) */
  interface PalletSessionCall extends Enum {
    readonly isSetKeys: boolean;
    readonly asSetKeys: {
      readonly keys_: T0rnParachainRuntimeParachainConfigSessionKeys;
      readonly proof: Bytes;
    } & Struct;
    readonly isPurgeKeys: boolean;
    readonly type: 'SetKeys' | 'PurgeKeys';
  }

  /** @name T0rnParachainRuntimeParachainConfigSessionKeys (326) */
  interface T0rnParachainRuntimeParachainConfigSessionKeys extends Struct {
    readonly aura: SpConsensusAuraSr25519AppSr25519Public;
  }

  /** @name SpConsensusAuraSr25519AppSr25519Public (327) */
  interface SpConsensusAuraSr25519AppSr25519Public extends SpCoreSr25519Public {}

  /** @name SpCoreSr25519Public (328) */
  interface SpCoreSr25519Public extends U8aFixed {}

  /** @name CumulusPalletXcmpQueueCall (329) */
  interface CumulusPalletXcmpQueueCall extends Enum {
    readonly isServiceOverweight: boolean;
    readonly asServiceOverweight: {
      readonly index: u64;
      readonly weightLimit: SpWeightsWeightV2Weight;
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
      readonly new_: SpWeightsWeightV2Weight;
    } & Struct;
    readonly isUpdateWeightRestrictDecay: boolean;
    readonly asUpdateWeightRestrictDecay: {
      readonly new_: SpWeightsWeightV2Weight;
    } & Struct;
    readonly isUpdateXcmpMaxIndividualWeight: boolean;
    readonly asUpdateXcmpMaxIndividualWeight: {
      readonly new_: SpWeightsWeightV2Weight;
    } & Struct;
    readonly type: 'ServiceOverweight' | 'SuspendXcmExecution' | 'ResumeXcmExecution' | 'UpdateSuspendThreshold' | 'UpdateDropThreshold' | 'UpdateResumeThreshold' | 'UpdateThresholdWeight' | 'UpdateWeightRestrictDecay' | 'UpdateXcmpMaxIndividualWeight';
  }

  /** @name PalletXcmCall (330) */
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
      readonly maxWeight: SpWeightsWeightV2Weight;
    } & Struct;
    readonly isForceXcmVersion: boolean;
    readonly asForceXcmVersion: {
      readonly location: XcmV3MultiLocation;
      readonly version: u32;
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
      readonly weightLimit: XcmV3WeightLimit;
    } & Struct;
    readonly isLimitedTeleportAssets: boolean;
    readonly asLimitedTeleportAssets: {
      readonly dest: XcmVersionedMultiLocation;
      readonly beneficiary: XcmVersionedMultiLocation;
      readonly assets: XcmVersionedMultiAssets;
      readonly feeAssetItem: u32;
      readonly weightLimit: XcmV3WeightLimit;
    } & Struct;
    readonly isForceSuspension: boolean;
    readonly asForceSuspension: {
      readonly suspended: bool;
    } & Struct;
    readonly type: 'Send' | 'TeleportAssets' | 'ReserveTransferAssets' | 'Execute' | 'ForceXcmVersion' | 'ForceDefaultXcmVersion' | 'ForceSubscribeVersionNotify' | 'ForceUnsubscribeVersionNotify' | 'LimitedReserveTransferAssets' | 'LimitedTeleportAssets' | 'ForceSuspension';
  }

  /** @name XcmVersionedXcm (331) */
  interface XcmVersionedXcm extends Enum {
    readonly isV2: boolean;
    readonly asV2: XcmV2Xcm;
    readonly isV3: boolean;
    readonly asV3: XcmV3Xcm;
    readonly type: 'V2' | 'V3';
  }

  /** @name XcmV2Xcm (332) */
  interface XcmV2Xcm extends Vec<XcmV2Instruction> {}

  /** @name XcmV2Instruction (334) */
  interface XcmV2Instruction extends Enum {
    readonly isWithdrawAsset: boolean;
    readonly asWithdrawAsset: XcmV2MultiassetMultiAssets;
    readonly isReserveAssetDeposited: boolean;
    readonly asReserveAssetDeposited: XcmV2MultiassetMultiAssets;
    readonly isReceiveTeleportedAsset: boolean;
    readonly asReceiveTeleportedAsset: XcmV2MultiassetMultiAssets;
    readonly isQueryResponse: boolean;
    readonly asQueryResponse: {
      readonly queryId: Compact<u64>;
      readonly response: XcmV2Response;
      readonly maxWeight: Compact<u64>;
    } & Struct;
    readonly isTransferAsset: boolean;
    readonly asTransferAsset: {
      readonly assets: XcmV2MultiassetMultiAssets;
      readonly beneficiary: XcmV2MultiLocation;
    } & Struct;
    readonly isTransferReserveAsset: boolean;
    readonly asTransferReserveAsset: {
      readonly assets: XcmV2MultiassetMultiAssets;
      readonly dest: XcmV2MultiLocation;
      readonly xcm: XcmV2Xcm;
    } & Struct;
    readonly isTransact: boolean;
    readonly asTransact: {
      readonly originType: XcmV2OriginKind;
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
    readonly asDescendOrigin: XcmV2MultilocationJunctions;
    readonly isReportError: boolean;
    readonly asReportError: {
      readonly queryId: Compact<u64>;
      readonly dest: XcmV2MultiLocation;
      readonly maxResponseWeight: Compact<u64>;
    } & Struct;
    readonly isDepositAsset: boolean;
    readonly asDepositAsset: {
      readonly assets: XcmV2MultiassetMultiAssetFilter;
      readonly maxAssets: Compact<u32>;
      readonly beneficiary: XcmV2MultiLocation;
    } & Struct;
    readonly isDepositReserveAsset: boolean;
    readonly asDepositReserveAsset: {
      readonly assets: XcmV2MultiassetMultiAssetFilter;
      readonly maxAssets: Compact<u32>;
      readonly dest: XcmV2MultiLocation;
      readonly xcm: XcmV2Xcm;
    } & Struct;
    readonly isExchangeAsset: boolean;
    readonly asExchangeAsset: {
      readonly give: XcmV2MultiassetMultiAssetFilter;
      readonly receive: XcmV2MultiassetMultiAssets;
    } & Struct;
    readonly isInitiateReserveWithdraw: boolean;
    readonly asInitiateReserveWithdraw: {
      readonly assets: XcmV2MultiassetMultiAssetFilter;
      readonly reserve: XcmV2MultiLocation;
      readonly xcm: XcmV2Xcm;
    } & Struct;
    readonly isInitiateTeleport: boolean;
    readonly asInitiateTeleport: {
      readonly assets: XcmV2MultiassetMultiAssetFilter;
      readonly dest: XcmV2MultiLocation;
      readonly xcm: XcmV2Xcm;
    } & Struct;
    readonly isQueryHolding: boolean;
    readonly asQueryHolding: {
      readonly queryId: Compact<u64>;
      readonly dest: XcmV2MultiLocation;
      readonly assets: XcmV2MultiassetMultiAssetFilter;
      readonly maxResponseWeight: Compact<u64>;
    } & Struct;
    readonly isBuyExecution: boolean;
    readonly asBuyExecution: {
      readonly fees: XcmV2MultiAsset;
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
      readonly assets: XcmV2MultiassetMultiAssets;
      readonly ticket: XcmV2MultiLocation;
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

  /** @name XcmV2Response (335) */
  interface XcmV2Response extends Enum {
    readonly isNull: boolean;
    readonly isAssets: boolean;
    readonly asAssets: XcmV2MultiassetMultiAssets;
    readonly isExecutionResult: boolean;
    readonly asExecutionResult: Option<ITuple<[u32, XcmV2TraitsError]>>;
    readonly isVersion: boolean;
    readonly asVersion: u32;
    readonly type: 'Null' | 'Assets' | 'ExecutionResult' | 'Version';
  }

  /** @name XcmV2TraitsError (338) */
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

  /** @name XcmV2MultiassetMultiAssetFilter (339) */
  interface XcmV2MultiassetMultiAssetFilter extends Enum {
    readonly isDefinite: boolean;
    readonly asDefinite: XcmV2MultiassetMultiAssets;
    readonly isWild: boolean;
    readonly asWild: XcmV2MultiassetWildMultiAsset;
    readonly type: 'Definite' | 'Wild';
  }

  /** @name XcmV2MultiassetWildMultiAsset (340) */
  interface XcmV2MultiassetWildMultiAsset extends Enum {
    readonly isAll: boolean;
    readonly isAllOf: boolean;
    readonly asAllOf: {
      readonly id: XcmV2MultiassetAssetId;
      readonly fun: XcmV2MultiassetWildFungibility;
    } & Struct;
    readonly type: 'All' | 'AllOf';
  }

  /** @name XcmV2MultiassetWildFungibility (341) */
  interface XcmV2MultiassetWildFungibility extends Enum {
    readonly isFungible: boolean;
    readonly isNonFungible: boolean;
    readonly type: 'Fungible' | 'NonFungible';
  }

  /** @name XcmV2WeightLimit (342) */
  interface XcmV2WeightLimit extends Enum {
    readonly isUnlimited: boolean;
    readonly isLimited: boolean;
    readonly asLimited: Compact<u64>;
    readonly type: 'Unlimited' | 'Limited';
  }

  /** @name CumulusPalletXcmCall (351) */
  type CumulusPalletXcmCall = Null;

  /** @name CumulusPalletDmpQueueCall (352) */
  interface CumulusPalletDmpQueueCall extends Enum {
    readonly isServiceOverweight: boolean;
    readonly asServiceOverweight: {
      readonly index: u64;
      readonly weightLimit: SpWeightsWeightV2Weight;
    } & Struct;
    readonly type: 'ServiceOverweight';
  }

  /** @name PalletAssetRegistryCall (353) */
  interface PalletAssetRegistryCall extends Enum {
    readonly isRegister: boolean;
    readonly asRegister: {
      readonly location: XcmV3MultiLocation;
      readonly id: u32;
    } & Struct;
    readonly isRegisterInfo: boolean;
    readonly asRegisterInfo: {
      readonly info: PalletAssetRegistryAssetInfo;
    } & Struct;
    readonly type: 'Register' | 'RegisterInfo';
  }

  /** @name PalletAssetRegistryAssetInfo (354) */
  interface PalletAssetRegistryAssetInfo extends Struct {
    readonly id: u32;
    readonly capabilities: Vec<PalletAssetRegistryCapability>;
    readonly location: XcmV3MultiLocation;
  }

  /** @name PalletAssetRegistryCapability (356) */
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

  /** @name PalletXdnsCall (357) */
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

  /** @name PalletAttestersCall (358) */
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
    readonly isReadPendingBatches: boolean;
    readonly isReadLatestBatchingFactorOverview: boolean;
    readonly isEstimateUserFinalityFee: boolean;
    readonly asEstimateUserFinalityFee: {
      readonly target: U8aFixed;
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
    readonly type: 'RegisterAttester' | 'DeregisterAttester' | 'RemoveAttestationTarget' | 'AgreeToNewAttestationTarget' | 'ForceActivateTarget' | 'AddAttestationTarget' | 'SubmitAttestation' | 'CommitBatch' | 'ReadPendingBatches' | 'ReadLatestBatchingFactorOverview' | 'EstimateUserFinalityFee' | 'Nominate' | 'Unnominate';
  }

  /** @name PalletRewardsCall (361) */
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

  /** @name PalletContractsRegistryCall (363) */
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

  /** @name T3rnPrimitivesContractsRegistryRegistryContract (364) */
  interface T3rnPrimitivesContractsRegistryRegistryContract extends Struct {
    readonly codeTxt: Bytes;
    readonly bytes: Bytes;
    readonly author: T3rnPrimitivesContractsRegistryAuthorInfo;
    readonly abi: Option<Bytes>;
    readonly actionDescriptions: Vec<T3rnTypesGatewayContractActionDesc>;
    readonly info: Option<T3rnPrimitivesStorageRawAliveContractInfo>;
    readonly meta: T3rnPrimitivesContractMetadata;
  }

  /** @name T3rnPrimitivesContractsRegistryAuthorInfo (365) */
  interface T3rnPrimitivesContractsRegistryAuthorInfo extends Struct {
    readonly account: AccountId32;
    readonly feesPerSingleUse: Option<u128>;
  }

  /** @name T3rnTypesGatewayContractActionDesc (367) */
  interface T3rnTypesGatewayContractActionDesc extends Struct {
    readonly actionId: H256;
    readonly targetId: Option<U8aFixed>;
    readonly to: Option<AccountId32>;
  }

  /** @name T3rnPrimitivesStorageRawAliveContractInfo (370) */
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

  /** @name T3rnPrimitivesContractMetadata (372) */
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

  /** @name PalletCircuitCall (373) */
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
    readonly isTriggerDlq: boolean;
    readonly isOnRemoteOriginTrigger: boolean;
    readonly asOnRemoteOriginTrigger: {
      readonly orderOrigin: AccountId32;
      readonly sideEffects: Vec<T3rnTypesSfxSideEffect>;
      readonly speedMode: T3rnPrimitivesSpeedMode;
    } & Struct;
    readonly isOnExtrinsicTrigger: boolean;
    readonly asOnExtrinsicTrigger: {
      readonly sideEffects: Vec<T3rnTypesSfxSideEffect>;
      readonly speedMode: T3rnPrimitivesSpeedMode;
      readonly preferredSecurityLevel: T3rnTypesSfxSecurityLvl;
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
    readonly type: 'OnLocalTrigger' | 'OnXcmTrigger' | 'OnRemoteGatewayTrigger' | 'CancelXtx' | 'Revert' | 'TriggerDlq' | 'OnRemoteOriginTrigger' | 'OnExtrinsicTrigger' | 'BidSfx' | 'ConfirmSideEffect';
  }

  /** @name T3rnPrimitivesSpeedMode (374) */
  interface T3rnPrimitivesSpeedMode extends Enum {
    readonly isFast: boolean;
    readonly isRational: boolean;
    readonly isFinalized: boolean;
    readonly type: 'Fast' | 'Rational' | 'Finalized';
  }

  /** @name PalletCircuitVacuumCall (375) */
  interface PalletCircuitVacuumCall extends Enum {
    readonly isOrder: boolean;
    readonly asOrder: {
      readonly sfxActions: Vec<T3rnPrimitivesCircuitTypesOrderSFX>;
      readonly speedMode: T3rnPrimitivesSpeedMode;
    } & Struct;
    readonly isReadOrderStatus: boolean;
    readonly asReadOrderStatus: {
      readonly xtxId: H256;
    } & Struct;
    readonly type: 'Order' | 'ReadOrderStatus';
  }

  /** @name T3rnPrimitivesCircuitTypesOrderSFX (377) */
  interface T3rnPrimitivesCircuitTypesOrderSFX extends Struct {
    readonly sfxAction: T3rnPrimitivesCircuitTypesSfxAction;
    readonly maxReward: u128;
    readonly rewardAsset: u32;
    readonly insurance: u128;
    readonly remoteOriginNonce: Option<u32>;
  }

  /** @name T3rnPrimitivesCircuitTypesSfxAction (378) */
  interface T3rnPrimitivesCircuitTypesSfxAction extends Enum {
    readonly isCall: boolean;
    readonly asCall: ITuple<[U8aFixed, AccountId32, u128, u128, Bytes]>;
    readonly isTransfer: boolean;
    readonly asTransfer: ITuple<[U8aFixed, u32, AccountId32, u128]>;
    readonly type: 'Call' | 'Transfer';
  }

  /** @name Pallet3vmCall (379) */
  type Pallet3vmCall = Null;

  /** @name PalletContractsCall (380) */
  interface PalletContractsCall extends Enum {
    readonly isCallOldWeight: boolean;
    readonly asCallOldWeight: {
      readonly dest: MultiAddress;
      readonly value: Compact<u128>;
      readonly gasLimit: Compact<u64>;
      readonly storageDepositLimit: Option<Compact<u128>>;
      readonly data: Bytes;
    } & Struct;
    readonly isInstantiateWithCodeOldWeight: boolean;
    readonly asInstantiateWithCodeOldWeight: {
      readonly value: Compact<u128>;
      readonly gasLimit: Compact<u64>;
      readonly storageDepositLimit: Option<Compact<u128>>;
      readonly code: Bytes;
      readonly data: Bytes;
      readonly salt: Bytes;
    } & Struct;
    readonly isInstantiateOldWeight: boolean;
    readonly asInstantiateOldWeight: {
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
      readonly determinism: PalletContractsWasmDeterminism;
    } & Struct;
    readonly isRemoveCode: boolean;
    readonly asRemoveCode: {
      readonly codeHash: H256;
    } & Struct;
    readonly isSetCode: boolean;
    readonly asSetCode: {
      readonly dest: MultiAddress;
      readonly codeHash: H256;
    } & Struct;
    readonly isCall: boolean;
    readonly asCall: {
      readonly dest: MultiAddress;
      readonly value: Compact<u128>;
      readonly gasLimit: SpWeightsWeightV2Weight;
      readonly storageDepositLimit: Option<Compact<u128>>;
      readonly data: Bytes;
    } & Struct;
    readonly isInstantiateWithCode: boolean;
    readonly asInstantiateWithCode: {
      readonly value: Compact<u128>;
      readonly gasLimit: SpWeightsWeightV2Weight;
      readonly storageDepositLimit: Option<Compact<u128>>;
      readonly code: Bytes;
      readonly data: Bytes;
      readonly salt: Bytes;
    } & Struct;
    readonly isInstantiate: boolean;
    readonly asInstantiate: {
      readonly value: Compact<u128>;
      readonly gasLimit: SpWeightsWeightV2Weight;
      readonly storageDepositLimit: Option<Compact<u128>>;
      readonly codeHash: H256;
      readonly data: Bytes;
      readonly salt: Bytes;
    } & Struct;
    readonly isMigrate: boolean;
    readonly asMigrate: {
      readonly weightLimit: SpWeightsWeightV2Weight;
    } & Struct;
    readonly type: 'CallOldWeight' | 'InstantiateWithCodeOldWeight' | 'InstantiateOldWeight' | 'UploadCode' | 'RemoveCode' | 'SetCode' | 'Call' | 'InstantiateWithCode' | 'Instantiate' | 'Migrate';
  }

  /** @name PalletContractsWasmDeterminism (382) */
  interface PalletContractsWasmDeterminism extends Enum {
    readonly isEnforced: boolean;
    readonly isRelaxed: boolean;
    readonly type: 'Enforced' | 'Relaxed';
  }

  /** @name PalletEvmCall (383) */
  interface PalletEvmCall extends Enum {
    readonly isWithdraw: boolean;
    readonly asWithdraw: {
      readonly address: H160;
      readonly value: u128;
    } & Struct;
    readonly isCall: boolean;
    readonly asCall: {
      readonly source: H160;
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
      readonly source: H160;
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
      readonly source: H160;
      readonly init: Bytes;
      readonly salt: H256;
      readonly value: U256;
      readonly gasLimit: u64;
      readonly maxFeePerGas: U256;
      readonly maxPriorityFeePerGas: Option<U256>;
      readonly nonce: Option<U256>;
      readonly accessList: Vec<ITuple<[H160, Vec<H256>]>>;
    } & Struct;
    readonly type: 'Withdraw' | 'Call' | 'Create' | 'Create2';
  }

  /** @name PalletPortalCall (384) */
  interface PalletPortalCall extends Enum {
    readonly isRegisterGateway: boolean;
    readonly asRegisterGateway: {
      readonly gatewayId: U8aFixed;
      readonly tokenId: u32;
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

  /** @name T3rnAbiRecodeCodec (385) */
  interface T3rnAbiRecodeCodec extends Enum {
    readonly isScale: boolean;
    readonly isRlp: boolean;
    readonly type: 'Scale' | 'Rlp';
  }

  /** @name T3rnPrimitivesTokenInfo (389) */
  interface T3rnPrimitivesTokenInfo extends Enum {
    readonly isSubstrate: boolean;
    readonly asSubstrate: T3rnPrimitivesSubstrateToken;
    readonly isEthereum: boolean;
    readonly asEthereum: T3rnPrimitivesEthereumToken;
    readonly type: 'Substrate' | 'Ethereum';
  }

  /** @name T3rnPrimitivesSubstrateToken (390) */
  interface T3rnPrimitivesSubstrateToken extends Struct {
    readonly id: u32;
    readonly symbol: Bytes;
    readonly decimals: u8;
  }

  /** @name T3rnPrimitivesEthereumToken (391) */
  interface T3rnPrimitivesEthereumToken extends Struct {
    readonly symbol: Bytes;
    readonly decimals: u8;
    readonly address: Option<U8aFixed>;
  }

  /** @name PalletGrandpaFinalityVerifierCall (392) */
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

  /** @name SpRuntimeHeader (394) */
  interface SpRuntimeHeader extends Struct {
    readonly parentHash: H256;
    readonly number: Compact<u32>;
    readonly stateRoot: H256;
    readonly extrinsicsRoot: H256;
    readonly digest: SpRuntimeDigest;
  }

  /** @name PalletGrandpaFinalityVerifierBridgesHeaderChainJustificationGrandpaJustification (395) */
  interface PalletGrandpaFinalityVerifierBridgesHeaderChainJustificationGrandpaJustification extends Struct {
    readonly round: u64;
    readonly commit: FinalityGrandpaCommit;
    readonly votesAncestries: Vec<SpRuntimeHeader>;
  }

  /** @name FinalityGrandpaCommit (396) */
  interface FinalityGrandpaCommit extends Struct {
    readonly targetHash: H256;
    readonly targetNumber: u32;
    readonly precommits: Vec<FinalityGrandpaSignedPrecommit>;
  }

  /** @name SpConsensusGrandpaAppSignature (397) */
  interface SpConsensusGrandpaAppSignature extends SpCoreEd25519Signature {}

  /** @name SpCoreEd25519Signature (398) */
  interface SpCoreEd25519Signature extends U8aFixed {}

  /** @name SpConsensusGrandpaAppPublic (400) */
  interface SpConsensusGrandpaAppPublic extends SpCoreEd25519Public {}

  /** @name SpCoreEd25519Public (401) */
  interface SpCoreEd25519Public extends U8aFixed {}

  /** @name FinalityGrandpaSignedPrecommit (403) */
  interface FinalityGrandpaSignedPrecommit extends Struct {
    readonly precommit: FinalityGrandpaPrecommit;
    readonly signature: SpConsensusGrandpaAppSignature;
    readonly id: SpConsensusGrandpaAppPublic;
  }

  /** @name FinalityGrandpaPrecommit (404) */
  interface FinalityGrandpaPrecommit extends Struct {
    readonly targetHash: H256;
    readonly targetNumber: u32;
  }

  /** @name PalletEth2FinalityVerifierCall (407) */
  interface PalletEth2FinalityVerifierCall extends Enum {
    readonly isSubmitEpochDebug: boolean;
    readonly asSubmitEpochDebug: {
      readonly attestedBeaconHeader: PalletEth2FinalityVerifierBeaconBlockHeader;
      readonly signature: U8aFixed;
      readonly signerBits: Vec<bool>;
      readonly justifiedProof: PalletEth2FinalityVerifierMerkleProof;
      readonly executionPayload: PalletEth2FinalityVerifierExecutionPayload;
      readonly payloadProof: PalletEth2FinalityVerifierMerkleProof;
      readonly executionRange: Vec<PalletEth2FinalityVerifierExecutionHeader>;
    } & Struct;
    readonly isSubmitEpoch: boolean;
    readonly asSubmitEpoch: {
      readonly encodedUpdate: Bytes;
    } & Struct;
    readonly isSubmitEpochSkippedSlot: boolean;
    readonly asSubmitEpochSkippedSlot: {
      readonly encodedUpdate: Bytes;
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
      readonly speedMode: T3rnPrimitivesSpeedMode;
    } & Struct;
    readonly isVerifyEventInclusion: boolean;
    readonly asVerifyEventInclusion: {
      readonly proof: PalletEth2FinalityVerifierEthereumEventInclusionProof;
      readonly speedMode: T3rnPrimitivesSpeedMode;
      readonly sourceAddress: Option<H160>;
    } & Struct;
    readonly isReset: boolean;
    readonly type: 'SubmitEpochDebug' | 'SubmitEpoch' | 'SubmitEpochSkippedSlot' | 'SubmitFork' | 'AddNextSyncCommittee' | 'VerifyReceiptInclusion' | 'VerifyEventInclusion' | 'Reset';
  }

  /** @name PalletEth2FinalityVerifierBeaconBlockHeader (408) */
  interface PalletEth2FinalityVerifierBeaconBlockHeader extends Struct {
    readonly slot: u64;
    readonly proposerIndex: u64;
    readonly parentRoot: U8aFixed;
    readonly stateRoot: U8aFixed;
    readonly bodyRoot: U8aFixed;
  }

  /** @name PalletEth2FinalityVerifierMerkleProof (411) */
  interface PalletEth2FinalityVerifierMerkleProof extends Struct {
    readonly gIndex: u64;
    readonly witness: Vec<U8aFixed>;
  }

  /** @name PalletEth2FinalityVerifierExecutionPayload (413) */
  interface PalletEth2FinalityVerifierExecutionPayload extends Struct {
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

  /** @name EthbloomBloom (414) */
  interface EthbloomBloom extends U8aFixed {}

  /** @name PalletEth2FinalityVerifierExecutionHeader (417) */
  interface PalletEth2FinalityVerifierExecutionHeader extends Struct {
    readonly parentHash: U8aFixed;
    readonly ommersHash: U8aFixed;
    readonly beneficiary: H160;
    readonly stateRoot: U8aFixed;
    readonly transactionsRoot: U8aFixed;
    readonly receiptsRoot: U8aFixed;
    readonly logsBloom: EthbloomBloom;
    readonly difficulty: U256;
    readonly number: u64;
    readonly gasLimit: u64;
    readonly gasUsed: u64;
    readonly timestamp: u64;
    readonly extraData: Bytes;
    readonly mixHash: U8aFixed;
    readonly nonce: u64;
    readonly baseFeePerGas: u64;
    readonly withdrawalsRoot: U8aFixed;
  }

  /** @name PalletEth2FinalityVerifierSyncCommittee (418) */
  interface PalletEth2FinalityVerifierSyncCommittee extends Struct {
    readonly pubs: Vec<U8aFixed>;
    readonly aggr: U8aFixed;
  }

  /** @name PalletEth2FinalityVerifierEthereumReceiptInclusionProof (421) */
  interface PalletEth2FinalityVerifierEthereumReceiptInclusionProof extends Struct {
    readonly blockNumber: u64;
    readonly witness: Vec<Bytes>;
    readonly index: Bytes;
  }

  /** @name PalletEth2FinalityVerifierEthereumEventInclusionProof (422) */
  interface PalletEth2FinalityVerifierEthereumEventInclusionProof extends Struct {
    readonly blockNumber: u64;
    readonly witness: Vec<Bytes>;
    readonly index: Bytes;
    readonly event: Bytes;
  }

  /** @name PalletSepoliaFinalityVerifierCall (424) */
  interface PalletSepoliaFinalityVerifierCall extends Enum {
    readonly isSubmitEpochDebug: boolean;
    readonly asSubmitEpochDebug: {
      readonly attestedBeaconHeader: PalletSepoliaFinalityVerifierBeaconBlockHeader;
      readonly signature: U8aFixed;
      readonly signerBits: Vec<bool>;
      readonly justifiedProof: PalletSepoliaFinalityVerifierMerkleProof;
      readonly executionPayload: PalletSepoliaFinalityVerifierExecutionPayload;
      readonly payloadProof: PalletSepoliaFinalityVerifierMerkleProof;
      readonly executionRange: Vec<PalletSepoliaFinalityVerifierExecutionHeader>;
    } & Struct;
    readonly isSubmitEpoch: boolean;
    readonly asSubmitEpoch: {
      readonly encodedUpdate: Bytes;
    } & Struct;
    readonly isSubmitEpochSkippedSlot: boolean;
    readonly asSubmitEpochSkippedSlot: {
      readonly encodedUpdate: Bytes;
    } & Struct;
    readonly isSubmitFork: boolean;
    readonly asSubmitFork: {
      readonly encodedNewUpdate: Bytes;
      readonly encodedOldUpdate: Bytes;
    } & Struct;
    readonly isAddNextSyncCommittee: boolean;
    readonly asAddNextSyncCommittee: {
      readonly nextSyncCommittee: PalletSepoliaFinalityVerifierSyncCommittee;
      readonly proof: PalletSepoliaFinalityVerifierMerkleProof;
      readonly proofSlot: u64;
    } & Struct;
    readonly isVerifyReceiptInclusion: boolean;
    readonly asVerifyReceiptInclusion: {
      readonly proof: PalletSepoliaFinalityVerifierEthereumReceiptInclusionProof;
      readonly speedMode: T3rnPrimitivesSpeedMode;
    } & Struct;
    readonly isVerifyEventInclusion: boolean;
    readonly asVerifyEventInclusion: {
      readonly proof: PalletSepoliaFinalityVerifierEthereumEventInclusionProof;
      readonly speedMode: T3rnPrimitivesSpeedMode;
      readonly sourceAddress: Option<H160>;
    } & Struct;
    readonly isReset: boolean;
    readonly type: 'SubmitEpochDebug' | 'SubmitEpoch' | 'SubmitEpochSkippedSlot' | 'SubmitFork' | 'AddNextSyncCommittee' | 'VerifyReceiptInclusion' | 'VerifyEventInclusion' | 'Reset';
  }

  /** @name PalletSepoliaFinalityVerifierBeaconBlockHeader (425) */
  interface PalletSepoliaFinalityVerifierBeaconBlockHeader extends Struct {
    readonly slot: u64;
    readonly proposerIndex: u64;
    readonly parentRoot: U8aFixed;
    readonly stateRoot: U8aFixed;
    readonly bodyRoot: U8aFixed;
  }

  /** @name PalletSepoliaFinalityVerifierMerkleProof (426) */
  interface PalletSepoliaFinalityVerifierMerkleProof extends Struct {
    readonly gIndex: u64;
    readonly witness: Vec<U8aFixed>;
  }

  /** @name PalletSepoliaFinalityVerifierExecutionPayload (427) */
  interface PalletSepoliaFinalityVerifierExecutionPayload extends Struct {
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

  /** @name PalletSepoliaFinalityVerifierExecutionHeader (429) */
  interface PalletSepoliaFinalityVerifierExecutionHeader extends Struct {
    readonly parentHash: U8aFixed;
    readonly ommersHash: U8aFixed;
    readonly beneficiary: H160;
    readonly stateRoot: U8aFixed;
    readonly transactionsRoot: U8aFixed;
    readonly receiptsRoot: U8aFixed;
    readonly logsBloom: EthbloomBloom;
    readonly difficulty: U256;
    readonly number: u64;
    readonly gasLimit: u64;
    readonly gasUsed: u64;
    readonly timestamp: u64;
    readonly extraData: Bytes;
    readonly mixHash: U8aFixed;
    readonly nonce: u64;
    readonly baseFeePerGas: u64;
    readonly withdrawalsRoot: U8aFixed;
  }

  /** @name PalletSepoliaFinalityVerifierSyncCommittee (430) */
  interface PalletSepoliaFinalityVerifierSyncCommittee extends Struct {
    readonly pubs: Vec<U8aFixed>;
    readonly aggr: U8aFixed;
  }

  /** @name PalletSepoliaFinalityVerifierEthereumReceiptInclusionProof (431) */
  interface PalletSepoliaFinalityVerifierEthereumReceiptInclusionProof extends Struct {
    readonly blockNumber: u64;
    readonly witness: Vec<Bytes>;
    readonly index: Bytes;
  }

  /** @name PalletSepoliaFinalityVerifierEthereumEventInclusionProof (432) */
  interface PalletSepoliaFinalityVerifierEthereumEventInclusionProof extends Struct {
    readonly blockNumber: u64;
    readonly witness: Vec<Bytes>;
    readonly index: Bytes;
    readonly event: Bytes;
  }

  /** @name PalletMaintenanceModeCall (433) */
  interface PalletMaintenanceModeCall extends Enum {
    readonly isEnterMaintenanceMode: boolean;
    readonly isResumeNormalOperation: boolean;
    readonly type: 'EnterMaintenanceMode' | 'ResumeNormalOperation';
  }

  /** @name PalletSudoCall (434) */
  interface PalletSudoCall extends Enum {
    readonly isSudo: boolean;
    readonly asSudo: {
      readonly call: Call;
    } & Struct;
    readonly isSudoUncheckedWeight: boolean;
    readonly asSudoUncheckedWeight: {
      readonly call: Call;
      readonly weight: SpWeightsWeightV2Weight;
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

  /** @name PalletSchedulerError (437) */
  interface PalletSchedulerError extends Enum {
    readonly isFailedToSchedule: boolean;
    readonly isNotFound: boolean;
    readonly isTargetBlockNumberInPast: boolean;
    readonly isRescheduleNoChange: boolean;
    readonly isNamed: boolean;
    readonly type: 'FailedToSchedule' | 'NotFound' | 'TargetBlockNumberInPast' | 'RescheduleNoChange' | 'Named';
  }

  /** @name PalletUtilityError (438) */
  interface PalletUtilityError extends Enum {
    readonly isTooManyCalls: boolean;
    readonly type: 'TooManyCalls';
  }

  /** @name PalletIdentityRegistration (439) */
  interface PalletIdentityRegistration extends Struct {
    readonly judgements: Vec<ITuple<[u32, PalletIdentityJudgement]>>;
    readonly deposit: u128;
    readonly info: PalletIdentityIdentityInfo;
  }

  /** @name PalletIdentityRegistrarInfo (447) */
  interface PalletIdentityRegistrarInfo extends Struct {
    readonly account: AccountId32;
    readonly fee: u128;
    readonly fields: PalletIdentityBitFlags;
  }

  /** @name PalletIdentityError (449) */
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
    readonly isJudgementForDifferentIdentity: boolean;
    readonly isJudgementPaymentFailed: boolean;
    readonly type: 'TooManySubAccounts' | 'NotFound' | 'NotNamed' | 'EmptyIndex' | 'FeeChanged' | 'NoIdentity' | 'StickyJudgement' | 'JudgementGiven' | 'InvalidJudgement' | 'InvalidIndex' | 'InvalidTarget' | 'TooManyFields' | 'TooManyRegistrars' | 'AlreadyClaimed' | 'NotSub' | 'NotOwned' | 'JudgementForDifferentIdentity' | 'JudgementPaymentFailed';
  }

  /** @name PalletBalancesBalanceLock (452) */
  interface PalletBalancesBalanceLock extends Struct {
    readonly id: U8aFixed;
    readonly amount: u128;
    readonly reasons: PalletBalancesReasons;
  }

  /** @name PalletBalancesReasons (453) */
  interface PalletBalancesReasons extends Enum {
    readonly isFee: boolean;
    readonly isMisc: boolean;
    readonly isAll: boolean;
    readonly type: 'Fee' | 'Misc' | 'All';
  }

  /** @name PalletBalancesReserveData (456) */
  interface PalletBalancesReserveData extends Struct {
    readonly id: U8aFixed;
    readonly amount: u128;
  }

  /** @name T0rnParachainRuntimeRuntimeHoldReason (460) */
  type T0rnParachainRuntimeRuntimeHoldReason = Null;

  /** @name PalletBalancesIdAmount (463) */
  interface PalletBalancesIdAmount extends Struct {
    readonly id: Null;
    readonly amount: u128;
  }

  /** @name PalletBalancesError (465) */
  interface PalletBalancesError extends Enum {
    readonly isVestingBalance: boolean;
    readonly isLiquidityRestrictions: boolean;
    readonly isInsufficientBalance: boolean;
    readonly isExistentialDeposit: boolean;
    readonly isExpendability: boolean;
    readonly isExistingVestingSchedule: boolean;
    readonly isDeadAccount: boolean;
    readonly isTooManyReserves: boolean;
    readonly isTooManyHolds: boolean;
    readonly isTooManyFreezes: boolean;
    readonly type: 'VestingBalance' | 'LiquidityRestrictions' | 'InsufficientBalance' | 'ExistentialDeposit' | 'Expendability' | 'ExistingVestingSchedule' | 'DeadAccount' | 'TooManyReserves' | 'TooManyHolds' | 'TooManyFreezes';
  }

  /** @name PalletTransactionPaymentReleases (467) */
  interface PalletTransactionPaymentReleases extends Enum {
    readonly isV1Ancient: boolean;
    readonly isV2: boolean;
    readonly type: 'V1Ancient' | 'V2';
  }

  /** @name PalletAssetsAssetDetails (468) */
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
    readonly status: PalletAssetsAssetStatus;
  }

  /** @name PalletAssetsAssetStatus (469) */
  interface PalletAssetsAssetStatus extends Enum {
    readonly isLive: boolean;
    readonly isFrozen: boolean;
    readonly isDestroying: boolean;
    readonly type: 'Live' | 'Frozen' | 'Destroying';
  }

  /** @name PalletAssetsAssetAccount (471) */
  interface PalletAssetsAssetAccount extends Struct {
    readonly balance: u128;
    readonly status: PalletAssetsAccountStatus;
    readonly reason: PalletAssetsExistenceReason;
    readonly extra: Null;
  }

  /** @name PalletAssetsAccountStatus (472) */
  interface PalletAssetsAccountStatus extends Enum {
    readonly isLiquid: boolean;
    readonly isFrozen: boolean;
    readonly isBlocked: boolean;
    readonly type: 'Liquid' | 'Frozen' | 'Blocked';
  }

  /** @name PalletAssetsExistenceReason (473) */
  interface PalletAssetsExistenceReason extends Enum {
    readonly isConsumer: boolean;
    readonly isSufficient: boolean;
    readonly isDepositHeld: boolean;
    readonly asDepositHeld: u128;
    readonly isDepositRefunded: boolean;
    readonly isDepositFrom: boolean;
    readonly asDepositFrom: ITuple<[AccountId32, u128]>;
    readonly type: 'Consumer' | 'Sufficient' | 'DepositHeld' | 'DepositRefunded' | 'DepositFrom';
  }

  /** @name PalletAssetsApproval (475) */
  interface PalletAssetsApproval extends Struct {
    readonly amount: u128;
    readonly deposit: u128;
  }

  /** @name PalletAssetsAssetMetadata (476) */
  interface PalletAssetsAssetMetadata extends Struct {
    readonly deposit: u128;
    readonly name: Bytes;
    readonly symbol: Bytes;
    readonly decimals: u8;
    readonly isFrozen: bool;
  }

  /** @name PalletAssetsError (478) */
  interface PalletAssetsError extends Enum {
    readonly isBalanceLow: boolean;
    readonly isNoAccount: boolean;
    readonly isNoPermission: boolean;
    readonly isUnknown: boolean;
    readonly isFrozen: boolean;
    readonly isInUse: boolean;
    readonly isBadWitness: boolean;
    readonly isMinBalanceZero: boolean;
    readonly isUnavailableConsumer: boolean;
    readonly isBadMetadata: boolean;
    readonly isUnapproved: boolean;
    readonly isWouldDie: boolean;
    readonly isAlreadyExists: boolean;
    readonly isNoDeposit: boolean;
    readonly isWouldBurn: boolean;
    readonly isLiveAsset: boolean;
    readonly isAssetNotLive: boolean;
    readonly isIncorrectStatus: boolean;
    readonly isNotFrozen: boolean;
    readonly isCallbackFailed: boolean;
    readonly type: 'BalanceLow' | 'NoAccount' | 'NoPermission' | 'Unknown' | 'Frozen' | 'InUse' | 'BadWitness' | 'MinBalanceZero' | 'UnavailableConsumer' | 'BadMetadata' | 'Unapproved' | 'WouldDie' | 'AlreadyExists' | 'NoDeposit' | 'WouldBurn' | 'LiveAsset' | 'AssetNotLive' | 'IncorrectStatus' | 'NotFrozen' | 'CallbackFailed';
  }

  /** @name T3rnPrimitivesAccountManagerRequestCharge (479) */
  interface T3rnPrimitivesAccountManagerRequestCharge extends Struct {
    readonly payee: AccountId32;
    readonly offeredReward: u128;
    readonly maybeAssetId: Option<u32>;
    readonly chargeFee: u128;
    readonly recipient: Option<AccountId32>;
    readonly source: T3rnPrimitivesClaimableBenefitSource;
    readonly role: T3rnPrimitivesClaimableCircuitRole;
  }

  /** @name T3rnPrimitivesCommonRoundInfo (481) */
  interface T3rnPrimitivesCommonRoundInfo extends Struct {
    readonly index: u32;
    readonly head: u32;
    readonly term: u32;
  }

  /** @name T3rnPrimitivesAccountManagerSettlement (482) */
  interface T3rnPrimitivesAccountManagerSettlement extends Struct {
    readonly requester: AccountId32;
    readonly recipient: AccountId32;
    readonly settlementAmount: u128;
    readonly maybeAssetId: Option<u32>;
    readonly outcome: T3rnPrimitivesAccountManagerOutcome;
    readonly source: T3rnPrimitivesClaimableBenefitSource;
    readonly role: T3rnPrimitivesClaimableCircuitRole;
  }

  /** @name PalletAccountManagerError (483) */
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

  /** @name PalletTreasuryProposal (484) */
  interface PalletTreasuryProposal extends Struct {
    readonly proposer: AccountId32;
    readonly value: u128;
    readonly beneficiary: AccountId32;
    readonly bond: u128;
  }

  /** @name FrameSupportPalletId (487) */
  interface FrameSupportPalletId extends U8aFixed {}

  /** @name PalletTreasuryError (488) */
  interface PalletTreasuryError extends Enum {
    readonly isInsufficientProposersBalance: boolean;
    readonly isInvalidIndex: boolean;
    readonly isTooManyApprovals: boolean;
    readonly isInsufficientPermission: boolean;
    readonly isProposalNotApproved: boolean;
    readonly type: 'InsufficientProposersBalance' | 'InvalidIndex' | 'TooManyApprovals' | 'InsufficientPermission' | 'ProposalNotApproved';
  }

  /** @name PalletCollatorSelectionCandidateInfo (495) */
  interface PalletCollatorSelectionCandidateInfo extends Struct {
    readonly who: AccountId32;
    readonly deposit: u128;
  }

  /** @name PalletCollatorSelectionError (497) */
  interface PalletCollatorSelectionError extends Enum {
    readonly isTooManyCandidates: boolean;
    readonly isTooFewEligibleCollators: boolean;
    readonly isAlreadyCandidate: boolean;
    readonly isNotCandidate: boolean;
    readonly isTooManyInvulnerables: boolean;
    readonly isAlreadyInvulnerable: boolean;
    readonly isNotInvulnerable: boolean;
    readonly isNoAssociatedValidatorId: boolean;
    readonly isValidatorNotRegistered: boolean;
    readonly type: 'TooManyCandidates' | 'TooFewEligibleCollators' | 'AlreadyCandidate' | 'NotCandidate' | 'TooManyInvulnerables' | 'AlreadyInvulnerable' | 'NotInvulnerable' | 'NoAssociatedValidatorId' | 'ValidatorNotRegistered';
  }

  /** @name SpCoreCryptoKeyTypeId (501) */
  interface SpCoreCryptoKeyTypeId extends U8aFixed {}

  /** @name PalletSessionError (502) */
  interface PalletSessionError extends Enum {
    readonly isInvalidProof: boolean;
    readonly isNoAssociatedValidatorId: boolean;
    readonly isDuplicatedKey: boolean;
    readonly isNoKeys: boolean;
    readonly isNoAccount: boolean;
    readonly type: 'InvalidProof' | 'NoAssociatedValidatorId' | 'DuplicatedKey' | 'NoKeys' | 'NoAccount';
  }

  /** @name CumulusPalletXcmpQueueInboundChannelDetails (507) */
  interface CumulusPalletXcmpQueueInboundChannelDetails extends Struct {
    readonly sender: u32;
    readonly state: CumulusPalletXcmpQueueInboundState;
    readonly messageMetadata: Vec<ITuple<[u32, PolkadotParachainPrimitivesXcmpMessageFormat]>>;
  }

  /** @name CumulusPalletXcmpQueueInboundState (508) */
  interface CumulusPalletXcmpQueueInboundState extends Enum {
    readonly isOk: boolean;
    readonly isSuspended: boolean;
    readonly type: 'Ok' | 'Suspended';
  }

  /** @name PolkadotParachainPrimitivesXcmpMessageFormat (511) */
  interface PolkadotParachainPrimitivesXcmpMessageFormat extends Enum {
    readonly isConcatenatedVersionedXcm: boolean;
    readonly isConcatenatedEncodedBlob: boolean;
    readonly isSignals: boolean;
    readonly type: 'ConcatenatedVersionedXcm' | 'ConcatenatedEncodedBlob' | 'Signals';
  }

  /** @name CumulusPalletXcmpQueueOutboundChannelDetails (514) */
  interface CumulusPalletXcmpQueueOutboundChannelDetails extends Struct {
    readonly recipient: u32;
    readonly state: CumulusPalletXcmpQueueOutboundState;
    readonly signalsExist: bool;
    readonly firstIndex: u16;
    readonly lastIndex: u16;
  }

  /** @name CumulusPalletXcmpQueueOutboundState (515) */
  interface CumulusPalletXcmpQueueOutboundState extends Enum {
    readonly isOk: boolean;
    readonly isSuspended: boolean;
    readonly type: 'Ok' | 'Suspended';
  }

  /** @name CumulusPalletXcmpQueueQueueConfigData (517) */
  interface CumulusPalletXcmpQueueQueueConfigData extends Struct {
    readonly suspendThreshold: u32;
    readonly dropThreshold: u32;
    readonly resumeThreshold: u32;
    readonly thresholdWeight: SpWeightsWeightV2Weight;
    readonly weightRestrictDecay: SpWeightsWeightV2Weight;
    readonly xcmpMaxIndividualWeight: SpWeightsWeightV2Weight;
  }

  /** @name CumulusPalletXcmpQueueError (519) */
  interface CumulusPalletXcmpQueueError extends Enum {
    readonly isFailedToSend: boolean;
    readonly isBadXcmOrigin: boolean;
    readonly isBadXcm: boolean;
    readonly isBadOverweightIndex: boolean;
    readonly isWeightOverLimit: boolean;
    readonly type: 'FailedToSend' | 'BadXcmOrigin' | 'BadXcm' | 'BadOverweightIndex' | 'WeightOverLimit';
  }

  /** @name PalletXcmQueryStatus (520) */
  interface PalletXcmQueryStatus extends Enum {
    readonly isPending: boolean;
    readonly asPending: {
      readonly responder: XcmVersionedMultiLocation;
      readonly maybeMatchQuerier: Option<XcmVersionedMultiLocation>;
      readonly maybeNotify: Option<ITuple<[u8, u8]>>;
      readonly timeout: u32;
    } & Struct;
    readonly isVersionNotifier: boolean;
    readonly asVersionNotifier: {
      readonly origin: XcmVersionedMultiLocation;
      readonly isActive: bool;
    } & Struct;
    readonly isReady: boolean;
    readonly asReady: {
      readonly response: XcmVersionedResponse;
      readonly at: u32;
    } & Struct;
    readonly type: 'Pending' | 'VersionNotifier' | 'Ready';
  }

  /** @name XcmVersionedResponse (524) */
  interface XcmVersionedResponse extends Enum {
    readonly isV2: boolean;
    readonly asV2: XcmV2Response;
    readonly isV3: boolean;
    readonly asV3: XcmV3Response;
    readonly type: 'V2' | 'V3';
  }

  /** @name PalletXcmVersionMigrationStage (530) */
  interface PalletXcmVersionMigrationStage extends Enum {
    readonly isMigrateSupportedVersion: boolean;
    readonly isMigrateVersionNotifiers: boolean;
    readonly isNotifyCurrentTargets: boolean;
    readonly asNotifyCurrentTargets: Option<Bytes>;
    readonly isMigrateAndNotifyOldTargets: boolean;
    readonly type: 'MigrateSupportedVersion' | 'MigrateVersionNotifiers' | 'NotifyCurrentTargets' | 'MigrateAndNotifyOldTargets';
  }

  /** @name XcmVersionedAssetId (532) */
  interface XcmVersionedAssetId extends Enum {
    readonly isV3: boolean;
    readonly asV3: XcmV3MultiassetAssetId;
    readonly type: 'V3';
  }

  /** @name PalletXcmRemoteLockedFungibleRecord (533) */
  interface PalletXcmRemoteLockedFungibleRecord extends Struct {
    readonly amount: u128;
    readonly owner: XcmVersionedMultiLocation;
    readonly locker: XcmVersionedMultiLocation;
    readonly consumers: Vec<ITuple<[Null, u128]>>;
  }

  /** @name PalletXcmError (540) */
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
    readonly isInvalidAsset: boolean;
    readonly isLowBalance: boolean;
    readonly isTooManyLocks: boolean;
    readonly isAccountNotSovereign: boolean;
    readonly isFeesNotMet: boolean;
    readonly isLockNotFound: boolean;
    readonly isInUse: boolean;
    readonly type: 'Unreachable' | 'SendFailure' | 'Filtered' | 'UnweighableMessage' | 'DestinationNotInvertible' | 'Empty' | 'CannotReanchor' | 'TooManyAssets' | 'InvalidOrigin' | 'BadVersion' | 'BadLocation' | 'NoSubscription' | 'AlreadySubscribed' | 'InvalidAsset' | 'LowBalance' | 'TooManyLocks' | 'AccountNotSovereign' | 'FeesNotMet' | 'LockNotFound' | 'InUse';
  }

  /** @name CumulusPalletXcmError (541) */
  type CumulusPalletXcmError = Null;

  /** @name CumulusPalletDmpQueueConfigData (542) */
  interface CumulusPalletDmpQueueConfigData extends Struct {
    readonly maxIndividual: SpWeightsWeightV2Weight;
  }

  /** @name CumulusPalletDmpQueuePageIndexData (543) */
  interface CumulusPalletDmpQueuePageIndexData extends Struct {
    readonly beginUsed: u32;
    readonly endUsed: u32;
    readonly overweightCount: u64;
  }

  /** @name CumulusPalletDmpQueueError (546) */
  interface CumulusPalletDmpQueueError extends Enum {
    readonly isUnknown: boolean;
    readonly isOverLimit: boolean;
    readonly type: 'Unknown' | 'OverLimit';
  }

  /** @name PalletAssetRegistryError (547) */
  interface PalletAssetRegistryError extends Enum {
    readonly isNotFound: boolean;
    readonly isLocationUnallowed: boolean;
    readonly isCapabilitiesNotPermitted: boolean;
    readonly isShouldntExecuteMessage: boolean;
    readonly type: 'NotFound' | 'LocationUnallowed' | 'CapabilitiesNotPermitted' | 'ShouldntExecuteMessage';
  }

  /** @name T3rnAbiSfxAbi (548) */
  interface T3rnAbiSfxAbi extends Struct {
    readonly argsNames: Vec<ITuple<[Bytes, bool]>>;
    readonly maybePrefixMemo: Option<u8>;
    readonly egressAbiDescriptors: T3rnAbiSfxAbiPerCodecAbiDescriptors;
    readonly ingressAbiDescriptors: T3rnAbiSfxAbiPerCodecAbiDescriptors;
  }

  /** @name T3rnAbiSfxAbiPerCodecAbiDescriptors (551) */
  interface T3rnAbiSfxAbiPerCodecAbiDescriptors extends Struct {
    readonly forRlp: Bytes;
    readonly forScale: Bytes;
  }

  /** @name T3rnPrimitivesXdnsGatewayRecord (553) */
  interface T3rnPrimitivesXdnsGatewayRecord extends Struct {
    readonly gatewayId: U8aFixed;
    readonly verificationVendor: T3rnPrimitivesGatewayVendor;
    readonly executionVendor: T3rnPrimitivesExecutionVendor;
    readonly codec: T3rnAbiRecodeCodec;
    readonly registrant: Option<AccountId32>;
    readonly escrowAccount: Option<AccountId32>;
    readonly allowedSideEffects: Vec<ITuple<[U8aFixed, Option<u8>]>>;
  }

  /** @name T3rnPrimitivesXdnsTokenRecord (555) */
  interface T3rnPrimitivesXdnsTokenRecord extends Struct {
    readonly tokenId: u32;
    readonly gatewayId: U8aFixed;
    readonly tokenProps: T3rnPrimitivesTokenInfo;
  }

  /** @name T3rnPrimitivesGatewayActivity (558) */
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

  /** @name T3rnPrimitivesFinalityVerifierActivity (561) */
  interface T3rnPrimitivesFinalityVerifierActivity extends Struct {
    readonly verifier: T3rnPrimitivesGatewayVendor;
    readonly reportedAt: u32;
    readonly justifiedHeight: u32;
    readonly finalizedHeight: u32;
    readonly updatedHeight: u32;
    readonly epoch: u32;
    readonly isActive: bool;
  }

  /** @name T3rnPrimitivesXdnsEpochEstimate (563) */
  interface T3rnPrimitivesXdnsEpochEstimate extends Struct {
    readonly local: u32;
    readonly remote: u32;
    readonly movingAverageLocal: u32;
    readonly movingAverageRemote: u32;
  }

  /** @name PalletXdnsError (564) */
  interface PalletXdnsError extends Enum {
    readonly isGatewayRecordAlreadyExists: boolean;
    readonly isXdnsRecordNotFound: boolean;
    readonly isEscrowAccountNotFound: boolean;
    readonly isTokenRecordAlreadyExists: boolean;
    readonly isTokenRecordNotFoundInAssetsOverlay: boolean;
    readonly isGatewayRecordNotFound: boolean;
    readonly isSideEffectABIAlreadyExists: boolean;
    readonly isSideEffectABINotFound: boolean;
    readonly isNoParachainInfoFound: boolean;
    readonly isTokenExecutionVendorMismatch: boolean;
    readonly isGatewayNotActive: boolean;
    readonly type: 'GatewayRecordAlreadyExists' | 'XdnsRecordNotFound' | 'EscrowAccountNotFound' | 'TokenRecordAlreadyExists' | 'TokenRecordNotFoundInAssetsOverlay' | 'GatewayRecordNotFound' | 'SideEffectABIAlreadyExists' | 'SideEffectABINotFound' | 'NoParachainInfoFound' | 'TokenExecutionVendorMismatch' | 'GatewayNotActive';
  }

  /** @name T3rnPrimitivesAttestersAttesterInfo (565) */
  interface T3rnPrimitivesAttestersAttesterInfo extends Struct {
    readonly keyEd: U8aFixed;
    readonly keyEc: U8aFixed;
    readonly keySr: U8aFixed;
    readonly commission: Percent;
    readonly index: u32;
  }

  /** @name PalletAttestersError (572) */
  interface PalletAttestersError extends Enum {
    readonly isAttesterNotFound: boolean;
    readonly isArithmeticOverflow: boolean;
    readonly isInvalidSignature: boolean;
    readonly isInvalidMessage: boolean;
    readonly isInvalidTargetInclusionProof: boolean;
    readonly isUnexpectedBatchHashRecoveredFromCommitment: boolean;
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
    readonly type: 'AttesterNotFound' | 'ArithmeticOverflow' | 'InvalidSignature' | 'InvalidMessage' | 'InvalidTargetInclusionProof' | 'UnexpectedBatchHashRecoveredFromCommitment' | 'AlreadyRegistered' | 'PublicKeyMissing' | 'AttestationSignatureInvalid' | 'AttestationDoubleSignAttempt' | 'NotActiveSet' | 'NotInCurrentCommittee' | 'AttesterDidNotAgreeToNewTarget' | 'NotRegistered' | 'NoNominationFound' | 'AlreadyNominated' | 'NominatorNotEnoughBalance' | 'NominatorBondTooSmall' | 'AttesterBondTooSmall' | 'MissingNominations' | 'BatchHashMismatch' | 'BatchNotFound' | 'CollusionWithPermanentSlashDetected' | 'BatchFoundWithUnsignableStatus' | 'RejectingFromSlashedAttester' | 'TargetAlreadyActive' | 'TargetNotActive' | 'XdnsTargetNotActive' | 'XdnsGatewayDoesNotHaveEscrowAddressRegistered' | 'SfxAlreadyRequested' | 'AddAttesterAlreadyRequested' | 'RemoveAttesterAlreadyRequested' | 'NextCommitteeAlreadyRequested' | 'BanAttesterAlreadyRequested' | 'BatchAlreadyCommitted' | 'CommitteeSizeTooLarge';
  }

  /** @name PalletRewardsAssetType (577) */
  interface PalletRewardsAssetType extends Enum {
    readonly isNative: boolean;
    readonly isNonNative: boolean;
    readonly asNonNative: u32;
    readonly type: 'Native' | 'NonNative';
  }

  /** @name PalletRewardsTreasuryBalanceSheet (578) */
  interface PalletRewardsTreasuryBalanceSheet extends Struct {
    readonly treasury: u128;
    readonly escrow: u128;
    readonly fee: u128;
    readonly slash: u128;
    readonly parachain: u128;
  }

  /** @name PalletRewardsDistributionRecord (580) */
  interface PalletRewardsDistributionRecord extends Struct {
    readonly blockNumber: u32;
    readonly attesterRewards: u128;
    readonly collatorRewards: u128;
    readonly executorRewards: u128;
    readonly treasuryRewards: u128;
    readonly available: u128;
    readonly distributed: u128;
  }

  /** @name T3rnPrimitivesClaimableClaimableArtifacts (582) */
  interface T3rnPrimitivesClaimableClaimableArtifacts extends Struct {
    readonly beneficiary: AccountId32;
    readonly role: T3rnPrimitivesClaimableCircuitRole;
    readonly totalRoundClaim: u128;
    readonly nonNativeAssetId: Option<u32>;
    readonly benefitSource: T3rnPrimitivesClaimableBenefitSource;
  }

  /** @name PalletRewardsError (584) */
  interface PalletRewardsError extends Enum {
    readonly isDistributionPeriodNotElapsed: boolean;
    readonly isNoPendingClaims: boolean;
    readonly isArithmeticOverflow: boolean;
    readonly isAttesterNotFound: boolean;
    readonly isTryIntoConversionU128ToBalanceFailed: boolean;
    readonly isHalted: boolean;
    readonly type: 'DistributionPeriodNotElapsed' | 'NoPendingClaims' | 'ArithmeticOverflow' | 'AttesterNotFound' | 'TryIntoConversionU128ToBalanceFailed' | 'Halted';
  }

  /** @name PalletContractsRegistryError (585) */
  interface PalletContractsRegistryError extends Enum {
    readonly isContractAlreadyExists: boolean;
    readonly isUnknownContract: boolean;
    readonly type: 'ContractAlreadyExists' | 'UnknownContract';
  }

  /** @name T3rnPrimitivesCircuitTypesXExecSignal (586) */
  interface T3rnPrimitivesCircuitTypesXExecSignal extends Struct {
    readonly requester: AccountId32;
    readonly requesterNonce: u32;
    readonly timeoutsAt: T3rnPrimitivesCircuitTypesAdaptiveTimeout;
    readonly speedMode: T3rnPrimitivesSpeedMode;
    readonly delayStepsAt: Option<Vec<u32>>;
    readonly status: T3rnPrimitivesCircuitTypesCircuitStatus;
    readonly stepsCnt: ITuple<[u32, u32]>;
  }

  /** @name T3rnPrimitivesVolatileLocalState (588) */
  interface T3rnPrimitivesVolatileLocalState extends Struct {
    readonly state: BTreeMap<U8aFixed, Bytes>;
  }

  /** @name T3rnSdkPrimitivesSignalExecutionSignal (595) */
  interface T3rnSdkPrimitivesSignalExecutionSignal extends Struct {
    readonly step: u32;
    readonly kind: T3rnSdkPrimitivesSignalSignalKind;
    readonly executionId: H256;
  }

  /** @name PalletCircuitError (597) */
  interface PalletCircuitError extends Enum {
    readonly isUpdateAttemptDoubleRevert: boolean;
    readonly isUpdateAttemptDoubleKill: boolean;
    readonly isUpdateStateTransitionDisallowed: boolean;
    readonly isUpdateForcedStateTransitionDisallowed: boolean;
    readonly isUpdateXtxTriggeredWithUnexpectedStatus: boolean;
    readonly isConfirmationFailed: boolean;
    readonly isInvalidOrderOrigin: boolean;
    readonly isApplyTriggeredWithUnexpectedStatus: boolean;
    readonly isBidderNotEnoughBalance: boolean;
    readonly isRequesterNotEnoughBalance: boolean;
    readonly isAssetsFailedToWithdraw: boolean;
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
    readonly isGatewayNotActive: boolean;
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
    readonly type: 'UpdateAttemptDoubleRevert' | 'UpdateAttemptDoubleKill' | 'UpdateStateTransitionDisallowed' | 'UpdateForcedStateTransitionDisallowed' | 'UpdateXtxTriggeredWithUnexpectedStatus' | 'ConfirmationFailed' | 'InvalidOrderOrigin' | 'ApplyTriggeredWithUnexpectedStatus' | 'BidderNotEnoughBalance' | 'RequesterNotEnoughBalance' | 'AssetsFailedToWithdraw' | 'SanityAfterCreatingSFXDepositsFailed' | 'ContractXtxKilledRunOutOfFunds' | 'ChargingTransferFailed' | 'ChargingTransferFailedAtPendingExecution' | 'XtxChargeFailedRequesterBalanceTooLow' | 'XtxChargeBondDepositFailedCantAccessBid' | 'FinalizeSquareUpFailed' | 'CriticalStateSquareUpCalledToFinishWithoutFsxConfirmed' | 'RewardTransferFailed' | 'RefundTransferFailed' | 'SideEffectsValidationFailed' | 'InsuranceBondNotRequired' | 'BiddingInactive' | 'BiddingRejectedBidBelowDust' | 'BiddingRejectedBidTooHigh' | 'BiddingRejectedInsuranceTooLow' | 'BiddingRejectedBetterBidFound' | 'BiddingRejectedFailedToDepositBidderBond' | 'BiddingFailedExecutorsBalanceTooLowToReserve' | 'InsuranceBondAlreadyDeposited' | 'InvalidFTXStateEmptyBidForReadyXtx' | 'InvalidFTXStateEmptyConfirmationForFinishedXtx' | 'InvalidFTXStateUnassignedExecutorForReadySFX' | 'InvalidFTXStateIncorrectExecutorForReadySFX' | 'GatewayNotActive' | 'SetupFailed' | 'SetupFailedXtxNotFound' | 'SetupFailedXtxStorageArtifactsNotFound' | 'SetupFailedIncorrectXtxStatus' | 'SetupFailedDuplicatedXtx' | 'SetupFailedEmptyXtx' | 'SetupFailedXtxAlreadyFinished' | 'SetupFailedXtxWasDroppedAtBidding' | 'SetupFailedXtxReverted' | 'SetupFailedXtxRevertedTimeout' | 'XtxDoesNotExist' | 'InvalidFSXBidStateLocated' | 'EnactSideEffectsCanOnlyBeCalledWithMin1StepFinished' | 'FatalXtxTimeoutXtxIdNotMatched' | 'RelayEscrowedFailedNothingToConfirm' | 'FatalCommitSideEffectWithoutConfirmationAttempt' | 'FatalErroredCommitSideEffectConfirmationAttempt' | 'FatalErroredRevertSideEffectConfirmationAttempt' | 'FailedToHardenFullSideEffect' | 'ApplyFailed' | 'DeterminedForbiddenXtxStatus' | 'SideEffectIsAlreadyScheduledToExecuteOverXBI' | 'FsxNotFoundById' | 'XtxNotFound' | 'LocalSideEffectExecutionNotApplicable' | 'LocalExecutionUnauthorized' | 'OnLocalTriggerFailedToSetupXtx' | 'UnauthorizedCancellation' | 'FailedToConvertSFX2XBI' | 'FailedToCheckInOverXBI' | 'FailedToCreateXBIMetadataDueToWrongAccountConversion' | 'FailedToConvertXBIResult2SFXConfirmation' | 'FailedToEnterXBIPortal' | 'FailedToExitXBIPortal' | 'FailedToCommitFSX' | 'XbiExitFailedOnSFXConfirmation' | 'UnsupportedRole' | 'InvalidLocalTrigger' | 'SignalQueueFull' | 'ArithmeticErrorOverflow' | 'ArithmeticErrorUnderflow' | 'ArithmeticErrorDivisionByZero';
  }

  /** @name PalletClockError (598) */
  type PalletClockError = Null;

  /** @name PalletCircuitVacuumError (599) */
  type PalletCircuitVacuumError = Null;

  /** @name Pallet3vmError (600) */
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

  /** @name PalletContractsWasmCodeInfo (602) */
  interface PalletContractsWasmCodeInfo extends Struct {
    readonly owner: AccountId32;
    readonly deposit: Compact<u128>;
    readonly refcount: Compact<u64>;
    readonly determinism: PalletContractsWasmDeterminism;
    readonly codeLen: u32;
  }

  /** @name PalletContractsStorageContractInfo (603) */
  interface PalletContractsStorageContractInfo extends Struct {
    readonly trieId: Bytes;
    readonly depositAccount: AccountId32;
    readonly codeHash: H256;
    readonly storageBytes: u32;
    readonly storageItems: u32;
    readonly storageByteDeposit: u128;
    readonly storageItemDeposit: u128;
    readonly storageBaseDeposit: u128;
  }

  /** @name PalletContractsStorageDeletionQueueManager (605) */
  interface PalletContractsStorageDeletionQueueManager extends Struct {
    readonly insertCounter: u32;
    readonly deleteCounter: u32;
  }

  /** @name PalletContractsSchedule (607) */
  interface PalletContractsSchedule extends Struct {
    readonly limits: PalletContractsScheduleLimits;
    readonly instructionWeights: PalletContractsScheduleInstructionWeights;
    readonly hostFnWeights: PalletContractsScheduleHostFnWeights;
  }

  /** @name PalletContractsScheduleLimits (608) */
  interface PalletContractsScheduleLimits extends Struct {
    readonly eventTopics: u32;
    readonly globals: u32;
    readonly locals: u32;
    readonly parameters: u32;
    readonly memoryPages: u32;
    readonly tableSize: u32;
    readonly brTableSize: u32;
    readonly subjectLen: u32;
    readonly payloadLen: u32;
    readonly runtimeMemory: u32;
  }

  /** @name PalletContractsScheduleInstructionWeights (609) */
  interface PalletContractsScheduleInstructionWeights extends Struct {
    readonly base: u32;
  }

  /** @name PalletContractsScheduleHostFnWeights (610) */
  interface PalletContractsScheduleHostFnWeights extends Struct {
    readonly caller: SpWeightsWeightV2Weight;
    readonly isContract: SpWeightsWeightV2Weight;
    readonly codeHash: SpWeightsWeightV2Weight;
    readonly ownCodeHash: SpWeightsWeightV2Weight;
    readonly callerIsOrigin: SpWeightsWeightV2Weight;
    readonly callerIsRoot: SpWeightsWeightV2Weight;
    readonly address: SpWeightsWeightV2Weight;
    readonly gasLeft: SpWeightsWeightV2Weight;
    readonly balance: SpWeightsWeightV2Weight;
    readonly valueTransferred: SpWeightsWeightV2Weight;
    readonly minimumBalance: SpWeightsWeightV2Weight;
    readonly blockNumber: SpWeightsWeightV2Weight;
    readonly now: SpWeightsWeightV2Weight;
    readonly weightToFee: SpWeightsWeightV2Weight;
    readonly input: SpWeightsWeightV2Weight;
    readonly inputPerByte: SpWeightsWeightV2Weight;
    readonly r_return: SpWeightsWeightV2Weight;
    readonly returnPerByte: SpWeightsWeightV2Weight;
    readonly terminate: SpWeightsWeightV2Weight;
    readonly random: SpWeightsWeightV2Weight;
    readonly depositEvent: SpWeightsWeightV2Weight;
    readonly depositEventPerTopic: SpWeightsWeightV2Weight;
    readonly depositEventPerByte: SpWeightsWeightV2Weight;
    readonly debugMessage: SpWeightsWeightV2Weight;
    readonly debugMessagePerByte: SpWeightsWeightV2Weight;
    readonly setStorage: SpWeightsWeightV2Weight;
    readonly setStoragePerNewByte: SpWeightsWeightV2Weight;
    readonly setStoragePerOldByte: SpWeightsWeightV2Weight;
    readonly setCodeHash: SpWeightsWeightV2Weight;
    readonly clearStorage: SpWeightsWeightV2Weight;
    readonly clearStoragePerByte: SpWeightsWeightV2Weight;
    readonly containsStorage: SpWeightsWeightV2Weight;
    readonly containsStoragePerByte: SpWeightsWeightV2Weight;
    readonly getStorage: SpWeightsWeightV2Weight;
    readonly getStoragePerByte: SpWeightsWeightV2Weight;
    readonly takeStorage: SpWeightsWeightV2Weight;
    readonly takeStoragePerByte: SpWeightsWeightV2Weight;
    readonly transfer: SpWeightsWeightV2Weight;
    readonly call: SpWeightsWeightV2Weight;
    readonly delegateCall: SpWeightsWeightV2Weight;
    readonly callTransferSurcharge: SpWeightsWeightV2Weight;
    readonly callPerClonedByte: SpWeightsWeightV2Weight;
    readonly instantiate: SpWeightsWeightV2Weight;
    readonly instantiateTransferSurcharge: SpWeightsWeightV2Weight;
    readonly instantiatePerInputByte: SpWeightsWeightV2Weight;
    readonly instantiatePerSaltByte: SpWeightsWeightV2Weight;
    readonly hashSha2256: SpWeightsWeightV2Weight;
    readonly hashSha2256PerByte: SpWeightsWeightV2Weight;
    readonly hashKeccak256: SpWeightsWeightV2Weight;
    readonly hashKeccak256PerByte: SpWeightsWeightV2Weight;
    readonly hashBlake2256: SpWeightsWeightV2Weight;
    readonly hashBlake2256PerByte: SpWeightsWeightV2Weight;
    readonly hashBlake2128: SpWeightsWeightV2Weight;
    readonly hashBlake2128PerByte: SpWeightsWeightV2Weight;
    readonly ecdsaRecover: SpWeightsWeightV2Weight;
    readonly ecdsaToEthAddress: SpWeightsWeightV2Weight;
    readonly sr25519Verify: SpWeightsWeightV2Weight;
    readonly sr25519VerifyPerByte: SpWeightsWeightV2Weight;
    readonly reentranceCount: SpWeightsWeightV2Weight;
    readonly accountReentranceCount: SpWeightsWeightV2Weight;
    readonly instantiationNonce: SpWeightsWeightV2Weight;
  }

  /** @name PalletContractsError (611) */
  interface PalletContractsError extends Enum {
    readonly isInvalidSchedule: boolean;
    readonly isInvalidCallFlags: boolean;
    readonly isOutOfGas: boolean;
    readonly isOutputBufferTooSmall: boolean;
    readonly isTransferFailed: boolean;
    readonly isMaxCallDepthReached: boolean;
    readonly isContractNotFound: boolean;
    readonly isCodeTooLarge: boolean;
    readonly isCodeNotFound: boolean;
    readonly isCodeInfoNotFound: boolean;
    readonly isOutOfBounds: boolean;
    readonly isDecodingFailed: boolean;
    readonly isContractTrapped: boolean;
    readonly isValueTooLarge: boolean;
    readonly isTerminatedWhileReentrant: boolean;
    readonly isInputForwarded: boolean;
    readonly isRandomSubjectTooLong: boolean;
    readonly isTooManyTopics: boolean;
    readonly isNoChainExtension: boolean;
    readonly isDuplicateContract: boolean;
    readonly isTerminatedInConstructor: boolean;
    readonly isReentranceDenied: boolean;
    readonly isStorageDepositNotEnoughFunds: boolean;
    readonly isStorageDepositLimitExhausted: boolean;
    readonly isCodeInUse: boolean;
    readonly isContractReverted: boolean;
    readonly isCodeRejected: boolean;
    readonly isIndeterministic: boolean;
    readonly isMigrationInProgress: boolean;
    readonly isNoMigrationPerformed: boolean;
    readonly type: 'InvalidSchedule' | 'InvalidCallFlags' | 'OutOfGas' | 'OutputBufferTooSmall' | 'TransferFailed' | 'MaxCallDepthReached' | 'ContractNotFound' | 'CodeTooLarge' | 'CodeNotFound' | 'CodeInfoNotFound' | 'OutOfBounds' | 'DecodingFailed' | 'ContractTrapped' | 'ValueTooLarge' | 'TerminatedWhileReentrant' | 'InputForwarded' | 'RandomSubjectTooLong' | 'TooManyTopics' | 'NoChainExtension' | 'DuplicateContract' | 'TerminatedInConstructor' | 'ReentranceDenied' | 'StorageDepositNotEnoughFunds' | 'StorageDepositLimitExhausted' | 'CodeInUse' | 'ContractReverted' | 'CodeRejected' | 'Indeterministic' | 'MigrationInProgress' | 'NoMigrationPerformed';
  }

  /** @name PalletEvmCodeMetadata (612) */
  interface PalletEvmCodeMetadata extends Struct {
    readonly size_: u64;
    readonly hash_: H256;
  }

  /** @name PalletEvmError (614) */
  interface PalletEvmError extends Enum {
    readonly isBalanceLow: boolean;
    readonly isFeeOverflow: boolean;
    readonly isPaymentOverflow: boolean;
    readonly isWithdrawFailed: boolean;
    readonly isGasPriceTooLow: boolean;
    readonly isInvalidNonce: boolean;
    readonly isGasLimitTooLow: boolean;
    readonly isGasLimitTooHigh: boolean;
    readonly isUndefined: boolean;
    readonly isReentrancy: boolean;
    readonly isTransactionMustComeFromEOA: boolean;
    readonly type: 'BalanceLow' | 'FeeOverflow' | 'PaymentOverflow' | 'WithdrawFailed' | 'GasPriceTooLow' | 'InvalidNonce' | 'GasLimitTooLow' | 'GasLimitTooHigh' | 'Undefined' | 'Reentrancy' | 'TransactionMustComeFromEOA';
  }

  /** @name PalletPortalError (615) */
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

  /** @name PalletGrandpaFinalityVerifierBridgesHeaderChainAuthoritySet (616) */
  interface PalletGrandpaFinalityVerifierBridgesHeaderChainAuthoritySet extends Struct {
    readonly authorities: Vec<ITuple<[SpConsensusGrandpaAppPublic, u64]>>;
    readonly setId: u64;
  }

  /** @name PalletGrandpaFinalityVerifierParachainRegistrationData (619) */
  interface PalletGrandpaFinalityVerifierParachainRegistrationData extends Struct {
    readonly relayGatewayId: U8aFixed;
    readonly id: u32;
  }

  /** @name PalletGrandpaFinalityVerifierError (620) */
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
    readonly isUnexpectedEventLength: boolean;
    readonly isUnexpectedSource: boolean;
    readonly isEventDecodingFailed: boolean;
    readonly isUnkownSideEffect: boolean;
    readonly isUnsupportedScheduledChange: boolean;
    readonly isHalted: boolean;
    readonly isBlockHeightConversionError: boolean;
    readonly isInvalidPayloadSource: boolean;
    readonly isInvalidSourceFormat: boolean;
    readonly type: 'EmptyRangeSubmitted' | 'RangeToLarge' | 'NoFinalizedHeader' | 'InvalidAuthoritySet' | 'InvalidGrandpaJustification' | 'InvalidRangeLinkage' | 'InvalidJustificationLinkage' | 'ParachainEntryNotFound' | 'StorageRootNotFound' | 'InclusionDataDecodeError' | 'InvalidStorageProof' | 'EventNotIncluded' | 'HeaderDecodingError' | 'HeaderDataDecodingError' | 'StorageRootMismatch' | 'UnknownHeader' | 'UnexpectedEventLength' | 'UnexpectedSource' | 'EventDecodingFailed' | 'UnkownSideEffect' | 'UnsupportedScheduledChange' | 'Halted' | 'BlockHeightConversionError' | 'InvalidPayloadSource' | 'InvalidSourceFormat';
  }

  /** @name PalletEth2FinalityVerifierCheckpoint (623) */
  interface PalletEth2FinalityVerifierCheckpoint extends Struct {
    readonly attestedBeacon: PalletEth2FinalityVerifierBeaconCheckpoint;
    readonly attestedExecution: PalletEth2FinalityVerifierExecutionCheckpoint;
    readonly justifiedBeacon: PalletEth2FinalityVerifierBeaconCheckpoint;
    readonly justifiedExecution: PalletEth2FinalityVerifierExecutionCheckpoint;
    readonly finalizedBeacon: PalletEth2FinalityVerifierBeaconCheckpoint;
    readonly finalizedExecution: PalletEth2FinalityVerifierExecutionCheckpoint;
  }

  /** @name PalletEth2FinalityVerifierBeaconCheckpoint (624) */
  interface PalletEth2FinalityVerifierBeaconCheckpoint extends Struct {
    readonly epoch: u64;
    readonly root: U8aFixed;
  }

  /** @name PalletEth2FinalityVerifierExecutionCheckpoint (625) */
  interface PalletEth2FinalityVerifierExecutionCheckpoint extends Struct {
    readonly height: u64;
    readonly root: U8aFixed;
  }

  /** @name PalletEth2FinalityVerifierError (626) */
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
    readonly isExecutionHeaderNotFinalized: boolean;
    readonly isInvalidBeaconLinkage: boolean;
    readonly isInvalidExecutionPayload: boolean;
    readonly isInvalidSourceAddress: boolean;
    readonly type: 'Halted' | 'AlreadyInitialized' | 'InvalidInitializationData' | 'SszForkDataHashTreeRootFailed' | 'SszSigningDataHashTreeRootFailed' | 'BlsPubkeyAggregationFaild' | 'InvalidBLSPublicKeyUsedForVerification' | 'InvalidInclusionProof' | 'ForkNotDetected' | 'ValidSyncCommitteeNotAvailable' | 'SubmittedHeaderToOld' | 'InvalidBLSSignature' | 'InvalidMerkleProof' | 'BeaconHeaderHashTreeRootFailed' | 'BeaconHeaderNotFound' | 'BeaconHeaderNotFinalized' | 'ExecutionHeaderHashTreeRootFailed' | 'InvalidExecutionRangeLinkage' | 'InvalidExecutionRange' | 'SyncCommitteeParticipantsNotSupermajority' | 'SyncCommitteeInvalid' | 'NotPeriodsFirstEpoch' | 'InvalidCheckpoint' | 'ExecutionHeaderNotFound' | 'EventNotInReceipt' | 'InvalidEncodedEpochUpdate' | 'InvalidSyncCommitteePeriod' | 'MathError' | 'CurrentSyncCommitteePeriodNotAvailable' | 'BeaconCheckpointHashTreeRootFailed' | 'InvalidFork' | 'ExecutionHeaderNotFinalized' | 'InvalidBeaconLinkage' | 'InvalidExecutionPayload' | 'InvalidSourceAddress';
  }

  /** @name PalletSepoliaFinalityVerifierCheckpoint (627) */
  interface PalletSepoliaFinalityVerifierCheckpoint extends Struct {
    readonly attestedBeacon: PalletSepoliaFinalityVerifierBeaconCheckpoint;
    readonly attestedExecution: PalletSepoliaFinalityVerifierExecutionCheckpoint;
    readonly justifiedBeacon: PalletSepoliaFinalityVerifierBeaconCheckpoint;
    readonly justifiedExecution: PalletSepoliaFinalityVerifierExecutionCheckpoint;
    readonly finalizedBeacon: PalletSepoliaFinalityVerifierBeaconCheckpoint;
    readonly finalizedExecution: PalletSepoliaFinalityVerifierExecutionCheckpoint;
  }

  /** @name PalletSepoliaFinalityVerifierBeaconCheckpoint (628) */
  interface PalletSepoliaFinalityVerifierBeaconCheckpoint extends Struct {
    readonly epoch: u64;
    readonly root: U8aFixed;
  }

  /** @name PalletSepoliaFinalityVerifierExecutionCheckpoint (629) */
  interface PalletSepoliaFinalityVerifierExecutionCheckpoint extends Struct {
    readonly height: u64;
    readonly root: U8aFixed;
  }

  /** @name PalletSepoliaFinalityVerifierError (630) */
  interface PalletSepoliaFinalityVerifierError extends Enum {
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
    readonly isExecutionHeaderNotFinalized: boolean;
    readonly isInvalidBeaconLinkage: boolean;
    readonly isInvalidExecutionPayload: boolean;
    readonly isInvalidSourceAddress: boolean;
    readonly type: 'Halted' | 'AlreadyInitialized' | 'InvalidInitializationData' | 'SszForkDataHashTreeRootFailed' | 'SszSigningDataHashTreeRootFailed' | 'BlsPubkeyAggregationFaild' | 'InvalidBLSPublicKeyUsedForVerification' | 'InvalidInclusionProof' | 'ForkNotDetected' | 'ValidSyncCommitteeNotAvailable' | 'SubmittedHeaderToOld' | 'InvalidBLSSignature' | 'InvalidMerkleProof' | 'BeaconHeaderHashTreeRootFailed' | 'BeaconHeaderNotFound' | 'BeaconHeaderNotFinalized' | 'ExecutionHeaderHashTreeRootFailed' | 'InvalidExecutionRangeLinkage' | 'InvalidExecutionRange' | 'SyncCommitteeParticipantsNotSupermajority' | 'SyncCommitteeInvalid' | 'NotPeriodsFirstEpoch' | 'InvalidCheckpoint' | 'ExecutionHeaderNotFound' | 'EventNotInReceipt' | 'InvalidEncodedEpochUpdate' | 'InvalidSyncCommitteePeriod' | 'MathError' | 'CurrentSyncCommitteePeriodNotAvailable' | 'BeaconCheckpointHashTreeRootFailed' | 'InvalidFork' | 'ExecutionHeaderNotFinalized' | 'InvalidBeaconLinkage' | 'InvalidExecutionPayload' | 'InvalidSourceAddress';
  }

  /** @name PalletMaintenanceModeError (631) */
  interface PalletMaintenanceModeError extends Enum {
    readonly isAlreadyInMaintenanceMode: boolean;
    readonly isNotInMaintenanceMode: boolean;
    readonly type: 'AlreadyInMaintenanceMode' | 'NotInMaintenanceMode';
  }

  /** @name PalletSudoError (632) */
  interface PalletSudoError extends Enum {
    readonly isRequireSudo: boolean;
    readonly type: 'RequireSudo';
  }

  /** @name SpRuntimeMultiSignature (634) */
  interface SpRuntimeMultiSignature extends Enum {
    readonly isEd25519: boolean;
    readonly asEd25519: SpCoreEd25519Signature;
    readonly isSr25519: boolean;
    readonly asSr25519: SpCoreSr25519Signature;
    readonly isEcdsa: boolean;
    readonly asEcdsa: SpCoreEcdsaSignature;
    readonly type: 'Ed25519' | 'Sr25519' | 'Ecdsa';
  }

  /** @name SpCoreSr25519Signature (635) */
  interface SpCoreSr25519Signature extends U8aFixed {}

  /** @name SpCoreEcdsaSignature (636) */
  interface SpCoreEcdsaSignature extends U8aFixed {}

  /** @name FrameSystemExtensionsCheckNonZeroSender (638) */
  type FrameSystemExtensionsCheckNonZeroSender = Null;

  /** @name FrameSystemExtensionsCheckSpecVersion (639) */
  type FrameSystemExtensionsCheckSpecVersion = Null;

  /** @name FrameSystemExtensionsCheckTxVersion (640) */
  type FrameSystemExtensionsCheckTxVersion = Null;

  /** @name FrameSystemExtensionsCheckGenesis (641) */
  type FrameSystemExtensionsCheckGenesis = Null;

  /** @name FrameSystemExtensionsCheckNonce (644) */
  interface FrameSystemExtensionsCheckNonce extends Compact<u32> {}

  /** @name FrameSystemExtensionsCheckWeight (645) */
  type FrameSystemExtensionsCheckWeight = Null;

  /** @name PalletAssetTxPaymentChargeAssetTxPayment (646) */
  interface PalletAssetTxPaymentChargeAssetTxPayment extends Struct {
    readonly tip: Compact<u128>;
    readonly assetId: Option<u32>;
  }

} // declare module
