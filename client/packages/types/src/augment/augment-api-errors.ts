// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/api-base/types/errors';

import type { ApiTypes, AugmentedError } from '@polkadot/api-base/types';

export type __AugmentedError<ApiType extends ApiTypes> = AugmentedError<ApiType>;

declare module '@polkadot/api-base/types/errors' {
  interface AugmentedErrors<ApiType extends ApiTypes> {
    accountManager: {
      ChargeAlreadyRegistered: AugmentedError<ApiType>;
      ChargeOrSettlementActualFeesOutgrowReserved: AugmentedError<ApiType>;
      ChargeOrSettlementCalculationOverflow: AugmentedError<ApiType>;
      DecodingExecutionIDFailed: AugmentedError<ApiType>;
      ExecutionAlreadyRegistered: AugmentedError<ApiType>;
      ExecutionNotRegistered: AugmentedError<ApiType>;
      NoChargeOfGivenIdRegistered: AugmentedError<ApiType>;
      PendingChargeNotFoundAtCommit: AugmentedError<ApiType>;
      PendingChargeNotFoundAtRefund: AugmentedError<ApiType>;
      SkippingEmptyCharges: AugmentedError<ApiType>;
      TransferDepositFailedOldChargeNotFound: AugmentedError<ApiType>;
      TransferDepositFailedToReleasePreviousCharge: AugmentedError<ApiType>;
    };
    assetRegistry: {
      /**
       * One of the passed capabilities is not valid for this asset
       **/
      CapabilitiesNotPermitted: AugmentedError<ApiType>;
      /**
       * This location mapping was unallowed for this user
       **/
      LocationUnallowed: AugmentedError<ApiType>;
      /**
       * The mapping or asset could not be found
       **/
      NotFound: AugmentedError<ApiType>;
      /**
       * The XCM message shouldnt be executed for given asset
       **/
      ShouldntExecuteMessage: AugmentedError<ApiType>;
    };
    assets: {
      /**
       * The asset-account already exists.
       **/
      AlreadyExists: AugmentedError<ApiType>;
      /**
       * The asset is not live, and likely being destroyed.
       **/
      AssetNotLive: AugmentedError<ApiType>;
      /**
       * Invalid metadata given.
       **/
      BadMetadata: AugmentedError<ApiType>;
      /**
       * Invalid witness data given.
       **/
      BadWitness: AugmentedError<ApiType>;
      /**
       * Account balance must be greater than or equal to the transfer amount.
       **/
      BalanceLow: AugmentedError<ApiType>;
      /**
       * Callback action resulted in error
       **/
      CallbackFailed: AugmentedError<ApiType>;
      /**
       * The origin account is frozen.
       **/
      Frozen: AugmentedError<ApiType>;
      /**
       * The asset status is not the expected status.
       **/
      IncorrectStatus: AugmentedError<ApiType>;
      /**
       * The asset ID is already taken.
       **/
      InUse: AugmentedError<ApiType>;
      /**
       * The asset is a live asset and is actively being used. Usually emit for operations such
       * as `start_destroy` which require the asset to be in a destroying state.
       **/
      LiveAsset: AugmentedError<ApiType>;
      /**
       * Minimum balance should be non-zero.
       **/
      MinBalanceZero: AugmentedError<ApiType>;
      /**
       * The account to alter does not exist.
       **/
      NoAccount: AugmentedError<ApiType>;
      /**
       * The asset-account doesn't have an associated deposit.
       **/
      NoDeposit: AugmentedError<ApiType>;
      /**
       * The signing account has no permission to do the operation.
       **/
      NoPermission: AugmentedError<ApiType>;
      /**
       * The asset should be frozen before the given operation.
       **/
      NotFrozen: AugmentedError<ApiType>;
      /**
       * No approval exists that would allow the transfer.
       **/
      Unapproved: AugmentedError<ApiType>;
      /**
       * Unable to increment the consumer reference counters on the account. Either no provider
       * reference exists to allow a non-zero balance of a non-self-sufficient asset, or one
       * fewer then the maximum number of consumers has been reached.
       **/
      UnavailableConsumer: AugmentedError<ApiType>;
      /**
       * The given asset ID is unknown.
       **/
      Unknown: AugmentedError<ApiType>;
      /**
       * The operation would result in funds being burned.
       **/
      WouldBurn: AugmentedError<ApiType>;
      /**
       * The source account would not survive the transfer and it needs to stay alive.
       **/
      WouldDie: AugmentedError<ApiType>;
    };
    attesters: {
      AddAttesterAlreadyRequested: AugmentedError<ApiType>;
      AlreadyNominated: AugmentedError<ApiType>;
      AlreadyRegistered: AugmentedError<ApiType>;
      ArithmeticOverflow: AugmentedError<ApiType>;
      AttestationDoubleSignAttempt: AugmentedError<ApiType>;
      AttestationSignatureInvalid: AugmentedError<ApiType>;
      AttesterBondTooSmall: AugmentedError<ApiType>;
      AttesterDidNotAgreeToNewTarget: AugmentedError<ApiType>;
      AttesterNotFound: AugmentedError<ApiType>;
      BanAttesterAlreadyRequested: AugmentedError<ApiType>;
      BatchAlreadyCommitted: AugmentedError<ApiType>;
      BatchFoundWithUnsignableStatus: AugmentedError<ApiType>;
      BatchHashMismatch: AugmentedError<ApiType>;
      BatchNotFound: AugmentedError<ApiType>;
      CollusionWithPermanentSlashDetected: AugmentedError<ApiType>;
      CommitteeSizeTooLarge: AugmentedError<ApiType>;
      InvalidMessage: AugmentedError<ApiType>;
      InvalidSignature: AugmentedError<ApiType>;
      InvalidTargetInclusionProof: AugmentedError<ApiType>;
      MissingNominations: AugmentedError<ApiType>;
      NextCommitteeAlreadyRequested: AugmentedError<ApiType>;
      NominatorBondTooSmall: AugmentedError<ApiType>;
      NominatorNotEnoughBalance: AugmentedError<ApiType>;
      NoNominationFound: AugmentedError<ApiType>;
      NotActiveSet: AugmentedError<ApiType>;
      NotInCurrentCommittee: AugmentedError<ApiType>;
      NotRegistered: AugmentedError<ApiType>;
      PublicKeyMissing: AugmentedError<ApiType>;
      RejectingFromSlashedAttester: AugmentedError<ApiType>;
      RemoveAttesterAlreadyRequested: AugmentedError<ApiType>;
      SfxAlreadyRequested: AugmentedError<ApiType>;
      TargetAlreadyActive: AugmentedError<ApiType>;
      TargetNotActive: AugmentedError<ApiType>;
      UnexpectedBatchHashRecoveredFromCommitment: AugmentedError<ApiType>;
      XdnsGatewayDoesNotHaveEscrowAddressRegistered: AugmentedError<ApiType>;
      XdnsTargetNotActive: AugmentedError<ApiType>;
    };
    balances: {
      /**
       * Beneficiary account must pre-exist.
       **/
      DeadAccount: AugmentedError<ApiType>;
      /**
       * Value too low to create account due to existential deposit.
       **/
      ExistentialDeposit: AugmentedError<ApiType>;
      /**
       * A vesting schedule already exists for this account.
       **/
      ExistingVestingSchedule: AugmentedError<ApiType>;
      /**
       * Transfer/payment would kill account.
       **/
      Expendability: AugmentedError<ApiType>;
      /**
       * Balance too low to send value.
       **/
      InsufficientBalance: AugmentedError<ApiType>;
      /**
       * Account liquidity restrictions prevent withdrawal.
       **/
      LiquidityRestrictions: AugmentedError<ApiType>;
      /**
       * Number of freezes exceed `MaxFreezes`.
       **/
      TooManyFreezes: AugmentedError<ApiType>;
      /**
       * Number of holds exceed `MaxHolds`.
       **/
      TooManyHolds: AugmentedError<ApiType>;
      /**
       * Number of named reserves exceed `MaxReserves`.
       **/
      TooManyReserves: AugmentedError<ApiType>;
      /**
       * Vesting balance too high to send value.
       **/
      VestingBalance: AugmentedError<ApiType>;
    };
    circuit: {
      ApplyFailed: AugmentedError<ApiType>;
      ApplyTriggeredWithUnexpectedStatus: AugmentedError<ApiType>;
      ArithmeticErrorDivisionByZero: AugmentedError<ApiType>;
      ArithmeticErrorOverflow: AugmentedError<ApiType>;
      ArithmeticErrorUnderflow: AugmentedError<ApiType>;
      AssetsFailedToWithdraw: AugmentedError<ApiType>;
      BidderNotEnoughBalance: AugmentedError<ApiType>;
      BiddingFailedExecutorsBalanceTooLowToReserve: AugmentedError<ApiType>;
      BiddingInactive: AugmentedError<ApiType>;
      BiddingRejectedBetterBidFound: AugmentedError<ApiType>;
      BiddingRejectedBidBelowDust: AugmentedError<ApiType>;
      BiddingRejectedBidTooHigh: AugmentedError<ApiType>;
      BiddingRejectedFailedToDepositBidderBond: AugmentedError<ApiType>;
      BiddingRejectedInsuranceTooLow: AugmentedError<ApiType>;
      ChargingTransferFailed: AugmentedError<ApiType>;
      ChargingTransferFailedAtPendingExecution: AugmentedError<ApiType>;
      ConfirmationFailed: AugmentedError<ApiType>;
      ContractXtxKilledRunOutOfFunds: AugmentedError<ApiType>;
      CriticalStateSquareUpCalledToFinishWithoutFsxConfirmed: AugmentedError<ApiType>;
      DeterminedForbiddenXtxStatus: AugmentedError<ApiType>;
      EnactSideEffectsCanOnlyBeCalledWithMin1StepFinished: AugmentedError<ApiType>;
      FailedToCheckInOverXBI: AugmentedError<ApiType>;
      FailedToCommitFSX: AugmentedError<ApiType>;
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
      FSXNotFoundById: AugmentedError<ApiType>;
      GatewayNotActive: AugmentedError<ApiType>;
      InsuranceBondAlreadyDeposited: AugmentedError<ApiType>;
      InsuranceBondNotRequired: AugmentedError<ApiType>;
      InvalidFSXBidStateLocated: AugmentedError<ApiType>;
      InvalidFTXStateEmptyBidForReadyXtx: AugmentedError<ApiType>;
      InvalidFTXStateEmptyConfirmationForFinishedXtx: AugmentedError<ApiType>;
      InvalidFTXStateIncorrectExecutorForReadySFX: AugmentedError<ApiType>;
      InvalidFTXStateUnassignedExecutorForReadySFX: AugmentedError<ApiType>;
      InvalidLocalTrigger: AugmentedError<ApiType>;
      InvalidOrderOrigin: AugmentedError<ApiType>;
      LocalExecutionUnauthorized: AugmentedError<ApiType>;
      LocalSideEffectExecutionNotApplicable: AugmentedError<ApiType>;
      OnLocalTriggerFailedToSetupXtx: AugmentedError<ApiType>;
      RefundTransferFailed: AugmentedError<ApiType>;
      RelayEscrowedFailedNothingToConfirm: AugmentedError<ApiType>;
      RequesterNotEnoughBalance: AugmentedError<ApiType>;
      RewardTransferFailed: AugmentedError<ApiType>;
      SanityAfterCreatingSFXDepositsFailed: AugmentedError<ApiType>;
      SetupFailed: AugmentedError<ApiType>;
      SetupFailedDuplicatedXtx: AugmentedError<ApiType>;
      SetupFailedEmptyXtx: AugmentedError<ApiType>;
      SetupFailedIncorrectXtxStatus: AugmentedError<ApiType>;
      SetupFailedXtxAlreadyFinished: AugmentedError<ApiType>;
      SetupFailedXtxNotFound: AugmentedError<ApiType>;
      SetupFailedXtxReverted: AugmentedError<ApiType>;
      SetupFailedXtxRevertedTimeout: AugmentedError<ApiType>;
      SetupFailedXtxStorageArtifactsNotFound: AugmentedError<ApiType>;
      SetupFailedXtxWasDroppedAtBidding: AugmentedError<ApiType>;
      SideEffectIsAlreadyScheduledToExecuteOverXBI: AugmentedError<ApiType>;
      SideEffectsValidationFailed: AugmentedError<ApiType>;
      SignalQueueFull: AugmentedError<ApiType>;
      UnauthorizedCancellation: AugmentedError<ApiType>;
      UnsupportedRole: AugmentedError<ApiType>;
      UpdateAttemptDoubleKill: AugmentedError<ApiType>;
      UpdateAttemptDoubleRevert: AugmentedError<ApiType>;
      UpdateForcedStateTransitionDisallowed: AugmentedError<ApiType>;
      UpdateStateTransitionDisallowed: AugmentedError<ApiType>;
      UpdateXtxTriggeredWithUnexpectedStatus: AugmentedError<ApiType>;
      XBIExitFailedOnSFXConfirmation: AugmentedError<ApiType>;
      XtxChargeBondDepositFailedCantAccessBid: AugmentedError<ApiType>;
      XtxChargeFailedRequesterBalanceTooLow: AugmentedError<ApiType>;
      XtxDoesNotExist: AugmentedError<ApiType>;
      XtxNotFound: AugmentedError<ApiType>;
    };
    clock: {
    };
    collatorSelection: {
      /**
       * Account is already a candidate.
       **/
      AlreadyCandidate: AugmentedError<ApiType>;
      /**
       * Account is already an Invulnerable.
       **/
      AlreadyInvulnerable: AugmentedError<ApiType>;
      /**
       * Account has no associated validator ID.
       **/
      NoAssociatedValidatorId: AugmentedError<ApiType>;
      /**
       * Account is not a candidate.
       **/
      NotCandidate: AugmentedError<ApiType>;
      /**
       * Account is not an Invulnerable.
       **/
      NotInvulnerable: AugmentedError<ApiType>;
      /**
       * Leaving would result in too few candidates.
       **/
      TooFewEligibleCollators: AugmentedError<ApiType>;
      /**
       * The pallet has too many candidates.
       **/
      TooManyCandidates: AugmentedError<ApiType>;
      /**
       * There are too many Invulnerables.
       **/
      TooManyInvulnerables: AugmentedError<ApiType>;
      /**
       * Validator ID is not yet registered.
       **/
      ValidatorNotRegistered: AugmentedError<ApiType>;
    };
    contracts: {
      /**
       * No code info could be found at the supplied code hash.
       **/
      CodeInfoNotFound: AugmentedError<ApiType>;
      /**
       * Code removal was denied because the code is still in use by at least one contract.
       **/
      CodeInUse: AugmentedError<ApiType>;
      /**
       * No code could be found at the supplied code hash.
       **/
      CodeNotFound: AugmentedError<ApiType>;
      /**
       * The contract's code was found to be invalid during validation.
       * 
       * The most likely cause of this is that an API was used which is not supported by the
       * node. This happens if an older node is used with a new version of ink!. Try updating
       * your node to the newest available version.
       * 
       * A more detailed error can be found on the node console if debug messages are enabled
       * by supplying `-lruntime::contracts=debug`.
       **/
      CodeRejected: AugmentedError<ApiType>;
      /**
       * The code supplied to `instantiate_with_code` exceeds the limit specified in the
       * current schedule.
       **/
      CodeTooLarge: AugmentedError<ApiType>;
      /**
       * No contract was found at the specified address.
       **/
      ContractNotFound: AugmentedError<ApiType>;
      /**
       * The contract ran to completion but decided to revert its storage changes.
       * Please note that this error is only returned from extrinsics. When called directly
       * or via RPC an `Ok` will be returned. In this case the caller needs to inspect the flags
       * to determine whether a reversion has taken place.
       **/
      ContractReverted: AugmentedError<ApiType>;
      /**
       * Contract trapped during execution.
       **/
      ContractTrapped: AugmentedError<ApiType>;
      /**
       * Input passed to a contract API function failed to decode as expected type.
       **/
      DecodingFailed: AugmentedError<ApiType>;
      /**
       * A contract with the same AccountId already exists.
       **/
      DuplicateContract: AugmentedError<ApiType>;
      /**
       * An indetermistic code was used in a context where this is not permitted.
       **/
      Indeterministic: AugmentedError<ApiType>;
      /**
       * `seal_call` forwarded this contracts input. It therefore is no longer available.
       **/
      InputForwarded: AugmentedError<ApiType>;
      /**
       * Invalid combination of flags supplied to `seal_call` or `seal_delegate_call`.
       **/
      InvalidCallFlags: AugmentedError<ApiType>;
      /**
       * Invalid schedule supplied, e.g. with zero weight of a basic operation.
       **/
      InvalidSchedule: AugmentedError<ApiType>;
      /**
       * Performing a call was denied because the calling depth reached the limit
       * of what is specified in the schedule.
       **/
      MaxCallDepthReached: AugmentedError<ApiType>;
      /**
       * A pending migration needs to complete before the extrinsic can be called.
       **/
      MigrationInProgress: AugmentedError<ApiType>;
      /**
       * The chain does not provide a chain extension. Calling the chain extension results
       * in this error. Note that this usually  shouldn't happen as deploying such contracts
       * is rejected.
       **/
      NoChainExtension: AugmentedError<ApiType>;
      /**
       * Migrate dispatch call was attempted but no migration was performed.
       **/
      NoMigrationPerformed: AugmentedError<ApiType>;
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
       * The only other cause is that a call from a contract into the runtime tried to call back
       * into `pallet-contracts`. This would make the whole pallet reentrant with regard to
       * contract code execution which is not supported.
       **/
      ReentranceDenied: AugmentedError<ApiType>;
      /**
       * More storage was created than allowed by the storage deposit limit.
       **/
      StorageDepositLimitExhausted: AugmentedError<ApiType>;
      /**
       * Origin doesn't have enough balance to pay the required storage deposits.
       **/
      StorageDepositNotEnoughFunds: AugmentedError<ApiType>;
      /**
       * A contract self destructed in its constructor.
       * 
       * This can be triggered by a call to `seal_terminate`.
       **/
      TerminatedInConstructor: AugmentedError<ApiType>;
      /**
       * Termination of a contract is not allowed while the contract is already
       * on the call stack. Can be triggered by `seal_terminate`.
       **/
      TerminatedWhileReentrant: AugmentedError<ApiType>;
      /**
       * The amount of topics passed to `seal_deposit_events` exceeds the limit.
       **/
      TooManyTopics: AugmentedError<ApiType>;
      /**
       * Performing the requested transfer failed. Probably because there isn't enough
       * free balance in the sender's account.
       **/
      TransferFailed: AugmentedError<ApiType>;
      /**
       * The size defined in `T::MaxValueSize` was exceeded.
       **/
      ValueTooLarge: AugmentedError<ApiType>;
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
    };
    cumulusXcm: {
    };
    dmpQueue: {
      /**
       * The amount of weight given is possibly not enough for executing the message.
       **/
      OverLimit: AugmentedError<ApiType>;
      /**
       * The message index given is unknown.
       **/
      Unknown: AugmentedError<ApiType>;
    };
    escrowTreasury: {
      /**
       * The spend origin is valid but the amount it is allowed to spend is lower than the
       * amount to be spent.
       **/
      InsufficientPermission: AugmentedError<ApiType>;
      /**
       * Proposer's balance is too low.
       **/
      InsufficientProposersBalance: AugmentedError<ApiType>;
      /**
       * No proposal or bounty at that index.
       **/
      InvalidIndex: AugmentedError<ApiType>;
      /**
       * Proposal has not been approved.
       **/
      ProposalNotApproved: AugmentedError<ApiType>;
      /**
       * Too many approvals in the queue.
       **/
      TooManyApprovals: AugmentedError<ApiType>;
    };
    ethereumBridge: {
      AlreadyInitialized: AugmentedError<ApiType>;
      BeaconCheckpointHashTreeRootFailed: AugmentedError<ApiType>;
      BeaconHeaderHashTreeRootFailed: AugmentedError<ApiType>;
      BeaconHeaderNotFinalized: AugmentedError<ApiType>;
      BeaconHeaderNotFound: AugmentedError<ApiType>;
      BLSPubkeyAggregationFaild: AugmentedError<ApiType>;
      CurrentSyncCommitteePeriodNotAvailable: AugmentedError<ApiType>;
      EventNotInReceipt: AugmentedError<ApiType>;
      ExecutionHeaderHashTreeRootFailed: AugmentedError<ApiType>;
      ExecutionHeaderNotFinalized: AugmentedError<ApiType>;
      ExecutionHeaderNotFound: AugmentedError<ApiType>;
      ForkNotDetected: AugmentedError<ApiType>;
      Halted: AugmentedError<ApiType>;
      InvalidBeaconLinkage: AugmentedError<ApiType>;
      InvalidBLSPublicKeyUsedForVerification: AugmentedError<ApiType>;
      InvalidBLSSignature: AugmentedError<ApiType>;
      InvalidCheckpoint: AugmentedError<ApiType>;
      InvalidEncodedEpochUpdate: AugmentedError<ApiType>;
      InvalidExecutionPayload: AugmentedError<ApiType>;
      InvalidExecutionRange: AugmentedError<ApiType>;
      InvalidExecutionRangeLinkage: AugmentedError<ApiType>;
      InvalidFork: AugmentedError<ApiType>;
      InvalidInclusionProof: AugmentedError<ApiType>;
      InvalidInitializationData: AugmentedError<ApiType>;
      InvalidMerkleProof: AugmentedError<ApiType>;
      InvalidSourceAddress: AugmentedError<ApiType>;
      InvalidSyncCommitteePeriod: AugmentedError<ApiType>;
      MathError: AugmentedError<ApiType>;
      NotPeriodsFirstEpoch: AugmentedError<ApiType>;
      SSZForkDataHashTreeRootFailed: AugmentedError<ApiType>;
      SSZSigningDataHashTreeRootFailed: AugmentedError<ApiType>;
      SubmittedHeaderToOld: AugmentedError<ApiType>;
      SyncCommitteeInvalid: AugmentedError<ApiType>;
      SyncCommitteeParticipantsNotSupermajority: AugmentedError<ApiType>;
      ValidSyncCommitteeNotAvailable: AugmentedError<ApiType>;
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
       * Gas limit is too high.
       **/
      GasLimitTooHigh: AugmentedError<ApiType>;
      /**
       * Gas limit is too low.
       **/
      GasLimitTooLow: AugmentedError<ApiType>;
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
       * EVM reentrancy
       **/
      Reentrancy: AugmentedError<ApiType>;
      /**
       * EIP-3607,
       **/
      TransactionMustComeFromEOA: AugmentedError<ApiType>;
      /**
       * Undefined error.
       **/
      Undefined: AugmentedError<ApiType>;
      /**
       * Withdraw fee failed
       **/
      WithdrawFailed: AugmentedError<ApiType>;
    };
    feeTreasury: {
      /**
       * The spend origin is valid but the amount it is allowed to spend is lower than the
       * amount to be spent.
       **/
      InsufficientPermission: AugmentedError<ApiType>;
      /**
       * Proposer's balance is too low.
       **/
      InsufficientProposersBalance: AugmentedError<ApiType>;
      /**
       * No proposal or bounty at that index.
       **/
      InvalidIndex: AugmentedError<ApiType>;
      /**
       * Proposal has not been approved.
       **/
      ProposalNotApproved: AugmentedError<ApiType>;
      /**
       * Too many approvals in the queue.
       **/
      TooManyApprovals: AugmentedError<ApiType>;
    };
    identity: {
      /**
       * Account ID is already named.
       **/
      AlreadyClaimed: AugmentedError<ApiType>;
      /**
       * Empty index.
       **/
      EmptyIndex: AugmentedError<ApiType>;
      /**
       * Fee is changed.
       **/
      FeeChanged: AugmentedError<ApiType>;
      /**
       * The index is invalid.
       **/
      InvalidIndex: AugmentedError<ApiType>;
      /**
       * Invalid judgement.
       **/
      InvalidJudgement: AugmentedError<ApiType>;
      /**
       * The target is invalid.
       **/
      InvalidTarget: AugmentedError<ApiType>;
      /**
       * The provided judgement was for a different identity.
       **/
      JudgementForDifferentIdentity: AugmentedError<ApiType>;
      /**
       * Judgement given.
       **/
      JudgementGiven: AugmentedError<ApiType>;
      /**
       * Error that occurs when there is an issue paying for judgement.
       **/
      JudgementPaymentFailed: AugmentedError<ApiType>;
      /**
       * No identity found.
       **/
      NoIdentity: AugmentedError<ApiType>;
      /**
       * Account isn't found.
       **/
      NotFound: AugmentedError<ApiType>;
      /**
       * Account isn't named.
       **/
      NotNamed: AugmentedError<ApiType>;
      /**
       * Sub-account isn't owned by sender.
       **/
      NotOwned: AugmentedError<ApiType>;
      /**
       * Sender is not a sub-account.
       **/
      NotSub: AugmentedError<ApiType>;
      /**
       * Sticky judgement.
       **/
      StickyJudgement: AugmentedError<ApiType>;
      /**
       * Too many additional fields.
       **/
      TooManyFields: AugmentedError<ApiType>;
      /**
       * Maximum amount of registrars reached. Cannot add any more.
       **/
      TooManyRegistrars: AugmentedError<ApiType>;
      /**
       * Too many subs-accounts.
       **/
      TooManySubAccounts: AugmentedError<ApiType>;
    };
    kusamaBridge: {
      /**
       * The block height couldn't be converted
       **/
      BlockHeightConversionError: AugmentedError<ApiType>;
      /**
       * The submitted range is empty
       **/
      EmptyRangeSubmitted: AugmentedError<ApiType>;
      /**
       * The events paramaters couldn't be decoded
       **/
      EventDecodingFailed: AugmentedError<ApiType>;
      /**
       * The event was not found in the specified block
       **/
      EventNotIncluded: AugmentedError<ApiType>;
      /**
       * The pallet is currently halted
       **/
      Halted: AugmentedError<ApiType>;
      /**
       * The given bytes couldn't be decoded as header data
       **/
      HeaderDataDecodingError: AugmentedError<ApiType>;
      /**
       * The given bytes couldn't be decoded as a header
       **/
      HeaderDecodingError: AugmentedError<ApiType>;
      /**
       * The inclusion data couldn't be decoded
       **/
      InclusionDataDecodeError: AugmentedError<ApiType>;
      /**
       * The authority set in invalid
       **/
      InvalidAuthoritySet: AugmentedError<ApiType>;
      /**
       * The submitted GrandpaJustification is not valid
       **/
      InvalidGrandpaJustification: AugmentedError<ApiType>;
      /**
       * The linkage with the justified header is not valid
       **/
      InvalidJustificationLinkage: AugmentedError<ApiType>;
      /**
       * The payload source is invalid
       **/
      InvalidPayloadSource: AugmentedError<ApiType>;
      /**
       * The header range linkage is not valid
       **/
      InvalidRangeLinkage: AugmentedError<ApiType>;
      /**
       * The payload source format is invalid
       **/
      InvalidSourceFormat: AugmentedError<ApiType>;
      /**
       * The submitted storage proof is invalid
       **/
      InvalidStorageProof: AugmentedError<ApiType>;
      /**
       * No finalized header was found in storage
       **/
      NoFinalizedHeader: AugmentedError<ApiType>;
      /**
       * The parachain entry was not found in storage
       **/
      ParachainEntryNotFound: AugmentedError<ApiType>;
      /**
       * The submitted range is larger the HeadersToStore, which is not permitted
       **/
      RangeToLarge: AugmentedError<ApiType>;
      /**
       * The headers storage root doesn't map the supplied once
       **/
      StorageRootMismatch: AugmentedError<ApiType>;
      /**
       * The relaychains storge root was not found. This implies the header is not available
       **/
      StorageRootNotFound: AugmentedError<ApiType>;
      /**
       * Failed to decode the source of the event
       **/
      UnexpectedEventLength: AugmentedError<ApiType>;
      /**
       * The source of event is not valid
       **/
      UnexpectedSource: AugmentedError<ApiType>;
      /**
       * The header couldn't be found in storage
       **/
      UnknownHeader: AugmentedError<ApiType>;
      /**
       * The side effect is not known for this vendor
       **/
      UnkownSideEffect: AugmentedError<ApiType>;
      /**
       * A forced change was detected, which is not supported
       **/
      UnsupportedScheduledChange: AugmentedError<ApiType>;
    };
    maintenance: {
      /**
       * The chain cannot enter maintenance mode because it is already in maintenance mode
       **/
      AlreadyInMaintenanceMode: AugmentedError<ApiType>;
      /**
       * The chain cannot resume normal operation because it is not in maintenance mode
       **/
      NotInMaintenanceMode: AugmentedError<ApiType>;
    };
    parachainSystem: {
      /**
       * The inherent which supplies the host configuration did not run this block.
       **/
      HostConfigurationNotAvailable: AugmentedError<ApiType>;
      /**
       * No code upgrade has been authorized.
       **/
      NothingAuthorized: AugmentedError<ApiType>;
      /**
       * No validation function upgrade is currently scheduled.
       **/
      NotScheduled: AugmentedError<ApiType>;
      /**
       * Attempt to upgrade validation function while existing upgrade pending.
       **/
      OverlappingUpgrades: AugmentedError<ApiType>;
      /**
       * Polkadot currently prohibits this parachain from upgrading its validation function.
       **/
      ProhibitedByPolkadot: AugmentedError<ApiType>;
      /**
       * The supplied validation function has compiled into a blob larger than Polkadot is
       * willing to run.
       **/
      TooBig: AugmentedError<ApiType>;
      /**
       * The given code upgrade has not been authorized.
       **/
      Unauthorized: AugmentedError<ApiType>;
      /**
       * The inherent which supplies the validation data did not run this block.
       **/
      ValidationDataNotAvailable: AugmentedError<ApiType>;
    };
    parachainTreasury: {
      /**
       * The spend origin is valid but the amount it is allowed to spend is lower than the
       * amount to be spent.
       **/
      InsufficientPermission: AugmentedError<ApiType>;
      /**
       * Proposer's balance is too low.
       **/
      InsufficientProposersBalance: AugmentedError<ApiType>;
      /**
       * No proposal or bounty at that index.
       **/
      InvalidIndex: AugmentedError<ApiType>;
      /**
       * Proposal has not been approved.
       **/
      ProposalNotApproved: AugmentedError<ApiType>;
      /**
       * Too many approvals in the queue.
       **/
      TooManyApprovals: AugmentedError<ApiType>;
    };
    polkadotBridge: {
      /**
       * The block height couldn't be converted
       **/
      BlockHeightConversionError: AugmentedError<ApiType>;
      /**
       * The submitted range is empty
       **/
      EmptyRangeSubmitted: AugmentedError<ApiType>;
      /**
       * The events paramaters couldn't be decoded
       **/
      EventDecodingFailed: AugmentedError<ApiType>;
      /**
       * The event was not found in the specified block
       **/
      EventNotIncluded: AugmentedError<ApiType>;
      /**
       * The pallet is currently halted
       **/
      Halted: AugmentedError<ApiType>;
      /**
       * The given bytes couldn't be decoded as header data
       **/
      HeaderDataDecodingError: AugmentedError<ApiType>;
      /**
       * The given bytes couldn't be decoded as a header
       **/
      HeaderDecodingError: AugmentedError<ApiType>;
      /**
       * The inclusion data couldn't be decoded
       **/
      InclusionDataDecodeError: AugmentedError<ApiType>;
      /**
       * The authority set in invalid
       **/
      InvalidAuthoritySet: AugmentedError<ApiType>;
      /**
       * The submitted GrandpaJustification is not valid
       **/
      InvalidGrandpaJustification: AugmentedError<ApiType>;
      /**
       * The linkage with the justified header is not valid
       **/
      InvalidJustificationLinkage: AugmentedError<ApiType>;
      /**
       * The payload source is invalid
       **/
      InvalidPayloadSource: AugmentedError<ApiType>;
      /**
       * The header range linkage is not valid
       **/
      InvalidRangeLinkage: AugmentedError<ApiType>;
      /**
       * The payload source format is invalid
       **/
      InvalidSourceFormat: AugmentedError<ApiType>;
      /**
       * The submitted storage proof is invalid
       **/
      InvalidStorageProof: AugmentedError<ApiType>;
      /**
       * No finalized header was found in storage
       **/
      NoFinalizedHeader: AugmentedError<ApiType>;
      /**
       * The parachain entry was not found in storage
       **/
      ParachainEntryNotFound: AugmentedError<ApiType>;
      /**
       * The submitted range is larger the HeadersToStore, which is not permitted
       **/
      RangeToLarge: AugmentedError<ApiType>;
      /**
       * The headers storage root doesn't map the supplied once
       **/
      StorageRootMismatch: AugmentedError<ApiType>;
      /**
       * The relaychains storge root was not found. This implies the header is not available
       **/
      StorageRootNotFound: AugmentedError<ApiType>;
      /**
       * Failed to decode the source of the event
       **/
      UnexpectedEventLength: AugmentedError<ApiType>;
      /**
       * The source of event is not valid
       **/
      UnexpectedSource: AugmentedError<ApiType>;
      /**
       * The header couldn't be found in storage
       **/
      UnknownHeader: AugmentedError<ApiType>;
      /**
       * The side effect is not known for this vendor
       **/
      UnkownSideEffect: AugmentedError<ApiType>;
      /**
       * A forced change was detected, which is not supported
       **/
      UnsupportedScheduledChange: AugmentedError<ApiType>;
    };
    polkadotXcm: {
      /**
       * The given account is not an identifiable sovereign account for any location.
       **/
      AccountNotSovereign: AugmentedError<ApiType>;
      /**
       * The location is invalid since it already has a subscription from us.
       **/
      AlreadySubscribed: AugmentedError<ApiType>;
      /**
       * The given location could not be used (e.g. because it cannot be expressed in the
       * desired version of XCM).
       **/
      BadLocation: AugmentedError<ApiType>;
      /**
       * The version of the `Versioned` value used is not able to be interpreted.
       **/
      BadVersion: AugmentedError<ApiType>;
      /**
       * Could not re-anchor the assets to declare the fees for the destination chain.
       **/
      CannotReanchor: AugmentedError<ApiType>;
      /**
       * The destination `MultiLocation` provided cannot be inverted.
       **/
      DestinationNotInvertible: AugmentedError<ApiType>;
      /**
       * The assets to be sent are empty.
       **/
      Empty: AugmentedError<ApiType>;
      /**
       * The operation required fees to be paid which the initiator could not meet.
       **/
      FeesNotMet: AugmentedError<ApiType>;
      /**
       * The message execution fails the filter.
       **/
      Filtered: AugmentedError<ApiType>;
      /**
       * The unlock operation cannot succeed because there are still consumers of the lock.
       **/
      InUse: AugmentedError<ApiType>;
      /**
       * Invalid asset for the operation.
       **/
      InvalidAsset: AugmentedError<ApiType>;
      /**
       * Origin is invalid for sending.
       **/
      InvalidOrigin: AugmentedError<ApiType>;
      /**
       * A remote lock with the corresponding data could not be found.
       **/
      LockNotFound: AugmentedError<ApiType>;
      /**
       * The owner does not own (all) of the asset that they wish to do the operation on.
       **/
      LowBalance: AugmentedError<ApiType>;
      /**
       * The referenced subscription could not be found.
       **/
      NoSubscription: AugmentedError<ApiType>;
      /**
       * There was some other issue (i.e. not to do with routing) in sending the message. Perhaps
       * a lack of space for buffering the message.
       **/
      SendFailure: AugmentedError<ApiType>;
      /**
       * Too many assets have been attempted for transfer.
       **/
      TooManyAssets: AugmentedError<ApiType>;
      /**
       * The asset owner has too many locks on the asset.
       **/
      TooManyLocks: AugmentedError<ApiType>;
      /**
       * The desired destination was unreachable, generally because there is a no way of routing
       * to it.
       **/
      Unreachable: AugmentedError<ApiType>;
      /**
       * The message's weight could not be determined.
       **/
      UnweighableMessage: AugmentedError<ApiType>;
    };
    portal: {
      /**
       * The gateways vendor is not available, which is a result of a missing XDNS record.
       **/
      GatewayVendorNotFound: AugmentedError<ApiType>;
      /**
       * The light client could not be found
       **/
      LightClientNotFoundByVendor: AugmentedError<ApiType>;
      /**
       * No gateway height could be found
       **/
      NoGatewayHeightAvailable: AugmentedError<ApiType>;
      /**
       * Gateway registration failed
       **/
      RegistrationError: AugmentedError<ApiType>;
      /**
       * Finality Verifiers operational status can't be updated
       **/
      SetOperationalError: AugmentedError<ApiType>;
      /**
       * Finality Verifier owner can't be set.
       **/
      SetOwnerError: AugmentedError<ApiType>;
      /**
       * Recoding failed
       **/
      SFXRecodeError: AugmentedError<ApiType>;
      /**
       * SideEffect confirmation failed
       **/
      SideEffectConfirmationFailed: AugmentedError<ApiType>;
      /**
       * The header could not be added
       **/
      SubmitHeaderError: AugmentedError<ApiType>;
      /**
       * Specified Vendor is not implemented
       **/
      UnimplementedGatewayVendor: AugmentedError<ApiType>;
      /**
       * The creation of the XDNS record was not successful
       **/
      XdnsRecordCreationFailed: AugmentedError<ApiType>;
    };
    preimage: {
      /**
       * Preimage has already been noted on-chain.
       **/
      AlreadyNoted: AugmentedError<ApiType>;
      /**
       * The user is not authorized to perform this action.
       **/
      NotAuthorized: AugmentedError<ApiType>;
      /**
       * The preimage cannot be removed since it has not yet been noted.
       **/
      NotNoted: AugmentedError<ApiType>;
      /**
       * The preimage request cannot be removed since no outstanding requests exist.
       **/
      NotRequested: AugmentedError<ApiType>;
      /**
       * A preimage may not be removed when there are outstanding requests.
       **/
      Requested: AugmentedError<ApiType>;
      /**
       * Preimage is too large to store on-chain.
       **/
      TooBig: AugmentedError<ApiType>;
    };
    rewards: {
      ArithmeticOverflow: AugmentedError<ApiType>;
      AttesterNotFound: AugmentedError<ApiType>;
      DistributionPeriodNotElapsed: AugmentedError<ApiType>;
      Halted: AugmentedError<ApiType>;
      NoPendingClaims: AugmentedError<ApiType>;
      TryIntoConversionU128ToBalanceFailed: AugmentedError<ApiType>;
    };
    rococoBridge: {
      /**
       * The block height couldn't be converted
       **/
      BlockHeightConversionError: AugmentedError<ApiType>;
      /**
       * The submitted range is empty
       **/
      EmptyRangeSubmitted: AugmentedError<ApiType>;
      /**
       * The events paramaters couldn't be decoded
       **/
      EventDecodingFailed: AugmentedError<ApiType>;
      /**
       * The event was not found in the specified block
       **/
      EventNotIncluded: AugmentedError<ApiType>;
      /**
       * The pallet is currently halted
       **/
      Halted: AugmentedError<ApiType>;
      /**
       * The given bytes couldn't be decoded as header data
       **/
      HeaderDataDecodingError: AugmentedError<ApiType>;
      /**
       * The given bytes couldn't be decoded as a header
       **/
      HeaderDecodingError: AugmentedError<ApiType>;
      /**
       * The inclusion data couldn't be decoded
       **/
      InclusionDataDecodeError: AugmentedError<ApiType>;
      /**
       * The authority set in invalid
       **/
      InvalidAuthoritySet: AugmentedError<ApiType>;
      /**
       * The submitted GrandpaJustification is not valid
       **/
      InvalidGrandpaJustification: AugmentedError<ApiType>;
      /**
       * The linkage with the justified header is not valid
       **/
      InvalidJustificationLinkage: AugmentedError<ApiType>;
      /**
       * The payload source is invalid
       **/
      InvalidPayloadSource: AugmentedError<ApiType>;
      /**
       * The header range linkage is not valid
       **/
      InvalidRangeLinkage: AugmentedError<ApiType>;
      /**
       * The payload source format is invalid
       **/
      InvalidSourceFormat: AugmentedError<ApiType>;
      /**
       * The submitted storage proof is invalid
       **/
      InvalidStorageProof: AugmentedError<ApiType>;
      /**
       * No finalized header was found in storage
       **/
      NoFinalizedHeader: AugmentedError<ApiType>;
      /**
       * The parachain entry was not found in storage
       **/
      ParachainEntryNotFound: AugmentedError<ApiType>;
      /**
       * The submitted range is larger the HeadersToStore, which is not permitted
       **/
      RangeToLarge: AugmentedError<ApiType>;
      /**
       * The headers storage root doesn't map the supplied once
       **/
      StorageRootMismatch: AugmentedError<ApiType>;
      /**
       * The relaychains storge root was not found. This implies the header is not available
       **/
      StorageRootNotFound: AugmentedError<ApiType>;
      /**
       * Failed to decode the source of the event
       **/
      UnexpectedEventLength: AugmentedError<ApiType>;
      /**
       * The source of event is not valid
       **/
      UnexpectedSource: AugmentedError<ApiType>;
      /**
       * The header couldn't be found in storage
       **/
      UnknownHeader: AugmentedError<ApiType>;
      /**
       * The side effect is not known for this vendor
       **/
      UnkownSideEffect: AugmentedError<ApiType>;
      /**
       * A forced change was detected, which is not supported
       **/
      UnsupportedScheduledChange: AugmentedError<ApiType>;
    };
    scheduler: {
      /**
       * Failed to schedule a call
       **/
      FailedToSchedule: AugmentedError<ApiType>;
      /**
       * Attempt to use a non-named function on a named task.
       **/
      Named: AugmentedError<ApiType>;
      /**
       * Cannot find the scheduled call.
       **/
      NotFound: AugmentedError<ApiType>;
      /**
       * Reschedule failed because it does not change scheduled time.
       **/
      RescheduleNoChange: AugmentedError<ApiType>;
      /**
       * Given target block number is in the past.
       **/
      TargetBlockNumberInPast: AugmentedError<ApiType>;
    };
    sepoliaBridge: {
      AlreadyInitialized: AugmentedError<ApiType>;
      BeaconCheckpointHashTreeRootFailed: AugmentedError<ApiType>;
      BeaconHeaderHashTreeRootFailed: AugmentedError<ApiType>;
      BeaconHeaderNotFinalized: AugmentedError<ApiType>;
      BeaconHeaderNotFound: AugmentedError<ApiType>;
      BLSPubkeyAggregationFaild: AugmentedError<ApiType>;
      CurrentSyncCommitteePeriodNotAvailable: AugmentedError<ApiType>;
      EventNotInReceipt: AugmentedError<ApiType>;
      ExecutionHeaderHashTreeRootFailed: AugmentedError<ApiType>;
      ExecutionHeaderNotFinalized: AugmentedError<ApiType>;
      ExecutionHeaderNotFound: AugmentedError<ApiType>;
      ForkNotDetected: AugmentedError<ApiType>;
      Halted: AugmentedError<ApiType>;
      InvalidBeaconLinkage: AugmentedError<ApiType>;
      InvalidBLSPublicKeyUsedForVerification: AugmentedError<ApiType>;
      InvalidBLSSignature: AugmentedError<ApiType>;
      InvalidCheckpoint: AugmentedError<ApiType>;
      InvalidEncodedEpochUpdate: AugmentedError<ApiType>;
      InvalidExecutionPayload: AugmentedError<ApiType>;
      InvalidExecutionRange: AugmentedError<ApiType>;
      InvalidExecutionRangeLinkage: AugmentedError<ApiType>;
      InvalidFork: AugmentedError<ApiType>;
      InvalidInclusionProof: AugmentedError<ApiType>;
      InvalidInitializationData: AugmentedError<ApiType>;
      InvalidMerkleProof: AugmentedError<ApiType>;
      InvalidSourceAddress: AugmentedError<ApiType>;
      InvalidSyncCommitteePeriod: AugmentedError<ApiType>;
      MathError: AugmentedError<ApiType>;
      NotPeriodsFirstEpoch: AugmentedError<ApiType>;
      SSZForkDataHashTreeRootFailed: AugmentedError<ApiType>;
      SSZSigningDataHashTreeRootFailed: AugmentedError<ApiType>;
      SubmittedHeaderToOld: AugmentedError<ApiType>;
      SyncCommitteeInvalid: AugmentedError<ApiType>;
      SyncCommitteeParticipantsNotSupermajority: AugmentedError<ApiType>;
      ValidSyncCommitteeNotAvailable: AugmentedError<ApiType>;
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
    };
    slashTreasury: {
      /**
       * The spend origin is valid but the amount it is allowed to spend is lower than the
       * amount to be spent.
       **/
      InsufficientPermission: AugmentedError<ApiType>;
      /**
       * Proposer's balance is too low.
       **/
      InsufficientProposersBalance: AugmentedError<ApiType>;
      /**
       * No proposal or bounty at that index.
       **/
      InvalidIndex: AugmentedError<ApiType>;
      /**
       * Proposal has not been approved.
       **/
      ProposalNotApproved: AugmentedError<ApiType>;
      /**
       * Too many approvals in the queue.
       **/
      TooManyApprovals: AugmentedError<ApiType>;
    };
    sudo: {
      /**
       * Sender must be the Sudo account
       **/
      RequireSudo: AugmentedError<ApiType>;
    };
    system: {
      /**
       * The origin filter prevent the call to be dispatched.
       **/
      CallFiltered: AugmentedError<ApiType>;
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
    };
    threeVm: {
      /**
       * The contract cannot be instantiated due to its type
       **/
      CannotInstantiateContract: AugmentedError<ApiType>;
      /**
       * You can't submit side effects without any side effects
       **/
      CannotTriggerWithoutSideEffects: AugmentedError<ApiType>;
      /**
       * The contract cannot generate side effects due to its type
       **/
      ContractCannotGenerateSideEffects: AugmentedError<ApiType>;
      /**
       * The contract cannot have storage due to its type
       **/
      ContractCannotHaveStorage: AugmentedError<ApiType>;
      /**
       * The contract cannot remunerate due to its type
       **/
      ContractCannotRemunerate: AugmentedError<ApiType>;
      /**
       * The contract could not be found in the registry
       **/
      ContractNotFound: AugmentedError<ApiType>;
      DownstreamCircuit: AugmentedError<ApiType>;
      /**
       * A user exceeded the bounce threshold for submitting signals
       **/
      ExceededSignalBounceThreshold: AugmentedError<ApiType>;
      /**
       * Invalid arithmetic computation causes overflow
       **/
      InvalidArithmeticOverflow: AugmentedError<ApiType>;
      /**
       * An origin could not be extracted from the buffer
       **/
      InvalidOrigin: AugmentedError<ApiType>;
      /**
       * Invalid precompile arguments
       **/
      InvalidPrecompileArgs: AugmentedError<ApiType>;
      /**
       * The precompile pointer was invalid
       **/
      InvalidPrecompilePointer: AugmentedError<ApiType>;
    };
    treasury: {
      /**
       * The spend origin is valid but the amount it is allowed to spend is lower than the
       * amount to be spent.
       **/
      InsufficientPermission: AugmentedError<ApiType>;
      /**
       * Proposer's balance is too low.
       **/
      InsufficientProposersBalance: AugmentedError<ApiType>;
      /**
       * No proposal or bounty at that index.
       **/
      InvalidIndex: AugmentedError<ApiType>;
      /**
       * Proposal has not been approved.
       **/
      ProposalNotApproved: AugmentedError<ApiType>;
      /**
       * Too many approvals in the queue.
       **/
      TooManyApprovals: AugmentedError<ApiType>;
    };
    utility: {
      /**
       * Too many calls batched.
       **/
      TooManyCalls: AugmentedError<ApiType>;
    };
    vacuum: {
    };
    xcmpQueue: {
      /**
       * Bad overweight index.
       **/
      BadOverweightIndex: AugmentedError<ApiType>;
      /**
       * Bad XCM data.
       **/
      BadXcm: AugmentedError<ApiType>;
      /**
       * Bad XCM origin.
       **/
      BadXcmOrigin: AugmentedError<ApiType>;
      /**
       * Failed to send XCM message.
       **/
      FailedToSend: AugmentedError<ApiType>;
      /**
       * Provided weight is possibly not enough to execute the message.
       **/
      WeightOverLimit: AugmentedError<ApiType>;
    };
    xdns: {
      /**
       * Escrow account not found
       **/
      EscrowAccountNotFound: AugmentedError<ApiType>;
      /**
       * Gateway verified as inactive
       **/
      GatewayNotActive: AugmentedError<ApiType>;
      /**
       * Stored gateway has already been added before
       **/
      GatewayRecordAlreadyExists: AugmentedError<ApiType>;
      /**
       * Gateway Record not found
       **/
      GatewayRecordNotFound: AugmentedError<ApiType>;
      /**
       * the xdns entry does not contain parachain information
       **/
      NoParachainInfoFound: AugmentedError<ApiType>;
      /**
       * SideEffectABI already exists
       **/
      SideEffectABIAlreadyExists: AugmentedError<ApiType>;
      /**
       * SideEffectABI not found
       **/
      SideEffectABINotFound: AugmentedError<ApiType>;
      /**
       * A token is not compatible with the gateways execution layer
       **/
      TokenExecutionVendorMismatch: AugmentedError<ApiType>;
      /**
       * Stored token has already been added before
       **/
      TokenRecordAlreadyExists: AugmentedError<ApiType>;
      /**
       * XDNS Token not found in assets overlay
       **/
      TokenRecordNotFoundInAssetsOverlay: AugmentedError<ApiType>;
      /**
       * XDNS Record not found
       **/
      XdnsRecordNotFound: AugmentedError<ApiType>;
    };
  } // AugmentedErrors
} // declare module
