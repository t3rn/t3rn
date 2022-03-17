// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

import type { ApiTypes } from '@polkadot/api-base/types';

declare module '@polkadot/api-base/types/errors' {
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
      ChargingTransferFailed: AugmentedError<ApiType>;
      DeterminedForbiddenXtxStatus: AugmentedError<ApiType>;
      InsuranceBondAlreadyDeposited: AugmentedError<ApiType>;
      InsuranceBondNotRequired: AugmentedError<ApiType>;
      RefundTransferFailed: AugmentedError<ApiType>;
      RequesterNotEnoughBalance: AugmentedError<ApiType>;
      RewardTransferFailed: AugmentedError<ApiType>;
      SetupFailed: AugmentedError<ApiType>;
      SetupFailedDuplicatedXtx: AugmentedError<ApiType>;
      SetupFailedEmptyXtx: AugmentedError<ApiType>;
      SetupFailedIncorrectXtxStatus: AugmentedError<ApiType>;
      SetupFailedUnknownXtx: AugmentedError<ApiType>;
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
      /** Attempt to signal GRANDPA pause when the authority set isn't live (either paused or already pending pause). */
      PauseFailed: AugmentedError<ApiType>;
      /** Attempt to signal GRANDPA resume when the authority set isn't paused (either live or already pending resume). */
      ResumeFailed: AugmentedError<ApiType>;
      /** Cannot signal forced change so soon after last. */
      TooSoon: AugmentedError<ApiType>;
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
      /** The header being imported is older than the best finalized header known to the pallet. */
      OldHeader: AugmentedError<ApiType>;
      /** The storage proof doesn't contains storage root. So it is invalid for given header. */
      StorageRootMismatch: AugmentedError<ApiType>;
      /** There are too many requests for the current window to handle. */
      TooManyRequests: AugmentedError<ApiType>;
      /** The header is unknown to the pallet. */
      UnknownHeader: AugmentedError<ApiType>;
      /**
       * The scheduled authority set change found in the header is unsupported by the pallet.
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
      /** The header being imported is older than the best finalized header known to the pallet. */
      OldHeader: AugmentedError<ApiType>;
      /** The storage proof doesn't contains storage root. So it is invalid for given header. */
      StorageRootMismatch: AugmentedError<ApiType>;
      /** There are too many requests for the current window to handle. */
      TooManyRequests: AugmentedError<ApiType>;
      /** The header is unknown to the pallet. */
      UnknownHeader: AugmentedError<ApiType>;
      /**
       * The scheduled authority set change found in the header is unsupported by the pallet.
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
      /** The header being imported is older than the best finalized header known to the pallet. */
      OldHeader: AugmentedError<ApiType>;
      /** The storage proof doesn't contains storage root. So it is invalid for given header. */
      StorageRootMismatch: AugmentedError<ApiType>;
      /** There are too many requests for the current window to handle. */
      TooManyRequests: AugmentedError<ApiType>;
      /** The header is unknown to the pallet. */
      UnknownHeader: AugmentedError<ApiType>;
      /**
       * The scheduled authority set change found in the header is unsupported by the pallet.
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
      /** The header being imported is older than the best finalized header known to the pallet. */
      OldHeader: AugmentedError<ApiType>;
      /** The storage proof doesn't contains storage root. So it is invalid for given header. */
      StorageRootMismatch: AugmentedError<ApiType>;
      /** There are too many requests for the current window to handle. */
      TooManyRequests: AugmentedError<ApiType>;
      /** The header is unknown to the pallet. */
      UnknownHeader: AugmentedError<ApiType>;
      /**
       * The scheduled authority set change found in the header is unsupported by the pallet.
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
      /** The name of specification does not match between the current runtime and the new runtime. */
      InvalidSpecName: AugmentedError<ApiType>;
      /** Suicide called when the account has non-default composite data. */
      NonDefaultComposite: AugmentedError<ApiType>;
      /** There is a non-zero reference count preventing the account from being purged. */
      NonZeroRefCount: AugmentedError<ApiType>;
      /** The specification version is not allowed to decrease between the current runtime and the new runtime. */
      SpecVersionNeedsToIncrease: AugmentedError<ApiType>;
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
