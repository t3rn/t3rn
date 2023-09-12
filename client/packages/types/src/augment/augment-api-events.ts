// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/api-base/types/events';

import type { ApiTypes, AugmentedEvent } from '@polkadot/api-base/types';
import type { Bytes, Null, Option, Result, U256, U8aFixed, Vec, bool, u128, u32, u64, u8 } from '@polkadot/types-codec';
import type { ITuple } from '@polkadot/types-codec/types';
import type { AccountId32, H160, H256, Percent } from '@polkadot/types/interfaces/runtime';
import type { EthereumLog, FrameSupportDispatchDispatchInfo, FrameSupportTokensMiscBalanceStatus, PalletAttestersBatchMessage, PalletCircuitVacuumOrderStatusRead, PalletContractsOrigin, PalletEth2FinalityVerifierEpochSubmitted, PalletSepoliaFinalityVerifierEpochSubmitted, SpRuntimeDispatchError, SpWeightsWeightV2Weight, T3rnPrimitivesAttestersBatchingFactor, T3rnPrimitivesContractMetadataContractType, T3rnPrimitivesExecutionVendor, T3rnPrimitivesGatewayVendor, T3rnSdkPrimitivesSignalSignalKind, T3rnTypesFsxFullSideEffect, T3rnTypesSfxSideEffect, XcmV3MultiLocation, XcmV3MultiassetMultiAssets, XcmV3Response, XcmV3TraitsError, XcmV3TraitsOutcome, XcmV3Xcm, XcmVersionedMultiAssets, XcmVersionedMultiLocation, XpFormatXbiResult } from '@polkadot/types/lookup';

export type __AugmentedEvent<ApiType extends ApiTypes> = AugmentedEvent<ApiType>;

declare module '@polkadot/api-base/types/events' {
  interface AugmentedEvents<ApiType extends ApiTypes> {
    accountManager: {
      ContractsRegistryExecutionFinalized: AugmentedEvent<ApiType, [executionId: u64], { executionId: u64 }>;
      DepositReceived: AugmentedEvent<ApiType, [chargeId: H256, payee: AccountId32, recipient: Option<AccountId32>, amount: u128], { chargeId: H256, payee: AccountId32, recipient: Option<AccountId32>, amount: u128 }>;
      Issued: AugmentedEvent<ApiType, [recipient: AccountId32, amount: u128], { recipient: AccountId32, amount: u128 }>;
    };
    assetRegistry: {
      /**
       * An asset's information was created or updated [id, location]
       **/
      Info: AugmentedEvent<ApiType, [assetId: u32, location: XcmV3MultiLocation], { assetId: u32, location: XcmV3MultiLocation }>;
      /**
       * An asset location mapping was registered. [id, location]
       **/
      Registered: AugmentedEvent<ApiType, [assetId: u32, location: XcmV3MultiLocation], { assetId: u32, location: XcmV3MultiLocation }>;
    };
    assets: {
      /**
       * Accounts were destroyed for given asset.
       **/
      AccountsDestroyed: AugmentedEvent<ApiType, [assetId: u32, accountsDestroyed: u32, accountsRemaining: u32], { assetId: u32, accountsDestroyed: u32, accountsRemaining: u32 }>;
      /**
       * An approval for account `delegate` was cancelled by `owner`.
       **/
      ApprovalCancelled: AugmentedEvent<ApiType, [assetId: u32, owner: AccountId32, delegate: AccountId32], { assetId: u32, owner: AccountId32, delegate: AccountId32 }>;
      /**
       * Approvals were destroyed for given asset.
       **/
      ApprovalsDestroyed: AugmentedEvent<ApiType, [assetId: u32, approvalsDestroyed: u32, approvalsRemaining: u32], { assetId: u32, approvalsDestroyed: u32, approvalsRemaining: u32 }>;
      /**
       * (Additional) funds have been approved for transfer to a destination account.
       **/
      ApprovedTransfer: AugmentedEvent<ApiType, [assetId: u32, source: AccountId32, delegate: AccountId32, amount: u128], { assetId: u32, source: AccountId32, delegate: AccountId32, amount: u128 }>;
      /**
       * Some asset `asset_id` was frozen.
       **/
      AssetFrozen: AugmentedEvent<ApiType, [assetId: u32], { assetId: u32 }>;
      /**
       * The min_balance of an asset has been updated by the asset owner.
       **/
      AssetMinBalanceChanged: AugmentedEvent<ApiType, [assetId: u32, newMinBalance: u128], { assetId: u32, newMinBalance: u128 }>;
      /**
       * An asset has had its attributes changed by the `Force` origin.
       **/
      AssetStatusChanged: AugmentedEvent<ApiType, [assetId: u32], { assetId: u32 }>;
      /**
       * Some asset `asset_id` was thawed.
       **/
      AssetThawed: AugmentedEvent<ApiType, [assetId: u32], { assetId: u32 }>;
      /**
       * Some account `who` was blocked.
       **/
      Blocked: AugmentedEvent<ApiType, [assetId: u32, who: AccountId32], { assetId: u32, who: AccountId32 }>;
      /**
       * Some assets were destroyed.
       **/
      Burned: AugmentedEvent<ApiType, [assetId: u32, owner: AccountId32, balance: u128], { assetId: u32, owner: AccountId32, balance: u128 }>;
      /**
       * Some asset class was created.
       **/
      Created: AugmentedEvent<ApiType, [assetId: u32, creator: AccountId32, owner: AccountId32], { assetId: u32, creator: AccountId32, owner: AccountId32 }>;
      /**
       * An asset class was destroyed.
       **/
      Destroyed: AugmentedEvent<ApiType, [assetId: u32], { assetId: u32 }>;
      /**
       * An asset class is in the process of being destroyed.
       **/
      DestructionStarted: AugmentedEvent<ApiType, [assetId: u32], { assetId: u32 }>;
      /**
       * Some asset class was force-created.
       **/
      ForceCreated: AugmentedEvent<ApiType, [assetId: u32, owner: AccountId32], { assetId: u32, owner: AccountId32 }>;
      /**
       * Some account `who` was frozen.
       **/
      Frozen: AugmentedEvent<ApiType, [assetId: u32, who: AccountId32], { assetId: u32, who: AccountId32 }>;
      /**
       * Some assets were issued.
       **/
      Issued: AugmentedEvent<ApiType, [assetId: u32, owner: AccountId32, amount: u128], { assetId: u32, owner: AccountId32, amount: u128 }>;
      /**
       * Metadata has been cleared for an asset.
       **/
      MetadataCleared: AugmentedEvent<ApiType, [assetId: u32], { assetId: u32 }>;
      /**
       * New metadata has been set for an asset.
       **/
      MetadataSet: AugmentedEvent<ApiType, [assetId: u32, name: Bytes, symbol_: Bytes, decimals: u8, isFrozen: bool], { assetId: u32, name: Bytes, symbol: Bytes, decimals: u8, isFrozen: bool }>;
      /**
       * The owner changed.
       **/
      OwnerChanged: AugmentedEvent<ApiType, [assetId: u32, owner: AccountId32], { assetId: u32, owner: AccountId32 }>;
      /**
       * The management team changed.
       **/
      TeamChanged: AugmentedEvent<ApiType, [assetId: u32, issuer: AccountId32, admin: AccountId32, freezer: AccountId32], { assetId: u32, issuer: AccountId32, admin: AccountId32, freezer: AccountId32 }>;
      /**
       * Some account `who` was thawed.
       **/
      Thawed: AugmentedEvent<ApiType, [assetId: u32, who: AccountId32], { assetId: u32, who: AccountId32 }>;
      /**
       * Some account `who` was created with a deposit from `depositor`.
       **/
      Touched: AugmentedEvent<ApiType, [assetId: u32, who: AccountId32, depositor: AccountId32], { assetId: u32, who: AccountId32, depositor: AccountId32 }>;
      /**
       * Some assets were transferred.
       **/
      Transferred: AugmentedEvent<ApiType, [assetId: u32, from: AccountId32, to: AccountId32, amount: u128], { assetId: u32, from: AccountId32, to: AccountId32, amount: u128 }>;
      /**
       * An `amount` was transferred in its entirety from `owner` to `destination` by
       * the approved `delegate`.
       **/
      TransferredApproved: AugmentedEvent<ApiType, [assetId: u32, owner: AccountId32, delegate: AccountId32, destination: AccountId32, amount: u128], { assetId: u32, owner: AccountId32, delegate: AccountId32, destination: AccountId32, amount: u128 }>;
    };
    assetTxPayment: {
      /**
       * A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,
       * has been paid by `who` in an asset `asset_id`.
       **/
      AssetTxFeePaid: AugmentedEvent<ApiType, [who: AccountId32, actualFee: u128, tip: u128, assetId: Option<u32>], { who: AccountId32, actualFee: u128, tip: u128, assetId: Option<u32> }>;
    };
    attesters: {
      AttestationsRemovedFromLateBatches: AugmentedEvent<ApiType, [Vec<u32>]>;
      AttestationSubmitted: AugmentedEvent<ApiType, [AccountId32]>;
      AttestationTargetRemoved: AugmentedEvent<ApiType, [U8aFixed, Vec<U8aFixed>]>;
      AttesterAgreedToNewTarget: AugmentedEvent<ApiType, [AccountId32, U8aFixed, Bytes]>;
      AttesterDeregistered: AugmentedEvent<ApiType, [AccountId32]>;
      AttesterDeregistrationScheduled: AugmentedEvent<ApiType, [AccountId32, u32]>;
      AttesterRegistered: AugmentedEvent<ApiType, [AccountId32]>;
      BatchCommitted: AugmentedEvent<ApiType, [U8aFixed, PalletAttestersBatchMessage, Bytes, H256, u128]>;
      BatchingFactorRead: AugmentedEvent<ApiType, [Vec<ITuple<[U8aFixed, Option<T3rnPrimitivesAttestersBatchingFactor>]>>]>;
      CollusionWithPermanentSlashDetected: AugmentedEvent<ApiType, [U8aFixed, H256]>;
      ConfirmationRewardCalculated: AugmentedEvent<ApiType, [U8aFixed, u32, u128, Percent, Percent]>;
      CurrentPendingAttestationBatches: AugmentedEvent<ApiType, [U8aFixed, Vec<ITuple<[u32, H256]>>]>;
      NewAttestationBatch: AugmentedEvent<ApiType, [U8aFixed, PalletAttestersBatchMessage]>;
      NewAttestationMessageHash: AugmentedEvent<ApiType, [U8aFixed, H256, T3rnPrimitivesExecutionVendor]>;
      NewConfirmationBatch: AugmentedEvent<ApiType, [U8aFixed, PalletAttestersBatchMessage, Bytes, H256]>;
      NewTargetActivated: AugmentedEvent<ApiType, [U8aFixed]>;
      NewTargetProposed: AugmentedEvent<ApiType, [U8aFixed]>;
      Nominated: AugmentedEvent<ApiType, [AccountId32, AccountId32, u128]>;
      ShufflingCompleted: AugmentedEvent<ApiType, [Vec<AccountId32>, Vec<AccountId32>, Vec<AccountId32>]>;
      UserFinalityFeeEstimated: AugmentedEvent<ApiType, [U8aFixed, u128]>;
    };
    balances: {
      /**
       * A balance was set by root.
       **/
      BalanceSet: AugmentedEvent<ApiType, [who: AccountId32, free: u128], { who: AccountId32, free: u128 }>;
      /**
       * Some amount was burned from an account.
       **/
      Burned: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32, amount: u128 }>;
      /**
       * Some amount was deposited (e.g. for transaction fees).
       **/
      Deposit: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32, amount: u128 }>;
      /**
       * An account was removed whose balance was non-zero but below ExistentialDeposit,
       * resulting in an outright loss.
       **/
      DustLost: AugmentedEvent<ApiType, [account: AccountId32, amount: u128], { account: AccountId32, amount: u128 }>;
      /**
       * An account was created with some free balance.
       **/
      Endowed: AugmentedEvent<ApiType, [account: AccountId32, freeBalance: u128], { account: AccountId32, freeBalance: u128 }>;
      /**
       * Some balance was frozen.
       **/
      Frozen: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32, amount: u128 }>;
      /**
       * Total issuance was increased by `amount`, creating a credit to be balanced.
       **/
      Issued: AugmentedEvent<ApiType, [amount: u128], { amount: u128 }>;
      /**
       * Some balance was locked.
       **/
      Locked: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32, amount: u128 }>;
      /**
       * Some amount was minted into an account.
       **/
      Minted: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32, amount: u128 }>;
      /**
       * Total issuance was decreased by `amount`, creating a debt to be balanced.
       **/
      Rescinded: AugmentedEvent<ApiType, [amount: u128], { amount: u128 }>;
      /**
       * Some balance was reserved (moved from free to reserved).
       **/
      Reserved: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32, amount: u128 }>;
      /**
       * Some balance was moved from the reserve of the first account to the second account.
       * Final argument indicates the destination balance type.
       **/
      ReserveRepatriated: AugmentedEvent<ApiType, [from: AccountId32, to: AccountId32, amount: u128, destinationStatus: FrameSupportTokensMiscBalanceStatus], { from: AccountId32, to: AccountId32, amount: u128, destinationStatus: FrameSupportTokensMiscBalanceStatus }>;
      /**
       * Some amount was restored into an account.
       **/
      Restored: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32, amount: u128 }>;
      /**
       * Some amount was removed from the account (e.g. for misbehavior).
       **/
      Slashed: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32, amount: u128 }>;
      /**
       * Some amount was suspended from an account (it can be restored later).
       **/
      Suspended: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32, amount: u128 }>;
      /**
       * Some balance was thawed.
       **/
      Thawed: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32, amount: u128 }>;
      /**
       * Transfer succeeded.
       **/
      Transfer: AugmentedEvent<ApiType, [from: AccountId32, to: AccountId32, amount: u128], { from: AccountId32, to: AccountId32, amount: u128 }>;
      /**
       * Some balance was unlocked.
       **/
      Unlocked: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32, amount: u128 }>;
      /**
       * Some balance was unreserved (moved from reserved to free).
       **/
      Unreserved: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32, amount: u128 }>;
      /**
       * An account was upgraded.
       **/
      Upgraded: AugmentedEvent<ApiType, [who: AccountId32], { who: AccountId32 }>;
      /**
       * Some amount was withdrawn from the account (e.g. for transaction fees).
       **/
      Withdraw: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32, amount: u128 }>;
    };
    circuit: {
      AddLiquidity: AugmentedEvent<ApiType, [AccountId32, u32, u32, u128, u128, u128]>;
      CallCustom: AugmentedEvent<ApiType, [AccountId32, AccountId32, AccountId32, u128, Bytes, u64, Bytes]>;
      CallEvm: AugmentedEvent<ApiType, [AccountId32, H160, H160, U256, Bytes, u64, U256, Option<U256>, Option<U256>, Vec<ITuple<[H160, Vec<H256>]>>]>;
      CallNative: AugmentedEvent<ApiType, [AccountId32, Bytes]>;
      CallWasm: AugmentedEvent<ApiType, [AccountId32, AccountId32, u128, u64, Option<u128>, Bytes]>;
      CancelledSideEffects: AugmentedEvent<ApiType, [AccountId32, H256, Vec<T3rnTypesSfxSideEffect>]>;
      EscrowTransfer: AugmentedEvent<ApiType, [AccountId32, AccountId32, u128]>;
      NewSideEffectsAvailable: AugmentedEvent<ApiType, [AccountId32, H256, Vec<T3rnTypesSfxSideEffect>, Vec<H256>]>;
      Result: AugmentedEvent<ApiType, [AccountId32, AccountId32, XpFormatXbiResult, Bytes, Bytes]>;
      SFXNewBidReceived: AugmentedEvent<ApiType, [H256, AccountId32, u128]>;
      SideEffectConfirmed: AugmentedEvent<ApiType, [H256]>;
      SideEffectsConfirmed: AugmentedEvent<ApiType, [H256, Vec<Vec<T3rnTypesFsxFullSideEffect>>]>;
      SuccessfulFSXCommitAttestationRequest: AugmentedEvent<ApiType, [H256]>;
      SuccessfulFSXRevertAttestationRequest: AugmentedEvent<ApiType, [H256]>;
      Swap: AugmentedEvent<ApiType, [AccountId32, u32, u32, u128, u128, u128]>;
      Transfer: AugmentedEvent<ApiType, [AccountId32, AccountId32, AccountId32, u128]>;
      TransferAssets: AugmentedEvent<ApiType, [AccountId32, u32, AccountId32, AccountId32, u128]>;
      TransferORML: AugmentedEvent<ApiType, [AccountId32, u32, AccountId32, AccountId32, u128]>;
      UnsuccessfulFSXCommitAttestationRequest: AugmentedEvent<ApiType, [H256]>;
      UnsuccessfulFSXRevertAttestationRequest: AugmentedEvent<ApiType, [H256]>;
      XTransactionFSXCommitted: AugmentedEvent<ApiType, [H256]>;
      XTransactionReadyForExec: AugmentedEvent<ApiType, [H256]>;
      XTransactionReceivedForExec: AugmentedEvent<ApiType, [H256]>;
      XTransactionStepFinishedExec: AugmentedEvent<ApiType, [H256]>;
      XTransactionXtxCommitted: AugmentedEvent<ApiType, [H256]>;
      XTransactionXtxDroppedAtBidding: AugmentedEvent<ApiType, [H256]>;
      XTransactionXtxFinishedExecAllSteps: AugmentedEvent<ApiType, [H256]>;
      XTransactionXtxRevertedAfterTimeOut: AugmentedEvent<ApiType, [H256]>;
    };
    clock: {
      NewRound: AugmentedEvent<ApiType, [index: u32, head: u32, term: u32], { index: u32, head: u32, term: u32 }>;
    };
    collatorSelection: {
      /**
       * A new candidate joined.
       **/
      CandidateAdded: AugmentedEvent<ApiType, [accountId: AccountId32, deposit: u128], { accountId: AccountId32, deposit: u128 }>;
      /**
       * A candidate was removed.
       **/
      CandidateRemoved: AugmentedEvent<ApiType, [accountId: AccountId32], { accountId: AccountId32 }>;
      /**
       * An account was unable to be added to the Invulnerables because they did not have keys
       * registered. Other Invulnerables may have been set.
       **/
      InvalidInvulnerableSkipped: AugmentedEvent<ApiType, [accountId: AccountId32], { accountId: AccountId32 }>;
      /**
       * A new Invulnerable was added.
       **/
      InvulnerableAdded: AugmentedEvent<ApiType, [accountId: AccountId32], { accountId: AccountId32 }>;
      /**
       * An Invulnerable was removed.
       **/
      InvulnerableRemoved: AugmentedEvent<ApiType, [accountId: AccountId32], { accountId: AccountId32 }>;
      /**
       * The candidacy bond was set.
       **/
      NewCandidacyBond: AugmentedEvent<ApiType, [bondAmount: u128], { bondAmount: u128 }>;
      /**
       * The number of desired candidates was set.
       **/
      NewDesiredCandidates: AugmentedEvent<ApiType, [desiredCandidates: u32], { desiredCandidates: u32 }>;
      /**
       * New Invulnerables were set.
       **/
      NewInvulnerables: AugmentedEvent<ApiType, [invulnerables: Vec<AccountId32>], { invulnerables: Vec<AccountId32> }>;
    };
    contracts: {
      /**
       * A contract was called either by a plain account or another contract.
       * 
       * # Note
       * 
       * Please keep in mind that like all events this is only emitted for successful
       * calls. This is because on failure all storage changes including events are
       * rolled back.
       **/
      Called: AugmentedEvent<ApiType, [caller: PalletContractsOrigin, contract: AccountId32], { caller: PalletContractsOrigin, contract: AccountId32 }>;
      /**
       * A code with the specified hash was removed.
       **/
      CodeRemoved: AugmentedEvent<ApiType, [codeHash: H256], { codeHash: H256 }>;
      /**
       * Code with the specified hash has been stored.
       **/
      CodeStored: AugmentedEvent<ApiType, [codeHash: H256], { codeHash: H256 }>;
      /**
       * A contract's code was updated.
       **/
      ContractCodeUpdated: AugmentedEvent<ApiType, [contract: AccountId32, newCodeHash: H256, oldCodeHash: H256], { contract: AccountId32, newCodeHash: H256, oldCodeHash: H256 }>;
      /**
       * A custom event emitted by the contract.
       **/
      ContractEmitted: AugmentedEvent<ApiType, [contract: AccountId32, data: Bytes], { contract: AccountId32, data: Bytes }>;
      /**
       * A contract delegate called a code hash.
       * 
       * # Note
       * 
       * Please keep in mind that like all events this is only emitted for successful
       * calls. This is because on failure all storage changes including events are
       * rolled back.
       **/
      DelegateCalled: AugmentedEvent<ApiType, [contract: AccountId32, codeHash: H256], { contract: AccountId32, codeHash: H256 }>;
      /**
       * Contract deployed by address at the specified address.
       **/
      Instantiated: AugmentedEvent<ApiType, [deployer: AccountId32, contract: AccountId32], { deployer: AccountId32, contract: AccountId32 }>;
      /**
       * Contract has been removed.
       * 
       * # Note
       * 
       * The only way for a contract to be removed and emitting this event is by calling
       * `seal_terminate`.
       **/
      Terminated: AugmentedEvent<ApiType, [contract: AccountId32, beneficiary: AccountId32], { contract: AccountId32, beneficiary: AccountId32 }>;
    };
    contractsRegistry: {
      /**
       * \[requester, contract_id\]
       **/
      ContractPurged: AugmentedEvent<ApiType, [AccountId32, H256]>;
      /**
       * \[requester, contract_id\]
       **/
      ContractStored: AugmentedEvent<ApiType, [AccountId32, H256]>;
    };
    cumulusXcm: {
      /**
       * Downward message executed with the given outcome.
       * \[ id, outcome \]
       **/
      ExecutedDownward: AugmentedEvent<ApiType, [U8aFixed, XcmV3TraitsOutcome]>;
      /**
       * Downward message is invalid XCM.
       * \[ id \]
       **/
      InvalidFormat: AugmentedEvent<ApiType, [U8aFixed]>;
      /**
       * Downward message is unsupported version of XCM.
       * \[ id \]
       **/
      UnsupportedVersion: AugmentedEvent<ApiType, [U8aFixed]>;
    };
    dmpQueue: {
      /**
       * Downward message executed with the given outcome.
       **/
      ExecutedDownward: AugmentedEvent<ApiType, [messageHash: U8aFixed, messageId: U8aFixed, outcome: XcmV3TraitsOutcome], { messageHash: U8aFixed, messageId: U8aFixed, outcome: XcmV3TraitsOutcome }>;
      /**
       * Downward message is invalid XCM.
       **/
      InvalidFormat: AugmentedEvent<ApiType, [messageHash: U8aFixed], { messageHash: U8aFixed }>;
      /**
       * The maximum number of downward messages was reached.
       **/
      MaxMessagesExhausted: AugmentedEvent<ApiType, [messageHash: U8aFixed], { messageHash: U8aFixed }>;
      /**
       * Downward message is overweight and was placed in the overweight queue.
       **/
      OverweightEnqueued: AugmentedEvent<ApiType, [messageHash: U8aFixed, messageId: U8aFixed, overweightIndex: u64, requiredWeight: SpWeightsWeightV2Weight], { messageHash: U8aFixed, messageId: U8aFixed, overweightIndex: u64, requiredWeight: SpWeightsWeightV2Weight }>;
      /**
       * Downward message from the overweight queue was executed.
       **/
      OverweightServiced: AugmentedEvent<ApiType, [overweightIndex: u64, weightUsed: SpWeightsWeightV2Weight], { overweightIndex: u64, weightUsed: SpWeightsWeightV2Weight }>;
      /**
       * Downward message is unsupported version of XCM.
       **/
      UnsupportedVersion: AugmentedEvent<ApiType, [messageHash: U8aFixed], { messageHash: U8aFixed }>;
      /**
       * The weight limit for handling downward messages was reached.
       **/
      WeightExhausted: AugmentedEvent<ApiType, [messageHash: U8aFixed, messageId: U8aFixed, remainingWeight: SpWeightsWeightV2Weight, requiredWeight: SpWeightsWeightV2Weight], { messageHash: U8aFixed, messageId: U8aFixed, remainingWeight: SpWeightsWeightV2Weight, requiredWeight: SpWeightsWeightV2Weight }>;
    };
    escrowTreasury: {
      /**
       * Some funds have been allocated.
       **/
      Awarded: AugmentedEvent<ApiType, [proposalIndex: u32, award: u128, account: AccountId32], { proposalIndex: u32, award: u128, account: AccountId32 }>;
      /**
       * Some of our funds have been burnt.
       **/
      Burnt: AugmentedEvent<ApiType, [burntFunds: u128], { burntFunds: u128 }>;
      /**
       * Some funds have been deposited.
       **/
      Deposit: AugmentedEvent<ApiType, [value: u128], { value: u128 }>;
      /**
       * New proposal.
       **/
      Proposed: AugmentedEvent<ApiType, [proposalIndex: u32], { proposalIndex: u32 }>;
      /**
       * A proposal was rejected; funds were slashed.
       **/
      Rejected: AugmentedEvent<ApiType, [proposalIndex: u32, slashed: u128], { proposalIndex: u32, slashed: u128 }>;
      /**
       * Spending has finished; this is the amount that rolls over until next spend.
       **/
      Rollover: AugmentedEvent<ApiType, [rolloverBalance: u128], { rolloverBalance: u128 }>;
      /**
       * A new spend proposal has been approved.
       **/
      SpendApproved: AugmentedEvent<ApiType, [proposalIndex: u32, amount: u128, beneficiary: AccountId32], { proposalIndex: u32, amount: u128, beneficiary: AccountId32 }>;
      /**
       * We have ended a spend period and will now allocate funds.
       **/
      Spending: AugmentedEvent<ApiType, [budgetRemaining: u128], { budgetRemaining: u128 }>;
      /**
       * The inactive funds of the pallet have been updated.
       **/
      UpdatedInactive: AugmentedEvent<ApiType, [reactivated: u128, deactivated: u128], { reactivated: u128, deactivated: u128 }>;
    };
    ethereumBridge: {
      EpochUpdate: AugmentedEvent<ApiType, [PalletEth2FinalityVerifierEpochSubmitted]>;
    };
    evm: {
      /**
       * A contract has been created at given address.
       **/
      Created: AugmentedEvent<ApiType, [address: H160], { address: H160 }>;
      /**
       * A contract was attempted to be created, but the execution failed.
       **/
      CreatedFailed: AugmentedEvent<ApiType, [address: H160], { address: H160 }>;
      /**
       * A contract has been executed successfully with states applied.
       **/
      Executed: AugmentedEvent<ApiType, [address: H160], { address: H160 }>;
      /**
       * A contract has been executed with errors. States are reverted with only gas fees applied.
       **/
      ExecutedFailed: AugmentedEvent<ApiType, [address: H160], { address: H160 }>;
      /**
       * Ethereum events from contracts.
       **/
      Log: AugmentedEvent<ApiType, [log: EthereumLog], { log: EthereumLog }>;
    };
    feeTreasury: {
      /**
       * Some funds have been allocated.
       **/
      Awarded: AugmentedEvent<ApiType, [proposalIndex: u32, award: u128, account: AccountId32], { proposalIndex: u32, award: u128, account: AccountId32 }>;
      /**
       * Some of our funds have been burnt.
       **/
      Burnt: AugmentedEvent<ApiType, [burntFunds: u128], { burntFunds: u128 }>;
      /**
       * Some funds have been deposited.
       **/
      Deposit: AugmentedEvent<ApiType, [value: u128], { value: u128 }>;
      /**
       * New proposal.
       **/
      Proposed: AugmentedEvent<ApiType, [proposalIndex: u32], { proposalIndex: u32 }>;
      /**
       * A proposal was rejected; funds were slashed.
       **/
      Rejected: AugmentedEvent<ApiType, [proposalIndex: u32, slashed: u128], { proposalIndex: u32, slashed: u128 }>;
      /**
       * Spending has finished; this is the amount that rolls over until next spend.
       **/
      Rollover: AugmentedEvent<ApiType, [rolloverBalance: u128], { rolloverBalance: u128 }>;
      /**
       * A new spend proposal has been approved.
       **/
      SpendApproved: AugmentedEvent<ApiType, [proposalIndex: u32, amount: u128, beneficiary: AccountId32], { proposalIndex: u32, amount: u128, beneficiary: AccountId32 }>;
      /**
       * We have ended a spend period and will now allocate funds.
       **/
      Spending: AugmentedEvent<ApiType, [budgetRemaining: u128], { budgetRemaining: u128 }>;
      /**
       * The inactive funds of the pallet have been updated.
       **/
      UpdatedInactive: AugmentedEvent<ApiType, [reactivated: u128, deactivated: u128], { reactivated: u128, deactivated: u128 }>;
    };
    identity: {
      /**
       * A name was cleared, and the given balance returned.
       **/
      IdentityCleared: AugmentedEvent<ApiType, [who: AccountId32, deposit: u128], { who: AccountId32, deposit: u128 }>;
      /**
       * A name was removed and the given balance slashed.
       **/
      IdentityKilled: AugmentedEvent<ApiType, [who: AccountId32, deposit: u128], { who: AccountId32, deposit: u128 }>;
      /**
       * A name was set or reset (which will remove all judgements).
       **/
      IdentitySet: AugmentedEvent<ApiType, [who: AccountId32], { who: AccountId32 }>;
      /**
       * A judgement was given by a registrar.
       **/
      JudgementGiven: AugmentedEvent<ApiType, [target: AccountId32, registrarIndex: u32], { target: AccountId32, registrarIndex: u32 }>;
      /**
       * A judgement was asked from a registrar.
       **/
      JudgementRequested: AugmentedEvent<ApiType, [who: AccountId32, registrarIndex: u32], { who: AccountId32, registrarIndex: u32 }>;
      /**
       * A judgement request was retracted.
       **/
      JudgementUnrequested: AugmentedEvent<ApiType, [who: AccountId32, registrarIndex: u32], { who: AccountId32, registrarIndex: u32 }>;
      /**
       * A registrar was added.
       **/
      RegistrarAdded: AugmentedEvent<ApiType, [registrarIndex: u32], { registrarIndex: u32 }>;
      /**
       * A sub-identity was added to an identity and the deposit paid.
       **/
      SubIdentityAdded: AugmentedEvent<ApiType, [sub: AccountId32, main: AccountId32, deposit: u128], { sub: AccountId32, main: AccountId32, deposit: u128 }>;
      /**
       * A sub-identity was removed from an identity and the deposit freed.
       **/
      SubIdentityRemoved: AugmentedEvent<ApiType, [sub: AccountId32, main: AccountId32, deposit: u128], { sub: AccountId32, main: AccountId32, deposit: u128 }>;
      /**
       * A sub-identity was cleared, and the given deposit repatriated from the
       * main identity account to the sub-identity account.
       **/
      SubIdentityRevoked: AugmentedEvent<ApiType, [sub: AccountId32, main: AccountId32, deposit: u128], { sub: AccountId32, main: AccountId32, deposit: u128 }>;
    };
    kusamaBridge: {
      HeadersAdded: AugmentedEvent<ApiType, [u32]>;
    };
    maintenance: {
      /**
       * The chain was put into Maintenance Mode
       **/
      EnteredMaintenanceMode: AugmentedEvent<ApiType, []>;
      /**
       * The call to resume on_idle XCM execution failed with inner error
       **/
      FailedToResumeIdleXcmExecution: AugmentedEvent<ApiType, [error: SpRuntimeDispatchError], { error: SpRuntimeDispatchError }>;
      /**
       * The call to suspend on_idle XCM execution failed with inner error
       **/
      FailedToSuspendIdleXcmExecution: AugmentedEvent<ApiType, [error: SpRuntimeDispatchError], { error: SpRuntimeDispatchError }>;
      /**
       * The chain returned to its normal operating state
       **/
      NormalOperationResumed: AugmentedEvent<ApiType, []>;
    };
    parachainSystem: {
      /**
       * Downward messages were processed using the given weight.
       **/
      DownwardMessagesProcessed: AugmentedEvent<ApiType, [weightUsed: SpWeightsWeightV2Weight, dmqHead: H256], { weightUsed: SpWeightsWeightV2Weight, dmqHead: H256 }>;
      /**
       * Some downward messages have been received and will be processed.
       **/
      DownwardMessagesReceived: AugmentedEvent<ApiType, [count: u32], { count: u32 }>;
      /**
       * An upgrade has been authorized.
       **/
      UpgradeAuthorized: AugmentedEvent<ApiType, [codeHash: H256], { codeHash: H256 }>;
      /**
       * An upward message was sent to the relay chain.
       **/
      UpwardMessageSent: AugmentedEvent<ApiType, [messageHash: Option<U8aFixed>], { messageHash: Option<U8aFixed> }>;
      /**
       * The validation function was applied as of the contained relay chain block number.
       **/
      ValidationFunctionApplied: AugmentedEvent<ApiType, [relayChainBlockNum: u32], { relayChainBlockNum: u32 }>;
      /**
       * The relay-chain aborted the upgrade process.
       **/
      ValidationFunctionDiscarded: AugmentedEvent<ApiType, []>;
      /**
       * The validation function has been scheduled to apply.
       **/
      ValidationFunctionStored: AugmentedEvent<ApiType, []>;
    };
    parachainTreasury: {
      /**
       * Some funds have been allocated.
       **/
      Awarded: AugmentedEvent<ApiType, [proposalIndex: u32, award: u128, account: AccountId32], { proposalIndex: u32, award: u128, account: AccountId32 }>;
      /**
       * Some of our funds have been burnt.
       **/
      Burnt: AugmentedEvent<ApiType, [burntFunds: u128], { burntFunds: u128 }>;
      /**
       * Some funds have been deposited.
       **/
      Deposit: AugmentedEvent<ApiType, [value: u128], { value: u128 }>;
      /**
       * New proposal.
       **/
      Proposed: AugmentedEvent<ApiType, [proposalIndex: u32], { proposalIndex: u32 }>;
      /**
       * A proposal was rejected; funds were slashed.
       **/
      Rejected: AugmentedEvent<ApiType, [proposalIndex: u32, slashed: u128], { proposalIndex: u32, slashed: u128 }>;
      /**
       * Spending has finished; this is the amount that rolls over until next spend.
       **/
      Rollover: AugmentedEvent<ApiType, [rolloverBalance: u128], { rolloverBalance: u128 }>;
      /**
       * A new spend proposal has been approved.
       **/
      SpendApproved: AugmentedEvent<ApiType, [proposalIndex: u32, amount: u128, beneficiary: AccountId32], { proposalIndex: u32, amount: u128, beneficiary: AccountId32 }>;
      /**
       * We have ended a spend period and will now allocate funds.
       **/
      Spending: AugmentedEvent<ApiType, [budgetRemaining: u128], { budgetRemaining: u128 }>;
      /**
       * The inactive funds of the pallet have been updated.
       **/
      UpdatedInactive: AugmentedEvent<ApiType, [reactivated: u128, deactivated: u128], { reactivated: u128, deactivated: u128 }>;
    };
    polkadotBridge: {
      HeadersAdded: AugmentedEvent<ApiType, [u32]>;
    };
    polkadotXcm: {
      /**
       * Some assets have been claimed from an asset trap
       **/
      AssetsClaimed: AugmentedEvent<ApiType, [hash_: H256, origin: XcmV3MultiLocation, assets: XcmVersionedMultiAssets], { hash_: H256, origin: XcmV3MultiLocation, assets: XcmVersionedMultiAssets }>;
      /**
       * Some assets have been placed in an asset trap.
       **/
      AssetsTrapped: AugmentedEvent<ApiType, [hash_: H256, origin: XcmV3MultiLocation, assets: XcmVersionedMultiAssets], { hash_: H256, origin: XcmV3MultiLocation, assets: XcmVersionedMultiAssets }>;
      /**
       * Execution of an XCM message was attempted.
       **/
      Attempted: AugmentedEvent<ApiType, [outcome: XcmV3TraitsOutcome], { outcome: XcmV3TraitsOutcome }>;
      /**
       * Fees were paid from a location for an operation (often for using `SendXcm`).
       **/
      FeesPaid: AugmentedEvent<ApiType, [paying: XcmV3MultiLocation, fees: XcmV3MultiassetMultiAssets], { paying: XcmV3MultiLocation, fees: XcmV3MultiassetMultiAssets }>;
      /**
       * Expected query response has been received but the querier location of the response does
       * not match the expected. The query remains registered for a later, valid, response to
       * be received and acted upon.
       **/
      InvalidQuerier: AugmentedEvent<ApiType, [origin: XcmV3MultiLocation, queryId: u64, expectedQuerier: XcmV3MultiLocation, maybeActualQuerier: Option<XcmV3MultiLocation>], { origin: XcmV3MultiLocation, queryId: u64, expectedQuerier: XcmV3MultiLocation, maybeActualQuerier: Option<XcmV3MultiLocation> }>;
      /**
       * Expected query response has been received but the expected querier location placed in
       * storage by this runtime previously cannot be decoded. The query remains registered.
       * 
       * This is unexpected (since a location placed in storage in a previously executing
       * runtime should be readable prior to query timeout) and dangerous since the possibly
       * valid response will be dropped. Manual governance intervention is probably going to be
       * needed.
       **/
      InvalidQuerierVersion: AugmentedEvent<ApiType, [origin: XcmV3MultiLocation, queryId: u64], { origin: XcmV3MultiLocation, queryId: u64 }>;
      /**
       * Expected query response has been received but the origin location of the response does
       * not match that expected. The query remains registered for a later, valid, response to
       * be received and acted upon.
       **/
      InvalidResponder: AugmentedEvent<ApiType, [origin: XcmV3MultiLocation, queryId: u64, expectedLocation: Option<XcmV3MultiLocation>], { origin: XcmV3MultiLocation, queryId: u64, expectedLocation: Option<XcmV3MultiLocation> }>;
      /**
       * Expected query response has been received but the expected origin location placed in
       * storage by this runtime previously cannot be decoded. The query remains registered.
       * 
       * This is unexpected (since a location placed in storage in a previously executing
       * runtime should be readable prior to query timeout) and dangerous since the possibly
       * valid response will be dropped. Manual governance intervention is probably going to be
       * needed.
       **/
      InvalidResponderVersion: AugmentedEvent<ApiType, [origin: XcmV3MultiLocation, queryId: u64], { origin: XcmV3MultiLocation, queryId: u64 }>;
      /**
       * Query response has been received and query is removed. The registered notification has
       * been dispatched and executed successfully.
       **/
      Notified: AugmentedEvent<ApiType, [queryId: u64, palletIndex: u8, callIndex: u8], { queryId: u64, palletIndex: u8, callIndex: u8 }>;
      /**
       * Query response has been received and query is removed. The dispatch was unable to be
       * decoded into a `Call`; this might be due to dispatch function having a signature which
       * is not `(origin, QueryId, Response)`.
       **/
      NotifyDecodeFailed: AugmentedEvent<ApiType, [queryId: u64, palletIndex: u8, callIndex: u8], { queryId: u64, palletIndex: u8, callIndex: u8 }>;
      /**
       * Query response has been received and query is removed. There was a general error with
       * dispatching the notification call.
       **/
      NotifyDispatchError: AugmentedEvent<ApiType, [queryId: u64, palletIndex: u8, callIndex: u8], { queryId: u64, palletIndex: u8, callIndex: u8 }>;
      /**
       * Query response has been received and query is removed. The registered notification could
       * not be dispatched because the dispatch weight is greater than the maximum weight
       * originally budgeted by this runtime for the query result.
       **/
      NotifyOverweight: AugmentedEvent<ApiType, [queryId: u64, palletIndex: u8, callIndex: u8, actualWeight: SpWeightsWeightV2Weight, maxBudgetedWeight: SpWeightsWeightV2Weight], { queryId: u64, palletIndex: u8, callIndex: u8, actualWeight: SpWeightsWeightV2Weight, maxBudgetedWeight: SpWeightsWeightV2Weight }>;
      /**
       * A given location which had a version change subscription was dropped owing to an error
       * migrating the location to our new XCM format.
       **/
      NotifyTargetMigrationFail: AugmentedEvent<ApiType, [location: XcmVersionedMultiLocation, queryId: u64], { location: XcmVersionedMultiLocation, queryId: u64 }>;
      /**
       * A given location which had a version change subscription was dropped owing to an error
       * sending the notification to it.
       **/
      NotifyTargetSendFail: AugmentedEvent<ApiType, [location: XcmV3MultiLocation, queryId: u64, error: XcmV3TraitsError], { location: XcmV3MultiLocation, queryId: u64, error: XcmV3TraitsError }>;
      /**
       * Query response has been received and is ready for taking with `take_response`. There is
       * no registered notification call.
       **/
      ResponseReady: AugmentedEvent<ApiType, [queryId: u64, response: XcmV3Response], { queryId: u64, response: XcmV3Response }>;
      /**
       * Received query response has been read and removed.
       **/
      ResponseTaken: AugmentedEvent<ApiType, [queryId: u64], { queryId: u64 }>;
      /**
       * A XCM message was sent.
       **/
      Sent: AugmentedEvent<ApiType, [origin: XcmV3MultiLocation, destination: XcmV3MultiLocation, message: XcmV3Xcm, messageId: U8aFixed], { origin: XcmV3MultiLocation, destination: XcmV3MultiLocation, message: XcmV3Xcm, messageId: U8aFixed }>;
      /**
       * The supported version of a location has been changed. This might be through an
       * automatic notification or a manual intervention.
       **/
      SupportedVersionChanged: AugmentedEvent<ApiType, [location: XcmV3MultiLocation, version: u32], { location: XcmV3MultiLocation, version: u32 }>;
      /**
       * Query response received which does not match a registered query. This may be because a
       * matching query was never registered, it may be because it is a duplicate response, or
       * because the query timed out.
       **/
      UnexpectedResponse: AugmentedEvent<ApiType, [origin: XcmV3MultiLocation, queryId: u64], { origin: XcmV3MultiLocation, queryId: u64 }>;
      /**
       * An XCM version change notification message has been attempted to be sent.
       * 
       * The cost of sending it (borne by the chain) is included.
       **/
      VersionChangeNotified: AugmentedEvent<ApiType, [destination: XcmV3MultiLocation, result: u32, cost: XcmV3MultiassetMultiAssets, messageId: U8aFixed], { destination: XcmV3MultiLocation, result: u32, cost: XcmV3MultiassetMultiAssets, messageId: U8aFixed }>;
      /**
       * We have requested that a remote chain send us XCM version change notifications.
       **/
      VersionNotifyRequested: AugmentedEvent<ApiType, [destination: XcmV3MultiLocation, cost: XcmV3MultiassetMultiAssets, messageId: U8aFixed], { destination: XcmV3MultiLocation, cost: XcmV3MultiassetMultiAssets, messageId: U8aFixed }>;
      /**
       * A remote has requested XCM version change notification from us and we have honored it.
       * A version information message is sent to them and its cost is included.
       **/
      VersionNotifyStarted: AugmentedEvent<ApiType, [destination: XcmV3MultiLocation, cost: XcmV3MultiassetMultiAssets, messageId: U8aFixed], { destination: XcmV3MultiLocation, cost: XcmV3MultiassetMultiAssets, messageId: U8aFixed }>;
      /**
       * We have requested that a remote chain stops sending us XCM version change notifications.
       **/
      VersionNotifyUnrequested: AugmentedEvent<ApiType, [destination: XcmV3MultiLocation, cost: XcmV3MultiassetMultiAssets, messageId: U8aFixed], { destination: XcmV3MultiLocation, cost: XcmV3MultiassetMultiAssets, messageId: U8aFixed }>;
    };
    portal: {
      /**
       * Event documentation should end with an array that provides descriptive names for event
       * Gateway was registered successsfully. [ChainId]
       **/
      GatewayRegistered: AugmentedEvent<ApiType, [U8aFixed]>;
      /**
       * Header was successfully added
       **/
      HeaderSubmitted: AugmentedEvent<ApiType, [T3rnPrimitivesGatewayVendor, Bytes]>;
      /**
       * Gateway was set operational. [ChainId, bool]
       **/
      SetOperational: AugmentedEvent<ApiType, [U8aFixed, bool]>;
      /**
       * Gateway owner was set successfully. [ChainId, Vec<u8>]
       **/
      SetOwner: AugmentedEvent<ApiType, [U8aFixed, Bytes]>;
    };
    preimage: {
      /**
       * A preimage has ben cleared.
       **/
      Cleared: AugmentedEvent<ApiType, [hash_: H256], { hash_: H256 }>;
      /**
       * A preimage has been noted.
       **/
      Noted: AugmentedEvent<ApiType, [hash_: H256], { hash_: H256 }>;
      /**
       * A preimage has been requested.
       **/
      Requested: AugmentedEvent<ApiType, [hash_: H256], { hash_: H256 }>;
    };
    rewards: {
      AttesterRewarded: AugmentedEvent<ApiType, [AccountId32, u128]>;
      Claimed: AugmentedEvent<ApiType, [AccountId32, Vec<ITuple<[u128, Option<u32>]>>]>;
      CollatorRewarded: AugmentedEvent<ApiType, [AccountId32, u128]>;
      ExecutorRewarded: AugmentedEvent<ApiType, [AccountId32, u128]>;
      NewMaxRewardExecutorsKickbackSet: AugmentedEvent<ApiType, [Percent, Percent]>;
      PendingClaim: AugmentedEvent<ApiType, [AccountId32, u128]>;
    };
    rococoBridge: {
      HeadersAdded: AugmentedEvent<ApiType, [u32]>;
    };
    scheduler: {
      /**
       * The call for the provided hash was not found so the task has been aborted.
       **/
      CallUnavailable: AugmentedEvent<ApiType, [task: ITuple<[u32, u32]>, id: Option<U8aFixed>], { task: ITuple<[u32, u32]>, id: Option<U8aFixed> }>;
      /**
       * Canceled some task.
       **/
      Canceled: AugmentedEvent<ApiType, [when: u32, index: u32], { when: u32, index: u32 }>;
      /**
       * Dispatched some task.
       **/
      Dispatched: AugmentedEvent<ApiType, [task: ITuple<[u32, u32]>, id: Option<U8aFixed>, result: Result<Null, SpRuntimeDispatchError>], { task: ITuple<[u32, u32]>, id: Option<U8aFixed>, result: Result<Null, SpRuntimeDispatchError> }>;
      /**
       * The given task was unable to be renewed since the agenda is full at that block.
       **/
      PeriodicFailed: AugmentedEvent<ApiType, [task: ITuple<[u32, u32]>, id: Option<U8aFixed>], { task: ITuple<[u32, u32]>, id: Option<U8aFixed> }>;
      /**
       * The given task can never be executed since it is overweight.
       **/
      PermanentlyOverweight: AugmentedEvent<ApiType, [task: ITuple<[u32, u32]>, id: Option<U8aFixed>], { task: ITuple<[u32, u32]>, id: Option<U8aFixed> }>;
      /**
       * Scheduled some task.
       **/
      Scheduled: AugmentedEvent<ApiType, [when: u32, index: u32], { when: u32, index: u32 }>;
    };
    sepoliaBridge: {
      EpochUpdate: AugmentedEvent<ApiType, [PalletSepoliaFinalityVerifierEpochSubmitted]>;
    };
    session: {
      /**
       * New session has happened. Note that the argument is the session index, not the
       * block number as the type might suggest.
       **/
      NewSession: AugmentedEvent<ApiType, [sessionIndex: u32], { sessionIndex: u32 }>;
    };
    slashTreasury: {
      /**
       * Some funds have been allocated.
       **/
      Awarded: AugmentedEvent<ApiType, [proposalIndex: u32, award: u128, account: AccountId32], { proposalIndex: u32, award: u128, account: AccountId32 }>;
      /**
       * Some of our funds have been burnt.
       **/
      Burnt: AugmentedEvent<ApiType, [burntFunds: u128], { burntFunds: u128 }>;
      /**
       * Some funds have been deposited.
       **/
      Deposit: AugmentedEvent<ApiType, [value: u128], { value: u128 }>;
      /**
       * New proposal.
       **/
      Proposed: AugmentedEvent<ApiType, [proposalIndex: u32], { proposalIndex: u32 }>;
      /**
       * A proposal was rejected; funds were slashed.
       **/
      Rejected: AugmentedEvent<ApiType, [proposalIndex: u32, slashed: u128], { proposalIndex: u32, slashed: u128 }>;
      /**
       * Spending has finished; this is the amount that rolls over until next spend.
       **/
      Rollover: AugmentedEvent<ApiType, [rolloverBalance: u128], { rolloverBalance: u128 }>;
      /**
       * A new spend proposal has been approved.
       **/
      SpendApproved: AugmentedEvent<ApiType, [proposalIndex: u32, amount: u128, beneficiary: AccountId32], { proposalIndex: u32, amount: u128, beneficiary: AccountId32 }>;
      /**
       * We have ended a spend period and will now allocate funds.
       **/
      Spending: AugmentedEvent<ApiType, [budgetRemaining: u128], { budgetRemaining: u128 }>;
      /**
       * The inactive funds of the pallet have been updated.
       **/
      UpdatedInactive: AugmentedEvent<ApiType, [reactivated: u128, deactivated: u128], { reactivated: u128, deactivated: u128 }>;
    };
    sudo: {
      /**
       * The \[sudoer\] just switched identity; the old key is supplied if one existed.
       **/
      KeyChanged: AugmentedEvent<ApiType, [oldSudoer: Option<AccountId32>], { oldSudoer: Option<AccountId32> }>;
      /**
       * A sudo just took place. \[result\]
       **/
      Sudid: AugmentedEvent<ApiType, [sudoResult: Result<Null, SpRuntimeDispatchError>], { sudoResult: Result<Null, SpRuntimeDispatchError> }>;
      /**
       * A sudo just took place. \[result\]
       **/
      SudoAsDone: AugmentedEvent<ApiType, [sudoResult: Result<Null, SpRuntimeDispatchError>], { sudoResult: Result<Null, SpRuntimeDispatchError> }>;
    };
    system: {
      /**
       * `:code` was updated.
       **/
      CodeUpdated: AugmentedEvent<ApiType, []>;
      /**
       * An extrinsic failed.
       **/
      ExtrinsicFailed: AugmentedEvent<ApiType, [dispatchError: SpRuntimeDispatchError, dispatchInfo: FrameSupportDispatchDispatchInfo], { dispatchError: SpRuntimeDispatchError, dispatchInfo: FrameSupportDispatchDispatchInfo }>;
      /**
       * An extrinsic completed successfully.
       **/
      ExtrinsicSuccess: AugmentedEvent<ApiType, [dispatchInfo: FrameSupportDispatchDispatchInfo], { dispatchInfo: FrameSupportDispatchDispatchInfo }>;
      /**
       * An account was reaped.
       **/
      KilledAccount: AugmentedEvent<ApiType, [account: AccountId32], { account: AccountId32 }>;
      /**
       * A new account was created.
       **/
      NewAccount: AugmentedEvent<ApiType, [account: AccountId32], { account: AccountId32 }>;
      /**
       * On on-chain remark happened.
       **/
      Remarked: AugmentedEvent<ApiType, [sender: AccountId32, hash_: H256], { sender: AccountId32, hash_: H256 }>;
    };
    threeVm: {
      /**
       * An author of a module was removed [contract]
       **/
      AuthorRemoved: AugmentedEvent<ApiType, [AccountId32]>;
      /**
       * An author of a module was stored [contract, author]
       **/
      AuthorStored: AugmentedEvent<ApiType, [ITuple<[AccountId32, AccountId32]>]>;
      /**
       * A signal event was bounced beyond the threshold. [step, kind, xtx_id]
       **/
      ExceededBounceThrehold: AugmentedEvent<ApiType, [ITuple<[u32, T3rnSdkPrimitivesSignalSignalKind, H256]>]>;
      /**
       * A module was instantiated from the registry [id, module_author, module_type, module_len]
       **/
      ModuleInstantiated: AugmentedEvent<ApiType, [ITuple<[H256, AccountId32, T3rnPrimitivesContractMetadataContractType, u32]>]>;
      /**
       * A signal event was bounced back, because a signal was already sent for the current step. [step, kind, xtx_id]
       **/
      SignalBounced: AugmentedEvent<ApiType, [ITuple<[u32, T3rnSdkPrimitivesSignalSignalKind, H256]>]>;
    };
    transactionPayment: {
      /**
       * A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,
       * has been paid by `who`.
       **/
      TransactionFeePaid: AugmentedEvent<ApiType, [who: AccountId32, actualFee: u128, tip: u128], { who: AccountId32, actualFee: u128, tip: u128 }>;
    };
    treasury: {
      /**
       * Some funds have been allocated.
       **/
      Awarded: AugmentedEvent<ApiType, [proposalIndex: u32, award: u128, account: AccountId32], { proposalIndex: u32, award: u128, account: AccountId32 }>;
      /**
       * Some of our funds have been burnt.
       **/
      Burnt: AugmentedEvent<ApiType, [burntFunds: u128], { burntFunds: u128 }>;
      /**
       * Some funds have been deposited.
       **/
      Deposit: AugmentedEvent<ApiType, [value: u128], { value: u128 }>;
      /**
       * New proposal.
       **/
      Proposed: AugmentedEvent<ApiType, [proposalIndex: u32], { proposalIndex: u32 }>;
      /**
       * A proposal was rejected; funds were slashed.
       **/
      Rejected: AugmentedEvent<ApiType, [proposalIndex: u32, slashed: u128], { proposalIndex: u32, slashed: u128 }>;
      /**
       * Spending has finished; this is the amount that rolls over until next spend.
       **/
      Rollover: AugmentedEvent<ApiType, [rolloverBalance: u128], { rolloverBalance: u128 }>;
      /**
       * A new spend proposal has been approved.
       **/
      SpendApproved: AugmentedEvent<ApiType, [proposalIndex: u32, amount: u128, beneficiary: AccountId32], { proposalIndex: u32, amount: u128, beneficiary: AccountId32 }>;
      /**
       * We have ended a spend period and will now allocate funds.
       **/
      Spending: AugmentedEvent<ApiType, [budgetRemaining: u128], { budgetRemaining: u128 }>;
      /**
       * The inactive funds of the pallet have been updated.
       **/
      UpdatedInactive: AugmentedEvent<ApiType, [reactivated: u128, deactivated: u128], { reactivated: u128, deactivated: u128 }>;
    };
    utility: {
      /**
       * Batch of dispatches completed fully with no error.
       **/
      BatchCompleted: AugmentedEvent<ApiType, []>;
      /**
       * Batch of dispatches completed but has errors.
       **/
      BatchCompletedWithErrors: AugmentedEvent<ApiType, []>;
      /**
       * Batch of dispatches did not complete fully. Index of first failing dispatch given, as
       * well as the error.
       **/
      BatchInterrupted: AugmentedEvent<ApiType, [index: u32, error: SpRuntimeDispatchError], { index: u32, error: SpRuntimeDispatchError }>;
      /**
       * A call was dispatched.
       **/
      DispatchedAs: AugmentedEvent<ApiType, [result: Result<Null, SpRuntimeDispatchError>], { result: Result<Null, SpRuntimeDispatchError> }>;
      /**
       * A single item within a Batch of dispatches has completed with no error.
       **/
      ItemCompleted: AugmentedEvent<ApiType, []>;
      /**
       * A single item within a Batch of dispatches has completed with error.
       **/
      ItemFailed: AugmentedEvent<ApiType, [error: SpRuntimeDispatchError], { error: SpRuntimeDispatchError }>;
    };
    vacuum: {
      OrderStatusRead: AugmentedEvent<ApiType, [PalletCircuitVacuumOrderStatusRead]>;
    };
    xcmpQueue: {
      /**
       * Bad XCM format used.
       **/
      BadFormat: AugmentedEvent<ApiType, [messageHash: U8aFixed], { messageHash: U8aFixed }>;
      /**
       * Bad XCM version used.
       **/
      BadVersion: AugmentedEvent<ApiType, [messageHash: U8aFixed], { messageHash: U8aFixed }>;
      /**
       * Some XCM failed.
       **/
      Fail: AugmentedEvent<ApiType, [messageHash: U8aFixed, messageId: U8aFixed, error: XcmV3TraitsError, weight: SpWeightsWeightV2Weight], { messageHash: U8aFixed, messageId: U8aFixed, error: XcmV3TraitsError, weight: SpWeightsWeightV2Weight }>;
      /**
       * An XCM exceeded the individual message weight budget.
       **/
      OverweightEnqueued: AugmentedEvent<ApiType, [sender: u32, sentAt: u32, index: u64, required: SpWeightsWeightV2Weight], { sender: u32, sentAt: u32, index: u64, required: SpWeightsWeightV2Weight }>;
      /**
       * An XCM from the overweight queue was executed with the given actual weight used.
       **/
      OverweightServiced: AugmentedEvent<ApiType, [index: u64, used: SpWeightsWeightV2Weight], { index: u64, used: SpWeightsWeightV2Weight }>;
      /**
       * Some XCM was executed ok.
       **/
      Success: AugmentedEvent<ApiType, [messageHash: U8aFixed, messageId: U8aFixed, weight: SpWeightsWeightV2Weight], { messageHash: U8aFixed, messageId: U8aFixed, weight: SpWeightsWeightV2Weight }>;
      /**
       * An HRMP message was sent to a sibling parachain.
       **/
      XcmpMessageSent: AugmentedEvent<ApiType, [messageHash: U8aFixed], { messageHash: U8aFixed }>;
    };
    xdns: {
      /**
       * \[requester, gateway_record_id\]
       **/
      GatewayRecordPurged: AugmentedEvent<ApiType, [AccountId32, U8aFixed]>;
      /**
       * \[gateway_4b_id\]
       **/
      GatewayRecordStored: AugmentedEvent<ApiType, [U8aFixed]>;
      /**
       * \[asset_id, gateway_4b_id\]
       **/
      NewTokenAssetRegistered: AugmentedEvent<ApiType, [u32, U8aFixed]>;
      /**
       * \[asset_id, gateway_4b_id\]
       **/
      NewTokenLinkedToGateway: AugmentedEvent<ApiType, [u32, U8aFixed]>;
      /**
       * \[requester, xdns_record_id\]
       **/
      XdnsRecordPurged: AugmentedEvent<ApiType, [AccountId32, U8aFixed]>;
      /**
       * \[xdns_record_id\]
       **/
      XdnsRecordUpdated: AugmentedEvent<ApiType, [U8aFixed]>;
    };
  } // AugmentedEvents
} // declare module
