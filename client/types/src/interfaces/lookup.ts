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
    },
  },
  /** Lookup23: sp_runtime::ModuleError */
  SpRuntimeModuleError: {
    index: "u8",
    error: "u8",
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
  /** Lookup26: pallet_grandpa::pallet::Event */
  PalletGrandpaEvent: {
    _enum: {
      NewAuthorities: {
        authoritySet: "Vec<(SpFinalityGrandpaAppPublic,u64)>",
      },
      Paused: "Null",
      Resumed: "Null",
    },
  },
  /** Lookup29: sp_finality_grandpa::app::Public */
  SpFinalityGrandpaAppPublic: "SpCoreEd25519Public",
  /** Lookup30: sp_core::ed25519::Public */
  SpCoreEd25519Public: "[u8;32]",
  /** Lookup31: pallet_balances::pallet::Event<T, I> */
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
  /** Lookup32: frame_support::traits::tokens::misc::BalanceStatus */
  FrameSupportTokensMiscBalanceStatus: {
    _enum: ["Free", "Reserved"],
  },
  /** Lookup33: pallet_sudo::pallet::Event<T> */
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
  /** Lookup37: orml_tokens::module::Event<T> */
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
  /** Lookup38: pallet_xdns::pallet::Event<T> */
  PalletXdnsEvent: {
    _enum: {
      XdnsRecordStored: "[u8;4]",
      XdnsRecordPurged: "(AccountId32,[u8;4])",
      XdnsRecordUpdated: "[u8;4]",
    },
  },
  /** Lookup39: pallet_contracts_registry::pallet::Event<T> */
  PalletContractsRegistryEvent: {
    _enum: {
      ContractStored: "(AccountId32,H256)",
      ContractPurged: "(AccountId32,H256)",
    },
  },
  /** Lookup40: pallet_circuit_portal::pallet::Event<T> */
  PalletCircuitPortalEvent: {
    _enum: {
      NewGatewayRegistered:
        "([u8;4],T3rnPrimitivesGatewayType,T3rnPrimitivesGatewayVendor,T3rnPrimitivesGatewaySysProps,Vec<[u8;4]>)",
      GatewayUpdated: "([u8;4],Option<Vec<[u8;4]>>)",
    },
  },
  /** Lookup41: t3rn_primitives::GatewayType */
  T3rnPrimitivesGatewayType: {
    _enum: {
      ProgrammableInternal: "u32",
      ProgrammableExternal: "u32",
      TxOnly: "u32",
    },
  },
  /** Lookup42: t3rn_primitives::GatewayVendor */
  T3rnPrimitivesGatewayVendor: {
    _enum: ["Substrate", "Ethereum"],
  },
  /** Lookup43: t3rn_primitives::GatewaySysProps */
  T3rnPrimitivesGatewaySysProps: {
    ss58Format: "u16",
    tokenSymbol: "Bytes",
    tokenDecimals: "u8",
  },
  /** Lookup47: pallet_circuit::pallet::Event<T> */
  PalletCircuitEvent: {
    _enum: {
      XTransactionReceivedForExec: "H256",
      XTransactionReadyForExec: "H256",
      XTransactionFinishedExec: "H256",
      NewSideEffectsAvailable:
        "(AccountId32,H256,Vec<T3rnPrimitivesSideEffect>)",
      CancelledSideEffects: "(AccountId32,H256,Vec<T3rnPrimitivesSideEffect>)",
      SideEffectsConfirmed:
        "(H256,Vec<Vec<T3rnPrimitivesSideEffectFullSideEffect>>)",
    },
  },
  /**
   * Lookup49:
   * t3rn_primitives::side_effect::SideEffect<sp_core::crypto::AccountId32,
   * BlockNumber, BalanceOf>
   */
  T3rnPrimitivesSideEffect: {
    target: "[u8;4]",
    prize: "u128",
    orderedAt: "u32",
    encodedAction: "Bytes",
    encodedArgs: "Vec<Bytes>",
    signature: "Bytes",
    enforceExecutioner: "Option<AccountId32>",
  },
  /**
   * Lookup53:
   * t3rn_primitives::side_effect::FullSideEffect<sp_core::crypto::AccountId32,
   * BlockNumber, BalanceOf>
   */
  T3rnPrimitivesSideEffectFullSideEffect: {
    input: "T3rnPrimitivesSideEffect",
    confirmed: "Option<T3rnPrimitivesSideEffectConfirmedSideEffect>",
  },
  /**
   * Lookup55:
   * t3rn_primitives::side_effect::ConfirmedSideEffect<sp_core::crypto::AccountId32,
   * BlockNumber, BalanceOf>
   */
  T3rnPrimitivesSideEffectConfirmedSideEffect: {
    err: "Option<Bytes>",
    output: "Option<Bytes>",
    encodedEffect: "Bytes",
    inclusionProof: "Option<Bytes>",
    executioner: "AccountId32",
    receivedAt: "u32",
    cost: "Option<u128>",
  },
  /** Lookup58: frame_system::Phase */
  FrameSystemPhase: {
    _enum: {
      ApplyExtrinsic: "u32",
      Finalization: "Null",
      Initialization: "Null",
    },
  },
  /** Lookup62: frame_system::LastRuntimeUpgradeInfo */
  FrameSystemLastRuntimeUpgradeInfo: {
    specVersion: "Compact<u32>",
    specName: "Text",
  },
  /** Lookup66: frame_system::pallet::Call<T> */
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
  /** Lookup70: frame_system::limits::BlockWeights */
  FrameSystemLimitsBlockWeights: {
    baseBlock: "u64",
    maxBlock: "u64",
    perClass: "FrameSupportWeightsPerDispatchClassWeightsPerClass",
  },
  /**
   * Lookup71:
   * frame_support::weights::PerDispatchClass<frame_system::limits::WeightsPerClass>
   */
  FrameSupportWeightsPerDispatchClassWeightsPerClass: {
    normal: "FrameSystemLimitsWeightsPerClass",
    operational: "FrameSystemLimitsWeightsPerClass",
    mandatory: "FrameSystemLimitsWeightsPerClass",
  },
  /** Lookup72: frame_system::limits::WeightsPerClass */
  FrameSystemLimitsWeightsPerClass: {
    baseExtrinsic: "u64",
    maxExtrinsic: "Option<u64>",
    maxTotal: "Option<u64>",
    reserved: "Option<u64>",
  },
  /** Lookup74: frame_system::limits::BlockLength */
  FrameSystemLimitsBlockLength: {
    max: "FrameSupportWeightsPerDispatchClassU32",
  },
  /** Lookup75: frame_support::weights::PerDispatchClass<T> */
  FrameSupportWeightsPerDispatchClassU32: {
    normal: "u32",
    operational: "u32",
    mandatory: "u32",
  },
  /** Lookup76: frame_support::weights::RuntimeDbWeight */
  FrameSupportWeightsRuntimeDbWeight: {
    read: "u64",
    write: "u64",
  },
  /** Lookup77: sp_version::RuntimeVersion */
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
  /** Lookup82: frame_system::pallet::Error<T> */
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
  /** Lookup84: pallet_timestamp::pallet::Call<T> */
  PalletTimestampCall: {
    _enum: {
      set: {
        now: "Compact<u64>",
      },
    },
  },
  /** Lookup87: sp_consensus_aura::sr25519::app_sr25519::Public */
  SpConsensusAuraSr25519AppSr25519Public: "SpCoreSr25519Public",
  /** Lookup88: sp_core::sr25519::Public */
  SpCoreSr25519Public: "[u8;32]",
  /** Lookup91: pallet_grandpa::StoredState<N> */
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
  /** Lookup92: pallet_grandpa::StoredPendingChange<N, Limit> */
  PalletGrandpaStoredPendingChange: {
    scheduledAt: "u32",
    delay: "u32",
    nextAuthorities: "Vec<(SpFinalityGrandpaAppPublic,u64)>",
    forced: "Option<u32>",
  },
  /** Lookup95: pallet_grandpa::pallet::Call<T> */
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
  /** Lookup96: sp_finality_grandpa::EquivocationProof<primitive_types::H256, N> */
  SpFinalityGrandpaEquivocationProof: {
    setId: "u64",
    equivocation: "SpFinalityGrandpaEquivocation",
  },
  /** Lookup97: sp_finality_grandpa::Equivocation<primitive_types::H256, N> */
  SpFinalityGrandpaEquivocation: {
    _enum: {
      Prevote: "FinalityGrandpaEquivocationPrevote",
      Precommit: "FinalityGrandpaEquivocationPrecommit",
    },
  },
  /**
   * Lookup98: finality_grandpa::Equivocation<sp_finality_grandpa::app::Public,
   * finality_grandpa::Prevote<primitive_types::H256, N>,
   * sp_finality_grandpa::app::Signature>
   */
  FinalityGrandpaEquivocationPrevote: {
    roundNumber: "u64",
    identity: "SpFinalityGrandpaAppPublic",
    first: "(FinalityGrandpaPrevote,SpFinalityGrandpaAppSignature)",
    second: "(FinalityGrandpaPrevote,SpFinalityGrandpaAppSignature)",
  },
  /** Lookup99: finality_grandpa::Prevote<primitive_types::H256, N> */
  FinalityGrandpaPrevote: {
    targetHash: "H256",
    targetNumber: "u32",
  },
  /** Lookup100: sp_finality_grandpa::app::Signature */
  SpFinalityGrandpaAppSignature: "SpCoreEd25519Signature",
  /** Lookup101: sp_core::ed25519::Signature */
  SpCoreEd25519Signature: "[u8;64]",
  /**
   * Lookup104: finality_grandpa::Equivocation<sp_finality_grandpa::app::Public,
   * finality_grandpa::Precommit<primitive_types::H256, N>,
   * sp_finality_grandpa::app::Signature>
   */
  FinalityGrandpaEquivocationPrecommit: {
    roundNumber: "u64",
    identity: "SpFinalityGrandpaAppPublic",
    first: "(FinalityGrandpaPrecommitU32,SpFinalityGrandpaAppSignature)",
    second: "(FinalityGrandpaPrecommitU32,SpFinalityGrandpaAppSignature)",
  },
  /** Lookup105: finality_grandpa::Precommit<primitive_types::H256, N> */
  FinalityGrandpaPrecommitU32: {
    targetHash: "H256",
    targetNumber: "u32",
  },
  /** Lookup107: sp_core::Void */
  SpCoreVoid: "Null",
  /** Lookup108: pallet_grandpa::pallet::Error<T> */
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
  /** Lookup110: pallet_balances::BalanceLock<Balance> */
  PalletBalancesBalanceLock: {
    id: "[u8;8]",
    amount: "u128",
    reasons: "PalletBalancesReasons",
  },
  /** Lookup111: pallet_balances::Reasons */
  PalletBalancesReasons: {
    _enum: ["Fee", "Misc", "All"],
  },
  /** Lookup114: pallet_balances::ReserveData<ReserveIdentifier, Balance> */
  PalletBalancesReserveData: {
    id: "[u8;8]",
    amount: "u128",
  },
  /** Lookup116: pallet_balances::Releases */
  PalletBalancesReleases: {
    _enum: ["V1_0_0", "V2_0_0"],
  },
  /** Lookup117: pallet_balances::pallet::Call<T, I> */
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
  /** Lookup122: pallet_balances::pallet::Error<T, I> */
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
  /** Lookup124: pallet_transaction_payment::Releases */
  PalletTransactionPaymentReleases: {
    _enum: ["V1Ancient", "V2"],
  },
  /** Lookup126: frame_support::weights::WeightToFeeCoefficient<Balance> */
  FrameSupportWeightsWeightToFeeCoefficient: {
    coeffInteger: "u128",
    coeffFrac: "Perbill",
    negative: "bool",
    degree: "u8",
  },
  /** Lookup127: pallet_sudo::pallet::Call<T> */
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
  /** Lookup129: pallet_xdns::pallet::Call<T> */
  PalletXdnsCall: {
    _enum: {
      add_new_xdns_record: {
        url: "Bytes",
        gatewayId: "[u8;4]",
        gatewayAbi: "T3rnPrimitivesAbiGatewayABIConfig",
        gatewayVendor: "T3rnPrimitivesGatewayVendor",
        gatewayType: "T3rnPrimitivesGatewayType",
        gatewayGenesis: "T3rnPrimitivesGatewayGenesisConfig",
        gatewaySysProps: "T3rnPrimitivesGatewaySysProps",
        allowedSideEffects: "Vec<[u8;4]>",
      },
      add_side_effect: {
        id: "[u8;4]",
        name: "Bytes",
        argumentAbi: "Vec<T3rnPrimitivesAbiType>",
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
  /** Lookup130: t3rn_primitives::abi::GatewayABIConfig */
  T3rnPrimitivesAbiGatewayABIConfig: {
    blockNumberTypeSize: "u16",
    hashSize: "u16",
    hasher: "T3rnPrimitivesAbiHasherAlgo",
    crypto: "T3rnPrimitivesAbiCryptoAlgo",
    addressLength: "u16",
    valueTypeSize: "u16",
    decimals: "u16",
    structs: "Vec<T3rnPrimitivesAbiStructDecl>",
  },
  /** Lookup131: t3rn_primitives::abi::HasherAlgo */
  T3rnPrimitivesAbiHasherAlgo: {
    _enum: ["Blake2", "Keccak256"],
  },
  /** Lookup132: t3rn_primitives::abi::CryptoAlgo */
  T3rnPrimitivesAbiCryptoAlgo: {
    _enum: ["Ed25519", "Sr25519", "Ecdsa"],
  },
  /** Lookup134: t3rn_primitives::abi::StructDecl */
  T3rnPrimitivesAbiStructDecl: {
    name: "T3rnPrimitivesAbiType",
    fields: "Vec<T3rnPrimitivesAbiParameter>",
    offsets: "Vec<u16>",
  },
  /** Lookup135: t3rn_primitives::abi::Type */
  T3rnPrimitivesAbiType: {
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
      Mapping: "(T3rnPrimitivesAbiType,T3rnPrimitivesAbiType)",
      Contract: "Null",
      Ref: "T3rnPrimitivesAbiType",
      Option: "T3rnPrimitivesAbiType",
      OptionalInsurance: "Null",
      OptionalReward: "Null",
      StorageRef: "T3rnPrimitivesAbiType",
      Value: "Null",
      Slice: "Null",
      Hasher: "(T3rnPrimitivesAbiHasherAlgo,u16)",
      Crypto: "T3rnPrimitivesAbiCryptoAlgo",
    },
  },
  /** Lookup137: t3rn_primitives::abi::Parameter */
  T3rnPrimitivesAbiParameter: {
    name: "Option<Bytes>",
    ty: "T3rnPrimitivesAbiType",
    no: "u32",
    indexed: "Option<bool>",
  },
  /** Lookup140: t3rn_primitives::GatewayGenesisConfig */
  T3rnPrimitivesGatewayGenesisConfig: {
    modulesEncoded: "Option<Bytes>",
    extrinsicsVersion: "u8",
    genesisHash: "Bytes",
  },
  /** Lookup142: pallet_multi_finality_verifier::pallet::Call<T, I> */
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
        justification: {
          round: "u64",
          commit: "FinalityGrandpaCommitU32",
          votesAncestries:
            'Vec<{"parentHash":"H256","number":"Compact<u32>","stateRoot":"H256","extrinsicsRoot":"H256","digest":"SpRuntimeDigest"}>',
        },
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
        },
        gatewayId: "[u8;4]",
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
  /** Lookup144: sp_runtime::traits::BlakeTwo256 */
  SpRuntimeBlakeTwo256: "Null",
  /**
   * Lookup146: finality_grandpa::Commit<primitive_types::H256, N,
   * sp_finality_grandpa::app::Signature, sp_finality_grandpa::app::Public>
   */
  FinalityGrandpaCommitU32: {
    targetHash: "H256",
    targetNumber: "u32",
    precommits: "Vec<FinalityGrandpaSignedPrecommitU32>",
  },
  /**
   * Lookup148: finality_grandpa::SignedPrecommit<primitive_types::H256, N,
   * sp_finality_grandpa::app::Signature, sp_finality_grandpa::app::Public>
   */
  FinalityGrandpaSignedPrecommitU32: {
    precommit: "FinalityGrandpaPrecommitU32",
    signature: "SpFinalityGrandpaAppSignature",
    id: "SpFinalityGrandpaAppPublic",
  },
  /** Lookup154: sp_runtime::traits::Keccak256 */
  SpRuntimeKeccak256: "Null",
  /**
   * Lookup156: finality_grandpa::Commit<primitive_types::H256, N,
   * sp_finality_grandpa::app::Signature, sp_finality_grandpa::app::Public>
   */
  FinalityGrandpaCommitU64: {
    targetHash: "H256",
    targetNumber: "u64",
    precommits: "Vec<FinalityGrandpaSignedPrecommitU64>",
  },
  /**
   * Lookup158: finality_grandpa::SignedPrecommit<primitive_types::H256, N,
   * sp_finality_grandpa::app::Signature, sp_finality_grandpa::app::Public>
   */
  FinalityGrandpaSignedPrecommitU64: {
    precommit: "FinalityGrandpaPrecommitU64",
    signature: "SpFinalityGrandpaAppSignature",
    id: "SpFinalityGrandpaAppPublic",
  },
  /** Lookup159: finality_grandpa::Precommit<primitive_types::H256, N> */
  FinalityGrandpaPrecommitU64: {
    targetHash: "H256",
    targetNumber: "u64",
  },
  /** Lookup168: pallet_contracts_registry::pallet::Call<T> */
  PalletContractsRegistryCall: {
    _enum: {
      add_new_contract: {
        requester: "AccountId32",
        contract: "PalletContractsRegistryRegistryContract",
      },
      purge: {
        requester: "AccountId32",
        contractId: "H256",
      },
    },
  },
  /**
   * Lookup169:
   * pallet_contracts_registry::types::RegistryContract<primitive_types::H256,
   * sp_core::crypto::AccountId32, BalanceOf, BlockNumber>
   */
  PalletContractsRegistryRegistryContract: {
    codeTxt: "Bytes",
    bytes: "Bytes",
    author: "AccountId32",
    authorFeesPerSingleUse: "Option<u128>",
    abi: "Option<Bytes>",
    actionDescriptions: "Vec<T3rnPrimitivesAbiContractActionDesc>",
    info: "Option<T3rnPrimitivesStorageRawAliveContractInfo>",
    meta: "T3rnPrimitivesContractMetadata",
  },
  /**
   * Lookup171: t3rn_primitives::abi::ContractActionDesc<primitive_types::H256,
   * TargetId, sp_core::crypto::AccountId32>
   */
  T3rnPrimitivesAbiContractActionDesc: {
    actionId: "H256",
    targetId: "Option<[u8;4]>",
    to: "Option<AccountId32>",
  },
  /**
   * Lookup174:
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
  /** Lookup176: t3rn_primitives::contract_metadata::ContractMetadata */
  T3rnPrimitivesContractMetadata: {
    metadataVersion: "Bytes",
    name: "Bytes",
    version: "Bytes",
    authors: "Vec<Bytes>",
    description: "Option<Bytes>",
    documentation: "Option<Bytes>",
    repository: "Option<Bytes>",
    homepage: "Option<Bytes>",
    license: "Option<Bytes>",
  },
  /** Lookup177: pallet_circuit_portal::pallet::Call<T> */
  PalletCircuitPortalCall: {
    _enum: {
      register_gateway: {
        url: "Bytes",
        gatewayId: "[u8;4]",
        gatewayAbi: "T3rnPrimitivesAbiGatewayABIConfig",
        gatewayVendor: "T3rnPrimitivesGatewayVendor",
        gatewayType: "T3rnPrimitivesGatewayType",
        gatewayGenesis: "T3rnPrimitivesGatewayGenesisConfig",
        gatewaySysProps: "T3rnPrimitivesGatewaySysProps",
        firstHeader: "Bytes",
        authorities: "Option<Vec<AccountId32>>",
        allowedSideEffects: "Vec<[u8;4]>",
      },
      update_gateway: {
        gatewayId: "[u8;4]",
        url: "Option<Bytes>",
        gatewayAbi: "Option<T3rnPrimitivesAbiGatewayABIConfig>",
        gatewaySysProps: "Option<T3rnPrimitivesGatewaySysProps>",
        authorities: "Option<Vec<AccountId32>>",
        allowedSideEffects: "Option<Vec<[u8;4]>>",
      },
    },
  },
  /** Lookup182: pallet_circuit::pallet::Call<T> */
  PalletCircuitCall: {
    _enum: {
      on_local_trigger: "Null",
      on_xcm_trigger: "Null",
      on_remote_gateway_trigger: "Null",
      on_extrinsic_trigger: {
        sideEffects: "Vec<T3rnPrimitivesSideEffect>",
        fee: "u128",
        sequential: "bool",
      },
      bond_insurance_deposit: {
        xtxId: "H256",
        sideEffectId: "H256",
      },
      confirm_side_effect: {
        xtxId: "H256",
        sideEffect: "T3rnPrimitivesSideEffect",
        confirmation: "T3rnPrimitivesSideEffectConfirmedSideEffect",
        inclusionProof: "Option<Vec<Bytes>>",
        blockHash: "Option<Bytes>",
      },
    },
  },
  /** Lookup184: pallet_sudo::pallet::Error<T> */
  PalletSudoError: {
    _enum: ["RequireSudo"],
  },
  /** Lookup187: orml_tokens::BalanceLock<Balance> */
  OrmlTokensBalanceLock: {
    id: "[u8;8]",
    amount: "u128",
  },
  /** Lookup189: orml_tokens::AccountData<Balance> */
  OrmlTokensAccountData: {
    free: "u128",
    reserved: "u128",
    frozen: "u128",
  },
  /** Lookup190: orml_tokens::module::Error<T> */
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
  /** Lookup191: pallet_xdns::types::SideEffectInterface */
  PalletXdnsSideEffectInterface: {
    id: "[u8;4]",
    name: "Bytes",
    argumentAbi: "Vec<T3rnPrimitivesAbiType>",
    argumentToStateMapper: "Vec<Bytes>",
    confirmEvents: "Vec<Bytes>",
    escrowedEvents: "Vec<Bytes>",
    commitEvents: "Vec<Bytes>",
    revertEvents: "Vec<Bytes>",
  },
  /** Lookup192: pallet_xdns::types::XdnsRecord<sp_core::crypto::AccountId32> */
  PalletXdnsXdnsRecord: {
    url: "Bytes",
    gatewayAbi: "T3rnPrimitivesAbiGatewayABIConfig",
    gatewayGenesis: "T3rnPrimitivesGatewayGenesisConfig",
    gatewayVendor: "T3rnPrimitivesGatewayVendor",
    gatewayType: "T3rnPrimitivesGatewayType",
    gatewayId: "[u8;4]",
    gatewaySysProps: "T3rnPrimitivesGatewaySysProps",
    registrant: "Option<AccountId32>",
    lastFinalized: "Option<u64>",
    allowedSideEffects: "Vec<[u8;4]>",
  },
  /** Lookup193: pallet_xdns::pallet::Error<T> */
  PalletXdnsError: {
    _enum: [
      "XdnsRecordAlreadyExists",
      "UnknownXdnsRecord",
      "XdnsRecordNotFound",
      "SideEffectInterfaceAlreadyExists",
    ],
  },
  /** Lookup197: t3rn_primitives::bridges::header_chain::AuthoritySet */
  T3rnPrimitivesBridgesHeaderChainAuthoritySet: {
    authorities: "Vec<(SpFinalityGrandpaAppPublic,u64)>",
    setId: "u64",
  },
  /** Lookup198: pallet_multi_finality_verifier::pallet::Error<T, I> */
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
    ],
  },
  /** Lookup203: pallet_contracts_registry::pallet::Error<T> */
  PalletContractsRegistryError: {
    _enum: ["ContractAlreadyExists", "UnknownContract"],
  },
  /** Lookup204: pallet_circuit_portal::pallet::Error<T> */
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
      "ContractDoesNotExists",
      "RequesterNotEnoughBalance",
    ],
  },
  /**
   * Lookup205:
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
  /** Lookup206: pallet_circuit::state::CircuitStatus */
  PalletCircuitStateCircuitStatus: {
    _enum: [
      "Requested",
      "PendingInsurance",
      "Bonded",
      "Ready",
      "PendingExecution",
      "Finished",
      "Committed",
      "Reverted",
      "RevertedTimedOut",
    ],
  },
  /**
   * Lookup207: pallet_circuit::state::XExecSignal<sp_core::crypto::AccountId32,
   * BlockNumber, BalanceOf>
   */
  PalletCircuitStateXExecSignal: {
    requester: "AccountId32",
    timeoutsAt: "Option<u32>",
    delayStepsAt: "Option<Vec<u32>>",
    status: "PalletCircuitStateCircuitStatus",
    totalReward: "Option<u128>",
  },
  /** Lookup210: t3rn_primitives::volatile::LocalState */
  T3rnPrimitivesVolatileLocalState: {
    state: "BTreeMap<[u8;32], Bytes>",
  },
  /** Lookup214: frame_support::PalletId */
  FrameSupportPalletId: "[u8;8]",
  /** Lookup215: pallet_circuit::pallet::Error<T> */
  PalletCircuitError: {
    _enum: [
      "RequesterNotEnoughBalance",
      "ChargingTransferFailed",
      "RewardTransferFailed",
      "RefundTransferFailed",
      "InsuranceBondNotRequired",
      "InsuranceBondAlreadyDeposited",
      "SetupFailed",
      "SetupFailedIncorrectXtxStatus",
      "SetupFailedUnknownXtx",
      "SetupFailedDuplicatedXtx",
      "SetupFailedEmptyXtx",
      "ApplyFailed",
      "DeterminedForbiddenXtxStatus",
      "UnsupportedRole",
    ],
  },
  /** Lookup217: sp_runtime::MultiSignature */
  SpRuntimeMultiSignature: {
    _enum: {
      Ed25519: "SpCoreEd25519Signature",
      Sr25519: "SpCoreSr25519Signature",
      Ecdsa: "SpCoreEcdsaSignature",
    },
  },
  /** Lookup218: sp_core::sr25519::Signature */
  SpCoreSr25519Signature: "[u8;64]",
  /** Lookup219: sp_core::ecdsa::Signature */
  SpCoreEcdsaSignature: "[u8;65]",
  /** Lookup222: frame_system::extensions::check_non_zero_sender::CheckNonZeroSender<T> */
  FrameSystemExtensionsCheckNonZeroSender: "Null",
  /** Lookup223: frame_system::extensions::check_spec_version::CheckSpecVersion<T> */
  FrameSystemExtensionsCheckSpecVersion: "Null",
  /** Lookup224: frame_system::extensions::check_tx_version::CheckTxVersion<T> */
  FrameSystemExtensionsCheckTxVersion: "Null",
  /** Lookup225: frame_system::extensions::check_genesis::CheckGenesis<T> */
  FrameSystemExtensionsCheckGenesis: "Null",
  /** Lookup228: frame_system::extensions::check_nonce::CheckNonce<T> */
  FrameSystemExtensionsCheckNonce: "Compact<u32>",
  /** Lookup229: frame_system::extensions::check_weight::CheckWeight<T> */
  FrameSystemExtensionsCheckWeight: "Null",
  /** Lookup230: pallet_transaction_payment::ChargeTransactionPayment<T> */
  PalletTransactionPaymentChargeTransactionPayment: "Compact<u128>",
  /** Lookup231: circuit_standalone_runtime::Runtime */
  CircuitStandaloneRuntimeRuntime: "Null",
};
