// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

/* eslint-disable sort-keys */

export default {
  /** Lookup3: frame_system::AccountInfo<Index, pallet_balances::AccountData<Balance>> */
  FrameSystemAccountInfo: {
    nonce: "u32",
    consumers: "u32",
    providers: "u32",
    sufficients: "u32",
    data: "PalletBalancesAccountData",
  },
  /** Lookup5: pallet_balances::AccountData<Balance> */
  PalletBalancesAccountData: {
    free: "u128",
    reserved: "u128",
    miscFrozen: "u128",
    feeFrozen: "u128",
  },
  /** Lookup7: frame_support::weights::PerDispatchClass<T> */
  FrameSupportWeightsPerDispatchClassU64: {
    normal: "u64",
    operational: "u64",
    mandatory: "u64",
  },
  /** Lookup11: sp_runtime::generic::digest::Digest */
  SpRuntimeDigest: {
    logs: "Vec<SpRuntimeDigestDigestItem>",
  },
  /** Lookup13: sp_runtime::generic::digest::DigestItem */
  SpRuntimeDigestDigestItem: {
    _enum: {
      Other: "Bytes",
      __Unused1: "Null",
      __Unused2: "Null",
      __Unused3: "Null",
      Consensus: "([u8;4],Bytes)",
      Seal: "([u8;4],Bytes)",
      PreRuntime: "([u8;4],Bytes)",
      __Unused7: "Null",
      RuntimeEnvironmentUpdated: "Null",
    },
  },
  /**
   * Lookup16: frame_system::EventRecord<circuit_standalone_runtime::Event,
   * primitive_types::H256>
   */
  FrameSystemEventRecord: {
    phase: "FrameSystemPhase",
    event: "Event",
    topics: "Vec<H256>",
  },
  /** Lookup18: frame_system::pallet::Event<T> */
  FrameSystemEvent: {
    _enum: {
      ExtrinsicSuccess: {
        dispatchInfo: "FrameSupportWeightsDispatchInfo",
      },
      ExtrinsicFailed: {
        dispatchError: "SpRuntimeDispatchError",
        dispatchInfo: "FrameSupportWeightsDispatchInfo",
      },
      CodeUpdated: "Null",
      NewAccount: {
        account: "AccountId32",
      },
      KilledAccount: {
        account: "AccountId32",
      },
      Remarked: {
        _alias: {
          hash_: "hash",
        },
        sender: "AccountId32",
        hash_: "H256",
      },
    },
  },
  /** Lookup19: frame_support::weights::DispatchInfo */
  FrameSupportWeightsDispatchInfo: {
    weight: "u64",
    class: "FrameSupportWeightsDispatchClass",
    paysFee: "FrameSupportWeightsPays",
  },
  /** Lookup20: frame_support::weights::DispatchClass */
  FrameSupportWeightsDispatchClass: {
    _enum: ["Normal", "Operational", "Mandatory"],
  },
  /** Lookup21: frame_support::weights::Pays */
  FrameSupportWeightsPays: {
    _enum: ["Yes", "No"],
  },
  /** Lookup22: sp_runtime::DispatchError */
  SpRuntimeDispatchError: {
    _enum: {
      Other: "Null",
      CannotLookup: "Null",
      BadOrigin: "Null",
      Module: "SpRuntimeModuleError",
      ConsumerRemaining: "Null",
      NoProviders: "Null",
      TooManyConsumers: "Null",
      Token: "SpRuntimeTokenError",
      Arithmetic: "SpRuntimeArithmeticError",
      Transactional: "SpRuntimeTransactionalError",
    },
  },
  /** Lookup23: sp_runtime::ModuleError */
  SpRuntimeModuleError: {
    index: "u8",
    error: "[u8;4]",
  },
  /** Lookup24: sp_runtime::TokenError */
  SpRuntimeTokenError: {
    _enum: [
      "NoFunds",
      "WouldDie",
      "BelowMinimum",
      "CannotCreate",
      "UnknownAsset",
      "Frozen",
      "Unsupported",
    ],
  },
  /** Lookup25: sp_runtime::ArithmeticError */
  SpRuntimeArithmeticError: {
    _enum: ["Underflow", "Overflow", "DivisionByZero"],
  },
  /** Lookup26: sp_runtime::TransactionalError */
  SpRuntimeTransactionalError: {
    _enum: ["LimitReached", "NoLayer"],
  },
  /** Lookup27: pallet_grandpa::pallet::Event */
  PalletGrandpaEvent: {
    _enum: {
      NewAuthorities: {
        authoritySet: "Vec<(SpFinalityGrandpaAppPublic,u64)>",
      },
      Paused: "Null",
      Resumed: "Null",
    },
  },
  /** Lookup30: sp_finality_grandpa::app::Public */
  SpFinalityGrandpaAppPublic: "SpCoreEd25519Public",
  /** Lookup31: sp_core::ed25519::Public */
  SpCoreEd25519Public: "[u8;32]",
  /** Lookup32: pallet_balances::pallet::Event<T, I> */
  PalletBalancesEvent: {
    _enum: {
      Endowed: {
        account: "AccountId32",
        freeBalance: "u128",
      },
      DustLost: {
        account: "AccountId32",
        amount: "u128",
      },
      Transfer: {
        from: "AccountId32",
        to: "AccountId32",
        amount: "u128",
      },
      BalanceSet: {
        who: "AccountId32",
        free: "u128",
        reserved: "u128",
      },
      Reserved: {
        who: "AccountId32",
        amount: "u128",
      },
      Unreserved: {
        who: "AccountId32",
        amount: "u128",
      },
      ReserveRepatriated: {
        from: "AccountId32",
        to: "AccountId32",
        amount: "u128",
        destinationStatus: "FrameSupportTokensMiscBalanceStatus",
      },
      Deposit: {
        who: "AccountId32",
        amount: "u128",
      },
      Withdraw: {
        who: "AccountId32",
        amount: "u128",
      },
      Slashed: {
        who: "AccountId32",
        amount: "u128",
      },
    },
  },
  /** Lookup33: frame_support::traits::tokens::misc::BalanceStatus */
  FrameSupportTokensMiscBalanceStatus: {
    _enum: ["Free", "Reserved"],
  },
  /** Lookup34: pallet_sudo::pallet::Event<T> */
  PalletSudoEvent: {
    _enum: {
      Sudid: {
        sudoResult: "Result<Null, SpRuntimeDispatchError>",
      },
      KeyChanged: {
        oldSudoer: "Option<AccountId32>",
      },
      SudoAsDone: {
        sudoResult: "Result<Null, SpRuntimeDispatchError>",
      },
    },
  },
  /** Lookup38: pallet_utility::pallet::Event */
  PalletUtilityEvent: {
    _enum: {
      BatchInterrupted: {
        index: "u32",
        error: "SpRuntimeDispatchError",
      },
      BatchCompleted: "Null",
      ItemCompleted: "Null",
      DispatchedAs: {
        result: "Result<Null, SpRuntimeDispatchError>",
      },
    },
  },
  /** Lookup39: orml_tokens::module::Event<T> */
  OrmlTokensModuleEvent: {
    _enum: {
      Endowed: {
        currencyId: "u32",
        who: "AccountId32",
        amount: "u128",
      },
      DustLost: {
        currencyId: "u32",
        who: "AccountId32",
        amount: "u128",
      },
      Transfer: {
        currencyId: "u32",
        from: "AccountId32",
        to: "AccountId32",
        amount: "u128",
      },
      Reserved: {
        currencyId: "u32",
        who: "AccountId32",
        amount: "u128",
      },
      Unreserved: {
        currencyId: "u32",
        who: "AccountId32",
        amount: "u128",
      },
      RepatriatedReserve: {
        currencyId: "u32",
        from: "AccountId32",
        to: "AccountId32",
        amount: "u128",
        status: "FrameSupportTokensMiscBalanceStatus",
      },
      BalanceSet: {
        currencyId: "u32",
        who: "AccountId32",
        free: "u128",
        reserved: "u128",
      },
    },
  },
  /** Lookup40: pallet_xdns::pallet::Event<T> */
  PalletXdnsEvent: {
    _enum: {
      XdnsRecordStored: "[u8;4]",
      XdnsRecordPurged: "(AccountId32,[u8;4])",
      XdnsRecordUpdated: "[u8;4]",
    },
  },
  /** Lookup41: pallet_multi_finality_verifier::pallet::Event<T, I> */
  PalletMultiFinalityVerifierEvent: {
    _enum: {
      NewHeaderRangeAvailable: "([u8;4],u32,u32)",
    },
  },
  /** Lookup46: pallet_contracts_registry::pallet::Event<T> */
  PalletContractsRegistryEvent: {
    _enum: {
      ContractStored: "(AccountId32,H256)",
      ContractPurged: "(AccountId32,H256)",
    },
  },
  /** Lookup47: pallet_circuit_portal::pallet::Event<T> */
  PalletCircuitPortalEvent: {
    _enum: {
      NewGatewayRegistered:
        "([u8;4],T3rnPrimitivesGatewayType,T3rnPrimitivesGatewayVendor,T3rnPrimitivesGatewaySysProps,Vec<[u8;4]>)",
      GatewayUpdated: "([u8;4],Option<Vec<[u8;4]>>)",
    },
  },
  /** Lookup48: t3rn_primitives::GatewayType */
  T3rnPrimitivesGatewayType: {
    _enum: {
      ProgrammableInternal: "u32",
      ProgrammableExternal: "u32",
      TxOnly: "u32",
      OnCircuit: "u32",
    },
  },
  /** Lookup49: t3rn_primitives::GatewayVendor */
  T3rnPrimitivesGatewayVendor: {
    _enum: ["Substrate", "Ethereum", "Rococo"],
  },
  /** Lookup50: t3rn_primitives::GatewaySysProps */
  T3rnPrimitivesGatewaySysProps: {
    ss58Format: "u16",
    tokenSymbol: "Bytes",
    tokenDecimals: "u8",
  },
  /** Lookup54: pallet_circuit::pallet::Event<T> */
  PalletCircuitEvent: {
    _enum: {
      XTransactionReceivedForExec: "H256",
      XTransactionReadyForExec: "H256",
      XTransactionStepFinishedExec: "H256",
      XTransactionXtxFinishedExecAllSteps: "H256",
      XTransactionXtxRevertedAfterTimeOut: "H256",
      NewSideEffectsAvailable:
        "(AccountId32,H256,Vec<T3rnTypesSideEffect>,Vec<H256>)",
      CancelledSideEffects: "(AccountId32,H256,Vec<T3rnTypesSideEffect>)",
      SideEffectsConfirmed:
        "(H256,Vec<Vec<T3rnPrimitivesSideEffectFullSideEffect>>)",
      EscrowTransfer: "(AccountId32,AccountId32,u128)",
    },
  },
  /**
   * Lookup56: t3rn_types::side_effect::SideEffect<sp_core::crypto::AccountId32,
   * BlockNumber, BalanceOf>
   */
  T3rnTypesSideEffect: {
    target: "[u8;4]",
    prize: "u128",
    orderedAt: "u32",
    encodedAction: "Bytes",
    encodedArgs: "Vec<Bytes>",
    signature: "Bytes",
    enforceExecutioner: "Option<AccountId32>",
  },
  /**
   * Lookup61:
   * t3rn_primitives::side_effect::FullSideEffect<sp_core::crypto::AccountId32,
   * BlockNumber, BalanceOf>
   */
  T3rnPrimitivesSideEffectFullSideEffect: {
    input: "T3rnTypesSideEffect",
    confirmed: "Option<T3rnTypesSideEffectConfirmedSideEffect>",
    securityLvl: "T3rnTypesSideEffectSecurityLvl",
    submissionTargetHeight: "Bytes",
  },
  /**
   * Lookup63:
   * t3rn_types::side_effect::ConfirmedSideEffect<sp_core::crypto::AccountId32,
   * BlockNumber, BalanceOf>
   */
  T3rnTypesSideEffectConfirmedSideEffect: {
    err: "Option<T3rnTypesSideEffectConfirmationOutcome>",
    output: "Option<Bytes>",
    inclusionData: "Bytes",
    executioner: "AccountId32",
    receivedAt: "u32",
    cost: "Option<u128>",
  },
  /** Lookup65: t3rn_types::side_effect::ConfirmationOutcome */
  T3rnTypesSideEffectConfirmationOutcome: {
    _enum: {
      Success: "Null",
      MisbehaviourMalformedValues: {
        key: "Bytes",
        expected: "Bytes",
        received: "Bytes",
      },
      TimedOut: "Null",
    },
  },
  /** Lookup68: t3rn_types::side_effect::SecurityLvl */
  T3rnTypesSideEffectSecurityLvl: {
    _enum: ["Dirty", "Optimistic", "Escrowed"],
  },
  /** Lookup69: pallet_portal::pallet::Event<T> */
  PalletPortalEvent: {
    _enum: {
      GatewayRegistered: "[u8;4]",
      SetOwner: "([u8;4],Bytes)",
      SetOperational: "([u8;4],bool)",
      HeaderSubmitted: "([u8;4],Bytes)",
    },
  },
  /** Lookup71: pallet_wasm_contracts::pallet::Event<T> */
  PalletWasmContractsEvent: {
    _enum: {
      Instantiated: {
        deployer: "AccountId32",
        contract: "AccountId32",
      },
      Terminated: {
        contract: "AccountId32",
        beneficiary: "AccountId32",
      },
      CodeStored: {
        codeHash: "H256",
      },
      ContractEmitted: {
        contract: "AccountId32",
        data: "Bytes",
      },
      CodeRemoved: {
        codeHash: "H256",
      },
      ContractCodeUpdated: {
        contract: "AccountId32",
        newCodeHash: "H256",
        oldCodeHash: "H256",
      },
    },
  },
  /** Lookup72: pallet_account_manager::pallet::Event<T> */
  PalletAccountManagerEvent: {
    _enum: {
      DepositReceived: {
        executionId: "u64",
        payee: "AccountId32",
        recipient: "AccountId32",
        amount: "u128",
      },
      ExecutionFinalized: {
        executionId: "u64",
      },
      Issued: {
        recipient: "AccountId32",
        amount: "u128",
      },
    },
  },
  /** Lookup73: frame_system::Phase */
  FrameSystemPhase: {
    _enum: {
      ApplyExtrinsic: "u32",
      Finalization: "Null",
      Initialization: "Null",
    },
  },
  /** Lookup76: frame_system::LastRuntimeUpgradeInfo */
  FrameSystemLastRuntimeUpgradeInfo: {
    specVersion: "Compact<u32>",
    specName: "Text",
  },
  /** Lookup79: frame_system::pallet::Call<T> */
  FrameSystemCall: {
    _enum: {
      fill_block: {
        ratio: "Perbill",
      },
      remark: {
        remark: "Bytes",
      },
      set_heap_pages: {
        pages: "u64",
      },
      set_code: {
        code: "Bytes",
      },
      set_code_without_checks: {
        code: "Bytes",
      },
      set_storage: {
        items: "Vec<(Bytes,Bytes)>",
      },
      kill_storage: {
        _alias: {
          keys_: "keys",
        },
        keys_: "Vec<Bytes>",
      },
      kill_prefix: {
        prefix: "Bytes",
        subkeys: "u32",
      },
      remark_with_event: {
        remark: "Bytes",
      },
    },
  },
  /** Lookup83: frame_system::limits::BlockWeights */
  FrameSystemLimitsBlockWeights: {
    baseBlock: "u64",
    maxBlock: "u64",
    perClass: "FrameSupportWeightsPerDispatchClassWeightsPerClass",
  },
  /**
   * Lookup84:
   * frame_support::weights::PerDispatchClass<frame_system::limits::WeightsPerClass>
   */
  FrameSupportWeightsPerDispatchClassWeightsPerClass: {
    normal: "FrameSystemLimitsWeightsPerClass",
    operational: "FrameSystemLimitsWeightsPerClass",
    mandatory: "FrameSystemLimitsWeightsPerClass",
  },
  /** Lookup85: frame_system::limits::WeightsPerClass */
  FrameSystemLimitsWeightsPerClass: {
    baseExtrinsic: "u64",
    maxExtrinsic: "Option<u64>",
    maxTotal: "Option<u64>",
    reserved: "Option<u64>",
  },
  /** Lookup87: frame_system::limits::BlockLength */
  FrameSystemLimitsBlockLength: {
    max: "FrameSupportWeightsPerDispatchClassU32",
  },
  /** Lookup88: frame_support::weights::PerDispatchClass<T> */
  FrameSupportWeightsPerDispatchClassU32: {
    normal: "u32",
    operational: "u32",
    mandatory: "u32",
  },
  /** Lookup89: frame_support::weights::RuntimeDbWeight */
  FrameSupportWeightsRuntimeDbWeight: {
    read: "u64",
    write: "u64",
  },
  /** Lookup90: sp_version::RuntimeVersion */
  SpVersionRuntimeVersion: {
    specName: "Text",
    implName: "Text",
    authoringVersion: "u32",
    specVersion: "u32",
    implVersion: "u32",
    apis: "Vec<([u8;8],u32)>",
    transactionVersion: "u32",
    stateVersion: "u8",
  },
  /** Lookup95: frame_system::pallet::Error<T> */
  FrameSystemError: {
    _enum: [
      "InvalidSpecName",
      "SpecVersionNeedsToIncrease",
      "FailedToExtractRuntimeVersion",
      "NonDefaultComposite",
      "NonZeroRefCount",
      "CallFiltered",
    ],
  },
  /** Lookup97: pallet_timestamp::pallet::Call<T> */
  PalletTimestampCall: {
    _enum: {
      set: {
        now: "Compact<u64>",
      },
    },
  },
  /** Lookup100: sp_consensus_aura::sr25519::app_sr25519::Public */
  SpConsensusAuraSr25519AppSr25519Public: "SpCoreSr25519Public",
  /** Lookup101: sp_core::sr25519::Public */
  SpCoreSr25519Public: "[u8;32]",
  /** Lookup104: pallet_grandpa::StoredState<N> */
  PalletGrandpaStoredState: {
    _enum: {
      Live: "Null",
      PendingPause: {
        scheduledAt: "u32",
        delay: "u32",
      },
      Paused: "Null",
      PendingResume: {
        scheduledAt: "u32",
        delay: "u32",
      },
    },
  },
  /** Lookup105: pallet_grandpa::StoredPendingChange<N, Limit> */
  PalletGrandpaStoredPendingChange: {
    scheduledAt: "u32",
    delay: "u32",
    nextAuthorities: "Vec<(SpFinalityGrandpaAppPublic,u64)>",
    forced: "Option<u32>",
  },
  /** Lookup108: pallet_grandpa::pallet::Call<T> */
  PalletGrandpaCall: {
    _enum: {
      report_equivocation: {
        equivocationProof: "SpFinalityGrandpaEquivocationProof",
        keyOwnerProof: "SpCoreVoid",
      },
      report_equivocation_unsigned: {
        equivocationProof: "SpFinalityGrandpaEquivocationProof",
        keyOwnerProof: "SpCoreVoid",
      },
      note_stalled: {
        delay: "u32",
        bestFinalizedBlockNumber: "u32",
      },
    },
  },
  /** Lookup109: sp_finality_grandpa::EquivocationProof<primitive_types::H256, N> */
  SpFinalityGrandpaEquivocationProof: {
    setId: "u64",
    equivocation: "SpFinalityGrandpaEquivocation",
  },
  /** Lookup110: sp_finality_grandpa::Equivocation<primitive_types::H256, N> */
  SpFinalityGrandpaEquivocation: {
    _enum: {
      Prevote: "FinalityGrandpaEquivocationPrevote",
      Precommit: "FinalityGrandpaEquivocationPrecommit",
    },
  },
  /**
   * Lookup111: finality_grandpa::Equivocation<sp_finality_grandpa::app::Public,
   * finality_grandpa::Prevote<primitive_types::H256, N>,
   * sp_finality_grandpa::app::Signature>
   */
  FinalityGrandpaEquivocationPrevote: {
    roundNumber: "u64",
    identity: "SpFinalityGrandpaAppPublic",
    first: "(FinalityGrandpaPrevote,SpFinalityGrandpaAppSignature)",
    second: "(FinalityGrandpaPrevote,SpFinalityGrandpaAppSignature)",
  },
  /** Lookup112: finality_grandpa::Prevote<primitive_types::H256, N> */
  FinalityGrandpaPrevote: {
    targetHash: "H256",
    targetNumber: "u32",
  },
  /** Lookup113: sp_finality_grandpa::app::Signature */
  SpFinalityGrandpaAppSignature: "SpCoreEd25519Signature",
  /** Lookup114: sp_core::ed25519::Signature */
  SpCoreEd25519Signature: "[u8;64]",
  /**
   * Lookup117: finality_grandpa::Equivocation<sp_finality_grandpa::app::Public,
   * finality_grandpa::Precommit<primitive_types::H256, N>,
   * sp_finality_grandpa::app::Signature>
   */
  FinalityGrandpaEquivocationPrecommit: {
    roundNumber: "u64",
    identity: "SpFinalityGrandpaAppPublic",
    first: "(FinalityGrandpaPrecommit,SpFinalityGrandpaAppSignature)",
    second: "(FinalityGrandpaPrecommit,SpFinalityGrandpaAppSignature)",
  },
  /** Lookup118: finality_grandpa::Precommit<primitive_types::H256, N> */
  FinalityGrandpaPrecommit: {
    targetHash: "H256",
    targetNumber: "u32",
  },
  /** Lookup120: sp_core::Void */
  SpCoreVoid: "Null",
  /** Lookup121: pallet_grandpa::pallet::Error<T> */
  PalletGrandpaError: {
    _enum: [
      "PauseFailed",
      "ResumeFailed",
      "ChangePending",
      "TooSoon",
      "InvalidKeyOwnershipProof",
      "InvalidEquivocationProof",
      "DuplicateOffenceReport",
    ],
  },
  /** Lookup123: pallet_balances::BalanceLock<Balance> */
  PalletBalancesBalanceLock: {
    id: "[u8;8]",
    amount: "u128",
    reasons: "PalletBalancesReasons",
  },
  /** Lookup124: pallet_balances::Reasons */
  PalletBalancesReasons: {
    _enum: ["Fee", "Misc", "All"],
  },
  /** Lookup127: pallet_balances::ReserveData<ReserveIdentifier, Balance> */
  PalletBalancesReserveData: {
    id: "[u8;8]",
    amount: "u128",
  },
  /** Lookup129: pallet_balances::Releases */
  PalletBalancesReleases: {
    _enum: ["V1_0_0", "V2_0_0"],
  },
  /** Lookup130: pallet_balances::pallet::Call<T, I> */
  PalletBalancesCall: {
    _enum: {
      transfer: {
        dest: "MultiAddress",
        value: "Compact<u128>",
      },
      set_balance: {
        who: "MultiAddress",
        newFree: "Compact<u128>",
        newReserved: "Compact<u128>",
      },
      force_transfer: {
        source: "MultiAddress",
        dest: "MultiAddress",
        value: "Compact<u128>",
      },
      transfer_keep_alive: {
        dest: "MultiAddress",
        value: "Compact<u128>",
      },
      transfer_all: {
        dest: "MultiAddress",
        keepAlive: "bool",
      },
      force_unreserve: {
        who: "MultiAddress",
        amount: "u128",
      },
    },
  },
  /** Lookup135: pallet_balances::pallet::Error<T, I> */
  PalletBalancesError: {
    _enum: [
      "VestingBalance",
      "LiquidityRestrictions",
      "InsufficientBalance",
      "ExistentialDeposit",
      "KeepAlive",
      "ExistingVestingSchedule",
      "DeadAccount",
      "TooManyReserves",
    ],
  },
  /** Lookup137: pallet_transaction_payment::Releases */
  PalletTransactionPaymentReleases: {
    _enum: ["V1Ancient", "V2"],
  },
  /** Lookup139: frame_support::weights::WeightToFeeCoefficient<Balance> */
  FrameSupportWeightsWeightToFeeCoefficient: {
    coeffInteger: "u128",
    coeffFrac: "Perbill",
    negative: "bool",
    degree: "u8",
  },
  /** Lookup140: pallet_sudo::pallet::Call<T> */
  PalletSudoCall: {
    _enum: {
      sudo: {
        call: "Call",
      },
      sudo_unchecked_weight: {
        call: "Call",
        weight: "u64",
      },
      set_key: {
        _alias: {
          new_: "new",
        },
        new_: "MultiAddress",
      },
      sudo_as: {
        who: "MultiAddress",
        call: "Call",
      },
    },
  },
  /** Lookup142: pallet_utility::pallet::Call<T> */
  PalletUtilityCall: {
    _enum: {
      batch: {
        calls: "Vec<Call>",
      },
      as_derivative: {
        index: "u16",
        call: "Call",
      },
      batch_all: {
        calls: "Vec<Call>",
      },
      dispatch_as: {
        asOrigin: "CircuitStandaloneRuntimeOriginCaller",
        call: "Call",
      },
    },
  },
  /** Lookup144: circuit_standalone_runtime::OriginCaller */
  CircuitStandaloneRuntimeOriginCaller: {
    _enum: {
      system: "FrameSupportDispatchRawOrigin",
      Void: "SpCoreVoid",
    },
  },
  /** Lookup145: frame_support::dispatch::RawOrigin<sp_core::crypto::AccountId32> */
  FrameSupportDispatchRawOrigin: {
    _enum: {
      Root: "Null",
      Signed: "AccountId32",
      None: "Null",
    },
  },
  /** Lookup146: pallet_xdns::pallet::Call<T> */
  PalletXdnsCall: {
    _enum: {
      add_new_xdns_record: {
        url: "Bytes",
        gatewayId: "[u8;4]",
        parachain: "Option<T3rnPrimitivesXdnsParachain>",
        gatewayAbi: "T3rnTypesAbiGatewayABIConfig",
        gatewayVendor: "T3rnPrimitivesGatewayVendor",
        gatewayType: "T3rnPrimitivesGatewayType",
        gatewayGenesis: "T3rnPrimitivesGatewayGenesisConfig",
        gatewaySysProps: "T3rnPrimitivesGatewaySysProps",
        allowedSideEffects: "Vec<[u8;4]>",
      },
      add_side_effect: {
        id: "[u8;4]",
        name: "Bytes",
        argumentAbi: "Vec<T3rnTypesAbiType>",
        argumentToStateMapper: "Vec<Bytes>",
        confirmEvents: "Vec<Bytes>",
        escrowedEvents: "Vec<Bytes>",
        commitEvents: "Vec<Bytes>",
        revertEvents: "Vec<Bytes>",
      },
      update_ttl: {
        gatewayId: "[u8;4]",
        lastFinalized: "u64",
      },
      purge_xdns_record: {
        requester: "AccountId32",
        xdnsRecordId: "[u8;4]",
      },
    },
  },
  /** Lookup148: t3rn_primitives::xdns::Parachain */
  T3rnPrimitivesXdnsParachain: {
    relayChainId: "[u8;4]",
    id: "u32",
  },
  /** Lookup149: t3rn_types::abi::GatewayABIConfig */
  T3rnTypesAbiGatewayABIConfig: {
    blockNumberTypeSize: "u16",
    hashSize: "u16",
    hasher: "T3rnTypesAbiHasherAlgo",
    crypto: "T3rnTypesAbiCryptoAlgo",
    addressLength: "u16",
    valueTypeSize: "u16",
    decimals: "u16",
    structs: "Vec<T3rnTypesAbiStructDecl>",
  },
  /** Lookup150: t3rn_types::abi::HasherAlgo */
  T3rnTypesAbiHasherAlgo: {
    _enum: ["Blake2", "Keccak256"],
  },
  /** Lookup151: t3rn_types::abi::CryptoAlgo */
  T3rnTypesAbiCryptoAlgo: {
    _enum: ["Ed25519", "Sr25519", "Ecdsa"],
  },
  /** Lookup153: t3rn_types::abi::StructDecl */
  T3rnTypesAbiStructDecl: {
    name: "T3rnTypesAbiType",
    fields: "Vec<T3rnTypesAbiParameter>",
    offsets: "Vec<u16>",
  },
  /** Lookup154: t3rn_types::abi::Type */
  T3rnTypesAbiType: {
    _enum: {
      Address: "u16",
      DynamicAddress: "Null",
      Bool: "Null",
      Int: "u16",
      Uint: "u16",
      Bytes: "u8",
      DynamicBytes: "Null",
      String: "Null",
      Enum: "u8",
      Struct: "u8",
      Mapping: "(T3rnTypesAbiType,T3rnTypesAbiType)",
      Contract: "Null",
      Ref: "T3rnTypesAbiType",
      Option: "T3rnTypesAbiType",
      OptionalInsurance: "Null",
      OptionalReward: "Null",
      StorageRef: "T3rnTypesAbiType",
      Value: "Null",
      Slice: "Null",
      Hasher: "(T3rnTypesAbiHasherAlgo,u16)",
      Crypto: "T3rnTypesAbiCryptoAlgo",
    },
  },
  /** Lookup156: t3rn_types::abi::Parameter */
  T3rnTypesAbiParameter: {
    name: "Option<Bytes>",
    ty: "T3rnTypesAbiType",
    no: "u32",
    indexed: "Option<bool>",
  },
  /** Lookup159: t3rn_primitives::GatewayGenesisConfig */
  T3rnPrimitivesGatewayGenesisConfig: {
    modulesEncoded: "Option<Bytes>",
    extrinsicsVersion: "u8",
    genesisHash: "Bytes",
  },
  /** Lookup161: pallet_multi_finality_verifier::pallet::Call<T, I> */
  PalletMultiFinalityVerifierCall: {
    _enum: {
      submit_finality_proof: {
        finalityTarget: {
          parentHash: "H256",
          number: "Compact<u32>",
          stateRoot: "H256",
          extrinsicsRoot: "H256",
          digest: "SpRuntimeDigest",
        },
        encodedJustification: "Bytes",
        gatewayId: "[u8;4]",
      },
      submit_header_range: {
        gatewayId: "[u8;4]",
        headersReversed:
          'Vec<{"parentHash":"H256","number":"Compact<u32>","stateRoot":"H256","extrinsicsRoot":"H256","digest":"SpRuntimeDigest"}>',
        anchorHeaderHash: "H256",
      },
      initialize_single: {
        initData: {
          header: {
            parentHash: "H256",
            number: "Compact<u32>",
            stateRoot: "H256",
            extrinsicsRoot: "H256",
            digest: "SpRuntimeDigest",
          },
          authorityList: "Vec<(SpFinalityGrandpaAppPublic,u64)>",
          setId: "u64",
          isHalted: "bool",
          gatewayId: "[u8;4]",
        },
      },
      set_owner: {
        newOwner: "Option<AccountId32>",
        gatewayId: "[u8;4]",
      },
      set_operational: {
        operational: "bool",
        gatewayId: "[u8;4]",
      },
    },
  },
  /** Lookup163: sp_runtime::traits::BlakeTwo256 */
  SpRuntimeBlakeTwo256: "Null",
  /** Lookup168: sp_runtime::generic::header::Header<Number, sp_runtime::traits::Keccak256> */
  SpRuntimeHeaderU64: {
    parentHash: "H256",
    number: "Compact<u64>",
    stateRoot: "H256",
    extrinsicsRoot: "H256",
    digest: "SpRuntimeDigest",
  },
  /** Lookup169: sp_runtime::traits::Keccak256 */
  SpRuntimeKeccak256: "Null",
  /** Lookup173: sp_runtime::generic::header::Header<Number, sp_runtime::traits::Keccak256> */
  SpRuntimeHeaderU32: {
    parentHash: "H256",
    number: "Compact<u32>",
    stateRoot: "H256",
    extrinsicsRoot: "H256",
    digest: "SpRuntimeDigest",
  },
  /**
   * Lookup175:
   * t3rn_primitives::bridges::header_chain::InitializationData<sp_runtime::generic::header::Header<Number,
   * sp_runtime::traits::Keccak256>>
   */
  T3rnPrimitivesBridgesHeaderChainInitializationData: {
    header: "SpRuntimeHeaderU32",
    authorityList: "Vec<(SpFinalityGrandpaAppPublic,u64)>",
    setId: "u64",
    isHalted: "bool",
    gatewayId: "[u8;4]",
  },
  /** Lookup177: pallet_contracts_registry::pallet::Call<T> */
  PalletContractsRegistryCall: {
    _enum: {
      add_new_contract: {
        requester: "AccountId32",
        contract: "T3rnPrimitivesContractsRegistryRegistryContract",
      },
      purge: {
        requester: "AccountId32",
        contractId: "H256",
      },
    },
  },
  /**
   * Lookup178:
   * t3rn_primitives::contracts_registry::RegistryContract<primitive_types::H256,
   * sp_core::crypto::AccountId32, BalanceOf, BlockNumber>
   */
  T3rnPrimitivesContractsRegistryRegistryContract: {
    codeTxt: "Bytes",
    bytes: "Bytes",
    author: "T3rnPrimitivesContractsRegistryAuthorInfo",
    abi: "Option<Bytes>",
    actionDescriptions: "Vec<T3rnTypesAbiContractActionDesc>",
    info: "Option<T3rnPrimitivesStorageRawAliveContractInfo>",
    meta: "T3rnPrimitivesContractMetadata",
  },
  /**
   * Lookup179:
   * t3rn_primitives::contracts_registry::AuthorInfo<sp_core::crypto::AccountId32,
   * BalanceOf>
   */
  T3rnPrimitivesContractsRegistryAuthorInfo: {
    account: "AccountId32",
    feesPerSingleUse: "Option<u128>",
  },
  /**
   * Lookup181: t3rn_types::abi::ContractActionDesc<primitive_types::H256,
   * TargetId, sp_core::crypto::AccountId32>
   */
  T3rnTypesAbiContractActionDesc: {
    actionId: "H256",
    targetId: "Option<[u8;4]>",
    to: "Option<AccountId32>",
  },
  /**
   * Lookup184:
   * t3rn_primitives::storage::RawAliveContractInfo<primitive_types::H256,
   * Balance, BlockNumber>
   */
  T3rnPrimitivesStorageRawAliveContractInfo: {
    trieId: "Bytes",
    storageSize: "u32",
    pairCount: "u32",
    codeHash: "H256",
    rentAllowance: "u128",
    rentPaid: "u128",
    deductBlock: "u32",
    lastWrite: "Option<u32>",
    reserved: "Option<Null>",
  },
  /** Lookup186: t3rn_primitives::contract_metadata::ContractMetadata */
  T3rnPrimitivesContractMetadata: {
    metadataVersion: "Bytes",
    name: "Bytes",
    contractType: "Bytes",
    version: "Bytes",
    authors: "Vec<Bytes>",
    description: "Option<Bytes>",
    documentation: "Option<Bytes>",
    repository: "Option<Bytes>",
    homepage: "Option<Bytes>",
    license: "Option<Bytes>",
  },
  /** Lookup187: pallet_circuit_portal::pallet::Call<T> */
  PalletCircuitPortalCall: {
    _enum: {
      register_gateway: {
        url: "Bytes",
        gatewayId: "[u8;4]",
        parachain: "Option<T3rnPrimitivesXdnsParachain>",
        gatewayAbi: "T3rnTypesAbiGatewayABIConfig",
        gatewayVendor: "T3rnPrimitivesGatewayVendor",
        gatewayType: "T3rnPrimitivesGatewayType",
        gatewayGenesis: "T3rnPrimitivesGatewayGenesisConfig",
        gatewaySysProps: "T3rnPrimitivesGatewaySysProps",
        firstHeader: "Bytes",
        authorities: "Option<Vec<AccountId32>>",
        authoritySetId: "Option<u64>",
        allowedSideEffects: "Vec<[u8;4]>",
      },
      submit_parachain_header: {
        blockHash: "Bytes",
        gatewayId: "[u8;4]",
        proof: "Vec<Bytes>",
      },
      update_gateway: {
        gatewayId: "[u8;4]",
        url: "Option<Bytes>",
        gatewayAbi: "Option<T3rnTypesAbiGatewayABIConfig>",
        gatewaySysProps: "Option<T3rnPrimitivesGatewaySysProps>",
        authorities: "Option<Vec<AccountId32>>",
        allowedSideEffects: "Option<Vec<[u8;4]>>",
      },
    },
  },
  /** Lookup192: pallet_circuit::pallet::Call<T> */
  PalletCircuitCall: {
    _enum: {
      on_local_trigger: {
        trigger: "Bytes",
      },
      on_xcm_trigger: "Null",
      on_remote_gateway_trigger: "Null",
      on_extrinsic_trigger: {
        sideEffects: "Vec<T3rnTypesSideEffect>",
        fee: "u128",
        sequential: "bool",
      },
      bond_insurance_deposit: {
        xtxId: "H256",
        sideEffectId: "H256",
      },
      execute_side_effects_via_circuit: {
        xtxId: "H256",
        sideEffect: "T3rnTypesSideEffect",
      },
      confirm_commit_revert_relay: {
        xtxId: "H256",
        sideEffect: "T3rnTypesSideEffect",
        confirmation: "T3rnTypesSideEffectConfirmedSideEffect",
      },
      confirm_side_effect: {
        xtxId: "H256",
        sideEffect: "T3rnTypesSideEffect",
        confirmation: "T3rnTypesSideEffectConfirmedSideEffect",
        inclusionProof: "Option<Vec<Bytes>>",
        blockHash: "Option<Bytes>",
      },
    },
  },
  /** Lookup194: pallet_portal::pallet::Call<T> */
  PalletPortalCall: {
    _enum: {
      register_gateway: {
        url: "Bytes",
        gatewayId: "[u8;4]",
        gatewayAbi: "T3rnTypesAbiGatewayABIConfig",
        gatewayVendor: "T3rnPrimitivesGatewayVendor",
        gatewayType: "T3rnPrimitivesGatewayType",
        gatewayGenesis: "T3rnPrimitivesGatewayGenesisConfig",
        gatewaySysProps: "T3rnPrimitivesGatewaySysProps",
        allowedSideEffects: "Vec<[u8;4]>",
        encodedRegistrationData: "Bytes",
      },
      set_owner: {
        gatewayId: "[u8;4]",
        encodedNewOwner: "Bytes",
      },
      set_operational: {
        gatewayId: "[u8;4]",
        operational: "bool",
      },
      submit_headers: {
        gatewayId: "[u8;4]",
        encodedHeaderData: "Bytes",
      },
    },
  },
  /** Lookup195: pallet_wasm_contracts::pallet::Call<T> */
  PalletWasmContractsCall: {
    _enum: {
      call: {
        dest: "MultiAddress",
        value: "Compact<u128>",
        gasLimit: "Compact<u64>",
        storageDepositLimit: "Option<Compact<u128>>",
        data: "Bytes",
      },
      instantiate_with_code: {
        value: "Compact<u128>",
        gasLimit: "Compact<u64>",
        storageDepositLimit: "Option<Compact<u128>>",
        code: "Bytes",
        data: "Bytes",
        salt: "Bytes",
      },
      instantiate: {
        value: "Compact<u128>",
        gasLimit: "Compact<u64>",
        storageDepositLimit: "Option<Compact<u128>>",
        codeHash: "H256",
        data: "Bytes",
        salt: "Bytes",
      },
      upload_code: {
        code: "Bytes",
        storageDepositLimit: "Option<Compact<u128>>",
      },
      remove_code: {
        codeHash: "H256",
      },
    },
  },
  /** Lookup197: pallet_account_manager::pallet::Call<T> */
  PalletAccountManagerCall: {
    _enum: {
      deposit: {
        payee: "AccountId32",
        recipient: "AccountId32",
        amount: "u128",
      },
      finalize: {
        executionId: "u64",
        reason: "Option<T3rnPrimitivesAccountManagerReason>",
      },
    },
  },
  /** Lookup199: t3rn_primitives::account_manager::Reason */
  T3rnPrimitivesAccountManagerReason: {
    _enum: ["UnexpectedFailure", "ContractReverted"],
  },
  /** Lookup200: pallet_sudo::pallet::Error<T> */
  PalletSudoError: {
    _enum: ["RequireSudo"],
  },
  /** Lookup201: pallet_utility::pallet::Error<T> */
  PalletUtilityError: {
    _enum: ["TooManyCalls"],
  },
  /** Lookup204: orml_tokens::BalanceLock<Balance> */
  OrmlTokensBalanceLock: {
    id: "[u8;8]",
    amount: "u128",
  },
  /** Lookup206: orml_tokens::AccountData<Balance> */
  OrmlTokensAccountData: {
    free: "u128",
    reserved: "u128",
    frozen: "u128",
  },
  /** Lookup207: orml_tokens::module::Error<T> */
  OrmlTokensModuleError: {
    _enum: [
      "BalanceTooLow",
      "AmountIntoBalanceFailed",
      "LiquidityRestrictions",
      "MaxLocksExceeded",
      "KeepAlive",
      "ExistentialDeposit",
      "DeadAccount",
    ],
  },
  /** Lookup208: t3rn_primitives::side_effect::interface::SideEffectInterface */
  T3rnPrimitivesSideEffectInterfaceSideEffectInterface: {
    id: "[u8;4]",
    name: "Bytes",
    argumentAbi: "Vec<T3rnTypesAbiType>",
    argumentToStateMapper: "Vec<Bytes>",
    confirmEvents: "Vec<Bytes>",
    escrowedEvents: "Vec<Bytes>",
    commitEvents: "Vec<Bytes>",
    revertEvents: "Vec<Bytes>",
  },
  /** Lookup209: t3rn_primitives::xdns::XdnsRecord<sp_core::crypto::AccountId32> */
  T3rnPrimitivesXdnsXdnsRecord: {
    url: "Bytes",
    gatewayAbi: "T3rnTypesAbiGatewayABIConfig",
    gatewayGenesis: "T3rnPrimitivesGatewayGenesisConfig",
    gatewayVendor: "T3rnPrimitivesGatewayVendor",
    gatewayType: "T3rnPrimitivesGatewayType",
    gatewayId: "[u8;4]",
    parachain: "Option<T3rnPrimitivesXdnsParachain>",
    gatewaySysProps: "T3rnPrimitivesGatewaySysProps",
    registrant: "Option<AccountId32>",
    lastFinalized: "Option<u64>",
    allowedSideEffects: "Vec<[u8;4]>",
  },
  /** Lookup210: pallet_xdns::pallet::Error<T> */
  PalletXdnsError: {
    _enum: [
      "XdnsRecordAlreadyExists",
      "UnknownXdnsRecord",
      "XdnsRecordNotFound",
      "SideEffectInterfaceAlreadyExists",
    ],
  },
  /** Lookup214: t3rn_primitives::bridges::header_chain::AuthoritySet */
  T3rnPrimitivesBridgesHeaderChainAuthoritySet: {
    authorities: "Vec<(SpFinalityGrandpaAppPublic,u64)>",
    setId: "u64",
  },
  /** Lookup215: pallet_multi_finality_verifier::pallet::Error<T, I> */
  PalletMultiFinalityVerifierError: {
    _enum: [
      "InvalidJustification",
      "InvalidAuthoritySet",
      "TooManyRequests",
      "OldHeader",
      "UnknownHeader",
      "UnsupportedScheduledChange",
      "AlreadyInitialized",
      "Halted",
      "StorageRootMismatch",
      "InvalidAnchorHeader",
      "NoFinalizedHeader",
      "NoParachainEntryFound",
    ],
  },
  /** Lookup220: pallet_contracts_registry::pallet::Error<T> */
  PalletContractsRegistryError: {
    _enum: ["ContractAlreadyExists", "UnknownContract"],
  },
  /** Lookup221: pallet_circuit_portal::pallet::Error<T> */
  PalletCircuitPortalError: {
    _enum: [
      "InvalidKey",
      "IOScheduleNoEndingSemicolon",
      "IOScheduleEmpty",
      "IOScheduleUnknownCompose",
      "ProcessStepGatewayNotRecognised",
      "StepConfirmationBlockUnrecognised",
      "StepConfirmationGatewayNotRecognised",
      "SideEffectConfirmationInvalidInclusionProof",
      "VendorUnknown",
      "SideEffectTypeNotRecognized",
      "StepConfirmationDecodingError",
      "ReadLatestTargetHashError",
      "ReadTargetHeightDecodeCmpHeightError",
      "ReadTargetHeightDecodeBlockHashError",
      "ReadTargetHeightReplayAttackDetected",
      "ReadTargetHeightError",
      "ContractDoesNotExists",
      "RequesterNotEnoughBalance",
      "ParachainHeaderNotVerified",
      "NoParachainEntryFound",
      "StorageProofDecodingError",
      "BridgedHeaderDecodingError",
    ],
  },
  /**
   * Lookup222:
   * pallet_circuit::state::InsuranceDeposit<sp_core::crypto::AccountId32,
   * BlockNumber, BalanceOf>
   */
  PalletCircuitStateInsuranceDeposit: {
    insurance: "u128",
    reward: "u128",
    requester: "AccountId32",
    bondedRelayer: "Option<AccountId32>",
    status: "PalletCircuitStateCircuitStatus",
    requestedAt: "u32",
  },
  /** Lookup223: pallet_circuit::state::CircuitStatus */
  PalletCircuitStateCircuitStatus: {
    _enum: [
      "Requested",
      "PendingInsurance",
      "Bonded",
      "Ready",
      "PendingExecution",
      "Finished",
      "FinishedAllSteps",
      "RevertTimedOut",
      "RevertKill",
      "Committed",
      "Reverted",
    ],
  },
  /**
   * Lookup225: pallet_circuit::state::XExecSignal<sp_core::crypto::AccountId32,
   * BlockNumber, BalanceOf>
   */
  PalletCircuitStateXExecSignal: {
    requester: "AccountId32",
    timeoutsAt: "u32",
    delayStepsAt: "Option<Vec<u32>>",
    status: "PalletCircuitStateCircuitStatus",
    stepsCnt: "(u32,u32)",
    totalReward: "Option<u128>",
  },
  /** Lookup228: t3rn_primitives::volatile::LocalState */
  T3rnPrimitivesVolatileLocalState: {
    state: "BTreeMap<[u8;32], Bytes>",
  },
  /** Lookup234: t3rn_sdk_primitives::signal::ExecutionSignal<primitive_types::H256> */
  T3rnSdkPrimitivesSignalExecutionSignal: {
    step: "u32",
    kind: "T3rnSdkPrimitivesSignalSignalKind",
    executionId: "H256",
  },
  /** Lookup235: t3rn_sdk_primitives::signal::SignalKind */
  T3rnSdkPrimitivesSignalSignalKind: {
    _enum: {
      Complete: "Null",
      Kill: "T3rnSdkPrimitivesSignalKillReason",
    },
  },
  /** Lookup236: t3rn_sdk_primitives::signal::KillReason */
  T3rnSdkPrimitivesSignalKillReason: {
    _enum: ["Unhandled", "Codec", "Timeout"],
  },
  /** Lookup238: frame_support::PalletId */
  FrameSupportPalletId: "[u8;8]",
  /** Lookup239: pallet_circuit::pallet::Error<T> */
  PalletCircuitError: {
    _enum: [
      "ApplyTriggeredWithUnexpectedStatus",
      "RequesterNotEnoughBalance",
      "ContractXtxKilledRunOutOfFunds",
      "ChargingTransferFailed",
      "RewardTransferFailed",
      "RefundTransferFailed",
      "SideEffectsValidationFailed",
      "InsuranceBondNotRequired",
      "InsuranceBondAlreadyDeposited",
      "SetupFailed",
      "SetupFailedXtxNotFound",
      "SetupFailedXtxStorageArtifactsNotFound",
      "SetupFailedIncorrectXtxStatus",
      "EnactSideEffectsCanOnlyBeCalledWithMin1StepFinished",
      "FatalXtxTimeoutXtxIdNotMatched",
      "RelayEscrowedFailedNothingToConfirm",
      "FatalCommitSideEffectWithoutConfirmationAttempt",
      "FatalErroredCommitSideEffectConfirmationAttempt",
      "FatalErroredRevertSideEffectConfirmationAttempt",
      "SetupFailedUnknownXtx",
      "FailedToHardenFullSideEffect",
      "SetupFailedDuplicatedXtx",
      "SetupFailedEmptyXtx",
      "ApplyFailed",
      "DeterminedForbiddenXtxStatus",
      "LocalSideEffectExecutionNotApplicable",
      "UnsupportedRole",
      "InvalidLocalTrigger",
      "SignalQueueFull",
    ],
  },
  /** Lookup240: pallet_portal::pallet::Error<T> */
  PalletPortalError: {
    _enum: [
      "XdnsRecordCreationFailed",
      "UnimplementedGatewayVendor",
      "RegistrationError",
      "GatewayVendorNotFound",
      "SetOwnerError",
      "SetOperationalError",
      "SubmitHeaderError",
      "NoGatewayHeightAvailable",
      "SideEffectConfirmationFailed",
    ],
  },
  /** Lookup241: pallet_grandpa_finality_verifier::types::Parachain */
  PalletGrandpaFinalityVerifierParachain: {
    relayChainId: "[u8;4]",
    id: "u32",
  },
  /** Lookup242: pallet_grandpa_finality_verifier::pallet::Error<T, I> */
  PalletGrandpaFinalityVerifierError: {
    _enum: [
      "InvalidJustification",
      "InvalidAuthoritySet",
      "TooManyRequests",
      "OldHeader",
      "UnknownHeader",
      "UnsupportedScheduledChange",
      "AlreadyInitialized",
      "Halted",
      "StorageRootMismatch",
      "InvalidAnchorHeader",
      "NoFinalizedHeader",
      "NoParachainEntryFound",
    ],
  },
  /** Lookup243: pallet_wasm_contracts::wasm::PrefabWasmModule<T> */
  PalletWasmContractsWasmPrefabWasmModule: {
    instructionWeightsVersion: "Compact<u32>",
    initial: "Compact<u32>",
    maximum: "Compact<u32>",
    code: "Bytes",
  },
  /** Lookup244: pallet_wasm_contracts::wasm::OwnerInfo<T> */
  PalletWasmContractsWasmOwnerInfo: {
    owner: "AccountId32",
    deposit: "Compact<u128>",
    refcount: "Compact<u64>",
  },
  /**
   * Lookup245:
   * pallet_wasm_contracts::storage::RawContractInfo<primitive_types::H256, Balance>
   */
  PalletWasmContractsStorageRawContractInfo: {
    kind: "PalletWasmContractsContractKind",
    trieId: "Bytes",
    codeHash: "H256",
    storageDeposit: "u128",
  },
  /** Lookup246: pallet_wasm_contracts::ContractKind */
  PalletWasmContractsContractKind: {
    _enum: ["Pallet", "System", "Registry"],
  },
  /** Lookup248: pallet_wasm_contracts::storage::DeletedContract */
  PalletWasmContractsStorageDeletedContract: {
    trieId: "Bytes",
  },
  /** Lookup250: pallet_wasm_contracts::schedule::Schedule<T> */
  PalletWasmContractsSchedule: {
    limits: "PalletWasmContractsScheduleLimits",
    instructionWeights: "PalletWasmContractsScheduleInstructionWeights",
    hostFnWeights: "PalletWasmContractsScheduleHostFnWeights",
  },
  /** Lookup251: pallet_wasm_contracts::schedule::Limits */
  PalletWasmContractsScheduleLimits: {
    eventTopics: "u32",
    stackHeight: "u32",
    globals: "u32",
    parameters: "u32",
    memoryPages: "u32",
    tableSize: "u32",
    brTableSize: "u32",
    subjectLen: "u32",
    callDepth: "u32",
    payloadLen: "u32",
    codeLen: "u32",
  },
  /** Lookup252: pallet_wasm_contracts::schedule::InstructionWeights<T> */
  PalletWasmContractsScheduleInstructionWeights: {
    _alias: {
      r_if: "r#if",
    },
    version: "u32",
    i64const: "u32",
    i64load: "u32",
    i64store: "u32",
    select: "u32",
    r_if: "u32",
    br: "u32",
    brIf: "u32",
    brTable: "u32",
    brTablePerEntry: "u32",
    call: "u32",
    callIndirect: "u32",
    callIndirectPerParam: "u32",
    localGet: "u32",
    localSet: "u32",
    localTee: "u32",
    globalGet: "u32",
    globalSet: "u32",
    memoryCurrent: "u32",
    memoryGrow: "u32",
    i64clz: "u32",
    i64ctz: "u32",
    i64popcnt: "u32",
    i64eqz: "u32",
    i64extendsi32: "u32",
    i64extendui32: "u32",
    i32wrapi64: "u32",
    i64eq: "u32",
    i64ne: "u32",
    i64lts: "u32",
    i64ltu: "u32",
    i64gts: "u32",
    i64gtu: "u32",
    i64les: "u32",
    i64leu: "u32",
    i64ges: "u32",
    i64geu: "u32",
    i64add: "u32",
    i64sub: "u32",
    i64mul: "u32",
    i64divs: "u32",
    i64divu: "u32",
    i64rems: "u32",
    i64remu: "u32",
    i64and: "u32",
    i64or: "u32",
    i64xor: "u32",
    i64shl: "u32",
    i64shrs: "u32",
    i64shru: "u32",
    i64rotl: "u32",
    i64rotr: "u32",
  },
  /** Lookup253: pallet_wasm_contracts::schedule::HostFnWeights<T> */
  PalletWasmContractsScheduleHostFnWeights: {
    _alias: {
      r_return: "r#return",
    },
    caller: "u64",
    isContract: "u64",
    callerIsOrigin: "u64",
    address: "u64",
    gasLeft: "u64",
    balance: "u64",
    valueTransferred: "u64",
    minimumBalance: "u64",
    blockNumber: "u64",
    now: "u64",
    weightToFee: "u64",
    gas: "u64",
    input: "u64",
    inputPerByte: "u64",
    r_return: "u64",
    returnPerByte: "u64",
    terminate: "u64",
    random: "u64",
    depositEvent: "u64",
    depositEventPerTopic: "u64",
    depositEventPerByte: "u64",
    debugMessage: "u64",
    setStorage: "u64",
    setStoragePerNewByte: "u64",
    setStoragePerOldByte: "u64",
    setCodeHash: "u64",
    clearStorage: "u64",
    clearStoragePerByte: "u64",
    containsStorage: "u64",
    containsStoragePerByte: "u64",
    getStorage: "u64",
    getStoragePerByte: "u64",
    takeStorage: "u64",
    takeStoragePerByte: "u64",
    transfer: "u64",
    call: "u64",
    delegateCall: "u64",
    callTransferSurcharge: "u64",
    callPerClonedByte: "u64",
    instantiate: "u64",
    instantiateTransferSurcharge: "u64",
    instantiatePerSaltByte: "u64",
    hashSha2256: "u64",
    hashSha2256PerByte: "u64",
    hashKeccak256: "u64",
    hashKeccak256PerByte: "u64",
    hashBlake2256: "u64",
    hashBlake2256PerByte: "u64",
    hashBlake2128: "u64",
    hashBlake2128PerByte: "u64",
    ecdsaRecover: "u64",
  },
  /** Lookup254: pallet_wasm_contracts::pallet::Error<T> */
  PalletWasmContractsError: {
    _enum: [
      "InvalidScheduleVersion",
      "InvalidCallFlags",
      "OutOfGas",
      "OutputBufferTooSmall",
      "TransferFailed",
      "MaxCallDepthReached",
      "ContractNotFound",
      "CodeTooLarge",
      "CodeNotFound",
      "OutOfBounds",
      "DecodingFailed",
      "ContractTrapped",
      "ValueTooLarge",
      "TerminatedWhileReentrant",
      "InputForwarded",
      "RandomSubjectTooLong",
      "TooManyTopics",
      "DuplicateTopics",
      "NoChainExtension",
      "DeletionQueueFull",
      "DuplicateContract",
      "TerminatedInConstructor",
      "DebugMessageInvalidUTF8",
      "ReentranceDenied",
      "StorageDepositNotEnoughFunds",
      "StorageDepositLimitExhausted",
      "CodeInUse",
      "ContractReverted",
      "CodeRejected",
      "NotAllowedInVolatileMode",
      "InvalidSideEffect",
      "CannotTriggerWithoutSideEffects",
      "CircuitStepHandlingFailed",
      "ExceededSignalBounceThreshold",
      "Noop",
      "ContractCannotBeInstantiated",
      "CannotTriggerSideEffects",
      "DoesNotHaveStorage",
      "AuthorNotRemuneratable",
    ],
  },
  /**
   * Lookup255:
   * t3rn_primitives::account_manager::ExecutionRegistryItem<sp_core::crypto::AccountId32,
   * Balance>
   */
  T3rnPrimitivesAccountManagerExecutionRegistryItem: {
    payee: "AccountId32",
    recipient: "AccountId32",
    balance: "u128",
  },
  /** Lookup256: pallet_account_manager::pallet::Error<T> */
  PalletAccountManagerError: {
    _enum: ["ExecutionNotRegistered", "ExecutionAlreadyRegistered"],
  },
  /** Lookup258: sp_runtime::MultiSignature */
  SpRuntimeMultiSignature: {
    _enum: {
      Ed25519: "SpCoreEd25519Signature",
      Sr25519: "SpCoreSr25519Signature",
      Ecdsa: "SpCoreEcdsaSignature",
    },
  },
  /** Lookup259: sp_core::sr25519::Signature */
  SpCoreSr25519Signature: "[u8;64]",
  /** Lookup260: sp_core::ecdsa::Signature */
  SpCoreEcdsaSignature: "[u8;65]",
  /** Lookup263: frame_system::extensions::check_non_zero_sender::CheckNonZeroSender<T> */
  FrameSystemExtensionsCheckNonZeroSender: "Null",
  /** Lookup264: frame_system::extensions::check_spec_version::CheckSpecVersion<T> */
  FrameSystemExtensionsCheckSpecVersion: "Null",
  /** Lookup265: frame_system::extensions::check_tx_version::CheckTxVersion<T> */
  FrameSystemExtensionsCheckTxVersion: "Null",
  /** Lookup266: frame_system::extensions::check_genesis::CheckGenesis<T> */
  FrameSystemExtensionsCheckGenesis: "Null",
  /** Lookup269: frame_system::extensions::check_nonce::CheckNonce<T> */
  FrameSystemExtensionsCheckNonce: "Compact<u32>",
  /** Lookup270: frame_system::extensions::check_weight::CheckWeight<T> */
  FrameSystemExtensionsCheckWeight: "Null",
  /** Lookup271: pallet_transaction_payment::ChargeTransactionPayment<T> */
  PalletTransactionPaymentChargeTransactionPayment: "Compact<u128>",
  /** Lookup272: circuit_standalone_runtime::Runtime */
  CircuitStandaloneRuntimeRuntime: "Null",
};
