// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

import type { ApiTypes } from "@polkadot/api-base/types";

declare module "@polkadot/api-base/types/errors" {
  export interface AugmentedErrors<ApiType extends ApiTypes> {
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
      DeterminedForbiddenXtxStatus: AugmentedError<ApiType>;
      EnactSideEffectsCanOnlyBeCalledWithMin1StepFinished: AugmentedError<ApiType>;
      FatalCommitSideEffectWithoutConfirmationAttempt: AugmentedError<ApiType>;
      FatalErroredCommitSideEffectConfirmationAttempt: AugmentedError<ApiType>;
      FatalErroredRevertSideEffectConfirmationAttempt: AugmentedError<ApiType>;
      FatalXtxTimeoutXtxIdNotMatched: AugmentedError<ApiType>;
      InsuranceBondAlreadyDeposited: AugmentedError<ApiType>;
      InsuranceBondNotRequired: AugmentedError<ApiType>;
      InvalidLocalTrigger: AugmentedError<ApiType>;
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
      SideEffectsValidationFailed: AugmentedError<ApiType>;
      UnsupportedRole: AugmentedError<ApiType>;
      /** Generic error */
      [key: string]: AugmentedError<ApiType>;
    };
    circuitPortal: {
      ContractDoesNotExists: AugmentedError<ApiType>;
      /** Non existent public key. */
      InvalidKey: AugmentedError<ApiType>;
      IOScheduleEmpty: AugmentedError<ApiType>;
      IOScheduleNoEndingSemicolon: AugmentedError<ApiType>;
      IOScheduleUnknownCompose: AugmentedError<ApiType>;
      NoParachainEntryFound: AugmentedError<ApiType>;
      ParachainHeaderNotVerified: AugmentedError<ApiType>;
      ProcessStepGatewayNotRecognised: AugmentedError<ApiType>;
      RequesterNotEnoughBalance: AugmentedError<ApiType>;
      SideEffectConfirmationInvalidInclusionProof: AugmentedError<ApiType>;
      SideEffectTypeNotRecognized: AugmentedError<ApiType>;
      StepConfirmationBlockUnrecognised: AugmentedError<ApiType>;
      StepConfirmationDecodingError: AugmentedError<ApiType>;
      StepConfirmationGatewayNotRecognised: AugmentedError<ApiType>;
      VendorUnknown: AugmentedError<ApiType>;
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
      /** The input data could not be validated */
      InvalidSideEffect: AugmentedError<ApiType>;
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
      /** The operation is not allowed when the execution is volatile. */
      NotAllowedInVolatileMode: AugmentedError<ApiType>;
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
    multiFinalityVerifierDefault: {
      /** The pallet has already been initialized. */
      AlreadyInitialized: AugmentedError<ApiType>;
      /** All pallet operations are halted. */
      Halted: AugmentedError<ApiType>;
      InvalidAnchorHeader: AugmentedError<ApiType>;
      /** The authority set from the underlying header chain is invalid. */
      InvalidAuthoritySet: AugmentedError<ApiType>;
      /** The given justification is invalid for the given header. */
      InvalidJustification: AugmentedError<ApiType>;
      NoFinalizedHeader: AugmentedError<ApiType>;
      NoParachainEntryFound: AugmentedError<ApiType>;
      /**
       * The header being imported is older than the best finalized header known
       * to the pallet.
       */
      OldHeader: AugmentedError<ApiType>;
      /**
       * The storage proof doesn't contains storage root. So it is invalid for
       * given header.
       */
      StorageRootMismatch: AugmentedError<ApiType>;
      /** There are too many requests for the current window to handle. */
      TooManyRequests: AugmentedError<ApiType>;
      /** The header is unknown to the pallet. */
      UnknownHeader: AugmentedError<ApiType>;
      /**
       * The scheduled authority set change found in the header is unsupported
       * by the pallet.
       *
       * This is the case for non-standard (e.g forced) authority set changes.
       */
      UnsupportedScheduledChange: AugmentedError<ApiType>;
      /** Generic error */
      [key: string]: AugmentedError<ApiType>;
    };
    multiFinalityVerifierEthereumLike: {
      /** The pallet has already been initialized. */
      AlreadyInitialized: AugmentedError<ApiType>;
      /** All pallet operations are halted. */
      Halted: AugmentedError<ApiType>;
      InvalidAnchorHeader: AugmentedError<ApiType>;
      /** The authority set from the underlying header chain is invalid. */
      InvalidAuthoritySet: AugmentedError<ApiType>;
      /** The given justification is invalid for the given header. */
      InvalidJustification: AugmentedError<ApiType>;
      NoFinalizedHeader: AugmentedError<ApiType>;
      NoParachainEntryFound: AugmentedError<ApiType>;
      /**
       * The header being imported is older than the best finalized header known
       * to the pallet.
       */
      OldHeader: AugmentedError<ApiType>;
      /**
       * The storage proof doesn't contains storage root. So it is invalid for
       * given header.
       */
      StorageRootMismatch: AugmentedError<ApiType>;
      /** There are too many requests for the current window to handle. */
      TooManyRequests: AugmentedError<ApiType>;
      /** The header is unknown to the pallet. */
      UnknownHeader: AugmentedError<ApiType>;
      /**
       * The scheduled authority set change found in the header is unsupported
       * by the pallet.
       *
       * This is the case for non-standard (e.g forced) authority set changes.
       */
      UnsupportedScheduledChange: AugmentedError<ApiType>;
      /** Generic error */
      [key: string]: AugmentedError<ApiType>;
    };
    multiFinalityVerifierGenericLike: {
      /** The pallet has already been initialized. */
      AlreadyInitialized: AugmentedError<ApiType>;
      /** All pallet operations are halted. */
      Halted: AugmentedError<ApiType>;
      InvalidAnchorHeader: AugmentedError<ApiType>;
      /** The authority set from the underlying header chain is invalid. */
      InvalidAuthoritySet: AugmentedError<ApiType>;
      /** The given justification is invalid for the given header. */
      InvalidJustification: AugmentedError<ApiType>;
      NoFinalizedHeader: AugmentedError<ApiType>;
      NoParachainEntryFound: AugmentedError<ApiType>;
      /**
       * The header being imported is older than the best finalized header known
       * to the pallet.
       */
      OldHeader: AugmentedError<ApiType>;
      /**
       * The storage proof doesn't contains storage root. So it is invalid for
       * given header.
       */
      StorageRootMismatch: AugmentedError<ApiType>;
      /** There are too many requests for the current window to handle. */
      TooManyRequests: AugmentedError<ApiType>;
      /** The header is unknown to the pallet. */
      UnknownHeader: AugmentedError<ApiType>;
      /**
       * The scheduled authority set change found in the header is unsupported
       * by the pallet.
       *
       * This is the case for non-standard (e.g forced) authority set changes.
       */
      UnsupportedScheduledChange: AugmentedError<ApiType>;
      /** Generic error */
      [key: string]: AugmentedError<ApiType>;
    };
    multiFinalityVerifierPolkadotLike: {
      /** The pallet has already been initialized. */
      AlreadyInitialized: AugmentedError<ApiType>;
      /** All pallet operations are halted. */
      Halted: AugmentedError<ApiType>;
      InvalidAnchorHeader: AugmentedError<ApiType>;
      /** The authority set from the underlying header chain is invalid. */
      InvalidAuthoritySet: AugmentedError<ApiType>;
      /** The given justification is invalid for the given header. */
      InvalidJustification: AugmentedError<ApiType>;
      NoFinalizedHeader: AugmentedError<ApiType>;
      NoParachainEntryFound: AugmentedError<ApiType>;
      /**
       * The header being imported is older than the best finalized header known
       * to the pallet.
       */
      OldHeader: AugmentedError<ApiType>;
      /**
       * The storage proof doesn't contains storage root. So it is invalid for
       * given header.
       */
      StorageRootMismatch: AugmentedError<ApiType>;
      /** There are too many requests for the current window to handle. */
      TooManyRequests: AugmentedError<ApiType>;
      /** The header is unknown to the pallet. */
      UnknownHeader: AugmentedError<ApiType>;
      /**
       * The scheduled authority set change found in the header is unsupported
       * by the pallet.
       *
       * This is the case for non-standard (e.g forced) authority set changes.
       */
      UnsupportedScheduledChange: AugmentedError<ApiType>;
      /** Generic error */
      [key: string]: AugmentedError<ApiType>;
    };
    multiFinalityVerifierSubstrateLike: {
      /** The pallet has already been initialized. */
      AlreadyInitialized: AugmentedError<ApiType>;
      /** All pallet operations are halted. */
      Halted: AugmentedError<ApiType>;
      InvalidAnchorHeader: AugmentedError<ApiType>;
      /** The authority set from the underlying header chain is invalid. */
      InvalidAuthoritySet: AugmentedError<ApiType>;
      /** The given justification is invalid for the given header. */
      InvalidJustification: AugmentedError<ApiType>;
      NoFinalizedHeader: AugmentedError<ApiType>;
      NoParachainEntryFound: AugmentedError<ApiType>;
      /**
       * The header being imported is older than the best finalized header known
       * to the pallet.
       */
      OldHeader: AugmentedError<ApiType>;
      /**
       * The storage proof doesn't contains storage root. So it is invalid for
       * given header.
       */
      StorageRootMismatch: AugmentedError<ApiType>;
      /** There are too many requests for the current window to handle. */
      TooManyRequests: AugmentedError<ApiType>;
      /** The header is unknown to the pallet. */
      UnknownHeader: AugmentedError<ApiType>;
      /**
       * The scheduled authority set change found in the header is unsupported
       * by the pallet.
       *
       * This is the case for non-standard (e.g forced) authority set changes.
       */
      UnsupportedScheduledChange: AugmentedError<ApiType>;
      /** Generic error */
      [key: string]: AugmentedError<ApiType>;
    };
    ormlTokens: {
      /** Cannot convert Amount into Balance type */
      AmountIntoBalanceFailed: AugmentedError<ApiType>;
      /** The balance is too low */
      BalanceTooLow: AugmentedError<ApiType>;
      /** Beneficiary account must pre-exist */
      DeadAccount: AugmentedError<ApiType>;
      /** Value too low to create account due to existential deposit */
      ExistentialDeposit: AugmentedError<ApiType>;
      /** Transfer/payment would kill account */
      KeepAlive: AugmentedError<ApiType>;
      /** Failed because liquidity restrictions due to locking */
      LiquidityRestrictions: AugmentedError<ApiType>;
      /** Failed because the maximum locks was exceeded */
      MaxLocksExceeded: AugmentedError<ApiType>;
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
    utility: {
      /** Too many calls batched. */
      TooManyCalls: AugmentedError<ApiType>;
      /** Generic error */
      [key: string]: AugmentedError<ApiType>;
    };
    xdns: {
      /** SideEffect already stored */
      SideEffectInterfaceAlreadyExists: AugmentedError<ApiType>;
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
