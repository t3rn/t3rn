// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

import type { ApiTypes } from '@polkadot/api/types';

declare module '@polkadot/api/types/errors' {
  export interface AugmentedErrors<ApiType> {
    balances: {
      /**
       * Beneficiary account must pre-exist
       **/
      DeadAccount: AugmentedError<ApiType>;
      /**
       * Value too low to create account due to existential deposit
       **/
      ExistentialDeposit: AugmentedError<ApiType>;
      /**
       * A vesting schedule already exists for this account
       **/
      ExistingVestingSchedule: AugmentedError<ApiType>;
      /**
       * Balance too low to send value
       **/
      InsufficientBalance: AugmentedError<ApiType>;
      /**
       * Transfer/payment would kill account
       **/
      KeepAlive: AugmentedError<ApiType>;
      /**
       * Account liquidity restrictions prevent withdrawal
       **/
      LiquidityRestrictions: AugmentedError<ApiType>;
      /**
       * Number of named reserves exceed MaxReserves
       **/
      TooManyReserves: AugmentedError<ApiType>;
      /**
       * Vesting balance too high to send value
       **/
      VestingBalance: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    bridgeGatewayGrandpa: {
      /**
       * The pallet has already been initialized.
       **/
      AlreadyInitialized: AugmentedError<ApiType>;
      /**
       * All pallet operations are halted.
       **/
      Halted: AugmentedError<ApiType>;
      /**
       * The authority set from the underlying header chain is invalid.
       **/
      InvalidAuthoritySet: AugmentedError<ApiType>;
      /**
       * The given justification is invalid for the given header.
       **/
      InvalidJustification: AugmentedError<ApiType>;
      /**
       * The pallet is not yet initialized.
       **/
      NotInitialized: AugmentedError<ApiType>;
      /**
       * The header being imported is older than the best finalized header known to the pallet.
       **/
      OldHeader: AugmentedError<ApiType>;
      /**
       * The storage proof doesn't contains storage root. So it is invalid for given header.
       **/
      StorageRootMismatch: AugmentedError<ApiType>;
      /**
       * There are too many requests for the current window to handle.
       **/
      TooManyRequests: AugmentedError<ApiType>;
      /**
       * The header is unknown to the pallet.
       **/
      UnknownHeader: AugmentedError<ApiType>;
      /**
       * The scheduled authority set change found in the header is unsupported by the pallet.
       *
       * This is the case for non-standard (e.g forced) authority set changes.
       **/
      UnsupportedScheduledChange: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    bridgePolkadotLikeMultiFinalityVerifier: {
      /**
       * The pallet has already been initialized.
       **/
      AlreadyInitialized: AugmentedError<ApiType>;
      /**
       * All pallet operations are halted.
       **/
      Halted: AugmentedError<ApiType>;
      /**
       * The authority set from the underlying header chain is invalid.
       **/
      InvalidAuthoritySet: AugmentedError<ApiType>;
      /**
       * The given justification is invalid for the given header.
       **/
      InvalidJustification: AugmentedError<ApiType>;
      /**
       * The header being imported is older than the best finalized header known to the pallet.
       **/
      OldHeader: AugmentedError<ApiType>;
      /**
       * The storage proof doesn't contains storage root. So it is invalid for given header.
       **/
      StorageRootMismatch: AugmentedError<ApiType>;
      /**
       * There are too many requests for the current window to handle.
       **/
      TooManyRequests: AugmentedError<ApiType>;
      /**
       * The header is unknown to the pallet.
       **/
      UnknownHeader: AugmentedError<ApiType>;
      /**
       * The scheduled authority set change found in the header is unsupported by the pallet.
       *
       * This is the case for non-standard (e.g forced) authority set changes.
       **/
      UnsupportedScheduledChange: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    contracts: {
      /**
       * Performing the requested transfer would have brought the contract below
       * the subsistence threshold. No transfer is allowed to do this in order to allow
       * for a tombstone to be created. Use `seal_terminate` to remove a contract without
       * leaving a tombstone behind.
       **/
      BelowSubsistenceThreshold: AugmentedError<ApiType>;
      /**
       * No code could be found at the supplied code hash.
       **/
      CodeNotFound: AugmentedError<ApiType>;
      /**
       * The code supplied to `instantiate_with_code` exceeds the limit specified in the
       * current schedule.
       **/
      CodeTooLarge: AugmentedError<ApiType>;
      /**
       * A tombstone exist at the specified address.
       *
       * Tombstone cannot be called. Anyone can use `seal_restore_to` in order to revive
       * the contract, though.
       **/
      ContractIsTombstone: AugmentedError<ApiType>;
      /**
       * A contract could not be evicted because it has enough balance to pay rent.
       *
       * This can be returned from [`Pallet::claim_surcharge`] because the target
       * contract has enough balance to pay for its rent.
       **/
      ContractNotEvictable: AugmentedError<ApiType>;
      /**
       * No contract was found at the specified address.
       **/
      ContractNotFound: AugmentedError<ApiType>;
      /**
       * Contract trapped during execution.
       **/
      ContractTrapped: AugmentedError<ApiType>;
      /**
       * The debug message specified to `seal_debug_message` does contain invalid UTF-8.
       **/
      DebugMessageInvalidUTF8: AugmentedError<ApiType>;
      /**
       * Input passed to a contract API function failed to decode as expected type.
       **/
      DecodingFailed: AugmentedError<ApiType>;
      /**
       * Removal of a contract failed because the deletion queue is full.
       *
       * This can happen when either calling [`Pallet::claim_surcharge`] or `seal_terminate`.
       * The queue is filled by deleting contracts and emptied by a fixed amount each block.
       * Trying again during another block is the only way to resolve this issue.
       **/
      DeletionQueueFull: AugmentedError<ApiType>;
      /**
       * A contract with the same AccountId already exists.
       **/
      DuplicateContract: AugmentedError<ApiType>;
      /**
       * The topics passed to `seal_deposit_events` contains at least one duplicate.
       **/
      DuplicateTopics: AugmentedError<ApiType>;
      /**
       * `seal_call` forwarded this contracts input. It therefore is no longer available.
       **/
      InputForwarded: AugmentedError<ApiType>;
      /**
       * An origin TrieId written in the current block.
       **/
      InvalidContractOrigin: AugmentedError<ApiType>;
      /**
       * Cannot restore to nonexisting or alive contract.
       **/
      InvalidDestinationContract: AugmentedError<ApiType>;
      /**
       * A new schedule must have a greater version than the current one.
       **/
      InvalidScheduleVersion: AugmentedError<ApiType>;
      /**
       * Cannot restore from nonexisting or tombstone contract.
       **/
      InvalidSourceContract: AugmentedError<ApiType>;
      /**
       * An origin must be signed or inherent and auxiliary sender only provided on inherent.
       **/
      InvalidSurchargeClaim: AugmentedError<ApiType>;
      /**
       * Tombstones don't match.
       **/
      InvalidTombstone: AugmentedError<ApiType>;
      /**
       * Performing a call was denied because the calling depth reached the limit
       * of what is specified in the schedule.
       **/
      MaxCallDepthReached: AugmentedError<ApiType>;
      /**
       * The newly created contract is below the subsistence threshold after executing
       * its contructor. No contracts are allowed to exist below that threshold.
       **/
      NewContractNotFunded: AugmentedError<ApiType>;
      /**
       * The chain does not provide a chain extension. Calling the chain extension results
       * in this error. Note that this usually  shouldn't happen as deploying such contracts
       * is rejected.
       **/
      NoChainExtension: AugmentedError<ApiType>;
      /**
       * A buffer outside of sandbox memory was passed to a contract API function.
       **/
      OutOfBounds: AugmentedError<ApiType>;
      /**
       * The executed contract exhausted its gas limit.
       **/
      OutOfGas: AugmentedError<ApiType>;
      /**
       * The output buffer supplied to a contract API call was too small.
       **/
      OutputBufferTooSmall: AugmentedError<ApiType>;
      /**
       * The subject passed to `seal_random` exceeds the limit.
       **/
      RandomSubjectTooLong: AugmentedError<ApiType>;
      /**
       * A call tried to invoke a contract that is flagged as non-reentrant.
       **/
      ReentranceDenied: AugmentedError<ApiType>;
      /**
       * The called contract does not have enough balance to pay for its storage.
       *
       * The contract ran out of balance and is therefore eligible for eviction into a
       * tombstone. Anyone can evict the contract by submitting a `claim_surcharge`
       * extrinsic. Alternatively, a plain balance transfer can be used in order to
       * increase the contracts funds so that it can be called again.
       **/
      RentNotPaid: AugmentedError<ApiType>;
      /**
       * A storage modification exhausted the 32bit type that holds the storage size.
       *
       * This can either happen when the accumulated storage in bytes is too large or
       * when number of storage items is too large.
       **/
      StorageExhausted: AugmentedError<ApiType>;
      /**
       * A contract self destructed in its constructor.
       *
       * This can be triggered by a call to `seal_terminate` or `seal_restore_to`.
       **/
      TerminatedInConstructor: AugmentedError<ApiType>;
      /**
       * Termination of a contract is not allowed while the contract is already
       * on the call stack. Can be triggered by `seal_terminate` or `seal_restore_to.
       **/
      TerminatedWhileReentrant: AugmentedError<ApiType>;
      /**
       * The amount of topics passed to `seal_deposit_events` exceeds the limit.
       **/
      TooManyTopics: AugmentedError<ApiType>;
      /**
       * Performing the requested transfer failed for a reason originating in the
       * chosen currency implementation of the runtime. Most probably the balance is
       * too low or locks are placed on it.
       **/
      TransferFailed: AugmentedError<ApiType>;
      /**
       * The size defined in `T::MaxValueSize` was exceeded.
       **/
      ValueTooLarge: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    contractsRegistry: {
      /**
       * Stored contract has already been added before
       **/
      ContractAlreadyExists: AugmentedError<ApiType>;
      /**
       * Access of unknown contract
       **/
      UnknownContract: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    ethereumLightClient: {
      /**
       * Header is same height or older than finalized block (we don't support forks).
       **/
      AncientHeader: AugmentedError<ApiType>;
      /**
       * Log could not be decoded
       **/
      DecodeFailed: AugmentedError<ApiType>;
      /**
       * Header has already been imported.
       **/
      DuplicateHeader: AugmentedError<ApiType>;
      /**
       * Header referenced in inclusion proof is not final yet.
       **/
      HeaderNotFinalized: AugmentedError<ApiType>;
      /**
       * Header is on a stale fork, i.e. it's not a descendant of the latest finalized block
       **/
      HeaderOnStaleFork: AugmentedError<ApiType>;
      /**
       * One or more header fields are invalid.
       **/
      InvalidHeader: AugmentedError<ApiType>;
      /**
       * Proof could not be applied / verified.
       **/
      InvalidProof: AugmentedError<ApiType>;
      /**
       * Header referenced in inclusion proof doesn't exist, e.g. because it's
       * pruned or older than genesis.
       **/
      MissingHeader: AugmentedError<ApiType>;
      /**
       * Header's parent has not been imported.
       **/
      MissingParentHeader: AugmentedError<ApiType>;
      /**
       * This should never be returned - indicates a bug
       **/
      Unknown: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    evm: {
      /**
       * Not enough balance to perform action
       **/
      BalanceLow: AugmentedError<ApiType>;
      /**
       * Calculating total fee overflowed
       **/
      FeeOverflow: AugmentedError<ApiType>;
      /**
       * Gas price is too low.
       **/
      GasPriceTooLow: AugmentedError<ApiType>;
      /**
       * Nonce is invalid
       **/
      InvalidNonce: AugmentedError<ApiType>;
      /**
       * Calculating total payment overflowed
       **/
      PaymentOverflow: AugmentedError<ApiType>;
      /**
       * Withdraw fee failed
       **/
      WithdrawFailed: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    execDelivery: {
      /**
       * Non existent public key.
       **/
      InvalidKey: AugmentedError<ApiType>;
      IOScheduleEmpty: AugmentedError<ApiType>;
      IOScheduleNoEndingSemicolon: AugmentedError<ApiType>;
      IOScheduleUnknownCompose: AugmentedError<ApiType>;
      ProcessStepGatewayNotRecognised: AugmentedError<ApiType>;
      StepConfirmationBlockUnrecognised: AugmentedError<ApiType>;
      StepConfirmationDecodingError: AugmentedError<ApiType>;
      StepConfirmationGatewayNotRecognised: AugmentedError<ApiType>;
      StepConfirmationInvalidInclusionProof: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    grandpa: {
      /**
       * Attempt to signal GRANDPA change with one already pending.
       **/
      ChangePending: AugmentedError<ApiType>;
      /**
       * A given equivocation report is valid but already previously reported.
       **/
      DuplicateOffenceReport: AugmentedError<ApiType>;
      /**
       * An equivocation proof provided as part of an equivocation report is invalid.
       **/
      InvalidEquivocationProof: AugmentedError<ApiType>;
      /**
       * A key ownership proof provided as part of an equivocation report is invalid.
       **/
      InvalidKeyOwnershipProof: AugmentedError<ApiType>;
      /**
       * Attempt to signal GRANDPA pause when the authority set isn't live
       * (either paused or already pending pause).
       **/
      PauseFailed: AugmentedError<ApiType>;
      /**
       * Attempt to signal GRANDPA resume when the authority set isn't paused
       * (either live or already pending resume).
       **/
      ResumeFailed: AugmentedError<ApiType>;
      /**
       * Cannot signal forced change so soon after last.
       **/
      TooSoon: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    multiFinalityVerifier: {
      /**
       * The pallet has already been initialized.
       **/
      AlreadyInitialized: AugmentedError<ApiType>;
      /**
       * All pallet operations are halted.
       **/
      Halted: AugmentedError<ApiType>;
      /**
       * The authority set from the underlying header chain is invalid.
       **/
      InvalidAuthoritySet: AugmentedError<ApiType>;
      /**
       * The given justification is invalid for the given header.
       **/
      InvalidJustification: AugmentedError<ApiType>;
      /**
       * The header being imported is older than the best finalized header known to the pallet.
       **/
      OldHeader: AugmentedError<ApiType>;
      /**
       * The storage proof doesn't contains storage root. So it is invalid for given header.
       **/
      StorageRootMismatch: AugmentedError<ApiType>;
      /**
       * There are too many requests for the current window to handle.
       **/
      TooManyRequests: AugmentedError<ApiType>;
      /**
       * The header is unknown to the pallet.
       **/
      UnknownHeader: AugmentedError<ApiType>;
      /**
       * The scheduled authority set change found in the header is unsupported by the pallet.
       *
       * This is the case for non-standard (e.g forced) authority set changes.
       **/
      UnsupportedScheduledChange: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    session: {
      /**
       * Registered duplicate key.
       **/
      DuplicatedKey: AugmentedError<ApiType>;
      /**
       * Invalid ownership proof.
       **/
      InvalidProof: AugmentedError<ApiType>;
      /**
       * Key setting account is not live, so it's impossible to associate keys.
       **/
      NoAccount: AugmentedError<ApiType>;
      /**
       * No associated validator ID for account.
       **/
      NoAssociatedValidatorId: AugmentedError<ApiType>;
      /**
       * No keys are associated with this account.
       **/
      NoKeys: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    sudo: {
      /**
       * Sender must be the Sudo account
       **/
      RequireSudo: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    system: {
      /**
       * Failed to extract the runtime version from the new runtime.
       *
       * Either calling `Core_version` or decoding `RuntimeVersion` failed.
       **/
      FailedToExtractRuntimeVersion: AugmentedError<ApiType>;
      /**
       * The name of specification does not match between the current runtime
       * and the new runtime.
       **/
      InvalidSpecName: AugmentedError<ApiType>;
      /**
       * Suicide called when the account has non-default composite data.
       **/
      NonDefaultComposite: AugmentedError<ApiType>;
      /**
       * There is a non-zero reference count preventing the account from being purged.
       **/
      NonZeroRefCount: AugmentedError<ApiType>;
      /**
       * The specification version is not allowed to decrease between the current runtime
       * and the new runtime.
       **/
      SpecVersionNeedsToIncrease: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    volatileVm: {
      /**
       * Performing the requested transfer would have brought the contract below
       * the subsistence threshold. No transfer is allowed to do this in order to allow
       * for a tombstone to be created. Use `seal_terminate` to remove a contract without
       * leaving a tombstone behind.
       **/
      BelowSubsistenceThreshold: AugmentedError<ApiType>;
      /**
       * No code could be found at the supplied code hash.
       **/
      CodeNotFound: AugmentedError<ApiType>;
      /**
       * No code could be found at the supplied code hash.
       **/
      CodeNotFoundLazyGet: AugmentedError<ApiType>;
      /**
       * No code could be found at the supplied code hash.
       **/
      CodeNotFoundLazyUpdate: AugmentedError<ApiType>;
      /**
       * No code could be found at the supplied code hash.
       **/
      CodeNotFoundOther: AugmentedError<ApiType>;
      /**
       * The code supplied to `instantiate_with_code` exceeds the limit specified in the
       * current schedule.
       **/
      CodeTooLarge: AugmentedError<ApiType>;
      /**
       * A tombstone exist at the specified address.
       *
       * Tombstone cannot be called. Anyone can use `seal_restore_to` in order to revive
       * the contract, though.
       **/
      ContractIsTombstone: AugmentedError<ApiType>;
      /**
       * A contract could not be evicted because it has enough balance to pay rent.
       *
       * This can be returned from [`Pallet::claim_surcharge`] because the target
       * contract has enough balance to pay for its rent.
       **/
      ContractNotEvictable: AugmentedError<ApiType>;
      /**
       * No contract was found at the specified address.
       **/
      ContractNotFound: AugmentedError<ApiType>;
      /**
       * Contract trapped during execution.
       **/
      ContractTrapped: AugmentedError<ApiType>;
      /**
       * The debug message specified to `seal_debug_message` does contain invalid UTF-8.
       **/
      DebugMessageInvalidUTF8: AugmentedError<ApiType>;
      /**
       * Input passed to a contract API function failed to decode as expected type.
       **/
      DecodingFailed: AugmentedError<ApiType>;
      /**
       * Removal of a contract failed because the deletion queue is full.
       *
       * This can happen when either calling [`Pallet::claim_surcharge`] or `seal_terminate`.
       * The queue is filled by deleting contracts and emptied by a fixed amount each block.
       * Trying again during another block is the only way to resolve this issue.
       **/
      DeletionQueueFull: AugmentedError<ApiType>;
      /**
       * A contract with the same AccountId already exists.
       **/
      DuplicateContract: AugmentedError<ApiType>;
      /**
       * The topics passed to `seal_deposit_events` contains at least one duplicate.
       **/
      DuplicateTopics: AugmentedError<ApiType>;
      /**
       * `seal_call` forwarded this contracts input. It therefore is no longer available.
       **/
      InputForwarded: AugmentedError<ApiType>;
      /**
       * An origin TrieId written in the current block.
       **/
      InvalidContractOrigin: AugmentedError<ApiType>;
      /**
       * Cannot restore to nonexisting or alive contract.
       **/
      InvalidDestinationContract: AugmentedError<ApiType>;
      /**
       * A new schedule must have a greater version than the current one.
       **/
      InvalidScheduleVersion: AugmentedError<ApiType>;
      /**
       * Cannot restore from nonexisting or tombstone contract.
       **/
      InvalidSourceContract: AugmentedError<ApiType>;
      /**
       * An origin must be signed or inherent and auxiliary sender only provided on inherent.
       **/
      InvalidSurchargeClaim: AugmentedError<ApiType>;
      /**
       * Tombstones don't match.
       **/
      InvalidTombstone: AugmentedError<ApiType>;
      /**
       * Performing a call was denied because the calling depth reached the limit
       * of what is specified in the schedule.
       **/
      MaxCallDepthReached: AugmentedError<ApiType>;
      /**
       * The newly created contract is below the subsistence threshold after executing
       * its contructor. No contracts are allowed to exist below that threshold.
       **/
      NewContractNotFunded: AugmentedError<ApiType>;
      /**
       * The chain does not provide a chain extension. Calling the chain extension results
       * in this error. Note that this usually  shouldn't happen as deploying such contracts
       * is rejected.
       **/
      NoChainExtension: AugmentedError<ApiType>;
      /**
       * A buffer outside of sandbox memory was passed to a contract API function.
       **/
      OutOfBounds: AugmentedError<ApiType>;
      /**
       * The executed contract exhausted its gas limit.
       **/
      OutOfGas: AugmentedError<ApiType>;
      /**
       * The output buffer supplied to a contract API call was too small.
       **/
      OutputBufferTooSmall: AugmentedError<ApiType>;
      /**
       * The subject passed to `seal_random` exceeds the limit.
       **/
      RandomSubjectTooLong: AugmentedError<ApiType>;
      /**
       * A call tried to invoke a contract that is flagged as non-reentrant.
       **/
      ReentranceDenied: AugmentedError<ApiType>;
      /**
       * The called contract does not have enough balance to pay for its storage.
       *
       * The contract ran out of balance and is therefore eligible for eviction into a
       * tombstone. Anyone can evict the contract by submitting a `claim_surcharge`
       * extrinsic. Alternatively, a plain balance transfer can be used in order to
       * increase the contracts funds so that it can be called again.
       **/
      RentNotPaid: AugmentedError<ApiType>;
      /**
       * A storage modification exhausted the 32bit type that holds the storage size.
       *
       * This can either happen when the accumulated storage in bytes is too large or
       * when number of storage items is too large.
       **/
      StorageExhausted: AugmentedError<ApiType>;
      /**
       * Contract trapped during execution.
       **/
      TargetActionDescNotFound: AugmentedError<ApiType>;
      /**
       * Target changes to an external one that causes execution to break and messages grouped in round.
       **/
      TargetChangeAndRoundFinished: AugmentedError<ApiType>;
      /**
       * A contract self destructed in its constructor.
       *
       * This can be triggered by a call to `seal_terminate` or `seal_restore_to`.
       **/
      TerminatedInConstructor: AugmentedError<ApiType>;
      /**
       * Termination of a contract is not allowed while the contract is already
       * on the call stack. Can be triggered by `seal_terminate` or `seal_restore_to.
       **/
      TerminatedWhileReentrant: AugmentedError<ApiType>;
      /**
       * The amount of topics passed to `seal_deposit_events` exceeds the limit.
       **/
      TooManyTopics: AugmentedError<ApiType>;
      /**
       * Performing the requested transfer failed for a reason originating in the
       * chosen currency implementation of the runtime. Most probably the balance is
       * too low or locks are placed on it.
       **/
      TransferFailed: AugmentedError<ApiType>;
      /**
       * The size defined in `T::MaxValueSize` was exceeded.
       **/
      ValueTooLarge: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    xdns: {
      /**
       * Access of unknown xdns_record
       **/
      UnknownXdnsRecord: AugmentedError<ApiType>;
      /**
       * Stored xdns_record has already been added before
       **/
      XdnsRecordAlreadyExists: AugmentedError<ApiType>;
      /**
       * Xdns Record not found
       **/
      XdnsRecordNotFound: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
  }

  export interface DecoratedErrors<ApiType extends ApiTypes> extends AugmentedErrors<ApiType> {
    [key: string]: ModuleErrors<ApiType>;
  }
}
