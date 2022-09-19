// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import "@polkadot/types/types/registry";

import type {
  CircuitStandaloneRuntimeOriginCaller,
  CircuitStandaloneRuntimeRuntime,
  EthereumLog,
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
  Pallet3vmCall,
  Pallet3vmError,
  Pallet3vmEvent,
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
  PalletCircuitStateCircuitStatus,
  PalletCircuitStateInsuranceDeposit,
  PalletCircuitStateXExecSignal,
  PalletClockError,
  PalletClockEvent,
  PalletContractsCall,
  PalletContractsError,
  PalletContractsEvent,
  PalletContractsRegistryCall,
  PalletContractsRegistryError,
  PalletContractsRegistryEvent,
  PalletContractsSchedule,
  PalletContractsScheduleHostFnWeights,
  PalletContractsScheduleInstructionWeights,
  PalletContractsScheduleLimits,
  PalletContractsStorageDeletedContract,
  PalletContractsStorageRawContractInfo,
  PalletContractsWasmOwnerInfo,
  PalletContractsWasmPrefabWasmModule,
  PalletEvmCall,
  PalletEvmError,
  PalletEvmEvent,
  PalletEvmThreeVmInfo,
  PalletGrandpaCall,
  PalletGrandpaError,
  PalletGrandpaEvent,
  PalletGrandpaFinalityVerifierBridgesHeaderChainAuthoritySet,
  PalletGrandpaFinalityVerifierError,
  PalletGrandpaFinalityVerifierParachain,
  PalletGrandpaStoredPendingChange,
  PalletGrandpaStoredState,
  PalletPortalCall,
  PalletPortalError,
  PalletPortalEvent,
  PalletSudoCall,
  PalletSudoError,
  PalletSudoEvent,
  PalletTimestampCall,
  PalletTransactionPaymentChargeTransactionPayment,
  PalletTransactionPaymentReleases,
  PalletTreasuryCall,
  PalletTreasuryError,
  PalletTreasuryEvent,
  PalletTreasuryInflationInflationInfo,
  PalletUtilityCall,
  PalletUtilityError,
  PalletUtilityEvent,
  PalletXbiPortalCall,
  PalletXbiPortalError,
  PalletXbiPortalEvent,
  PalletXbiPortalXbiFormat,
  PalletXbiPortalXbiFormatActionNotificationTimeouts,
  PalletXbiPortalXbiFormatXbiCheckIn,
  PalletXbiPortalXbiFormatXbiCheckOut,
  PalletXbiPortalXbiFormatXbiCheckOutStatus,
  PalletXbiPortalXbiFormatXbiInstr,
  PalletXbiPortalXbiFormatXbiMetadata,
  PalletXbiPortalXbiFormatXbiNotificationKind,
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
  SpRuntimeHeader,
  SpRuntimeModuleError,
  SpRuntimeMultiSignature,
  SpRuntimeTokenError,
  SpRuntimeTransactionalError,
  SpVersionRuntimeVersion,
  T3rnPrimitivesAccountManagerOutcome,
  T3rnPrimitivesAccountManagerRequestCharge,
  T3rnPrimitivesAccountManagerSettlement,
  T3rnPrimitivesClaimableBenefitSource,
  T3rnPrimitivesClaimableCircuitRole,
  T3rnPrimitivesClaimableClaimableArtifacts,
  T3rnPrimitivesCommonRoundInfo,
  T3rnPrimitivesContractMetadata,
  T3rnPrimitivesContractMetadataContractType,
  T3rnPrimitivesContractsRegistryAuthorInfo,
  T3rnPrimitivesContractsRegistryRegistryContract,
  T3rnPrimitivesGatewayGenesisConfig,
  T3rnPrimitivesGatewaySysProps,
  T3rnPrimitivesGatewayType,
  T3rnPrimitivesGatewayVendor,
  T3rnPrimitivesMonetaryBeneficiaryRole,
  T3rnPrimitivesMonetaryInflationAllocation,
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
  XcmDoubleEncoded,
  XcmV0JunctionBodyId,
  XcmV0JunctionBodyPart,
  XcmV0JunctionNetworkId,
  XcmV0OriginKind,
  XcmV1Junction,
  XcmV1MultiAsset,
  XcmV1MultiLocation,
  XcmV1MultiassetAssetId,
  XcmV1MultiassetAssetInstance,
  XcmV1MultiassetFungibility,
  XcmV1MultiassetMultiAssetFilter,
  XcmV1MultiassetMultiAssets,
  XcmV1MultiassetWildFungibility,
  XcmV1MultiassetWildMultiAsset,
  XcmV1MultilocationJunctions,
  XcmV2Instruction,
  XcmV2Response,
  XcmV2TraitsError,
  XcmV2WeightLimit,
  XcmV2Xcm,
} from "@polkadot/types/lookup";

declare module "@polkadot/types/types/registry" {
  interface InterfaceTypes {
    CircuitStandaloneRuntimeOriginCaller: CircuitStandaloneRuntimeOriginCaller;
    CircuitStandaloneRuntimeRuntime: CircuitStandaloneRuntimeRuntime;
    EthereumLog: EthereumLog;
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
    Pallet3vmCall: Pallet3vmCall;
    Pallet3vmError: Pallet3vmError;
    Pallet3vmEvent: Pallet3vmEvent;
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
    PalletCircuitStateCircuitStatus: PalletCircuitStateCircuitStatus;
    PalletCircuitStateInsuranceDeposit: PalletCircuitStateInsuranceDeposit;
    PalletCircuitStateXExecSignal: PalletCircuitStateXExecSignal;
    PalletClockError: PalletClockError;
    PalletClockEvent: PalletClockEvent;
    PalletContractsCall: PalletContractsCall;
    PalletContractsError: PalletContractsError;
    PalletContractsEvent: PalletContractsEvent;
    PalletContractsRegistryCall: PalletContractsRegistryCall;
    PalletContractsRegistryError: PalletContractsRegistryError;
    PalletContractsRegistryEvent: PalletContractsRegistryEvent;
    PalletContractsSchedule: PalletContractsSchedule;
    PalletContractsScheduleHostFnWeights: PalletContractsScheduleHostFnWeights;
    PalletContractsScheduleInstructionWeights: PalletContractsScheduleInstructionWeights;
    PalletContractsScheduleLimits: PalletContractsScheduleLimits;
    PalletContractsStorageDeletedContract: PalletContractsStorageDeletedContract;
    PalletContractsStorageRawContractInfo: PalletContractsStorageRawContractInfo;
    PalletContractsWasmOwnerInfo: PalletContractsWasmOwnerInfo;
    PalletContractsWasmPrefabWasmModule: PalletContractsWasmPrefabWasmModule;
    PalletEvmCall: PalletEvmCall;
    PalletEvmError: PalletEvmError;
    PalletEvmEvent: PalletEvmEvent;
    PalletEvmThreeVmInfo: PalletEvmThreeVmInfo;
    PalletGrandpaCall: PalletGrandpaCall;
    PalletGrandpaError: PalletGrandpaError;
    PalletGrandpaEvent: PalletGrandpaEvent;
    PalletGrandpaFinalityVerifierBridgesHeaderChainAuthoritySet: PalletGrandpaFinalityVerifierBridgesHeaderChainAuthoritySet;
    PalletGrandpaFinalityVerifierError: PalletGrandpaFinalityVerifierError;
    PalletGrandpaFinalityVerifierParachain: PalletGrandpaFinalityVerifierParachain;
    PalletGrandpaStoredPendingChange: PalletGrandpaStoredPendingChange;
    PalletGrandpaStoredState: PalletGrandpaStoredState;
    PalletPortalCall: PalletPortalCall;
    PalletPortalError: PalletPortalError;
    PalletPortalEvent: PalletPortalEvent;
    PalletSudoCall: PalletSudoCall;
    PalletSudoError: PalletSudoError;
    PalletSudoEvent: PalletSudoEvent;
    PalletTimestampCall: PalletTimestampCall;
    PalletTransactionPaymentChargeTransactionPayment: PalletTransactionPaymentChargeTransactionPayment;
    PalletTransactionPaymentReleases: PalletTransactionPaymentReleases;
    PalletTreasuryCall: PalletTreasuryCall;
    PalletTreasuryError: PalletTreasuryError;
    PalletTreasuryEvent: PalletTreasuryEvent;
    PalletTreasuryInflationInflationInfo: PalletTreasuryInflationInflationInfo;
    PalletUtilityCall: PalletUtilityCall;
    PalletUtilityError: PalletUtilityError;
    PalletUtilityEvent: PalletUtilityEvent;
    PalletXbiPortalCall: PalletXbiPortalCall;
    PalletXbiPortalError: PalletXbiPortalError;
    PalletXbiPortalEvent: PalletXbiPortalEvent;
    PalletXbiPortalXbiFormat: PalletXbiPortalXbiFormat;
    PalletXbiPortalXbiFormatActionNotificationTimeouts: PalletXbiPortalXbiFormatActionNotificationTimeouts;
    PalletXbiPortalXbiFormatXbiCheckIn: PalletXbiPortalXbiFormatXbiCheckIn;
    PalletXbiPortalXbiFormatXbiCheckOut: PalletXbiPortalXbiFormatXbiCheckOut;
    PalletXbiPortalXbiFormatXbiCheckOutStatus: PalletXbiPortalXbiFormatXbiCheckOutStatus;
    PalletXbiPortalXbiFormatXbiInstr: PalletXbiPortalXbiFormatXbiInstr;
    PalletXbiPortalXbiFormatXbiMetadata: PalletXbiPortalXbiFormatXbiMetadata;
    PalletXbiPortalXbiFormatXbiNotificationKind: PalletXbiPortalXbiFormatXbiNotificationKind;
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
    SpRuntimeHeader: SpRuntimeHeader;
    SpRuntimeModuleError: SpRuntimeModuleError;
    SpRuntimeMultiSignature: SpRuntimeMultiSignature;
    SpRuntimeTokenError: SpRuntimeTokenError;
    SpRuntimeTransactionalError: SpRuntimeTransactionalError;
    SpVersionRuntimeVersion: SpVersionRuntimeVersion;
    T3rnPrimitivesAccountManagerOutcome: T3rnPrimitivesAccountManagerOutcome;
    T3rnPrimitivesAccountManagerRequestCharge: T3rnPrimitivesAccountManagerRequestCharge;
    T3rnPrimitivesAccountManagerSettlement: T3rnPrimitivesAccountManagerSettlement;
    T3rnPrimitivesClaimableBenefitSource: T3rnPrimitivesClaimableBenefitSource;
    T3rnPrimitivesClaimableCircuitRole: T3rnPrimitivesClaimableCircuitRole;
    T3rnPrimitivesClaimableClaimableArtifacts: T3rnPrimitivesClaimableClaimableArtifacts;
    T3rnPrimitivesCommonRoundInfo: T3rnPrimitivesCommonRoundInfo;
    T3rnPrimitivesContractMetadata: T3rnPrimitivesContractMetadata;
    T3rnPrimitivesContractMetadataContractType: T3rnPrimitivesContractMetadataContractType;
    T3rnPrimitivesContractsRegistryAuthorInfo: T3rnPrimitivesContractsRegistryAuthorInfo;
    T3rnPrimitivesContractsRegistryRegistryContract: T3rnPrimitivesContractsRegistryRegistryContract;
    T3rnPrimitivesGatewayGenesisConfig: T3rnPrimitivesGatewayGenesisConfig;
    T3rnPrimitivesGatewaySysProps: T3rnPrimitivesGatewaySysProps;
    T3rnPrimitivesGatewayType: T3rnPrimitivesGatewayType;
    T3rnPrimitivesGatewayVendor: T3rnPrimitivesGatewayVendor;
    T3rnPrimitivesMonetaryBeneficiaryRole: T3rnPrimitivesMonetaryBeneficiaryRole;
    T3rnPrimitivesMonetaryInflationAllocation: T3rnPrimitivesMonetaryInflationAllocation;
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
    XcmDoubleEncoded: XcmDoubleEncoded;
    XcmV0JunctionBodyId: XcmV0JunctionBodyId;
    XcmV0JunctionBodyPart: XcmV0JunctionBodyPart;
    XcmV0JunctionNetworkId: XcmV0JunctionNetworkId;
    XcmV0OriginKind: XcmV0OriginKind;
    XcmV1Junction: XcmV1Junction;
    XcmV1MultiAsset: XcmV1MultiAsset;
    XcmV1MultiLocation: XcmV1MultiLocation;
    XcmV1MultiassetAssetId: XcmV1MultiassetAssetId;
    XcmV1MultiassetAssetInstance: XcmV1MultiassetAssetInstance;
    XcmV1MultiassetFungibility: XcmV1MultiassetFungibility;
    XcmV1MultiassetMultiAssetFilter: XcmV1MultiassetMultiAssetFilter;
    XcmV1MultiassetMultiAssets: XcmV1MultiassetMultiAssets;
    XcmV1MultiassetWildFungibility: XcmV1MultiassetWildFungibility;
    XcmV1MultiassetWildMultiAsset: XcmV1MultiassetWildMultiAsset;
    XcmV1MultilocationJunctions: XcmV1MultilocationJunctions;
    XcmV2Instruction: XcmV2Instruction;
    XcmV2Response: XcmV2Response;
    XcmV2TraitsError: XcmV2TraitsError;
    XcmV2WeightLimit: XcmV2WeightLimit;
    XcmV2Xcm: XcmV2Xcm;
  } // InterfaceTypes
} // declare module
