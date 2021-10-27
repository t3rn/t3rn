// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

import type { ApiTypes, SubmittableExtrinsic } from '@polkadot/api/types';
import type { Bytes, Compact, Option, Vec, bool, u16, u32, u64 } from '@polkadot/types';
import type { BridgedBlockHash, BridgedHeader, ChainId, InboundRelayer, InitializationData, LaneId, MessageNonce, MessagesDeliveryProofOf, MessagesProofOf, OperatingMode, OutboundMessageFee, OutboundPayload, Parameter, UnrewardedRelayersState } from '@polkadot/types/interfaces/bridges';
import type { CodeHash } from '@polkadot/types/interfaces/contracts';
import type { Extrinsic } from '@polkadot/types/interfaces/extrinsics';
import type { GrandpaEquivocationProof, GrandpaJustification, KeyOwnerProof } from '@polkadot/types/interfaces/grandpa';
import type { AccountId, Balance, BalanceOf, BlockNumber, Call, ChangesTrieConfiguration, KeyValue, LookupSource, Moment, Perbill, Weight } from '@polkadot/types/interfaces/runtime';
import type { Keys } from '@polkadot/types/interfaces/session';
import type { Key } from '@polkadot/types/interfaces/system';
import type { AnyNumber } from '@polkadot/types/types';
import type { Compose, GatewayABIConfig } from 't3rn-circuit-typegen/interfaces/primitives';
import type { XdnsRecordId } from 't3rn-circuit-typegen/interfaces/xdns';

declare module '@polkadot/api/types/submittable' {
  export interface AugmentedSubmittables<ApiType> {
    balances: {
      /**
       * Exactly as `transfer`, except the origin must be root and the source account may be
       * specified.
       * # <weight>
       * - Same as transfer, but additional read and write because the source account is
       * not assumed to be in the overlay.
       * # </weight>
       **/
      forceTransfer: AugmentedSubmittable<(source: LookupSource | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, dest: LookupSource | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, value: Compact<Balance> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [LookupSource, LookupSource, Compact<Balance>]>;
      /**
       * Set the balances of a given account.
       *
       * This will alter `FreeBalance` and `ReservedBalance` in storage. it will
       * also decrease the total issuance of the system (`TotalIssuance`).
       * If the new free or reserved balance is below the existential deposit,
       * it will reset the account nonce (`frame_system::AccountNonce`).
       *
       * The dispatch origin for this call is `root`.
       *
       * # <weight>
       * - Independent of the arguments.
       * - Contains a limited number of reads and writes.
       * ---------------------
       * - Base Weight:
       * - Creating: 27.56 µs
       * - Killing: 35.11 µs
       * - DB Weight: 1 Read, 1 Write to `who`
       * # </weight>
       **/
      setBalance: AugmentedSubmittable<(who: LookupSource | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, newFree: Compact<Balance> | AnyNumber | Uint8Array, newReserved: Compact<Balance> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [LookupSource, Compact<Balance>, Compact<Balance>]>;
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
       * - Dependent on arguments but not critical, given proper implementations for
       * input config types. See related functions below.
       * - It contains a limited number of reads and writes internally and no complex computation.
       *
       * Related functions:
       *
       * - `ensure_can_withdraw` is always called internally but has a bounded complexity.
       * - Transferring balances to accounts that did not exist before will cause
       * `T::OnNewAccount::on_new_account` to be called.
       * - Removing enough funds from an account will trigger `T::DustRemoval::on_unbalanced`.
       * - `transfer_keep_alive` works the same way as `transfer`, but has an additional
       * check that the transfer will not kill the origin account.
       * ---------------------------------
       * - Base Weight: 73.64 µs, worst case scenario (account created, account removed)
       * - DB Weight: 1 Read and 1 Write to destination account
       * - Origin account is already in memory, so no DB operations for them.
       * # </weight>
       **/
      transfer: AugmentedSubmittable<(dest: LookupSource | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, value: Compact<Balance> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [LookupSource, Compact<Balance>]>;
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
       * keep the sender account alive (true).
       * # <weight>
       * - O(1). Just like transfer, but reading the user's transferable balance first.
       * #</weight>
       **/
      transferAll: AugmentedSubmittable<(dest: LookupSource | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, keepAlive: bool | boolean | Uint8Array) => SubmittableExtrinsic<ApiType>, [LookupSource, bool]>;
      /**
       * Same as the [`transfer`] call, but with a check that the transfer will not kill the
       * origin account.
       *
       * 99% of the time you want [`transfer`] instead.
       *
       * [`transfer`]: struct.Pallet.html#method.transfer
       * # <weight>
       * - Cheaper than transfer because account cannot be killed.
       * - Base Weight: 51.4 µs
       * - DB Weight: 1 Read and 1 Write to dest (sender is in overlay already)
       * #</weight>
       **/
      transferKeepAlive: AugmentedSubmittable<(dest: LookupSource | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, value: Compact<Balance> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [LookupSource, Compact<Balance>]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    bridgeGatewayGrandpa: {
      /**
       * Bootstrap the bridge pallet with an initial header and authority set from which to sync.
       *
       * The initial configuration provided does not need to be the genesis header of the bridged
       * chain, it can be any arbirary header. You can also provide the next scheduled set change
       * if it is already know.
       *
       * This function is only allowed to be called from a trusted origin and writes to storage
       * with practically no checks in terms of the validity of the data. It is important that
       * you ensure that valid data is being passed in.
       **/
      initialize: AugmentedSubmittable<(initData: InitializationData | { header?: any; authorityList?: any; setId?: any; isHalted?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [InitializationData]>;
      /**
       * Halt or resume all pallet operations.
       *
       * May only be called either by root, or by `PalletOwner`.
       **/
      setOperational: AugmentedSubmittable<(operational: bool | boolean | Uint8Array) => SubmittableExtrinsic<ApiType>, [bool]>;
      /**
       * Change `PalletOwner`.
       *
       * May only be called either by root, or by `PalletOwner`.
       **/
      setOwner: AugmentedSubmittable<(newOwner: Option<AccountId> | null | object | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Option<AccountId>]>;
      /**
       * Verify a target header is finalized according to the given finality proof.
       *
       * It will use the underlying storage pallet to fetch information about the current
       * authorities and best finalized header in order to verify that the header is finalized.
       *
       * If successful in verification, it will write the target header to the underlying storage
       * pallet.
       **/
      submitFinalityProof: AugmentedSubmittable<(finalityTarget: BridgedHeader | { parentHash?: any; number?: any; stateRoot?: any; extrinsicsRoot?: any; digest?: any } | string | Uint8Array, justification: GrandpaJustification | { round?: any; commit?: any; votesAncestries?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [BridgedHeader, GrandpaJustification]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    bridgeGatewayMessages: {
      /**
       * Pay additional fee for the message.
       **/
      increaseMessageFee: AugmentedSubmittable<(laneId: LaneId | string | Uint8Array, nonce: MessageNonce | AnyNumber | Uint8Array, additionalFee: OutboundMessageFee | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [LaneId, MessageNonce, OutboundMessageFee]>;
      /**
       * Receive messages delivery proof from bridged chain.
       **/
      receiveMessagesDeliveryProof: AugmentedSubmittable<(proof: MessagesDeliveryProofOf | { bridgedHeaderHash?: any; storageProof?: any; lane?: any } | string | Uint8Array, relayersState: UnrewardedRelayersState | { unrewardedRelayer_Entries?: any; messagesInOldestEntry?: any; totalMessages?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [MessagesDeliveryProofOf, UnrewardedRelayersState]>;
      /**
       * Receive messages proof from bridged chain.
       *
       * The weight of the call assumes that the transaction always brings outbound lane
       * state update. Because of that, the submitter (relayer) has no benefit of not including
       * this data in the transaction, so reward confirmations lags should be minimal.
       **/
      receiveMessagesProof: AugmentedSubmittable<(relayerId: InboundRelayer | string | Uint8Array, proof: MessagesProofOf | { bridgedHeaderHash?: any; storageProof?: any; lane?: any; noncesStart?: any; noncesEnd?: any } | string | Uint8Array, messagesCount: u32 | AnyNumber | Uint8Array, dispatchWeight: Weight | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [InboundRelayer, MessagesProofOf, u32, Weight]>;
      /**
       * Send message over lane.
       **/
      sendMessage: AugmentedSubmittable<(laneId: LaneId | string | Uint8Array, payload: OutboundPayload | { specVersion?: any; weight?: any; origin?: any; dispatchFeePayment?: any; call?: any } | string | Uint8Array, deliveryAndDispatchFee: OutboundMessageFee | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [LaneId, OutboundPayload, OutboundMessageFee]>;
      /**
       * Halt or resume all/some pallet operations.
       *
       * May only be called either by root, or by `PalletOwner`.
       **/
      setOperatingMode: AugmentedSubmittable<(operatingMode: OperatingMode | 'Normal' | 'RejectingOutboundMessages' | 'Halted' | number | Uint8Array) => SubmittableExtrinsic<ApiType>, [OperatingMode]>;
      /**
       * Change `PalletOwner`.
       *
       * May only be called either by root, or by `PalletOwner`.
       **/
      setOwner: AugmentedSubmittable<(newOwner: Option<AccountId> | null | object | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Option<AccountId>]>;
      /**
       * Update pallet parameter.
       *
       * May only be called either by root, or by `PalletOwner`.
       *
       * The weight is: single read for permissions check + 2 writes for parameter value and event.
       **/
      updatePalletParameter: AugmentedSubmittable<(parameter: Parameter | null) => SubmittableExtrinsic<ApiType>, [Parameter]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    bridgePolkadotLikeMultiFinalityVerifier: {
      /**
       * Bootstrap the bridge pallet with an initial header and authority set from which to sync.
       *
       * The initial configuration provided does not need to be the genesis header of the bridged
       * chain, it can be any arbirary header. You can also provide the next scheduled set change
       * if it is already know.
       *
       * This function is only allowed to be called from a trusted origin and writes to storage
       * with practically no checks in terms of the validity of the data. It is important that
       * you ensure that valid data is being passed in.
       **/
      initializeSingle: AugmentedSubmittable<(initData: InitializationData | { header?: any; authorityList?: any; setId?: any; isHalted?: any } | string | Uint8Array, gatewayId: ChainId | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [InitializationData, ChainId]>;
      /**
       * Halt or resume all pallet operations.
       *
       * May only be called either by root, or by `PalletOwner`.
       **/
      setOperational: AugmentedSubmittable<(operational: bool | boolean | Uint8Array, gatewayId: ChainId | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [bool, ChainId]>;
      /**
       * Change `PalletOwner`.
       *
       * May only be called either by root, or by `PalletOwner`.
       **/
      setOwner: AugmentedSubmittable<(newOwner: Option<AccountId> | null | object | string | Uint8Array, gatewayId: ChainId | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Option<AccountId>, ChainId]>;
      /**
       * Verify a target header is finalized according to the given finality proof.
       *
       * It will use the underlying storage pallet to fetch information about the current
       * authorities and best finalized header in order to verify that the header is finalized.
       *
       * If successful in verification, it will write the target header to the underlying storage
       * pallet.
       **/
      submitFinalityProof: AugmentedSubmittable<(finalityTarget: BridgedHeader | { parentHash?: any; number?: any; stateRoot?: any; extrinsicsRoot?: any; digest?: any } | string | Uint8Array, justification: GrandpaJustification | { round?: any; commit?: any; votesAncestries?: any } | string | Uint8Array, gatewayId: ChainId | string | Uint8Array, extrinsicsRoot: BridgedBlockHash | string | Uint8Array, stateRoot: BridgedBlockHash | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [BridgedHeader, GrandpaJustification, ChainId, BridgedBlockHash, BridgedBlockHash]>;
      /**
       * Submit finality proofs for the header and additionally preserve state and extrinics root.
       **/
      submitFinalityProofAndRoots: AugmentedSubmittable<(finalityTarget: BridgedHeader | { parentHash?: any; number?: any; stateRoot?: any; extrinsicsRoot?: any; digest?: any } | string | Uint8Array, justification: GrandpaJustification | { round?: any; commit?: any; votesAncestries?: any } | string | Uint8Array, gatewayId: ChainId | string | Uint8Array, stateRoot: BridgedBlockHash | string | Uint8Array, extrinsicsRoot: BridgedBlockHash | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [BridgedHeader, GrandpaJustification, ChainId, BridgedBlockHash, BridgedBlockHash]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    contracts: {
      /**
       * Makes a call to an account, optionally transferring some balance.
       *
       * * If the account is a smart-contract account, the associated code will be
       * executed and any value will be transferred.
       * * If the account is a regular account, any value will be transferred.
       * * If no account exists and the call value is not less than `existential_deposit`,
       * a regular account will be created and any value will be transferred.
       **/
      call: AugmentedSubmittable<(dest: LookupSource | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, value: Compact<BalanceOf> | AnyNumber | Uint8Array, gasLimit: Compact<Weight> | AnyNumber | Uint8Array, data: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [LookupSource, Compact<BalanceOf>, Compact<Weight>, Bytes]>;
      /**
       * Allows block producers to claim a small reward for evicting a contract. If a block
       * producer fails to do so, a regular users will be allowed to claim the reward.
       *
       * In case of a successful eviction no fees are charged from the sender. However, the
       * reward is capped by the total amount of rent that was paid by the contract while
       * it was alive.
       *
       * If contract is not evicted as a result of this call, [`Error::ContractNotEvictable`]
       * is returned and the sender is not eligible for the reward.
       **/
      claimSurcharge: AugmentedSubmittable<(dest: AccountId | string | Uint8Array, auxSender: Option<AccountId> | null | object | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [AccountId, Option<AccountId>]>;
      /**
       * Instantiates a contract from a previously deployed wasm binary.
       *
       * This function is identical to [`Self::instantiate_with_code`] but without the
       * code deployment step. Instead, the `code_hash` of an on-chain deployed wasm binary
       * must be supplied.
       **/
      instantiate: AugmentedSubmittable<(endowment: Compact<BalanceOf> | AnyNumber | Uint8Array, gasLimit: Compact<Weight> | AnyNumber | Uint8Array, codeHash: CodeHash | string | Uint8Array, data: Bytes | string | Uint8Array, salt: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<BalanceOf>, Compact<Weight>, CodeHash, Bytes, Bytes]>;
      /**
       * Instantiates a new contract from the supplied `code` optionally transferring
       * some balance.
       *
       * This is the only function that can deploy new code to the chain.
       *
       * # Parameters
       *
       * * `endowment`: The balance to transfer from the `origin` to the newly created contract.
       * * `gas_limit`: The gas limit enforced when executing the constructor.
       * * `code`: The contract code to deploy in raw bytes.
       * * `data`: The input data to pass to the contract constructor.
       * * `salt`: Used for the address derivation. See [`Pallet::contract_address`].
       *
       * Instantiation is executed as follows:
       *
       * - The supplied `code` is instrumented, deployed, and a `code_hash` is created for that code.
       * - If the `code_hash` already exists on the chain the underlying `code` will be shared.
       * - The destination address is computed based on the sender, code_hash and the salt.
       * - The smart-contract account is created at the computed address.
       * - The `endowment` is transferred to the new account.
       * - The `deploy` function is executed in the context of the newly-created account.
       **/
      instantiateWithCode: AugmentedSubmittable<(endowment: Compact<BalanceOf> | AnyNumber | Uint8Array, gasLimit: Compact<Weight> | AnyNumber | Uint8Array, code: Bytes | string | Uint8Array, data: Bytes | string | Uint8Array, salt: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<BalanceOf>, Compact<Weight>, Bytes, Bytes, Bytes]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    contractsRegistry: {
      /**
       * Inserts a contract into the on-chain registry. Root only access.
       **/
      addNewContract: AugmentedSubmittable<(requester: AccountId | string | Uint8Array, contract: RegistryContract) => SubmittableExtrinsic<ApiType>, [AccountId, RegistryContract]>;
      /**
       * Removes a contract from the onchain registry. Root only access.
       **/
      purge: AugmentedSubmittable<(requester: AccountId | string | Uint8Array, contractId: RegistryContractId) => SubmittableExtrinsic<ApiType>, [AccountId, RegistryContractId]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    ethereumLightClient: {
      /**
       * Import a single Ethereum PoW header.
       *
       * Note that this extrinsic has a very high weight. The weight is affected by the
       * value of `DescendantsUntilFinalized`. Regenerate weights if it changes.
       *
       * The largest contributors to the worst case weight, in decreasing order, are:
       * - Pruning: max 2 writes per pruned header + 2 writes to finalize pruning state.
       * Up to `HEADERS_TO_PRUNE_IN_SINGLE_IMPORT` can be pruned in one call.
       * - Ethash validation: this cost is pure CPU. EthashProver checks a merkle proof
       * for each DAG node selected in the "hashimoto"-loop.
       * - Iterating over ancestors: min `DescendantsUntilFinalized` reads to find the
       * newly finalized ancestor of a header.
       **/
      importHeader: AugmentedSubmittable<(header: EthereumHeader, proof: Vec<EthashProofData> | (EthashProofData)[]) => SubmittableExtrinsic<ApiType>, [EthereumHeader, Vec<EthashProofData>]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    execDelivery: {
      registerGateway: AugmentedSubmittable<(url: Bytes | string | Uint8Array, gatewayId: ChainId | string | Uint8Array, gatewayAbi: GatewayABIConfig | { block_number_type_size?: any; hash_size?: any; hasher?: any; crypto?: any; address_length?: any; value_type_size?: any; decimals?: any; structs?: any } | string | Uint8Array, gatewayVendor: GatewayVendor, gatewayType: GatewayType, gatewayGenesis: GatewayGenesisConfig, firstHeader: Bytes | string | Uint8Array, authorities: Option<Vec<AccountId>> | null | object | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes, ChainId, GatewayABIConfig, GatewayVendor, GatewayType, GatewayGenesisConfig, Bytes, Option<Vec<AccountId>>]>;
      submitComposableExecOrder: AugmentedSubmittable<(ioSchedule: Bytes | string | Uint8Array, components: Vec<Compose> | (Compose | { name?: any; code_txt?: any; exec_type?: any; dest?: any; value?: any; bytes?: any; input_data?: any } | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [Bytes, Vec<Compose>]>;
      submitStepConfirmation: AugmentedSubmittable<(stepConfirmation: StepConfirmation, xtxId: XtxId) => SubmittableExtrinsic<ApiType>, [StepConfirmation, XtxId]>;
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
      noteStalled: AugmentedSubmittable<(delay: BlockNumber | AnyNumber | Uint8Array, bestFinalizedBlockNumber: BlockNumber | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [BlockNumber, BlockNumber]>;
      /**
       * Report voter equivocation/misbehavior. This method will verify the
       * equivocation proof and validate the given key ownership proof
       * against the extracted offender. If both are valid, the offence
       * will be reported.
       **/
      reportEquivocation: AugmentedSubmittable<(equivocationProof: GrandpaEquivocationProof | { setId?: any; equivocation?: any } | string | Uint8Array, keyOwnerProof: KeyOwnerProof | { session?: any; trieNodes?: any; validatorCount?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [GrandpaEquivocationProof, KeyOwnerProof]>;
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
      reportEquivocationUnsigned: AugmentedSubmittable<(equivocationProof: GrandpaEquivocationProof | { setId?: any; equivocation?: any } | string | Uint8Array, keyOwnerProof: KeyOwnerProof | { session?: any; trieNodes?: any; validatorCount?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [GrandpaEquivocationProof, KeyOwnerProof]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    multiFinalityVerifier: {
      /**
       * Bootstrap the bridge pallet with an initial header and authority set from which to sync.
       *
       * The initial configuration provided does not need to be the genesis header of the bridged
       * chain, it can be any arbirary header. You can also provide the next scheduled set change
       * if it is already know.
       *
       * This function is only allowed to be called from a trusted origin and writes to storage
       * with practically no checks in terms of the validity of the data. It is important that
       * you ensure that valid data is being passed in.
       **/
      initializeSingle: AugmentedSubmittable<(initData: InitializationData | { header?: any; authorityList?: any; setId?: any; isHalted?: any } | string | Uint8Array, gatewayId: ChainId | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [InitializationData, ChainId]>;
      /**
       * Halt or resume all pallet operations.
       *
       * May only be called either by root, or by `PalletOwner`.
       **/
      setOperational: AugmentedSubmittable<(operational: bool | boolean | Uint8Array, gatewayId: ChainId | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [bool, ChainId]>;
      /**
       * Change `PalletOwner`.
       *
       * May only be called either by root, or by `PalletOwner`.
       **/
      setOwner: AugmentedSubmittable<(newOwner: Option<AccountId> | null | object | string | Uint8Array, gatewayId: ChainId | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Option<AccountId>, ChainId]>;
      /**
       * Verify a target header is finalized according to the given finality proof.
       *
       * It will use the underlying storage pallet to fetch information about the current
       * authorities and best finalized header in order to verify that the header is finalized.
       *
       * If successful in verification, it will write the target header to the underlying storage
       * pallet.
       **/
      submitFinalityProof: AugmentedSubmittable<(finalityTarget: BridgedHeader | { parentHash?: any; number?: any; stateRoot?: any; extrinsicsRoot?: any; digest?: any } | string | Uint8Array, justification: GrandpaJustification | { round?: any; commit?: any; votesAncestries?: any } | string | Uint8Array, gatewayId: ChainId | string | Uint8Array, extrinsicsRoot: BridgedBlockHash | string | Uint8Array, stateRoot: BridgedBlockHash | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [BridgedHeader, GrandpaJustification, ChainId, BridgedBlockHash, BridgedBlockHash]>;
      /**
       * Submit finality proofs for the header and additionally preserve state and extrinics root.
       **/
      submitFinalityProofAndRoots: AugmentedSubmittable<(finalityTarget: BridgedHeader | { parentHash?: any; number?: any; stateRoot?: any; extrinsicsRoot?: any; digest?: any } | string | Uint8Array, justification: GrandpaJustification | { round?: any; commit?: any; votesAncestries?: any } | string | Uint8Array, gatewayId: ChainId | string | Uint8Array, stateRoot: BridgedBlockHash | string | Uint8Array, extrinsicsRoot: BridgedBlockHash | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [BridgedHeader, GrandpaJustification, ChainId, BridgedBlockHash, BridgedBlockHash]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    session: {
      /**
       * Removes any session key(s) of the function caller.
       * This doesn't take effect until the next session.
       *
       * The dispatch origin of this function must be signed.
       *
       * # <weight>
       * - Complexity: `O(1)` in number of key types.
       * Actual cost depends on the number of length of `T::Keys::key_ids()` which is fixed.
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
       * - Complexity: `O(1)`
       * Actual cost depends on the number of length of `T::Keys::key_ids()` which is fixed.
       * - DbReads: `origin account`, `T::ValidatorIdOf`, `NextKeys`
       * - DbWrites: `origin account`, `NextKeys`
       * - DbReads per key id: `KeyOwner`
       * - DbWrites per key id: `KeyOwner`
       * # </weight>
       **/
      setKeys: AugmentedSubmittable<(keys: Keys | string | Uint8Array, proof: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Keys, Bytes]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    sudo: {
      /**
       * Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo key.
       *
       * The dispatch origin for this call must be _Signed_.
       *
       * # <weight>
       * - O(1).
       * - Limited storage reads.
       * - One DB change.
       * # </weight>
       **/
      setKey: AugmentedSubmittable<(updated: LookupSource | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [LookupSource]>;
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
      sudoAs: AugmentedSubmittable<(who: LookupSource | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, call: Call | { callIndex?: any; args?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [LookupSource, Call]>;
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
      sudoUncheckedWeight: AugmentedSubmittable<(call: Call | { callIndex?: any; args?: any } | string | Uint8Array, weight: Weight | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Call, Weight]>;
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
       *
       * # <weight>
       * - `O(P)` where `P` amount of keys with prefix `prefix`
       * - `P` storage deletions.
       * - Base Weight: 0.834 * P µs
       * - Writes: Number of subkeys + 1
       * # </weight>
       **/
      killPrefix: AugmentedSubmittable<(prefix: Key | string | Uint8Array, subkeys: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Key, u32]>;
      /**
       * Kill some items from storage.
       *
       * # <weight>
       * - `O(IK)` where `I` length of `keys` and `K` length of one key
       * - `I` storage deletions.
       * - Base Weight: .378 * i µs
       * - Writes: Number of items
       * # </weight>
       **/
      killStorage: AugmentedSubmittable<(keys: Vec<Key> | (Key | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [Vec<Key>]>;
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
       * Set the new changes trie configuration.
       *
       * # <weight>
       * - `O(1)`
       * - 1 storage write or delete (codec `O(1)`).
       * - 1 call to `deposit_log`: Uses `append` API, so O(1)
       * - Base Weight: 7.218 µs
       * - DB Weight:
       * - Writes: Changes Trie, System Digest
       * # </weight>
       **/
      setChangesTrieConfig: AugmentedSubmittable<(changesTrieConfig: Option<ChangesTrieConfiguration> | null | object | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Option<ChangesTrieConfiguration>]>;
      /**
       * Set the new runtime code.
       *
       * # <weight>
       * - `O(C + S)` where `C` length of `code` and `S` complexity of `can_set_code`
       * - 1 storage write (codec `O(C)`).
       * - 1 call to `can_set_code`: `O(S)` (calls `sp_io::misc::runtime_version` which is expensive).
       * - 1 event.
       * The weight of this function is dependent on the runtime, but generally this is very expensive.
       * We will treat this as a full block.
       * # </weight>
       **/
      setCode: AugmentedSubmittable<(code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes]>;
      /**
       * Set the new runtime code without doing any checks of the given `code`.
       *
       * # <weight>
       * - `O(C)` where `C` length of `code`
       * - 1 storage write (codec `O(C)`).
       * - 1 event.
       * The weight of this function is dependent on the runtime. We will treat this as a full block.
       * # </weight>
       **/
      setCodeWithoutChecks: AugmentedSubmittable<(code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes]>;
      /**
       * Set the number of pages in the WebAssembly environment's heap.
       *
       * # <weight>
       * - `O(1)`
       * - 1 storage write.
       * - Base Weight: 1.405 µs
       * - 1 write to HEAP_PAGES
       * # </weight>
       **/
      setHeapPages: AugmentedSubmittable<(pages: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u64]>;
      /**
       * Set some items of storage.
       *
       * # <weight>
       * - `O(I)` where `I` length of `items`
       * - `I` storage writes (`O(1)`).
       * - Base Weight: 0.568 * i µs
       * - Writes: Number of items
       * # </weight>
       **/
      setStorage: AugmentedSubmittable<(items: Vec<KeyValue> | (KeyValue)[]) => SubmittableExtrinsic<ApiType>, [Vec<KeyValue>]>;
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
       * - 1 storage read and 1 storage mutation (codec `O(1)`). (because of `DidUpdate::take` in `on_finalize`)
       * - 1 event handler `on_timestamp_set`. Must be `O(1)`.
       * # </weight>
       **/
      set: AugmentedSubmittable<(now: Compact<Moment> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<Moment>]>;
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
       * - `calls`: The calls to be dispatched from the same origin.
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
       * - `calls`: The calls to be dispatched from the same origin.
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
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    volatileVm: {
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    xdns: {
      /**
       * Inserts a xdns_record into the on-chain registry. Root only access.
       **/
      addNewXdnsRecord: AugmentedSubmittable<(url: Bytes | string | Uint8Array, gatewayId: ChainId | string | Uint8Array, gatewayAbi: GatewayABIConfig | { block_number_type_size?: any; hash_size?: any; hasher?: any; crypto?: any; address_length?: any; value_type_size?: any; decimals?: any; structs?: any } | string | Uint8Array, gatewayVendor: GatewayVendor, gatewayType: GatewayType, gatewayGenesis: GatewayGenesisConfig) => SubmittableExtrinsic<ApiType>, [Bytes, ChainId, GatewayABIConfig, GatewayVendor, GatewayType, GatewayGenesisConfig]>;
      /**
       * Removes a xdns_record from the onchain registry. Root only access.
       **/
      purgeXdnsRecord: AugmentedSubmittable<(requester: AccountId | string | Uint8Array, xdnsRecordId: XdnsRecordId | {  } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [AccountId, XdnsRecordId]>;
      /**
       * Updates the last_finalized field for an xdns_record from the onchain registry. Root only access.
       **/
      updateTtl: AugmentedSubmittable<(gatewayId: ChainId | string | Uint8Array, lastFinalized: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [ChainId, u64]>;
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
