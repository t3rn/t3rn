// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/types/lookup';

import type { Data } from '@polkadot/types';
import type { BTreeMap, Bytes, Compact, Enum, Null, Option, Result, Set, Struct, Text, U256, U8aFixed, Vec, bool, u128, u16, u32, u64, u8 } from '@polkadot/types-codec';
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

  /** @name PalletSudoEvent (27) */
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

  /** @name PalletGrandpaEvent (31) */
  interface PalletGrandpaEvent extends Enum {
    readonly isNewAuthorities: boolean;
    readonly asNewAuthorities: {
      readonly authoritySet: Vec<ITuple<[SpFinalityGrandpaAppPublic, u64]>>;
    } & Struct;
    readonly isPaused: boolean;
    readonly isResumed: boolean;
    readonly type: 'NewAuthorities' | 'Paused' | 'Resumed';
  }

  /** @name SpFinalityGrandpaAppPublic (34) */
  interface SpFinalityGrandpaAppPublic extends SpCoreEd25519Public {}

  /** @name SpCoreEd25519Public (35) */
  interface SpCoreEd25519Public extends U8aFixed {}

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
    readonly type: 'BatchInterrupted' | 'BatchCompleted' | 'BatchCompletedWithErrors' | 'ItemCompleted' | 'ItemFailed' | 'DispatchedAs';
  }

  /** @name PalletIdentityEvent (37) */
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

  /** @name PalletBalancesEvent (38) */
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

  /** @name FrameSupportTokensMiscBalanceStatus (39) */
  interface FrameSupportTokensMiscBalanceStatus extends Enum {
    readonly isFree: boolean;
    readonly isReserved: boolean;
    readonly type: 'Free' | 'Reserved';
  }

  /** @name PalletTransactionPaymentEvent (40) */
  interface PalletTransactionPaymentEvent extends Enum {
    readonly isTransactionFeePaid: boolean;
    readonly asTransactionFeePaid: {
      readonly who: AccountId32;
      readonly actualFee: u128;
      readonly tip: u128;
    } & Struct;
    readonly type: 'TransactionFeePaid';
  }

  /** @name PalletAssetsEvent (41) */
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

  /** @name PalletTreasuryEvent (43) */
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

  /** @name PalletXdnsEvent (48) */
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

  /** @name PalletAttestersEvent (49) */
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

  /** @name PalletAttestersBatchMessage (50) */
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

  /** @name PalletAttestersBatchStatus (58) */
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

  /** @name T3rnPrimitivesAttestersLatencyStatus (59) */
  interface T3rnPrimitivesAttestersLatencyStatus extends Enum {
    readonly isOnTime: boolean;
    readonly isLate: boolean;
    readonly asLate: ITuple<[u32, u32]>;
    readonly type: 'OnTime' | 'Late';
  }

  /** @name T3rnPrimitivesExecutionVendor (60) */
  interface T3rnPrimitivesExecutionVendor extends Enum {
    readonly isSubstrate: boolean;
    readonly isEvm: boolean;
    readonly type: 'Substrate' | 'Evm';
  }

  /** @name PalletRewardsEvent (63) */
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

  /** @name PalletContractsRegistryEvent (68) */
  interface PalletContractsRegistryEvent extends Enum {
    readonly isContractStored: boolean;
    readonly asContractStored: ITuple<[AccountId32, H256]>;
    readonly isContractPurged: boolean;
    readonly asContractPurged: ITuple<[AccountId32, H256]>;
    readonly type: 'ContractStored' | 'ContractPurged';
  }

  /** @name PalletCircuitEvent (69) */
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

  /** @name XpFormatXbiResult (78) */
  interface XpFormatXbiResult extends Struct {
    readonly status: XpFormatStatus;
    readonly output: Bytes;
    readonly witness: Bytes;
  }

  /** @name XpFormatStatus (79) */
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

  /** @name T3rnTypesSfxSideEffect (81) */
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

  /** @name T3rnTypesFsxFullSideEffect (84) */
  interface T3rnTypesFsxFullSideEffect extends Struct {
    readonly input: T3rnTypesSfxSideEffect;
    readonly confirmed: Option<T3rnTypesSfxConfirmedSideEffect>;
    readonly securityLvl: T3rnTypesSfxSecurityLvl;
    readonly submissionTargetHeight: u32;
    readonly bestBid: Option<T3rnTypesBidSfxBid>;
    readonly index: u32;
  }

  /** @name T3rnTypesSfxConfirmedSideEffect (86) */
  interface T3rnTypesSfxConfirmedSideEffect extends Struct {
    readonly err: Option<T3rnTypesSfxConfirmationOutcome>;
    readonly output: Option<Bytes>;
    readonly inclusionData: Bytes;
    readonly executioner: AccountId32;
    readonly receivedAt: u32;
    readonly cost: Option<u128>;
  }

  /** @name T3rnTypesSfxConfirmationOutcome (88) */
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

  /** @name T3rnTypesSfxSecurityLvl (90) */
  interface T3rnTypesSfxSecurityLvl extends Enum {
    readonly isOptimistic: boolean;
    readonly isEscrow: boolean;
    readonly type: 'Optimistic' | 'Escrow';
  }

  /** @name T3rnTypesBidSfxBid (92) */
  interface T3rnTypesBidSfxBid extends Struct {
    readonly amount: u128;
    readonly insurance: u128;
    readonly reservedBond: Option<u128>;
    readonly rewardAssetId: Option<u32>;
    readonly executor: AccountId32;
    readonly requester: AccountId32;
  }

  /** @name PalletClockEvent (93) */
  interface PalletClockEvent extends Enum {
    readonly isNewRound: boolean;
    readonly asNewRound: {
      readonly index: u32;
      readonly head: u32;
      readonly term: u32;
    } & Struct;
    readonly type: 'NewRound';
  }

  /** @name PalletCircuitVacuumEvent (94) */
  interface PalletCircuitVacuumEvent extends Enum {
    readonly isOrderStatusRead: boolean;
    readonly asOrderStatusRead: PalletCircuitVacuumOrderStatusRead;
    readonly type: 'OrderStatusRead';
  }

  /** @name PalletCircuitVacuumOrderStatusRead (95) */
  interface PalletCircuitVacuumOrderStatusRead extends Struct {
    readonly xtxId: H256;
    readonly status: T3rnPrimitivesCircuitTypesCircuitStatus;
    readonly allIncludedSfx: Vec<ITuple<[H256, T3rnPrimitivesCircuitTypesCircuitStatus]>>;
    readonly timeoutsAt: T3rnPrimitivesCircuitTypesAdaptiveTimeout;
  }

  /** @name T3rnPrimitivesCircuitTypesCircuitStatus (96) */
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

  /** @name T3rnPrimitivesCircuitTypesCause (97) */
  interface T3rnPrimitivesCircuitTypesCause extends Enum {
    readonly isTimeout: boolean;
    readonly isIntentionalKill: boolean;
    readonly type: 'Timeout' | 'IntentionalKill';
  }

  /** @name T3rnPrimitivesCircuitTypesAdaptiveTimeout (100) */
  interface T3rnPrimitivesCircuitTypesAdaptiveTimeout extends Struct {
    readonly estimatedHeightHere: u32;
    readonly estimatedHeightThere: u32;
    readonly submitByHeightHere: u32;
    readonly submitByHeightThere: u32;
    readonly emergencyTimeoutHere: u32;
    readonly there: U8aFixed;
    readonly dlq: Option<u32>;
  }

  /** @name Pallet3vmEvent (101) */
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

  /** @name T3rnSdkPrimitivesSignalSignalKind (103) */
  interface T3rnSdkPrimitivesSignalSignalKind extends Enum {
    readonly isComplete: boolean;
    readonly isKill: boolean;
    readonly asKill: T3rnSdkPrimitivesSignalKillReason;
    readonly type: 'Complete' | 'Kill';
  }

  /** @name T3rnSdkPrimitivesSignalKillReason (104) */
  interface T3rnSdkPrimitivesSignalKillReason extends Enum {
    readonly isUnhandled: boolean;
    readonly isCodec: boolean;
    readonly isTimeout: boolean;
    readonly type: 'Unhandled' | 'Codec' | 'Timeout';
  }

  /** @name T3rnPrimitivesContractMetadataContractType (106) */
  interface T3rnPrimitivesContractMetadataContractType extends Enum {
    readonly isSystem: boolean;
    readonly isVanillaEvm: boolean;
    readonly isVanillaWasm: boolean;
    readonly isVolatileEvm: boolean;
    readonly isVolatileWasm: boolean;
    readonly type: 'System' | 'VanillaEvm' | 'VanillaWasm' | 'VolatileEvm' | 'VolatileWasm';
  }

  /** @name PalletContractsEvent (108) */
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

  /** @name PalletEvmEvent (109) */
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

  /** @name EthereumLog (110) */
  interface EthereumLog extends Struct {
    readonly address: H160;
    readonly topics: Vec<H256>;
    readonly data: Bytes;
  }

  /** @name PalletAccountManagerEvent (111) */
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

  /** @name PalletPortalEvent (112) */
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

  /** @name T3rnPrimitivesGatewayVendor (113) */
  interface T3rnPrimitivesGatewayVendor extends Enum {
    readonly isPolkadot: boolean;
    readonly isKusama: boolean;
    readonly isRococo: boolean;
    readonly isEthereum: boolean;
    readonly type: 'Polkadot' | 'Kusama' | 'Rococo' | 'Ethereum';
  }

  /** @name PalletGrandpaFinalityVerifierEvent (114) */
  interface PalletGrandpaFinalityVerifierEvent extends Enum {
    readonly isHeadersAdded: boolean;
    readonly asHeadersAdded: u32;
    readonly type: 'HeadersAdded';
  }

  /** @name PalletEth2FinalityVerifierEvent (117) */
  interface PalletEth2FinalityVerifierEvent extends Enum {
    readonly isEpochUpdate: boolean;
    readonly asEpochUpdate: PalletEth2FinalityVerifierEpochSubmitted;
    readonly type: 'EpochUpdate';
  }

  /** @name PalletEth2FinalityVerifierEpochSubmitted (118) */
  interface PalletEth2FinalityVerifierEpochSubmitted extends Struct {
    readonly epoch: u64;
    readonly beaconHeight: u64;
    readonly executionHeight: u64;
  }

  /** @name PalletMaintenanceModeEvent (119) */
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

  /** @name FrameSystemPhase (120) */
  interface FrameSystemPhase extends Enum {
    readonly isApplyExtrinsic: boolean;
    readonly asApplyExtrinsic: u32;
    readonly isFinalization: boolean;
    readonly isInitialization: boolean;
    readonly type: 'ApplyExtrinsic' | 'Finalization' | 'Initialization';
  }

  /** @name FrameSystemLastRuntimeUpgradeInfo (123) */
  interface FrameSystemLastRuntimeUpgradeInfo extends Struct {
    readonly specVersion: Compact<u32>;
    readonly specName: Text;
  }

  /** @name FrameSystemCall (126) */
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

  /** @name FrameSystemLimitsBlockWeights (130) */
  interface FrameSystemLimitsBlockWeights extends Struct {
    readonly baseBlock: u64;
    readonly maxBlock: u64;
    readonly perClass: FrameSupportWeightsPerDispatchClassWeightsPerClass;
  }

  /** @name FrameSupportWeightsPerDispatchClassWeightsPerClass (131) */
  interface FrameSupportWeightsPerDispatchClassWeightsPerClass extends Struct {
    readonly normal: FrameSystemLimitsWeightsPerClass;
    readonly operational: FrameSystemLimitsWeightsPerClass;
    readonly mandatory: FrameSystemLimitsWeightsPerClass;
  }

  /** @name FrameSystemLimitsWeightsPerClass (132) */
  interface FrameSystemLimitsWeightsPerClass extends Struct {
    readonly baseExtrinsic: u64;
    readonly maxExtrinsic: Option<u64>;
    readonly maxTotal: Option<u64>;
    readonly reserved: Option<u64>;
  }

  /** @name FrameSystemLimitsBlockLength (134) */
  interface FrameSystemLimitsBlockLength extends Struct {
    readonly max: FrameSupportWeightsPerDispatchClassU32;
  }

  /** @name FrameSupportWeightsPerDispatchClassU32 (135) */
  interface FrameSupportWeightsPerDispatchClassU32 extends Struct {
    readonly normal: u32;
    readonly operational: u32;
    readonly mandatory: u32;
  }

  /** @name FrameSupportWeightsRuntimeDbWeight (136) */
  interface FrameSupportWeightsRuntimeDbWeight extends Struct {
    readonly read: u64;
    readonly write: u64;
  }

  /** @name SpVersionRuntimeVersion (137) */
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

  /** @name FrameSystemError (143) */
  interface FrameSystemError extends Enum {
    readonly isInvalidSpecName: boolean;
    readonly isSpecVersionNeedsToIncrease: boolean;
    readonly isFailedToExtractRuntimeVersion: boolean;
    readonly isNonDefaultComposite: boolean;
    readonly isNonZeroRefCount: boolean;
    readonly isCallFiltered: boolean;
    readonly type: 'InvalidSpecName' | 'SpecVersionNeedsToIncrease' | 'FailedToExtractRuntimeVersion' | 'NonDefaultComposite' | 'NonZeroRefCount' | 'CallFiltered';
  }

  /** @name PalletSudoCall (144) */
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

  /** @name PalletTimestampCall (146) */
  interface PalletTimestampCall extends Enum {
    readonly isSet: boolean;
    readonly asSet: {
      readonly now: Compact<u64>;
    } & Struct;
    readonly type: 'Set';
  }

  /** @name PalletGrandpaCall (148) */
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
    readonly type: 'ReportEquivocation' | 'ReportEquivocationUnsigned' | 'NoteStalled';
  }

  /** @name SpFinalityGrandpaEquivocationProof (149) */
  interface SpFinalityGrandpaEquivocationProof extends Struct {
    readonly setId: u64;
    readonly equivocation: SpFinalityGrandpaEquivocation;
  }

  /** @name SpFinalityGrandpaEquivocation (150) */
  interface SpFinalityGrandpaEquivocation extends Enum {
    readonly isPrevote: boolean;
    readonly asPrevote: FinalityGrandpaEquivocationPrevote;
    readonly isPrecommit: boolean;
    readonly asPrecommit: FinalityGrandpaEquivocationPrecommit;
    readonly type: 'Prevote' | 'Precommit';
  }

  /** @name FinalityGrandpaEquivocationPrevote (151) */
  interface FinalityGrandpaEquivocationPrevote extends Struct {
    readonly roundNumber: u64;
    readonly identity: SpFinalityGrandpaAppPublic;
    readonly first: ITuple<[FinalityGrandpaPrevote, SpFinalityGrandpaAppSignature]>;
    readonly second: ITuple<[FinalityGrandpaPrevote, SpFinalityGrandpaAppSignature]>;
  }

  /** @name FinalityGrandpaPrevote (152) */
  interface FinalityGrandpaPrevote extends Struct {
    readonly targetHash: H256;
    readonly targetNumber: u32;
  }

  /** @name SpFinalityGrandpaAppSignature (153) */
  interface SpFinalityGrandpaAppSignature extends SpCoreEd25519Signature {}

  /** @name SpCoreEd25519Signature (154) */
  interface SpCoreEd25519Signature extends U8aFixed {}

  /** @name FinalityGrandpaEquivocationPrecommit (157) */
  interface FinalityGrandpaEquivocationPrecommit extends Struct {
    readonly roundNumber: u64;
    readonly identity: SpFinalityGrandpaAppPublic;
    readonly first: ITuple<[FinalityGrandpaPrecommit, SpFinalityGrandpaAppSignature]>;
    readonly second: ITuple<[FinalityGrandpaPrecommit, SpFinalityGrandpaAppSignature]>;
  }

  /** @name FinalityGrandpaPrecommit (158) */
  interface FinalityGrandpaPrecommit extends Struct {
    readonly targetHash: H256;
    readonly targetNumber: u32;
  }

  /** @name SpCoreVoid (160) */
  type SpCoreVoid = Null;

  /** @name PalletUtilityCall (161) */
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
    readonly type: 'Batch' | 'AsDerivative' | 'BatchAll' | 'DispatchAs' | 'ForceBatch';
  }

  /** @name CircuitStandaloneRuntimeOriginCaller (163) */
  interface CircuitStandaloneRuntimeOriginCaller extends Enum {
    readonly isSystem: boolean;
    readonly asSystem: FrameSupportDispatchRawOrigin;
    readonly isVoid: boolean;
    readonly type: 'System' | 'Void';
  }

  /** @name FrameSupportDispatchRawOrigin (164) */
  interface FrameSupportDispatchRawOrigin extends Enum {
    readonly isRoot: boolean;
    readonly isSigned: boolean;
    readonly asSigned: AccountId32;
    readonly isNone: boolean;
    readonly type: 'Root' | 'Signed' | 'None';
  }

  /** @name PalletIdentityCall (165) */
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

  /** @name PalletIdentityIdentityInfo (166) */
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

  /** @name PalletIdentityBitFlags (204) */
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

  /** @name PalletIdentityIdentityField (205) */
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

  /** @name PalletIdentityJudgement (208) */
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

  /** @name PalletBalancesCall (209) */
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

  /** @name PalletAssetsCall (210) */
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

  /** @name PalletAssetsDestroyWitness (211) */
  interface PalletAssetsDestroyWitness extends Struct {
    readonly accounts: Compact<u32>;
    readonly sufficients: Compact<u32>;
    readonly approvals: Compact<u32>;
  }

  /** @name PalletAuthorshipCall (212) */
  interface PalletAuthorshipCall extends Enum {
    readonly isSetUncles: boolean;
    readonly asSetUncles: {
      readonly newUncles: Vec<SpRuntimeHeader>;
    } & Struct;
    readonly type: 'SetUncles';
  }

  /** @name SpRuntimeHeader (214) */
  interface SpRuntimeHeader extends Struct {
    readonly parentHash: H256;
    readonly number: Compact<u32>;
    readonly stateRoot: H256;
    readonly extrinsicsRoot: H256;
    readonly digest: SpRuntimeDigest;
  }

  /** @name SpRuntimeBlakeTwo256 (215) */
  type SpRuntimeBlakeTwo256 = Null;

  /** @name PalletTreasuryCall (216) */
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

  /** @name PalletXdnsCall (221) */
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

  /** @name PalletAttestersCall (222) */
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

  /** @name PalletRewardsCall (225) */
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

  /** @name T3rnPrimitivesClaimableCircuitRole (227) */
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

  /** @name PalletContractsRegistryCall (228) */
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

  /** @name T3rnPrimitivesContractsRegistryRegistryContract (229) */
  interface T3rnPrimitivesContractsRegistryRegistryContract extends Struct {
    readonly codeTxt: Bytes;
    readonly bytes: Bytes;
    readonly author: T3rnPrimitivesContractsRegistryAuthorInfo;
    readonly abi: Option<Bytes>;
    readonly actionDescriptions: Vec<T3rnTypesGatewayContractActionDesc>;
    readonly info: Option<T3rnPrimitivesStorageRawAliveContractInfo>;
    readonly meta: T3rnPrimitivesContractMetadata;
  }

  /** @name T3rnPrimitivesContractsRegistryAuthorInfo (230) */
  interface T3rnPrimitivesContractsRegistryAuthorInfo extends Struct {
    readonly account: AccountId32;
    readonly feesPerSingleUse: Option<u128>;
  }

  /** @name T3rnTypesGatewayContractActionDesc (232) */
  interface T3rnTypesGatewayContractActionDesc extends Struct {
    readonly actionId: H256;
    readonly targetId: Option<U8aFixed>;
    readonly to: Option<AccountId32>;
  }

  /** @name T3rnPrimitivesStorageRawAliveContractInfo (235) */
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

  /** @name T3rnPrimitivesContractMetadata (237) */
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

  /** @name PalletCircuitCall (238) */
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

  /** @name T3rnPrimitivesSpeedMode (239) */
  interface T3rnPrimitivesSpeedMode extends Enum {
    readonly isFast: boolean;
    readonly isRational: boolean;
    readonly isFinalized: boolean;
    readonly type: 'Fast' | 'Rational' | 'Finalized';
  }

  /** @name PalletCircuitVacuumCall (240) */
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

  /** @name T3rnPrimitivesCircuitTypesOrderSFX (242) */
  interface T3rnPrimitivesCircuitTypesOrderSFX extends Struct {
    readonly sfxAction: T3rnPrimitivesCircuitTypesSfxAction;
    readonly maxReward: u128;
    readonly rewardAsset: u32;
    readonly insurance: u128;
    readonly remoteOriginNonce: Option<u32>;
  }

  /** @name T3rnPrimitivesCircuitTypesSfxAction (243) */
  interface T3rnPrimitivesCircuitTypesSfxAction extends Enum {
    readonly isCall: boolean;
    readonly asCall: ITuple<[U8aFixed, AccountId32, u128, u128, Bytes]>;
    readonly isTransfer: boolean;
    readonly asTransfer: ITuple<[U8aFixed, u32, AccountId32, u128]>;
    readonly type: 'Call' | 'Transfer';
  }

  /** @name Pallet3vmCall (244) */
  type Pallet3vmCall = Null;

  /** @name PalletContractsCall (245) */
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

  /** @name PalletEvmCall (247) */
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

  /** @name PalletAccountManagerCall (248) */
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

  /** @name T3rnPrimitivesClaimableBenefitSource (249) */
  interface T3rnPrimitivesClaimableBenefitSource extends Enum {
    readonly isBootstrapPool: boolean;
    readonly isInflation: boolean;
    readonly isTrafficFees: boolean;
    readonly isTrafficRewards: boolean;
    readonly isUnsettled: boolean;
    readonly isSlashTreasury: boolean;
    readonly type: 'BootstrapPool' | 'Inflation' | 'TrafficFees' | 'TrafficRewards' | 'Unsettled' | 'SlashTreasury';
  }

  /** @name T3rnPrimitivesAccountManagerOutcome (250) */
  interface T3rnPrimitivesAccountManagerOutcome extends Enum {
    readonly isUnexpectedFailure: boolean;
    readonly isRevert: boolean;
    readonly isCommit: boolean;
    readonly isSlash: boolean;
    readonly type: 'UnexpectedFailure' | 'Revert' | 'Commit' | 'Slash';
  }

  /** @name PalletPortalCall (251) */
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

  /** @name T3rnAbiRecodeCodec (252) */
  interface T3rnAbiRecodeCodec extends Enum {
    readonly isScale: boolean;
    readonly isRlp: boolean;
    readonly type: 'Scale' | 'Rlp';
  }

  /** @name T3rnPrimitivesTokenInfo (256) */
  interface T3rnPrimitivesTokenInfo extends Enum {
    readonly isSubstrate: boolean;
    readonly asSubstrate: T3rnPrimitivesSubstrateToken;
    readonly isEthereum: boolean;
    readonly asEthereum: T3rnPrimitivesEthereumToken;
    readonly type: 'Substrate' | 'Ethereum';
  }

  /** @name T3rnPrimitivesSubstrateToken (257) */
  interface T3rnPrimitivesSubstrateToken extends Struct {
    readonly id: u32;
    readonly symbol: Bytes;
    readonly decimals: u8;
  }

  /** @name T3rnPrimitivesEthereumToken (258) */
  interface T3rnPrimitivesEthereumToken extends Struct {
    readonly symbol: Bytes;
    readonly decimals: u8;
    readonly address: Option<U8aFixed>;
  }

  /** @name PalletGrandpaFinalityVerifierCall (259) */
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

  /** @name PalletGrandpaFinalityVerifierBridgesHeaderChainJustificationGrandpaJustification (260) */
  interface PalletGrandpaFinalityVerifierBridgesHeaderChainJustificationGrandpaJustification extends Struct {
    readonly round: u64;
    readonly commit: FinalityGrandpaCommit;
    readonly votesAncestries: Vec<SpRuntimeHeader>;
  }

  /** @name FinalityGrandpaCommit (261) */
  interface FinalityGrandpaCommit extends Struct {
    readonly targetHash: H256;
    readonly targetNumber: u32;
    readonly precommits: Vec<FinalityGrandpaSignedPrecommit>;
  }

  /** @name FinalityGrandpaSignedPrecommit (263) */
  interface FinalityGrandpaSignedPrecommit extends Struct {
    readonly precommit: FinalityGrandpaPrecommit;
    readonly signature: SpFinalityGrandpaAppSignature;
    readonly id: SpFinalityGrandpaAppPublic;
  }

  /** @name PalletEth2FinalityVerifierCall (266) */
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

  /** @name PalletEth2FinalityVerifierBeaconBlockHeader (267) */
  interface PalletEth2FinalityVerifierBeaconBlockHeader extends Struct {
    readonly slot: u64;
    readonly proposerIndex: u64;
    readonly parentRoot: U8aFixed;
    readonly stateRoot: U8aFixed;
    readonly bodyRoot: U8aFixed;
  }

  /** @name PalletEth2FinalityVerifierMerkleProof (270) */
  interface PalletEth2FinalityVerifierMerkleProof extends Struct {
    readonly gIndex: u64;
    readonly witness: Vec<U8aFixed>;
  }

  /** @name PalletEth2FinalityVerifierExecutionPayload (272) */
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

  /** @name EthbloomBloom (273) */
  interface EthbloomBloom extends U8aFixed {}

  /** @name PalletEth2FinalityVerifierExecutionHeader (277) */
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

  /** @name PalletEth2FinalityVerifierSyncCommittee (279) */
  interface PalletEth2FinalityVerifierSyncCommittee extends Struct {
    readonly pubs: Vec<U8aFixed>;
    readonly aggr: U8aFixed;
  }

  /** @name PalletEth2FinalityVerifierEthereumReceiptInclusionProof (282) */
  interface PalletEth2FinalityVerifierEthereumReceiptInclusionProof extends Struct {
    readonly blockNumber: u64;
    readonly witness: Vec<Bytes>;
    readonly index: Bytes;
  }

  /** @name PalletEth2FinalityVerifierEthereumEventInclusionProof (283) */
  interface PalletEth2FinalityVerifierEthereumEventInclusionProof extends Struct {
    readonly blockNumber: u64;
    readonly witness: Vec<Bytes>;
    readonly index: Bytes;
    readonly event: Bytes;
  }

  /** @name PalletMaintenanceModeCall (285) */
  interface PalletMaintenanceModeCall extends Enum {
    readonly isEnterMaintenanceMode: boolean;
    readonly isResumeNormalOperation: boolean;
    readonly type: 'EnterMaintenanceMode' | 'ResumeNormalOperation';
  }

  /** @name PalletSudoError (286) */
  interface PalletSudoError extends Enum {
    readonly isRequireSudo: boolean;
    readonly type: 'RequireSudo';
  }

  /** @name SpConsensusAuraSr25519AppSr25519Public (289) */
  interface SpConsensusAuraSr25519AppSr25519Public extends SpCoreSr25519Public {}

  /** @name SpCoreSr25519Public (290) */
  interface SpCoreSr25519Public extends U8aFixed {}

  /** @name PalletGrandpaStoredState (293) */
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
    readonly type: 'Live' | 'PendingPause' | 'Paused' | 'PendingResume';
  }

  /** @name PalletGrandpaStoredPendingChange (294) */
  interface PalletGrandpaStoredPendingChange extends Struct {
    readonly scheduledAt: u32;
    readonly delay: u32;
    readonly nextAuthorities: Vec<ITuple<[SpFinalityGrandpaAppPublic, u64]>>;
    readonly forced: Option<u32>;
  }

  /** @name PalletGrandpaError (296) */
  interface PalletGrandpaError extends Enum {
    readonly isPauseFailed: boolean;
    readonly isResumeFailed: boolean;
    readonly isChangePending: boolean;
    readonly isTooSoon: boolean;
    readonly isInvalidKeyOwnershipProof: boolean;
    readonly isInvalidEquivocationProof: boolean;
    readonly isDuplicateOffenceReport: boolean;
    readonly type: 'PauseFailed' | 'ResumeFailed' | 'ChangePending' | 'TooSoon' | 'InvalidKeyOwnershipProof' | 'InvalidEquivocationProof' | 'DuplicateOffenceReport';
  }

  /** @name PalletUtilityError (297) */
  interface PalletUtilityError extends Enum {
    readonly isTooManyCalls: boolean;
    readonly type: 'TooManyCalls';
  }

  /** @name PalletIdentityRegistration (298) */
  interface PalletIdentityRegistration extends Struct {
    readonly judgements: Vec<ITuple<[u32, PalletIdentityJudgement]>>;
    readonly deposit: u128;
    readonly info: PalletIdentityIdentityInfo;
  }

  /** @name PalletIdentityRegistrarInfo (307) */
  interface PalletIdentityRegistrarInfo extends Struct {
    readonly account: AccountId32;
    readonly fee: u128;
    readonly fields: PalletIdentityBitFlags;
  }

  /** @name PalletIdentityError (309) */
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

  /** @name PalletBalancesBalanceLock (311) */
  interface PalletBalancesBalanceLock extends Struct {
    readonly id: U8aFixed;
    readonly amount: u128;
    readonly reasons: PalletBalancesReasons;
  }

  /** @name PalletBalancesReasons (312) */
  interface PalletBalancesReasons extends Enum {
    readonly isFee: boolean;
    readonly isMisc: boolean;
    readonly isAll: boolean;
    readonly type: 'Fee' | 'Misc' | 'All';
  }

  /** @name PalletBalancesReserveData (315) */
  interface PalletBalancesReserveData extends Struct {
    readonly id: U8aFixed;
    readonly amount: u128;
  }

  /** @name PalletBalancesReleases (317) */
  interface PalletBalancesReleases extends Enum {
    readonly isV100: boolean;
    readonly isV200: boolean;
    readonly type: 'V100' | 'V200';
  }

  /** @name PalletBalancesError (318) */
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

  /** @name PalletTransactionPaymentReleases (320) */
  interface PalletTransactionPaymentReleases extends Enum {
    readonly isV1Ancient: boolean;
    readonly isV2: boolean;
    readonly type: 'V1Ancient' | 'V2';
  }

  /** @name PalletAssetsAssetDetails (321) */
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

  /** @name PalletAssetsAssetAccount (323) */
  interface PalletAssetsAssetAccount extends Struct {
    readonly balance: u128;
    readonly isFrozen: bool;
    readonly reason: PalletAssetsExistenceReason;
    readonly extra: Null;
  }

  /** @name PalletAssetsExistenceReason (324) */
  interface PalletAssetsExistenceReason extends Enum {
    readonly isConsumer: boolean;
    readonly isSufficient: boolean;
    readonly isDepositHeld: boolean;
    readonly asDepositHeld: u128;
    readonly isDepositRefunded: boolean;
    readonly type: 'Consumer' | 'Sufficient' | 'DepositHeld' | 'DepositRefunded';
  }

  /** @name PalletAssetsApproval (326) */
  interface PalletAssetsApproval extends Struct {
    readonly amount: u128;
    readonly deposit: u128;
  }

  /** @name PalletAssetsAssetMetadata (327) */
  interface PalletAssetsAssetMetadata extends Struct {
    readonly deposit: u128;
    readonly name: Bytes;
    readonly symbol: Bytes;
    readonly decimals: u8;
    readonly isFrozen: bool;
  }

  /** @name PalletAssetsError (329) */
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

  /** @name PalletAuthorshipUncleEntryItem (331) */
  interface PalletAuthorshipUncleEntryItem extends Enum {
    readonly isInclusionHeight: boolean;
    readonly asInclusionHeight: u32;
    readonly isUncle: boolean;
    readonly asUncle: ITuple<[H256, Option<AccountId32>]>;
    readonly type: 'InclusionHeight' | 'Uncle';
  }

  /** @name PalletAuthorshipError (333) */
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

  /** @name PalletTreasuryProposal (334) */
  interface PalletTreasuryProposal extends Struct {
    readonly proposer: AccountId32;
    readonly value: u128;
    readonly beneficiary: AccountId32;
    readonly bond: u128;
  }

  /** @name FrameSupportPalletId (338) */
  interface FrameSupportPalletId extends U8aFixed {}

  /** @name PalletTreasuryError (339) */
  interface PalletTreasuryError extends Enum {
    readonly isInsufficientProposersBalance: boolean;
    readonly isInvalidIndex: boolean;
    readonly isTooManyApprovals: boolean;
    readonly isInsufficientPermission: boolean;
    readonly isProposalNotApproved: boolean;
    readonly type: 'InsufficientProposersBalance' | 'InvalidIndex' | 'TooManyApprovals' | 'InsufficientPermission' | 'ProposalNotApproved';
  }

  /** @name T3rnAbiSfxAbi (344) */
  interface T3rnAbiSfxAbi extends Struct {
    readonly argsNames: Vec<ITuple<[Bytes, bool]>>;
    readonly maybePrefixMemo: Option<u8>;
    readonly egressAbiDescriptors: T3rnAbiSfxAbiPerCodecAbiDescriptors;
    readonly ingressAbiDescriptors: T3rnAbiSfxAbiPerCodecAbiDescriptors;
  }

  /** @name T3rnAbiSfxAbiPerCodecAbiDescriptors (347) */
  interface T3rnAbiSfxAbiPerCodecAbiDescriptors extends Struct {
    readonly forRlp: Bytes;
    readonly forScale: Bytes;
  }

  /** @name T3rnPrimitivesXdnsGatewayRecord (349) */
  interface T3rnPrimitivesXdnsGatewayRecord extends Struct {
    readonly gatewayId: U8aFixed;
    readonly verificationVendor: T3rnPrimitivesGatewayVendor;
    readonly executionVendor: T3rnPrimitivesExecutionVendor;
    readonly codec: T3rnAbiRecodeCodec;
    readonly registrant: Option<AccountId32>;
    readonly escrowAccount: Option<AccountId32>;
    readonly allowedSideEffects: Vec<ITuple<[U8aFixed, Option<u8>]>>;
  }

  /** @name T3rnPrimitivesXdnsTokenRecord (351) */
  interface T3rnPrimitivesXdnsTokenRecord extends Struct {
    readonly tokenId: u32;
    readonly gatewayId: U8aFixed;
    readonly tokenProps: T3rnPrimitivesTokenInfo;
  }

  /** @name T3rnPrimitivesGatewayActivity (355) */
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

  /** @name T3rnPrimitivesFinalityVerifierActivity (358) */
  interface T3rnPrimitivesFinalityVerifierActivity extends Struct {
    readonly verifier: T3rnPrimitivesGatewayVendor;
    readonly reportedAt: u32;
    readonly justifiedHeight: u32;
    readonly finalizedHeight: u32;
    readonly updatedHeight: u32;
    readonly epoch: u32;
    readonly isActive: bool;
  }

  /** @name T3rnPrimitivesXdnsEpochEstimate (360) */
  interface T3rnPrimitivesXdnsEpochEstimate extends Struct {
    readonly local: u32;
    readonly remote: u32;
    readonly movingAverageLocal: u32;
    readonly movingAverageRemote: u32;
  }

  /** @name PalletXdnsError (361) */
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

  /** @name T3rnPrimitivesAttestersAttesterInfo (362) */
  interface T3rnPrimitivesAttestersAttesterInfo extends Struct {
    readonly keyEd: U8aFixed;
    readonly keyEc: U8aFixed;
    readonly keySr: U8aFixed;
    readonly commission: Percent;
    readonly index: u32;
  }

  /** @name PalletAttestersError (371) */
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

  /** @name PalletRewardsAssetType (376) */
  interface PalletRewardsAssetType extends Enum {
    readonly isNative: boolean;
    readonly isNonNative: boolean;
    readonly asNonNative: u32;
    readonly type: 'Native' | 'NonNative';
  }

  /** @name PalletRewardsTreasuryBalanceSheet (377) */
  interface PalletRewardsTreasuryBalanceSheet extends Struct {
    readonly treasury: u128;
    readonly escrow: u128;
    readonly fee: u128;
    readonly slash: u128;
    readonly parachain: u128;
  }

  /** @name PalletRewardsDistributionRecord (379) */
  interface PalletRewardsDistributionRecord extends Struct {
    readonly blockNumber: u32;
    readonly attesterRewards: u128;
    readonly collatorRewards: u128;
    readonly executorRewards: u128;
    readonly treasuryRewards: u128;
    readonly available: u128;
    readonly distributed: u128;
  }

  /** @name T3rnPrimitivesCommonRoundInfo (380) */
  interface T3rnPrimitivesCommonRoundInfo extends Struct {
    readonly index: u32;
    readonly head: u32;
    readonly term: u32;
  }

  /** @name T3rnPrimitivesClaimableClaimableArtifacts (382) */
  interface T3rnPrimitivesClaimableClaimableArtifacts extends Struct {
    readonly beneficiary: AccountId32;
    readonly role: T3rnPrimitivesClaimableCircuitRole;
    readonly totalRoundClaim: u128;
    readonly nonNativeAssetId: Option<u32>;
    readonly benefitSource: T3rnPrimitivesClaimableBenefitSource;
  }

  /** @name PalletRewardsError (383) */
  interface PalletRewardsError extends Enum {
    readonly isDistributionPeriodNotElapsed: boolean;
    readonly isNoPendingClaims: boolean;
    readonly isArithmeticOverflow: boolean;
    readonly isAttesterNotFound: boolean;
    readonly isTryIntoConversionU128ToBalanceFailed: boolean;
    readonly isHalted: boolean;
    readonly type: 'DistributionPeriodNotElapsed' | 'NoPendingClaims' | 'ArithmeticOverflow' | 'AttesterNotFound' | 'TryIntoConversionU128ToBalanceFailed' | 'Halted';
  }

  /** @name PalletContractsRegistryError (384) */
  interface PalletContractsRegistryError extends Enum {
    readonly isContractAlreadyExists: boolean;
    readonly isUnknownContract: boolean;
    readonly type: 'ContractAlreadyExists' | 'UnknownContract';
  }

  /** @name T3rnPrimitivesCircuitTypesXExecSignal (385) */
  interface T3rnPrimitivesCircuitTypesXExecSignal extends Struct {
    readonly requester: AccountId32;
    readonly requesterNonce: u32;
    readonly timeoutsAt: T3rnPrimitivesCircuitTypesAdaptiveTimeout;
    readonly speedMode: T3rnPrimitivesSpeedMode;
    readonly delayStepsAt: Option<Vec<u32>>;
    readonly status: T3rnPrimitivesCircuitTypesCircuitStatus;
    readonly stepsCnt: ITuple<[u32, u32]>;
  }

  /** @name T3rnPrimitivesVolatileLocalState (387) */
  interface T3rnPrimitivesVolatileLocalState extends Struct {
    readonly state: BTreeMap<U8aFixed, Bytes>;
  }

  /** @name T3rnSdkPrimitivesSignalExecutionSignal (394) */
  interface T3rnSdkPrimitivesSignalExecutionSignal extends Struct {
    readonly step: u32;
    readonly kind: T3rnSdkPrimitivesSignalSignalKind;
    readonly executionId: H256;
  }

  /** @name PalletCircuitError (396) */
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

  /** @name PalletClockError (397) */
  type PalletClockError = Null;

  /** @name PalletCircuitVacuumError (398) */
  type PalletCircuitVacuumError = Null;

  /** @name Pallet3vmError (400) */
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

  /** @name PalletContractsWasmPrefabWasmModule (401) */
  interface PalletContractsWasmPrefabWasmModule extends Struct {
    readonly instructionWeightsVersion: Compact<u32>;
    readonly initial: Compact<u32>;
    readonly maximum: Compact<u32>;
    readonly code: Bytes;
    readonly author: Option<T3rnPrimitivesContractsRegistryAuthorInfo>;
    readonly kind: T3rnPrimitivesContractMetadataContractType;
  }

  /** @name PalletContractsWasmOwnerInfo (403) */
  interface PalletContractsWasmOwnerInfo extends Struct {
    readonly owner: AccountId32;
    readonly deposit: Compact<u128>;
    readonly refcount: Compact<u64>;
  }

  /** @name PalletContractsStorageRawContractInfo (404) */
  interface PalletContractsStorageRawContractInfo extends Struct {
    readonly trieId: Bytes;
    readonly codeHash: H256;
    readonly storageDeposit: u128;
  }

  /** @name PalletContractsStorageDeletedContract (406) */
  interface PalletContractsStorageDeletedContract extends Struct {
    readonly trieId: Bytes;
  }

  /** @name PalletContractsSchedule (407) */
  interface PalletContractsSchedule extends Struct {
    readonly limits: PalletContractsScheduleLimits;
    readonly instructionWeights: PalletContractsScheduleInstructionWeights;
    readonly hostFnWeights: PalletContractsScheduleHostFnWeights;
  }

  /** @name PalletContractsScheduleLimits (408) */
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

  /** @name PalletContractsScheduleInstructionWeights (409) */
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

  /** @name PalletContractsScheduleHostFnWeights (410) */
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

  /** @name PalletContractsError (411) */
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

  /** @name PalletEvmThreeVmInfo (413) */
  interface PalletEvmThreeVmInfo extends Struct {
    readonly author: T3rnPrimitivesContractsRegistryAuthorInfo;
    readonly kind: T3rnPrimitivesContractMetadataContractType;
  }

  /** @name PalletEvmError (414) */
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

  /** @name T3rnPrimitivesAccountManagerRequestCharge (415) */
  interface T3rnPrimitivesAccountManagerRequestCharge extends Struct {
    readonly payee: AccountId32;
    readonly offeredReward: u128;
    readonly maybeAssetId: Option<u32>;
    readonly chargeFee: u128;
    readonly recipient: Option<AccountId32>;
    readonly source: T3rnPrimitivesClaimableBenefitSource;
    readonly role: T3rnPrimitivesClaimableCircuitRole;
  }

  /** @name T3rnPrimitivesAccountManagerSettlement (417) */
  interface T3rnPrimitivesAccountManagerSettlement extends Struct {
    readonly requester: AccountId32;
    readonly recipient: AccountId32;
    readonly settlementAmount: u128;
    readonly maybeAssetId: Option<u32>;
    readonly outcome: T3rnPrimitivesAccountManagerOutcome;
    readonly source: T3rnPrimitivesClaimableBenefitSource;
    readonly role: T3rnPrimitivesClaimableCircuitRole;
  }

  /** @name PalletAccountManagerError (418) */
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

  /** @name PalletPortalError (419) */
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

  /** @name PalletGrandpaFinalityVerifierBridgesHeaderChainAuthoritySet (420) */
  interface PalletGrandpaFinalityVerifierBridgesHeaderChainAuthoritySet extends Struct {
    readonly authorities: Vec<ITuple<[SpFinalityGrandpaAppPublic, u64]>>;
    readonly setId: u64;
  }

  /** @name PalletGrandpaFinalityVerifierParachainRegistrationData (421) */
  interface PalletGrandpaFinalityVerifierParachainRegistrationData extends Struct {
    readonly relayGatewayId: U8aFixed;
    readonly id: u32;
  }

  /** @name PalletGrandpaFinalityVerifierError (422) */
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

  /** @name PalletEth2FinalityVerifierCheckpoint (425) */
  interface PalletEth2FinalityVerifierCheckpoint extends Struct {
    readonly attestedBeacon: PalletEth2FinalityVerifierBeaconCheckpoint;
    readonly attestedExecution: PalletEth2FinalityVerifierExecutionCheckpoint;
    readonly justifiedBeacon: PalletEth2FinalityVerifierBeaconCheckpoint;
    readonly justifiedExecution: PalletEth2FinalityVerifierExecutionCheckpoint;
    readonly finalizedBeacon: PalletEth2FinalityVerifierBeaconCheckpoint;
    readonly finalizedExecution: PalletEth2FinalityVerifierExecutionCheckpoint;
  }

  /** @name PalletEth2FinalityVerifierBeaconCheckpoint (426) */
  interface PalletEth2FinalityVerifierBeaconCheckpoint extends Struct {
    readonly epoch: u64;
    readonly root: U8aFixed;
  }

  /** @name PalletEth2FinalityVerifierExecutionCheckpoint (427) */
  interface PalletEth2FinalityVerifierExecutionCheckpoint extends Struct {
    readonly height: u64;
    readonly root: U8aFixed;
  }

  /** @name PalletEth2FinalityVerifierError (428) */
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

  /** @name PalletMaintenanceModeError (429) */
  interface PalletMaintenanceModeError extends Enum {
    readonly isAlreadyInMaintenanceMode: boolean;
    readonly isNotInMaintenanceMode: boolean;
    readonly type: 'AlreadyInMaintenanceMode' | 'NotInMaintenanceMode';
  }

  /** @name SpRuntimeMultiSignature (431) */
  interface SpRuntimeMultiSignature extends Enum {
    readonly isEd25519: boolean;
    readonly asEd25519: SpCoreEd25519Signature;
    readonly isSr25519: boolean;
    readonly asSr25519: SpCoreSr25519Signature;
    readonly isEcdsa: boolean;
    readonly asEcdsa: SpCoreEcdsaSignature;
    readonly type: 'Ed25519' | 'Sr25519' | 'Ecdsa';
  }

  /** @name SpCoreSr25519Signature (432) */
  interface SpCoreSr25519Signature extends U8aFixed {}

  /** @name SpCoreEcdsaSignature (433) */
  interface SpCoreEcdsaSignature extends U8aFixed {}

  /** @name FrameSystemExtensionsCheckNonZeroSender (435) */
  type FrameSystemExtensionsCheckNonZeroSender = Null;

  /** @name FrameSystemExtensionsCheckSpecVersion (436) */
  type FrameSystemExtensionsCheckSpecVersion = Null;

  /** @name FrameSystemExtensionsCheckTxVersion (437) */
  type FrameSystemExtensionsCheckTxVersion = Null;

  /** @name FrameSystemExtensionsCheckGenesis (438) */
  type FrameSystemExtensionsCheckGenesis = Null;

  /** @name FrameSystemExtensionsCheckNonce (441) */
  interface FrameSystemExtensionsCheckNonce extends Compact<u32> {}

  /** @name FrameSystemExtensionsCheckWeight (442) */
  type FrameSystemExtensionsCheckWeight = Null;

  /** @name PalletAssetTxPaymentChargeAssetTxPayment (443) */
  interface PalletAssetTxPaymentChargeAssetTxPayment extends Struct {
    readonly tip: Compact<u128>;
    readonly assetId: Option<u32>;
  }

  /** @name CircuitStandaloneRuntimeRuntime (444) */
  type CircuitStandaloneRuntimeRuntime = Null;

} // declare module
