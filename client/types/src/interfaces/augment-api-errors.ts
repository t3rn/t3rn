// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import "@polkadot/api-base/types/errors";

import type { ApiTypes, AugmentedError } from "@polkadot/api-base/types";

export type __AugmentedError<ApiType extends ApiTypes> =
  AugmentedError<ApiType>;

declare module "@polkadot/api-base/types/errors" {
  interface AugmentedErrors<ApiType extends ApiTypes> {
    accountManager: {
      ChargeAlreadyRegistered: AugmentedError<ApiType>;
      ChargeOrSettlementCalculationOverflow: AugmentedError<ApiType>;
      DecodingExecutionIDFailed: AugmentedError<ApiType>;
      ExecutionAlreadyRegistered: AugmentedError<ApiType>;
      ExecutionNotRegistered: AugmentedError<ApiType>;
      NoChargeOfGivenIdRegistered: AugmentedError<ApiType>;
      PendingChargeNotFoundAtCommit: AugmentedError<ApiType>;
      PendingChargeNotFoundAtRefund: AugmentedError<ApiType>;
      SkippingEmptyCharges: AugmentedError<ApiType>;
      /** Generic error */
      [key: string]: AugmentedError<ApiType>;
    };
    assets: {
      /** The asset-account already exists. */
      AlreadyExists: AugmentedError<ApiType>;
      /** Invalid metadata given. */
      BadMetadata: AugmentedError<ApiType>;
      /** Invalid witness data given. */
      BadWitness: AugmentedError<ApiType>;
      /** Account balance must be greater than or equal to the transfer amount. */
      BalanceLow: AugmentedError<ApiType>;
      /** The origin account is frozen. */
      Frozen: AugmentedError<ApiType>;
      /** The asset ID is already taken. */
      InUse: AugmentedError<ApiType>;
      /** Minimum balance should be non-zero. */
      MinBalanceZero: AugmentedError<ApiType>;
      /** The account to alter does not exist. */
      NoAccount: AugmentedError<ApiType>;
      /** The asset-account doesn't have an associated deposit. */
      NoDeposit: AugmentedError<ApiType>;
      /** The signing account has no permission to do the operation. */
      NoPermission: AugmentedError<ApiType>;
      /**
       * Unable to increment the consumer reference counters on the account.
       * Either no provider reference exists to allow a non-zero balance of a
       * non-self-sufficient asset, or the maximum number of consumers has been reached.
       */
      NoProvider: AugmentedError<ApiType>;
      /** No approval exists that would allow the transfer. */
      Unapproved: AugmentedError<ApiType>;
      /** The given asset ID is unknown. */
      Unknown: AugmentedError<ApiType>;
      /** The operation would result in funds being burned. */
      WouldBurn: AugmentedError<ApiType>;
      /** The source account would not survive the transfer and it needs to stay alive. */
      WouldDie: AugmentedError<ApiType>;
      /** Generic error */
      [key: string]: AugmentedError<ApiType>;
    };
    balances: {
      /** Beneficiary account must pre-exist */
      DeadAccount: AugmentedError<ApiType>;
      /** Value too low to create account due to existential deposit */
      ExistentialDeposit: AugmentedError<ApiType>;
      /** A vesting schedule already exists for this account */
      ExistingVestingSchedule: AugmentedError<ApiType>;
      /** Balance too low to send value */
      InsufficientBalance: AugmentedError<ApiType>;
      /** Transfer/payment would kill account */
      KeepAlive: AugmentedError<ApiType>;
      /** Account liquidity restrictions prevent withdrawal */
      LiquidityRestrictions: AugmentedError<ApiType>;
      /** Number of named reserves exceed MaxReserves */
      TooManyReserves: AugmentedError<ApiType>;
      /** Vesting balance too high to send value */
      VestingBalance: AugmentedError<ApiType>;
      /** Generic error */
      [key: string]: AugmentedError<ApiType>;
    };
    circuit: {
      ApplyFailed: AugmentedError<ApiType>;
      ApplyTriggeredWithUnexpectedStatus: AugmentedError<ApiType>;
      ChargingTransferFailed: AugmentedError<ApiType>;
      ContractXtxKilledRunOutOfFunds: AugmentedError<ApiType>;
      CriticalStateSquareUpCalledToFinishWithoutFsxConfirmed: AugmentedError<ApiType>;
      DeterminedForbiddenXtxStatus: AugmentedError<ApiType>;
      EnactSideEffectsCanOnlyBeCalledWithMin1StepFinished: AugmentedError<ApiType>;
      FailedToCheckInOverXBI: AugmentedError<ApiType>;
      FailedToConvertSFX2XBI: AugmentedError<ApiType>;
      FailedToConvertXBIResult2SFXConfirmation: AugmentedError<ApiType>;
      FailedToCreateXBIMetadataDueToWrongAccountConversion: AugmentedError<ApiType>;
      FailedToEnterXBIPortal: AugmentedError<ApiType>;
      FailedToExitXBIPortal: AugmentedError<ApiType>;
      FailedToHardenFullSideEffect: AugmentedError<ApiType>;
      FatalCommitSideEffectWithoutConfirmationAttempt: AugmentedError<ApiType>;
      FatalErroredCommitSideEffectConfirmationAttempt: AugmentedError<ApiType>;
      FatalErroredRevertSideEffectConfirmationAttempt: AugmentedError<ApiType>;
      FatalXtxTimeoutXtxIdNotMatched: AugmentedError<ApiType>;
      FinalizeSquareUpFailed: AugmentedError<ApiType>;
      InsuranceBondAlreadyDeposited: AugmentedError<ApiType>;
      InsuranceBondNotRequired: AugmentedError<ApiType>;
      InsuranceBondTooLow: AugmentedError<ApiType>;
      InvalidLocalTrigger: AugmentedError<ApiType>;
      LocalExecutionUnauthorized: AugmentedError<ApiType>;
      LocalSideEffectExecutionNotApplicable: AugmentedError<ApiType>;
      RefundTransferFailed: AugmentedError<ApiType>;
      RelayEscrowedFailedNothingToConfirm: AugmentedError<ApiType>;
      RequesterNotEnoughBalance: AugmentedError<ApiType>;
      RewardTransferFailed: AugmentedError<ApiType>;
      SetupFailed: AugmentedError<ApiType>;
      SetupFailedDuplicatedXtx: AugmentedError<ApiType>;
      SetupFailedEmptyXtx: AugmentedError<ApiType>;
      SetupFailedIncorrectXtxStatus: AugmentedError<ApiType>;
      SetupFailedUnknownXtx: AugmentedError<ApiType>;
      SetupFailedXtxNotFound: AugmentedError<ApiType>;
      SetupFailedXtxStorageArtifactsNotFound: AugmentedError<ApiType>;
      SideEffectIsAlreadyScheduledToExecuteOverXBI: AugmentedError<ApiType>;
      SideEffectsValidationFailed: AugmentedError<ApiType>;
      SignalQueueFull: AugmentedError<ApiType>;
      UnsupportedRole: AugmentedError<ApiType>;
      UpdateXtxTriggeredWithUnexpectedStatus: AugmentedError<ApiType>;
      XBIExitFailedOnSFXConfirmation: AugmentedError<ApiType>;
      /** Generic error */
      [key: string]: AugmentedError<ApiType>;
    };
    clock: {
      /** Generic error */
      [key: string]: AugmentedError<ApiType>;
    };
    contracts: {
      /**
       * Code removal was denied because the code is still in use by at least
       * one contract.
       */
      CodeInUse: AugmentedError<ApiType>;
      /** No code could be found at the supplied code hash. */
      CodeNotFound: AugmentedError<ApiType>;
      /**
       * The contract's code was found to be invalid during validation or
       * instrumentation. A more detailed error can be found on the node console
       * if debug messages are enabled or in the debug buffer which is returned
       * to RPC clients.
       */
      CodeRejected: AugmentedError<ApiType>;
      /**
       * The code supplied to `instantiate_with_code` exceeds the limit
       * specified in the current schedule.
       */
      CodeTooLarge: AugmentedError<ApiType>;
      /** No contract was found at the specified address. */
      ContractNotFound: AugmentedError<ApiType>;
      /**
       * The contract ran to completion but decided to revert its storage
       * changes. Please note that this error is only returned from extrinsics.
       * When called directly or via RPC an `Ok` will be returned. In this case
       * the caller needs to inspect the flags to determine whether a reversion
       * has taken place.
       */
      ContractReverted: AugmentedError<ApiType>;
      /** Contract trapped during execution. */
      ContractTrapped: AugmentedError<ApiType>;
      /** The debug message specified to `seal_debug_message` does contain invalid UTF-8. */
      DebugMessageInvalidUTF8: AugmentedError<ApiType>;
      /** Input passed to a contract API function failed to decode as expected type. */
      DecodingFailed: AugmentedError<ApiType>;
      /**
       * Removal of a contract failed because the deletion queue is full.
       *
       * This can happen when calling `seal_terminate`. The queue is filled by
       * deleting contracts and emptied by a fixed amount each block. Trying
       * again during another block is the only way to resolve this issue.
       */
      DeletionQueueFull: AugmentedError<ApiType>;
      /** A contract with the same AccountId already exists. */
      DuplicateContract: AugmentedError<ApiType>;
      /** The topics passed to `seal_deposit_events` contains at least one duplicate. */
      DuplicateTopics: AugmentedError<ApiType>;
      /** `seal_call` forwarded this contracts input. It therefore is no longer available. */
      InputForwarded: AugmentedError<ApiType>;
      /** Invalid combination of flags supplied to `seal_call` or `seal_delegate_call`. */
      InvalidCallFlags: AugmentedError<ApiType>;
      /** A new schedule must have a greater version than the current one. */
      InvalidScheduleVersion: AugmentedError<ApiType>;
      /**
       * Performing a call was denied because the calling depth reached the
       * limit of what is specified in the schedule.
       */
      MaxCallDepthReached: AugmentedError<ApiType>;
      /**
       * The chain does not provide a chain extension. Calling the chain
       * extension results in this error. Note that this usually shouldn't
       * happen as deploying such contracts is rejected.
       */
      NoChainExtension: AugmentedError<ApiType>;
      /** Failed to unwrap local state */
      NoStateReturned: AugmentedError<ApiType>;
      /** A buffer outside of sandbox memory was passed to a contract API function. */
      OutOfBounds: AugmentedError<ApiType>;
      /** The executed contract exhausted its gas limit. */
      OutOfGas: AugmentedError<ApiType>;
      /** The output buffer supplied to a contract API call was too small. */
      OutputBufferTooSmall: AugmentedError<ApiType>;
      /** The subject passed to `seal_random` exceeds the limit. */
      RandomSubjectTooLong: AugmentedError<ApiType>;
      /** A call tried to invoke a contract that is flagged as non-reentrant. */
      ReentranceDenied: AugmentedError<ApiType>;
      /** More storage was created than allowed by the storage deposit limit. */
      StorageDepositLimitExhausted: AugmentedError<ApiType>;
      /** Origin doesn't have enough balance to pay the required storage deposits. */
      StorageDepositNotEnoughFunds: AugmentedError<ApiType>;
      /**
       * A contract self destructed in its constructor.
       *
       * This can be triggered by a call to `seal_terminate`.
       */
      TerminatedInConstructor: AugmentedError<ApiType>;
      /**
       * Termination of a contract is not allowed while the contract is already
       * on the call stack. Can be triggered by `seal_terminate`.
       */
      TerminatedWhileReentrant: AugmentedError<ApiType>;
      /** The amount of topics passed to `seal_deposit_events` exceeds the limit. */
      TooManyTopics: AugmentedError<ApiType>;
      /**
       * Performing the requested transfer failed. Probably because there isn't
       * enough free balance in the sender's account.
       */
      TransferFailed: AugmentedError<ApiType>;
      /** The size defined in `T::MaxValueSize` was exceeded. */
      ValueTooLarge: AugmentedError<ApiType>;
      /** Generic error */
      [key: string]: AugmentedError<ApiType>;
    };
    contractsRegistry: {
      /** Stored contract has already been added before */
      ContractAlreadyExists: AugmentedError<ApiType>;
      /** Access of unknown contract */
      UnknownContract: AugmentedError<ApiType>;
      /** Generic error */
      [key: string]: AugmentedError<ApiType>;
    };
    evm: {
      /** Not enough balance to perform action */
      BalanceLow: AugmentedError<ApiType>;
      CreatedFailed: AugmentedError<ApiType>;
      ExecutedFailed: AugmentedError<ApiType>;
      /** Calculating total fee overflowed */
      FeeOverflow: AugmentedError<ApiType>;
      /** Gas price is too low. */
      GasPriceTooLow: AugmentedError<ApiType>;
      /** Nonce is invalid */
      InvalidNonce: AugmentedError<ApiType>;
      /** Tried to instantiate a contract with an invalid hash */
      InvalidRegistryHash: AugmentedError<ApiType>;
      /** Calculating total payment overflowed */
      PaymentOverflow: AugmentedError<ApiType>;
      /** 3VM failed to remunerate author */
      RemunerateAuthor: AugmentedError<ApiType>;
      /** Withdraw fee failed */
      WithdrawFailed: AugmentedError<ApiType>;
      /** Generic error */
      [key: string]: AugmentedError<ApiType>;
    };
    grandpa: {
      /** Attempt to signal GRANDPA change with one already pending. */
      ChangePending: AugmentedError<ApiType>;
      /** A given equivocation report is valid but already previously reported. */
      DuplicateOffenceReport: AugmentedError<ApiType>;
      /** An equivocation proof provided as part of an equivocation report is invalid. */
      InvalidEquivocationProof: AugmentedError<ApiType>;
      /** A key ownership proof provided as part of an equivocation report is invalid. */
      InvalidKeyOwnershipProof: AugmentedError<ApiType>;
      /**
       * Attempt to signal GRANDPA pause when the authority set isn't live
       * (either paused or already pending pause).
       */
      PauseFailed: AugmentedError<ApiType>;
      /**
       * Attempt to signal GRANDPA resume when the authority set isn't paused
       * (either live or already pending resume).
       */
      ResumeFailed: AugmentedError<ApiType>;
      /** Cannot signal forced change so soon after last. */
      TooSoon: AugmentedError<ApiType>;
      /** Generic error */
      [key: string]: AugmentedError<ApiType>;
    };
    portal: {
      /** The gateways vendor is not available, which is a result of a missing XDNS record. */
      GatewayVendorNotFound: AugmentedError<ApiType>;
      /** No gateway height could be found */
      NoGatewayHeightAvailable: AugmentedError<ApiType>;
      /** Gateway registration failed */
      RegistrationError: AugmentedError<ApiType>;
      /** Finality Verifiers operational status can't be updated */
      SetOperationalError: AugmentedError<ApiType>;
      /** Finality Verifier owner can't be set. */
      SetOwnerError: AugmentedError<ApiType>;
      /** SideEffect confirmation failed */
      SideEffectConfirmationFailed: AugmentedError<ApiType>;
      /** The header could not be added */
      SubmitHeaderError: AugmentedError<ApiType>;
      /** Specified Vendor is not implemented */
      UnimplementedGatewayVendor: AugmentedError<ApiType>;
      /** The creation of the XDNS record was not successful */
      XdnsRecordCreationFailed: AugmentedError<ApiType>;
      /** Generic error */
      [key: string]: AugmentedError<ApiType>;
    };
    rococoBridge: {
      /** The block height couldn't be converted */
      BlockHeightConversionError: AugmentedError<ApiType>;
      /** The submitted range is empty */
      EmptyRangeSubmitted: AugmentedError<ApiType>;
      /** The events paramaters couldn't be decoded */
      EventDecodingFailed: AugmentedError<ApiType>;
      /** The event was not found in the specified block */
      EventNotIncluded: AugmentedError<ApiType>;
      /** The pallet is currently halted */
      Halted: AugmentedError<ApiType>;
      /** The given bytes couldn't be decoded as header data */
      HeaderDataDecodingError: AugmentedError<ApiType>;
      /** The given bytes couldn't be decoded as a header */
      HeaderDecodingError: AugmentedError<ApiType>;
      /** The inclusion data couldn't be decoded */
      InclusionDataDecodeError: AugmentedError<ApiType>;
      /** The authority set in invalid */
      InvalidAuthoritySet: AugmentedError<ApiType>;
      /** The submitted GrandpaJustification is not valid */
      InvalidGrandpaJustification: AugmentedError<ApiType>;
      /** The linkage with the justified header is not valid */
      InvalidJustificationLinkage: AugmentedError<ApiType>;
      /** The header range linkage is not valid */
      InvalidRangeLinkage: AugmentedError<ApiType>;
      /** The submitted storage proof is invalid */
      InvalidStorageProof: AugmentedError<ApiType>;
      /** No finalized header was found in storage */
      NoFinalizedHeader: AugmentedError<ApiType>;
      /** The parachain entry was not found in storage */
      ParachainEntryNotFound: AugmentedError<ApiType>;
      /** The submitted range is larger the HeadersToStore, which is not permitted */
      RangeToLarge: AugmentedError<ApiType>;
      /** The headers storage root doesn't map the supplied once */
      StorageRootMismatch: AugmentedError<ApiType>;
      /**
       * The relaychains storge root was not found. This implies the header is
       * not available
       */
      StorageRootNotFound: AugmentedError<ApiType>;
      /** The header couldn't be found in storage */
      UnknownHeader: AugmentedError<ApiType>;
      /** The side effect is not known for this vendor */
      UnkownSideEffect: AugmentedError<ApiType>;
      /** A forced change was detected, which is not supported */
      UnsupportedScheduledChange: AugmentedError<ApiType>;
      /** Generic error */
      [key: string]: AugmentedError<ApiType>;
    };
    sudo: {
      /** Sender must be the Sudo account */
      RequireSudo: AugmentedError<ApiType>;
      /** Generic error */
      [key: string]: AugmentedError<ApiType>;
    };
    system: {
      /** The origin filter prevent the call to be dispatched. */
      CallFiltered: AugmentedError<ApiType>;
      /**
       * Failed to extract the runtime version from the new runtime.
       *
       * Either calling `Core_version` or decoding `RuntimeVersion` failed.
       */
      FailedToExtractRuntimeVersion: AugmentedError<ApiType>;
      /**
       * The name of specification does not match between the current runtime
       * and the new runtime.
       */
      InvalidSpecName: AugmentedError<ApiType>;
      /** Suicide called when the account has non-default composite data. */
      NonDefaultComposite: AugmentedError<ApiType>;
      /** There is a non-zero reference count preventing the account from being purged. */
      NonZeroRefCount: AugmentedError<ApiType>;
      /**
       * The specification version is not allowed to decrease between the
       * current runtime and the new runtime.
       */
      SpecVersionNeedsToIncrease: AugmentedError<ApiType>;
      /** Generic error */
      [key: string]: AugmentedError<ApiType>;
    };
    threeVm: {
      /** The contract cannot be instantiated due to its type */
      CannotInstantiateContract: AugmentedError<ApiType>;
      /** You can't submit side effects without any side effects */
      CannotTriggerWithoutSideEffects: AugmentedError<ApiType>;
      /** The contract cannot generate side effects due to its type */
      ContractCannotGenerateSideEffects: AugmentedError<ApiType>;
      /** The contract cannot have storage due to its type */
      ContractCannotHaveStorage: AugmentedError<ApiType>;
      /** The contract cannot remunerate due to its type */
      ContractCannotRemunerate: AugmentedError<ApiType>;
      /** The contract could not be found in the registry */
      ContractNotFound: AugmentedError<ApiType>;
      /** A user exceeded the bounce threshold for submitting signals */
      ExceededSignalBounceThreshold: AugmentedError<ApiType>;
      /** An origin could not be extracted from the buffer */
      InvalidOrigin: AugmentedError<ApiType>;
      /** Invalid precompile arguments */
      InvalidPrecompileArgs: AugmentedError<ApiType>;
      /** The precompile pointer was invalid */
      InvalidPrecompilePointer: AugmentedError<ApiType>;
      /** Generic error */
      [key: string]: AugmentedError<ApiType>;
    };
    treasury: {
      InvalidInflationAllocation: AugmentedError<ApiType>;
      InvalidInflationConfig: AugmentedError<ApiType>;
      NoRewardsAvailable: AugmentedError<ApiType>;
      NotBeneficiary: AugmentedError<ApiType>;
      RoundTermTooShort: AugmentedError<ApiType>;
      ValueNotChanged: AugmentedError<ApiType>;
      /** Generic error */
      [key: string]: AugmentedError<ApiType>;
    };
    utility: {
      /** Too many calls batched. */
      TooManyCalls: AugmentedError<ApiType>;
      /** Generic error */
      [key: string]: AugmentedError<ApiType>;
    };
    xbiPortal: {
      EnterFailedOnMultiLocationTransform: AugmentedError<ApiType>;
      EnterFailedOnXcmSend: AugmentedError<ApiType>;
      ExitUnhandled: AugmentedError<ApiType>;
      No3VMSupportedAtDest: AugmentedError<ApiType>;
      NoAddLiquiditySupportedAtDest: AugmentedError<ApiType>;
      NoEVMSupportedAtDest: AugmentedError<ApiType>;
      NoSwapSupportedAtDest: AugmentedError<ApiType>;
      NoTransferAssetsSupportedAtDest: AugmentedError<ApiType>;
      NoTransferEscrowSupportedAtDest: AugmentedError<ApiType>;
      NoTransferMultiEscrowSupportedAtDest: AugmentedError<ApiType>;
      NoTransferORMLSupportedAtDest: AugmentedError<ApiType>;
      NoTransferSupportedAtDest: AugmentedError<ApiType>;
      NoWASMSupportedAtDest: AugmentedError<ApiType>;
      NoXBICallbackSupported: AugmentedError<ApiType>;
      XBIABIFailedToCastBetweenTypesAddress: AugmentedError<ApiType>;
      XBIABIFailedToCastBetweenTypesValue: AugmentedError<ApiType>;
      XBIAlreadyCheckedIn: AugmentedError<ApiType>;
      XBIInstructionNotAllowedHere: AugmentedError<ApiType>;
      XBINotificationTimeOutDelivery: AugmentedError<ApiType>;
      XBINotificationTimeOutExecution: AugmentedError<ApiType>;
      /** Generic error */
      [key: string]: AugmentedError<ApiType>;
    };
    xdns: {
      /** The xdns entry does not contain parachain information */
      NoParachainInfoFound: AugmentedError<ApiType>;
      /** SideEffect already stored */
      SideEffectInterfaceAlreadyExists: AugmentedError<ApiType>;
      /** SideEffect interface was not found in storage */
      SideEffectInterfaceNotFound: AugmentedError<ApiType>;
      /** Access of unknown xdns_record */
      UnknownXdnsRecord: AugmentedError<ApiType>;
      /** Stored xdns_record has already been added before */
      XdnsRecordAlreadyExists: AugmentedError<ApiType>;
      /** Xdns Record not found */
      XdnsRecordNotFound: AugmentedError<ApiType>;
      /** Generic error */
      [key: string]: AugmentedError<ApiType>;
    };
  } // AugmentedErrors
} // declare module
