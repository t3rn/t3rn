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
  /** Lookup41: pallet_contracts_registry::pallet::Event<T> */
  PalletContractsRegistryEvent: {
    _enum: {
      ContractStored: "(AccountId32,H256)",
      ContractPurged: "(AccountId32,H256)",
    },
  },
  /** Lookup42: pallet_circuit::pallet::Event<T> */
  PalletCircuitEvent: {
    _enum: {
      Transfer: "(AccountId32,AccountId32,AccountId32,u128)",
      TransferAssets: "(AccountId32,u64,AccountId32,AccountId32,u128)",
      TransferORML: "(AccountId32,u64,AccountId32,AccountId32,u128)",
      AddLiquidity: "(AccountId32,u64,u64,u128,u128,u128)",
      Swap: "(AccountId32,u64,u64,u128,u128,u128)",
      CallNative: "(AccountId32,Bytes)",
      CallEvm:
        "(AccountId32,H160,H160,U256,Bytes,u64,U256,Option<U256>,Option<U256>,Vec<(H160,Vec<H256>)>)",
      CallWasm: "(AccountId32,AccountId32,u128,u64,Option<u128>,Bytes)",
      CallCustom: "(AccountId32,AccountId32,AccountId32,u128,Bytes,u64,Bytes)",
      Notification:
        "(AccountId32,AccountId32,PalletXbiPortalXbiFormatXbiNotificationKind,Bytes,Bytes)",
      Result:
        "(AccountId32,AccountId32,PalletXbiPortalXbiFormatXbiCheckOutStatus,Bytes,Bytes)",
      XTransactionReceivedForExec: "H256",
      SideEffectInsuranceReceived: "(H256,AccountId32)",
      SideEffectConfirmed: "H256",
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
  /** Lookup52: pallet_xbi_portal::xbi_format::XBINotificationKind */
  PalletXbiPortalXbiFormatXbiNotificationKind: {
    _enum: ["Sent", "Delivered", "Executed"],
  },
  /** Lookup53: pallet_xbi_portal::xbi_format::XBICheckOutStatus */
  PalletXbiPortalXbiFormatXbiCheckOutStatus: {
    _enum: [
      "SuccessfullyExecuted",
      "ErrorFailedExecution",
      "ErrorFailedXCMDispatch",
      "ErrorDeliveryTimeout",
      "ErrorExecutionTimeout",
    ],
  },
  /**
   * Lookup55: t3rn_types::side_effect::SideEffect<sp_core::crypto::AccountId32,
   * BlockNumber, BalanceOf>
   */
  T3rnTypesSideEffect: {
    target: "[u8;4]",
    prize: "u128",
    encodedAction: "Bytes",
    encodedArgs: "Vec<Bytes>",
    orderedAt: "u32",
    signature: "Bytes",
    enforceExecutioner: "Option<AccountId32>",
  },
  /**
   * Lookup59:
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
   * Lookup61:
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
  /** Lookup63: t3rn_types::side_effect::ConfirmationOutcome */
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
  /** Lookup65: t3rn_types::side_effect::SecurityLvl */
  T3rnTypesSideEffectSecurityLvl: {
    _enum: ["Dirty", "Optimistic", "Escrowed"],
  },
  /** Lookup66: pallet_treasury::pallet::Event<T> */
  PalletTreasuryEvent: {
    _enum: {
      NewRound: {
        round: "u32",
        head: "u32",
      },
      RoundTermChanged: {
        _alias: {
          new_: "new",
        },
        old: "u32",
        new_: "u32",
        roundMin: "Perbill",
        roundIdeal: "Perbill",
        roundMax: "Perbill",
      },
      InflationConfigChanged: {
        annualMin: "Perbill",
        annualIdeal: "Perbill",
        annualMax: "Perbill",
        roundMin: "Perbill",
        roundIdeal: "Perbill",
        roundMax: "Perbill",
      },
      InflationAllocationChanged: {
        developer: "Perbill",
        executor: "Perbill",
      },
      RoundTokensIssued: "(u32,u128)",
      BeneficiaryTokensIssued: "(AccountId32,u128)",
      RewardsClaimed: "(AccountId32,u128)",
    },
  },
  /** Lookup68: pallet_clock::pallet::Event<T> */
  PalletClockEvent: "Null",
  /** Lookup69: pallet_xbi_portal::pallet::Event<T> */
  PalletXbiPortalEvent: {
    _enum: ["AbiInstructionExecuted"],
  },
  /** Lookup70: pallet_3vm::pallet::Event<T> */
  Pallet3vmEvent: {
    _enum: {
      SignalBounced: "(u32,T3rnSdkPrimitivesSignalSignalKind,H256)",
      ExceededBounceThrehold: "(u32,T3rnSdkPrimitivesSignalSignalKind,H256)",
      ModuleInstantiated:
        "(H256,AccountId32,T3rnPrimitivesContractMetadataContractType,u32)",
      AuthorStored: "(AccountId32,AccountId32)",
      AuthorRemoved: "AccountId32",
    },
  },
  /** Lookup72: t3rn_sdk_primitives::signal::SignalKind */
  T3rnSdkPrimitivesSignalSignalKind: {
    _enum: {
      Complete: "Null",
      Kill: "T3rnSdkPrimitivesSignalKillReason",
    },
  },
  /** Lookup73: t3rn_sdk_primitives::signal::KillReason */
  T3rnSdkPrimitivesSignalKillReason: {
    _enum: ["Unhandled", "Codec", "Timeout"],
  },
  /** Lookup75: t3rn_primitives::contract_metadata::ContractType */
  T3rnPrimitivesContractMetadataContractType: {
    _enum: [
      "System",
      "VanillaEvm",
      "VanillaWasm",
      "VolatileEvm",
      "VolatileWasm",
    ],
  },
  /** Lookup77: pallet_contracts::pallet::Event<T> */
  PalletContractsEvent: {
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
  /** Lookup78: pallet_evm::pallet::Event<T> */
  PalletEvmEvent: {
    _enum: {
      Log: "EthereumLog",
      Created: "H160",
      CreatedFailed: "H160",
      Executed: "H160",
      ExecutedFailed: "H160",
      BalanceDeposit: "(AccountId32,H160,U256)",
      BalanceWithdraw: "(AccountId32,H160,U256)",
      ClaimAccount: {
        accountId: "AccountId32",
        evmAddress: "H160",
      },
    },
  },
  /** Lookup79: ethereum::log::Log */
  EthereumLog: {
    address: "H160",
    topics: "Vec<H256>",
    data: "Bytes",
  },
  /** Lookup80: pallet_account_manager::pallet::Event<T> */
  PalletAccountManagerEvent: {
    _enum: {
      ContractsRegistryExecutionFinalized: {
        executionId: "u64",
      },
      Issued: {
        recipient: "AccountId32",
        amount: "u128",
      },
    },
  },
  /** Lookup81: pallet_portal::pallet::Event<T> */
  PalletPortalEvent: {
    _enum: {
      GatewayRegistered: "[u8;4]",
      SetOwner: "([u8;4],Bytes)",
      SetOperational: "([u8;4],bool)",
      HeaderSubmitted: "([u8;4],Bytes)",
    },
  },
  /** Lookup83: frame_system::Phase */
  FrameSystemPhase: {
    _enum: {
      ApplyExtrinsic: "u32",
      Finalization: "Null",
      Initialization: "Null",
    },
  },
  /** Lookup86: frame_system::LastRuntimeUpgradeInfo */
  FrameSystemLastRuntimeUpgradeInfo: {
    specVersion: "Compact<u32>",
    specName: "Text",
  },
  /** Lookup89: frame_system::pallet::Call<T> */
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
  /** Lookup92: frame_system::limits::BlockWeights */
  FrameSystemLimitsBlockWeights: {
    baseBlock: "u64",
    maxBlock: "u64",
    perClass: "FrameSupportWeightsPerDispatchClassWeightsPerClass",
  },
  /**
   * Lookup93:
   * frame_support::weights::PerDispatchClass<frame_system::limits::WeightsPerClass>
   */
  FrameSupportWeightsPerDispatchClassWeightsPerClass: {
    normal: "FrameSystemLimitsWeightsPerClass",
    operational: "FrameSystemLimitsWeightsPerClass",
    mandatory: "FrameSystemLimitsWeightsPerClass",
  },
  /** Lookup94: frame_system::limits::WeightsPerClass */
  FrameSystemLimitsWeightsPerClass: {
    baseExtrinsic: "u64",
    maxExtrinsic: "Option<u64>",
    maxTotal: "Option<u64>",
    reserved: "Option<u64>",
  },
  /** Lookup96: frame_system::limits::BlockLength */
  FrameSystemLimitsBlockLength: {
    max: "FrameSupportWeightsPerDispatchClassU32",
  },
  /** Lookup97: frame_support::weights::PerDispatchClass<T> */
  FrameSupportWeightsPerDispatchClassU32: {
    normal: "u32",
    operational: "u32",
    mandatory: "u32",
  },
  /** Lookup98: frame_support::weights::RuntimeDbWeight */
  FrameSupportWeightsRuntimeDbWeight: {
    read: "u64",
    write: "u64",
  },
  /** Lookup99: sp_version::RuntimeVersion */
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
  /** Lookup105: frame_system::pallet::Error<T> */
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
  /** Lookup107: pallet_timestamp::pallet::Call<T> */
  PalletTimestampCall: {
    _enum: {
      set: {
        now: "Compact<u64>",
      },
    },
  },
  /** Lookup110: sp_consensus_aura::sr25519::app_sr25519::Public */
  SpConsensusAuraSr25519AppSr25519Public: "SpCoreSr25519Public",
  /** Lookup111: sp_core::sr25519::Public */
  SpCoreSr25519Public: "[u8;32]",
  /** Lookup114: pallet_grandpa::StoredState<N> */
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
  /** Lookup115: pallet_grandpa::StoredPendingChange<N, Limit> */
  PalletGrandpaStoredPendingChange: {
    scheduledAt: "u32",
    delay: "u32",
    nextAuthorities: "Vec<(SpFinalityGrandpaAppPublic,u64)>",
    forced: "Option<u32>",
  },
  /** Lookup118: pallet_grandpa::pallet::Call<T> */
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
  /** Lookup119: sp_finality_grandpa::EquivocationProof<primitive_types::H256, N> */
  SpFinalityGrandpaEquivocationProof: {
    setId: "u64",
    equivocation: "SpFinalityGrandpaEquivocation",
  },
  /** Lookup120: sp_finality_grandpa::Equivocation<primitive_types::H256, N> */
  SpFinalityGrandpaEquivocation: {
    _enum: {
      Prevote: "FinalityGrandpaEquivocationPrevote",
      Precommit: "FinalityGrandpaEquivocationPrecommit",
    },
  },
  /**
   * Lookup121: finality_grandpa::Equivocation<sp_finality_grandpa::app::Public,
   * finality_grandpa::Prevote<primitive_types::H256, N>,
   * sp_finality_grandpa::app::Signature>
   */
  FinalityGrandpaEquivocationPrevote: {
    roundNumber: "u64",
    identity: "SpFinalityGrandpaAppPublic",
    first: "(FinalityGrandpaPrevote,SpFinalityGrandpaAppSignature)",
    second: "(FinalityGrandpaPrevote,SpFinalityGrandpaAppSignature)",
  },
  /** Lookup122: finality_grandpa::Prevote<primitive_types::H256, N> */
  FinalityGrandpaPrevote: {
    targetHash: "H256",
    targetNumber: "u32",
  },
  /** Lookup123: sp_finality_grandpa::app::Signature */
  SpFinalityGrandpaAppSignature: "SpCoreEd25519Signature",
  /** Lookup124: sp_core::ed25519::Signature */
  SpCoreEd25519Signature: "[u8;64]",
  /**
   * Lookup127: finality_grandpa::Equivocation<sp_finality_grandpa::app::Public,
   * finality_grandpa::Precommit<primitive_types::H256, N>,
   * sp_finality_grandpa::app::Signature>
   */
  FinalityGrandpaEquivocationPrecommit: {
    roundNumber: "u64",
    identity: "SpFinalityGrandpaAppPublic",
    first: "(FinalityGrandpaPrecommit,SpFinalityGrandpaAppSignature)",
    second: "(FinalityGrandpaPrecommit,SpFinalityGrandpaAppSignature)",
  },
  /** Lookup128: finality_grandpa::Precommit<primitive_types::H256, N> */
  FinalityGrandpaPrecommit: {
    targetHash: "H256",
    targetNumber: "u32",
  },
  /** Lookup130: sp_core::Void */
  SpCoreVoid: "Null",
  /** Lookup131: pallet_grandpa::pallet::Error<T> */
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
  /** Lookup133: pallet_balances::BalanceLock<Balance> */
  PalletBalancesBalanceLock: {
    id: "[u8;8]",
    amount: "u128",
    reasons: "PalletBalancesReasons",
  },
  /** Lookup134: pallet_balances::Reasons */
  PalletBalancesReasons: {
    _enum: ["Fee", "Misc", "All"],
  },
  /** Lookup137: pallet_balances::ReserveData<ReserveIdentifier, Balance> */
  PalletBalancesReserveData: {
    id: "[u8;8]",
    amount: "u128",
  },
  /** Lookup139: pallet_balances::Releases */
  PalletBalancesReleases: {
    _enum: ["V1_0_0", "V2_0_0"],
  },
  /** Lookup140: pallet_balances::pallet::Call<T, I> */
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
  /** Lookup144: pallet_balances::pallet::Error<T, I> */
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
  /** Lookup146: pallet_transaction_payment::Releases */
  PalletTransactionPaymentReleases: {
    _enum: ["V1Ancient", "V2"],
  },
  /** Lookup148: frame_support::weights::WeightToFeeCoefficient<Balance> */
  FrameSupportWeightsWeightToFeeCoefficient: {
    coeffInteger: "u128",
    coeffFrac: "Perbill",
    negative: "bool",
    degree: "u8",
  },
  /** Lookup149: pallet_sudo::pallet::Call<T> */
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
  /** Lookup151: pallet_utility::pallet::Call<T> */
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
  /** Lookup153: circuit_standalone_runtime::OriginCaller */
  CircuitStandaloneRuntimeOriginCaller: {
    _enum: {
      system: "FrameSupportDispatchRawOrigin",
      Void: "SpCoreVoid",
    },
  },
  /** Lookup154: frame_support::dispatch::RawOrigin<sp_core::crypto::AccountId32> */
  FrameSupportDispatchRawOrigin: {
    _enum: {
      Root: "Null",
      Signed: "AccountId32",
      None: "Null",
    },
  },
  /** Lookup155: pallet_xdns::pallet::Call<T> */
  PalletXdnsCall: {
    _enum: {
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
  /** Lookup157: t3rn_types::abi::Type */
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
  /** Lookup158: t3rn_types::abi::HasherAlgo */
  T3rnTypesAbiHasherAlgo: {
    _enum: ["Blake2", "Keccak256"],
  },
  /** Lookup159: t3rn_types::abi::CryptoAlgo */
  T3rnTypesAbiCryptoAlgo: {
    _enum: ["Ed25519", "Sr25519", "Ecdsa"],
  },
  /** Lookup160: pallet_contracts_registry::pallet::Call<T> */
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
   * Lookup161:
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
   * Lookup162:
   * t3rn_primitives::contracts_registry::AuthorInfo<sp_core::crypto::AccountId32,
   * BalanceOf>
   */
  T3rnPrimitivesContractsRegistryAuthorInfo: {
    account: "AccountId32",
    feesPerSingleUse: "Option<u128>",
  },
  /**
   * Lookup164: t3rn_types::abi::ContractActionDesc<primitive_types::H256,
   * TargetId, sp_core::crypto::AccountId32>
   */
  T3rnTypesAbiContractActionDesc: {
    actionId: "H256",
    targetId: "Option<[u8;4]>",
    to: "Option<AccountId32>",
  },
  /**
   * Lookup167:
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
  /** Lookup169: t3rn_primitives::contract_metadata::ContractMetadata */
  T3rnPrimitivesContractMetadata: {
    metadataVersion: "Bytes",
    name: "Bytes",
    contractType: "T3rnPrimitivesContractMetadataContractType",
    version: "Bytes",
    authors: "Vec<Bytes>",
    description: "Option<Bytes>",
    documentation: "Option<Bytes>",
    repository: "Option<Bytes>",
    homepage: "Option<Bytes>",
    license: "Option<Bytes>",
  },
  /** Lookup170: pallet_circuit::pallet::Call<T> */
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
      execute_side_effects_with_xbi: {
        xtxId: "H256",
        sideEffect: "T3rnTypesSideEffect",
        maxExecCost: "u128",
        maxNotificationsCost: "u128",
      },
      on_xbi_sfx_resolved: {
        sfxId: "H256",
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
  /** Lookup172: pallet_treasury::pallet::Call<T> */
  PalletTreasuryCall: {
    _enum: {
      mint_for_round: {
        roundIndex: "u32",
        amount: "Compact<u128>",
      },
      claim_rewards: "Null",
      set_inflation: {
        annualInflationConfig: {
          min: "Perbill",
          ideal: "Perbill",
          max: "Perbill",
        },
      },
      set_inflation_alloc: {
        inflationAlloc: "T3rnPrimitivesMonetaryInflationAllocation",
      },
      set_round_term: {
        _alias: {
          new_: "new",
        },
        new_: "u32",
      },
      add_beneficiary: {
        beneficiary: "AccountId32",
        role: "T3rnPrimitivesMonetaryBeneficiaryRole",
      },
      remove_beneficiary: {
        beneficiary: "AccountId32",
      },
      set_total_stake_expectation: {
        expectations: {
          min: "u128",
          ideal: "u128",
          max: "u128",
        },
      },
    },
  },
  /** Lookup174: t3rn_primitives::monetary::InflationAllocation */
  T3rnPrimitivesMonetaryInflationAllocation: {
    developer: "Perbill",
    executor: "Perbill",
  },
  /** Lookup175: t3rn_primitives::monetary::BeneficiaryRole */
  T3rnPrimitivesMonetaryBeneficiaryRole: {
    _enum: ["Developer", "Executor"],
  },
  /** Lookup177: pallet_xbi_portal::pallet::Call<T> */
  PalletXbiPortalCall: {
    _enum: {
      execute_xcm: {
        xcm: "XcmV2Xcm",
      },
      cleanup: "Null",
      enter_call: {
        checkin: "PalletXbiPortalXbiFormatXbiCheckIn",
        xbiId: "H256",
      },
      check_in_xbi: {
        xbi: "PalletXbiPortalXbiFormat",
      },
    },
  },
  /** Lookup178: xcm::v2::Xcm<Call> */
  XcmV2Xcm: "Vec<XcmV2Instruction>",
  /** Lookup180: xcm::v2::Instruction<Call> */
  XcmV2Instruction: {
    _enum: {
      WithdrawAsset: "XcmV1MultiassetMultiAssets",
      ReserveAssetDeposited: "XcmV1MultiassetMultiAssets",
      ReceiveTeleportedAsset: "XcmV1MultiassetMultiAssets",
      QueryResponse: {
        queryId: "Compact<u64>",
        response: "XcmV2Response",
        maxWeight: "Compact<u64>",
      },
      TransferAsset: {
        assets: "XcmV1MultiassetMultiAssets",
        beneficiary: "XcmV1MultiLocation",
      },
      TransferReserveAsset: {
        assets: "XcmV1MultiassetMultiAssets",
        dest: "XcmV1MultiLocation",
        xcm: "XcmV2Xcm",
      },
      Transact: {
        originType: "XcmV0OriginKind",
        requireWeightAtMost: "Compact<u64>",
        call: "XcmDoubleEncoded",
      },
      HrmpNewChannelOpenRequest: {
        sender: "Compact<u32>",
        maxMessageSize: "Compact<u32>",
        maxCapacity: "Compact<u32>",
      },
      HrmpChannelAccepted: {
        recipient: "Compact<u32>",
      },
      HrmpChannelClosing: {
        initiator: "Compact<u32>",
        sender: "Compact<u32>",
        recipient: "Compact<u32>",
      },
      ClearOrigin: "Null",
      DescendOrigin: "XcmV1MultilocationJunctions",
      ReportError: {
        queryId: "Compact<u64>",
        dest: "XcmV1MultiLocation",
        maxResponseWeight: "Compact<u64>",
      },
      DepositAsset: {
        assets: "XcmV1MultiassetMultiAssetFilter",
        maxAssets: "Compact<u32>",
        beneficiary: "XcmV1MultiLocation",
      },
      DepositReserveAsset: {
        assets: "XcmV1MultiassetMultiAssetFilter",
        maxAssets: "Compact<u32>",
        dest: "XcmV1MultiLocation",
        xcm: "XcmV2Xcm",
      },
      ExchangeAsset: {
        give: "XcmV1MultiassetMultiAssetFilter",
        receive: "XcmV1MultiassetMultiAssets",
      },
      InitiateReserveWithdraw: {
        assets: "XcmV1MultiassetMultiAssetFilter",
        reserve: "XcmV1MultiLocation",
        xcm: "XcmV2Xcm",
      },
      InitiateTeleport: {
        assets: "XcmV1MultiassetMultiAssetFilter",
        dest: "XcmV1MultiLocation",
        xcm: "XcmV2Xcm",
      },
      QueryHolding: {
        queryId: "Compact<u64>",
        dest: "XcmV1MultiLocation",
        assets: "XcmV1MultiassetMultiAssetFilter",
        maxResponseWeight: "Compact<u64>",
      },
      BuyExecution: {
        fees: "XcmV1MultiAsset",
        weightLimit: "XcmV2WeightLimit",
      },
      RefundSurplus: "Null",
      SetErrorHandler: "XcmV2Xcm",
      SetAppendix: "XcmV2Xcm",
      ClearError: "Null",
      ClaimAsset: {
        assets: "XcmV1MultiassetMultiAssets",
        ticket: "XcmV1MultiLocation",
      },
      Trap: "Compact<u64>",
      SubscribeVersion: {
        queryId: "Compact<u64>",
        maxResponseWeight: "Compact<u64>",
      },
      UnsubscribeVersion: "Null",
    },
  },
  /** Lookup181: xcm::v1::multiasset::MultiAssets */
  XcmV1MultiassetMultiAssets: "Vec<XcmV1MultiAsset>",
  /** Lookup183: xcm::v1::multiasset::MultiAsset */
  XcmV1MultiAsset: {
    id: "XcmV1MultiassetAssetId",
    fun: "XcmV1MultiassetFungibility",
  },
  /** Lookup184: xcm::v1::multiasset::AssetId */
  XcmV1MultiassetAssetId: {
    _enum: {
      Concrete: "XcmV1MultiLocation",
      Abstract: "Bytes",
    },
  },
  /** Lookup185: xcm::v1::multilocation::MultiLocation */
  XcmV1MultiLocation: {
    parents: "u8",
    interior: "XcmV1MultilocationJunctions",
  },
  /** Lookup186: xcm::v1::multilocation::Junctions */
  XcmV1MultilocationJunctions: {
    _enum: {
      Here: "Null",
      X1: "XcmV1Junction",
      X2: "(XcmV1Junction,XcmV1Junction)",
      X3: "(XcmV1Junction,XcmV1Junction,XcmV1Junction)",
      X4: "(XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction)",
      X5: "(XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction)",
      X6: "(XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction)",
      X7: "(XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction)",
      X8: "(XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction)",
    },
  },
  /** Lookup187: xcm::v1::junction::Junction */
  XcmV1Junction: {
    _enum: {
      Parachain: "Compact<u32>",
      AccountId32: {
        network: "XcmV0JunctionNetworkId",
        id: "[u8;32]",
      },
      AccountIndex64: {
        network: "XcmV0JunctionNetworkId",
        index: "Compact<u64>",
      },
      AccountKey20: {
        network: "XcmV0JunctionNetworkId",
        key: "[u8;20]",
      },
      PalletInstance: "u8",
      GeneralIndex: "Compact<u128>",
      GeneralKey: "Bytes",
      OnlyChild: "Null",
      Plurality: {
        id: "XcmV0JunctionBodyId",
        part: "XcmV0JunctionBodyPart",
      },
    },
  },
  /** Lookup188: xcm::v0::junction::NetworkId */
  XcmV0JunctionNetworkId: {
    _enum: {
      Any: "Null",
      Named: "Bytes",
      Polkadot: "Null",
      Kusama: "Null",
    },
  },
  /** Lookup189: xcm::v0::junction::BodyId */
  XcmV0JunctionBodyId: {
    _enum: {
      Unit: "Null",
      Named: "Bytes",
      Index: "Compact<u32>",
      Executive: "Null",
      Technical: "Null",
      Legislative: "Null",
      Judicial: "Null",
    },
  },
  /** Lookup190: xcm::v0::junction::BodyPart */
  XcmV0JunctionBodyPart: {
    _enum: {
      Voice: "Null",
      Members: {
        count: "Compact<u32>",
      },
      Fraction: {
        nom: "Compact<u32>",
        denom: "Compact<u32>",
      },
      AtLeastProportion: {
        nom: "Compact<u32>",
        denom: "Compact<u32>",
      },
      MoreThanProportion: {
        nom: "Compact<u32>",
        denom: "Compact<u32>",
      },
    },
  },
  /** Lookup191: xcm::v1::multiasset::Fungibility */
  XcmV1MultiassetFungibility: {
    _enum: {
      Fungible: "Compact<u128>",
      NonFungible: "XcmV1MultiassetAssetInstance",
    },
  },
  /** Lookup192: xcm::v1::multiasset::AssetInstance */
  XcmV1MultiassetAssetInstance: {
    _enum: {
      Undefined: "Null",
      Index: "Compact<u128>",
      Array4: "[u8;4]",
      Array8: "[u8;8]",
      Array16: "[u8;16]",
      Array32: "[u8;32]",
      Blob: "Bytes",
    },
  },
  /** Lookup194: xcm::v2::Response */
  XcmV2Response: {
    _enum: {
      Null: "Null",
      Assets: "XcmV1MultiassetMultiAssets",
      ExecutionResult: "Option<(u32,XcmV2TraitsError)>",
      Version: "u32",
    },
  },
  /** Lookup197: xcm::v2::traits::Error */
  XcmV2TraitsError: {
    _enum: {
      Overflow: "Null",
      Unimplemented: "Null",
      UntrustedReserveLocation: "Null",
      UntrustedTeleportLocation: "Null",
      MultiLocationFull: "Null",
      MultiLocationNotInvertible: "Null",
      BadOrigin: "Null",
      InvalidLocation: "Null",
      AssetNotFound: "Null",
      FailedToTransactAsset: "Null",
      NotWithdrawable: "Null",
      LocationCannotHold: "Null",
      ExceedsMaxMessageSize: "Null",
      DestinationUnsupported: "Null",
      Transport: "Null",
      Unroutable: "Null",
      UnknownClaim: "Null",
      FailedToDecode: "Null",
      MaxWeightInvalid: "Null",
      NotHoldingFees: "Null",
      TooExpensive: "Null",
      Trap: "u64",
      UnhandledXcmVersion: "Null",
      WeightLimitReached: "u64",
      Barrier: "Null",
      WeightNotComputable: "Null",
    },
  },
  /** Lookup201: xcm::v0::OriginKind */
  XcmV0OriginKind: {
    _enum: ["Native", "SovereignAccount", "Superuser", "Xcm"],
  },
  /** Lookup202: xcm::double_encoded::DoubleEncoded<T> */
  XcmDoubleEncoded: {
    encoded: "Bytes",
  },
  /** Lookup203: xcm::v1::multiasset::MultiAssetFilter */
  XcmV1MultiassetMultiAssetFilter: {
    _enum: {
      Definite: "XcmV1MultiassetMultiAssets",
      Wild: "XcmV1MultiassetWildMultiAsset",
    },
  },
  /** Lookup204: xcm::v1::multiasset::WildMultiAsset */
  XcmV1MultiassetWildMultiAsset: {
    _enum: {
      All: "Null",
      AllOf: {
        id: "XcmV1MultiassetAssetId",
        fun: "XcmV1MultiassetWildFungibility",
      },
    },
  },
  /** Lookup205: xcm::v1::multiasset::WildFungibility */
  XcmV1MultiassetWildFungibility: {
    _enum: ["Fungible", "NonFungible"],
  },
  /** Lookup206: xcm::v2::WeightLimit */
  XcmV2WeightLimit: {
    _enum: {
      Unlimited: "Null",
      Limited: "Compact<u64>",
    },
  },
  /** Lookup208: pallet_xbi_portal::xbi_format::XBICheckIn<BlockNumber> */
  PalletXbiPortalXbiFormatXbiCheckIn: {
    xbi: "PalletXbiPortalXbiFormat",
    notificationDeliveryTimeout: "u32",
    notificationExecutionTimeout: "u32",
  },
  /** Lookup209: pallet_xbi_portal::xbi_format::XBIFormat */
  PalletXbiPortalXbiFormat: {
    instr: "PalletXbiPortalXbiFormatXbiInstr",
    metadata: "PalletXbiPortalXbiFormatXbiMetadata",
  },
  /** Lookup210: pallet_xbi_portal::xbi_format::XBIInstr */
  PalletXbiPortalXbiFormatXbiInstr: {
    _enum: {
      Unknown: {
        identifier: "u8",
        params: "Bytes",
      },
      CallNative: {
        payload: "Bytes",
      },
      CallEvm: {
        source: "H160",
        target: "H160",
        value: "U256",
        input: "Bytes",
        gasLimit: "u64",
        maxFeePerGas: "U256",
        maxPriorityFeePerGas: "Option<U256>",
        nonce: "Option<U256>",
        accessList: "Vec<(H160,Vec<H256>)>",
      },
      CallWasm: {
        dest: "AccountId32",
        value: "u128",
        gasLimit: "u64",
        storageDepositLimit: "Option<u128>",
        data: "Bytes",
      },
      CallCustom: {
        caller: "AccountId32",
        dest: "AccountId32",
        value: "u128",
        input: "Bytes",
        limit: "u64",
        additionalParams: "Bytes",
      },
      Transfer: {
        dest: "AccountId32",
        value: "u128",
      },
      TransferORML: {
        currencyId: "u64",
        dest: "AccountId32",
        value: "u128",
      },
      TransferAssets: {
        currencyId: "u64",
        dest: "AccountId32",
        value: "u128",
      },
      Result: {
        outcome: "PalletXbiPortalXbiFormatXbiCheckOutStatus",
        output: "Bytes",
        witness: "Bytes",
      },
      Notification: {
        kind: "PalletXbiPortalXbiFormatXbiNotificationKind",
        instructionId: "Bytes",
        extra: "Bytes",
      },
    },
  },
  /** Lookup211: pallet_xbi_portal::xbi_format::XBIMetadata */
  PalletXbiPortalXbiFormatXbiMetadata: {
    id: "H256",
    destParaId: "u32",
    srcParaId: "u32",
    sent: "PalletXbiPortalXbiFormatActionNotificationTimeouts",
    delivered: "PalletXbiPortalXbiFormatActionNotificationTimeouts",
    executed: "PalletXbiPortalXbiFormatActionNotificationTimeouts",
    maxExecCost: "u128",
    maxNotificationsCost: "u128",
    actualAggregatedCost: "Option<u128>",
    maybeKnownOrigin: "Option<AccountId32>",
  },
  /** Lookup212: pallet_xbi_portal::xbi_format::ActionNotificationTimeouts */
  PalletXbiPortalXbiFormatActionNotificationTimeouts: {
    action: "u32",
    notification: "u32",
  },
  /** Lookup213: pallet_3vm::pallet::Call<T> */
  Pallet3vmCall: "Null",
  /** Lookup214: pallet_contracts::pallet::Call<T> */
  PalletContractsCall: {
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
  /** Lookup216: pallet_evm::pallet::Call<T> */
  PalletEvmCall: {
    _enum: {
      withdraw: {
        address: "H160",
        value: "u128",
      },
      call: {
        target: "H160",
        input: "Bytes",
        value: "U256",
        gasLimit: "u64",
        maxFeePerGas: "U256",
        maxPriorityFeePerGas: "Option<U256>",
        nonce: "Option<U256>",
        accessList: "Vec<(H160,Vec<H256>)>",
      },
      create: {
        init: "Bytes",
        value: "U256",
        gasLimit: "u64",
        maxFeePerGas: "U256",
        maxPriorityFeePerGas: "Option<U256>",
        nonce: "Option<U256>",
        accessList: "Vec<(H160,Vec<H256>)>",
      },
      create2: {
        init: "Bytes",
        salt: "H256",
        value: "U256",
        gasLimit: "u64",
        maxFeePerGas: "U256",
        maxPriorityFeePerGas: "Option<U256>",
        nonce: "Option<U256>",
        accessList: "Vec<(H160,Vec<H256>)>",
      },
      claim: "Null",
    },
  },
  /** Lookup217: pallet_account_manager::pallet::Call<T> */
  PalletAccountManagerCall: {
    _enum: {
      deposit: {
        chargeId: "H256",
        payee: "AccountId32",
        chargeFee: "u128",
        offeredReward: "u128",
        source: "T3rnPrimitivesClaimableBenefitSource",
        role: "T3rnPrimitivesClaimableCircuitRole",
        maybeRecipient: "Option<AccountId32>",
      },
      finalize: {
        chargeId: "H256",
        outcome: "T3rnPrimitivesAccountManagerOutcome",
        maybeRecipient: "Option<AccountId32>",
        maybeActualFees: "Option<u128>",
      },
    },
  },
  /** Lookup218: t3rn_primitives::claimable::BenefitSource */
  T3rnPrimitivesClaimableBenefitSource: {
    _enum: ["TrafficFees", "TrafficRewards", "BootstrapPool", "Inflation"],
  },
  /** Lookup219: t3rn_primitives::claimable::CircuitRole */
  T3rnPrimitivesClaimableCircuitRole: {
    _enum: [
      "Ambassador",
      "Executor",
      "Attester",
      "Staker",
      "Collator",
      "ContractAuthor",
      "Relayer",
      "Requester",
      "Local",
    ],
  },
  /** Lookup220: t3rn_primitives::account_manager::Outcome */
  T3rnPrimitivesAccountManagerOutcome: {
    _enum: ["UnexpectedFailure", "Revert", "Commit"],
  },
  /** Lookup221: pallet_portal::pallet::Call<T> */
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
        securityCoordinates: "Bytes",
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
  /** Lookup222: t3rn_types::abi::GatewayABIConfig */
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
  /** Lookup224: t3rn_types::abi::StructDecl */
  T3rnTypesAbiStructDecl: {
    name: "T3rnTypesAbiType",
    fields: "Vec<T3rnTypesAbiParameter>",
    offsets: "Vec<u16>",
  },
  /** Lookup226: t3rn_types::abi::Parameter */
  T3rnTypesAbiParameter: {
    name: "Option<Bytes>",
    ty: "T3rnTypesAbiType",
    no: "u32",
    indexed: "Option<bool>",
  },
  /** Lookup229: t3rn_primitives::GatewayVendor */
  T3rnPrimitivesGatewayVendor: {
    _enum: ["InternalXBI", "PolkadotLike", "Rococo", "EvmBased"],
  },
  /** Lookup230: t3rn_primitives::GatewayType */
  T3rnPrimitivesGatewayType: {
    _enum: {
      ProgrammableInternal: "u32",
      ProgrammableExternal: "u32",
      TxOnly: "u32",
      OnCircuit: "u32",
    },
  },
  /** Lookup231: t3rn_primitives::GatewayGenesisConfig */
  T3rnPrimitivesGatewayGenesisConfig: {
    modulesEncoded: "Option<Bytes>",
    extrinsicsVersion: "u8",
    genesisHash: "Bytes",
  },
  /** Lookup232: t3rn_primitives::GatewaySysProps */
  T3rnPrimitivesGatewaySysProps: {
    ss58Format: "u16",
    tokenSymbol: "Bytes",
    tokenDecimals: "u8",
  },
  /** Lookup234: pallet_sudo::pallet::Error<T> */
  PalletSudoError: {
    _enum: ["RequireSudo"],
  },
  /** Lookup235: pallet_utility::pallet::Error<T> */
  PalletUtilityError: {
    _enum: ["TooManyCalls"],
  },
  /** Lookup238: orml_tokens::BalanceLock<Balance> */
  OrmlTokensBalanceLock: {
    id: "[u8;8]",
    amount: "u128",
  },
  /** Lookup240: orml_tokens::AccountData<Balance> */
  OrmlTokensAccountData: {
    free: "u128",
    reserved: "u128",
    frozen: "u128",
  },
  /** Lookup241: orml_tokens::module::Error<T> */
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
  /** Lookup242: t3rn_primitives::side_effect::interface::SideEffectInterface */
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
  /** Lookup243: t3rn_primitives::xdns::XdnsRecord<sp_core::crypto::AccountId32> */
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
    securityCoordinates: "Bytes",
    lastFinalized: "Option<u64>",
    allowedSideEffects: "Vec<[u8;4]>",
  },
  /** Lookup245: t3rn_primitives::xdns::Parachain */
  T3rnPrimitivesXdnsParachain: {
    relayChainId: "[u8;4]",
    id: "u32",
  },
  /** Lookup246: pallet_xdns::pallet::Error<T> */
  PalletXdnsError: {
    _enum: [
      "XdnsRecordAlreadyExists",
      "UnknownXdnsRecord",
      "XdnsRecordNotFound",
      "SideEffectInterfaceAlreadyExists",
      "SideEffectInterfaceNotFound",
      "NoParachainInfoFound",
    ],
  },
  /** Lookup247: pallet_contracts_registry::pallet::Error<T> */
  PalletContractsRegistryError: {
    _enum: ["ContractAlreadyExists", "UnknownContract"],
  },
  /**
   * Lookup249:
   * pallet_circuit::state::InsuranceDeposit<sp_core::crypto::AccountId32,
   * BlockNumber, BalanceOf>
   */
  PalletCircuitStateInsuranceDeposit: {
    insurance: "u128",
    reservedBond: "u128",
    reward: "u128",
    requester: "AccountId32",
    bondedRelayer: "Option<AccountId32>",
    status: "PalletCircuitStateCircuitStatus",
    requestedAt: "u32",
  },
  /** Lookup250: pallet_circuit::state::CircuitStatus */
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
      "RevertMisbehaviour",
      "Committed",
      "Reverted",
    ],
  },
  /**
   * Lookup251: pallet_circuit::state::XExecSignal<sp_core::crypto::AccountId32,
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
  /** Lookup254: t3rn_primitives::volatile::LocalState */
  T3rnPrimitivesVolatileLocalState: {
    state: "BTreeMap<[u8;32], Bytes>",
  },
  /** Lookup260: t3rn_sdk_primitives::signal::ExecutionSignal<primitive_types::H256> */
  T3rnSdkPrimitivesSignalExecutionSignal: {
    step: "u32",
    kind: "T3rnSdkPrimitivesSignalSignalKind",
    executionId: "H256",
  },
  /** Lookup262: frame_support::PalletId */
  FrameSupportPalletId: "[u8;8]",
  /** Lookup263: pallet_circuit::pallet::Error<T> */
  PalletCircuitError: {
    _enum: [
      "UpdateXtxTriggeredWithUnexpectedStatus",
      "ApplyTriggeredWithUnexpectedStatus",
      "RequesterNotEnoughBalance",
      "ContractXtxKilledRunOutOfFunds",
      "ChargingTransferFailed",
      "FinalizeSquareUpFailed",
      "CriticalStateSquareUpCalledToFinishWithoutFsxConfirmed",
      "RewardTransferFailed",
      "RefundTransferFailed",
      "SideEffectsValidationFailed",
      "InsuranceBondNotRequired",
      "InsuranceBondTooLow",
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
      "SideEffectIsAlreadyScheduledToExecuteOverXBI",
      "LocalSideEffectExecutionNotApplicable",
      "LocalExecutionUnauthorized",
      "FailedToConvertSFX2XBI",
      "FailedToCheckInOverXBI",
      "FailedToCreateXBIMetadataDueToWrongAccountConversion",
      "FailedToConvertXBIResult2SFXConfirmation",
      "FailedToEnterXBIPortal",
      "FailedToExitXBIPortal",
      "XBIExitFailedOnSFXConfirmation",
      "UnsupportedRole",
      "InvalidLocalTrigger",
      "SignalQueueFull",
    ],
  },
  /** Lookup264: pallet_treasury::inflation::InflationInfo */
  PalletTreasuryInflationInflationInfo: {
    annual: {
      min: "Perbill",
      ideal: "Perbill",
      max: "Perbill",
    },
    round: {
      min: "Perbill",
      ideal: "Perbill",
      max: "Perbill",
    },
  },
  /** Lookup265: t3rn_primitives::common::RoundInfo<BlockNumber> */
  T3rnPrimitivesCommonRoundInfo: {
    index: "u32",
    head: "u32",
    term: "u32",
  },
  /** Lookup267: pallet_treasury::pallet::Error<T> */
  PalletTreasuryError: {
    _enum: [
      "InvalidInflationConfig",
      "InvalidInflationAllocation",
      "ValueNotChanged",
      "RoundTermTooShort",
      "NotBeneficiary",
      "NoRewardsAvailable",
    ],
  },
  /**
   * Lookup269:
   * t3rn_primitives::claimable::ClaimableArtifacts<sp_core::crypto::AccountId32,
   * Balance>
   */
  T3rnPrimitivesClaimableClaimableArtifacts: {
    beneficiary: "AccountId32",
    role: "T3rnPrimitivesClaimableCircuitRole",
    totalRoundClaim: "u128",
    benefitSource: "T3rnPrimitivesClaimableBenefitSource",
  },
  /** Lookup270: pallet_clock::pallet::Error<T> */
  PalletClockError: "Null",
  /** Lookup271: pallet_xbi_portal::xbi_format::XBICheckOut */
  PalletXbiPortalXbiFormatXbiCheckOut: {
    xbi: "PalletXbiPortalXbiFormatXbiInstr",
    resolutionStatus: "PalletXbiPortalXbiFormatXbiCheckOutStatus",
    checkoutTimeout: "u32",
    actualExecutionCost: "u128",
    actualDeliveryCost: "u128",
    actualAggregatedCost: "u128",
  },
  /** Lookup272: pallet_xbi_portal::pallet::Error<T> */
  PalletXbiPortalError: {
    _enum: [
      "EnterFailedOnXcmSend",
      "EnterFailedOnMultiLocationTransform",
      "ExitUnhandled",
      "XBIABIFailedToCastBetweenTypesValue",
      "XBIABIFailedToCastBetweenTypesAddress",
      "XBIInstructionNotAllowedHere",
      "XBIAlreadyCheckedIn",
      "XBINotificationTimeOutDelivery",
      "XBINotificationTimeOutExecution",
      "NoXBICallbackSupported",
      "NoEVMSupportedAtDest",
      "NoWASMSupportedAtDest",
      "No3VMSupportedAtDest",
      "NoTransferSupportedAtDest",
      "NoTransferAssetsSupportedAtDest",
      "NoTransferORMLSupportedAtDest",
      "NoTransferEscrowSupportedAtDest",
      "NoTransferMultiEscrowSupportedAtDest",
      "NoSwapSupportedAtDest",
      "NoAddLiquiditySupportedAtDest",
    ],
  },
  /** Lookup274: pallet_3vm::pallet::Error<T> */
  Pallet3vmError: {
    _enum: [
      "ExceededSignalBounceThreshold",
      "CannotTriggerWithoutSideEffects",
      "ContractNotFound",
      "InvalidOrigin",
      "CannotInstantiateContract",
      "ContractCannotRemunerate",
      "ContractCannotHaveStorage",
      "ContractCannotGenerateSideEffects",
      "InvalidPrecompilePointer",
      "InvalidPrecompileArgs",
    ],
  },
  /** Lookup275: pallet_contracts::wasm::PrefabWasmModule<T> */
  PalletContractsWasmPrefabWasmModule: {
    instructionWeightsVersion: "Compact<u32>",
    initial: "Compact<u32>",
    maximum: "Compact<u32>",
    code: "Bytes",
    author: "Option<T3rnPrimitivesContractsRegistryAuthorInfo>",
    kind: "T3rnPrimitivesContractMetadataContractType",
  },
  /** Lookup277: pallet_contracts::wasm::OwnerInfo<T> */
  PalletContractsWasmOwnerInfo: {
    owner: "AccountId32",
    deposit: "Compact<u128>",
    refcount: "Compact<u64>",
  },
  /** Lookup278: pallet_contracts::storage::RawContractInfo<primitive_types::H256, Balance> */
  PalletContractsStorageRawContractInfo: {
    trieId: "Bytes",
    codeHash: "H256",
    storageDeposit: "u128",
  },
  /** Lookup280: pallet_contracts::storage::DeletedContract */
  PalletContractsStorageDeletedContract: {
    trieId: "Bytes",
  },
  /** Lookup281: pallet_contracts::schedule::Schedule<T> */
  PalletContractsSchedule: {
    limits: "PalletContractsScheduleLimits",
    instructionWeights: "PalletContractsScheduleInstructionWeights",
    hostFnWeights: "PalletContractsScheduleHostFnWeights",
  },
  /** Lookup282: pallet_contracts::schedule::Limits */
  PalletContractsScheduleLimits: {
    eventTopics: "u32",
    stackHeight: "Option<u32>",
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
  /** Lookup283: pallet_contracts::schedule::InstructionWeights<T> */
  PalletContractsScheduleInstructionWeights: {
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
  /** Lookup284: pallet_contracts::schedule::HostFnWeights<T> */
  PalletContractsScheduleHostFnWeights: {
    _alias: {
      r_return: "r#return",
    },
    caller: "u64",
    isContract: "u64",
    codeHash: "u64",
    ownCodeHash: "u64",
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
  /** Lookup285: pallet_contracts::pallet::Error<T> */
  PalletContractsError: {
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
      "NoStateReturned",
    ],
  },
  /** Lookup287: pallet_evm::ThreeVmInfo<T> */
  PalletEvmThreeVmInfo: {
    author: "T3rnPrimitivesContractsRegistryAuthorInfo",
    kind: "T3rnPrimitivesContractMetadataContractType",
  },
  /** Lookup288: pallet_evm::pallet::Error<T> */
  PalletEvmError: {
    _enum: [
      "BalanceLow",
      "FeeOverflow",
      "PaymentOverflow",
      "WithdrawFailed",
      "GasPriceTooLow",
      "InvalidNonce",
      "InvalidRegistryHash",
      "RemunerateAuthor",
      "ExecutedFailed",
      "CreatedFailed",
    ],
  },
  /**
   * Lookup290:
   * t3rn_primitives::account_manager::RequestCharge<sp_core::crypto::AccountId32,
   * Balance>
   */
  T3rnPrimitivesAccountManagerRequestCharge: {
    payee: "AccountId32",
    offeredReward: "u128",
    chargeFee: "u128",
    recipient: "AccountId32",
    source: "T3rnPrimitivesClaimableBenefitSource",
    role: "T3rnPrimitivesClaimableCircuitRole",
  },
  /**
   * Lookup291:
   * t3rn_primitives::account_manager::Settlement<sp_core::crypto::AccountId32, Balance>
   */
  T3rnPrimitivesAccountManagerSettlement: {
    requester: "AccountId32",
    recipient: "AccountId32",
    settlementAmount: "u128",
    outcome: "T3rnPrimitivesAccountManagerOutcome",
    source: "T3rnPrimitivesClaimableBenefitSource",
    role: "T3rnPrimitivesClaimableCircuitRole",
  },
  /** Lookup292: pallet_account_manager::pallet::Error<T> */
  PalletAccountManagerError: {
    _enum: [
      "PendingChargeNotFoundAtCommit",
      "PendingChargeNotFoundAtRefund",
      "ExecutionNotRegistered",
      "ExecutionAlreadyRegistered",
      "SkippingEmptyCharges",
      "NoChargeOfGivenIdRegistered",
      "ChargeAlreadyRegistered",
      "ChargeOrSettlementCalculationOverflow",
      "DecodingExecutionIDFailed",
    ],
  },
  /** Lookup293: pallet_portal::pallet::Error<T> */
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
  /**
   * Lookup296: sp_runtime::generic::header::Header<Number,
   * sp_runtime::traits::BlakeTwo256>
   */
  SpRuntimeHeader: {
    parentHash: "H256",
    number: "Compact<u32>",
    stateRoot: "H256",
    extrinsicsRoot: "H256",
    digest: "SpRuntimeDigest",
  },
  /** Lookup297: sp_runtime::traits::BlakeTwo256 */
  SpRuntimeBlakeTwo256: "Null",
  /** Lookup298: pallet_grandpa_finality_verifier::bridges::header_chain::AuthoritySet */
  PalletGrandpaFinalityVerifierBridgesHeaderChainAuthoritySet: {
    authorities: "Vec<(SpFinalityGrandpaAppPublic,u64)>",
    setId: "u64",
  },
  /** Lookup299: pallet_grandpa_finality_verifier::types::Parachain */
  PalletGrandpaFinalityVerifierParachain: {
    relayChainId: "[u8;4]",
    id: "u32",
  },
  /** Lookup300: pallet_grandpa_finality_verifier::pallet::Error<T, I> */
  PalletGrandpaFinalityVerifierError: {
    _enum: [
      "EmptyRangeSubmitted",
      "RangeToLarge",
      "NoFinalizedHeader",
      "InvalidAuthoritySet",
      "InvalidGrandpaJustification",
      "InvalidRangeLinkage",
      "InvalidJustificationLinkage",
      "ParachainEntryNotFound",
      "StorageRootNotFound",
      "InclusionDataDecodeError",
      "InvalidStorageProof",
      "EventNotIncluded",
      "HeaderDecodingError",
      "HeaderDataDecodingError",
      "StorageRootMismatch",
      "UnknownHeader",
      "EventDecodingFailed",
      "UnkownSideEffect",
      "UnsupportedScheduledChange",
      "Halted",
    ],
  },
  /** Lookup302: sp_runtime::MultiSignature */
  SpRuntimeMultiSignature: {
    _enum: {
      Ed25519: "SpCoreEd25519Signature",
      Sr25519: "SpCoreSr25519Signature",
      Ecdsa: "SpCoreEcdsaSignature",
    },
  },
  /** Lookup303: sp_core::sr25519::Signature */
  SpCoreSr25519Signature: "[u8;64]",
  /** Lookup304: sp_core::ecdsa::Signature */
  SpCoreEcdsaSignature: "[u8;65]",
  /** Lookup307: frame_system::extensions::check_non_zero_sender::CheckNonZeroSender<T> */
  FrameSystemExtensionsCheckNonZeroSender: "Null",
  /** Lookup308: frame_system::extensions::check_spec_version::CheckSpecVersion<T> */
  FrameSystemExtensionsCheckSpecVersion: "Null",
  /** Lookup309: frame_system::extensions::check_tx_version::CheckTxVersion<T> */
  FrameSystemExtensionsCheckTxVersion: "Null",
  /** Lookup310: frame_system::extensions::check_genesis::CheckGenesis<T> */
  FrameSystemExtensionsCheckGenesis: "Null",
  /** Lookup313: frame_system::extensions::check_nonce::CheckNonce<T> */
  FrameSystemExtensionsCheckNonce: "Compact<u32>",
  /** Lookup314: frame_system::extensions::check_weight::CheckWeight<T> */
  FrameSystemExtensionsCheckWeight: "Null",
  /** Lookup315: pallet_transaction_payment::ChargeTransactionPayment<T> */
  PalletTransactionPaymentChargeTransactionPayment: "Compact<u128>",
  /** Lookup316: circuit_standalone_runtime::Runtime */
  CircuitStandaloneRuntimeRuntime: "Null",
};
