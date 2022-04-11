// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

declare module "@polkadot/types/lookup" {
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
    H256,
    MultiAddress,
    Perbill,
  } from "@polkadot/types/interfaces/runtime";
  import type { Event } from "@polkadot/types/interfaces/system";

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
    readonly type:
      | "Other"
      | "Consensus"
      | "Seal"
      | "PreRuntime"
      | "RuntimeEnvironmentUpdated";
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
    readonly type:
      | "ExtrinsicSuccess"
      | "ExtrinsicFailed"
      | "CodeUpdated"
      | "NewAccount"
      | "KilledAccount"
      | "Remarked";
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
    readonly type: "Normal" | "Operational" | "Mandatory";
  }

  /** @name FrameSupportWeightsPays (21) */
  export interface FrameSupportWeightsPays extends Enum {
    readonly isYes: boolean;
    readonly isNo: boolean;
    readonly type: "Yes" | "No";
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
    readonly type:
      | "Other"
      | "CannotLookup"
      | "BadOrigin"
      | "Module"
      | "ConsumerRemaining"
      | "NoProviders"
      | "TooManyConsumers"
      | "Token"
      | "Arithmetic";
  }

  /** @name SpRuntimeModuleError (23) */
  export interface SpRuntimeModuleError extends Struct {
    readonly index: u8;
    readonly error: u8;
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
  export interface SpRuntimeArithmeticError extends Enum {
    readonly isUnderflow: boolean;
    readonly isOverflow: boolean;
    readonly isDivisionByZero: boolean;
    readonly type: "Underflow" | "Overflow" | "DivisionByZero";
  }

  /** @name PalletGrandpaEvent (26) */
  export interface PalletGrandpaEvent extends Enum {
    readonly isNewAuthorities: boolean;
    readonly asNewAuthorities: {
      readonly authoritySet: Vec<ITuple<[SpFinalityGrandpaAppPublic, u64]>>;
    } & Struct;
    readonly isPaused: boolean;
    readonly isResumed: boolean;
    readonly type: "NewAuthorities" | "Paused" | "Resumed";
  }

  /** @name SpFinalityGrandpaAppPublic (29) */
  export interface SpFinalityGrandpaAppPublic extends SpCoreEd25519Public {}

  /** @name SpCoreEd25519Public (30) */
  export interface SpCoreEd25519Public extends U8aFixed {}

  /** @name PalletBalancesEvent (31) */
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

  /** @name FrameSupportTokensMiscBalanceStatus (32) */
  export interface FrameSupportTokensMiscBalanceStatus extends Enum {
    readonly isFree: boolean;
    readonly isReserved: boolean;
    readonly type: "Free" | "Reserved";
  }

  /** @name PalletSudoEvent (33) */
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
    readonly type: "Sudid" | "KeyChanged" | "SudoAsDone";
  }

  /** @name OrmlTokensModuleEvent (37) */
  export interface OrmlTokensModuleEvent extends Enum {
    readonly isEndowed: boolean;
    readonly asEndowed: {
      readonly currencyId: u32;
      readonly who: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isDustLost: boolean;
    readonly asDustLost: {
      readonly currencyId: u32;
      readonly who: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isTransfer: boolean;
    readonly asTransfer: {
      readonly currencyId: u32;
      readonly from: AccountId32;
      readonly to: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isReserved: boolean;
    readonly asReserved: {
      readonly currencyId: u32;
      readonly who: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isUnreserved: boolean;
    readonly asUnreserved: {
      readonly currencyId: u32;
      readonly who: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isRepatriatedReserve: boolean;
    readonly asRepatriatedReserve: {
      readonly currencyId: u32;
      readonly from: AccountId32;
      readonly to: AccountId32;
      readonly amount: u128;
      readonly status: FrameSupportTokensMiscBalanceStatus;
    } & Struct;
    readonly isBalanceSet: boolean;
    readonly asBalanceSet: {
      readonly currencyId: u32;
      readonly who: AccountId32;
      readonly free: u128;
      readonly reserved: u128;
    } & Struct;
    readonly type:
      | "Endowed"
      | "DustLost"
      | "Transfer"
      | "Reserved"
      | "Unreserved"
      | "RepatriatedReserve"
      | "BalanceSet";
  }

  /** @name PalletXdnsEvent (38) */
  export interface PalletXdnsEvent extends Enum {
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

  /** @name PalletContractsRegistryEvent (39) */
  export interface PalletContractsRegistryEvent extends Enum {
    readonly isContractStored: boolean;
    readonly asContractStored: ITuple<[AccountId32, H256]>;
    readonly isContractPurged: boolean;
    readonly asContractPurged: ITuple<[AccountId32, H256]>;
    readonly type: "ContractStored" | "ContractPurged";
  }

  /** @name PalletCircuitPortalEvent (40) */
  export interface PalletCircuitPortalEvent extends Enum {
    readonly isNewGatewayRegistered: boolean;
    readonly asNewGatewayRegistered: ITuple<
      [
        U8aFixed,
        T3rnPrimitivesGatewayType,
        T3rnPrimitivesGatewayVendor,
        T3rnPrimitivesGatewaySysProps,
        Vec<U8aFixed>
      ]
    >;
    readonly isGatewayUpdated: boolean;
    readonly asGatewayUpdated: ITuple<[U8aFixed, Option<Vec<U8aFixed>>]>;
    readonly type: "NewGatewayRegistered" | "GatewayUpdated";
  }

  /** @name T3rnPrimitivesGatewayType (41) */
  export interface T3rnPrimitivesGatewayType extends Enum {
    readonly isProgrammableInternal: boolean;
    readonly asProgrammableInternal: u32;
    readonly isProgrammableExternal: boolean;
    readonly asProgrammableExternal: u32;
    readonly isTxOnly: boolean;
    readonly asTxOnly: u32;
    readonly type: "ProgrammableInternal" | "ProgrammableExternal" | "TxOnly";
  }

  /** @name T3rnPrimitivesGatewayVendor (42) */
  export interface T3rnPrimitivesGatewayVendor extends Enum {
    readonly isSubstrate: boolean;
    readonly isEthereum: boolean;
    readonly type: "Substrate" | "Ethereum";
  }

  /** @name T3rnPrimitivesGatewaySysProps (43) */
  export interface T3rnPrimitivesGatewaySysProps extends Struct {
    readonly ss58Format: u16;
    readonly tokenSymbol: Bytes;
    readonly tokenDecimals: u8;
  }

  /** @name PalletCircuitEvent (47) */
  export interface PalletCircuitEvent extends Enum {
    readonly isXTransactionReceivedForExec: boolean;
    readonly asXTransactionReceivedForExec: H256;
    readonly isXTransactionReadyForExec: boolean;
    readonly asXTransactionReadyForExec: H256;
    readonly isXTransactionFinishedExec: boolean;
    readonly asXTransactionFinishedExec: H256;
    readonly isNewSideEffectsAvailable: boolean;
    readonly asNewSideEffectsAvailable: ITuple<
      [AccountId32, H256, Vec<T3rnPrimitivesSideEffect>]
    >;
    readonly isCancelledSideEffects: boolean;
    readonly asCancelledSideEffects: ITuple<
      [AccountId32, H256, Vec<T3rnPrimitivesSideEffect>]
    >;
    readonly isSideEffectsConfirmed: boolean;
    readonly asSideEffectsConfirmed: ITuple<
      [H256, Vec<Vec<T3rnPrimitivesSideEffectFullSideEffect>>]
    >;
    readonly type:
      | "XTransactionReceivedForExec"
      | "XTransactionReadyForExec"
      | "XTransactionFinishedExec"
      | "NewSideEffectsAvailable"
      | "CancelledSideEffects"
      | "SideEffectsConfirmed";
  }

  /** @name T3rnPrimitivesSideEffect (49) */
  export interface T3rnPrimitivesSideEffect extends Struct {
    readonly target: U8aFixed;
    readonly prize: u128;
    readonly orderedAt: u32;
    readonly encodedAction: Bytes;
    readonly encodedArgs: Vec<Bytes>;
    readonly signature: Bytes;
    readonly enforceExecutioner: Option<AccountId32>;
  }

  /** @name T3rnPrimitivesSideEffectFullSideEffect (53) */
  export interface T3rnPrimitivesSideEffectFullSideEffect extends Struct {
    readonly input: T3rnPrimitivesSideEffect;
    readonly confirmed: Option<T3rnPrimitivesSideEffectConfirmedSideEffect>;
  }

  /** @name T3rnPrimitivesSideEffectConfirmedSideEffect (55) */
  export interface T3rnPrimitivesSideEffectConfirmedSideEffect extends Struct {
    readonly err: Option<Bytes>;
    readonly output: Option<Bytes>;
    readonly encodedEffect: Bytes;
    readonly inclusionProof: Option<Bytes>;
    readonly executioner: AccountId32;
    readonly receivedAt: u32;
    readonly cost: Option<u128>;
  }

  /** @name FrameSystemPhase (58) */
  export interface FrameSystemPhase extends Enum {
    readonly isApplyExtrinsic: boolean;
    readonly asApplyExtrinsic: u32;
    readonly isFinalization: boolean;
    readonly isInitialization: boolean;
    readonly type: "ApplyExtrinsic" | "Finalization" | "Initialization";
  }

  /** @name FrameSystemLastRuntimeUpgradeInfo (62) */
  export interface FrameSystemLastRuntimeUpgradeInfo extends Struct {
    readonly specVersion: Compact<u32>;
    readonly specName: Text;
  }

  /** @name FrameSystemCall (66) */
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

  /** @name FrameSystemLimitsBlockWeights (70) */
  export interface FrameSystemLimitsBlockWeights extends Struct {
    readonly baseBlock: u64;
    readonly maxBlock: u64;
    readonly perClass: FrameSupportWeightsPerDispatchClassWeightsPerClass;
  }

  /** @name FrameSupportWeightsPerDispatchClassWeightsPerClass (71) */
  export interface FrameSupportWeightsPerDispatchClassWeightsPerClass
    extends Struct {
    readonly normal: FrameSystemLimitsWeightsPerClass;
    readonly operational: FrameSystemLimitsWeightsPerClass;
    readonly mandatory: FrameSystemLimitsWeightsPerClass;
  }

  /** @name FrameSystemLimitsWeightsPerClass (72) */
  export interface FrameSystemLimitsWeightsPerClass extends Struct {
    readonly baseExtrinsic: u64;
    readonly maxExtrinsic: Option<u64>;
    readonly maxTotal: Option<u64>;
    readonly reserved: Option<u64>;
  }

  /** @name FrameSystemLimitsBlockLength (74) */
  export interface FrameSystemLimitsBlockLength extends Struct {
    readonly max: FrameSupportWeightsPerDispatchClassU32;
  }

  /** @name FrameSupportWeightsPerDispatchClassU32 (75) */
  export interface FrameSupportWeightsPerDispatchClassU32 extends Struct {
    readonly normal: u32;
    readonly operational: u32;
    readonly mandatory: u32;
  }

  /** @name FrameSupportWeightsRuntimeDbWeight (76) */
  export interface FrameSupportWeightsRuntimeDbWeight extends Struct {
    readonly read: u64;
    readonly write: u64;
  }

  /** @name SpVersionRuntimeVersion (77) */
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

  /** @name FrameSystemError (82) */
  export interface FrameSystemError extends Enum {
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

  /** @name PalletTimestampCall (84) */
  export interface PalletTimestampCall extends Enum {
    readonly isSet: boolean;
    readonly asSet: {
      readonly now: Compact<u64>;
    } & Struct;
    readonly type: "Set";
  }

  /** @name SpConsensusAuraSr25519AppSr25519Public (87) */
  export interface SpConsensusAuraSr25519AppSr25519Public
    extends SpCoreSr25519Public {}

  /** @name SpCoreSr25519Public (88) */
  export interface SpCoreSr25519Public extends U8aFixed {}

  /** @name PalletGrandpaStoredState (91) */
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
    readonly type: "Live" | "PendingPause" | "Paused" | "PendingResume";
  }

  /** @name PalletGrandpaStoredPendingChange (92) */
  export interface PalletGrandpaStoredPendingChange extends Struct {
    readonly scheduledAt: u32;
    readonly delay: u32;
    readonly nextAuthorities: Vec<ITuple<[SpFinalityGrandpaAppPublic, u64]>>;
    readonly forced: Option<u32>;
  }

  /** @name PalletGrandpaCall (95) */
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
    readonly type:
      | "ReportEquivocation"
      | "ReportEquivocationUnsigned"
      | "NoteStalled";
  }

  /** @name SpFinalityGrandpaEquivocationProof (96) */
  export interface SpFinalityGrandpaEquivocationProof extends Struct {
    readonly setId: u64;
    readonly equivocation: SpFinalityGrandpaEquivocation;
  }

  /** @name SpFinalityGrandpaEquivocation (97) */
  export interface SpFinalityGrandpaEquivocation extends Enum {
    readonly isPrevote: boolean;
    readonly asPrevote: FinalityGrandpaEquivocationPrevote;
    readonly isPrecommit: boolean;
    readonly asPrecommit: FinalityGrandpaEquivocationPrecommit;
    readonly type: "Prevote" | "Precommit";
  }

  /** @name FinalityGrandpaEquivocationPrevote (98) */
  export interface FinalityGrandpaEquivocationPrevote extends Struct {
    readonly roundNumber: u64;
    readonly identity: SpFinalityGrandpaAppPublic;
    readonly first: ITuple<
      [FinalityGrandpaPrevote, SpFinalityGrandpaAppSignature]
    >;
    readonly second: ITuple<
      [FinalityGrandpaPrevote, SpFinalityGrandpaAppSignature]
    >;
  }

  /** @name FinalityGrandpaPrevote (99) */
  export interface FinalityGrandpaPrevote extends Struct {
    readonly targetHash: H256;
    readonly targetNumber: u32;
  }

  /** @name SpFinalityGrandpaAppSignature (100) */
  export interface SpFinalityGrandpaAppSignature
    extends SpCoreEd25519Signature {}

  /** @name SpCoreEd25519Signature (101) */
  export interface SpCoreEd25519Signature extends U8aFixed {}

  /** @name FinalityGrandpaEquivocationPrecommit (104) */
  export interface FinalityGrandpaEquivocationPrecommit extends Struct {
    readonly roundNumber: u64;
    readonly identity: SpFinalityGrandpaAppPublic;
    readonly first: ITuple<
      [FinalityGrandpaPrecommitU32, SpFinalityGrandpaAppSignature]
    >;
    readonly second: ITuple<
      [FinalityGrandpaPrecommitU32, SpFinalityGrandpaAppSignature]
    >;
  }

  /** @name FinalityGrandpaPrecommitU32 (105) */
  export interface FinalityGrandpaPrecommitU32 extends Struct {
    readonly targetHash: H256;
    readonly targetNumber: u32;
  }

  /** @name SpCoreVoid (107) */
  export type SpCoreVoid = Null;

  /** @name PalletGrandpaError (108) */
  export interface PalletGrandpaError extends Enum {
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

  /** @name PalletBalancesBalanceLock (110) */
  export interface PalletBalancesBalanceLock extends Struct {
    readonly id: U8aFixed;
    readonly amount: u128;
    readonly reasons: PalletBalancesReasons;
  }

  /** @name PalletBalancesReasons (111) */
  export interface PalletBalancesReasons extends Enum {
    readonly isFee: boolean;
    readonly isMisc: boolean;
    readonly isAll: boolean;
    readonly type: "Fee" | "Misc" | "All";
  }

  /** @name PalletBalancesReserveData (114) */
  export interface PalletBalancesReserveData extends Struct {
    readonly id: U8aFixed;
    readonly amount: u128;
  }

  /** @name PalletBalancesReleases (116) */
  export interface PalletBalancesReleases extends Enum {
    readonly isV100: boolean;
    readonly isV200: boolean;
    readonly type: "V100" | "V200";
  }

  /** @name PalletBalancesCall (117) */
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
    readonly type:
      | "Transfer"
      | "SetBalance"
      | "ForceTransfer"
      | "TransferKeepAlive"
      | "TransferAll"
      | "ForceUnreserve";
  }

  /** @name PalletBalancesError (122) */
  export interface PalletBalancesError extends Enum {
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

  /** @name PalletTransactionPaymentReleases (124) */
  export interface PalletTransactionPaymentReleases extends Enum {
    readonly isV1Ancient: boolean;
    readonly isV2: boolean;
    readonly type: "V1Ancient" | "V2";
  }

  /** @name FrameSupportWeightsWeightToFeeCoefficient (126) */
  export interface FrameSupportWeightsWeightToFeeCoefficient extends Struct {
    readonly coeffInteger: u128;
    readonly coeffFrac: Perbill;
    readonly negative: bool;
    readonly degree: u8;
  }

  /** @name PalletSudoCall (127) */
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
    readonly type: "Sudo" | "SudoUncheckedWeight" | "SetKey" | "SudoAs";
  }

  /** @name PalletXdnsCall (129) */
  export interface PalletXdnsCall extends Enum {
    readonly isAddNewXdnsRecord: boolean;
    readonly asAddNewXdnsRecord: {
      readonly url: Bytes;
      readonly gatewayId: U8aFixed;
      readonly gatewayAbi: T3rnPrimitivesAbiGatewayABIConfig;
      readonly gatewayVendor: T3rnPrimitivesGatewayVendor;
      readonly gatewayType: T3rnPrimitivesGatewayType;
      readonly gatewayGenesis: T3rnPrimitivesGatewayGenesisConfig;
      readonly gatewaySysProps: T3rnPrimitivesGatewaySysProps;
      readonly allowedSideEffects: Vec<U8aFixed>;
    } & Struct;
    readonly isAddSideEffect: boolean;
    readonly asAddSideEffect: {
      readonly id: U8aFixed;
      readonly name: Bytes;
      readonly argumentAbi: Vec<T3rnPrimitivesAbiType>;
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
    readonly type:
      | "AddNewXdnsRecord"
      | "AddSideEffect"
      | "UpdateTtl"
      | "PurgeXdnsRecord";
  }

  /** @name T3rnPrimitivesAbiGatewayABIConfig (130) */
  export interface T3rnPrimitivesAbiGatewayABIConfig extends Struct {
    readonly blockNumberTypeSize: u16;
    readonly hashSize: u16;
    readonly hasher: T3rnPrimitivesAbiHasherAlgo;
    readonly crypto: T3rnPrimitivesAbiCryptoAlgo;
    readonly addressLength: u16;
    readonly valueTypeSize: u16;
    readonly decimals: u16;
    readonly structs: Vec<T3rnPrimitivesAbiStructDecl>;
  }

  /** @name T3rnPrimitivesAbiHasherAlgo (131) */
  export interface T3rnPrimitivesAbiHasherAlgo extends Enum {
    readonly isBlake2: boolean;
    readonly isKeccak256: boolean;
    readonly type: "Blake2" | "Keccak256";
  }

  /** @name T3rnPrimitivesAbiCryptoAlgo (132) */
  export interface T3rnPrimitivesAbiCryptoAlgo extends Enum {
    readonly isEd25519: boolean;
    readonly isSr25519: boolean;
    readonly isEcdsa: boolean;
    readonly type: "Ed25519" | "Sr25519" | "Ecdsa";
  }

  /** @name T3rnPrimitivesAbiStructDecl (134) */
  export interface T3rnPrimitivesAbiStructDecl extends Struct {
    readonly name: T3rnPrimitivesAbiType;
    readonly fields: Vec<T3rnPrimitivesAbiParameter>;
    readonly offsets: Vec<u16>;
  }

  /** @name T3rnPrimitivesAbiType (135) */
  export interface T3rnPrimitivesAbiType extends Enum {
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
    readonly asMapping: ITuple<[T3rnPrimitivesAbiType, T3rnPrimitivesAbiType]>;
    readonly isContract: boolean;
    readonly isRef: boolean;
    readonly asRef: T3rnPrimitivesAbiType;
    readonly isOption: boolean;
    readonly asOption: T3rnPrimitivesAbiType;
    readonly isOptionalInsurance: boolean;
    readonly isOptionalReward: boolean;
    readonly isStorageRef: boolean;
    readonly asStorageRef: T3rnPrimitivesAbiType;
    readonly isValue: boolean;
    readonly isSlice: boolean;
    readonly isHasher: boolean;
    readonly asHasher: ITuple<[T3rnPrimitivesAbiHasherAlgo, u16]>;
    readonly isCrypto: boolean;
    readonly asCrypto: T3rnPrimitivesAbiCryptoAlgo;
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

  /** @name T3rnPrimitivesAbiParameter (137) */
  export interface T3rnPrimitivesAbiParameter extends Struct {
    readonly name: Option<Bytes>;
    readonly ty: T3rnPrimitivesAbiType;
    readonly no: u32;
    readonly indexed: Option<bool>;
  }

  /** @name T3rnPrimitivesGatewayGenesisConfig (140) */
  export interface T3rnPrimitivesGatewayGenesisConfig extends Struct {
    readonly modulesEncoded: Option<Bytes>;
    readonly extrinsicsVersion: u8;
    readonly genesisHash: Bytes;
  }

  /** @name PalletMultiFinalityVerifierCall (142) */
  export interface PalletMultiFinalityVerifierCall extends Enum {
    readonly isSubmitFinalityProof: boolean;
    readonly asSubmitFinalityProof: {
      readonly finalityTarget: {
        readonly parentHash: H256;
        readonly number: Compact<u32>;
        readonly stateRoot: H256;
        readonly extrinsicsRoot: H256;
        readonly digest: SpRuntimeDigest;
      } & Struct;
      readonly justification: {
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
      } & Struct;
      readonly gatewayId: U8aFixed;
    } & Struct;
    readonly isSubmitHeaderRange: boolean;
    readonly asSubmitHeaderRange: {
      readonly gatewayId: U8aFixed;
      readonly headersReversed: Vec<
        {
          readonly parentHash: H256;
          readonly number: Compact<u32>;
          readonly stateRoot: H256;
          readonly extrinsicsRoot: H256;
          readonly digest: SpRuntimeDigest;
        } & Struct
      >;
      readonly anchorHeaderHash: H256;
    } & Struct;
    readonly isInitializeSingle: boolean;
    readonly asInitializeSingle: {
      readonly initData: {
        readonly header: {
          readonly parentHash: H256;
          readonly number: Compact<u32>;
          readonly stateRoot: H256;
          readonly extrinsicsRoot: H256;
          readonly digest: SpRuntimeDigest;
        } & Struct;
        readonly authorityList: Vec<ITuple<[SpFinalityGrandpaAppPublic, u64]>>;
        readonly setId: u64;
        readonly isHalted: bool;
      } & Struct;
      readonly gatewayId: U8aFixed;
    } & Struct;
    readonly isSetOwner: boolean;
    readonly asSetOwner: {
      readonly newOwner: Option<AccountId32>;
      readonly gatewayId: U8aFixed;
    } & Struct;
    readonly isSetOperational: boolean;
    readonly asSetOperational: {
      readonly operational: bool;
      readonly gatewayId: U8aFixed;
    } & Struct;
    readonly type:
      | "SubmitFinalityProof"
      | "SubmitHeaderRange"
      | "InitializeSingle"
      | "SetOwner"
      | "SetOperational";
  }

  /** @name SpRuntimeBlakeTwo256 (144) */
  export type SpRuntimeBlakeTwo256 = Null;

  /** @name FinalityGrandpaCommitU32 (146) */
  export interface FinalityGrandpaCommitU32 extends Struct {
    readonly targetHash: H256;
    readonly targetNumber: u32;
    readonly precommits: Vec<FinalityGrandpaSignedPrecommitU32>;
  }

  /** @name FinalityGrandpaSignedPrecommitU32 (148) */
  export interface FinalityGrandpaSignedPrecommitU32 extends Struct {
    readonly precommit: FinalityGrandpaPrecommitU32;
    readonly signature: SpFinalityGrandpaAppSignature;
    readonly id: SpFinalityGrandpaAppPublic;
  }

  /** @name SpRuntimeKeccak256 (154) */
  export type SpRuntimeKeccak256 = Null;

  /** @name FinalityGrandpaCommitU64 (156) */
  export interface FinalityGrandpaCommitU64 extends Struct {
    readonly targetHash: H256;
    readonly targetNumber: u64;
    readonly precommits: Vec<FinalityGrandpaSignedPrecommitU64>;
  }

  /** @name FinalityGrandpaSignedPrecommitU64 (158) */
  export interface FinalityGrandpaSignedPrecommitU64 extends Struct {
    readonly precommit: FinalityGrandpaPrecommitU64;
    readonly signature: SpFinalityGrandpaAppSignature;
    readonly id: SpFinalityGrandpaAppPublic;
  }

  /** @name FinalityGrandpaPrecommitU64 (159) */
  export interface FinalityGrandpaPrecommitU64 extends Struct {
    readonly targetHash: H256;
    readonly targetNumber: u64;
  }

  /** @name PalletContractsRegistryCall (168) */
  export interface PalletContractsRegistryCall extends Enum {
    readonly isAddNewContract: boolean;
    readonly asAddNewContract: {
      readonly requester: AccountId32;
      readonly contract: PalletContractsRegistryRegistryContract;
    } & Struct;
    readonly isPurge: boolean;
    readonly asPurge: {
      readonly requester: AccountId32;
      readonly contractId: H256;
    } & Struct;
    readonly type: "AddNewContract" | "Purge";
  }

  /** @name PalletContractsRegistryRegistryContract (169) */
  export interface PalletContractsRegistryRegistryContract extends Struct {
    readonly codeTxt: Bytes;
    readonly bytes: Bytes;
    readonly author: AccountId32;
    readonly authorFeesPerSingleUse: Option<u128>;
    readonly abi: Option<Bytes>;
    readonly actionDescriptions: Vec<T3rnPrimitivesAbiContractActionDesc>;
    readonly info: Option<T3rnPrimitivesStorageRawAliveContractInfo>;
    readonly meta: T3rnPrimitivesContractMetadata;
  }

  /** @name T3rnPrimitivesAbiContractActionDesc (171) */
  export interface T3rnPrimitivesAbiContractActionDesc extends Struct {
    readonly actionId: H256;
    readonly targetId: Option<U8aFixed>;
    readonly to: Option<AccountId32>;
  }

  /** @name T3rnPrimitivesStorageRawAliveContractInfo (174) */
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

  /** @name T3rnPrimitivesContractMetadata (176) */
  export interface T3rnPrimitivesContractMetadata extends Struct {
    readonly metadataVersion: Bytes;
    readonly name: Bytes;
    readonly version: Bytes;
    readonly authors: Vec<Bytes>;
    readonly description: Option<Bytes>;
    readonly documentation: Option<Bytes>;
    readonly repository: Option<Bytes>;
    readonly homepage: Option<Bytes>;
    readonly license: Option<Bytes>;
  }

  /** @name PalletCircuitPortalCall (177) */
  export interface PalletCircuitPortalCall extends Enum {
    readonly isRegisterGateway: boolean;
    readonly asRegisterGateway: {
      readonly url: Bytes;
      readonly gatewayId: U8aFixed;
      readonly gatewayAbi: T3rnPrimitivesAbiGatewayABIConfig;
      readonly gatewayVendor: T3rnPrimitivesGatewayVendor;
      readonly gatewayType: T3rnPrimitivesGatewayType;
      readonly gatewayGenesis: T3rnPrimitivesGatewayGenesisConfig;
      readonly gatewaySysProps: T3rnPrimitivesGatewaySysProps;
      readonly firstHeader: Bytes;
      readonly authorities: Option<Vec<AccountId32>>;
      readonly allowedSideEffects: Vec<U8aFixed>;
    } & Struct;
    readonly isUpdateGateway: boolean;
    readonly asUpdateGateway: {
      readonly gatewayId: U8aFixed;
      readonly url: Option<Bytes>;
      readonly gatewayAbi: Option<T3rnPrimitivesAbiGatewayABIConfig>;
      readonly gatewaySysProps: Option<T3rnPrimitivesGatewaySysProps>;
      readonly authorities: Option<Vec<AccountId32>>;
      readonly allowedSideEffects: Option<Vec<U8aFixed>>;
    } & Struct;
    readonly type: "RegisterGateway" | "UpdateGateway";
  }

  /** @name PalletCircuitCall (182) */
  export interface PalletCircuitCall extends Enum {
    readonly isOnLocalTrigger: boolean;
    readonly isOnXcmTrigger: boolean;
    readonly isOnRemoteGatewayTrigger: boolean;
    readonly isOnExtrinsicTrigger: boolean;
    readonly asOnExtrinsicTrigger: {
      readonly sideEffects: Vec<T3rnPrimitivesSideEffect>;
      readonly fee: u128;
      readonly sequential: bool;
    } & Struct;
    readonly isBondInsuranceDeposit: boolean;
    readonly asBondInsuranceDeposit: {
      readonly xtxId: H256;
      readonly sideEffectId: H256;
    } & Struct;
    readonly isConfirmSideEffect: boolean;
    readonly asConfirmSideEffect: {
      readonly xtxId: H256;
      readonly sideEffect: T3rnPrimitivesSideEffect;
      readonly confirmation: T3rnPrimitivesSideEffectConfirmedSideEffect;
      readonly inclusionProof: Option<Vec<Bytes>>;
      readonly blockHash: Option<Bytes>;
    } & Struct;
    readonly type:
      | "OnLocalTrigger"
      | "OnXcmTrigger"
      | "OnRemoteGatewayTrigger"
      | "OnExtrinsicTrigger"
      | "BondInsuranceDeposit"
      | "ConfirmSideEffect";
  }

  /** @name PalletSudoError (184) */
  export interface PalletSudoError extends Enum {
    readonly isRequireSudo: boolean;
    readonly type: "RequireSudo";
  }

  /** @name OrmlTokensBalanceLock (187) */
  export interface OrmlTokensBalanceLock extends Struct {
    readonly id: U8aFixed;
    readonly amount: u128;
  }

  /** @name OrmlTokensAccountData (189) */
  export interface OrmlTokensAccountData extends Struct {
    readonly free: u128;
    readonly reserved: u128;
    readonly frozen: u128;
  }

  /** @name OrmlTokensModuleError (190) */
  export interface OrmlTokensModuleError extends Enum {
    readonly isBalanceTooLow: boolean;
    readonly isAmountIntoBalanceFailed: boolean;
    readonly isLiquidityRestrictions: boolean;
    readonly isMaxLocksExceeded: boolean;
    readonly isKeepAlive: boolean;
    readonly isExistentialDeposit: boolean;
    readonly isDeadAccount: boolean;
    readonly type:
      | "BalanceTooLow"
      | "AmountIntoBalanceFailed"
      | "LiquidityRestrictions"
      | "MaxLocksExceeded"
      | "KeepAlive"
      | "ExistentialDeposit"
      | "DeadAccount";
  }

  /** @name PalletXdnsSideEffectInterface (191) */
  export interface PalletXdnsSideEffectInterface extends Struct {
    readonly id: U8aFixed;
    readonly name: Bytes;
    readonly argumentAbi: Vec<T3rnPrimitivesAbiType>;
    readonly argumentToStateMapper: Vec<Bytes>;
    readonly confirmEvents: Vec<Bytes>;
    readonly escrowedEvents: Vec<Bytes>;
    readonly commitEvents: Vec<Bytes>;
    readonly revertEvents: Vec<Bytes>;
  }

  /** @name PalletXdnsXdnsRecord (192) */
  export interface PalletXdnsXdnsRecord extends Struct {
    readonly url: Bytes;
    readonly gatewayAbi: T3rnPrimitivesAbiGatewayABIConfig;
    readonly gatewayGenesis: T3rnPrimitivesGatewayGenesisConfig;
    readonly gatewayVendor: T3rnPrimitivesGatewayVendor;
    readonly gatewayType: T3rnPrimitivesGatewayType;
    readonly gatewayId: U8aFixed;
    readonly gatewaySysProps: T3rnPrimitivesGatewaySysProps;
    readonly registrant: Option<AccountId32>;
    readonly lastFinalized: Option<u64>;
    readonly allowedSideEffects: Vec<U8aFixed>;
  }

  /** @name PalletXdnsError (193) */
  export interface PalletXdnsError extends Enum {
    readonly isXdnsRecordAlreadyExists: boolean;
    readonly isUnknownXdnsRecord: boolean;
    readonly isXdnsRecordNotFound: boolean;
    readonly isSideEffectInterfaceAlreadyExists: boolean;
    readonly type:
      | "XdnsRecordAlreadyExists"
      | "UnknownXdnsRecord"
      | "XdnsRecordNotFound"
      | "SideEffectInterfaceAlreadyExists";
  }

  /** @name T3rnPrimitivesBridgesHeaderChainAuthoritySet (197) */
  export interface T3rnPrimitivesBridgesHeaderChainAuthoritySet extends Struct {
    readonly authorities: Vec<ITuple<[SpFinalityGrandpaAppPublic, u64]>>;
    readonly setId: u64;
  }

  /** @name PalletMultiFinalityVerifierError (198) */
  export interface PalletMultiFinalityVerifierError extends Enum {
    readonly isInvalidJustification: boolean;
    readonly isInvalidAuthoritySet: boolean;
    readonly isTooManyRequests: boolean;
    readonly isOldHeader: boolean;
    readonly isUnknownHeader: boolean;
    readonly isUnsupportedScheduledChange: boolean;
    readonly isAlreadyInitialized: boolean;
    readonly isHalted: boolean;
    readonly isStorageRootMismatch: boolean;
    readonly isInvalidAnchorHeader: boolean;
    readonly isNoFinalizedHeader: boolean;
    readonly type:
      | "InvalidJustification"
      | "InvalidAuthoritySet"
      | "TooManyRequests"
      | "OldHeader"
      | "UnknownHeader"
      | "UnsupportedScheduledChange"
      | "AlreadyInitialized"
      | "Halted"
      | "StorageRootMismatch"
      | "InvalidAnchorHeader"
      | "NoFinalizedHeader";
  }

  /** @name PalletContractsRegistryError (203) */
  export interface PalletContractsRegistryError extends Enum {
    readonly isContractAlreadyExists: boolean;
    readonly isUnknownContract: boolean;
    readonly type: "ContractAlreadyExists" | "UnknownContract";
  }

  /** @name PalletCircuitPortalError (204) */
  export interface PalletCircuitPortalError extends Enum {
    readonly isInvalidKey: boolean;
    readonly isIoScheduleNoEndingSemicolon: boolean;
    readonly isIoScheduleEmpty: boolean;
    readonly isIoScheduleUnknownCompose: boolean;
    readonly isProcessStepGatewayNotRecognised: boolean;
    readonly isStepConfirmationBlockUnrecognised: boolean;
    readonly isStepConfirmationGatewayNotRecognised: boolean;
    readonly isSideEffectConfirmationInvalidInclusionProof: boolean;
    readonly isVendorUnknown: boolean;
    readonly isSideEffectTypeNotRecognized: boolean;
    readonly isStepConfirmationDecodingError: boolean;
    readonly isContractDoesNotExists: boolean;
    readonly isRequesterNotEnoughBalance: boolean;
    readonly type:
      | "InvalidKey"
      | "IoScheduleNoEndingSemicolon"
      | "IoScheduleEmpty"
      | "IoScheduleUnknownCompose"
      | "ProcessStepGatewayNotRecognised"
      | "StepConfirmationBlockUnrecognised"
      | "StepConfirmationGatewayNotRecognised"
      | "SideEffectConfirmationInvalidInclusionProof"
      | "VendorUnknown"
      | "SideEffectTypeNotRecognized"
      | "StepConfirmationDecodingError"
      | "ContractDoesNotExists"
      | "RequesterNotEnoughBalance";
  }

  /** @name PalletCircuitStateInsuranceDeposit (205) */
  export interface PalletCircuitStateInsuranceDeposit extends Struct {
    readonly insurance: u128;
    readonly reward: u128;
    readonly requester: AccountId32;
    readonly bondedRelayer: Option<AccountId32>;
    readonly status: PalletCircuitStateCircuitStatus;
    readonly requestedAt: u32;
  }

  /** @name PalletCircuitStateCircuitStatus (206) */
  export interface PalletCircuitStateCircuitStatus extends Enum {
    readonly isRequested: boolean;
    readonly isPendingInsurance: boolean;
    readonly isBonded: boolean;
    readonly isReady: boolean;
    readonly isPendingExecution: boolean;
    readonly isFinished: boolean;
    readonly isCommitted: boolean;
    readonly isReverted: boolean;
    readonly isRevertedTimedOut: boolean;
    readonly type:
      | "Requested"
      | "PendingInsurance"
      | "Bonded"
      | "Ready"
      | "PendingExecution"
      | "Finished"
      | "Committed"
      | "Reverted"
      | "RevertedTimedOut";
  }

  /** @name PalletCircuitStateXExecSignal (207) */
  export interface PalletCircuitStateXExecSignal extends Struct {
    readonly requester: AccountId32;
    readonly timeoutsAt: Option<u32>;
    readonly delayStepsAt: Option<Vec<u32>>;
    readonly status: PalletCircuitStateCircuitStatus;
    readonly totalReward: Option<u128>;
  }

  /** @name T3rnPrimitivesVolatileLocalState (210) */
  export interface T3rnPrimitivesVolatileLocalState extends Struct {
    readonly state: BTreeMap<U8aFixed, Bytes>;
  }

  /** @name FrameSupportPalletId (214) */
  export interface FrameSupportPalletId extends U8aFixed {}

  /** @name PalletCircuitError (215) */
  export interface PalletCircuitError extends Enum {
    readonly isRequesterNotEnoughBalance: boolean;
    readonly isChargingTransferFailed: boolean;
    readonly isRewardTransferFailed: boolean;
    readonly isRefundTransferFailed: boolean;
    readonly isInsuranceBondNotRequired: boolean;
    readonly isInsuranceBondAlreadyDeposited: boolean;
    readonly isSetupFailed: boolean;
    readonly isSetupFailedIncorrectXtxStatus: boolean;
    readonly isSetupFailedUnknownXtx: boolean;
    readonly isSetupFailedDuplicatedXtx: boolean;
    readonly isSetupFailedEmptyXtx: boolean;
    readonly isApplyFailed: boolean;
    readonly isDeterminedForbiddenXtxStatus: boolean;
    readonly isUnsupportedRole: boolean;
    readonly type:
      | "RequesterNotEnoughBalance"
      | "ChargingTransferFailed"
      | "RewardTransferFailed"
      | "RefundTransferFailed"
      | "InsuranceBondNotRequired"
      | "InsuranceBondAlreadyDeposited"
      | "SetupFailed"
      | "SetupFailedIncorrectXtxStatus"
      | "SetupFailedUnknownXtx"
      | "SetupFailedDuplicatedXtx"
      | "SetupFailedEmptyXtx"
      | "ApplyFailed"
      | "DeterminedForbiddenXtxStatus"
      | "UnsupportedRole";
  }

  /** @name SpRuntimeMultiSignature (217) */
  export interface SpRuntimeMultiSignature extends Enum {
    readonly isEd25519: boolean;
    readonly asEd25519: SpCoreEd25519Signature;
    readonly isSr25519: boolean;
    readonly asSr25519: SpCoreSr25519Signature;
    readonly isEcdsa: boolean;
    readonly asEcdsa: SpCoreEcdsaSignature;
    readonly type: "Ed25519" | "Sr25519" | "Ecdsa";
  }

  /** @name SpCoreSr25519Signature (218) */
  export interface SpCoreSr25519Signature extends U8aFixed {}

  /** @name SpCoreEcdsaSignature (219) */
  export interface SpCoreEcdsaSignature extends U8aFixed {}

  /** @name FrameSystemExtensionsCheckNonZeroSender (222) */
  export type FrameSystemExtensionsCheckNonZeroSender = Null;

  /** @name FrameSystemExtensionsCheckSpecVersion (223) */
  export type FrameSystemExtensionsCheckSpecVersion = Null;

  /** @name FrameSystemExtensionsCheckTxVersion (224) */
  export type FrameSystemExtensionsCheckTxVersion = Null;

  /** @name FrameSystemExtensionsCheckGenesis (225) */
  export type FrameSystemExtensionsCheckGenesis = Null;

  /** @name FrameSystemExtensionsCheckNonce (228) */
  export interface FrameSystemExtensionsCheckNonce extends Compact<u32> {}

  /** @name FrameSystemExtensionsCheckWeight (229) */
  export type FrameSystemExtensionsCheckWeight = Null;

  /** @name PalletTransactionPaymentChargeTransactionPayment (230) */
  export interface PalletTransactionPaymentChargeTransactionPayment
    extends Compact<u128> {}

  /** @name CircuitStandaloneRuntimeRuntime (231) */
  export type CircuitStandaloneRuntimeRuntime = Null;
} // declare module
