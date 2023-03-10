// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

declare module '@polkadot/types/lookup' {
  import type { Data } from '@polkadot/types';
  import type { BTreeMap, Bytes, Compact, Enum, Null, Option, Result, Set, Struct, Text, U256, U8aFixed, Vec, bool, u128, u16, u32, u64, u8 } from '@polkadot/types-codec';
  import type { ITuple } from '@polkadot/types-codec/types';
  import type { AccountId32, Call, H160, H256, MultiAddress, Perbill } from '@polkadot/types/interfaces/runtime';
  import type { Event } from '@polkadot/types/interfaces/system';

  /** @name FrameSystemAccountInfo (3) */
  export interface FrameSystemAccountInfo extends Struct {
    readonly nonce: u32;
    readonly consumers: u32;
    readonly providers: u32;
    readonly sufficients: u32;
    readonly data: PalletBalancesAccountData;
  }

  /** @name PalletBalancesAccountData (5) */
  export interface PalletBalancesAccountData extends Struct {
    readonly free: u128;
    readonly reserved: u128;
    readonly miscFrozen: u128;
    readonly feeFrozen: u128;
  }

  /** @name FrameSupportWeightsPerDispatchClassU64 (7) */
  export interface FrameSupportWeightsPerDispatchClassU64 extends Struct {
    readonly normal: u64;
    readonly operational: u64;
    readonly mandatory: u64;
  }

  /** @name SpRuntimeDigest (11) */
  export interface SpRuntimeDigest extends Struct {
    readonly logs: Vec<SpRuntimeDigestDigestItem>;
  }

  /** @name SpRuntimeDigestDigestItem (13) */
  export interface SpRuntimeDigestDigestItem extends Enum {
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
  export interface FrameSystemEventRecord extends Struct {
    readonly phase: FrameSystemPhase;
    readonly event: Event;
    readonly topics: Vec<H256>;
  }

  /** @name FrameSystemEvent (18) */
  export interface FrameSystemEvent extends Enum {
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
  export interface FrameSupportWeightsDispatchInfo extends Struct {
    readonly weight: u64;
    readonly class: FrameSupportWeightsDispatchClass;
    readonly paysFee: FrameSupportWeightsPays;
  }

  /** @name FrameSupportWeightsDispatchClass (20) */
  export interface FrameSupportWeightsDispatchClass extends Enum {
    readonly isNormal: boolean;
    readonly isOperational: boolean;
    readonly isMandatory: boolean;
    readonly type: 'Normal' | 'Operational' | 'Mandatory';
  }

  /** @name FrameSupportWeightsPays (21) */
  export interface FrameSupportWeightsPays extends Enum {
    readonly isYes: boolean;
    readonly isNo: boolean;
    readonly type: 'Yes' | 'No';
  }

  /** @name SpRuntimeDispatchError (22) */
  export interface SpRuntimeDispatchError extends Enum {
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
  export interface SpRuntimeModuleError extends Struct {
    readonly index: u8;
    readonly error: U8aFixed;
  }

  /** @name SpRuntimeTokenError (24) */
  export interface SpRuntimeTokenError extends Enum {
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
  export interface SpRuntimeArithmeticError extends Enum {
    readonly isUnderflow: boolean;
    readonly isOverflow: boolean;
    readonly isDivisionByZero: boolean;
    readonly type: 'Underflow' | 'Overflow' | 'DivisionByZero';
  }

  /** @name SpRuntimeTransactionalError (26) */
  export interface SpRuntimeTransactionalError extends Enum {
    readonly isLimitReached: boolean;
    readonly isNoLayer: boolean;
    readonly type: 'LimitReached' | 'NoLayer';
  }

  /** @name PalletGrandpaEvent (27) */
  export interface PalletGrandpaEvent extends Enum {
    readonly isNewAuthorities: boolean;
    readonly asNewAuthorities: {
      readonly authoritySet: Vec<ITuple<[SpFinalityGrandpaAppPublic, u64]>>;
    } & Struct;
    readonly isPaused: boolean;
    readonly isResumed: boolean;
    readonly type: 'NewAuthorities' | 'Paused' | 'Resumed';
  }

  /** @name SpFinalityGrandpaAppPublic (30) */
  export interface SpFinalityGrandpaAppPublic extends SpCoreEd25519Public {}

  /** @name SpCoreEd25519Public (31) */
  export interface SpCoreEd25519Public extends U8aFixed {}

  /** @name PalletSudoEvent (32) */
  export interface PalletSudoEvent extends Enum {
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

  /** @name PalletUtilityEvent (36) */
  export interface PalletUtilityEvent extends Enum {
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

  /** @name PalletIdentityEvent (37) */
  export interface PalletIdentityEvent extends Enum {
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

  /** @name PalletBalancesEvent (38) */
  export interface PalletBalancesEvent extends Enum {
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

  /** @name FrameSupportTokensMiscBalanceStatus (39) */
  export interface FrameSupportTokensMiscBalanceStatus extends Enum {
    readonly isFree: boolean;
    readonly isReserved: boolean;
    readonly type: 'Free' | 'Reserved';
  }

  /** @name PalletTransactionPaymentEvent (40) */
  export interface PalletTransactionPaymentEvent extends Enum {
    readonly isTransactionFeePaid: boolean;
    readonly asTransactionFeePaid: {
      readonly who: AccountId32;
      readonly actualFee: u128;
      readonly tip: u128;
    } & Struct;
    readonly type: 'TransactionFeePaid';
  }

  /** @name PalletTreasuryEvent (41) */
  export interface PalletTreasuryEvent extends Enum {
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

  /** @name PalletAssetsEvent (42) */
  export interface PalletAssetsEvent extends Enum {
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

  /** @name PalletXdnsEvent (44) */
  export interface PalletXdnsEvent extends Enum {
    readonly isXdnsRecordStored: boolean;
    readonly asXdnsRecordStored: U8aFixed;
    readonly isXdnsRecordPurged: boolean;
    readonly asXdnsRecordPurged: ITuple<[AccountId32, U8aFixed]>;
    readonly isXdnsRecordUpdated: boolean;
    readonly asXdnsRecordUpdated: U8aFixed;
    readonly type: 'XdnsRecordStored' | 'XdnsRecordPurged' | 'XdnsRecordUpdated';
  }

  /** @name PalletContractsRegistryEvent (45) */
  export interface PalletContractsRegistryEvent extends Enum {
    readonly isContractStored: boolean;
    readonly asContractStored: ITuple<[AccountId32, H256]>;
    readonly isContractPurged: boolean;
    readonly asContractPurged: ITuple<[AccountId32, H256]>;
    readonly type: 'ContractStored' | 'ContractPurged';
  }

  /** @name PalletCircuitEvent (46) */
  export interface PalletCircuitEvent extends Enum {
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
    readonly asResult: ITuple<[AccountId32, AccountId32, XbiFormatXbiCheckOutStatus, Bytes, Bytes]>;
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
    readonly type: 'Transfer' | 'TransferAssets' | 'TransferORML' | 'AddLiquidity' | 'Swap' | 'CallNative' | 'CallEvm' | 'CallWasm' | 'CallCustom' | 'Result' | 'XTransactionReceivedForExec' | 'SfxNewBidReceived' | 'SideEffectConfirmed' | 'XTransactionReadyForExec' | 'XTransactionStepFinishedExec' | 'XTransactionXtxFinishedExecAllSteps' | 'XTransactionXtxRevertedAfterTimeOut' | 'XTransactionXtxDroppedAtBidding' | 'NewSideEffectsAvailable' | 'CancelledSideEffects' | 'SideEffectsConfirmed' | 'EscrowTransfer';
  }

  /** @name XbiFormatXbiCheckOutStatus (56) */
  export interface XbiFormatXbiCheckOutStatus extends Enum {
    readonly isSuccessfullyExecuted: boolean;
    readonly isErrorFailedExecution: boolean;
    readonly isErrorFailedOnXCMDispatch: boolean;
    readonly isErrorExecutionCostsExceededAllowedMax: boolean;
    readonly isErrorNotificationsCostsExceededAllowedMax: boolean;
    readonly isErrorSentTimeoutExceeded: boolean;
    readonly isErrorDeliveryTimeoutExceeded: boolean;
    readonly isErrorExecutionTimeoutExceeded: boolean;
    readonly type: 'SuccessfullyExecuted' | 'ErrorFailedExecution' | 'ErrorFailedOnXCMDispatch' | 'ErrorExecutionCostsExceededAllowedMax' | 'ErrorNotificationsCostsExceededAllowedMax' | 'ErrorSentTimeoutExceeded' | 'ErrorDeliveryTimeoutExceeded' | 'ErrorExecutionTimeoutExceeded';
  }

  /** @name T3rnTypesSfxSideEffect (58) */
  export interface T3rnTypesSfxSideEffect extends Struct {
    readonly target: U8aFixed;
    readonly action: U8aFixed;
    readonly maxReward: u128;
    readonly insurance: u128;
    readonly encodedArgs: Vec<Bytes>;
    readonly signature: Bytes;
    readonly enforceExecutor: Option<AccountId32>;
    readonly rewardAssetId: Option<u32>;
  }

  /** @name T3rnTypesFsxFullSideEffect (63) */
  export interface T3rnTypesFsxFullSideEffect extends Struct {
    readonly input: T3rnTypesSfxSideEffect;
    readonly confirmed: Option<T3rnTypesSfxConfirmedSideEffect>;
    readonly securityLvl: T3rnTypesSfxSecurityLvl;
    readonly submissionTargetHeight: Bytes;
    readonly bestBid: Option<T3rnTypesBidSfxBid>;
    readonly index: u32;
  }

  /** @name T3rnTypesSfxConfirmedSideEffect (65) */
  export interface T3rnTypesSfxConfirmedSideEffect extends Struct {
    readonly err: Option<T3rnTypesSfxConfirmationOutcome>;
    readonly output: Option<Bytes>;
    readonly inclusionData: Bytes;
    readonly executioner: AccountId32;
    readonly receivedAt: u32;
    readonly cost: Option<u128>;
  }

  /** @name T3rnTypesSfxConfirmationOutcome (67) */
  export interface T3rnTypesSfxConfirmationOutcome extends Enum {
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

  /** @name T3rnTypesSfxSecurityLvl (69) */
  export interface T3rnTypesSfxSecurityLvl extends Enum {
    readonly isOptimistic: boolean;
    readonly isEscrow: boolean;
    readonly type: 'Optimistic' | 'Escrow';
  }

  /** @name T3rnTypesBidSfxBid (71) */
  export interface T3rnTypesBidSfxBid extends Struct {
    readonly amount: u128;
    readonly insurance: u128;
    readonly reservedBond: Option<u128>;
    readonly rewardAssetId: Option<u32>;
    readonly executor: AccountId32;
    readonly requester: AccountId32;
  }

  /** @name PalletClockEvent (72) */
  export interface PalletClockEvent extends Enum {
    readonly isNewRound: boolean;
    readonly asNewRound: {
      readonly index: u32;
      readonly head: u32;
      readonly term: u32;
    } & Struct;
    readonly type: 'NewRound';
  }

  /** @name Pallet3vmEvent (73) */
  export interface Pallet3vmEvent extends Enum {
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

  /** @name T3rnSdkPrimitivesSignalSignalKind (75) */
  export interface T3rnSdkPrimitivesSignalSignalKind extends Enum {
    readonly isComplete: boolean;
    readonly isKill: boolean;
    readonly asKill: T3rnSdkPrimitivesSignalKillReason;
    readonly type: 'Complete' | 'Kill';
  }

  /** @name T3rnSdkPrimitivesSignalKillReason (76) */
  export interface T3rnSdkPrimitivesSignalKillReason extends Enum {
    readonly isUnhandled: boolean;
    readonly isCodec: boolean;
    readonly isTimeout: boolean;
    readonly type: 'Unhandled' | 'Codec' | 'Timeout';
  }

  /** @name T3rnPrimitivesContractMetadataContractType (78) */
  export interface T3rnPrimitivesContractMetadataContractType extends Enum {
    readonly isSystem: boolean;
    readonly isVanillaEvm: boolean;
    readonly isVanillaWasm: boolean;
    readonly isVolatileEvm: boolean;
    readonly isVolatileWasm: boolean;
    readonly type: 'System' | 'VanillaEvm' | 'VanillaWasm' | 'VolatileEvm' | 'VolatileWasm';
  }

  /** @name PalletContractsEvent (80) */
  export interface PalletContractsEvent extends Enum {
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

  /** @name PalletEvmEvent (81) */
  export interface PalletEvmEvent extends Enum {
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

  /** @name EthereumLog (82) */
  export interface EthereumLog extends Struct {
    readonly address: H160;
    readonly topics: Vec<H256>;
    readonly data: Bytes;
  }

  /** @name PalletAccountManagerEvent (83) */
  export interface PalletAccountManagerEvent extends Enum {
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

  /** @name PalletPortalEvent (84) */
  export interface PalletPortalEvent extends Enum {
    readonly isGatewayRegistered: boolean;
    readonly asGatewayRegistered: U8aFixed;
    readonly isSetOwner: boolean;
    readonly asSetOwner: ITuple<[U8aFixed, Bytes]>;
    readonly isSetOperational: boolean;
    readonly asSetOperational: ITuple<[U8aFixed, bool]>;
    readonly isHeaderSubmitted: boolean;
    readonly asHeaderSubmitted: ITuple<[U8aFixed, Bytes]>;
    readonly type: 'GatewayRegistered' | 'SetOwner' | 'SetOperational' | 'HeaderSubmitted';
  }

  /** @name FrameSystemPhase (85) */
  export interface FrameSystemPhase extends Enum {
    readonly isApplyExtrinsic: boolean;
    readonly asApplyExtrinsic: u32;
    readonly isFinalization: boolean;
    readonly isInitialization: boolean;
    readonly type: 'ApplyExtrinsic' | 'Finalization' | 'Initialization';
  }

  /** @name FrameSystemLastRuntimeUpgradeInfo (88) */
  export interface FrameSystemLastRuntimeUpgradeInfo extends Struct {
    readonly specVersion: Compact<u32>;
    readonly specName: Text;
  }

  /** @name FrameSystemCall (91) */
  export interface FrameSystemCall extends Enum {
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

  /** @name FrameSystemLimitsBlockWeights (95) */
  export interface FrameSystemLimitsBlockWeights extends Struct {
    readonly baseBlock: u64;
    readonly maxBlock: u64;
    readonly perClass: FrameSupportWeightsPerDispatchClassWeightsPerClass;
  }

  /** @name FrameSupportWeightsPerDispatchClassWeightsPerClass (96) */
  export interface FrameSupportWeightsPerDispatchClassWeightsPerClass extends Struct {
    readonly normal: FrameSystemLimitsWeightsPerClass;
    readonly operational: FrameSystemLimitsWeightsPerClass;
    readonly mandatory: FrameSystemLimitsWeightsPerClass;
  }

  /** @name FrameSystemLimitsWeightsPerClass (97) */
  export interface FrameSystemLimitsWeightsPerClass extends Struct {
    readonly baseExtrinsic: u64;
    readonly maxExtrinsic: Option<u64>;
    readonly maxTotal: Option<u64>;
    readonly reserved: Option<u64>;
  }

  /** @name FrameSystemLimitsBlockLength (99) */
  export interface FrameSystemLimitsBlockLength extends Struct {
    readonly max: FrameSupportWeightsPerDispatchClassU32;
  }

  /** @name FrameSupportWeightsPerDispatchClassU32 (100) */
  export interface FrameSupportWeightsPerDispatchClassU32 extends Struct {
    readonly normal: u32;
    readonly operational: u32;
    readonly mandatory: u32;
  }

  /** @name FrameSupportWeightsRuntimeDbWeight (101) */
  export interface FrameSupportWeightsRuntimeDbWeight extends Struct {
    readonly read: u64;
    readonly write: u64;
  }

  /** @name SpVersionRuntimeVersion (102) */
  export interface SpVersionRuntimeVersion extends Struct {
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
  export interface FrameSystemError extends Enum {
    readonly isInvalidSpecName: boolean;
    readonly isSpecVersionNeedsToIncrease: boolean;
    readonly isFailedToExtractRuntimeVersion: boolean;
    readonly isNonDefaultComposite: boolean;
    readonly isNonZeroRefCount: boolean;
    readonly isCallFiltered: boolean;
    readonly type: 'InvalidSpecName' | 'SpecVersionNeedsToIncrease' | 'FailedToExtractRuntimeVersion' | 'NonDefaultComposite' | 'NonZeroRefCount' | 'CallFiltered';
  }

  /** @name PalletTimestampCall (110) */
  export interface PalletTimestampCall extends Enum {
    readonly isSet: boolean;
    readonly asSet: {
      readonly now: Compact<u64>;
    } & Struct;
    readonly type: 'Set';
  }

  /** @name SpConsensusAuraSr25519AppSr25519Public (113) */
  export interface SpConsensusAuraSr25519AppSr25519Public extends SpCoreSr25519Public {}

  /** @name SpCoreSr25519Public (114) */
  export interface SpCoreSr25519Public extends U8aFixed {}

  /** @name PalletGrandpaStoredState (117) */
  export interface PalletGrandpaStoredState extends Enum {
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
    readonly type: 'Live' | 'PendingPause' | 'Paused' | 'PendingResume';
  }

  /** @name PalletGrandpaStoredPendingChange (118) */
  export interface PalletGrandpaStoredPendingChange extends Struct {
    readonly scheduledAt: u32;
    readonly delay: u32;
    readonly nextAuthorities: Vec<ITuple<[SpFinalityGrandpaAppPublic, u64]>>;
    readonly forced: Option<u32>;
  }

  /** @name PalletGrandpaCall (120) */
  export interface PalletGrandpaCall extends Enum {
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
    readonly type: 'ReportEquivocation' | 'ReportEquivocationUnsigned' | 'NoteStalled';
  }

  /** @name SpFinalityGrandpaEquivocationProof (121) */
  export interface SpFinalityGrandpaEquivocationProof extends Struct {
    readonly setId: u64;
    readonly equivocation: SpFinalityGrandpaEquivocation;
  }

  /** @name SpFinalityGrandpaEquivocation (122) */
  export interface SpFinalityGrandpaEquivocation extends Enum {
    readonly isPrevote: boolean;
    readonly asPrevote: FinalityGrandpaEquivocationPrevote;
    readonly isPrecommit: boolean;
    readonly asPrecommit: FinalityGrandpaEquivocationPrecommit;
    readonly type: 'Prevote' | 'Precommit';
  }

  /** @name FinalityGrandpaEquivocationPrevote (123) */
  export interface FinalityGrandpaEquivocationPrevote extends Struct {
    readonly roundNumber: u64;
    readonly identity: SpFinalityGrandpaAppPublic;
    readonly first: ITuple<[FinalityGrandpaPrevote, SpFinalityGrandpaAppSignature]>;
    readonly second: ITuple<[FinalityGrandpaPrevote, SpFinalityGrandpaAppSignature]>;
  }

  /** @name FinalityGrandpaPrevote (124) */
  export interface FinalityGrandpaPrevote extends Struct {
    readonly targetHash: H256;
    readonly targetNumber: u32;
  }

  /** @name SpFinalityGrandpaAppSignature (125) */
  export interface SpFinalityGrandpaAppSignature extends SpCoreEd25519Signature {}

  /** @name SpCoreEd25519Signature (126) */
  export interface SpCoreEd25519Signature extends U8aFixed {}

  /** @name FinalityGrandpaEquivocationPrecommit (129) */
  export interface FinalityGrandpaEquivocationPrecommit extends Struct {
    readonly roundNumber: u64;
    readonly identity: SpFinalityGrandpaAppPublic;
    readonly first: ITuple<[FinalityGrandpaPrecommit, SpFinalityGrandpaAppSignature]>;
    readonly second: ITuple<[FinalityGrandpaPrecommit, SpFinalityGrandpaAppSignature]>;
  }

  /** @name FinalityGrandpaPrecommit (130) */
  export interface FinalityGrandpaPrecommit extends Struct {
    readonly targetHash: H256;
    readonly targetNumber: u32;
  }

  /** @name SpCoreVoid (132) */
  export type SpCoreVoid = Null;

  /** @name PalletGrandpaError (133) */
  export interface PalletGrandpaError extends Enum {
    readonly isPauseFailed: boolean;
    readonly isResumeFailed: boolean;
    readonly isChangePending: boolean;
    readonly isTooSoon: boolean;
    readonly isInvalidKeyOwnershipProof: boolean;
    readonly isInvalidEquivocationProof: boolean;
    readonly isDuplicateOffenceReport: boolean;
    readonly type: 'PauseFailed' | 'ResumeFailed' | 'ChangePending' | 'TooSoon' | 'InvalidKeyOwnershipProof' | 'InvalidEquivocationProof' | 'DuplicateOffenceReport';
  }

  /** @name PalletSudoCall (134) */
  export interface PalletSudoCall extends Enum {
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

  /** @name PalletUtilityCall (136) */
  export interface PalletUtilityCall extends Enum {
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
    readonly type: 'Batch' | 'AsDerivative' | 'BatchAll' | 'DispatchAs' | 'ForceBatch';
  }

  /** @name CircuitStandaloneRuntimeOriginCaller (138) */
  export interface CircuitStandaloneRuntimeOriginCaller extends Enum {
    readonly isSystem: boolean;
    readonly asSystem: FrameSupportDispatchRawOrigin;
    readonly isVoid: boolean;
    readonly type: 'System' | 'Void';
  }

  /** @name FrameSupportDispatchRawOrigin (139) */
  export interface FrameSupportDispatchRawOrigin extends Enum {
    readonly isRoot: boolean;
    readonly isSigned: boolean;
    readonly asSigned: AccountId32;
    readonly isNone: boolean;
    readonly type: 'Root' | 'Signed' | 'None';
  }

  /** @name PalletIdentityCall (140) */
  export interface PalletIdentityCall extends Enum {
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

  /** @name PalletIdentityIdentityInfo (141) */
  export interface PalletIdentityIdentityInfo extends Struct {
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

  /** @name PalletIdentityBitFlags (179) */
  export interface PalletIdentityBitFlags extends Set {
    readonly isDisplay: boolean;
    readonly isLegal: boolean;
    readonly isWeb: boolean;
    readonly isRiot: boolean;
    readonly isEmail: boolean;
    readonly isPgpFingerprint: boolean;
    readonly isImage: boolean;
    readonly isTwitter: boolean;
  }

  /** @name PalletIdentityIdentityField (180) */
  export interface PalletIdentityIdentityField extends Enum {
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

  /** @name PalletIdentityJudgement (183) */
  export interface PalletIdentityJudgement extends Enum {
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

  /** @name PalletBalancesCall (184) */
  export interface PalletBalancesCall extends Enum {
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

  /** @name PalletTreasuryCall (185) */
  export interface PalletTreasuryCall extends Enum {
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

  /** @name PalletAssetsCall (186) */
  export interface PalletAssetsCall extends Enum {
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

  /** @name PalletAssetsDestroyWitness (187) */
  export interface PalletAssetsDestroyWitness extends Struct {
    readonly accounts: Compact<u32>;
    readonly sufficients: Compact<u32>;
    readonly approvals: Compact<u32>;
  }

  /** @name PalletAuthorshipCall (188) */
  export interface PalletAuthorshipCall extends Enum {
    readonly isSetUncles: boolean;
    readonly asSetUncles: {
      readonly newUncles: Vec<SpRuntimeHeader>;
    } & Struct;
    readonly type: 'SetUncles';
  }

  /** @name SpRuntimeHeader (190) */
  export interface SpRuntimeHeader extends Struct {
    readonly parentHash: H256;
    readonly number: Compact<u32>;
    readonly stateRoot: H256;
    readonly extrinsicsRoot: H256;
    readonly digest: SpRuntimeDigest;
  }

  /** @name SpRuntimeBlakeTwo256 (191) */
  export type SpRuntimeBlakeTwo256 = Null;

  /** @name PalletXdnsCall (192) */
  export interface PalletXdnsCall extends Enum {
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
    readonly type: 'AddSideEffect' | 'UpdateTtl' | 'PurgeXdnsRecord';
  }

  /** @name T3rnTypesAbiType (194) */
  export interface T3rnTypesAbiType extends Enum {
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
    readonly type: 'Address' | 'DynamicAddress' | 'Bool' | 'Int' | 'Uint' | 'Bytes' | 'DynamicBytes' | 'String' | 'Enum' | 'Struct' | 'Mapping' | 'Contract' | 'Ref' | 'Option' | 'OptionalInsurance' | 'OptionalReward' | 'StorageRef' | 'Value' | 'Slice' | 'Hasher' | 'Crypto';
  }

  /** @name T3rnTypesAbiHasherAlgo (195) */
  export interface T3rnTypesAbiHasherAlgo extends Enum {
    readonly isBlake2: boolean;
    readonly isKeccak256: boolean;
    readonly type: 'Blake2' | 'Keccak256';
  }

  /** @name T3rnTypesAbiCryptoAlgo (196) */
  export interface T3rnTypesAbiCryptoAlgo extends Enum {
    readonly isEd25519: boolean;
    readonly isSr25519: boolean;
    readonly isEcdsa: boolean;
    readonly type: 'Ed25519' | 'Sr25519' | 'Ecdsa';
  }

  /** @name PalletContractsRegistryCall (197) */
  export interface PalletContractsRegistryCall extends Enum {
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

  /** @name T3rnPrimitivesContractsRegistryRegistryContract (198) */
  export interface T3rnPrimitivesContractsRegistryRegistryContract extends Struct {
    readonly codeTxt: Bytes;
    readonly bytes: Bytes;
    readonly author: T3rnPrimitivesContractsRegistryAuthorInfo;
    readonly abi: Option<Bytes>;
    readonly actionDescriptions: Vec<T3rnTypesAbiContractActionDesc>;
    readonly info: Option<T3rnPrimitivesStorageRawAliveContractInfo>;
    readonly meta: T3rnPrimitivesContractMetadata;
  }

  /** @name T3rnPrimitivesContractsRegistryAuthorInfo (199) */
  export interface T3rnPrimitivesContractsRegistryAuthorInfo extends Struct {
    readonly account: AccountId32;
    readonly feesPerSingleUse: Option<u128>;
  }

  /** @name T3rnTypesAbiContractActionDesc (201) */
  export interface T3rnTypesAbiContractActionDesc extends Struct {
    readonly actionId: H256;
    readonly targetId: Option<U8aFixed>;
    readonly to: Option<AccountId32>;
  }

  /** @name T3rnPrimitivesStorageRawAliveContractInfo (204) */
  export interface T3rnPrimitivesStorageRawAliveContractInfo extends Struct {
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

  /** @name T3rnPrimitivesContractMetadata (206) */
  export interface T3rnPrimitivesContractMetadata extends Struct {
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

  /** @name PalletCircuitCall (207) */
  export interface PalletCircuitCall extends Enum {
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
      readonly sequential: bool;
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

  /** @name Pallet3vmCall (208) */
  export type Pallet3vmCall = Null;

  /** @name PalletContractsCall (209) */
  export interface PalletContractsCall extends Enum {
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

  /** @name PalletEvmCall (211) */
  export interface PalletEvmCall extends Enum {
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

  /** @name PalletAccountManagerCall (212) */
  export interface PalletAccountManagerCall extends Enum {
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

  /** @name T3rnPrimitivesClaimableBenefitSource (213) */
  export interface T3rnPrimitivesClaimableBenefitSource extends Enum {
    readonly isBootstrapPool: boolean;
    readonly isInflation: boolean;
    readonly isTrafficFees: boolean;
    readonly isTrafficRewards: boolean;
    readonly isUnsettled: boolean;
    readonly type: 'BootstrapPool' | 'Inflation' | 'TrafficFees' | 'TrafficRewards' | 'Unsettled';
  }

  /** @name T3rnPrimitivesClaimableCircuitRole (214) */
  export interface T3rnPrimitivesClaimableCircuitRole extends Enum {
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

  /** @name T3rnPrimitivesAccountManagerOutcome (215) */
  export interface T3rnPrimitivesAccountManagerOutcome extends Enum {
    readonly isUnexpectedFailure: boolean;
    readonly isRevert: boolean;
    readonly isCommit: boolean;
    readonly isSlash: boolean;
    readonly type: 'UnexpectedFailure' | 'Revert' | 'Commit' | 'Slash';
  }

  /** @name PalletPortalCall (216) */
  export interface PalletPortalCall extends Enum {
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
    readonly type: 'RegisterGateway' | 'SetOwner' | 'SetOperational' | 'SubmitHeaders';
  }

  /** @name T3rnTypesAbiGatewayABIConfig (217) */
  export interface T3rnTypesAbiGatewayABIConfig extends Struct {
    readonly blockNumberTypeSize: u16;
    readonly hashSize: u16;
    readonly hasher: T3rnTypesAbiHasherAlgo;
    readonly crypto: T3rnTypesAbiCryptoAlgo;
    readonly addressLength: u16;
    readonly valueTypeSize: u16;
    readonly decimals: u16;
    readonly structs: Vec<T3rnTypesAbiStructDecl>;
  }

  /** @name T3rnTypesAbiStructDecl (219) */
  export interface T3rnTypesAbiStructDecl extends Struct {
    readonly name: T3rnTypesAbiType;
    readonly fields: Vec<T3rnTypesAbiParameter>;
    readonly offsets: Vec<u16>;
  }

  /** @name T3rnTypesAbiParameter (221) */
  export interface T3rnTypesAbiParameter extends Struct {
    readonly name: Option<Bytes>;
    readonly ty: T3rnTypesAbiType;
    readonly no: u32;
    readonly indexed: Option<bool>;
  }

  /** @name T3rnPrimitivesGatewayVendor (224) */
  export interface T3rnPrimitivesGatewayVendor extends Enum {
    readonly isPolkadot: boolean;
    readonly isKusama: boolean;
    readonly isRococo: boolean;
    readonly isEthereum: boolean;
    readonly type: 'Polkadot' | 'Kusama' | 'Rococo' | 'Ethereum';
  }

  /** @name T3rnPrimitivesGatewayType (225) */
  export interface T3rnPrimitivesGatewayType extends Enum {
    readonly isProgrammableInternal: boolean;
    readonly asProgrammableInternal: u32;
    readonly isProgrammableExternal: boolean;
    readonly asProgrammableExternal: u32;
    readonly isTxOnly: boolean;
    readonly asTxOnly: u32;
    readonly isOnCircuit: boolean;
    readonly asOnCircuit: u32;
    readonly type: 'ProgrammableInternal' | 'ProgrammableExternal' | 'TxOnly' | 'OnCircuit';
  }

  /** @name T3rnPrimitivesGatewayGenesisConfig (226) */
  export interface T3rnPrimitivesGatewayGenesisConfig extends Struct {
    readonly modulesEncoded: Option<Bytes>;
    readonly extrinsicsVersion: u8;
    readonly genesisHash: Bytes;
  }

  /** @name T3rnPrimitivesGatewaySysProps (227) */
  export interface T3rnPrimitivesGatewaySysProps extends Struct {
    readonly ss58Format: u16;
    readonly tokenSymbol: Bytes;
    readonly tokenDecimals: u8;
  }

  /** @name PalletSudoError (229) */
  export interface PalletSudoError extends Enum {
    readonly isRequireSudo: boolean;
    readonly type: 'RequireSudo';
  }

  /** @name PalletUtilityError (230) */
  export interface PalletUtilityError extends Enum {
    readonly isTooManyCalls: boolean;
    readonly type: 'TooManyCalls';
  }

  /** @name PalletIdentityRegistration (231) */
  export interface PalletIdentityRegistration extends Struct {
    readonly judgements: Vec<ITuple<[u32, PalletIdentityJudgement]>>;
    readonly deposit: u128;
    readonly info: PalletIdentityIdentityInfo;
  }

  /** @name PalletIdentityRegistrarInfo (240) */
  export interface PalletIdentityRegistrarInfo extends Struct {
    readonly account: AccountId32;
    readonly fee: u128;
    readonly fields: PalletIdentityBitFlags;
  }

  /** @name PalletIdentityError (242) */
  export interface PalletIdentityError extends Enum {
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

  /** @name PalletBalancesBalanceLock (244) */
  export interface PalletBalancesBalanceLock extends Struct {
    readonly id: U8aFixed;
    readonly amount: u128;
    readonly reasons: PalletBalancesReasons;
  }

  /** @name PalletBalancesReasons (245) */
  export interface PalletBalancesReasons extends Enum {
    readonly isFee: boolean;
    readonly isMisc: boolean;
    readonly isAll: boolean;
    readonly type: 'Fee' | 'Misc' | 'All';
  }

  /** @name PalletBalancesReserveData (248) */
  export interface PalletBalancesReserveData extends Struct {
    readonly id: U8aFixed;
    readonly amount: u128;
  }

  /** @name PalletBalancesReleases (250) */
  export interface PalletBalancesReleases extends Enum {
    readonly isV100: boolean;
    readonly isV200: boolean;
    readonly type: 'V100' | 'V200';
  }

  /** @name PalletBalancesError (251) */
  export interface PalletBalancesError extends Enum {
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

  /** @name PalletTransactionPaymentReleases (253) */
  export interface PalletTransactionPaymentReleases extends Enum {
    readonly isV1Ancient: boolean;
    readonly isV2: boolean;
    readonly type: 'V1Ancient' | 'V2';
  }

  /** @name PalletTreasuryProposal (254) */
  export interface PalletTreasuryProposal extends Struct {
    readonly proposer: AccountId32;
    readonly value: u128;
    readonly beneficiary: AccountId32;
    readonly bond: u128;
  }

  /** @name FrameSupportPalletId (258) */
  export interface FrameSupportPalletId extends U8aFixed {}

  /** @name PalletTreasuryError (259) */
  export interface PalletTreasuryError extends Enum {
    readonly isInsufficientProposersBalance: boolean;
    readonly isInvalidIndex: boolean;
    readonly isTooManyApprovals: boolean;
    readonly isInsufficientPermission: boolean;
    readonly isProposalNotApproved: boolean;
    readonly type: 'InsufficientProposersBalance' | 'InvalidIndex' | 'TooManyApprovals' | 'InsufficientPermission' | 'ProposalNotApproved';
  }

  /** @name PalletAssetsAssetDetails (260) */
  export interface PalletAssetsAssetDetails extends Struct {
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

  /** @name PalletAssetsAssetAccount (262) */
  export interface PalletAssetsAssetAccount extends Struct {
    readonly balance: u128;
    readonly isFrozen: bool;
    readonly reason: PalletAssetsExistenceReason;
    readonly extra: Null;
  }

  /** @name PalletAssetsExistenceReason (263) */
  export interface PalletAssetsExistenceReason extends Enum {
    readonly isConsumer: boolean;
    readonly isSufficient: boolean;
    readonly isDepositHeld: boolean;
    readonly asDepositHeld: u128;
    readonly isDepositRefunded: boolean;
    readonly type: 'Consumer' | 'Sufficient' | 'DepositHeld' | 'DepositRefunded';
  }

  /** @name PalletAssetsApproval (265) */
  export interface PalletAssetsApproval extends Struct {
    readonly amount: u128;
    readonly deposit: u128;
  }

  /** @name PalletAssetsAssetMetadata (266) */
  export interface PalletAssetsAssetMetadata extends Struct {
    readonly deposit: u128;
    readonly name: Bytes;
    readonly symbol: Bytes;
    readonly decimals: u8;
    readonly isFrozen: bool;
  }

  /** @name PalletAssetsError (268) */
  export interface PalletAssetsError extends Enum {
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

  /** @name PalletAuthorshipUncleEntryItem (270) */
  export interface PalletAuthorshipUncleEntryItem extends Enum {
    readonly isInclusionHeight: boolean;
    readonly asInclusionHeight: u32;
    readonly isUncle: boolean;
    readonly asUncle: ITuple<[H256, Option<AccountId32>]>;
    readonly type: 'InclusionHeight' | 'Uncle';
  }

  /** @name PalletAuthorshipError (272) */
  export interface PalletAuthorshipError extends Enum {
    readonly isInvalidUncleParent: boolean;
    readonly isUnclesAlreadySet: boolean;
    readonly isTooManyUncles: boolean;
    readonly isGenesisUncle: boolean;
    readonly isTooHighUncle: boolean;
    readonly isUncleAlreadyIncluded: boolean;
    readonly isOldUncle: boolean;
    readonly type: 'InvalidUncleParent' | 'UnclesAlreadySet' | 'TooManyUncles' | 'GenesisUncle' | 'TooHighUncle' | 'UncleAlreadyIncluded' | 'OldUncle';
  }

  /** @name T3rnTypesInterfaceSideEffectInterface (273) */
  export interface T3rnTypesInterfaceSideEffectInterface extends Struct {
    readonly id: U8aFixed;
    readonly name: Bytes;
    readonly argumentAbi: Vec<T3rnTypesAbiType>;
    readonly argumentToStateMapper: Vec<Bytes>;
    readonly confirmEvents: Vec<Bytes>;
    readonly escrowedEvents: Vec<Bytes>;
    readonly commitEvents: Vec<Bytes>;
    readonly revertEvents: Vec<Bytes>;
  }

  /** @name T3rnPrimitivesXdnsXdnsRecord (274) */
  export interface T3rnPrimitivesXdnsXdnsRecord extends Struct {
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

  /** @name T3rnPrimitivesXdnsParachain (276) */
  export interface T3rnPrimitivesXdnsParachain extends Struct {
    readonly relayChainId: U8aFixed;
    readonly id: u32;
  }

  /** @name PalletXdnsError (277) */
  export interface PalletXdnsError extends Enum {
    readonly isXdnsRecordAlreadyExists: boolean;
    readonly isUnknownXdnsRecord: boolean;
    readonly isXdnsRecordNotFound: boolean;
    readonly isSideEffectInterfaceAlreadyExists: boolean;
    readonly isSideEffectInterfaceNotFound: boolean;
    readonly isNoParachainInfoFound: boolean;
    readonly type: 'XdnsRecordAlreadyExists' | 'UnknownXdnsRecord' | 'XdnsRecordNotFound' | 'SideEffectInterfaceAlreadyExists' | 'SideEffectInterfaceNotFound' | 'NoParachainInfoFound';
  }

  /** @name PalletContractsRegistryError (278) */
  export interface PalletContractsRegistryError extends Enum {
    readonly isContractAlreadyExists: boolean;
    readonly isUnknownContract: boolean;
    readonly type: 'ContractAlreadyExists' | 'UnknownContract';
  }

  /** @name PalletCircuitStateXExecSignal (279) */
  export interface PalletCircuitStateXExecSignal extends Struct {
    readonly requester: AccountId32;
    readonly requesterNonce: u32;
    readonly timeoutsAt: u32;
    readonly delayStepsAt: Option<Vec<u32>>;
    readonly status: PalletCircuitStateCircuitStatus;
    readonly stepsCnt: ITuple<[u32, u32]>;
  }

  /** @name PalletCircuitStateCircuitStatus (281) */
  export interface PalletCircuitStateCircuitStatus extends Enum {
    readonly isRequested: boolean;
    readonly isReserved: boolean;
    readonly isPendingBidding: boolean;
    readonly isInBidding: boolean;
    readonly isKilled: boolean;
    readonly asKilled: PalletCircuitStateCause;
    readonly isReady: boolean;
    readonly isPendingExecution: boolean;
    readonly isFinished: boolean;
    readonly isFinishedAllSteps: boolean;
    readonly isReverted: boolean;
    readonly asReverted: PalletCircuitStateCause;
    readonly isCommitted: boolean;
    readonly type: 'Requested' | 'Reserved' | 'PendingBidding' | 'InBidding' | 'Killed' | 'Ready' | 'PendingExecution' | 'Finished' | 'FinishedAllSteps' | 'Reverted' | 'Committed';
  }

  /** @name PalletCircuitStateCause (282) */
  export interface PalletCircuitStateCause extends Enum {
    readonly isTimeout: boolean;
    readonly isIntentionalKill: boolean;
    readonly type: 'Timeout' | 'IntentionalKill';
  }

  /** @name T3rnPrimitivesVolatileLocalState (283) */
  export interface T3rnPrimitivesVolatileLocalState extends Struct {
    readonly state: BTreeMap<U8aFixed, Bytes>;
  }

  /** @name T3rnSdkPrimitivesSignalExecutionSignal (289) */
  export interface T3rnSdkPrimitivesSignalExecutionSignal extends Struct {
    readonly step: u32;
    readonly kind: T3rnSdkPrimitivesSignalSignalKind;
    readonly executionId: H256;
  }

  /** @name PalletCircuitError (291) */
  export interface PalletCircuitError extends Enum {
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
    readonly isXbiExitFailedOnSFXConfirmation: boolean;
    readonly isUnsupportedRole: boolean;
    readonly isInvalidLocalTrigger: boolean;
    readonly isSignalQueueFull: boolean;
    readonly isArithmeticErrorOverflow: boolean;
    readonly isArithmeticErrorUnderflow: boolean;
    readonly isArithmeticErrorDivisionByZero: boolean;
    readonly type: 'UpdateAttemptDoubleRevert' | 'UpdateAttemptDoubleKill' | 'UpdateStateTransitionDisallowed' | 'UpdateForcedStateTransitionDisallowed' | 'UpdateXtxTriggeredWithUnexpectedStatus' | 'ConfirmationFailed' | 'ApplyTriggeredWithUnexpectedStatus' | 'BidderNotEnoughBalance' | 'RequesterNotEnoughBalance' | 'SanityAfterCreatingSFXDepositsFailed' | 'ContractXtxKilledRunOutOfFunds' | 'ChargingTransferFailed' | 'ChargingTransferFailedAtPendingExecution' | 'XtxChargeFailedRequesterBalanceTooLow' | 'XtxChargeBondDepositFailedCantAccessBid' | 'FinalizeSquareUpFailed' | 'CriticalStateSquareUpCalledToFinishWithoutFsxConfirmed' | 'RewardTransferFailed' | 'RefundTransferFailed' | 'SideEffectsValidationFailed' | 'InsuranceBondNotRequired' | 'BiddingInactive' | 'BiddingRejectedBidBelowDust' | 'BiddingRejectedBidTooHigh' | 'BiddingRejectedInsuranceTooLow' | 'BiddingRejectedBetterBidFound' | 'BiddingRejectedFailedToDepositBidderBond' | 'BiddingFailedExecutorsBalanceTooLowToReserve' | 'InsuranceBondAlreadyDeposited' | 'InvalidFTXStateEmptyBidForReadyXtx' | 'InvalidFTXStateEmptyConfirmationForFinishedXtx' | 'InvalidFTXStateUnassignedExecutorForReadySFX' | 'InvalidFTXStateIncorrectExecutorForReadySFX' | 'SetupFailed' | 'SetupFailedXtxNotFound' | 'SetupFailedXtxStorageArtifactsNotFound' | 'SetupFailedIncorrectXtxStatus' | 'SetupFailedDuplicatedXtx' | 'SetupFailedEmptyXtx' | 'SetupFailedXtxAlreadyFinished' | 'SetupFailedXtxWasDroppedAtBidding' | 'SetupFailedXtxReverted' | 'SetupFailedXtxRevertedTimeout' | 'XtxDoesNotExist' | 'InvalidFSXBidStateLocated' | 'EnactSideEffectsCanOnlyBeCalledWithMin1StepFinished' | 'FatalXtxTimeoutXtxIdNotMatched' | 'RelayEscrowedFailedNothingToConfirm' | 'FatalCommitSideEffectWithoutConfirmationAttempt' | 'FatalErroredCommitSideEffectConfirmationAttempt' | 'FatalErroredRevertSideEffectConfirmationAttempt' | 'FailedToHardenFullSideEffect' | 'ApplyFailed' | 'DeterminedForbiddenXtxStatus' | 'SideEffectIsAlreadyScheduledToExecuteOverXBI' | 'FsxNotFoundById' | 'LocalSideEffectExecutionNotApplicable' | 'LocalExecutionUnauthorized' | 'OnLocalTriggerFailedToSetupXtx' | 'UnauthorizedCancellation' | 'FailedToConvertSFX2XBI' | 'FailedToCheckInOverXBI' | 'FailedToCreateXBIMetadataDueToWrongAccountConversion' | 'FailedToConvertXBIResult2SFXConfirmation' | 'FailedToEnterXBIPortal' | 'FailedToExitXBIPortal' | 'XbiExitFailedOnSFXConfirmation' | 'UnsupportedRole' | 'InvalidLocalTrigger' | 'SignalQueueFull' | 'ArithmeticErrorOverflow' | 'ArithmeticErrorUnderflow' | 'ArithmeticErrorDivisionByZero';
  }

  /** @name T3rnPrimitivesCommonRoundInfo (292) */
  export interface T3rnPrimitivesCommonRoundInfo extends Struct {
    readonly index: u32;
    readonly head: u32;
    readonly term: u32;
  }

  /** @name T3rnPrimitivesClaimableClaimableArtifacts (294) */
  export interface T3rnPrimitivesClaimableClaimableArtifacts extends Struct {
    readonly beneficiary: AccountId32;
    readonly role: T3rnPrimitivesClaimableCircuitRole;
    readonly totalRoundClaim: u128;
    readonly benefitSource: T3rnPrimitivesClaimableBenefitSource;
  }

  /** @name PalletClockError (295) */
  export type PalletClockError = Null;

  /** @name Pallet3vmError (297) */
  export interface Pallet3vmError extends Enum {
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
    readonly type: 'ExceededSignalBounceThreshold' | 'CannotTriggerWithoutSideEffects' | 'ContractNotFound' | 'InvalidOrigin' | 'CannotInstantiateContract' | 'ContractCannotRemunerate' | 'ContractCannotHaveStorage' | 'ContractCannotGenerateSideEffects' | 'InvalidPrecompilePointer' | 'InvalidPrecompileArgs' | 'InvalidArithmeticOverflow';
  }

  /** @name PalletContractsWasmPrefabWasmModule (298) */
  export interface PalletContractsWasmPrefabWasmModule extends Struct {
    readonly instructionWeightsVersion: Compact<u32>;
    readonly initial: Compact<u32>;
    readonly maximum: Compact<u32>;
    readonly code: Bytes;
    readonly author: Option<T3rnPrimitivesContractsRegistryAuthorInfo>;
    readonly kind: T3rnPrimitivesContractMetadataContractType;
  }

  /** @name PalletContractsWasmOwnerInfo (300) */
  export interface PalletContractsWasmOwnerInfo extends Struct {
    readonly owner: AccountId32;
    readonly deposit: Compact<u128>;
    readonly refcount: Compact<u64>;
  }

  /** @name PalletContractsStorageRawContractInfo (301) */
  export interface PalletContractsStorageRawContractInfo extends Struct {
    readonly trieId: Bytes;
    readonly codeHash: H256;
    readonly storageDeposit: u128;
  }

  /** @name PalletContractsStorageDeletedContract (303) */
  export interface PalletContractsStorageDeletedContract extends Struct {
    readonly trieId: Bytes;
  }

  /** @name PalletContractsSchedule (304) */
  export interface PalletContractsSchedule extends Struct {
    readonly limits: PalletContractsScheduleLimits;
    readonly instructionWeights: PalletContractsScheduleInstructionWeights;
    readonly hostFnWeights: PalletContractsScheduleHostFnWeights;
  }

  /** @name PalletContractsScheduleLimits (305) */
  export interface PalletContractsScheduleLimits extends Struct {
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

  /** @name PalletContractsScheduleInstructionWeights (306) */
  export interface PalletContractsScheduleInstructionWeights extends Struct {
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

  /** @name PalletContractsScheduleHostFnWeights (307) */
  export interface PalletContractsScheduleHostFnWeights extends Struct {
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

  /** @name PalletContractsError (308) */
  export interface PalletContractsError extends Enum {
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

  /** @name PalletEvmThreeVmInfo (310) */
  export interface PalletEvmThreeVmInfo extends Struct {
    readonly author: T3rnPrimitivesContractsRegistryAuthorInfo;
    readonly kind: T3rnPrimitivesContractMetadataContractType;
  }

  /** @name PalletEvmError (311) */
  export interface PalletEvmError extends Enum {
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

  /** @name T3rnPrimitivesAccountManagerRequestCharge (312) */
  export interface T3rnPrimitivesAccountManagerRequestCharge extends Struct {
    readonly payee: AccountId32;
    readonly offeredReward: u128;
    readonly maybeAssetId: Option<u32>;
    readonly chargeFee: u128;
    readonly recipient: Option<AccountId32>;
    readonly source: T3rnPrimitivesClaimableBenefitSource;
    readonly role: T3rnPrimitivesClaimableCircuitRole;
  }

  /** @name T3rnPrimitivesAccountManagerSettlement (314) */
  export interface T3rnPrimitivesAccountManagerSettlement extends Struct {
    readonly requester: AccountId32;
    readonly recipient: AccountId32;
    readonly settlementAmount: u128;
    readonly outcome: T3rnPrimitivesAccountManagerOutcome;
    readonly source: T3rnPrimitivesClaimableBenefitSource;
    readonly role: T3rnPrimitivesClaimableCircuitRole;
  }

  /** @name PalletAccountManagerError (315) */
  export interface PalletAccountManagerError extends Enum {
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

  /** @name PalletPortalError (316) */
  export interface PalletPortalError extends Enum {
    readonly isXdnsRecordCreationFailed: boolean;
    readonly isUnimplementedGatewayVendor: boolean;
    readonly isRegistrationError: boolean;
    readonly isGatewayVendorNotFound: boolean;
    readonly isSetOwnerError: boolean;
    readonly isSetOperationalError: boolean;
    readonly isSubmitHeaderError: boolean;
    readonly isNoGatewayHeightAvailable: boolean;
    readonly isSideEffectConfirmationFailed: boolean;
    readonly type: 'XdnsRecordCreationFailed' | 'UnimplementedGatewayVendor' | 'RegistrationError' | 'GatewayVendorNotFound' | 'SetOwnerError' | 'SetOperationalError' | 'SubmitHeaderError' | 'NoGatewayHeightAvailable' | 'SideEffectConfirmationFailed';
  }

  /** @name PalletGrandpaFinalityVerifierBridgesHeaderChainAuthoritySet (320) */
  export interface PalletGrandpaFinalityVerifierBridgesHeaderChainAuthoritySet extends Struct {
    readonly authorities: Vec<ITuple<[SpFinalityGrandpaAppPublic, u64]>>;
    readonly setId: u64;
  }

  /** @name PalletGrandpaFinalityVerifierParachain (321) */
  export interface PalletGrandpaFinalityVerifierParachain extends Struct {
    readonly relayChainId: U8aFixed;
    readonly id: u32;
  }

  /** @name PalletGrandpaFinalityVerifierError (322) */
  export interface PalletGrandpaFinalityVerifierError extends Enum {
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

  /** @name SpRuntimeMultiSignature (324) */
  export interface SpRuntimeMultiSignature extends Enum {
    readonly isEd25519: boolean;
    readonly asEd25519: SpCoreEd25519Signature;
    readonly isSr25519: boolean;
    readonly asSr25519: SpCoreSr25519Signature;
    readonly isEcdsa: boolean;
    readonly asEcdsa: SpCoreEcdsaSignature;
    readonly type: 'Ed25519' | 'Sr25519' | 'Ecdsa';
  }

  /** @name SpCoreSr25519Signature (325) */
  export interface SpCoreSr25519Signature extends U8aFixed {}

  /** @name SpCoreEcdsaSignature (326) */
  export interface SpCoreEcdsaSignature extends U8aFixed {}

  /** @name FrameSystemExtensionsCheckNonZeroSender (329) */
  export type FrameSystemExtensionsCheckNonZeroSender = Null;

  /** @name FrameSystemExtensionsCheckSpecVersion (330) */
  export type FrameSystemExtensionsCheckSpecVersion = Null;

  /** @name FrameSystemExtensionsCheckTxVersion (331) */
  export type FrameSystemExtensionsCheckTxVersion = Null;

  /** @name FrameSystemExtensionsCheckGenesis (332) */
  export type FrameSystemExtensionsCheckGenesis = Null;

  /** @name FrameSystemExtensionsCheckNonce (335) */
  export interface FrameSystemExtensionsCheckNonce extends Compact<u32> {}

  /** @name FrameSystemExtensionsCheckWeight (336) */
  export type FrameSystemExtensionsCheckWeight = Null;

  /** @name PalletAssetTxPaymentChargeAssetTxPayment (337) */
  export interface PalletAssetTxPaymentChargeAssetTxPayment extends Struct {
    readonly tip: Compact<u128>;
    readonly assetId: Option<u32>;
  }

  /** @name CircuitStandaloneRuntimeRuntime (338) */
  export type CircuitStandaloneRuntimeRuntime = Null;

} // declare module
