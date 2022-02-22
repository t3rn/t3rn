// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

import type { ApiTypes } from '@polkadot/api/types';
import type { Bytes, Null, Option, U8aFixed, Vec, WrapperOpaque, bool, u128, u32, u64 } from '@polkadot/types';
import type { AccountId32, Call, H256 } from '@polkadot/types/interfaces/runtime';
import type { FrameSupportWeightsPerDispatchClassU64, FrameSystemAccountInfo, FrameSystemEventRecord, FrameSystemLastRuntimeUpgradeInfo, FrameSystemPhase, PalletAuthorshipUncleEntryItem, PalletBalancesAccountData, PalletBalancesBalanceLock, PalletBalancesReleases, PalletBalancesReserveData, PalletCollectiveVotes, PalletGrandpaStoredPendingChange, PalletGrandpaStoredState, PalletImOnlineBoundedOpaqueNetworkState, PalletImOnlineSr25519AppSr25519Public, PalletMultisigMultisig, PalletProxyAnnouncement, PalletProxyProxyDefinition, PalletTransactionPaymentReleases, PalletXcmQueryStatus, PalletXcmVersionMigrationStage, PolkadotCorePrimitivesInboundDownwardMessage, PolkadotCorePrimitivesInboundHrmpMessage, PolkadotParachainPrimitivesHrmpChannelId, PolkadotPrimitivesV0ValidatorAppPublic, PolkadotPrimitivesV1AssignmentAppPublic, PolkadotPrimitivesV1CandidateCommitments, PolkadotPrimitivesV1CoreOccupied, PolkadotPrimitivesV1ScrapedOnChainVotes, PolkadotPrimitivesV1SessionInfo, PolkadotPrimitivesV1UpgradeGoAhead, PolkadotPrimitivesV1UpgradeRestriction, PolkadotRuntimeCommonCrowdloanFundInfo, PolkadotRuntimeCommonParasRegistrarParaInfo, PolkadotRuntimeParachainsConfigurationHostConfiguration, PolkadotRuntimeParachainsHrmpHrmpChannel, PolkadotRuntimeParachainsHrmpHrmpOpenChannelRequest, PolkadotRuntimeParachainsInclusionAvailabilityBitfieldRecord, PolkadotRuntimeParachainsInclusionCandidatePendingAvailability, PolkadotRuntimeParachainsInitializerBufferedSessionChange, PolkadotRuntimeParachainsParasParaGenesisArgs, PolkadotRuntimeParachainsParasParaLifecycle, PolkadotRuntimeParachainsParasParaPastCodeMeta, PolkadotRuntimeParachainsSchedulerCoreAssignment, PolkadotRuntimeParachainsSchedulerParathreadClaimQueue, SpConsensusBabeAppPublic, SpConsensusBabeBabeEpochConfiguration, SpConsensusBabeDigestsNextConfigDescriptor, SpCoreCryptoKeyTypeId, SpRuntimeGenericDigest, SpStakingOffenceOffenceDetails, XcmVersionedMultiLocation } from '@polkadot/types/lookup';
import type { AnyNumber, ITuple, Observable } from '@polkadot/types/types';

declare module '@polkadot/api/types/storage' {
  export interface AugmentedQueries<ApiType> {
    auctions: {
      /**
       * Number of auctions started so far.
       **/
      auctionCounter: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Information relating to the current auction, if there is one.
       * 
       * The first item in the tuple is the lease period index that the first of the four
       * contiguous lease periods on auction is for. The second is the block number when the
       * auction will "begin to end", i.e. the first block of the Ending Period of the auction.
       **/
      auctionInfo: AugmentedQuery<ApiType, () => Observable<Option<ITuple<[u32, u32]>>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Amounts currently reserved in the accounts of the bidders currently winning
       * (sub-)ranges.
       **/
      reservedAmounts: AugmentedQuery<ApiType, (arg: ITuple<[AccountId32, u32]> | [AccountId32 | string | Uint8Array, u32 | AnyNumber | Uint8Array]) => Observable<Option<u128>>, [ITuple<[AccountId32, u32]>]> & QueryableStorageEntry<ApiType, [ITuple<[AccountId32, u32]>]>;
      /**
       * The winning bids for each of the 10 ranges at each sample in the final Ending Period of
       * the current auction. The map's key is the 0-based index into the Sample Size. The
       * first sample of the ending period is 0; the last is `Sample Size - 1`.
       **/
      winning: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<Vec<Option<ITuple<[AccountId32, u32, u128]>>>>>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    authorship: {
      /**
       * Author of current block.
       **/
      author: AugmentedQuery<ApiType, () => Observable<Option<AccountId32>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Whether uncles were already set in this block.
       **/
      didSetUncles: AugmentedQuery<ApiType, () => Observable<bool>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Uncles
       **/
      uncles: AugmentedQuery<ApiType, () => Observable<Vec<PalletAuthorshipUncleEntryItem>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    babe: {
      /**
       * Current epoch authorities.
       **/
      authorities: AugmentedQuery<ApiType, () => Observable<Vec<ITuple<[SpConsensusBabeAppPublic, u64]>>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * This field should always be populated during block processing unless
       * secondary plain slots are enabled (which don't contain a VRF output).
       * 
       * It is set in `on_initialize`, before it will contain the value from the last block.
       **/
      authorVrfRandomness: AugmentedQuery<ApiType, () => Observable<Option<U8aFixed>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Current slot number.
       **/
      currentSlot: AugmentedQuery<ApiType, () => Observable<u64>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * The configuration for the current epoch. Should never be `None` as it is initialized in
       * genesis.
       **/
      epochConfig: AugmentedQuery<ApiType, () => Observable<Option<SpConsensusBabeBabeEpochConfiguration>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Current epoch index.
       **/
      epochIndex: AugmentedQuery<ApiType, () => Observable<u64>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * The block numbers when the last and current epoch have started, respectively `N-1` and
       * `N`.
       * NOTE: We track this is in order to annotate the block number when a given pool of
       * entropy was fixed (i.e. it was known to chain observers). Since epochs are defined in
       * slots, which may be skipped, the block numbers may not line up with the slot numbers.
       **/
      epochStart: AugmentedQuery<ApiType, () => Observable<ITuple<[u32, u32]>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * The slot at which the first epoch actually started. This is 0
       * until the first block of the chain.
       **/
      genesisSlot: AugmentedQuery<ApiType, () => Observable<u64>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Temporary value (cleared at block finalization) which is `Some`
       * if per-block initialization has already been called for current block.
       **/
      initialized: AugmentedQuery<ApiType, () => Observable<Option<Option<U8aFixed>>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * How late the current block is compared to its parent.
       * 
       * This entry is populated as part of block execution and is cleaned up
       * on block finalization. Querying this storage entry outside of block
       * execution context should always yield zero.
       **/
      lateness: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Next epoch authorities.
       **/
      nextAuthorities: AugmentedQuery<ApiType, () => Observable<Vec<ITuple<[SpConsensusBabeAppPublic, u64]>>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * The configuration for the next epoch, `None` if the config will not change
       * (you can fallback to `EpochConfig` instead in that case).
       **/
      nextEpochConfig: AugmentedQuery<ApiType, () => Observable<Option<SpConsensusBabeBabeEpochConfiguration>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Next epoch randomness.
       **/
      nextRandomness: AugmentedQuery<ApiType, () => Observable<U8aFixed>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Pending epoch configuration change that will be applied when the next epoch is enacted.
       **/
      pendingEpochConfigChange: AugmentedQuery<ApiType, () => Observable<Option<SpConsensusBabeDigestsNextConfigDescriptor>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * The epoch randomness for the *current* epoch.
       * 
       * # Security
       * 
       * This MUST NOT be used for gambling, as it can be influenced by a
       * malicious validator in the short term. It MAY be used in many
       * cryptographic protocols, however, so long as one remembers that this
       * (like everything else on-chain) it is public. For example, it can be
       * used where a number is needed that cannot have been chosen by an
       * adversary, for purposes such as public-coin zero-knowledge proofs.
       **/
      randomness: AugmentedQuery<ApiType, () => Observable<U8aFixed>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Randomness under construction.
       * 
       * We make a trade-off between storage accesses and list length.
       * We store the under-construction randomness in segments of up to
       * `UNDER_CONSTRUCTION_SEGMENT_LENGTH`.
       * 
       * Once a segment reaches this length, we begin the next one.
       * We reset all segments and return to `0` at the beginning of every
       * epoch.
       **/
      segmentIndex: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * TWOX-NOTE: `SegmentIndex` is an increasing integer, so this is okay.
       **/
      underConstruction: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Vec<U8aFixed>>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    balances: {
      /**
       * The balance of an account.
       * 
       * NOTE: This is only used in the case that this pallet is used to store balances.
       **/
      account: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<PalletBalancesAccountData>, [AccountId32]> & QueryableStorageEntry<ApiType, [AccountId32]>;
      /**
       * Any liquidity locks on some account balances.
       * NOTE: Should only be accessed when setting, changing and freeing a lock.
       **/
      locks: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<Vec<PalletBalancesBalanceLock>>, [AccountId32]> & QueryableStorageEntry<ApiType, [AccountId32]>;
      /**
       * Named reserves on some account balances.
       **/
      reserves: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<Vec<PalletBalancesReserveData>>, [AccountId32]> & QueryableStorageEntry<ApiType, [AccountId32]>;
      /**
       * Storage version of the pallet.
       * 
       * This is set to v2.0.0 for new networks.
       **/
      storageVersion: AugmentedQuery<ApiType, () => Observable<PalletBalancesReleases>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * The total units issued in the system.
       **/
      totalIssuance: AugmentedQuery<ApiType, () => Observable<u128>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    beefy: {
      /**
       * The current authorities set
       **/
      authorities: AugmentedQuery<ApiType, () => Observable<Vec<BeefyPrimitivesCryptoPublic>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Authorities set scheduled to be used with the next session
       **/
      nextAuthorities: AugmentedQuery<ApiType, () => Observable<Vec<BeefyPrimitivesCryptoPublic>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * The current validator set id
       **/
      validatorSetId: AugmentedQuery<ApiType, () => Observable<u64>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    collective: {
      /**
       * The current members of the collective. This is stored sorted (just by value).
       **/
      members: AugmentedQuery<ApiType, () => Observable<Vec<AccountId32>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * The prime member that helps determine the default vote behavior in case of absentations.
       **/
      prime: AugmentedQuery<ApiType, () => Observable<Option<AccountId32>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Proposals so far.
       **/
      proposalCount: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Actual proposal for a given hash, if it's current.
       **/
      proposalOf: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Option<Call>>, [H256]> & QueryableStorageEntry<ApiType, [H256]>;
      /**
       * The hashes of the active proposals.
       **/
      proposals: AugmentedQuery<ApiType, () => Observable<Vec<H256>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Votes on a given proposal, if it is ongoing.
       **/
      voting: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Option<PalletCollectiveVotes>>, [H256]> & QueryableStorageEntry<ApiType, [H256]>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    configuration: {
      /**
       * The active configuration for the current session.
       **/
      activeConfig: AugmentedQuery<ApiType, () => Observable<PolkadotRuntimeParachainsConfigurationHostConfiguration>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Pending configuration (if any) for the next session.
       **/
      pendingConfig: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<PolkadotRuntimeParachainsConfigurationHostConfiguration>>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    crowdloan: {
      /**
       * The number of auctions that have entered into their ending period so far.
       **/
      endingsCount: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Info on all of the funds.
       **/
      funds: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<PolkadotRuntimeCommonCrowdloanFundInfo>>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * The funds that have had additional contributions during the last block. This is used
       * in order to determine which funds should submit new or updated bids.
       **/
      newRaise: AugmentedQuery<ApiType, () => Observable<Vec<u32>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Tracker for the next available trie index
       **/
      nextTrieIndex: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    dmp: {
      /**
       * A mapping that stores the downward message queue MQC head for each para.
       * 
       * Each link in this chain has a form:
       * `(prev_head, B, H(M))`, where
       * - `prev_head`: is the previous head hash or zero if none.
       * - `B`: is the relay-chain block number in which a message was appended.
       * - `H(M)`: is the hash of the message being appended.
       **/
      downwardMessageQueueHeads: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<H256>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * The downward messages addressed for a certain para.
       **/
      downwardMessageQueues: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Vec<PolkadotCorePrimitivesInboundDownwardMessage>>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    grandpa: {
      /**
       * The number of changes (both in terms of keys and underlying economic responsibilities)
       * in the "set" of Grandpa validators from genesis.
       **/
      currentSetId: AugmentedQuery<ApiType, () => Observable<u64>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * next block number where we can force a change.
       **/
      nextForced: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Pending change: (signaled at, scheduled change).
       **/
      pendingChange: AugmentedQuery<ApiType, () => Observable<Option<PalletGrandpaStoredPendingChange>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * A mapping from grandpa set ID to the index of the *most recent* session for which its
       * members were responsible.
       * 
       * TWOX-NOTE: `SetId` is not under user control.
       **/
      setIdSession: AugmentedQuery<ApiType, (arg: u64 | AnyNumber | Uint8Array) => Observable<Option<u32>>, [u64]> & QueryableStorageEntry<ApiType, [u64]>;
      /**
       * `true` if we are currently stalled.
       **/
      stalled: AugmentedQuery<ApiType, () => Observable<Option<ITuple<[u32, u32]>>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * State of the current authority set.
       **/
      state: AugmentedQuery<ApiType, () => Observable<PalletGrandpaStoredState>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    hrmp: {
      /**
       * This mapping tracks how many open channel requests were accepted by a given recipient para.
       * Invariant: `HrmpOpenChannelRequests` should contain the same number of items `(_, X)` with
       * `confirmed` set to true, as the number of `HrmpAcceptedChannelRequestCount` for `X`.
       **/
      hrmpAcceptedChannelRequestCount: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<u32>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * Storage for the messages for each channel.
       * Invariant: cannot be non-empty if the corresponding channel in `HrmpChannels` is `None`.
       **/
      hrmpChannelContents: AugmentedQuery<ApiType, (arg: PolkadotParachainPrimitivesHrmpChannelId | { sender?: any; recipient?: any } | string | Uint8Array) => Observable<Vec<PolkadotCorePrimitivesInboundHrmpMessage>>, [PolkadotParachainPrimitivesHrmpChannelId]> & QueryableStorageEntry<ApiType, [PolkadotParachainPrimitivesHrmpChannelId]>;
      /**
       * Maintains a mapping that can be used to answer the question:
       * What paras sent a message at the given block number for a given receiver.
       * Invariants:
       * - The inner `Vec<ParaId>` is never empty.
       * - The inner `Vec<ParaId>` cannot store two same `ParaId`.
       * - The outer vector is sorted ascending by block number and cannot store two items with the same
       * block number.
       **/
      hrmpChannelDigests: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Vec<ITuple<[u32, Vec<u32>]>>>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * HRMP channel data associated with each para.
       * Invariant:
       * - each participant in the channel should satisfy `Paras::is_valid_para(P)` within a session.
       **/
      hrmpChannels: AugmentedQuery<ApiType, (arg: PolkadotParachainPrimitivesHrmpChannelId | { sender?: any; recipient?: any } | string | Uint8Array) => Observable<Option<PolkadotRuntimeParachainsHrmpHrmpChannel>>, [PolkadotParachainPrimitivesHrmpChannelId]> & QueryableStorageEntry<ApiType, [PolkadotParachainPrimitivesHrmpChannelId]>;
      /**
       * A set of pending HRMP close channel requests that are going to be closed during the session change.
       * Used for checking if a given channel is registered for closure.
       * 
       * The set is accompanied by a list for iteration.
       * 
       * Invariant:
       * - There are no channels that exists in list but not in the set and vice versa.
       **/
      hrmpCloseChannelRequests: AugmentedQuery<ApiType, (arg: PolkadotParachainPrimitivesHrmpChannelId | { sender?: any; recipient?: any } | string | Uint8Array) => Observable<Option<Null>>, [PolkadotParachainPrimitivesHrmpChannelId]> & QueryableStorageEntry<ApiType, [PolkadotParachainPrimitivesHrmpChannelId]>;
      hrmpCloseChannelRequestsList: AugmentedQuery<ApiType, () => Observable<Vec<PolkadotParachainPrimitivesHrmpChannelId>>, []> & QueryableStorageEntry<ApiType, []>;
      hrmpEgressChannelsIndex: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Vec<u32>>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * Ingress/egress indexes allow to find all the senders and receivers given the opposite
       * side. I.e.
       * 
       * (a) ingress index allows to find all the senders for a given recipient.
       * (b) egress index allows to find all the recipients for a given sender.
       * 
       * Invariants:
       * - for each ingress index entry for `P` each item `I` in the index should present in `HrmpChannels`
       * as `(I, P)`.
       * - for each egress index entry for `P` each item `E` in the index should present in `HrmpChannels`
       * as `(P, E)`.
       * - there should be no other dangling channels in `HrmpChannels`.
       * - the vectors are sorted.
       **/
      hrmpIngressChannelsIndex: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Vec<u32>>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * This mapping tracks how many open channel requests are initiated by a given sender para.
       * Invariant: `HrmpOpenChannelRequests` should contain the same number of items that has `(X, _)`
       * as the number of `HrmpOpenChannelRequestCount` for `X`.
       **/
      hrmpOpenChannelRequestCount: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<u32>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * The set of pending HRMP open channel requests.
       * 
       * The set is accompanied by a list for iteration.
       * 
       * Invariant:
       * - There are no channels that exists in list but not in the set and vice versa.
       **/
      hrmpOpenChannelRequests: AugmentedQuery<ApiType, (arg: PolkadotParachainPrimitivesHrmpChannelId | { sender?: any; recipient?: any } | string | Uint8Array) => Observable<Option<PolkadotRuntimeParachainsHrmpHrmpOpenChannelRequest>>, [PolkadotParachainPrimitivesHrmpChannelId]> & QueryableStorageEntry<ApiType, [PolkadotParachainPrimitivesHrmpChannelId]>;
      hrmpOpenChannelRequestsList: AugmentedQuery<ApiType, () => Observable<Vec<PolkadotParachainPrimitivesHrmpChannelId>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * The HRMP watermark associated with each para.
       * Invariant:
       * - each para `P` used here as a key should satisfy `Paras::is_valid_para(P)` within a session.
       **/
      hrmpWatermarks: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<u32>>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    imOnline: {
      /**
       * For each session index, we keep a mapping of `ValidatorId<T>` to the
       * number of blocks authored by the given authority.
       **/
      authoredBlocks: AugmentedQuery<ApiType, (arg1: u32 | AnyNumber | Uint8Array, arg2: AccountId32 | string | Uint8Array) => Observable<u32>, [u32, AccountId32]> & QueryableStorageEntry<ApiType, [u32, AccountId32]>;
      /**
       * The block number after which it's ok to send heartbeats in the current
       * session.
       * 
       * At the beginning of each session we set this to a value that should fall
       * roughly in the middle of the session duration. The idea is to first wait for
       * the validators to produce a block in the current session, so that the
       * heartbeat later on will not be necessary.
       * 
       * This value will only be used as a fallback if we fail to get a proper session
       * progress estimate from `NextSessionRotation`, as those estimates should be
       * more accurate then the value we calculate for `HeartbeatAfter`.
       **/
      heartbeatAfter: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * The current set of keys that may issue a heartbeat.
       **/
      keys: AugmentedQuery<ApiType, () => Observable<Vec<PalletImOnlineSr25519AppSr25519Public>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * For each session index, we keep a mapping of `SessionIndex` and `AuthIndex` to
       * `WrapperOpaque<BoundedOpaqueNetworkState>`.
       **/
      receivedHeartbeats: AugmentedQuery<ApiType, (arg1: u32 | AnyNumber | Uint8Array, arg2: u32 | AnyNumber | Uint8Array) => Observable<Option<WrapperOpaque<PalletImOnlineBoundedOpaqueNetworkState>>>, [u32, u32]> & QueryableStorageEntry<ApiType, [u32, u32]>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    indices: {
      /**
       * The lookup from index to account.
       **/
      accounts: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<ITuple<[AccountId32, u128, bool]>>>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    initializer: {
      /**
       * Buffered session changes along with the block number at which they should be applied.
       * 
       * Typically this will be empty or one element long. Apart from that this item never hits
       * the storage.
       * 
       * However this is a `Vec` regardless to handle various edge cases that may occur at runtime
       * upgrade boundaries or if governance intervenes.
       **/
      bufferedSessionChanges: AugmentedQuery<ApiType, () => Observable<Vec<PolkadotRuntimeParachainsInitializerBufferedSessionChange>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Whether the parachains modules have been initialized within this block.
       * 
       * Semantically a `bool`, but this guarantees it should never hit the trie,
       * as this is cleared in `on_finalize` and Frame optimizes `None` values to be empty values.
       * 
       * As a `bool`, `set(false)` and `remove()` both lead to the next `get()` being false, but one of
       * them writes to the trie and one does not. This confusion makes `Option<()>` more suitable for
       * the semantics of this variable.
       **/
      hasInitialized: AugmentedQuery<ApiType, () => Observable<Option<Null>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    membership: {
      /**
       * The current membership, stored as an ordered Vec.
       **/
      members: AugmentedQuery<ApiType, () => Observable<Vec<AccountId32>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * The current prime member, if one exists.
       **/
      prime: AugmentedQuery<ApiType, () => Observable<Option<AccountId32>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    mmr: {
      /**
       * Hashes of the nodes in the MMR.
       * 
       * Note this collection only contains MMR peaks, the inner nodes (and leaves)
       * are pruned and only stored in the Offchain DB.
       **/
      nodes: AugmentedQuery<ApiType, (arg: u64 | AnyNumber | Uint8Array) => Observable<Option<H256>>, [u64]> & QueryableStorageEntry<ApiType, [u64]>;
      /**
       * Current size of the MMR (number of leaves).
       **/
      numberOfLeaves: AugmentedQuery<ApiType, () => Observable<u64>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Latest MMR Root hash.
       **/
      rootHash: AugmentedQuery<ApiType, () => Observable<H256>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    mmrLeaf: {
      /**
       * Details of next BEEFY authority set.
       * 
       * This storage entry is used as cache for calls to `update_beefy_next_authority_set`.
       **/
      beefyNextAuthorities: AugmentedQuery<ApiType, () => Observable<BeefyPrimitivesMmrBeefyNextAuthoritySet>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    multisig: {
      calls: AugmentedQuery<ApiType, (arg: U8aFixed | string | Uint8Array) => Observable<Option<ITuple<[Bytes, AccountId32, u128]>>>, [U8aFixed]> & QueryableStorageEntry<ApiType, [U8aFixed]>;
      /**
       * The set of open multisig operations.
       **/
      multisigs: AugmentedQuery<ApiType, (arg1: AccountId32 | string | Uint8Array, arg2: U8aFixed | string | Uint8Array) => Observable<Option<PalletMultisigMultisig>>, [AccountId32, U8aFixed]> & QueryableStorageEntry<ApiType, [AccountId32, U8aFixed]>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    offences: {
      /**
       * A vector of reports of the same kind that happened at the same time slot.
       **/
      concurrentReportsIndex: AugmentedQuery<ApiType, (arg1: U8aFixed | string | Uint8Array, arg2: Bytes | string | Uint8Array) => Observable<Vec<H256>>, [U8aFixed, Bytes]> & QueryableStorageEntry<ApiType, [U8aFixed, Bytes]>;
      /**
       * The primary structure that holds all offence records keyed by report identifiers.
       **/
      reports: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Option<SpStakingOffenceOffenceDetails>>, [H256]> & QueryableStorageEntry<ApiType, [H256]>;
      /**
       * Enumerates all reports of a kind along with the time they happened.
       * 
       * All reports are sorted by the time of offence.
       * 
       * Note that the actual type of this mapping is `Vec<u8>`, this is because values of
       * different types are not supported at the moment so we are doing the manual serialization.
       **/
      reportsByKindIndex: AugmentedQuery<ApiType, (arg: U8aFixed | string | Uint8Array) => Observable<Bytes>, [U8aFixed]> & QueryableStorageEntry<ApiType, [U8aFixed]>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    paraInclusion: {
      /**
       * The latest bitfield for each validator, referred to by their index in the validator set.
       **/
      availabilityBitfields: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<PolkadotRuntimeParachainsInclusionAvailabilityBitfieldRecord>>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * Candidates pending availability by `ParaId`.
       **/
      pendingAvailability: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<PolkadotRuntimeParachainsInclusionCandidatePendingAvailability>>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * The commitments of candidates pending availability, by `ParaId`.
       **/
      pendingAvailabilityCommitments: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<PolkadotPrimitivesV1CandidateCommitments>>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    paraInherent: {
      /**
       * Whether the paras inherent was included within this block.
       * 
       * The `Option<()>` is effectively a `bool`, but it never hits storage in the `None` variant
       * due to the guarantees of FRAME's storage APIs.
       * 
       * If this is `None` at the end of the block, we panic and render the block invalid.
       **/
      included: AugmentedQuery<ApiType, () => Observable<Option<Null>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Scraped on chain data for extracting resolved disputes as well as backing votes.
       **/
      onChainVotes: AugmentedQuery<ApiType, () => Observable<Option<PolkadotPrimitivesV1ScrapedOnChainVotes>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    paras: {
      /**
       * The actions to perform during the start of a specific session index.
       **/
      actionsQueue: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Vec<u32>>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * Validation code stored by its hash.
       * 
       * This storage is consistent with [`FutureCodeHash`], [`CurrentCodeHash`] and
       * [`PastCodeHash`].
       **/
      codeByHash: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Option<Bytes>>, [H256]> & QueryableStorageEntry<ApiType, [H256]>;
      /**
       * The number of reference on the validation code in [`CodeByHash`] storage.
       **/
      codeByHashRefs: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<u32>, [H256]> & QueryableStorageEntry<ApiType, [H256]>;
      /**
       * The validation code hash of every live para.
       * 
       * Corresponding code can be retrieved with [`CodeByHash`].
       **/
      currentCodeHash: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<H256>>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * The actual future code hash of a para.
       * 
       * Corresponding code can be retrieved with [`CodeByHash`].
       **/
      futureCodeHash: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<H256>>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * The block number at which the planned code change is expected for a para.
       * The change will be applied after the first parablock for this ID included which executes
       * in the context of a relay chain block with a number >= `expected_at`.
       **/
      futureCodeUpgrades: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<u32>>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * The head-data of every registered para.
       **/
      heads: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<Bytes>>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * All parachains. Ordered ascending by `ParaId`. Parathreads are not included.
       **/
      parachains: AugmentedQuery<ApiType, () => Observable<Vec<u32>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * The current lifecycle of a all known Para IDs.
       **/
      paraLifecycles: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<PolkadotRuntimeParachainsParasParaLifecycle>>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * Actual past code hash, indicated by the para id as well as the block number at which it
       * became outdated.
       * 
       * Corresponding code can be retrieved with [`CodeByHash`].
       **/
      pastCodeHash: AugmentedQuery<ApiType, (arg: ITuple<[u32, u32]> | [u32 | AnyNumber | Uint8Array, u32 | AnyNumber | Uint8Array]) => Observable<Option<H256>>, [ITuple<[u32, u32]>]> & QueryableStorageEntry<ApiType, [ITuple<[u32, u32]>]>;
      /**
       * Past code of parachains. The parachains themselves may not be registered anymore,
       * but we also keep their code on-chain for the same amount of time as outdated code
       * to keep it available for secondary checkers.
       **/
      pastCodeMeta: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<PolkadotRuntimeParachainsParasParaPastCodeMeta>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * Which paras have past code that needs pruning and the relay-chain block at which the code was replaced.
       * Note that this is the actual height of the included block, not the expected height at which the
       * code upgrade would be applied, although they may be equal.
       * This is to ensure the entire acceptance period is covered, not an offset acceptance period starting
       * from the time at which the parachain perceives a code upgrade as having occurred.
       * Multiple entries for a single para are permitted. Ordered ascending by block number.
       **/
      pastCodePruning: AugmentedQuery<ApiType, () => Observable<Vec<ITuple<[u32, u32]>>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Upcoming paras instantiation arguments.
       **/
      upcomingParasGenesis: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<PolkadotRuntimeParachainsParasParaGenesisArgs>>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * The list of upcoming code upgrades. Each item is a pair of which para performs a code
       * upgrade and at which relay-chain block it is expected at.
       * 
       * Ordered ascending by block number.
       **/
      upcomingUpgrades: AugmentedQuery<ApiType, () => Observable<Vec<ITuple<[u32, u32]>>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * The list of parachains that are awaiting for their upgrade restriction to cooldown.
       * 
       * Ordered ascending by block number.
       **/
      upgradeCooldowns: AugmentedQuery<ApiType, () => Observable<Vec<ITuple<[u32, u32]>>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * This is used by the relay-chain to communicate to a parachain a go-ahead with in the upgrade procedure.
       * 
       * This value is absent when there are no upgrades scheduled or during the time the relay chain
       * performs the checks. It is set at the first relay-chain block when the corresponding parachain
       * can switch its upgrade function. As soon as the parachain's block is included, the value
       * gets reset to `None`.
       * 
       * NOTE that this field is used by parachains via merkle storage proofs, therefore changing
       * the format will require migration of parachains.
       **/
      upgradeGoAheadSignal: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<PolkadotPrimitivesV1UpgradeGoAhead>>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * This is used by the relay-chain to communicate that there are restrictions for performing
       * an upgrade for this parachain.
       * 
       * This may be a because the parachain waits for the upgrade cooldown to expire. Another
       * potential use case is when we want to perform some maintenance (such as storage migration)
       * we could restrict upgrades to make the process simpler.
       * 
       * NOTE that this field is used by parachains via merkle storage proofs, therefore changing
       * the format will require migration of parachains.
       **/
      upgradeRestrictionSignal: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<PolkadotPrimitivesV1UpgradeRestriction>>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    paraScheduler: {
      /**
       * One entry for each availability core. Entries are `None` if the core is not currently occupied. Can be
       * temporarily `Some` if scheduled but not occupied.
       * The i'th parachain belongs to the i'th core, with the remaining cores all being
       * parathread-multiplexers.
       * 
       * Bounded by the maximum of either of these two values:
       * * The number of parachains and parathread multiplexers
       * * The number of validators divided by `configuration.max_validators_per_core`.
       **/
      availabilityCores: AugmentedQuery<ApiType, () => Observable<Vec<Option<PolkadotPrimitivesV1CoreOccupied>>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * An index used to ensure that only one claim on a parathread exists in the queue or is
       * currently being handled by an occupied core.
       * 
       * Bounded by the number of parathread cores and scheduling lookahead. Reasonably, 10 * 50 = 500.
       **/
      parathreadClaimIndex: AugmentedQuery<ApiType, () => Observable<Vec<u32>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * A queue of upcoming claims and which core they should be mapped onto.
       * 
       * The number of queued claims is bounded at the `scheduling_lookahead`
       * multiplied by the number of parathread multiplexer cores. Reasonably, 10 * 50 = 500.
       **/
      parathreadQueue: AugmentedQuery<ApiType, () => Observable<PolkadotRuntimeParachainsSchedulerParathreadClaimQueue>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Currently scheduled cores - free but up to be occupied.
       * 
       * Bounded by the number of cores: one for each parachain and parathread multiplexer.
       * 
       * The value contained here will not be valid after the end of a block. Runtime APIs should be used to determine scheduled cores/
       * for the upcoming block.
       **/
      scheduled: AugmentedQuery<ApiType, () => Observable<Vec<PolkadotRuntimeParachainsSchedulerCoreAssignment>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * The block number where the session start occurred. Used to track how many group rotations have occurred.
       * 
       * Note that in the context of parachains modules the session change is signaled during
       * the block and enacted at the end of the block (at the finalization stage, to be exact).
       * Thus for all intents and purposes the effect of the session change is observed at the
       * block following the session change, block number of which we save in this storage value.
       **/
      sessionStartBlock: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * All the validator groups. One for each core. Indices are into `ActiveValidators` - not the
       * broader set of Polkadot validators, but instead just the subset used for parachains during
       * this session.
       * 
       * Bound: The number of cores is the sum of the numbers of parachains and parathread multiplexers.
       * Reasonably, 100-1000. The dominant factor is the number of validators: safe upper bound at 10k.
       **/
      validatorGroups: AugmentedQuery<ApiType, () => Observable<Vec<Vec<u32>>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    parasDisputes: {
      /**
       * All ongoing or concluded disputes for the last several sessions.
       **/
      disputes: AugmentedQuery<ApiType, (arg1: u32 | AnyNumber | Uint8Array, arg2: H256 | string | Uint8Array) => Observable<Option<PolkadotPrimitivesV1DisputeState>>, [u32, H256]> & QueryableStorageEntry<ApiType, [u32, H256]>;
      /**
       * Whether the chain is frozen. Starts as `None`. When this is `Some`,
       * the chain will not accept any new parachain blocks for backing or inclusion,
       * and its value indicates the last valid block number in the chain.
       * It can only be set back to `None` by governance intervention.
       **/
      frozen: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * All included blocks on the chain, as well as the block number in this chain that
       * should be reverted back to if the candidate is disputed and determined to be invalid.
       **/
      included: AugmentedQuery<ApiType, (arg1: u32 | AnyNumber | Uint8Array, arg2: H256 | string | Uint8Array) => Observable<Option<u32>>, [u32, H256]> & QueryableStorageEntry<ApiType, [u32, H256]>;
      /**
       * The last pruned session, if any. All data stored by this module
       * references sessions.
       **/
      lastPrunedSession: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Maps session indices to a vector indicating the number of potentially-spam disputes
       * each validator is participating in. Potentially-spam disputes are remote disputes which have
       * fewer than `byzantine_threshold + 1` validators.
       * 
       * The i'th entry of the vector corresponds to the i'th validator in the session.
       **/
      spamSlots: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<Vec<u32>>>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    paraSessionInfo: {
      /**
       * Assignment keys for the current session.
       * Note that this API is private due to it being prone to 'off-by-one' at session boundaries.
       * When in doubt, use `Sessions` API instead.
       **/
      assignmentKeysUnsafe: AugmentedQuery<ApiType, () => Observable<Vec<PolkadotPrimitivesV1AssignmentAppPublic>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * The earliest session for which previous session info is stored.
       **/
      earliestStoredSession: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Session information in a rolling window.
       * Should have an entry in range `EarliestStoredSession..=CurrentSessionIndex`.
       * Does not have any entries before the session index in the first session change notification.
       **/
      sessions: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<PolkadotPrimitivesV1SessionInfo>>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    parasShared: {
      /**
       * All the validators actively participating in parachain consensus.
       * Indices are into the broader validator set.
       **/
      activeValidatorIndices: AugmentedQuery<ApiType, () => Observable<Vec<u32>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * The parachain attestation keys of the validators actively participating in parachain consensus.
       * This should be the same length as `ActiveValidatorIndices`.
       **/
      activeValidatorKeys: AugmentedQuery<ApiType, () => Observable<Vec<PolkadotPrimitivesV0ValidatorAppPublic>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * The current session index.
       **/
      currentSessionIndex: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    proxy: {
      /**
       * The announcements made by the proxy (key).
       **/
      announcements: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<ITuple<[Vec<PalletProxyAnnouncement>, u128]>>, [AccountId32]> & QueryableStorageEntry<ApiType, [AccountId32]>;
      /**
       * The set of account proxies. Maps the account which has delegated to the accounts
       * which are being delegated to, together with the amount held on deposit.
       **/
      proxies: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<ITuple<[Vec<PalletProxyProxyDefinition>, u128]>>, [AccountId32]> & QueryableStorageEntry<ApiType, [AccountId32]>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    registrar: {
      /**
       * The next free `ParaId`.
       **/
      nextFreeParaId: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Amount held on deposit for each para and the original depositor.
       * 
       * The given account ID is responsible for registering the code and initial head data, but may only do
       * so if it isn't yet registered. (After that, it's up to governance to do so.)
       **/
      paras: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<PolkadotRuntimeCommonParasRegistrarParaInfo>>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * Pending swap operations.
       **/
      pendingSwap: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<u32>>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    session: {
      /**
       * Current index of the session.
       **/
      currentIndex: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Indices of disabled validators.
       * 
       * The vec is always kept sorted so that we can find whether a given validator is
       * disabled using binary search. It gets cleared when `on_session_ending` returns
       * a new set of identities.
       **/
      disabledValidators: AugmentedQuery<ApiType, () => Observable<Vec<u32>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * The owner of a key. The key is the `KeyTypeId` + the encoded key.
       **/
      keyOwner: AugmentedQuery<ApiType, (arg: ITuple<[SpCoreCryptoKeyTypeId, Bytes]> | [SpCoreCryptoKeyTypeId | string | Uint8Array, Bytes | string | Uint8Array]) => Observable<Option<AccountId32>>, [ITuple<[SpCoreCryptoKeyTypeId, Bytes]>]> & QueryableStorageEntry<ApiType, [ITuple<[SpCoreCryptoKeyTypeId, Bytes]>]>;
      /**
       * The next session keys for a validator.
       **/
      nextKeys: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<Option<RococoRuntimeSessionKeys>>, [AccountId32]> & QueryableStorageEntry<ApiType, [AccountId32]>;
      /**
       * True if the underlying economic identities or weighting behind the validators
       * has changed in the queued validator set.
       **/
      queuedChanged: AugmentedQuery<ApiType, () => Observable<bool>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * The queued keys for the next session. When the next session begins, these keys
       * will be used to determine the validator's session keys.
       **/
      queuedKeys: AugmentedQuery<ApiType, () => Observable<Vec<ITuple<[AccountId32, RococoRuntimeSessionKeys]>>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * The current set of validators.
       **/
      validators: AugmentedQuery<ApiType, () => Observable<Vec<AccountId32>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    slots: {
      /**
       * Amounts held on deposit for each (possibly future) leased parachain.
       * 
       * The actual amount locked on its behalf by any account at any time is the maximum of the second values
       * of the items in this list whose first value is the account.
       * 
       * The first item in the list is the amount locked for the current Lease Period. Following
       * items are for the subsequent lease periods.
       * 
       * The default value (an empty list) implies that the parachain no longer exists (or never
       * existed) as far as this pallet is concerned.
       * 
       * If a parachain doesn't exist *yet* but is scheduled to exist in the future, then it
       * will be left-padded with one or more `None`s to denote the fact that nothing is held on
       * deposit for the non-existent chain currently, but is held at some point in the future.
       * 
       * It is illegal for a `None` value to trail in the list.
       **/
      leases: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Vec<Option<ITuple<[AccountId32, u128]>>>>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    sudo: {
      /**
       * The `AccountId` of the sudo key.
       **/
      key: AugmentedQuery<ApiType, () => Observable<AccountId32>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    system: {
      /**
       * The full account information for a particular account ID.
       **/
      account: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<FrameSystemAccountInfo>, [AccountId32]> & QueryableStorageEntry<ApiType, [AccountId32]>;
      /**
       * Total length (in bytes) for all extrinsics put together, for the current block.
       **/
      allExtrinsicsLen: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Map of block numbers to block hashes.
       **/
      blockHash: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<H256>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * The current weight for the block.
       **/
      blockWeight: AugmentedQuery<ApiType, () => Observable<FrameSupportWeightsPerDispatchClassU64>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Digest of the current block, also part of the block header.
       **/
      digest: AugmentedQuery<ApiType, () => Observable<SpRuntimeGenericDigest>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * The number of events in the `Events<T>` list.
       **/
      eventCount: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Events deposited for the current block.
       * 
       * NOTE: This storage item is explicitly unbounded since it is never intended to be read
       * from within the runtime.
       **/
      events: AugmentedQuery<ApiType, () => Observable<Vec<FrameSystemEventRecord>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Mapping between a topic (represented by T::Hash) and a vector of indexes
       * of events in the `<Events<T>>` list.
       * 
       * All topic vectors have deterministic storage locations depending on the topic. This
       * allows light-clients to leverage the changes trie storage tracking mechanism and
       * in case of changes fetch the list of events of interest.
       * 
       * The value has the type `(T::BlockNumber, EventIndex)` because if we used only just
       * the `EventIndex` then in case if the topic has the same contents on the next block
       * no notification will be triggered thus the event might be lost.
       **/
      eventTopics: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Vec<ITuple<[u32, u32]>>>, [H256]> & QueryableStorageEntry<ApiType, [H256]>;
      /**
       * The execution phase of the block.
       **/
      executionPhase: AugmentedQuery<ApiType, () => Observable<Option<FrameSystemPhase>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Total extrinsics count for the current block.
       **/
      extrinsicCount: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Extrinsics data for the current block (maps an extrinsic's index to its data).
       **/
      extrinsicData: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Bytes>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * Stores the `spec_version` and `spec_name` of when the last runtime upgrade happened.
       **/
      lastRuntimeUpgrade: AugmentedQuery<ApiType, () => Observable<Option<FrameSystemLastRuntimeUpgradeInfo>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * The current block number being processed. Set by `execute_block`.
       **/
      number: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Hash of the previous block.
       **/
      parentHash: AugmentedQuery<ApiType, () => Observable<H256>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * True if we have upgraded so that AccountInfo contains three types of `RefCount`. False
       * (default) if not.
       **/
      upgradedToTripleRefCount: AugmentedQuery<ApiType, () => Observable<bool>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * True if we have upgraded so that `type RefCount` is `u32`. False (default) if not.
       **/
      upgradedToU32RefCount: AugmentedQuery<ApiType, () => Observable<bool>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    timestamp: {
      /**
       * Did the timestamp get updated in this block?
       **/
      didUpdate: AugmentedQuery<ApiType, () => Observable<bool>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Current time for the current block.
       **/
      now: AugmentedQuery<ApiType, () => Observable<u64>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    transactionPayment: {
      nextFeeMultiplier: AugmentedQuery<ApiType, () => Observable<u128>, []> & QueryableStorageEntry<ApiType, []>;
      storageVersion: AugmentedQuery<ApiType, () => Observable<PalletTransactionPaymentReleases>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    ump: {
      /**
       * The ordered list of `ParaId`s that have a `RelayDispatchQueue` entry.
       * 
       * Invariant:
       * - The set of items from this vector should be exactly the set of the keys in
       * `RelayDispatchQueues` and `RelayDispatchQueueSize`.
       **/
      needsDispatch: AugmentedQuery<ApiType, () => Observable<Vec<u32>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * This is the para that gets will get dispatched first during the next upward dispatchable queue
       * execution round.
       * 
       * Invariant:
       * - If `Some(para)`, then `para` must be present in `NeedsDispatch`.
       **/
      nextDispatchRoundStartWith: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * The messages that exceeded max individual message weight budget.
       * 
       * These messages stay there until manually dispatched.
       **/
      overweight: AugmentedQuery<ApiType, (arg: u64 | AnyNumber | Uint8Array) => Observable<Option<ITuple<[u32, Bytes]>>>, [u64]> & QueryableStorageEntry<ApiType, [u64]>;
      /**
       * The number of overweight messages ever recorded in `Overweight` (and thus the lowest free
       * index).
       **/
      overweightCount: AugmentedQuery<ApiType, () => Observable<u64>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * The messages waiting to be handled by the relay-chain originating from a certain parachain.
       * 
       * Note that some upward messages might have been already processed by the inclusion logic. E.g.
       * channel management messages.
       * 
       * The messages are processed in FIFO order.
       **/
      relayDispatchQueues: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Vec<Bytes>>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * Size of the dispatch queues. Caches sizes of the queues in `RelayDispatchQueue`.
       * 
       * First item in the tuple is the count of messages and second
       * is the total length (in bytes) of the message payloads.
       * 
       * Note that this is an auxiliary mapping: it's possible to tell the byte size and the number of
       * messages only looking at `RelayDispatchQueues`. This mapping is separate to avoid the cost of
       * loading the whole message queue if only the total size and count are required.
       * 
       * Invariant:
       * - The set of keys should exactly match the set of keys of `RelayDispatchQueues`.
       **/
      relayDispatchQueueSize: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<ITuple<[u32, u32]>>, [u32]> & QueryableStorageEntry<ApiType, [u32]>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    validatorManager: {
      /**
       * Validators that should be added.
       **/
      validatorsToAdd: AugmentedQuery<ApiType, () => Observable<Vec<AccountId32>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Validators that should be retired, because their Parachain was deregistered.
       **/
      validatorsToRetire: AugmentedQuery<ApiType, () => Observable<Vec<AccountId32>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
    xcmPallet: {
      /**
       * The existing asset traps.
       * 
       * Key is the blake2 256 hash of (origin, versioned `MultiAssets`) pair. Value is the number of
       * times this pair has been trapped (usually just 1 if it exists at all).
       **/
      assetTraps: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<u32>, [H256]> & QueryableStorageEntry<ApiType, [H256]>;
      /**
       * The current migration's stage, if any.
       **/
      currentMigration: AugmentedQuery<ApiType, () => Observable<Option<PalletXcmVersionMigrationStage>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * The ongoing queries.
       **/
      queries: AugmentedQuery<ApiType, (arg: u64 | AnyNumber | Uint8Array) => Observable<Option<PalletXcmQueryStatus>>, [u64]> & QueryableStorageEntry<ApiType, [u64]>;
      /**
       * The latest available query index.
       **/
      queryCounter: AugmentedQuery<ApiType, () => Observable<u64>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * Default version to encode XCM when latest version of destination is unknown. If `None`,
       * then the destinations whose XCM version is unknown are considered unreachable.
       **/
      safeXcmVersion: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * The Latest versions that we know various locations support.
       **/
      supportedVersion: AugmentedQuery<ApiType, (arg1: u32 | AnyNumber | Uint8Array, arg2: XcmVersionedMultiLocation | { V0: any } | { V1: any } | string | Uint8Array) => Observable<Option<u32>>, [u32, XcmVersionedMultiLocation]> & QueryableStorageEntry<ApiType, [u32, XcmVersionedMultiLocation]>;
      /**
       * Destinations whose latest XCM version we would like to know. Duplicates not allowed, and
       * the `u32` counter is the number of times that a send to the destination has been attempted,
       * which is used as a prioritization.
       **/
      versionDiscoveryQueue: AugmentedQuery<ApiType, () => Observable<Vec<ITuple<[XcmVersionedMultiLocation, u32]>>>, []> & QueryableStorageEntry<ApiType, []>;
      /**
       * All locations that we have requested version notifications from.
       **/
      versionNotifiers: AugmentedQuery<ApiType, (arg1: u32 | AnyNumber | Uint8Array, arg2: XcmVersionedMultiLocation | { V0: any } | { V1: any } | string | Uint8Array) => Observable<Option<u64>>, [u32, XcmVersionedMultiLocation]> & QueryableStorageEntry<ApiType, [u32, XcmVersionedMultiLocation]>;
      /**
       * The target locations that are subscribed to our version changes, as well as the most recent
       * of our versions we informed them of.
       **/
      versionNotifyTargets: AugmentedQuery<ApiType, (arg1: u32 | AnyNumber | Uint8Array, arg2: XcmVersionedMultiLocation | { V0: any } | { V1: any } | string | Uint8Array) => Observable<Option<ITuple<[u64, u64, u32]>>>, [u32, XcmVersionedMultiLocation]> & QueryableStorageEntry<ApiType, [u32, XcmVersionedMultiLocation]>;
      /**
       * Generic query
       **/
      [key: string]: QueryableStorageEntry<ApiType>;
    };
  }

  export interface QueryableStorage<ApiType extends ApiTypes> extends AugmentedQueries<ApiType> {
    [key: string]: QueryableModuleStorage<ApiType>;
  }
}
