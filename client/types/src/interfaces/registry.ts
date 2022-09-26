// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import "@polkadot/types/types/registry";

import type {
  CircuitStandaloneRuntimeOriginCaller,
  CircuitStandaloneRuntimeRuntime,
  FinalityGrandpaEquivocationPrecommit,
  FinalityGrandpaEquivocationPrevote,
  FinalityGrandpaPrecommit,
  FinalityGrandpaPrevote,
  FrameSupportDispatchRawOrigin,
  FrameSupportPalletId,
  FrameSupportTokensMiscBalanceStatus,
  FrameSupportWeightsDispatchClass,
  FrameSupportWeightsDispatchInfo,
  FrameSupportWeightsPays,
  FrameSupportWeightsPerDispatchClassU32,
  FrameSupportWeightsPerDispatchClassU64,
  FrameSupportWeightsPerDispatchClassWeightsPerClass,
  FrameSupportWeightsRuntimeDbWeight,
  FrameSupportWeightsWeightToFeeCoefficient,
  FrameSystemAccountInfo,
  FrameSystemCall,
  FrameSystemError,
  FrameSystemEvent,
  FrameSystemEventRecord,
  FrameSystemExtensionsCheckGenesis,
  FrameSystemExtensionsCheckNonZeroSender,
  FrameSystemExtensionsCheckNonce,
  FrameSystemExtensionsCheckSpecVersion,
  FrameSystemExtensionsCheckTxVersion,
  FrameSystemExtensionsCheckWeight,
  FrameSystemLastRuntimeUpgradeInfo,
  FrameSystemLimitsBlockLength,
  FrameSystemLimitsBlockWeights,
  FrameSystemLimitsWeightsPerClass,
  FrameSystemPhase,
  OrmlTokensAccountData,
  OrmlTokensBalanceLock,
  OrmlTokensModuleError,
  OrmlTokensModuleEvent,
  PalletAccountManagerCall,
  PalletAccountManagerError,
  PalletAccountManagerEvent,
  PalletBalancesAccountData,
  PalletBalancesBalanceLock,
  PalletBalancesCall,
  PalletBalancesError,
  PalletBalancesEvent,
  PalletBalancesReasons,
  PalletBalancesReleases,
  PalletBalancesReserveData,
  PalletCircuitCall,
  PalletCircuitError,
  PalletCircuitEvent,
  PalletCircuitPortalCall,
  PalletCircuitPortalError,
  PalletCircuitPortalEvent,
  PalletCircuitStateCircuitStatus,
  PalletCircuitStateInsuranceDeposit,
  PalletCircuitStateXExecSignal,
  PalletContractsRegistryCall,
  PalletContractsRegistryError,
  PalletContractsRegistryEvent,
  PalletGrandpaCall,
  PalletGrandpaError,
  PalletGrandpaEvent,
  PalletGrandpaFinalityVerifierError,
  PalletGrandpaFinalityVerifierParachain,
  PalletGrandpaStoredPendingChange,
  PalletGrandpaStoredState,
  PalletMultiFinalityVerifierCall,
  PalletMultiFinalityVerifierError,
  PalletMultiFinalityVerifierEvent,
  PalletPortalCall,
  PalletPortalError,
  PalletPortalEvent,
  PalletSudoCall,
  PalletSudoError,
  PalletSudoEvent,
  PalletTimestampCall,
  PalletTransactionPaymentChargeTransactionPayment,
  PalletTransactionPaymentReleases,
  PalletUtilityCall,
  PalletUtilityError,
  PalletUtilityEvent,
  PalletWasmContractsCall,
  PalletWasmContractsContractKind,
  PalletWasmContractsError,
  PalletWasmContractsEvent,
  PalletWasmContractsSchedule,
  PalletWasmContractsScheduleHostFnWeights,
  PalletWasmContractsScheduleInstructionWeights,
  PalletWasmContractsScheduleLimits,
  PalletWasmContractsStorageDeletedContract,
  PalletWasmContractsStorageRawContractInfo,
  PalletWasmContractsWasmOwnerInfo,
  PalletWasmContractsWasmPrefabWasmModule,
  PalletXdnsCall,
  PalletXdnsError,
  PalletXdnsEvent,
  SpConsensusAuraSr25519AppSr25519Public,
  SpCoreEcdsaSignature,
  SpCoreEd25519Public,
  SpCoreEd25519Signature,
  SpCoreSr25519Public,
  SpCoreSr25519Signature,
  SpCoreVoid,
  SpFinalityGrandpaAppPublic,
  SpFinalityGrandpaAppSignature,
  SpFinalityGrandpaEquivocation,
  SpFinalityGrandpaEquivocationProof,
  SpRuntimeArithmeticError,
  SpRuntimeBlakeTwo256,
  SpRuntimeDigest,
  SpRuntimeDigestDigestItem,
  SpRuntimeDispatchError,
  SpRuntimeHeaderU32,
  SpRuntimeHeaderU64,
  SpRuntimeKeccak256,
  SpRuntimeModuleError,
  SpRuntimeMultiSignature,
  SpRuntimeTokenError,
  SpRuntimeTransactionalError,
  SpVersionRuntimeVersion,
  T3rnPrimitivesAccountManagerExecutionRegistryItem,
  T3rnPrimitivesAccountManagerReason,
  T3rnPrimitivesBridgesHeaderChainAuthoritySet,
  T3rnPrimitivesBridgesHeaderChainInitializationData,
  T3rnPrimitivesContractMetadata,
  T3rnPrimitivesContractsRegistryAuthorInfo,
  T3rnPrimitivesContractsRegistryRegistryContract,
  T3rnPrimitivesGatewayGenesisConfig,
  T3rnPrimitivesGatewaySysProps,
  T3rnPrimitivesGatewayType,
  T3rnPrimitivesGatewayVendor,
  T3rnPrimitivesSideEffectFullSideEffect,
  T3rnPrimitivesSideEffectInterfaceSideEffectInterface,
  T3rnPrimitivesStorageRawAliveContractInfo,
  T3rnPrimitivesVolatileLocalState,
  T3rnPrimitivesXdnsParachain,
  T3rnPrimitivesXdnsXdnsRecord,
  T3rnSdkPrimitivesSignalExecutionSignal,
  T3rnSdkPrimitivesSignalKillReason,
  T3rnSdkPrimitivesSignalSignalKind,
  T3rnTypesAbiContractActionDesc,
  T3rnTypesAbiCryptoAlgo,
  T3rnTypesAbiGatewayABIConfig,
  T3rnTypesAbiHasherAlgo,
  T3rnTypesAbiParameter,
  T3rnTypesAbiStructDecl,
  T3rnTypesAbiType,
  T3rnTypesSideEffect,
  T3rnTypesSideEffectConfirmationOutcome,
  T3rnTypesSideEffectConfirmedSideEffect,
  T3rnTypesSideEffectSecurityLvl,
} from "@polkadot/types/lookup";

declare module "@polkadot/types/types/registry" {
  interface InterfaceTypes {
    CircuitStandaloneRuntimeOriginCaller: CircuitStandaloneRuntimeOriginCaller;
    CircuitStandaloneRuntimeRuntime: CircuitStandaloneRuntimeRuntime;
    FinalityGrandpaEquivocationPrecommit: FinalityGrandpaEquivocationPrecommit;
    FinalityGrandpaEquivocationPrevote: FinalityGrandpaEquivocationPrevote;
    FinalityGrandpaPrecommit: FinalityGrandpaPrecommit;
    FinalityGrandpaPrevote: FinalityGrandpaPrevote;
    FrameSupportDispatchRawOrigin: FrameSupportDispatchRawOrigin;
    FrameSupportPalletId: FrameSupportPalletId;
    FrameSupportTokensMiscBalanceStatus: FrameSupportTokensMiscBalanceStatus;
    FrameSupportWeightsDispatchClass: FrameSupportWeightsDispatchClass;
    FrameSupportWeightsDispatchInfo: FrameSupportWeightsDispatchInfo;
    FrameSupportWeightsPays: FrameSupportWeightsPays;
    FrameSupportWeightsPerDispatchClassU32: FrameSupportWeightsPerDispatchClassU32;
    FrameSupportWeightsPerDispatchClassU64: FrameSupportWeightsPerDispatchClassU64;
    FrameSupportWeightsPerDispatchClassWeightsPerClass: FrameSupportWeightsPerDispatchClassWeightsPerClass;
    FrameSupportWeightsRuntimeDbWeight: FrameSupportWeightsRuntimeDbWeight;
    FrameSupportWeightsWeightToFeeCoefficient: FrameSupportWeightsWeightToFeeCoefficient;
    FrameSystemAccountInfo: FrameSystemAccountInfo;
    FrameSystemCall: FrameSystemCall;
    FrameSystemError: FrameSystemError;
    FrameSystemEvent: FrameSystemEvent;
    FrameSystemEventRecord: FrameSystemEventRecord;
    FrameSystemExtensionsCheckGenesis: FrameSystemExtensionsCheckGenesis;
    FrameSystemExtensionsCheckNonZeroSender: FrameSystemExtensionsCheckNonZeroSender;
    FrameSystemExtensionsCheckNonce: FrameSystemExtensionsCheckNonce;
    FrameSystemExtensionsCheckSpecVersion: FrameSystemExtensionsCheckSpecVersion;
    FrameSystemExtensionsCheckTxVersion: FrameSystemExtensionsCheckTxVersion;
    FrameSystemExtensionsCheckWeight: FrameSystemExtensionsCheckWeight;
    FrameSystemLastRuntimeUpgradeInfo: FrameSystemLastRuntimeUpgradeInfo;
    FrameSystemLimitsBlockLength: FrameSystemLimitsBlockLength;
    FrameSystemLimitsBlockWeights: FrameSystemLimitsBlockWeights;
    FrameSystemLimitsWeightsPerClass: FrameSystemLimitsWeightsPerClass;
    FrameSystemPhase: FrameSystemPhase;
    OrmlTokensAccountData: OrmlTokensAccountData;
    OrmlTokensBalanceLock: OrmlTokensBalanceLock;
    OrmlTokensModuleError: OrmlTokensModuleError;
    OrmlTokensModuleEvent: OrmlTokensModuleEvent;
    PalletAccountManagerCall: PalletAccountManagerCall;
    PalletAccountManagerError: PalletAccountManagerError;
    PalletAccountManagerEvent: PalletAccountManagerEvent;
    PalletBalancesAccountData: PalletBalancesAccountData;
    PalletBalancesBalanceLock: PalletBalancesBalanceLock;
    PalletBalancesCall: PalletBalancesCall;
    PalletBalancesError: PalletBalancesError;
    PalletBalancesEvent: PalletBalancesEvent;
    PalletBalancesReasons: PalletBalancesReasons;
    PalletBalancesReleases: PalletBalancesReleases;
    PalletBalancesReserveData: PalletBalancesReserveData;
    PalletCircuitCall: PalletCircuitCall;
    PalletCircuitError: PalletCircuitError;
    PalletCircuitEvent: PalletCircuitEvent;
    PalletCircuitPortalCall: PalletCircuitPortalCall;
    PalletCircuitPortalError: PalletCircuitPortalError;
    PalletCircuitPortalEvent: PalletCircuitPortalEvent;
    PalletCircuitStateCircuitStatus: PalletCircuitStateCircuitStatus;
    PalletCircuitStateInsuranceDeposit: PalletCircuitStateInsuranceDeposit;
    PalletCircuitStateXExecSignal: PalletCircuitStateXExecSignal;
    PalletContractsRegistryCall: PalletContractsRegistryCall;
    PalletContractsRegistryError: PalletContractsRegistryError;
    PalletContractsRegistryEvent: PalletContractsRegistryEvent;
    PalletGrandpaCall: PalletGrandpaCall;
    PalletGrandpaError: PalletGrandpaError;
    PalletGrandpaEvent: PalletGrandpaEvent;
    PalletGrandpaFinalityVerifierError: PalletGrandpaFinalityVerifierError;
    PalletGrandpaFinalityVerifierParachain: PalletGrandpaFinalityVerifierParachain;
    PalletGrandpaStoredPendingChange: PalletGrandpaStoredPendingChange;
    PalletGrandpaStoredState: PalletGrandpaStoredState;
    PalletMultiFinalityVerifierCall: PalletMultiFinalityVerifierCall;
    PalletMultiFinalityVerifierError: PalletMultiFinalityVerifierError;
    PalletMultiFinalityVerifierEvent: PalletMultiFinalityVerifierEvent;
    PalletPortalCall: PalletPortalCall;
    PalletPortalError: PalletPortalError;
    PalletPortalEvent: PalletPortalEvent;
    PalletSudoCall: PalletSudoCall;
    PalletSudoError: PalletSudoError;
    PalletSudoEvent: PalletSudoEvent;
    PalletTimestampCall: PalletTimestampCall;
    PalletTransactionPaymentChargeTransactionPayment: PalletTransactionPaymentChargeTransactionPayment;
    PalletTransactionPaymentReleases: PalletTransactionPaymentReleases;
    PalletUtilityCall: PalletUtilityCall;
    PalletUtilityError: PalletUtilityError;
    PalletUtilityEvent: PalletUtilityEvent;
    PalletWasmContractsCall: PalletWasmContractsCall;
    PalletWasmContractsContractKind: PalletWasmContractsContractKind;
    PalletWasmContractsError: PalletWasmContractsError;
    PalletWasmContractsEvent: PalletWasmContractsEvent;
    PalletWasmContractsSchedule: PalletWasmContractsSchedule;
    PalletWasmContractsScheduleHostFnWeights: PalletWasmContractsScheduleHostFnWeights;
    PalletWasmContractsScheduleInstructionWeights: PalletWasmContractsScheduleInstructionWeights;
    PalletWasmContractsScheduleLimits: PalletWasmContractsScheduleLimits;
    PalletWasmContractsStorageDeletedContract: PalletWasmContractsStorageDeletedContract;
    PalletWasmContractsStorageRawContractInfo: PalletWasmContractsStorageRawContractInfo;
    PalletWasmContractsWasmOwnerInfo: PalletWasmContractsWasmOwnerInfo;
    PalletWasmContractsWasmPrefabWasmModule: PalletWasmContractsWasmPrefabWasmModule;
    PalletXdnsCall: PalletXdnsCall;
    PalletXdnsError: PalletXdnsError;
    PalletXdnsEvent: PalletXdnsEvent;
    SpConsensusAuraSr25519AppSr25519Public: SpConsensusAuraSr25519AppSr25519Public;
    SpCoreEcdsaSignature: SpCoreEcdsaSignature;
    SpCoreEd25519Public: SpCoreEd25519Public;
    SpCoreEd25519Signature: SpCoreEd25519Signature;
    SpCoreSr25519Public: SpCoreSr25519Public;
    SpCoreSr25519Signature: SpCoreSr25519Signature;
    SpCoreVoid: SpCoreVoid;
    SpFinalityGrandpaAppPublic: SpFinalityGrandpaAppPublic;
    SpFinalityGrandpaAppSignature: SpFinalityGrandpaAppSignature;
    SpFinalityGrandpaEquivocation: SpFinalityGrandpaEquivocation;
    SpFinalityGrandpaEquivocationProof: SpFinalityGrandpaEquivocationProof;
    SpRuntimeArithmeticError: SpRuntimeArithmeticError;
    SpRuntimeBlakeTwo256: SpRuntimeBlakeTwo256;
    SpRuntimeDigest: SpRuntimeDigest;
    SpRuntimeDigestDigestItem: SpRuntimeDigestDigestItem;
    SpRuntimeDispatchError: SpRuntimeDispatchError;
    SpRuntimeHeaderU32: SpRuntimeHeaderU32;
    SpRuntimeHeaderU64: SpRuntimeHeaderU64;
    SpRuntimeKeccak256: SpRuntimeKeccak256;
    SpRuntimeModuleError: SpRuntimeModuleError;
    SpRuntimeMultiSignature: SpRuntimeMultiSignature;
    SpRuntimeTokenError: SpRuntimeTokenError;
    SpRuntimeTransactionalError: SpRuntimeTransactionalError;
    SpVersionRuntimeVersion: SpVersionRuntimeVersion;
    T3rnPrimitivesAccountManagerExecutionRegistryItem: T3rnPrimitivesAccountManagerExecutionRegistryItem;
    T3rnPrimitivesAccountManagerReason: T3rnPrimitivesAccountManagerReason;
    T3rnPrimitivesBridgesHeaderChainAuthoritySet: T3rnPrimitivesBridgesHeaderChainAuthoritySet;
    T3rnPrimitivesBridgesHeaderChainInitializationData: T3rnPrimitivesBridgesHeaderChainInitializationData;
    T3rnPrimitivesContractMetadata: T3rnPrimitivesContractMetadata;
    T3rnPrimitivesContractsRegistryAuthorInfo: T3rnPrimitivesContractsRegistryAuthorInfo;
    T3rnPrimitivesContractsRegistryRegistryContract: T3rnPrimitivesContractsRegistryRegistryContract;
    T3rnPrimitivesGatewayGenesisConfig: T3rnPrimitivesGatewayGenesisConfig;
    T3rnPrimitivesGatewaySysProps: T3rnPrimitivesGatewaySysProps;
    T3rnPrimitivesGatewayType: T3rnPrimitivesGatewayType;
    T3rnPrimitivesGatewayVendor: T3rnPrimitivesGatewayVendor;
    T3rnPrimitivesSideEffectFullSideEffect: T3rnPrimitivesSideEffectFullSideEffect;
    T3rnPrimitivesSideEffectInterfaceSideEffectInterface: T3rnPrimitivesSideEffectInterfaceSideEffectInterface;
    T3rnPrimitivesStorageRawAliveContractInfo: T3rnPrimitivesStorageRawAliveContractInfo;
    T3rnPrimitivesVolatileLocalState: T3rnPrimitivesVolatileLocalState;
    T3rnPrimitivesXdnsParachain: T3rnPrimitivesXdnsParachain;
    T3rnPrimitivesXdnsXdnsRecord: T3rnPrimitivesXdnsXdnsRecord;
    T3rnSdkPrimitivesSignalExecutionSignal: T3rnSdkPrimitivesSignalExecutionSignal;
    T3rnSdkPrimitivesSignalKillReason: T3rnSdkPrimitivesSignalKillReason;
    T3rnSdkPrimitivesSignalSignalKind: T3rnSdkPrimitivesSignalSignalKind;
    T3rnTypesAbiContractActionDesc: T3rnTypesAbiContractActionDesc;
    T3rnTypesAbiCryptoAlgo: T3rnTypesAbiCryptoAlgo;
    T3rnTypesAbiGatewayABIConfig: T3rnTypesAbiGatewayABIConfig;
    T3rnTypesAbiHasherAlgo: T3rnTypesAbiHasherAlgo;
    T3rnTypesAbiParameter: T3rnTypesAbiParameter;
    T3rnTypesAbiStructDecl: T3rnTypesAbiStructDecl;
    T3rnTypesAbiType: T3rnTypesAbiType;
    T3rnTypesSideEffect: T3rnTypesSideEffect;
    T3rnTypesSideEffectConfirmationOutcome: T3rnTypesSideEffectConfirmationOutcome;
    T3rnTypesSideEffectConfirmedSideEffect: T3rnTypesSideEffectConfirmedSideEffect;
    T3rnTypesSideEffectSecurityLvl: T3rnTypesSideEffectSecurityLvl;
  } // InterfaceTypes
} // declare module
