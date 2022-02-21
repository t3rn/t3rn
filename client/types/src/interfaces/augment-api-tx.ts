// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

import type { ApiTypes, SubmittableExtrinsic } from '@polkadot/api/types';
import type { Bytes, Compact, Option, U8aFixed, Vec, bool, u128, u16, u32, u64 } from '@polkadot/types';
import type { Extrinsic } from '@polkadot/types/interfaces/extrinsics';
import type { AccountId32, Call, H256, MultiAddress, Perbill } from '@polkadot/types/interfaces/runtime';
import type { PalletImOnlineHeartbeat, PalletImOnlineSr25519AppSr25519Signature, PalletMultisigTimepoint, PolkadotParachainPrimitivesHrmpChannelId, PolkadotPrimitivesV1InherentData, PolkadotRuntimeParachainsParasParaGenesisArgs, SpConsensusBabeDigestsNextConfigDescriptor, SpConsensusSlotsEquivocationProof, SpFinalityGrandpaEquivocationProof, SpRuntimeGenericHeader, SpRuntimeMultiSignature, SpRuntimeMultiSigner, SpSessionMembershipProof, XcmV1MultiLocation, XcmV2WeightLimit, XcmVersionedMultiAssets, XcmVersionedMultiLocation, XcmVersionedXcm } from '@polkadot/types/lookup';
import type { AnyNumber, ITuple } from '@polkadot/types/types';

declare module '@polkadot/api/types/submittable' {
  export interface AugmentedSubmittables<ApiType> {
    auctions: {
      /**
       * Make a new bid from an account (including a parachain account) for deploying a new
       * parachain.
       * 
       * Multiple simultaneous bids from the same bidder are allowed only as long as all active
       * bids overlap each other (i.e. are mutually exclusive). Bids cannot be redacted.
       * 
       * - `sub` is the sub-bidder ID, allowing for multiple competing bids to be made by (and
       * funded by) the same account.
       * - `auction_index` is the index of the auction to bid on. Should just be the present
       * value of `AuctionCounter`.
       * - `first_slot` is the first lease period index of the range to bid on. This is the
       * absolute lease period index value, not an auction-specific offset.
       * - `last_slot` is the last lease period index of the range to bid on. This is the
       * absolute lease period index value, not an auction-specific offset.
       * - `amount` is the amount to bid to be held as deposit for the parachain should the
       * bid win. This amount is held throughout the range.
       **/
      bid: AugmentedSubmittable<(para: Compact<u32> | AnyNumber | Uint8Array, auctionIndex: Compact<u32> | AnyNumber | Uint8Array, firstSlot: Compact<u32> | AnyNumber | Uint8Array, lastSlot: Compact<u32> | AnyNumber | Uint8Array, amount: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u32>, Compact<u32>, Compact<u32>, Compact<u32>, Compact<u128>]>;
      /**
       * Cancel an in-progress auction.
       * 
       * Can only be called by Root origin.
       **/
      cancelAuction: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
      /**
       * Create a new auction.
       * 
       * This can only happen when there isn't already an auction in progress and may only be
       * called by the root origin. Accepts the `duration` of this auction and the
       * `lease_period_index` of the initial lease period of the four that are to be auctioned.
       **/
      newAuction: AugmentedSubmittable<(duration: Compact<u32> | AnyNumber | Uint8Array, leasePeriodIndex: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u32>, Compact<u32>]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    authorship: {
      /**
       * Provide a set of uncles.
       **/
      setUncles: AugmentedSubmittable<(newUncles: Vec<SpRuntimeGenericHeader> | (SpRuntimeGenericHeader | { parentHash?: any; number?: any; stateRoot?: any; extrinsicsRoot?: any; digest?: any } | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [Vec<SpRuntimeGenericHeader>]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    babe: {
      /**
       * Plan an epoch config change. The epoch config change is recorded and will be enacted on
       * the next call to `enact_epoch_change`. The config will be activated one epoch after.
       * Multiple calls to this method will replace any existing planned config change that had
       * not been enacted yet.
       **/
      planConfigChange: AugmentedSubmittable<(config: SpConsensusBabeDigestsNextConfigDescriptor | { V1: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [SpConsensusBabeDigestsNextConfigDescriptor]>;
      /**
       * Report authority equivocation/misbehavior. This method will verify
       * the equivocation proof and validate the given key ownership proof
       * against the extracted offender. If both are valid, the offence will
       * be reported.
       **/
      reportEquivocation: AugmentedSubmittable<(equivocationProof: SpConsensusSlotsEquivocationProof | { offender?: any; slot?: any; firstHeader?: any; secondHeader?: any } | string | Uint8Array, keyOwnerProof: SpSessionMembershipProof | { session?: any; trieNodes?: any; validatorCount?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [SpConsensusSlotsEquivocationProof, SpSessionMembershipProof]>;
      /**
       * Report authority equivocation/misbehavior. This method will verify
       * the equivocation proof and validate the given key ownership proof
       * against the extracted offender. If both are valid, the offence will
       * be reported.
       * This extrinsic must be called unsigned and it is expected that only
       * block authors will call it (validated in `ValidateUnsigned`), as such
       * if the block author is defined it will be defined as the equivocation
       * reporter.
       **/
      reportEquivocationUnsigned: AugmentedSubmittable<(equivocationProof: SpConsensusSlotsEquivocationProof | { offender?: any; slot?: any; firstHeader?: any; secondHeader?: any } | string | Uint8Array, keyOwnerProof: SpSessionMembershipProof | { session?: any; trieNodes?: any; validatorCount?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [SpConsensusSlotsEquivocationProof, SpSessionMembershipProof]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    balances: {
      /**
       * Exactly as `transfer`, except the origin must be root and the source account may be
       * specified.
       * # <weight>
       * - Same as transfer, but additional read and write because the source account is not
       * assumed to be in the overlay.
       * # </weight>
       **/
      forceTransfer: AugmentedSubmittable<(source: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, dest: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, value: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, MultiAddress, Compact<u128>]>;
      /**
       * Unreserve some balance from a user by force.
       * 
       * Can only be called by ROOT.
       **/
      forceUnreserve: AugmentedSubmittable<(who: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, amount: u128 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, u128]>;
      /**
       * Set the balances of a given account.
       * 
       * This will alter `FreeBalance` and `ReservedBalance` in storage. it will
       * also decrease the total issuance of the system (`TotalIssuance`).
       * If the new free or reserved balance is below the existential deposit,
       * it will reset the account nonce (`frame_system::AccountNonce`).
       * 
       * The dispatch origin for this call is `root`.
       **/
      setBalance: AugmentedSubmittable<(who: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, newFree: Compact<u128> | AnyNumber | Uint8Array, newReserved: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, Compact<u128>, Compact<u128>]>;
      /**
       * Transfer some liquid free balance to another account.
       * 
       * `transfer` will set the `FreeBalance` of the sender and receiver.
       * It will decrease the total issuance of the system by the `TransferFee`.
       * If the sender's account is below the existential deposit as a result
       * of the transfer, the account will be reaped.
       * 
       * The dispatch origin for this call must be `Signed` by the transactor.
       * 
       * # <weight>
       * - Dependent on arguments but not critical, given proper implementations for input config
       * types. See related functions below.
       * - It contains a limited number of reads and writes internally and no complex
       * computation.
       * 
       * Related functions:
       * 
       * - `ensure_can_withdraw` is always called internally but has a bounded complexity.
       * - Transferring balances to accounts that did not exist before will cause
       * `T::OnNewAccount::on_new_account` to be called.
       * - Removing enough funds from an account will trigger `T::DustRemoval::on_unbalanced`.
       * - `transfer_keep_alive` works the same way as `transfer`, but has an additional check
       * that the transfer will not kill the origin account.
       * ---------------------------------
       * - Origin account is already in memory, so no DB operations for them.
       * # </weight>
       **/
      transfer: AugmentedSubmittable<(dest: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, value: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, Compact<u128>]>;
      /**
       * Transfer the entire transferable balance from the caller account.
       * 
       * NOTE: This function only attempts to transfer _transferable_ balances. This means that
       * any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be
       * transferred by this function. To ensure that this function results in a killed account,
       * you might need to prepare the account by removing any reference counters, storage
       * deposits, etc...
       * 
       * The dispatch origin of this call must be Signed.
       * 
       * - `dest`: The recipient of the transfer.
       * - `keep_alive`: A boolean to determine if the `transfer_all` operation should send all
       * of the funds the account has, causing the sender account to be killed (false), or
       * transfer everything except at least the existential deposit, which will guarantee to
       * keep the sender account alive (true). # <weight>
       * - O(1). Just like transfer, but reading the user's transferable balance first.
       * #</weight>
       **/
      transferAll: AugmentedSubmittable<(dest: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, keepAlive: bool | boolean | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, bool]>;
      /**
       * Same as the [`transfer`] call, but with a check that the transfer will not kill the
       * origin account.
       * 
       * 99% of the time you want [`transfer`] instead.
       * 
       * [`transfer`]: struct.Pallet.html#method.transfer
       **/
      transferKeepAlive: AugmentedSubmittable<(dest: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, value: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, Compact<u128>]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    collective: {
      /**
       * Close a vote that is either approved, disapproved or whose voting period has ended.
       * 
       * May be called by any signed account in order to finish voting and close the proposal.
       * 
       * If called before the end of the voting period it will only close the vote if it is
       * has enough votes to be approved or disapproved.
       * 
       * If called after the end of the voting period abstentions are counted as rejections
       * unless there is a prime member set and the prime member cast an approval.
       * 
       * If the close operation completes successfully with disapproval, the transaction fee will
       * be waived. Otherwise execution of the approved operation will be charged to the caller.
       * 
       * + `proposal_weight_bound`: The maximum amount of weight consumed by executing the closed
       * proposal.
       * + `length_bound`: The upper bound for the length of the proposal in storage. Checked via
       * `storage::read` so it is `size_of::<u32>() == 4` larger than the pure length.
       * 
       * # <weight>
       * ## Weight
       * - `O(B + M + P1 + P2)` where:
       * - `B` is `proposal` size in bytes (length-fee-bounded)
       * - `M` is members-count (code- and governance-bounded)
       * - `P1` is the complexity of `proposal` preimage.
       * - `P2` is proposal-count (code-bounded)
       * - DB:
       * - 2 storage reads (`Members`: codec `O(M)`, `Prime`: codec `O(1)`)
       * - 3 mutations (`Voting`: codec `O(M)`, `ProposalOf`: codec `O(B)`, `Proposals`: codec
       * `O(P2)`)
       * - any mutations done while executing `proposal` (`P1`)
       * - up to 3 events
       * # </weight>
       **/
      close: AugmentedSubmittable<(proposalHash: H256 | string | Uint8Array, index: Compact<u32> | AnyNumber | Uint8Array, proposalWeightBound: Compact<u64> | AnyNumber | Uint8Array, lengthBound: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [H256, Compact<u32>, Compact<u64>, Compact<u32>]>;
      /**
       * Disapprove a proposal, close, and remove it from the system, regardless of its current
       * state.
       * 
       * Must be called by the Root origin.
       * 
       * Parameters:
       * * `proposal_hash`: The hash of the proposal that should be disapproved.
       * 
       * # <weight>
       * Complexity: O(P) where P is the number of max proposals
       * DB Weight:
       * * Reads: Proposals
       * * Writes: Voting, Proposals, ProposalOf
       * # </weight>
       **/
      disapproveProposal: AugmentedSubmittable<(proposalHash: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [H256]>;
      /**
       * Dispatch a proposal from a member using the `Member` origin.
       * 
       * Origin must be a member of the collective.
       * 
       * # <weight>
       * ## Weight
       * - `O(M + P)` where `M` members-count (code-bounded) and `P` complexity of dispatching
       * `proposal`
       * - DB: 1 read (codec `O(M)`) + DB access of `proposal`
       * - 1 event
       * # </weight>
       **/
      execute: AugmentedSubmittable<(proposal: Call | { callIndex?: any; args?: any } | string | Uint8Array, lengthBound: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Call, Compact<u32>]>;
      /**
       * Add a new proposal to either be voted on or executed directly.
       * 
       * Requires the sender to be member.
       * 
       * `threshold` determines whether `proposal` is executed directly (`threshold < 2`)
       * or put up for voting.
       * 
       * # <weight>
       * ## Weight
       * - `O(B + M + P1)` or `O(B + M + P2)` where:
       * - `B` is `proposal` size in bytes (length-fee-bounded)
       * - `M` is members-count (code- and governance-bounded)
       * - branching is influenced by `threshold` where:
       * - `P1` is proposal execution complexity (`threshold < 2`)
       * - `P2` is proposals-count (code-bounded) (`threshold >= 2`)
       * - DB:
       * - 1 storage read `is_member` (codec `O(M)`)
       * - 1 storage read `ProposalOf::contains_key` (codec `O(1)`)
       * - DB accesses influenced by `threshold`:
       * - EITHER storage accesses done by `proposal` (`threshold < 2`)
       * - OR proposal insertion (`threshold <= 2`)
       * - 1 storage mutation `Proposals` (codec `O(P2)`)
       * - 1 storage mutation `ProposalCount` (codec `O(1)`)
       * - 1 storage write `ProposalOf` (codec `O(B)`)
       * - 1 storage write `Voting` (codec `O(M)`)
       * - 1 event
       * # </weight>
       **/
      propose: AugmentedSubmittable<(threshold: Compact<u32> | AnyNumber | Uint8Array, proposal: Call | { callIndex?: any; args?: any } | string | Uint8Array, lengthBound: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u32>, Call, Compact<u32>]>;
      /**
       * Set the collective's membership.
       * 
       * - `new_members`: The new member list. Be nice to the chain and provide it sorted.
       * - `prime`: The prime member whose vote sets the default.
       * - `old_count`: The upper bound for the previous number of members in storage. Used for
       * weight estimation.
       * 
       * Requires root origin.
       * 
       * NOTE: Does not enforce the expected `MaxMembers` limit on the amount of members, but
       * the weight estimations rely on it to estimate dispatchable weight.
       * 
       * # WARNING:
       * 
       * The `pallet-collective` can also be managed by logic outside of the pallet through the
       * implementation of the trait [`ChangeMembers`].
       * Any call to `set_members` must be careful that the member set doesn't get out of sync
       * with other logic managing the member set.
       * 
       * # <weight>
       * ## Weight
       * - `O(MP + N)` where:
       * - `M` old-members-count (code- and governance-bounded)
       * - `N` new-members-count (code- and governance-bounded)
       * - `P` proposals-count (code-bounded)
       * - DB:
       * - 1 storage mutation (codec `O(M)` read, `O(N)` write) for reading and writing the
       * members
       * - 1 storage read (codec `O(P)`) for reading the proposals
       * - `P` storage mutations (codec `O(M)`) for updating the votes for each proposal
       * - 1 storage write (codec `O(1)`) for deleting the old `prime` and setting the new one
       * # </weight>
       **/
      setMembers: AugmentedSubmittable<(newMembers: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[], prime: Option<AccountId32> | null | object | string | Uint8Array, oldCount: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Vec<AccountId32>, Option<AccountId32>, u32]>;
      /**
       * Add an aye or nay vote for the sender to the given proposal.
       * 
       * Requires the sender to be a member.
       * 
       * Transaction fees will be waived if the member is voting on any particular proposal
       * for the first time and the call is successful. Subsequent vote changes will charge a
       * fee.
       * # <weight>
       * ## Weight
       * - `O(M)` where `M` is members-count (code- and governance-bounded)
       * - DB:
       * - 1 storage read `Members` (codec `O(M)`)
       * - 1 storage mutation `Voting` (codec `O(M)`)
       * - 1 event
       * # </weight>
       **/
      vote: AugmentedSubmittable<(proposal: H256 | string | Uint8Array, index: Compact<u32> | AnyNumber | Uint8Array, approve: bool | boolean | Uint8Array) => SubmittableExtrinsic<ApiType>, [H256, Compact<u32>, bool]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    configuration: {
      /**
       * Set the availability period for parachains.
       **/
      setChainAvailabilityPeriod: AugmentedSubmittable<(updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Set the acceptance period for an included candidate.
       **/
      setCodeRetentionPeriod: AugmentedSubmittable<(updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Set the dispute conclusion by time out period.
       **/
      setDisputeConclusionByTimeOutPeriod: AugmentedSubmittable<(updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Set the maximum number of dispute spam slots.
       **/
      setDisputeMaxSpamSlots: AugmentedSubmittable<(updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Set the dispute period, in number of sessions to keep for disputes.
       **/
      setDisputePeriod: AugmentedSubmittable<(updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Set the dispute post conclusion acceptance period.
       **/
      setDisputePostConclusionAcceptancePeriod: AugmentedSubmittable<(updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Set the parachain validator-group rotation frequency
       **/
      setGroupRotationFrequency: AugmentedSubmittable<(updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Sets the maximum number of messages allowed in an HRMP channel at once.
       **/
      setHrmpChannelMaxCapacity: AugmentedSubmittable<(updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Sets the maximum size of a message that could ever be put into an HRMP channel.
       **/
      setHrmpChannelMaxMessageSize: AugmentedSubmittable<(updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Sets the maximum total size of messages in bytes allowed in an HRMP channel at once.
       **/
      setHrmpChannelMaxTotalSize: AugmentedSubmittable<(updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Sets the maximum number of outbound HRMP messages can be sent by a candidate.
       **/
      setHrmpMaxMessageNumPerCandidate: AugmentedSubmittable<(updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Sets the maximum number of inbound HRMP channels a parachain is allowed to accept.
       **/
      setHrmpMaxParachainInboundChannels: AugmentedSubmittable<(updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Sets the maximum number of outbound HRMP channels a parachain is allowed to open.
       **/
      setHrmpMaxParachainOutboundChannels: AugmentedSubmittable<(updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Sets the maximum number of inbound HRMP channels a parathread is allowed to accept.
       **/
      setHrmpMaxParathreadInboundChannels: AugmentedSubmittable<(updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Sets the maximum number of outbound HRMP channels a parathread is allowed to open.
       **/
      setHrmpMaxParathreadOutboundChannels: AugmentedSubmittable<(updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Sets the number of sessions after which an HRMP open channel request expires.
       **/
      setHrmpOpenRequestTtl: AugmentedSubmittable<(updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Sets the amount of funds that the recipient should provide for accepting opening an HRMP
       * channel.
       **/
      setHrmpRecipientDeposit: AugmentedSubmittable<(updated: u128 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u128]>;
      /**
       * Sets the amount of funds that the sender should provide for opening an HRMP channel.
       **/
      setHrmpSenderDeposit: AugmentedSubmittable<(updated: u128 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u128]>;
      /**
       * Set the max validation code size for incoming upgrades.
       **/
      setMaxCodeSize: AugmentedSubmittable<(updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Set the critical downward message size.
       **/
      setMaxDownwardMessageSize: AugmentedSubmittable<(updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Set the max head data size for paras.
       **/
      setMaxHeadDataSize: AugmentedSubmittable<(updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Set the max POV block size for incoming upgrades.
       **/
      setMaxPovSize: AugmentedSubmittable<(updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Sets the maximum number of messages that a candidate can contain.
       **/
      setMaxUpwardMessageNumPerCandidate: AugmentedSubmittable<(updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Sets the maximum size of an upward message that can be sent by a candidate.
       **/
      setMaxUpwardMessageSize: AugmentedSubmittable<(updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Sets the maximum items that can present in a upward dispatch queue at once.
       **/
      setMaxUpwardQueueCount: AugmentedSubmittable<(updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Sets the maximum total size of items that can present in a upward dispatch queue at once.
       **/
      setMaxUpwardQueueSize: AugmentedSubmittable<(updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Set the maximum number of validators to use in parachain consensus.
       **/
      setMaxValidators: AugmentedSubmittable<(updated: Option<u32> | null | object | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Option<u32>]>;
      /**
       * Set the maximum number of validators to assign to any core.
       **/
      setMaxValidatorsPerCore: AugmentedSubmittable<(updated: Option<u32> | null | object | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Option<u32>]>;
      /**
       * Set the total number of delay tranches.
       **/
      setNDelayTranches: AugmentedSubmittable<(updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Set the number of validators needed to approve a block.
       **/
      setNeededApprovals: AugmentedSubmittable<(updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Set the no show slots, in number of number of consensus slots.
       * Must be at least 1.
       **/
      setNoShowSlots: AugmentedSubmittable<(updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Set the number of parathread execution cores.
       **/
      setParathreadCores: AugmentedSubmittable<(updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Set the number of retries for a particular parathread.
       **/
      setParathreadRetries: AugmentedSubmittable<(updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Set the number of samples to do of the `RelayVRFModulo` approval assignment criterion.
       **/
      setRelayVrfModuloSamples: AugmentedSubmittable<(updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Set the scheduling lookahead, in expected number of blocks at peak throughput.
       **/
      setSchedulingLookahead: AugmentedSubmittable<(updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Set the availability period for parathreads.
       **/
      setThreadAvailabilityPeriod: AugmentedSubmittable<(updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Sets the maximum amount of weight any individual upward message may consume.
       **/
      setUmpMaxIndividualWeight: AugmentedSubmittable<(updated: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u64]>;
      /**
       * Sets the soft limit for the phase of dispatching dispatchable upward messages.
       **/
      setUmpServiceTotalWeight: AugmentedSubmittable<(updated: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u64]>;
      /**
       * Set the validation upgrade delay.
       **/
      setValidationUpgradeDelay: AugmentedSubmittable<(updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Set the validation upgrade frequency.
       **/
      setValidationUpgradeFrequency: AugmentedSubmittable<(updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Set the zeroth delay tranche width.
       **/
      setZerothDelayTrancheWidth: AugmentedSubmittable<(updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    crowdloan: {
      /**
       * Add an optional memo to an existing crowdloan contribution.
       * 
       * Origin must be Signed, and the user must have contributed to the crowdloan.
       **/
      addMemo: AugmentedSubmittable<(index: u32 | AnyNumber | Uint8Array, memo: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, Bytes]>;
      /**
       * Contribute to a crowd sale. This will transfer some balance over to fund a parachain
       * slot. It will be withdrawable when the crowdloan has ended and the funds are unused.
       **/
      contribute: AugmentedSubmittable<(index: Compact<u32> | AnyNumber | Uint8Array, value: Compact<u128> | AnyNumber | Uint8Array, signature: Option<SpRuntimeMultiSignature> | null | object | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u32>, Compact<u128>, Option<SpRuntimeMultiSignature>]>;
      /**
       * Create a new crowdloaning campaign for a parachain slot with the given lease period range.
       * 
       * This applies a lock to your parachain configuration, ensuring that it cannot be changed
       * by the parachain manager.
       **/
      create: AugmentedSubmittable<(index: Compact<u32> | AnyNumber | Uint8Array, cap: Compact<u128> | AnyNumber | Uint8Array, firstPeriod: Compact<u32> | AnyNumber | Uint8Array, lastPeriod: Compact<u32> | AnyNumber | Uint8Array, end: Compact<u32> | AnyNumber | Uint8Array, verifier: Option<SpRuntimeMultiSigner> | null | object | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u32>, Compact<u128>, Compact<u32>, Compact<u32>, Compact<u32>, Option<SpRuntimeMultiSigner>]>;
      /**
       * Remove a fund after the retirement period has ended and all funds have been returned.
       **/
      dissolve: AugmentedSubmittable<(index: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u32>]>;
      /**
       * Edit the configuration for an in-progress crowdloan.
       * 
       * Can only be called by Root origin.
       **/
      edit: AugmentedSubmittable<(index: Compact<u32> | AnyNumber | Uint8Array, cap: Compact<u128> | AnyNumber | Uint8Array, firstPeriod: Compact<u32> | AnyNumber | Uint8Array, lastPeriod: Compact<u32> | AnyNumber | Uint8Array, end: Compact<u32> | AnyNumber | Uint8Array, verifier: Option<SpRuntimeMultiSigner> | null | object | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u32>, Compact<u128>, Compact<u32>, Compact<u32>, Compact<u32>, Option<SpRuntimeMultiSigner>]>;
      /**
       * Poke the fund into `NewRaise`
       * 
       * Origin must be Signed, and the fund has non-zero raise.
       **/
      poke: AugmentedSubmittable<(index: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Automatically refund contributors of an ended crowdloan.
       * Due to weight restrictions, this function may need to be called multiple
       * times to fully refund all users. We will refund `RemoveKeysLimit` users at a time.
       * 
       * Origin must be signed, but can come from anyone.
       **/
      refund: AugmentedSubmittable<(index: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u32>]>;
      /**
       * Withdraw full balance of a specific contributor.
       * 
       * Origin must be signed, but can come from anyone.
       * 
       * The fund must be either in, or ready for, retirement. For a fund to be *in* retirement, then the retirement
       * flag must be set. For a fund to be ready for retirement, then:
       * - it must not already be in retirement;
       * - the amount of raised funds must be bigger than the _free_ balance of the account;
       * - and either:
       * - the block number must be at least `end`; or
       * - the current lease period must be greater than the fund's `last_period`.
       * 
       * In this case, the fund's retirement flag is set and its `end` is reset to the current block
       * number.
       * 
       * - `who`: The account whose contribution should be withdrawn.
       * - `index`: The parachain to whose crowdloan the contribution was made.
       **/
      withdraw: AugmentedSubmittable<(who: AccountId32 | string | Uint8Array, index: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [AccountId32, Compact<u32>]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    dmp: {
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    grandpa: {
      /**
       * Note that the current authority set of the GRANDPA finality gadget has
       * stalled. This will trigger a forced authority set change at the beginning
       * of the next session, to be enacted `delay` blocks after that. The delay
       * should be high enough to safely assume that the block signalling the
       * forced change will not be re-orged (e.g. 1000 blocks). The GRANDPA voters
       * will start the new authority set using the given finalized block as base.
       * Only callable by root.
       **/
      noteStalled: AugmentedSubmittable<(delay: u32 | AnyNumber | Uint8Array, bestFinalizedBlockNumber: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32]>;
      /**
       * Report voter equivocation/misbehavior. This method will verify the
       * equivocation proof and validate the given key ownership proof
       * against the extracted offender. If both are valid, the offence
       * will be reported.
       **/
      reportEquivocation: AugmentedSubmittable<(equivocationProof: SpFinalityGrandpaEquivocationProof | { setId?: any; equivocation?: any } | string | Uint8Array, keyOwnerProof: SpSessionMembershipProof | { session?: any; trieNodes?: any; validatorCount?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [SpFinalityGrandpaEquivocationProof, SpSessionMembershipProof]>;
      /**
       * Report voter equivocation/misbehavior. This method will verify the
       * equivocation proof and validate the given key ownership proof
       * against the extracted offender. If both are valid, the offence
       * will be reported.
       * 
       * This extrinsic must be called unsigned and it is expected that only
       * block authors will call it (validated in `ValidateUnsigned`), as such
       * if the block author is defined it will be defined as the equivocation
       * reporter.
       **/
      reportEquivocationUnsigned: AugmentedSubmittable<(equivocationProof: SpFinalityGrandpaEquivocationProof | { setId?: any; equivocation?: any } | string | Uint8Array, keyOwnerProof: SpSessionMembershipProof | { session?: any; trieNodes?: any; validatorCount?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [SpFinalityGrandpaEquivocationProof, SpSessionMembershipProof]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    hrmp: {
      /**
       * This extrinsic triggers the cleanup of all the HRMP storage items that
       * a para may have. Normally this happens once per session, but this allows
       * you to trigger the cleanup immediately for a specific parachain.
       * 
       * Origin must be Root.
       **/
      forceCleanHrmp: AugmentedSubmittable<(para: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Force process HRMP close channel requests.
       * 
       * If there are pending HRMP close channel requests, you can use this
       * function process all of those requests immediately.
       **/
      forceProcessHrmpClose: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
      /**
       * Force process HRMP open channel requests.
       * 
       * If there are pending HRMP open channel requests, you can use this
       * function process all of those requests immediately.
       **/
      forceProcessHrmpOpen: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
      /**
       * Accept a pending open channel request from the given sender.
       * 
       * The channel will be opened only on the next session boundary.
       **/
      hrmpAcceptOpenChannel: AugmentedSubmittable<(sender: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * This cancels a pending open channel request. It can be canceled be either of the sender
       * or the recipient for that request. The origin must be either of those.
       * 
       * The cancellation happens immediately. It is not possible to cancel the request if it is
       * already accepted.
       **/
      hrmpCancelOpenRequest: AugmentedSubmittable<(channelId: PolkadotParachainPrimitivesHrmpChannelId | { sender?: any; recipient?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [PolkadotParachainPrimitivesHrmpChannelId]>;
      /**
       * Initiate unilateral closing of a channel. The origin must be either the sender or the
       * recipient in the channel being closed.
       * 
       * The closure can only happen on a session change.
       **/
      hrmpCloseChannel: AugmentedSubmittable<(channelId: PolkadotParachainPrimitivesHrmpChannelId | { sender?: any; recipient?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [PolkadotParachainPrimitivesHrmpChannelId]>;
      /**
       * Initiate opening a channel from a parachain to a given recipient with given channel
       * parameters.
       * 
       * - `proposed_max_capacity` - specifies how many messages can be in the channel at once.
       * - `proposed_max_message_size` - specifies the maximum size of the messages.
       * 
       * These numbers are a subject to the relay-chain configuration limits.
       * 
       * The channel can be opened only after the recipient confirms it and only on a session
       * change.
       **/
      hrmpInitOpenChannel: AugmentedSubmittable<(recipient: u32 | AnyNumber | Uint8Array, proposedMaxCapacity: u32 | AnyNumber | Uint8Array, proposedMaxMessageSize: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, u32]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    imOnline: {
      /**
       * # <weight>
       * - Complexity: `O(K + E)` where K is length of `Keys` (heartbeat.validators_len) and E is
       * length of `heartbeat.network_state.external_address`
       * - `O(K)`: decoding of length `K`
       * - `O(E)`: decoding/encoding of length `E`
       * - DbReads: pallet_session `Validators`, pallet_session `CurrentIndex`, `Keys`,
       * `ReceivedHeartbeats`
       * - DbWrites: `ReceivedHeartbeats`
       * # </weight>
       **/
      heartbeat: AugmentedSubmittable<(heartbeat: PalletImOnlineHeartbeat | { blockNumber?: any; networkState?: any; sessionIndex?: any; authorityIndex?: any; validatorsLen?: any } | string | Uint8Array, signature: PalletImOnlineSr25519AppSr25519Signature | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [PalletImOnlineHeartbeat, PalletImOnlineSr25519AppSr25519Signature]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    indices: {
      /**
       * Assign an previously unassigned index.
       * 
       * Payment: `Deposit` is reserved from the sender account.
       * 
       * The dispatch origin for this call must be _Signed_.
       * 
       * - `index`: the index to be claimed. This must not be in use.
       * 
       * Emits `IndexAssigned` if successful.
       * 
       * # <weight>
       * - `O(1)`.
       * - One storage mutation (codec `O(1)`).
       * - One reserve operation.
       * - One event.
       * -------------------
       * - DB Weight: 1 Read/Write (Accounts)
       * # </weight>
       **/
      claim: AugmentedSubmittable<(index: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Force an index to an account. This doesn't require a deposit. If the index is already
       * held, then any deposit is reimbursed to its current owner.
       * 
       * The dispatch origin for this call must be _Root_.
       * 
       * - `index`: the index to be (re-)assigned.
       * - `new`: the new owner of the index. This function is a no-op if it is equal to sender.
       * - `freeze`: if set to `true`, will freeze the index so it cannot be transferred.
       * 
       * Emits `IndexAssigned` if successful.
       * 
       * # <weight>
       * - `O(1)`.
       * - One storage mutation (codec `O(1)`).
       * - Up to one reserve operation.
       * - One event.
       * -------------------
       * - DB Weight:
       * - Reads: Indices Accounts, System Account (original owner)
       * - Writes: Indices Accounts, System Account (original owner)
       * # </weight>
       **/
      forceTransfer: AugmentedSubmittable<(updated: AccountId32 | string | Uint8Array, index: u32 | AnyNumber | Uint8Array, freeze: bool | boolean | Uint8Array) => SubmittableExtrinsic<ApiType>, [AccountId32, u32, bool]>;
      /**
       * Free up an index owned by the sender.
       * 
       * Payment: Any previous deposit placed for the index is unreserved in the sender account.
       * 
       * The dispatch origin for this call must be _Signed_ and the sender must own the index.
       * 
       * - `index`: the index to be freed. This must be owned by the sender.
       * 
       * Emits `IndexFreed` if successful.
       * 
       * # <weight>
       * - `O(1)`.
       * - One storage mutation (codec `O(1)`).
       * - One reserve operation.
       * - One event.
       * -------------------
       * - DB Weight: 1 Read/Write (Accounts)
       * # </weight>
       **/
      free: AugmentedSubmittable<(index: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Freeze an index so it will always point to the sender account. This consumes the
       * deposit.
       * 
       * The dispatch origin for this call must be _Signed_ and the signing account must have a
       * non-frozen account `index`.
       * 
       * - `index`: the index to be frozen in place.
       * 
       * Emits `IndexFrozen` if successful.
       * 
       * # <weight>
       * - `O(1)`.
       * - One storage mutation (codec `O(1)`).
       * - Up to one slash operation.
       * - One event.
       * -------------------
       * - DB Weight: 1 Read/Write (Accounts)
       * # </weight>
       **/
      freeze: AugmentedSubmittable<(index: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Assign an index already owned by the sender to another account. The balance reservation
       * is effectively transferred to the new account.
       * 
       * The dispatch origin for this call must be _Signed_.
       * 
       * - `index`: the index to be re-assigned. This must be owned by the sender.
       * - `new`: the new owner of the index. This function is a no-op if it is equal to sender.
       * 
       * Emits `IndexAssigned` if successful.
       * 
       * # <weight>
       * - `O(1)`.
       * - One storage mutation (codec `O(1)`).
       * - One transfer operation.
       * - One event.
       * -------------------
       * - DB Weight:
       * - Reads: Indices Accounts, System Account (recipient)
       * - Writes: Indices Accounts, System Account (recipient)
       * # </weight>
       **/
      transfer: AugmentedSubmittable<(updated: AccountId32 | string | Uint8Array, index: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [AccountId32, u32]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    initializer: {
      /**
       * Issue a signal to the consensus engine to forcibly act as though all parachain
       * blocks in all relay chain blocks up to and including the given number in the current
       * chain are valid and should be finalized.
       **/
      forceApprove: AugmentedSubmittable<(upTo: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    membership: {
      /**
       * Add a member `who` to the set.
       * 
       * May only be called from `T::AddOrigin`.
       **/
      addMember: AugmentedSubmittable<(who: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [AccountId32]>;
      /**
       * Swap out the sending member for some other key `new`.
       * 
       * May only be called from `Signed` origin of a current member.
       * 
       * Prime membership is passed from the origin account to `new`, if extant.
       **/
      changeKey: AugmentedSubmittable<(updated: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [AccountId32]>;
      /**
       * Remove the prime member if it exists.
       * 
       * May only be called from `T::PrimeOrigin`.
       **/
      clearPrime: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
      /**
       * Remove a member `who` from the set.
       * 
       * May only be called from `T::RemoveOrigin`.
       **/
      removeMember: AugmentedSubmittable<(who: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [AccountId32]>;
      /**
       * Change the membership to a new set, disregarding the existing membership. Be nice and
       * pass `members` pre-sorted.
       * 
       * May only be called from `T::ResetOrigin`.
       **/
      resetMembers: AugmentedSubmittable<(members: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [Vec<AccountId32>]>;
      /**
       * Set the prime member. Must be a current member.
       * 
       * May only be called from `T::PrimeOrigin`.
       **/
      setPrime: AugmentedSubmittable<(who: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [AccountId32]>;
      /**
       * Swap out one member `remove` for another `add`.
       * 
       * May only be called from `T::SwapOrigin`.
       * 
       * Prime membership is *not* passed from `remove` to `add`, if extant.
       **/
      swapMember: AugmentedSubmittable<(remove: AccountId32 | string | Uint8Array, add: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [AccountId32, AccountId32]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    multisig: {
      /**
       * Register approval for a dispatch to be made from a deterministic composite account if
       * approved by a total of `threshold - 1` of `other_signatories`.
       * 
       * Payment: `DepositBase` will be reserved if this is the first approval, plus
       * `threshold` times `DepositFactor`. It is returned once this dispatch happens or
       * is cancelled.
       * 
       * The dispatch origin for this call must be _Signed_.
       * 
       * - `threshold`: The total number of approvals for this dispatch before it is executed.
       * - `other_signatories`: The accounts (other than the sender) who can approve this
       * dispatch. May not be empty.
       * - `maybe_timepoint`: If this is the first approval, then this must be `None`. If it is
       * not the first approval, then it must be `Some`, with the timepoint (block number and
       * transaction index) of the first approval transaction.
       * - `call_hash`: The hash of the call to be executed.
       * 
       * NOTE: If this is the final approval, you will want to use `as_multi` instead.
       * 
       * # <weight>
       * - `O(S)`.
       * - Up to one balance-reserve or unreserve operation.
       * - One passthrough operation, one insert, both `O(S)` where `S` is the number of
       * signatories. `S` is capped by `MaxSignatories`, with weight being proportional.
       * - One encode & hash, both of complexity `O(S)`.
       * - Up to one binary search and insert (`O(logS + S)`).
       * - I/O: 1 read `O(S)`, up to 1 mutate `O(S)`. Up to one remove.
       * - One event.
       * - Storage: inserts one item, value size bounded by `MaxSignatories`, with a deposit
       * taken for its lifetime of `DepositBase + threshold * DepositFactor`.
       * ----------------------------------
       * - DB Weight:
       * - Read: Multisig Storage, [Caller Account]
       * - Write: Multisig Storage, [Caller Account]
       * # </weight>
       **/
      approveAsMulti: AugmentedSubmittable<(threshold: u16 | AnyNumber | Uint8Array, otherSignatories: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[], maybeTimepoint: Option<PalletMultisigTimepoint> | null | object | string | Uint8Array, callHash: U8aFixed | string | Uint8Array, maxWeight: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u16, Vec<AccountId32>, Option<PalletMultisigTimepoint>, U8aFixed, u64]>;
      /**
       * Register approval for a dispatch to be made from a deterministic composite account if
       * approved by a total of `threshold - 1` of `other_signatories`.
       * 
       * If there are enough, then dispatch the call.
       * 
       * Payment: `DepositBase` will be reserved if this is the first approval, plus
       * `threshold` times `DepositFactor`. It is returned once this dispatch happens or
       * is cancelled.
       * 
       * The dispatch origin for this call must be _Signed_.
       * 
       * - `threshold`: The total number of approvals for this dispatch before it is executed.
       * - `other_signatories`: The accounts (other than the sender) who can approve this
       * dispatch. May not be empty.
       * - `maybe_timepoint`: If this is the first approval, then this must be `None`. If it is
       * not the first approval, then it must be `Some`, with the timepoint (block number and
       * transaction index) of the first approval transaction.
       * - `call`: The call to be executed.
       * 
       * NOTE: Unless this is the final approval, you will generally want to use
       * `approve_as_multi` instead, since it only requires a hash of the call.
       * 
       * Result is equivalent to the dispatched result if `threshold` is exactly `1`. Otherwise
       * on success, result is `Ok` and the result from the interior call, if it was executed,
       * may be found in the deposited `MultisigExecuted` event.
       * 
       * # <weight>
       * - `O(S + Z + Call)`.
       * - Up to one balance-reserve or unreserve operation.
       * - One passthrough operation, one insert, both `O(S)` where `S` is the number of
       * signatories. `S` is capped by `MaxSignatories`, with weight being proportional.
       * - One call encode & hash, both of complexity `O(Z)` where `Z` is tx-len.
       * - One encode & hash, both of complexity `O(S)`.
       * - Up to one binary search and insert (`O(logS + S)`).
       * - I/O: 1 read `O(S)`, up to 1 mutate `O(S)`. Up to one remove.
       * - One event.
       * - The weight of the `call`.
       * - Storage: inserts one item, value size bounded by `MaxSignatories`, with a deposit
       * taken for its lifetime of `DepositBase + threshold * DepositFactor`.
       * -------------------------------
       * - DB Weight:
       * - Reads: Multisig Storage, [Caller Account], Calls (if `store_call`)
       * - Writes: Multisig Storage, [Caller Account], Calls (if `store_call`)
       * - Plus Call Weight
       * # </weight>
       **/
      asMulti: AugmentedSubmittable<(threshold: u16 | AnyNumber | Uint8Array, otherSignatories: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[], maybeTimepoint: Option<PalletMultisigTimepoint> | null | object | string | Uint8Array, call: Bytes | string | Uint8Array, storeCall: bool | boolean | Uint8Array, maxWeight: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u16, Vec<AccountId32>, Option<PalletMultisigTimepoint>, Bytes, bool, u64]>;
      /**
       * Immediately dispatch a multi-signature call using a single approval from the caller.
       * 
       * The dispatch origin for this call must be _Signed_.
       * 
       * - `other_signatories`: The accounts (other than the sender) who are part of the
       * multi-signature, but do not participate in the approval process.
       * - `call`: The call to be executed.
       * 
       * Result is equivalent to the dispatched result.
       * 
       * # <weight>
       * O(Z + C) where Z is the length of the call and C its execution weight.
       * -------------------------------
       * - DB Weight: None
       * - Plus Call Weight
       * # </weight>
       **/
      asMultiThreshold1: AugmentedSubmittable<(otherSignatories: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[], call: Call | { callIndex?: any; args?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Vec<AccountId32>, Call]>;
      /**
       * Cancel a pre-existing, on-going multisig transaction. Any deposit reserved previously
       * for this operation will be unreserved on success.
       * 
       * The dispatch origin for this call must be _Signed_.
       * 
       * - `threshold`: The total number of approvals for this dispatch before it is executed.
       * - `other_signatories`: The accounts (other than the sender) who can approve this
       * dispatch. May not be empty.
       * - `timepoint`: The timepoint (block number and transaction index) of the first approval
       * transaction for this dispatch.
       * - `call_hash`: The hash of the call to be executed.
       * 
       * # <weight>
       * - `O(S)`.
       * - Up to one balance-reserve or unreserve operation.
       * - One passthrough operation, one insert, both `O(S)` where `S` is the number of
       * signatories. `S` is capped by `MaxSignatories`, with weight being proportional.
       * - One encode & hash, both of complexity `O(S)`.
       * - One event.
       * - I/O: 1 read `O(S)`, one remove.
       * - Storage: removes one item.
       * ----------------------------------
       * - DB Weight:
       * - Read: Multisig Storage, [Caller Account], Refund Account, Calls
       * - Write: Multisig Storage, [Caller Account], Refund Account, Calls
       * # </weight>
       **/
      cancelAsMulti: AugmentedSubmittable<(threshold: u16 | AnyNumber | Uint8Array, otherSignatories: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[], timepoint: PalletMultisigTimepoint | { height?: any; index?: any } | string | Uint8Array, callHash: U8aFixed | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u16, Vec<AccountId32>, PalletMultisigTimepoint, U8aFixed]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    paraInclusion: {
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    paraInherent: {
      /**
       * Enter the paras inherent. This will process bitfields and backed candidates.
       **/
      enter: AugmentedSubmittable<(data: PolkadotPrimitivesV1InherentData | { bitfields?: any; backedCandidates?: any; disputes?: any; parentHeader?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [PolkadotPrimitivesV1InherentData]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    paras: {
      /**
       * Note a new block head for para within the context of the current block.
       **/
      forceNoteNewHead: AugmentedSubmittable<(para: u32 | AnyNumber | Uint8Array, newHead: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, Bytes]>;
      /**
       * Put a parachain directly into the next session's action queue.
       * We can't queue it any sooner than this without going into the
       * initializer...
       **/
      forceQueueAction: AugmentedSubmittable<(para: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Schedule an upgrade as if it was scheduled in the given relay parent block.
       **/
      forceScheduleCodeUpgrade: AugmentedSubmittable<(para: u32 | AnyNumber | Uint8Array, newCode: Bytes | string | Uint8Array, relayParentNumber: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, Bytes, u32]>;
      /**
       * Set the storage for the parachain validation code immediately.
       **/
      forceSetCurrentCode: AugmentedSubmittable<(para: u32 | AnyNumber | Uint8Array, newCode: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, Bytes]>;
      /**
       * Set the storage for the current parachain head data immediately.
       **/
      forceSetCurrentHead: AugmentedSubmittable<(para: u32 | AnyNumber | Uint8Array, newHead: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, Bytes]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    parasDisputes: {
      forceUnfreeze: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    parasShared: {
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    parasSudoWrapper: {
      /**
       * Forcefully establish a channel from the sender to the recipient.
       * 
       * This is equivalent to sending an `Hrmp::hrmp_init_open_channel` extrinsic followed by
       * `Hrmp::hrmp_accept_open_channel`.
       **/
      sudoEstablishHrmpChannel: AugmentedSubmittable<(sender: u32 | AnyNumber | Uint8Array, recipient: u32 | AnyNumber | Uint8Array, maxCapacity: u32 | AnyNumber | Uint8Array, maxMessageSize: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, u32, u32]>;
      /**
       * Send a downward XCM to the given para.
       * 
       * The given parachain should exist and the payload should not exceed the preconfigured size
       * `config.max_downward_message_size`.
       **/
      sudoQueueDownwardXcm: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array, xcm: XcmVersionedXcm | { V0: any } | { V1: any } | { V2: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, XcmVersionedXcm]>;
      /**
       * Downgrade a parachain to a parathread
       **/
      sudoScheduleParachainDowngrade: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Schedule a para to be cleaned up at the start of the next session.
       **/
      sudoScheduleParaCleanup: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Schedule a para to be initialized at the start of the next session.
       **/
      sudoScheduleParaInitialize: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array, genesis: PolkadotRuntimeParachainsParasParaGenesisArgs | { genesisHead?: any; validationCode?: any; parachain?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, PolkadotRuntimeParachainsParasParaGenesisArgs]>;
      /**
       * Upgrade a parathread to a parachain
       **/
      sudoScheduleParathreadUpgrade: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    proxy: {
      /**
       * Register a proxy account for the sender that is able to make calls on its behalf.
       * 
       * The dispatch origin for this call must be _Signed_.
       * 
       * Parameters:
       * - `proxy`: The account that the `caller` would like to make a proxy.
       * - `proxy_type`: The permissions allowed for this proxy account.
       * - `delay`: The announcement period required of the initial proxy. Will generally be
       * zero.
       * 
       * # <weight>
       * Weight is a function of the number of proxies the user has (P).
       * # </weight>
       **/
      addProxy: AugmentedSubmittable<(delegate: AccountId32 | string | Uint8Array, proxyType: RococoRuntimeProxyType | 'Any' | 'CancelProxy' | 'Auction' | number | Uint8Array, delay: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [AccountId32, RococoRuntimeProxyType, u32]>;
      /**
       * Publish the hash of a proxy-call that will be made in the future.
       * 
       * This must be called some number of blocks before the corresponding `proxy` is attempted
       * if the delay associated with the proxy relationship is greater than zero.
       * 
       * No more than `MaxPending` announcements may be made at any one time.
       * 
       * This will take a deposit of `AnnouncementDepositFactor` as well as
       * `AnnouncementDepositBase` if there are no other pending announcements.
       * 
       * The dispatch origin for this call must be _Signed_ and a proxy of `real`.
       * 
       * Parameters:
       * - `real`: The account that the proxy will make a call on behalf of.
       * - `call_hash`: The hash of the call to be made by the `real` account.
       * 
       * # <weight>
       * Weight is a function of:
       * - A: the number of announcements made.
       * - P: the number of proxies the user has.
       * # </weight>
       **/
      announce: AugmentedSubmittable<(real: AccountId32 | string | Uint8Array, callHash: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [AccountId32, H256]>;
      /**
       * Spawn a fresh new account that is guaranteed to be otherwise inaccessible, and
       * initialize it with a proxy of `proxy_type` for `origin` sender.
       * 
       * Requires a `Signed` origin.
       * 
       * - `proxy_type`: The type of the proxy that the sender will be registered as over the
       * new account. This will almost always be the most permissive `ProxyType` possible to
       * allow for maximum flexibility.
       * - `index`: A disambiguation index, in case this is called multiple times in the same
       * transaction (e.g. with `utility::batch`). Unless you're using `batch` you probably just
       * want to use `0`.
       * - `delay`: The announcement period required of the initial proxy. Will generally be
       * zero.
       * 
       * Fails with `Duplicate` if this has already been called in this transaction, from the
       * same sender, with the same parameters.
       * 
       * Fails if there are insufficient funds to pay for deposit.
       * 
       * # <weight>
       * Weight is a function of the number of proxies the user has (P).
       * # </weight>
       * TODO: Might be over counting 1 read
       **/
      anonymous: AugmentedSubmittable<(proxyType: RococoRuntimeProxyType | 'Any' | 'CancelProxy' | 'Auction' | number | Uint8Array, delay: u32 | AnyNumber | Uint8Array, index: u16 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [RococoRuntimeProxyType, u32, u16]>;
      /**
       * Removes a previously spawned anonymous proxy.
       * 
       * WARNING: **All access to this account will be lost.** Any funds held in it will be
       * inaccessible.
       * 
       * Requires a `Signed` origin, and the sender account must have been created by a call to
       * `anonymous` with corresponding parameters.
       * 
       * - `spawner`: The account that originally called `anonymous` to create this account.
       * - `index`: The disambiguation index originally passed to `anonymous`. Probably `0`.
       * - `proxy_type`: The proxy type originally passed to `anonymous`.
       * - `height`: The height of the chain when the call to `anonymous` was processed.
       * - `ext_index`: The extrinsic index in which the call to `anonymous` was processed.
       * 
       * Fails with `NoPermission` in case the caller is not a previously created anonymous
       * account whose `anonymous` call has corresponding parameters.
       * 
       * # <weight>
       * Weight is a function of the number of proxies the user has (P).
       * # </weight>
       **/
      killAnonymous: AugmentedSubmittable<(spawner: AccountId32 | string | Uint8Array, proxyType: RococoRuntimeProxyType | 'Any' | 'CancelProxy' | 'Auction' | number | Uint8Array, index: u16 | AnyNumber | Uint8Array, height: Compact<u32> | AnyNumber | Uint8Array, extIndex: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [AccountId32, RococoRuntimeProxyType, u16, Compact<u32>, Compact<u32>]>;
      /**
       * Dispatch the given `call` from an account that the sender is authorised for through
       * `add_proxy`.
       * 
       * Removes any corresponding announcement(s).
       * 
       * The dispatch origin for this call must be _Signed_.
       * 
       * Parameters:
       * - `real`: The account that the proxy will make a call on behalf of.
       * - `force_proxy_type`: Specify the exact proxy type to be used and checked for this call.
       * - `call`: The call to be made by the `real` account.
       * 
       * # <weight>
       * Weight is a function of the number of proxies the user has (P).
       * # </weight>
       **/
      proxy: AugmentedSubmittable<(real: AccountId32 | string | Uint8Array, forceProxyType: Option<RococoRuntimeProxyType> | null | object | string | Uint8Array, call: Call | { callIndex?: any; args?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [AccountId32, Option<RococoRuntimeProxyType>, Call]>;
      /**
       * Dispatch the given `call` from an account that the sender is authorized for through
       * `add_proxy`.
       * 
       * Removes any corresponding announcement(s).
       * 
       * The dispatch origin for this call must be _Signed_.
       * 
       * Parameters:
       * - `real`: The account that the proxy will make a call on behalf of.
       * - `force_proxy_type`: Specify the exact proxy type to be used and checked for this call.
       * - `call`: The call to be made by the `real` account.
       * 
       * # <weight>
       * Weight is a function of:
       * - A: the number of announcements made.
       * - P: the number of proxies the user has.
       * # </weight>
       **/
      proxyAnnounced: AugmentedSubmittable<(delegate: AccountId32 | string | Uint8Array, real: AccountId32 | string | Uint8Array, forceProxyType: Option<RococoRuntimeProxyType> | null | object | string | Uint8Array, call: Call | { callIndex?: any; args?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [AccountId32, AccountId32, Option<RococoRuntimeProxyType>, Call]>;
      /**
       * Remove the given announcement of a delegate.
       * 
       * May be called by a target (proxied) account to remove a call that one of their delegates
       * (`delegate`) has announced they want to execute. The deposit is returned.
       * 
       * The dispatch origin for this call must be _Signed_.
       * 
       * Parameters:
       * - `delegate`: The account that previously announced the call.
       * - `call_hash`: The hash of the call to be made.
       * 
       * # <weight>
       * Weight is a function of:
       * - A: the number of announcements made.
       * - P: the number of proxies the user has.
       * # </weight>
       **/
      rejectAnnouncement: AugmentedSubmittable<(delegate: AccountId32 | string | Uint8Array, callHash: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [AccountId32, H256]>;
      /**
       * Remove a given announcement.
       * 
       * May be called by a proxy account to remove a call they previously announced and return
       * the deposit.
       * 
       * The dispatch origin for this call must be _Signed_.
       * 
       * Parameters:
       * - `real`: The account that the proxy will make a call on behalf of.
       * - `call_hash`: The hash of the call to be made by the `real` account.
       * 
       * # <weight>
       * Weight is a function of:
       * - A: the number of announcements made.
       * - P: the number of proxies the user has.
       * # </weight>
       **/
      removeAnnouncement: AugmentedSubmittable<(real: AccountId32 | string | Uint8Array, callHash: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [AccountId32, H256]>;
      /**
       * Unregister all proxy accounts for the sender.
       * 
       * The dispatch origin for this call must be _Signed_.
       * 
       * WARNING: This may be called on accounts created by `anonymous`, however if done, then
       * the unreserved fees will be inaccessible. **All access to this account will be lost.**
       * 
       * # <weight>
       * Weight is a function of the number of proxies the user has (P).
       * # </weight>
       **/
      removeProxies: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
      /**
       * Unregister a proxy account for the sender.
       * 
       * The dispatch origin for this call must be _Signed_.
       * 
       * Parameters:
       * - `proxy`: The account that the `caller` would like to remove as a proxy.
       * - `proxy_type`: The permissions currently enabled for the removed proxy account.
       * 
       * # <weight>
       * Weight is a function of the number of proxies the user has (P).
       * # </weight>
       **/
      removeProxy: AugmentedSubmittable<(delegate: AccountId32 | string | Uint8Array, proxyType: RococoRuntimeProxyType | 'Any' | 'CancelProxy' | 'Auction' | number | Uint8Array, delay: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [AccountId32, RococoRuntimeProxyType, u32]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    registrar: {
      /**
       * Deregister a Para Id, freeing all data and returning any deposit.
       * 
       * The caller must be Root, the `para` owner, or the `para` itself. The para must be a parathread.
       **/
      deregister: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Force the registration of a Para Id on the relay chain.
       * 
       * This function must be called by a Root origin.
       * 
       * The deposit taken can be specified for this registration. Any `ParaId`
       * can be registered, including sub-1000 IDs which are System Parachains.
       **/
      forceRegister: AugmentedSubmittable<(who: AccountId32 | string | Uint8Array, deposit: u128 | AnyNumber | Uint8Array, id: u32 | AnyNumber | Uint8Array, genesisHead: Bytes | string | Uint8Array, validationCode: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [AccountId32, u128, u32, Bytes, Bytes]>;
      /**
       * Remove a manager lock from a para. This will allow the manager of a
       * previously locked para to deregister or swap a para without using governance.
       * 
       * Can only be called by the Root origin.
       **/
      forceRemoveLock: AugmentedSubmittable<(para: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Register head data and validation code for a reserved Para Id.
       * 
       * ## Arguments
       * - `origin`: Must be called by a `Signed` origin.
       * - `id`: The para ID. Must be owned/managed by the `origin` signing account.
       * - `genesis_head`: The genesis head data of the parachain/thread.
       * - `validation_code`: The initial validation code of the parachain/thread.
       * 
       * ## Deposits/Fees
       * The origin signed account must reserve a corresponding deposit for the registration. Anything already
       * reserved previously for this para ID is accounted for.
       * 
       * ## Events
       * The `Registered` event is emitted in case of success.
       **/
      register: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array, genesisHead: Bytes | string | Uint8Array, validationCode: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, Bytes, Bytes]>;
      /**
       * Reserve a Para Id on the relay chain.
       * 
       * This function will reserve a new Para Id to be owned/managed by the origin account.
       * The origin account is able to register head data and validation code using `register` to create
       * a parathread. Using the Slots pallet, a parathread can then be upgraded to get a parachain slot.
       * 
       * ## Arguments
       * - `origin`: Must be called by a `Signed` origin. Becomes the manager/owner of the new para ID.
       * 
       * ## Deposits/Fees
       * The origin must reserve a deposit of `ParaDeposit` for the registration.
       * 
       * ## Events
       * The `Reserved` event is emitted in case of success, which provides the ID reserved for use.
       **/
      reserve: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
      /**
       * Swap a parachain with another parachain or parathread.
       * 
       * The origin must be Root, the `para` owner, or the `para` itself.
       * 
       * The swap will happen only if there is already an opposite swap pending. If there is not,
       * the swap will be stored in the pending swaps map, ready for a later confirmatory swap.
       * 
       * The `ParaId`s remain mapped to the same head data and code so external code can rely on
       * `ParaId` to be a long-term identifier of a notional "parachain". However, their
       * scheduling info (i.e. whether they're a parathread or parachain), auction information
       * and the auction deposit are switched.
       **/
      swap: AugmentedSubmittable<(id: u32 | AnyNumber | Uint8Array, other: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    session: {
      /**
       * Removes any session key(s) of the function caller.
       * 
       * This doesn't take effect until the next session.
       * 
       * The dispatch origin of this function must be Signed and the account must be either be
       * convertible to a validator ID using the chain's typical addressing system (this usually
       * means being a controller account) or directly convertible into a validator ID (which
       * usually means being a stash account).
       * 
       * # <weight>
       * - Complexity: `O(1)` in number of key types. Actual cost depends on the number of length
       * of `T::Keys::key_ids()` which is fixed.
       * - DbReads: `T::ValidatorIdOf`, `NextKeys`, `origin account`
       * - DbWrites: `NextKeys`, `origin account`
       * - DbWrites per key id: `KeyOwner`
       * # </weight>
       **/
      purgeKeys: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
      /**
       * Sets the session key(s) of the function caller to `keys`.
       * Allows an account to set its session key prior to becoming a validator.
       * This doesn't take effect until the next session.
       * 
       * The dispatch origin of this function must be signed.
       * 
       * # <weight>
       * - Complexity: `O(1)`. Actual cost depends on the number of length of
       * `T::Keys::key_ids()` which is fixed.
       * - DbReads: `origin account`, `T::ValidatorIdOf`, `NextKeys`
       * - DbWrites: `origin account`, `NextKeys`
       * - DbReads per key id: `KeyOwner`
       * - DbWrites per key id: `KeyOwner`
       * # </weight>
       **/
      setKeys: AugmentedSubmittable<(keys: RococoRuntimeSessionKeys | { grandpa?: any; babe?: any; imOnline?: any; paraValidator?: any; paraAssignment?: any; authorityDiscovery?: any; beefy?: any } | string | Uint8Array, proof: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [RococoRuntimeSessionKeys, Bytes]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    slots: {
      /**
       * Clear all leases for a Para Id, refunding any deposits back to the original owners.
       * 
       * Can only be called by the Root origin.
       **/
      clearAllLeases: AugmentedSubmittable<(para: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Just a connect into the `lease_out` call, in case Root wants to force some lease to happen
       * independently of any other on-chain mechanism to use it.
       * 
       * Can only be called by the Root origin.
       **/
      forceLease: AugmentedSubmittable<(para: u32 | AnyNumber | Uint8Array, leaser: AccountId32 | string | Uint8Array, amount: u128 | AnyNumber | Uint8Array, periodBegin: u32 | AnyNumber | Uint8Array, periodCount: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, AccountId32, u128, u32, u32]>;
      /**
       * Try to onboard a parachain that has a lease for the current lease period.
       * 
       * This function can be useful if there was some state issue with a para that should
       * have onboarded, but was unable to. As long as they have a lease period, we can
       * let them onboard from here.
       * 
       * Origin must be signed, but can be called by anyone.
       **/
      triggerOnboard: AugmentedSubmittable<(para: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    sudo: {
      /**
       * Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo
       * key.
       * 
       * The dispatch origin for this call must be _Signed_.
       * 
       * # <weight>
       * - O(1).
       * - Limited storage reads.
       * - One DB change.
       * # </weight>
       **/
      setKey: AugmentedSubmittable<(updated: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress]>;
      /**
       * Authenticates the sudo key and dispatches a function call with `Root` origin.
       * 
       * The dispatch origin for this call must be _Signed_.
       * 
       * # <weight>
       * - O(1).
       * - Limited storage reads.
       * - One DB write (event).
       * - Weight of derivative `call` execution + 10,000.
       * # </weight>
       **/
      sudo: AugmentedSubmittable<(call: Call | { callIndex?: any; args?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Call]>;
      /**
       * Authenticates the sudo key and dispatches a function call with `Signed` origin from
       * a given account.
       * 
       * The dispatch origin for this call must be _Signed_.
       * 
       * # <weight>
       * - O(1).
       * - Limited storage reads.
       * - One DB write (event).
       * - Weight of derivative `call` execution + 10,000.
       * # </weight>
       **/
      sudoAs: AugmentedSubmittable<(who: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, call: Call | { callIndex?: any; args?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, Call]>;
      /**
       * Authenticates the sudo key and dispatches a function call with `Root` origin.
       * This function does not check the weight of the call, and instead allows the
       * Sudo user to specify the weight of the call.
       * 
       * The dispatch origin for this call must be _Signed_.
       * 
       * # <weight>
       * - O(1).
       * - The weight of this call is defined by the caller.
       * # </weight>
       **/
      sudoUncheckedWeight: AugmentedSubmittable<(call: Call | { callIndex?: any; args?: any } | string | Uint8Array, weight: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Call, u64]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    system: {
      /**
       * A dispatch that will fill the block weight up to the given ratio.
       **/
      fillBlock: AugmentedSubmittable<(ratio: Perbill | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Perbill]>;
      /**
       * Kill all storage items with a key that starts with the given prefix.
       * 
       * **NOTE:** We rely on the Root origin to provide us the number of subkeys under
       * the prefix we are removing to accurately calculate the weight of this function.
       **/
      killPrefix: AugmentedSubmittable<(prefix: Bytes | string | Uint8Array, subkeys: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes, u32]>;
      /**
       * Kill some items from storage.
       **/
      killStorage: AugmentedSubmittable<(keys: Vec<Bytes> | (Bytes | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [Vec<Bytes>]>;
      /**
       * Make some on-chain remark.
       * 
       * # <weight>
       * - `O(1)`
       * # </weight>
       **/
      remark: AugmentedSubmittable<(remark: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes]>;
      /**
       * Make some on-chain remark and emit event.
       * 
       * # <weight>
       * - `O(b)` where b is the length of the remark.
       * - 1 event.
       * # </weight>
       **/
      remarkWithEvent: AugmentedSubmittable<(remark: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes]>;
      /**
       * Set the new runtime code.
       * 
       * # <weight>
       * - `O(C + S)` where `C` length of `code` and `S` complexity of `can_set_code`
       * - 1 call to `can_set_code`: `O(S)` (calls `sp_io::misc::runtime_version` which is
       * expensive).
       * - 1 storage write (codec `O(C)`).
       * - 1 digest item.
       * - 1 event.
       * The weight of this function is dependent on the runtime, but generally this is very
       * expensive. We will treat this as a full block.
       * # </weight>
       **/
      setCode: AugmentedSubmittable<(code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes]>;
      /**
       * Set the new runtime code without doing any checks of the given `code`.
       * 
       * # <weight>
       * - `O(C)` where `C` length of `code`
       * - 1 storage write (codec `O(C)`).
       * - 1 digest item.
       * - 1 event.
       * The weight of this function is dependent on the runtime. We will treat this as a full
       * block. # </weight>
       **/
      setCodeWithoutChecks: AugmentedSubmittable<(code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes]>;
      /**
       * Set the number of pages in the WebAssembly environment's heap.
       **/
      setHeapPages: AugmentedSubmittable<(pages: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u64]>;
      /**
       * Set some items of storage.
       **/
      setStorage: AugmentedSubmittable<(items: Vec<ITuple<[Bytes, Bytes]>> | ([Bytes | string | Uint8Array, Bytes | string | Uint8Array])[]) => SubmittableExtrinsic<ApiType>, [Vec<ITuple<[Bytes, Bytes]>>]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    timestamp: {
      /**
       * Set the current time.
       * 
       * This call should be invoked exactly once per block. It will panic at the finalization
       * phase, if this call hasn't been invoked by that time.
       * 
       * The timestamp should be greater than the previous one by the amount specified by
       * `MinimumPeriod`.
       * 
       * The dispatch origin for this call must be `Inherent`.
       * 
       * # <weight>
       * - `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)
       * - 1 storage read and 1 storage mutation (codec `O(1)`). (because of `DidUpdate::take` in
       * `on_finalize`)
       * - 1 event handler `on_timestamp_set`. Must be `O(1)`.
       * # </weight>
       **/
      set: AugmentedSubmittable<(now: Compact<u64> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u64>]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    ump: {
      /**
       * Service a single overweight upward message.
       * 
       * - `origin`: Must pass `ExecuteOverweightOrigin`.
       * - `index`: The index of the overweight message to service.
       * - `weight_limit`: The amount of weight that message execution may take.
       * 
       * Errors:
       * - `UnknownMessageIndex`: Message of `index` is unknown.
       * - `WeightOverLimit`: Message execution may use greater than `weight_limit`.
       * 
       * Events:
       * - `OverweightServiced`: On success.
       **/
      serviceOverweight: AugmentedSubmittable<(index: u64 | AnyNumber | Uint8Array, weightLimit: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u64, u64]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    utility: {
      /**
       * Send a call through an indexed pseudonym of the sender.
       * 
       * Filter from origin are passed along. The call will be dispatched with an origin which
       * use the same filter as the origin of this call.
       * 
       * NOTE: If you need to ensure that any account-based filtering is not honored (i.e.
       * because you expect `proxy` to have been used prior in the call stack and you do not want
       * the call restrictions to apply to any sub-accounts), then use `as_multi_threshold_1`
       * in the Multisig pallet instead.
       * 
       * NOTE: Prior to version *12, this was called `as_limited_sub`.
       * 
       * The dispatch origin for this call must be _Signed_.
       **/
      asDerivative: AugmentedSubmittable<(index: u16 | AnyNumber | Uint8Array, call: Call | { callIndex?: any; args?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u16, Call]>;
      /**
       * Send a batch of dispatch calls.
       * 
       * May be called from any origin.
       * 
       * - `calls`: The calls to be dispatched from the same origin. The number of call must not
       * exceed the constant: `batched_calls_limit` (available in constant metadata).
       * 
       * If origin is root then call are dispatch without checking origin filter. (This includes
       * bypassing `frame_system::Config::BaseCallFilter`).
       * 
       * # <weight>
       * - Complexity: O(C) where C is the number of calls to be batched.
       * # </weight>
       * 
       * This will return `Ok` in all circumstances. To determine the success of the batch, an
       * event is deposited. If a call failed and the batch was interrupted, then the
       * `BatchInterrupted` event is deposited, along with the number of successful calls made
       * and the error of the failed call. If all were successful, then the `BatchCompleted`
       * event is deposited.
       **/
      batch: AugmentedSubmittable<(calls: Vec<Call> | (Call | { callIndex?: any; args?: any } | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [Vec<Call>]>;
      /**
       * Send a batch of dispatch calls and atomically execute them.
       * The whole transaction will rollback and fail if any of the calls failed.
       * 
       * May be called from any origin.
       * 
       * - `calls`: The calls to be dispatched from the same origin. The number of call must not
       * exceed the constant: `batched_calls_limit` (available in constant metadata).
       * 
       * If origin is root then call are dispatch without checking origin filter. (This includes
       * bypassing `frame_system::Config::BaseCallFilter`).
       * 
       * # <weight>
       * - Complexity: O(C) where C is the number of calls to be batched.
       * # </weight>
       **/
      batchAll: AugmentedSubmittable<(calls: Vec<Call> | (Call | { callIndex?: any; args?: any } | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [Vec<Call>]>;
      /**
       * Dispatches a function call with a provided origin.
       * 
       * The dispatch origin for this call must be _Root_.
       * 
       * # <weight>
       * - O(1).
       * - Limited storage reads.
       * - One DB write (event).
       * - Weight of derivative `call` execution + T::WeightInfo::dispatch_as().
       * # </weight>
       **/
      dispatchAs: AugmentedSubmittable<(asOrigin: RococoRuntimeOriginCaller | { system: any } | { Void: any } | { ParachainsOrigin: any } | { Collective: any } | { XcmPallet: any } | string | Uint8Array, call: Call | { callIndex?: any; args?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [RococoRuntimeOriginCaller, Call]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    validatorManager: {
      /**
       * Remove validators from the set.
       * 
       * The removed validators will be deactivated from current session + 2.
       **/
      deregisterValidators: AugmentedSubmittable<(validators: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [Vec<AccountId32>]>;
      /**
       * Add new validators to the set.
       * 
       * The new validators will be active from current session + 2.
       **/
      registerValidators: AugmentedSubmittable<(validators: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [Vec<AccountId32>]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    xcmPallet: {
      /**
       * Execute an XCM message from a local, signed, origin.
       * 
       * An event is deposited indicating whether `msg` could be executed completely or only
       * partially.
       * 
       * No more than `max_weight` will be used in its attempted execution. If this is less than the
       * maximum amount of weight that the message could take to be executed, then no execution
       * attempt will be made.
       * 
       * NOTE: A successful return to this does *not* imply that the `msg` was executed successfully
       * to completion; only that *some* of it was executed.
       **/
      execute: AugmentedSubmittable<(message: XcmVersionedXcm | { V0: any } | { V1: any } | { V2: any } | string | Uint8Array, maxWeight: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [XcmVersionedXcm, u64]>;
      /**
       * Set a safe XCM version (the version that XCM should be encoded with if the most recent
       * version a destination can accept is unknown).
       * 
       * - `origin`: Must be Root.
       * - `maybe_xcm_version`: The default XCM encoding version, or `None` to disable.
       **/
      forceDefaultXcmVersion: AugmentedSubmittable<(maybeXcmVersion: Option<u32> | null | object | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Option<u32>]>;
      /**
       * Ask a location to notify us regarding their XCM version and any changes to it.
       * 
       * - `origin`: Must be Root.
       * - `location`: The location to which we should subscribe for XCM version notifications.
       **/
      forceSubscribeVersionNotify: AugmentedSubmittable<(location: XcmVersionedMultiLocation | { V0: any } | { V1: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [XcmVersionedMultiLocation]>;
      /**
       * Require that a particular destination should no longer notify us regarding any XCM
       * version changes.
       * 
       * - `origin`: Must be Root.
       * - `location`: The location to which we are currently subscribed for XCM version
       * notifications which we no longer desire.
       **/
      forceUnsubscribeVersionNotify: AugmentedSubmittable<(location: XcmVersionedMultiLocation | { V0: any } | { V1: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [XcmVersionedMultiLocation]>;
      /**
       * Extoll that a particular destination can be communicated with through a particular
       * version of XCM.
       * 
       * - `origin`: Must be Root.
       * - `location`: The destination that is being described.
       * - `xcm_version`: The latest version of XCM that `location` supports.
       **/
      forceXcmVersion: AugmentedSubmittable<(location: XcmV1MultiLocation | { parents?: any; interior?: any } | string | Uint8Array, xcmVersion: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [XcmV1MultiLocation, u32]>;
      /**
       * Transfer some assets from the local chain to the sovereign account of a destination chain and forward
       * a notification XCM.
       * 
       * Fee payment on the destination side is made from the first asset listed in the `assets` vector.
       * 
       * - `origin`: Must be capable of withdrawing the `assets` and executing XCM.
       * - `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send
       * from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain.
       * - `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be
       * an `AccountId32` value.
       * - `assets`: The assets to be withdrawn. This should include the assets used to pay the fee on the
       * `dest` side.
       * - `fee_asset_item`: The index into `assets` of the item which should be used to pay
       * fees.
       * - `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase.
       **/
      limitedReserveTransferAssets: AugmentedSubmittable<(dest: XcmVersionedMultiLocation | { V0: any } | { V1: any } | string | Uint8Array, beneficiary: XcmVersionedMultiLocation | { V0: any } | { V1: any } | string | Uint8Array, assets: XcmVersionedMultiAssets | { V0: any } | { V1: any } | string | Uint8Array, feeAssetItem: u32 | AnyNumber | Uint8Array, weightLimit: XcmV2WeightLimit | { Unlimited: any } | { Limited: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [XcmVersionedMultiLocation, XcmVersionedMultiLocation, XcmVersionedMultiAssets, u32, XcmV2WeightLimit]>;
      /**
       * Teleport some assets from the local chain to some destination chain.
       * 
       * Fee payment on the destination side is made from the first asset listed in the `assets` vector.
       * 
       * - `origin`: Must be capable of withdrawing the `assets` and executing XCM.
       * - `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send
       * from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain.
       * - `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be
       * an `AccountId32` value.
       * - `assets`: The assets to be withdrawn. The first item should be the currency used to to pay the fee on the
       * `dest` side. May not be empty.
       * - `dest_weight`: Equal to the total weight on `dest` of the XCM message
       * `Teleport { assets, effects: [ BuyExecution{..}, DepositAsset{..} ] }`.
       * - `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase.
       **/
      limitedTeleportAssets: AugmentedSubmittable<(dest: XcmVersionedMultiLocation | { V0: any } | { V1: any } | string | Uint8Array, beneficiary: XcmVersionedMultiLocation | { V0: any } | { V1: any } | string | Uint8Array, assets: XcmVersionedMultiAssets | { V0: any } | { V1: any } | string | Uint8Array, feeAssetItem: u32 | AnyNumber | Uint8Array, weightLimit: XcmV2WeightLimit | { Unlimited: any } | { Limited: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [XcmVersionedMultiLocation, XcmVersionedMultiLocation, XcmVersionedMultiAssets, u32, XcmV2WeightLimit]>;
      /**
       * Transfer some assets from the local chain to the sovereign account of a destination chain and forward
       * a notification XCM.
       * 
       * Fee payment on the destination side is made from the first asset listed in the `assets` vector and
       * fee-weight is calculated locally and thus remote weights are assumed to be equal to
       * local weights.
       * 
       * - `origin`: Must be capable of withdrawing the `assets` and executing XCM.
       * - `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send
       * from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain.
       * - `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be
       * an `AccountId32` value.
       * - `assets`: The assets to be withdrawn. This should include the assets used to pay the fee on the
       * `dest` side.
       * - `fee_asset_item`: The index into `assets` of the item which should be used to pay
       * fees.
       **/
      reserveTransferAssets: AugmentedSubmittable<(dest: XcmVersionedMultiLocation | { V0: any } | { V1: any } | string | Uint8Array, beneficiary: XcmVersionedMultiLocation | { V0: any } | { V1: any } | string | Uint8Array, assets: XcmVersionedMultiAssets | { V0: any } | { V1: any } | string | Uint8Array, feeAssetItem: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [XcmVersionedMultiLocation, XcmVersionedMultiLocation, XcmVersionedMultiAssets, u32]>;
      send: AugmentedSubmittable<(dest: XcmVersionedMultiLocation | { V0: any } | { V1: any } | string | Uint8Array, message: XcmVersionedXcm | { V0: any } | { V1: any } | { V2: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [XcmVersionedMultiLocation, XcmVersionedXcm]>;
      /**
       * Teleport some assets from the local chain to some destination chain.
       * 
       * Fee payment on the destination side is made from the first asset listed in the `assets` vector and
       * fee-weight is calculated locally and thus remote weights are assumed to be equal to
       * local weights.
       * 
       * - `origin`: Must be capable of withdrawing the `assets` and executing XCM.
       * - `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send
       * from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain.
       * - `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be
       * an `AccountId32` value.
       * - `assets`: The assets to be withdrawn. The first item should be the currency used to to pay the fee on the
       * `dest` side. May not be empty.
       * - `dest_weight`: Equal to the total weight on `dest` of the XCM message
       * `Teleport { assets, effects: [ BuyExecution{..}, DepositAsset{..} ] }`.
       **/
      teleportAssets: AugmentedSubmittable<(dest: XcmVersionedMultiLocation | { V0: any } | { V1: any } | string | Uint8Array, beneficiary: XcmVersionedMultiLocation | { V0: any } | { V1: any } | string | Uint8Array, assets: XcmVersionedMultiAssets | { V0: any } | { V1: any } | string | Uint8Array, feeAssetItem: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [XcmVersionedMultiLocation, XcmVersionedMultiLocation, XcmVersionedMultiAssets, u32]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
  }

  export interface SubmittableExtrinsics<ApiType extends ApiTypes> extends AugmentedSubmittables<ApiType> {
    (extrinsic: Call | Extrinsic | Uint8Array | string): SubmittableExtrinsic<ApiType>;
    [key: string]: SubmittableModuleExtrinsics<ApiType>;
  }
}
