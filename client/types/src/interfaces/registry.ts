// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type {
  CircuitStandaloneRuntimeRuntime,
  FinalityGrandpaCommitU32,
  FinalityGrandpaCommitU64,
  FinalityGrandpaEquivocationPrecommit,
  FinalityGrandpaEquivocationPrevote,
  FinalityGrandpaPrecommitU32,
  FinalityGrandpaPrecommitU64,
  FinalityGrandpaPrevote,
  FinalityGrandpaSignedPrecommitU32,
  FinalityGrandpaSignedPrecommitU64,
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
  PalletContractsRegistryRegistryContract,
  PalletGrandpaCall,
  PalletGrandpaError,
  PalletGrandpaEvent,
  PalletGrandpaStoredPendingChange,
  PalletGrandpaStoredState,
  PalletMultiFinalityVerifierCall,
  PalletMultiFinalityVerifierError,
  PalletSudoCall,
  PalletSudoError,
  PalletSudoEvent,
  PalletTimestampCall,
  PalletTransactionPaymentChargeTransactionPayment,
  PalletTransactionPaymentReleases,
  PalletXdnsCall,
  PalletXdnsError,
  PalletXdnsEvent,
  PalletXdnsSideEffectInterface,
  PalletXdnsXdnsRecord,
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
  SpRuntimeKeccak256,
  SpRuntimeModuleError,
  SpRuntimeMultiSignature,
  SpRuntimeTokenError,
  SpVersionRuntimeVersion,
  T3rnPrimitivesAbiContractActionDesc,
  T3rnPrimitivesAbiCryptoAlgo,
  T3rnPrimitivesAbiGatewayABIConfig,
  T3rnPrimitivesAbiHasherAlgo,
  T3rnPrimitivesAbiParameter,
  T3rnPrimitivesAbiStructDecl,
  T3rnPrimitivesAbiType,
  T3rnPrimitivesBridgesHeaderChainAuthoritySet,
  T3rnPrimitivesContractMetadata,
  T3rnPrimitivesGatewayGenesisConfig,
  T3rnPrimitivesGatewaySysProps,
  T3rnPrimitivesGatewayType,
  T3rnPrimitivesGatewayVendor,
  T3rnPrimitivesSideEffect,
  T3rnPrimitivesSideEffectConfirmedSideEffect,
  T3rnPrimitivesSideEffectFullSideEffect,
  T3rnPrimitivesStorageRawAliveContractInfo,
  T3rnPrimitivesVolatileLocalState,
} from "@polkadot/types/lookup";

declare module "@polkadot/types/types/registry" {
  export interface InterfaceTypes {
    CircuitStandaloneRuntimeRuntime: CircuitStandaloneRuntimeRuntime;
    FinalityGrandpaCommitU32: FinalityGrandpaCommitU32;
    FinalityGrandpaCommitU64: FinalityGrandpaCommitU64;
    FinalityGrandpaEquivocationPrecommit: FinalityGrandpaEquivocationPrecommit;
    FinalityGrandpaEquivocationPrevote: FinalityGrandpaEquivocationPrevote;
    FinalityGrandpaPrecommitU32: FinalityGrandpaPrecommitU32;
    FinalityGrandpaPrecommitU64: FinalityGrandpaPrecommitU64;
    FinalityGrandpaPrevote: FinalityGrandpaPrevote;
    FinalityGrandpaSignedPrecommitU32: FinalityGrandpaSignedPrecommitU32;
    FinalityGrandpaSignedPrecommitU64: FinalityGrandpaSignedPrecommitU64;
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
    PalletContractsRegistryRegistryContract: PalletContractsRegistryRegistryContract;
    PalletGrandpaCall: PalletGrandpaCall;
    PalletGrandpaError: PalletGrandpaError;
    PalletGrandpaEvent: PalletGrandpaEvent;
    PalletGrandpaStoredPendingChange: PalletGrandpaStoredPendingChange;
    PalletGrandpaStoredState: PalletGrandpaStoredState;
    PalletMultiFinalityVerifierCall: PalletMultiFinalityVerifierCall;
    PalletMultiFinalityVerifierError: PalletMultiFinalityVerifierError;
    PalletSudoCall: PalletSudoCall;
    PalletSudoError: PalletSudoError;
    PalletSudoEvent: PalletSudoEvent;
    PalletTimestampCall: PalletTimestampCall;
    PalletTransactionPaymentChargeTransactionPayment: PalletTransactionPaymentChargeTransactionPayment;
    PalletTransactionPaymentReleases: PalletTransactionPaymentReleases;
    PalletXdnsCall: PalletXdnsCall;
    PalletXdnsError: PalletXdnsError;
    PalletXdnsEvent: PalletXdnsEvent;
    PalletXdnsSideEffectInterface: PalletXdnsSideEffectInterface;
    PalletXdnsXdnsRecord: PalletXdnsXdnsRecord;
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
    SpRuntimeKeccak256: SpRuntimeKeccak256;
    SpRuntimeModuleError: SpRuntimeModuleError;
    SpRuntimeMultiSignature: SpRuntimeMultiSignature;
    SpRuntimeTokenError: SpRuntimeTokenError;
    SpVersionRuntimeVersion: SpVersionRuntimeVersion;
    T3rnPrimitivesAbiContractActionDesc: T3rnPrimitivesAbiContractActionDesc;
    T3rnPrimitivesAbiCryptoAlgo: T3rnPrimitivesAbiCryptoAlgo;
    T3rnPrimitivesAbiGatewayABIConfig: T3rnPrimitivesAbiGatewayABIConfig;
    T3rnPrimitivesAbiHasherAlgo: T3rnPrimitivesAbiHasherAlgo;
    T3rnPrimitivesAbiParameter: T3rnPrimitivesAbiParameter;
    T3rnPrimitivesAbiStructDecl: T3rnPrimitivesAbiStructDecl;
    T3rnPrimitivesAbiType: T3rnPrimitivesAbiType;
    T3rnPrimitivesBridgesHeaderChainAuthoritySet: T3rnPrimitivesBridgesHeaderChainAuthoritySet;
    T3rnPrimitivesContractMetadata: T3rnPrimitivesContractMetadata;
    T3rnPrimitivesGatewayGenesisConfig: T3rnPrimitivesGatewayGenesisConfig;
    T3rnPrimitivesGatewaySysProps: T3rnPrimitivesGatewaySysProps;
    T3rnPrimitivesGatewayType: T3rnPrimitivesGatewayType;
    T3rnPrimitivesGatewayVendor: T3rnPrimitivesGatewayVendor;
    T3rnPrimitivesSideEffect: T3rnPrimitivesSideEffect;
    T3rnPrimitivesSideEffectConfirmedSideEffect: T3rnPrimitivesSideEffectConfirmedSideEffect;
    T3rnPrimitivesSideEffectFullSideEffect: T3rnPrimitivesSideEffectFullSideEffect;
    T3rnPrimitivesStorageRawAliveContractInfo: T3rnPrimitivesStorageRawAliveContractInfo;
    T3rnPrimitivesVolatileLocalState: T3rnPrimitivesVolatileLocalState;
  } // InterfaceTypes
} // declare module
