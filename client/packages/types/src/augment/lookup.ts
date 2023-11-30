// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

/* eslint-disable sort-keys */

export default {
  /**
   * Lookup3: frame_system::AccountInfo<Nonce, pallet_balances::types::AccountData<Balance>>
   **/
  FrameSystemAccountInfo: {
    nonce: 'u32',
    consumers: 'u32',
    providers: 'u32',
    sufficients: 'u32',
    data: 'PalletBalancesAccountData'
  },
  /**
   * Lookup5: pallet_balances::types::AccountData<Balance>
   **/
  PalletBalancesAccountData: {
    free: 'u128',
    reserved: 'u128',
    frozen: 'u128',
    flags: 'u128'
  },
  /**
   * Lookup8: frame_support::dispatch::PerDispatchClass<sp_weights::weight_v2::Weight>
   **/
  FrameSupportDispatchPerDispatchClassWeight: {
    normal: 'SpWeightsWeightV2Weight',
    operational: 'SpWeightsWeightV2Weight',
    mandatory: 'SpWeightsWeightV2Weight'
  },
  /**
   * Lookup9: sp_weights::weight_v2::Weight
   **/
  SpWeightsWeightV2Weight: {
    refTime: 'Compact<u64>',
    proofSize: 'Compact<u64>'
  },
  /**
   * Lookup14: sp_runtime::generic::digest::Digest
   **/
  SpRuntimeDigest: {
    logs: 'Vec<SpRuntimeDigestDigestItem>'
  },
  /**
   * Lookup16: sp_runtime::generic::digest::DigestItem
   **/
  SpRuntimeDigestDigestItem: {
    _enum: {
      Other: 'Bytes',
      __Unused1: 'Null',
      __Unused2: 'Null',
      __Unused3: 'Null',
      Consensus: '([u8;4],Bytes)',
      Seal: '([u8;4],Bytes)',
      PreRuntime: '([u8;4],Bytes)',
      __Unused7: 'Null',
      RuntimeEnvironmentUpdated: 'Null'
    }
  },
  /**
   * Lookup19: frame_system::EventRecord<circuit_standalone_runtime::RuntimeEvent, primitive_types::H256>
   **/
  FrameSystemEventRecord: {
    phase: 'FrameSystemPhase',
    event: 'Event',
    topics: 'Vec<H256>'
  },
  /**
   * Lookup21: frame_system::pallet::Event<T>
   **/
  FrameSystemEvent: {
    _enum: {
      ExtrinsicSuccess: {
        dispatchInfo: 'FrameSupportDispatchDispatchInfo',
      },
      ExtrinsicFailed: {
        dispatchError: 'SpRuntimeDispatchError',
        dispatchInfo: 'FrameSupportDispatchDispatchInfo',
      },
      CodeUpdated: 'Null',
      NewAccount: {
        account: 'AccountId32',
      },
      KilledAccount: {
        account: 'AccountId32',
      },
      Remarked: {
        _alias: {
          hash_: 'hash',
        },
        sender: 'AccountId32',
        hash_: 'H256'
      }
    }
  },
  /**
   * Lookup22: frame_support::dispatch::DispatchInfo
   **/
  FrameSupportDispatchDispatchInfo: {
    weight: 'SpWeightsWeightV2Weight',
    class: 'FrameSupportDispatchDispatchClass',
    paysFee: 'FrameSupportDispatchPays'
  },
  /**
   * Lookup23: frame_support::dispatch::DispatchClass
   **/
  FrameSupportDispatchDispatchClass: {
    _enum: ['Normal', 'Operational', 'Mandatory']
  },
  /**
   * Lookup24: frame_support::dispatch::Pays
   **/
  FrameSupportDispatchPays: {
    _enum: ['Yes', 'No']
  },
  /**
   * Lookup25: sp_runtime::DispatchError
   **/
  SpRuntimeDispatchError: {
    _enum: {
      Other: 'Null',
      CannotLookup: 'Null',
      BadOrigin: 'Null',
      Module: 'SpRuntimeModuleError',
      ConsumerRemaining: 'Null',
      NoProviders: 'Null',
      TooManyConsumers: 'Null',
      Token: 'SpRuntimeTokenError',
      Arithmetic: 'SpArithmeticArithmeticError',
      Transactional: 'SpRuntimeTransactionalError',
      Exhausted: 'Null',
      Corruption: 'Null',
      Unavailable: 'Null',
      RootNotAllowed: 'Null'
    }
  },
  /**
   * Lookup26: sp_runtime::ModuleError
   **/
  SpRuntimeModuleError: {
    index: 'u8',
    error: '[u8;4]'
  },
  /**
   * Lookup27: sp_runtime::TokenError
   **/
  SpRuntimeTokenError: {
    _enum: ['FundsUnavailable', 'OnlyProvider', 'BelowMinimum', 'CannotCreate', 'UnknownAsset', 'Frozen', 'Unsupported', 'CannotCreateHold', 'NotExpendable', 'Blocked']
  },
  /**
   * Lookup28: sp_arithmetic::ArithmeticError
   **/
  SpArithmeticArithmeticError: {
    _enum: ['Underflow', 'Overflow', 'DivisionByZero']
  },
  /**
   * Lookup29: sp_runtime::TransactionalError
   **/
  SpRuntimeTransactionalError: {
    _enum: ['LimitReached', 'NoLayer']
  },
  /**
   * Lookup30: pallet_grandpa::pallet::Event
   **/
  PalletGrandpaEvent: {
    _enum: {
      NewAuthorities: {
        authoritySet: 'Vec<(SpConsensusGrandpaAppPublic,u64)>',
      },
      Paused: 'Null',
      Resumed: 'Null'
    }
  },
  /**
   * Lookup33: sp_consensus_grandpa::app::Public
   **/
  SpConsensusGrandpaAppPublic: 'SpCoreEd25519Public',
  /**
   * Lookup34: sp_core::ed25519::Public
   **/
  SpCoreEd25519Public: '[u8;32]',
  /**
   * Lookup35: pallet_utility::pallet::Event
   **/
  PalletUtilityEvent: {
    _enum: {
      BatchInterrupted: {
        index: 'u32',
        error: 'SpRuntimeDispatchError',
      },
      BatchCompleted: 'Null',
      BatchCompletedWithErrors: 'Null',
      ItemCompleted: 'Null',
      ItemFailed: {
        error: 'SpRuntimeDispatchError',
      },
      DispatchedAs: {
        result: 'Result<Null, SpRuntimeDispatchError>'
      }
    }
  },
  /**
   * Lookup38: pallet_balances::pallet::Event<T, I>
   **/
  PalletBalancesEvent: {
    _enum: {
      Endowed: {
        account: 'AccountId32',
        freeBalance: 'u128',
      },
      DustLost: {
        account: 'AccountId32',
        amount: 'u128',
      },
      Transfer: {
        from: 'AccountId32',
        to: 'AccountId32',
        amount: 'u128',
      },
      BalanceSet: {
        who: 'AccountId32',
        free: 'u128',
      },
      Reserved: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Unreserved: {
        who: 'AccountId32',
        amount: 'u128',
      },
      ReserveRepatriated: {
        from: 'AccountId32',
        to: 'AccountId32',
        amount: 'u128',
        destinationStatus: 'FrameSupportTokensMiscBalanceStatus',
      },
      Deposit: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Withdraw: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Slashed: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Minted: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Burned: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Suspended: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Restored: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Upgraded: {
        who: 'AccountId32',
      },
      Issued: {
        amount: 'u128',
      },
      Rescinded: {
        amount: 'u128',
      },
      Locked: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Unlocked: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Frozen: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Thawed: {
        who: 'AccountId32',
        amount: 'u128'
      }
    }
  },
  /**
   * Lookup39: frame_support::traits::tokens::misc::BalanceStatus
   **/
  FrameSupportTokensMiscBalanceStatus: {
    _enum: ['Free', 'Reserved']
  },
  /**
   * Lookup40: pallet_transaction_payment::pallet::Event<T>
   **/
  PalletTransactionPaymentEvent: {
    _enum: {
      TransactionFeePaid: {
        who: 'AccountId32',
        actualFee: 'u128',
        tip: 'u128'
      }
    }
  },
  /**
   * Lookup41: pallet_assets::pallet::Event<T, I>
   **/
  PalletAssetsEvent: {
    _enum: {
      Created: {
        assetId: 'u32',
        creator: 'AccountId32',
        owner: 'AccountId32',
      },
      Issued: {
        assetId: 'u32',
        owner: 'AccountId32',
        amount: 'u128',
      },
      Transferred: {
        assetId: 'u32',
        from: 'AccountId32',
        to: 'AccountId32',
        amount: 'u128',
      },
      Burned: {
        assetId: 'u32',
        owner: 'AccountId32',
        balance: 'u128',
      },
      TeamChanged: {
        assetId: 'u32',
        issuer: 'AccountId32',
        admin: 'AccountId32',
        freezer: 'AccountId32',
      },
      OwnerChanged: {
        assetId: 'u32',
        owner: 'AccountId32',
      },
      Frozen: {
        assetId: 'u32',
        who: 'AccountId32',
      },
      Thawed: {
        assetId: 'u32',
        who: 'AccountId32',
      },
      AssetFrozen: {
        assetId: 'u32',
      },
      AssetThawed: {
        assetId: 'u32',
      },
      AccountsDestroyed: {
        assetId: 'u32',
        accountsDestroyed: 'u32',
        accountsRemaining: 'u32',
      },
      ApprovalsDestroyed: {
        assetId: 'u32',
        approvalsDestroyed: 'u32',
        approvalsRemaining: 'u32',
      },
      DestructionStarted: {
        assetId: 'u32',
      },
      Destroyed: {
        assetId: 'u32',
      },
      ForceCreated: {
        assetId: 'u32',
        owner: 'AccountId32',
      },
      MetadataSet: {
        assetId: 'u32',
        name: 'Bytes',
        symbol: 'Bytes',
        decimals: 'u8',
        isFrozen: 'bool',
      },
      MetadataCleared: {
        assetId: 'u32',
      },
      ApprovedTransfer: {
        assetId: 'u32',
        source: 'AccountId32',
        delegate: 'AccountId32',
        amount: 'u128',
      },
      ApprovalCancelled: {
        assetId: 'u32',
        owner: 'AccountId32',
        delegate: 'AccountId32',
      },
      TransferredApproved: {
        assetId: 'u32',
        owner: 'AccountId32',
        delegate: 'AccountId32',
        destination: 'AccountId32',
        amount: 'u128',
      },
      AssetStatusChanged: {
        assetId: 'u32',
      },
      AssetMinBalanceChanged: {
        assetId: 'u32',
        newMinBalance: 'u128',
      },
      Touched: {
        assetId: 'u32',
        who: 'AccountId32',
        depositor: 'AccountId32',
      },
      Blocked: {
        assetId: 'u32',
        who: 'AccountId32'
      }
    }
  },
  /**
   * Lookup43: pallet_asset_tx_payment::pallet::Event<T>
   **/
  PalletAssetTxPaymentEvent: {
    _enum: {
      AssetTxFeePaid: {
        who: 'AccountId32',
        actualFee: 'u128',
        tip: 'u128',
        assetId: 'Option<u32>'
      }
    }
  },
  /**
   * Lookup45: pallet_account_manager::pallet::Event<T>
   **/
  PalletAccountManagerEvent: {
    _enum: {
      ContractsRegistryExecutionFinalized: {
        executionId: 'u64',
      },
      Issued: {
        recipient: 'AccountId32',
        amount: 'u128',
      },
      DepositReceived: {
        chargeId: 'H256',
        payee: 'AccountId32',
        recipient: 'Option<AccountId32>',
        amount: 'u128'
      }
    }
  },
  /**
   * Lookup47: pallet_treasury::pallet::Event<T, I>
   **/
  PalletTreasuryEvent: {
    _enum: {
      Proposed: {
        proposalIndex: 'u32',
      },
      Spending: {
        budgetRemaining: 'u128',
      },
      Awarded: {
        proposalIndex: 'u32',
        award: 'u128',
        account: 'AccountId32',
      },
      Rejected: {
        proposalIndex: 'u32',
        slashed: 'u128',
      },
      Burnt: {
        burntFunds: 'u128',
      },
      Rollover: {
        rolloverBalance: 'u128',
      },
      Deposit: {
        value: 'u128',
      },
      SpendApproved: {
        proposalIndex: 'u32',
        amount: 'u128',
        beneficiary: 'AccountId32',
      },
      UpdatedInactive: {
        reactivated: 'u128',
        deactivated: 'u128'
      }
    }
  },
  /**
   * Lookup52: pallet_clock::pallet::Event<T>
   **/
  PalletClockEvent: {
    _enum: {
      NewRound: {
        index: 'u32',
        head: 'u32',
        term: 'u32'
      }
    }
  },
  /**
   * Lookup53: pallet_xdns::pallet::Event<T>
   **/
  PalletXdnsEvent: {
    _enum: {
      GatewayRecordStored: '[u8;4]',
      NewTokenLinkedToGateway: '(u32,[u8;4])',
      NewTokenAssetRegistered: '(u32,[u8;4])',
      GatewayRecordPurged: '(AccountId32,[u8;4])',
      XdnsRecordPurged: '(AccountId32,[u8;4])',
      XdnsRecordUpdated: '[u8;4]',
      XDNSTopologyZip: 'T3rnPrimitivesXdnsTopology'
    }
  },
  /**
   * Lookup54: t3rn_primitives::XDNSTopology<sp_core::crypto::AccountId32>
   **/
  T3rnPrimitivesXdnsTopology: {
    gateways: 'Vec<T3rnPrimitivesXdnsFullGatewayRecord>',
    assets: 'Vec<T3rnPrimitivesXdnsTokenRecord>'
  },
  /**
   * Lookup56: t3rn_primitives::xdns::FullGatewayRecord<sp_core::crypto::AccountId32>
   **/
  T3rnPrimitivesXdnsFullGatewayRecord: {
    gatewayRecord: 'T3rnPrimitivesXdnsGatewayRecord',
    tokens: 'Vec<T3rnPrimitivesXdnsTokenRecord>'
  },
  /**
   * Lookup57: t3rn_primitives::xdns::GatewayRecord<sp_core::crypto::AccountId32>
   **/
  T3rnPrimitivesXdnsGatewayRecord: {
    gatewayId: '[u8;4]',
    verificationVendor: 'T3rnPrimitivesGatewayVendor',
    executionVendor: 'T3rnPrimitivesExecutionVendor',
    codec: 'T3rnAbiRecodeCodec',
    registrant: 'Option<AccountId32>',
    escrowAccount: 'Option<AccountId32>',
    allowedSideEffects: 'Vec<([u8;4],Option<u8>)>'
  },
  /**
   * Lookup58: t3rn_primitives::GatewayVendor
   **/
  T3rnPrimitivesGatewayVendor: {
    _enum: ['Polkadot', 'Kusama', 'Rococo', 'Ethereum', 'Sepolia', 'XBI', 'Attesters']
  },
  /**
   * Lookup59: t3rn_primitives::ExecutionVendor
   **/
  T3rnPrimitivesExecutionVendor: {
    _enum: ['Substrate', 'EVM']
  },
  /**
   * Lookup60: t3rn_abi::recode::Codec
   **/
  T3rnAbiRecodeCodec: {
    _enum: ['Scale', 'Rlp']
  },
  /**
   * Lookup65: t3rn_primitives::xdns::TokenRecord
   **/
  T3rnPrimitivesXdnsTokenRecord: {
    tokenId: 'u32',
    gatewayId: '[u8;4]',
    tokenProps: 'T3rnPrimitivesTokenInfo',
    tokenLocation: 'Option<XcmV3MultiLocation>'
  },
  /**
   * Lookup66: t3rn_primitives::TokenInfo
   **/
  T3rnPrimitivesTokenInfo: {
    _enum: {
      Substrate: 'T3rnPrimitivesSubstrateToken',
      Ethereum: 'T3rnPrimitivesEthereumToken'
    }
  },
  /**
   * Lookup67: t3rn_primitives::SubstrateToken
   **/
  T3rnPrimitivesSubstrateToken: {
    id: 'u32',
    symbol: 'Bytes',
    decimals: 'u8'
  },
  /**
   * Lookup68: t3rn_primitives::EthereumToken
   **/
  T3rnPrimitivesEthereumToken: {
    symbol: 'Bytes',
    decimals: 'u8',
    address: 'Option<[u8;20]>'
  },
  /**
   * Lookup72: xcm::v3::multilocation::MultiLocation
   **/
  XcmV3MultiLocation: {
    parents: 'u8',
    interior: 'XcmV3Junctions'
  },
  /**
   * Lookup73: xcm::v3::junctions::Junctions
   **/
  XcmV3Junctions: {
    _enum: {
      Here: 'Null',
      X1: 'XcmV3Junction',
      X2: '(XcmV3Junction,XcmV3Junction)',
      X3: '(XcmV3Junction,XcmV3Junction,XcmV3Junction)',
      X4: '(XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction)',
      X5: '(XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction)',
      X6: '(XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction)',
      X7: '(XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction)',
      X8: '(XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction)'
    }
  },
  /**
   * Lookup74: xcm::v3::junction::Junction
   **/
  XcmV3Junction: {
    _enum: {
      Parachain: 'Compact<u32>',
      AccountId32: {
        network: 'Option<XcmV3JunctionNetworkId>',
        id: '[u8;32]',
      },
      AccountIndex64: {
        network: 'Option<XcmV3JunctionNetworkId>',
        index: 'Compact<u64>',
      },
      AccountKey20: {
        network: 'Option<XcmV3JunctionNetworkId>',
        key: '[u8;20]',
      },
      PalletInstance: 'u8',
      GeneralIndex: 'Compact<u128>',
      GeneralKey: {
        length: 'u8',
        data: '[u8;32]',
      },
      OnlyChild: 'Null',
      Plurality: {
        id: 'XcmV3JunctionBodyId',
        part: 'XcmV3JunctionBodyPart',
      },
      GlobalConsensus: 'XcmV3JunctionNetworkId'
    }
  },
  /**
   * Lookup77: xcm::v3::junction::NetworkId
   **/
  XcmV3JunctionNetworkId: {
    _enum: {
      ByGenesis: '[u8;32]',
      ByFork: {
        blockNumber: 'u64',
        blockHash: '[u8;32]',
      },
      Polkadot: 'Null',
      Kusama: 'Null',
      Westend: 'Null',
      Rococo: 'Null',
      Wococo: 'Null',
      Ethereum: {
        chainId: 'Compact<u64>',
      },
      BitcoinCore: 'Null',
      BitcoinCash: 'Null'
    }
  },
  /**
   * Lookup79: xcm::v3::junction::BodyId
   **/
  XcmV3JunctionBodyId: {
    _enum: {
      Unit: 'Null',
      Moniker: '[u8;4]',
      Index: 'Compact<u32>',
      Executive: 'Null',
      Technical: 'Null',
      Legislative: 'Null',
      Judicial: 'Null',
      Defense: 'Null',
      Administration: 'Null',
      Treasury: 'Null'
    }
  },
  /**
   * Lookup80: xcm::v3::junction::BodyPart
   **/
  XcmV3JunctionBodyPart: {
    _enum: {
      Voice: 'Null',
      Members: {
        count: 'Compact<u32>',
      },
      Fraction: {
        nom: 'Compact<u32>',
        denom: 'Compact<u32>',
      },
      AtLeastProportion: {
        nom: 'Compact<u32>',
        denom: 'Compact<u32>',
      },
      MoreThanProportion: {
        nom: 'Compact<u32>',
        denom: 'Compact<u32>'
      }
    }
  },
  /**
   * Lookup81: pallet_attesters::pallet::Event<T>
   **/
  PalletAttestersEvent: {
    _enum: {
      AttesterRegistered: 'AccountId32',
      AttesterDeregistrationScheduled: '(AccountId32,u32)',
      AttesterDeregistered: 'AccountId32',
      AttestationSubmitted: 'AccountId32',
      BatchingFactorRead: 'Vec<([u8;4],Option<T3rnPrimitivesAttestersBatchingFactor>)>',
      BatchCommitted: '([u8;4],PalletAttestersBatchMessage,Bytes,H256,u128)',
      ConfirmationRewardCalculated: '([u8;4],u32,u128,Percent,Percent)',
      CollusionWithPermanentSlashDetected: '([u8;4],H256)',
      UserFinalityFeeEstimated: '([u8;4],u128)',
      NewAttestationBatch: '([u8;4],PalletAttestersBatchMessage)',
      NewAttestationMessageHash: '([u8;4],H256,T3rnPrimitivesExecutionVendor)',
      NewConfirmationBatch: '([u8;4],PalletAttestersBatchMessage,Bytes,H256)',
      Nominated: '(AccountId32,AccountId32,u128)',
      NewTargetActivated: '[u8;4]',
      NewTargetProposed: '[u8;4]',
      AttesterAgreedToNewTarget: '(AccountId32,[u8;4],Bytes)',
      CurrentPendingAttestationBatches: '([u8;4],Vec<(u32,H256)>)',
      AttestationsRemovedFromLateBatches: 'Vec<u32>',
      AttestationTargetRemoved: '([u8;4],Vec<[u8;4]>)',
      ShufflingCompleted: '(Vec<AccountId32>,Vec<AccountId32>,Vec<AccountId32>)'
    }
  },
  /**
   * Lookup85: t3rn_primitives::attesters::BatchingFactor
   **/
  T3rnPrimitivesAttestersBatchingFactor: {
    latestConfirmed: 'u16',
    latestSigned: 'u16',
    currentNext: 'u16',
    upToLast10Confirmed: 'Vec<u16>'
  },
  /**
   * Lookup88: pallet_attesters::pallet::BatchMessage<BlockNumber>
   **/
  PalletAttestersBatchMessage: {
    availableToCommitAt: 'u32',
    committedSfx: 'Option<Vec<H512>>',
    revertedSfx: 'Option<Vec<H256>>',
    nextCommittee: 'Option<Vec<Bytes>>',
    bannedCommittee: 'Option<Vec<Bytes>>',
    index: 'u32',
    signatures: 'Vec<(u32,[u8;65])>',
    created: 'u32',
    status: 'PalletAttestersBatchStatus',
    latency: 'T3rnPrimitivesAttestersLatencyStatus',
    halt: 'bool'
  },
  /**
   * Lookup100: pallet_attesters::pallet::BatchStatus
   **/
  PalletAttestersBatchStatus: {
    _enum: ['PendingMessage', 'PendingAttestation', 'ReadyForSubmissionByMajority', 'ReadyForSubmissionFullyApproved', 'Repatriated', 'Expired', 'Committed']
  },
  /**
   * Lookup101: t3rn_primitives::attesters::LatencyStatus
   **/
  T3rnPrimitivesAttestersLatencyStatus: {
    _enum: {
      OnTime: 'Null',
      Late: '(u32,u32)'
    }
  },
  /**
   * Lookup108: pallet_rewards::pallet::Event<T>
   **/
  PalletRewardsEvent: {
    _enum: {
      AttesterRewarded: '(AccountId32,u128)',
      CollatorRewarded: '(AccountId32,u128)',
      ExecutorRewarded: '(AccountId32,u128)',
      NewMaxRewardExecutorsKickbackSet: '(Percent,Percent)',
      Claimed: '(AccountId32,Vec<(u128,Option<u32>)>)',
      PendingClaim: '(AccountId32,u128)'
    }
  },
  /**
   * Lookup111: pallet_contracts_registry::pallet::Event<T>
   **/
  PalletContractsRegistryEvent: {
    _enum: {
      ContractStored: '(AccountId32,H256)',
      ContractPurged: '(AccountId32,H256)'
    }
  },
  /**
   * Lookup112: pallet_circuit::pallet::Event<T>
   **/
  PalletCircuitEvent: {
    _enum: {
      Transfer: '(AccountId32,AccountId32,AccountId32,u128)',
      TransferAssets: '(AccountId32,u32,AccountId32,AccountId32,u128)',
      TransferORML: '(AccountId32,u32,AccountId32,AccountId32,u128)',
      AddLiquidity: '(AccountId32,u32,u32,u128,u128,u128)',
      Swap: '(AccountId32,u32,u32,u128,u128,u128)',
      CallNative: '(AccountId32,Bytes)',
      CallEvm: '(AccountId32,H160,H160,U256,Bytes,u64,U256,Option<U256>,Option<U256>,Vec<(H160,Vec<H256>)>)',
      CallWasm: '(AccountId32,AccountId32,u128,u64,Option<u128>,Bytes)',
      CallCustom: '(AccountId32,AccountId32,AccountId32,u128,Bytes,u64,Bytes)',
      Result: '(AccountId32,AccountId32,PalletCircuitXbiResult,Bytes,Bytes)',
      XTransactionReceivedForExec: 'H256',
      SFXNewBidReceived: '(H256,AccountId32,u128)',
      SideEffectConfirmed: 'H256',
      XTransactionReadyForExec: 'H256',
      XTransactionStepFinishedExec: 'H256',
      XTransactionXtxFinishedExecAllSteps: 'H256',
      XTransactionFSXCommitted: 'H256',
      XTransactionXtxCommitted: 'H256',
      XTransactionXtxRevertedAfterTimeOut: 'H256',
      XTransactionXtxDroppedAtBidding: 'H256',
      NewSideEffectsAvailable: '(AccountId32,H256,Vec<T3rnTypesSfxSideEffect>,Vec<H256>)',
      CancelledSideEffects: '(AccountId32,H256,Vec<T3rnTypesSfxSideEffect>)',
      SideEffectsConfirmed: '(H256,Vec<Vec<T3rnTypesFsxFullSideEffect>>)',
      EscrowTransfer: '(AccountId32,AccountId32,u128)',
      SuccessfulFSXCommitAttestationRequest: 'H256',
      UnsuccessfulFSXCommitAttestationRequest: 'H256',
      SuccessfulFSXRevertAttestationRequest: 'H256',
      UnsuccessfulFSXRevertAttestationRequest: 'H256'
    }
  },
  /**
   * Lookup120: pallet_circuit::pallet::XbiResult
   **/
  PalletCircuitXbiResult: {
    status: 'PalletCircuitStatus',
    output: 'Bytes',
    witness: 'Bytes'
  },
  /**
   * Lookup121: pallet_circuit::pallet::Status
   **/
  PalletCircuitStatus: {
    _enum: ['Success', 'FailedExecution', 'DispatchFailed', 'ExecutionLimitExceeded', 'NotificationLimitExceeded', 'SendTimeout', 'DeliveryTimeout', 'ExecutionTimeout']
  },
  /**
   * Lookup123: t3rn_types::sfx::SideEffect<sp_core::crypto::AccountId32, BalanceOf>
   **/
  T3rnTypesSfxSideEffect: {
    target: '[u8;4]',
    maxReward: 'u128',
    insurance: 'u128',
    action: '[u8;4]',
    encodedArgs: 'Vec<Bytes>',
    signature: 'Bytes',
    enforceExecutor: 'Option<AccountId32>',
    rewardAssetId: 'Option<u32>'
  },
  /**
   * Lookup126: t3rn_types::fsx::FullSideEffect<sp_core::crypto::AccountId32, BlockNumber, BalanceOf>
   **/
  T3rnTypesFsxFullSideEffect: {
    input: 'T3rnTypesSfxSideEffect',
    confirmed: 'Option<T3rnTypesSfxConfirmedSideEffect>',
    securityLvl: 'T3rnTypesSfxSecurityLvl',
    submissionTargetHeight: 'u32',
    bestBid: 'Option<T3rnTypesBidSfxBid>',
    index: 'u32'
  },
  /**
   * Lookup128: t3rn_types::sfx::ConfirmedSideEffect<sp_core::crypto::AccountId32, BlockNumber, BalanceOf>
   **/
  T3rnTypesSfxConfirmedSideEffect: {
    err: 'Option<T3rnTypesSfxConfirmationOutcome>',
    output: 'Option<Bytes>',
    inclusionData: 'Bytes',
    executioner: 'AccountId32',
    receivedAt: 'u32',
    cost: 'Option<u128>'
  },
  /**
   * Lookup130: t3rn_types::sfx::ConfirmationOutcome
   **/
  T3rnTypesSfxConfirmationOutcome: {
    _enum: {
      Success: 'Null',
      MisbehaviourMalformedValues: {
        key: 'Bytes',
        expected: 'Bytes',
        received: 'Bytes',
      },
      TimedOut: 'Null'
    }
  },
  /**
   * Lookup132: t3rn_types::sfx::SecurityLvl
   **/
  T3rnTypesSfxSecurityLvl: {
    _enum: ['Optimistic', 'Escrow']
  },
  /**
   * Lookup134: t3rn_types::bid::SFXBid<sp_core::crypto::AccountId32, BalanceOf, AssetId>
   **/
  T3rnTypesBidSfxBid: {
    amount: 'u128',
    insurance: 'u128',
    reservedBond: 'Option<u128>',
    rewardAssetId: 'Option<u32>',
    executor: 'AccountId32',
    requester: 'AccountId32'
  },
  /**
   * Lookup135: pallet_circuit_vacuum::pallet::Event<T>
   **/
  PalletCircuitVacuumEvent: {
    _enum: {
      OrderStatusRead: 'PalletCircuitVacuumOrderStatusRead'
    }
  },
  /**
   * Lookup136: pallet_circuit_vacuum::OrderStatusRead<primitive_types::H256, BlockNumber, sp_core::crypto::AccountId32>
   **/
  PalletCircuitVacuumOrderStatusRead: {
    xtxId: 'H256',
    status: 'T3rnPrimitivesCircuitTypesCircuitStatus',
    allIncludedSfx: 'Vec<(H256,T3rnPrimitivesCircuitTypesCircuitStatus,Option<AccountId32>)>',
    timeoutsAt: 'T3rnPrimitivesCircuitTypesAdaptiveTimeout'
  },
  /**
   * Lookup137: t3rn_primitives::circuit::types::CircuitStatus
   **/
  T3rnPrimitivesCircuitTypesCircuitStatus: {
    _enum: {
      Requested: 'Null',
      Reserved: 'Null',
      PendingBidding: 'Null',
      InBidding: 'Null',
      Killed: 'T3rnPrimitivesCircuitTypesCause',
      Ready: 'Null',
      PendingExecution: 'Null',
      Finished: 'Null',
      FinishedAllSteps: 'Null',
      Reverted: 'T3rnPrimitivesCircuitTypesCause',
      Committed: 'Null'
    }
  },
  /**
   * Lookup138: t3rn_primitives::circuit::types::Cause
   **/
  T3rnPrimitivesCircuitTypesCause: {
    _enum: ['Timeout', 'IntentionalKill']
  },
  /**
   * Lookup141: t3rn_primitives::circuit::types::AdaptiveTimeout<BlockNumber, TargetId>
   **/
  T3rnPrimitivesCircuitTypesAdaptiveTimeout: {
    estimatedHeightHere: 'u32',
    estimatedHeightThere: 'u32',
    submitByHeightHere: 'u32',
    submitByHeightThere: 'u32',
    emergencyTimeoutHere: 'u32',
    there: '[u8;4]',
    dlq: 'Option<u32>'
  },
  /**
   * Lookup142: pallet_3vm::pallet::Event<T>
   **/
  Pallet3vmEvent: {
    _enum: {
      SignalBounced: '(u32,T3rnSdkPrimitivesSignalSignalKind,H256)',
      ExceededBounceThrehold: '(u32,T3rnSdkPrimitivesSignalSignalKind,H256)',
      ModuleInstantiated: '(H256,AccountId32,T3rnPrimitivesContractMetadataContractType,u32)',
      AuthorStored: '(AccountId32,AccountId32)',
      AuthorRemoved: 'AccountId32'
    }
  },
  /**
   * Lookup144: t3rn_sdk_primitives::signal::SignalKind
   **/
  T3rnSdkPrimitivesSignalSignalKind: {
    _enum: {
      Complete: 'Null',
      Kill: 'T3rnSdkPrimitivesSignalKillReason'
    }
  },
  /**
   * Lookup145: t3rn_sdk_primitives::signal::KillReason
   **/
  T3rnSdkPrimitivesSignalKillReason: {
    _enum: ['Unhandled', 'Codec', 'Timeout']
  },
  /**
   * Lookup147: t3rn_primitives::contract_metadata::ContractType
   **/
  T3rnPrimitivesContractMetadataContractType: {
    _enum: ['System', 'VanillaEvm', 'VanillaWasm', 'VolatileEvm', 'VolatileWasm']
  },
  /**
   * Lookup149: pallet_contracts::pallet::Event<T>
   **/
  PalletContractsEvent: {
    _enum: {
      Instantiated: {
        deployer: 'AccountId32',
        contract: 'AccountId32',
      },
      Terminated: {
        contract: 'AccountId32',
        beneficiary: 'AccountId32',
      },
      CodeStored: {
        codeHash: 'H256',
      },
      ContractEmitted: {
        contract: 'AccountId32',
        data: 'Bytes',
      },
      CodeRemoved: {
        codeHash: 'H256',
      },
      ContractCodeUpdated: {
        contract: 'AccountId32',
        newCodeHash: 'H256',
        oldCodeHash: 'H256',
      },
      Called: {
        caller: 'PalletContractsOrigin',
        contract: 'AccountId32',
      },
      DelegateCalled: {
        contract: 'AccountId32',
        codeHash: 'H256'
      }
    }
  },
  /**
   * Lookup150: pallet_contracts::Origin<circuit_standalone_runtime::Runtime>
   **/
  PalletContractsOrigin: {
    _enum: {
      Root: 'Null',
      Signed: 'AccountId32'
    }
  },
  /**
   * Lookup151: circuit_standalone_runtime::Runtime
   **/
  CircuitStandaloneRuntimeRuntime: 'Null',
  /**
   * Lookup152: pallet_evm::pallet::Event<T>
   **/
  PalletEvmEvent: {
    _enum: {
      Log: {
        log: 'EthereumLog',
      },
      Created: {
        address: 'H160',
      },
      CreatedFailed: {
        address: 'H160',
      },
      Executed: {
        address: 'H160',
      },
      ExecutedFailed: {
        address: 'H160'
      }
    }
  },
  /**
   * Lookup153: ethereum::log::Log
   **/
  EthereumLog: {
    address: 'H160',
    topics: 'Vec<H256>',
    data: 'Bytes'
  },
  /**
   * Lookup154: pallet_portal::pallet::Event<T>
   **/
  PalletPortalEvent: {
    _enum: {
      GatewayRegistered: '[u8;4]',
      SetOwner: '([u8;4],Bytes)',
      SetOperational: '([u8;4],bool)',
      HeaderSubmitted: '(T3rnPrimitivesGatewayVendor,Bytes)'
    }
  },
  /**
   * Lookup155: pallet_grandpa_finality_verifier::pallet::Event<T, I>
   **/
  PalletGrandpaFinalityVerifierEvent: {
    _enum: {
      HeadersAdded: 'u32'
    }
  },
  /**
   * Lookup158: pallet_eth2_finality_verifier::pallet::Event<T>
   **/
  PalletEth2FinalityVerifierEvent: {
    _enum: {
      EpochUpdate: 'PalletEth2FinalityVerifierEpochSubmitted'
    }
  },
  /**
   * Lookup159: pallet_eth2_finality_verifier::types::EpochSubmitted
   **/
  PalletEth2FinalityVerifierEpochSubmitted: {
    epoch: 'u64',
    beaconHeight: 'u64',
    executionHeight: 'u64'
  },
  /**
   * Lookup160: pallet_sepolia_finality_verifier::pallet::Event<T>
   **/
  PalletSepoliaFinalityVerifierEvent: {
    _enum: {
      EpochUpdate: 'PalletSepoliaFinalityVerifierEpochSubmitted'
    }
  },
  /**
   * Lookup161: pallet_sepolia_finality_verifier::types::EpochSubmitted
   **/
  PalletSepoliaFinalityVerifierEpochSubmitted: {
    epoch: 'u64',
    beaconHeight: 'u64',
    executionHeight: 'u64'
  },
  /**
   * Lookup162: pallet_identity::pallet::Event<T>
   **/
  PalletIdentityEvent: {
    _enum: {
      IdentitySet: {
        who: 'AccountId32',
      },
      IdentityCleared: {
        who: 'AccountId32',
        deposit: 'u128',
      },
      IdentityKilled: {
        who: 'AccountId32',
        deposit: 'u128',
      },
      JudgementRequested: {
        who: 'AccountId32',
        registrarIndex: 'u32',
      },
      JudgementUnrequested: {
        who: 'AccountId32',
        registrarIndex: 'u32',
      },
      JudgementGiven: {
        target: 'AccountId32',
        registrarIndex: 'u32',
      },
      RegistrarAdded: {
        registrarIndex: 'u32',
      },
      SubIdentityAdded: {
        sub: 'AccountId32',
        main: 'AccountId32',
        deposit: 'u128',
      },
      SubIdentityRemoved: {
        sub: 'AccountId32',
        main: 'AccountId32',
        deposit: 'u128',
      },
      SubIdentityRevoked: {
        sub: 'AccountId32',
        main: 'AccountId32',
        deposit: 'u128'
      }
    }
  },
  /**
   * Lookup163: pallet_maintenance_mode::pallet::Event
   **/
  PalletMaintenanceModeEvent: {
    _enum: {
      EnteredMaintenanceMode: 'Null',
      NormalOperationResumed: 'Null',
      FailedToSuspendIdleXcmExecution: {
        error: 'SpRuntimeDispatchError',
      },
      FailedToResumeIdleXcmExecution: {
        error: 'SpRuntimeDispatchError'
      }
    }
  },
  /**
   * Lookup164: pallet_sudo::pallet::Event<T>
   **/
  PalletSudoEvent: {
    _enum: {
      Sudid: {
        sudoResult: 'Result<Null, SpRuntimeDispatchError>',
      },
      KeyChanged: {
        oldSudoer: 'Option<AccountId32>',
      },
      SudoAsDone: {
        sudoResult: 'Result<Null, SpRuntimeDispatchError>'
      }
    }
  },
  /**
   * Lookup165: frame_system::Phase
   **/
  FrameSystemPhase: {
    _enum: {
      ApplyExtrinsic: 'u32',
      Finalization: 'Null',
      Initialization: 'Null'
    }
  },
  /**
   * Lookup168: frame_system::LastRuntimeUpgradeInfo
   **/
  FrameSystemLastRuntimeUpgradeInfo: {
    specVersion: 'Compact<u32>',
    specName: 'Text'
  },
  /**
   * Lookup170: frame_system::pallet::Call<T>
   **/
  FrameSystemCall: {
    _enum: {
      remark: {
        remark: 'Bytes',
      },
      set_heap_pages: {
        pages: 'u64',
      },
      set_code: {
        code: 'Bytes',
      },
      set_code_without_checks: {
        code: 'Bytes',
      },
      set_storage: {
        items: 'Vec<(Bytes,Bytes)>',
      },
      kill_storage: {
        _alias: {
          keys_: 'keys',
        },
        keys_: 'Vec<Bytes>',
      },
      kill_prefix: {
        prefix: 'Bytes',
        subkeys: 'u32',
      },
      remark_with_event: {
        remark: 'Bytes'
      }
    }
  },
  /**
   * Lookup173: frame_system::limits::BlockWeights
   **/
  FrameSystemLimitsBlockWeights: {
    baseBlock: 'SpWeightsWeightV2Weight',
    maxBlock: 'SpWeightsWeightV2Weight',
    perClass: 'FrameSupportDispatchPerDispatchClassWeightsPerClass'
  },
  /**
   * Lookup174: frame_support::dispatch::PerDispatchClass<frame_system::limits::WeightsPerClass>
   **/
  FrameSupportDispatchPerDispatchClassWeightsPerClass: {
    normal: 'FrameSystemLimitsWeightsPerClass',
    operational: 'FrameSystemLimitsWeightsPerClass',
    mandatory: 'FrameSystemLimitsWeightsPerClass'
  },
  /**
   * Lookup175: frame_system::limits::WeightsPerClass
   **/
  FrameSystemLimitsWeightsPerClass: {
    baseExtrinsic: 'SpWeightsWeightV2Weight',
    maxExtrinsic: 'Option<SpWeightsWeightV2Weight>',
    maxTotal: 'Option<SpWeightsWeightV2Weight>',
    reserved: 'Option<SpWeightsWeightV2Weight>'
  },
  /**
   * Lookup177: frame_system::limits::BlockLength
   **/
  FrameSystemLimitsBlockLength: {
    max: 'FrameSupportDispatchPerDispatchClassU32'
  },
  /**
   * Lookup178: frame_support::dispatch::PerDispatchClass<T>
   **/
  FrameSupportDispatchPerDispatchClassU32: {
    normal: 'u32',
    operational: 'u32',
    mandatory: 'u32'
  },
  /**
   * Lookup179: sp_weights::RuntimeDbWeight
   **/
  SpWeightsRuntimeDbWeight: {
    read: 'u64',
    write: 'u64'
  },
  /**
   * Lookup180: sp_version::RuntimeVersion
   **/
  SpVersionRuntimeVersion: {
    specName: 'Text',
    implName: 'Text',
    authoringVersion: 'u32',
    specVersion: 'u32',
    implVersion: 'u32',
    apis: 'Vec<([u8;8],u32)>',
    transactionVersion: 'u32',
    stateVersion: 'u8'
  },
  /**
   * Lookup185: frame_system::pallet::Error<T>
   **/
  FrameSystemError: {
    _enum: ['InvalidSpecName', 'SpecVersionNeedsToIncrease', 'FailedToExtractRuntimeVersion', 'NonDefaultComposite', 'NonZeroRefCount', 'CallFiltered']
  },
  /**
   * Lookup186: pallet_timestamp::pallet::Call<T>
   **/
  PalletTimestampCall: {
    _enum: {
      set: {
        now: 'Compact<u64>'
      }
    }
  },
  /**
   * Lookup188: sp_consensus_aura::sr25519::app_sr25519::Public
   **/
  SpConsensusAuraSr25519AppSr25519Public: 'SpCoreSr25519Public',
  /**
   * Lookup189: sp_core::sr25519::Public
   **/
  SpCoreSr25519Public: '[u8;32]',
  /**
   * Lookup192: pallet_grandpa::StoredState<N>
   **/
  PalletGrandpaStoredState: {
    _enum: {
      Live: 'Null',
      PendingPause: {
        scheduledAt: 'u32',
        delay: 'u32',
      },
      Paused: 'Null',
      PendingResume: {
        scheduledAt: 'u32',
        delay: 'u32'
      }
    }
  },
  /**
   * Lookup193: pallet_grandpa::StoredPendingChange<N, Limit>
   **/
  PalletGrandpaStoredPendingChange: {
    scheduledAt: 'u32',
    delay: 'u32',
    nextAuthorities: 'Vec<(SpConsensusGrandpaAppPublic,u64)>',
    forced: 'Option<u32>'
  },
  /**
   * Lookup195: pallet_grandpa::pallet::Call<T>
   **/
  PalletGrandpaCall: {
    _enum: {
      report_equivocation: {
        equivocationProof: 'SpConsensusGrandpaEquivocationProof',
        keyOwnerProof: 'SpCoreVoid',
      },
      report_equivocation_unsigned: {
        equivocationProof: 'SpConsensusGrandpaEquivocationProof',
        keyOwnerProof: 'SpCoreVoid',
      },
      note_stalled: {
        delay: 'u32',
        bestFinalizedBlockNumber: 'u32'
      }
    }
  },
  /**
   * Lookup196: sp_consensus_grandpa::EquivocationProof<primitive_types::H256, N>
   **/
  SpConsensusGrandpaEquivocationProof: {
    setId: 'u64',
    equivocation: 'SpConsensusGrandpaEquivocation'
  },
  /**
   * Lookup197: sp_consensus_grandpa::Equivocation<primitive_types::H256, N>
   **/
  SpConsensusGrandpaEquivocation: {
    _enum: {
      Prevote: 'FinalityGrandpaEquivocationPrevote',
      Precommit: 'FinalityGrandpaEquivocationPrecommit'
    }
  },
  /**
   * Lookup198: finality_grandpa::Equivocation<sp_consensus_grandpa::app::Public, finality_grandpa::Prevote<primitive_types::H256, N>, sp_consensus_grandpa::app::Signature>
   **/
  FinalityGrandpaEquivocationPrevote: {
    roundNumber: 'u64',
    identity: 'SpConsensusGrandpaAppPublic',
    first: '(FinalityGrandpaPrevote,SpConsensusGrandpaAppSignature)',
    second: '(FinalityGrandpaPrevote,SpConsensusGrandpaAppSignature)'
  },
  /**
   * Lookup199: finality_grandpa::Prevote<primitive_types::H256, N>
   **/
  FinalityGrandpaPrevote: {
    targetHash: 'H256',
    targetNumber: 'u32'
  },
  /**
   * Lookup200: sp_consensus_grandpa::app::Signature
   **/
  SpConsensusGrandpaAppSignature: 'SpCoreEd25519Signature',
  /**
   * Lookup201: sp_core::ed25519::Signature
   **/
  SpCoreEd25519Signature: '[u8;64]',
  /**
   * Lookup203: finality_grandpa::Equivocation<sp_consensus_grandpa::app::Public, finality_grandpa::Precommit<primitive_types::H256, N>, sp_consensus_grandpa::app::Signature>
   **/
  FinalityGrandpaEquivocationPrecommit: {
    roundNumber: 'u64',
    identity: 'SpConsensusGrandpaAppPublic',
    first: '(FinalityGrandpaPrecommit,SpConsensusGrandpaAppSignature)',
    second: '(FinalityGrandpaPrecommit,SpConsensusGrandpaAppSignature)'
  },
  /**
   * Lookup204: finality_grandpa::Precommit<primitive_types::H256, N>
   **/
  FinalityGrandpaPrecommit: {
    targetHash: 'H256',
    targetNumber: 'u32'
  },
  /**
   * Lookup206: sp_core::Void
   **/
  SpCoreVoid: 'Null',
  /**
   * Lookup207: pallet_grandpa::pallet::Error<T>
   **/
  PalletGrandpaError: {
    _enum: ['PauseFailed', 'ResumeFailed', 'ChangePending', 'TooSoon', 'InvalidKeyOwnershipProof', 'InvalidEquivocationProof', 'DuplicateOffenceReport']
  },
  /**
   * Lookup208: pallet_utility::pallet::Call<T>
   **/
  PalletUtilityCall: {
    _enum: {
      batch: {
        calls: 'Vec<Call>',
      },
      as_derivative: {
        index: 'u16',
        call: 'Call',
      },
      batch_all: {
        calls: 'Vec<Call>',
      },
      dispatch_as: {
        asOrigin: 'CircuitStandaloneRuntimeOriginCaller',
        call: 'Call',
      },
      force_batch: {
        calls: 'Vec<Call>',
      },
      with_weight: {
        call: 'Call',
        weight: 'SpWeightsWeightV2Weight'
      }
    }
  },
  /**
   * Lookup211: pallet_balances::pallet::Call<T, I>
   **/
  PalletBalancesCall: {
    _enum: {
      transfer_allow_death: {
        dest: 'MultiAddress',
        value: 'Compact<u128>',
      },
      set_balance_deprecated: {
        who: 'MultiAddress',
        newFree: 'Compact<u128>',
        oldReserved: 'Compact<u128>',
      },
      force_transfer: {
        source: 'MultiAddress',
        dest: 'MultiAddress',
        value: 'Compact<u128>',
      },
      transfer_keep_alive: {
        dest: 'MultiAddress',
        value: 'Compact<u128>',
      },
      transfer_all: {
        dest: 'MultiAddress',
        keepAlive: 'bool',
      },
      force_unreserve: {
        who: 'MultiAddress',
        amount: 'u128',
      },
      upgrade_accounts: {
        who: 'Vec<AccountId32>',
      },
      transfer: {
        dest: 'MultiAddress',
        value: 'Compact<u128>',
      },
      force_set_balance: {
        who: 'MultiAddress',
        newFree: 'Compact<u128>'
      }
    }
  },
  /**
   * Lookup214: pallet_assets::pallet::Call<T, I>
   **/
  PalletAssetsCall: {
    _enum: {
      create: {
        id: 'u32',
        admin: 'MultiAddress',
        minBalance: 'u128',
      },
      force_create: {
        id: 'u32',
        owner: 'MultiAddress',
        isSufficient: 'bool',
        minBalance: 'Compact<u128>',
      },
      start_destroy: {
        id: 'u32',
      },
      destroy_accounts: {
        id: 'u32',
      },
      destroy_approvals: {
        id: 'u32',
      },
      finish_destroy: {
        id: 'u32',
      },
      mint: {
        id: 'u32',
        beneficiary: 'MultiAddress',
        amount: 'Compact<u128>',
      },
      burn: {
        id: 'u32',
        who: 'MultiAddress',
        amount: 'Compact<u128>',
      },
      transfer: {
        id: 'u32',
        target: 'MultiAddress',
        amount: 'Compact<u128>',
      },
      transfer_keep_alive: {
        id: 'u32',
        target: 'MultiAddress',
        amount: 'Compact<u128>',
      },
      force_transfer: {
        id: 'u32',
        source: 'MultiAddress',
        dest: 'MultiAddress',
        amount: 'Compact<u128>',
      },
      freeze: {
        id: 'u32',
        who: 'MultiAddress',
      },
      thaw: {
        id: 'u32',
        who: 'MultiAddress',
      },
      freeze_asset: {
        id: 'u32',
      },
      thaw_asset: {
        id: 'u32',
      },
      transfer_ownership: {
        id: 'u32',
        owner: 'MultiAddress',
      },
      set_team: {
        id: 'u32',
        issuer: 'MultiAddress',
        admin: 'MultiAddress',
        freezer: 'MultiAddress',
      },
      set_metadata: {
        id: 'u32',
        name: 'Bytes',
        symbol: 'Bytes',
        decimals: 'u8',
      },
      clear_metadata: {
        id: 'u32',
      },
      force_set_metadata: {
        id: 'u32',
        name: 'Bytes',
        symbol: 'Bytes',
        decimals: 'u8',
        isFrozen: 'bool',
      },
      force_clear_metadata: {
        id: 'u32',
      },
      force_asset_status: {
        id: 'u32',
        owner: 'MultiAddress',
        issuer: 'MultiAddress',
        admin: 'MultiAddress',
        freezer: 'MultiAddress',
        minBalance: 'Compact<u128>',
        isSufficient: 'bool',
        isFrozen: 'bool',
      },
      approve_transfer: {
        id: 'u32',
        delegate: 'MultiAddress',
        amount: 'Compact<u128>',
      },
      cancel_approval: {
        id: 'u32',
        delegate: 'MultiAddress',
      },
      force_cancel_approval: {
        id: 'u32',
        owner: 'MultiAddress',
        delegate: 'MultiAddress',
      },
      transfer_approved: {
        id: 'u32',
        owner: 'MultiAddress',
        destination: 'MultiAddress',
        amount: 'Compact<u128>',
      },
      touch: {
        id: 'u32',
      },
      refund: {
        id: 'u32',
        allowBurn: 'bool',
      },
      set_min_balance: {
        id: 'u32',
        minBalance: 'u128',
      },
      touch_other: {
        id: 'u32',
        who: 'MultiAddress',
      },
      refund_other: {
        id: 'u32',
        who: 'MultiAddress',
      },
      block: {
        id: 'u32',
        who: 'MultiAddress'
      }
    }
  },
  /**
   * Lookup215: pallet_account_manager::pallet::Call<T>
   **/
  PalletAccountManagerCall: {
    _enum: {
      deposit: {
        chargeId: 'H256',
        payee: 'AccountId32',
        chargeFee: 'u128',
        offeredReward: 'u128',
        source: 'T3rnPrimitivesClaimableBenefitSource',
        role: 'T3rnPrimitivesClaimableCircuitRole',
        recipient: 'Option<AccountId32>',
        maybeAssetId: 'Option<u32>',
      },
      finalize: {
        chargeId: 'H256',
        outcome: 'T3rnPrimitivesAccountManagerOutcome',
        maybeRecipient: 'Option<AccountId32>',
        maybeActualFees: 'Option<u128>'
      }
    }
  },
  /**
   * Lookup216: t3rn_primitives::claimable::BenefitSource
   **/
  T3rnPrimitivesClaimableBenefitSource: {
    _enum: ['BootstrapPool', 'Inflation', 'TrafficFees', 'TrafficRewards', 'Unsettled', 'SlashTreasury']
  },
  /**
   * Lookup217: t3rn_primitives::claimable::CircuitRole
   **/
  T3rnPrimitivesClaimableCircuitRole: {
    _enum: ['Ambassador', 'Executor', 'Attester', 'Staker', 'Collator', 'ContractAuthor', 'Relayer', 'Requester', 'Local']
  },
  /**
   * Lookup218: t3rn_primitives::account_manager::Outcome
   **/
  T3rnPrimitivesAccountManagerOutcome: {
    _enum: ['UnexpectedFailure', 'Revert', 'Commit', 'Slash']
  },
  /**
   * Lookup219: pallet_treasury::pallet::Call<T, I>
   **/
  PalletTreasuryCall: {
    _enum: {
      propose_spend: {
        value: 'Compact<u128>',
        beneficiary: 'MultiAddress',
      },
      reject_proposal: {
        proposalId: 'Compact<u32>',
      },
      approve_proposal: {
        proposalId: 'Compact<u32>',
      },
      spend: {
        amount: 'Compact<u128>',
        beneficiary: 'MultiAddress',
      },
      remove_approval: {
        proposalId: 'Compact<u32>'
      }
    }
  },
  /**
   * Lookup224: pallet_xdns::pallet::Call<T>
   **/
  PalletXdnsCall: {
    _enum: {
      reboot_self_gateway: {
        vendor: 'T3rnPrimitivesGatewayVendor',
      },
      add_supported_bridging_asset: {
        assetId: 'u32',
        targetId: '[u8;4]',
      },
      enroll_bridge_asset: {
        assetId: 'u32',
        targetId: '[u8;4]',
        tokenInfo: 'T3rnPrimitivesTokenInfo',
        tokenLocation: 'Option<XcmV3MultiLocation>',
      },
      enroll_new_abi_to_selected_gateway: {
        targetId: '[u8;4]',
        sfx4bId: '[u8;4]',
        sfxExpectedAbi: 'Option<T3rnAbiSfxAbi>',
        maybePalletId: 'Option<u8>',
      },
      unroll_abi_of_selected_gateway: {
        targetId: '[u8;4]',
        sfx4bId: '[u8;4]',
      },
      add_remote_order_address: {
        targetId: '[u8;4]',
        remoteAddress: 'H256',
      },
      purge_supported_bridging_asset: {
        assetId: 'u32',
        targetId: '[u8;4]',
      },
      purge_gateway_record: {
        requester: 'AccountId32',
        gatewayId: '[u8;4]',
      },
      unlink_token: {
        gatewayId: '[u8;4]',
        tokenId: 'u32',
      },
      link_token: {
        gatewayId: '[u8;4]',
        tokenId: 'u32',
        tokenProps: 'T3rnPrimitivesTokenInfo',
        tokenLocation: 'Option<XcmV3MultiLocation>',
      },
      purge_token_record: {
        tokenId: 'u32',
      },
      zip_topology: 'Null',
      unzip_topology: {
        topologyDecoded: 'Option<T3rnPrimitivesXdnsTopology>',
        topologyEncoded: 'Option<Bytes>',
      },
      force_add_new_gateway: {
        gatewayId: '[u8;4]',
        verificationVendor: 'T3rnPrimitivesGatewayVendor',
        executionVendor: 'T3rnPrimitivesExecutionVendor',
        codec: 'T3rnAbiRecodeCodec',
        registrant: 'Option<AccountId32>',
        escrowAccount: 'Option<AccountId32>',
        allowedSideEffects: 'Vec<([u8;4],Option<u8>)>'
      }
    }
  },
  /**
   * Lookup226: t3rn_abi::sfx_abi::SFXAbi
   **/
  T3rnAbiSfxAbi: {
    argsNames: 'Vec<(Bytes,bool)>',
    maybePrefixMemo: 'Option<u8>',
    egressAbiDescriptors: 'T3rnAbiSfxAbiPerCodecAbiDescriptors',
    ingressAbiDescriptors: 'T3rnAbiSfxAbiPerCodecAbiDescriptors'
  },
  /**
   * Lookup229: t3rn_abi::sfx_abi::PerCodecAbiDescriptors
   **/
  T3rnAbiSfxAbiPerCodecAbiDescriptors: {
    forRlp: 'Bytes',
    forScale: 'Bytes'
  },
  /**
   * Lookup231: pallet_attesters::pallet::Call<T>
   **/
  PalletAttestersCall: {
    _enum: {
      register_attester: {
        selfNominateAmount: 'u128',
        ecdsaKey: '[u8;33]',
        ed25519Key: '[u8;32]',
        sr25519Key: '[u8;32]',
        customCommission: 'Option<Percent>',
      },
      register_invulnerable_attester: {
        selfNominateAmount: 'u128',
        ecdsaKey: '[u8;33]',
        ed25519Key: '[u8;32]',
        sr25519Key: '[u8;32]',
        customCommission: 'Option<Percent>',
      },
      deregister_attester: 'Null',
      remove_attestation_target: {
        target: '[u8;4]',
      },
      agree_to_new_attestation_target: {
        target: '[u8;4]',
        recoverable: 'Bytes',
      },
      force_activate_target: {
        target: '[u8;4]',
      },
      add_attestation_target: {
        target: '[u8;4]',
      },
      submit_for_influx_attestation: {
        message: 'H256',
        messageHash: 'H256',
        heightThere: 'u32',
        target: '[u8;4]',
        signature: 'Bytes',
      },
      submit_attestation: {
        message: 'H256',
        signature: 'Bytes',
        target: '[u8;4]',
      },
      commit_batch: {
        target: '[u8;4]',
        targetInclusionProofEncoded: 'Bytes',
      },
      read_pending_batches: 'Null',
      read_latest_batching_factor_overview: 'Null',
      estimate_user_finality_fee: {
        target: '[u8;4]',
      },
      nominate: {
        attester: 'AccountId32',
        amount: 'u128',
      },
      unnominate: {
        attester: 'AccountId32'
      }
    }
  },
  /**
   * Lookup234: pallet_rewards::pallet::Call<T>
   **/
  PalletRewardsCall: {
    _enum: {
      set_max_rewards_executors_kickback: {
        newKickback: 'Percent',
      },
      trigger_distribution: 'Null',
      turn_on_off_distribution: 'Null',
      turn_on_off_claims: 'Null',
      turn_on_off_settlement_accumulation: 'Null',
      claim: {
        roleToClaim: 'Option<T3rnPrimitivesClaimableCircuitRole>'
      }
    }
  },
  /**
   * Lookup236: pallet_contracts_registry::pallet::Call<T>
   **/
  PalletContractsRegistryCall: {
    _enum: {
      add_new_contract: {
        requester: 'AccountId32',
        contract: 'T3rnPrimitivesContractsRegistryRegistryContract',
      },
      purge: {
        requester: 'AccountId32',
        contractId: 'H256'
      }
    }
  },
  /**
   * Lookup237: t3rn_primitives::contracts_registry::RegistryContract<primitive_types::H256, sp_core::crypto::AccountId32, BalanceOf, BlockNumber>
   **/
  T3rnPrimitivesContractsRegistryRegistryContract: {
    codeTxt: 'Bytes',
    bytes: 'Bytes',
    author: 'T3rnPrimitivesContractsRegistryAuthorInfo',
    abi: 'Option<Bytes>',
    actionDescriptions: 'Vec<T3rnTypesGatewayContractActionDesc>',
    info: 'Option<T3rnPrimitivesStorageRawAliveContractInfo>',
    meta: 'T3rnPrimitivesContractMetadata'
  },
  /**
   * Lookup238: t3rn_primitives::contracts_registry::AuthorInfo<sp_core::crypto::AccountId32, BalanceOf>
   **/
  T3rnPrimitivesContractsRegistryAuthorInfo: {
    account: 'AccountId32',
    feesPerSingleUse: 'Option<u128>'
  },
  /**
   * Lookup240: t3rn_types::gateway::ContractActionDesc<primitive_types::H256, TargetId, sp_core::crypto::AccountId32>
   **/
  T3rnTypesGatewayContractActionDesc: {
    actionId: 'H256',
    targetId: 'Option<[u8;4]>',
    to: 'Option<AccountId32>'
  },
  /**
   * Lookup243: t3rn_primitives::storage::RawAliveContractInfo<primitive_types::H256, Balance, BlockNumber>
   **/
  T3rnPrimitivesStorageRawAliveContractInfo: {
    trieId: 'Bytes',
    storageSize: 'u32',
    pairCount: 'u32',
    codeHash: 'H256',
    rentAllowance: 'u128',
    rentPaid: 'u128',
    deductBlock: 'u32',
    lastWrite: 'Option<u32>',
    reserved: 'Option<Null>'
  },
  /**
   * Lookup245: t3rn_primitives::contract_metadata::ContractMetadata
   **/
  T3rnPrimitivesContractMetadata: {
    metadataVersion: 'Bytes',
    name: 'Bytes',
    contractType: 'T3rnPrimitivesContractMetadataContractType',
    version: 'Bytes',
    authors: 'Vec<Bytes>',
    description: 'Option<Bytes>',
    documentation: 'Option<Bytes>',
    repository: 'Option<Bytes>',
    homepage: 'Option<Bytes>',
    license: 'Option<Bytes>'
  },
  /**
   * Lookup246: pallet_circuit::pallet::Call<T>
   **/
  PalletCircuitCall: {
    _enum: {
      on_local_trigger: {
        trigger: 'Bytes',
      },
      on_xcm_trigger: 'Null',
      on_remote_gateway_trigger: 'Null',
      cancel_xtx: {
        xtxId: 'H256',
      },
      revert: {
        xtxId: 'H256',
      },
      trigger_dlq: 'Null',
      on_remote_origin_trigger: {
        orderOrigin: 'AccountId32',
        sideEffects: 'Vec<T3rnTypesSfxSideEffect>',
        speedMode: 'T3rnPrimitivesSpeedMode',
      },
      on_extrinsic_trigger: {
        sideEffects: 'Vec<T3rnTypesSfxSideEffect>',
        speedMode: 'T3rnPrimitivesSpeedMode',
        preferredSecurityLevel: 'T3rnTypesSfxSecurityLvl',
      },
      bid_sfx: {
        sfxId: 'H256',
        bidAmount: 'u128',
      },
      confirm_side_effect: {
        sfxId: 'H256',
        confirmation: 'T3rnTypesSfxConfirmedSideEffect'
      }
    }
  },
  /**
   * Lookup247: t3rn_primitives::SpeedMode
   **/
  T3rnPrimitivesSpeedMode: {
    _enum: ['Fast', 'Rational', 'Finalized', 'Instant']
  },
  /**
   * Lookup248: pallet_circuit_vacuum::pallet::Call<T>
   **/
  PalletCircuitVacuumCall: {
    _enum: {
      order: {
        sfxActions: 'Vec<T3rnPrimitivesCircuitTypesOrderSFX>',
        speedMode: 'T3rnPrimitivesSpeedMode',
      },
      single_order: {
        destination: '[u8;4]',
        asset: 'u32',
        amount: 'u128',
        rewardAsset: 'u32',
        maxReward: 'u128',
        insurance: 'u128',
        targetAccount: 'AccountId32',
        speedMode: 'T3rnPrimitivesSpeedMode',
      },
      remote_order: {
        orderRemoteProof: 'Bytes',
        remoteTargetId: '[u8;4]',
        speedMode: 'T3rnPrimitivesSpeedMode',
      },
      read_order_status: {
        xtxId: 'H256',
      },
      read_all_pending_orders_status: 'Null'
    }
  },
  /**
   * Lookup250: t3rn_primitives::circuit::types::OrderSFX<sp_core::crypto::AccountId32, Asset, Balance, Destination, Input, MaxCost>
   **/
  T3rnPrimitivesCircuitTypesOrderSFX: {
    sfxAction: 'T3rnPrimitivesCircuitTypesSfxAction',
    maxReward: 'u128',
    rewardAsset: 'u32',
    insurance: 'u128',
    remoteOriginNonce: 'Option<u32>'
  },
  /**
   * Lookup251: t3rn_primitives::circuit::types::SFXAction<sp_core::crypto::AccountId32, Asset, Balance, Destination, Input, MaxCost>
   **/
  T3rnPrimitivesCircuitTypesSfxAction: {
    _enum: {
      Call: '([u8;4],AccountId32,u128,u128,Bytes)',
      Transfer: '([u8;4],u32,AccountId32,u128)'
    }
  },
  /**
   * Lookup252: pallet_3vm::pallet::Call<T>
   **/
  Pallet3vmCall: 'Null',
  /**
   * Lookup253: pallet_contracts::pallet::Call<T>
   **/
  PalletContractsCall: {
    _enum: {
      call_old_weight: {
        dest: 'MultiAddress',
        value: 'Compact<u128>',
        gasLimit: 'Compact<u64>',
        storageDepositLimit: 'Option<Compact<u128>>',
        data: 'Bytes',
      },
      instantiate_with_code_old_weight: {
        value: 'Compact<u128>',
        gasLimit: 'Compact<u64>',
        storageDepositLimit: 'Option<Compact<u128>>',
        code: 'Bytes',
        data: 'Bytes',
        salt: 'Bytes',
      },
      instantiate_old_weight: {
        value: 'Compact<u128>',
        gasLimit: 'Compact<u64>',
        storageDepositLimit: 'Option<Compact<u128>>',
        codeHash: 'H256',
        data: 'Bytes',
        salt: 'Bytes',
      },
      upload_code: {
        code: 'Bytes',
        storageDepositLimit: 'Option<Compact<u128>>',
        determinism: 'PalletContractsWasmDeterminism',
      },
      remove_code: {
        codeHash: 'H256',
      },
      set_code: {
        dest: 'MultiAddress',
        codeHash: 'H256',
      },
      call: {
        dest: 'MultiAddress',
        value: 'Compact<u128>',
        gasLimit: 'SpWeightsWeightV2Weight',
        storageDepositLimit: 'Option<Compact<u128>>',
        data: 'Bytes',
      },
      instantiate_with_code: {
        value: 'Compact<u128>',
        gasLimit: 'SpWeightsWeightV2Weight',
        storageDepositLimit: 'Option<Compact<u128>>',
        code: 'Bytes',
        data: 'Bytes',
        salt: 'Bytes',
      },
      instantiate: {
        value: 'Compact<u128>',
        gasLimit: 'SpWeightsWeightV2Weight',
        storageDepositLimit: 'Option<Compact<u128>>',
        codeHash: 'H256',
        data: 'Bytes',
        salt: 'Bytes',
      },
      migrate: {
        weightLimit: 'SpWeightsWeightV2Weight'
      }
    }
  },
  /**
   * Lookup255: pallet_contracts::wasm::Determinism
   **/
  PalletContractsWasmDeterminism: {
    _enum: ['Enforced', 'Relaxed']
  },
  /**
   * Lookup256: pallet_evm::pallet::Call<T>
   **/
  PalletEvmCall: {
    _enum: {
      withdraw: {
        address: 'H160',
        value: 'u128',
      },
      call: {
        source: 'H160',
        target: 'H160',
        input: 'Bytes',
        value: 'U256',
        gasLimit: 'u64',
        maxFeePerGas: 'U256',
        maxPriorityFeePerGas: 'Option<U256>',
        nonce: 'Option<U256>',
        accessList: 'Vec<(H160,Vec<H256>)>',
      },
      create: {
        source: 'H160',
        init: 'Bytes',
        value: 'U256',
        gasLimit: 'u64',
        maxFeePerGas: 'U256',
        maxPriorityFeePerGas: 'Option<U256>',
        nonce: 'Option<U256>',
        accessList: 'Vec<(H160,Vec<H256>)>',
      },
      create2: {
        source: 'H160',
        init: 'Bytes',
        salt: 'H256',
        value: 'U256',
        gasLimit: 'u64',
        maxFeePerGas: 'U256',
        maxPriorityFeePerGas: 'Option<U256>',
        nonce: 'Option<U256>',
        accessList: 'Vec<(H160,Vec<H256>)>'
      }
    }
  },
  /**
   * Lookup257: pallet_portal::pallet::Call<T>
   **/
  PalletPortalCall: {
    _enum: {
      register_gateway: {
        gatewayId: '[u8;4]',
        tokenId: 'u32',
        verificationVendor: 'T3rnPrimitivesGatewayVendor',
        executionVendor: 'T3rnPrimitivesExecutionVendor',
        codec: 'T3rnAbiRecodeCodec',
        registrant: 'Option<AccountId32>',
        escrowAccount: 'Option<AccountId32>',
        allowedSideEffects: 'Vec<([u8;4],Option<u8>)>',
        tokenProps: 'T3rnPrimitivesTokenInfo',
        tokenLocation: 'Option<XcmV3MultiLocation>',
        encodedRegistrationData: 'Bytes'
      }
    }
  },
  /**
   * Lookup258: pallet_grandpa_finality_verifier::pallet::Call<T, I>
   **/
  PalletGrandpaFinalityVerifierCall: {
    _enum: {
      submit_headers: {
        range: 'Vec<SpRuntimeHeader>',
        signedHeader: 'SpRuntimeHeader',
        justification: 'PalletGrandpaFinalityVerifierBridgesHeaderChainJustificationGrandpaJustification',
      },
      reset: 'Null'
    }
  },
  /**
   * Lookup260: sp_runtime::generic::header::Header<Number, Hash>
   **/
  SpRuntimeHeader: {
    parentHash: 'H256',
    number: 'Compact<u32>',
    stateRoot: 'H256',
    extrinsicsRoot: 'H256',
    digest: 'SpRuntimeDigest'
  },
  /**
   * Lookup261: pallet_grandpa_finality_verifier::bridges::header_chain::justification::GrandpaJustification<sp_runtime::generic::header::Header<Number, Hash>>
   **/
  PalletGrandpaFinalityVerifierBridgesHeaderChainJustificationGrandpaJustification: {
    round: 'u64',
    commit: 'FinalityGrandpaCommit',
    votesAncestries: 'Vec<SpRuntimeHeader>'
  },
  /**
   * Lookup262: finality_grandpa::Commit<primitive_types::H256, N, sp_consensus_grandpa::app::Signature, sp_consensus_grandpa::app::Public>
   **/
  FinalityGrandpaCommit: {
    targetHash: 'H256',
    targetNumber: 'u32',
    precommits: 'Vec<FinalityGrandpaSignedPrecommit>'
  },
  /**
   * Lookup264: finality_grandpa::SignedPrecommit<primitive_types::H256, N, sp_consensus_grandpa::app::Signature, sp_consensus_grandpa::app::Public>
   **/
  FinalityGrandpaSignedPrecommit: {
    precommit: 'FinalityGrandpaPrecommit',
    signature: 'SpConsensusGrandpaAppSignature',
    id: 'SpConsensusGrandpaAppPublic'
  },
  /**
   * Lookup267: pallet_eth2_finality_verifier::pallet::Call<T>
   **/
  PalletEth2FinalityVerifierCall: {
    _enum: {
      submit_epoch_debug: {
        attestedBeaconHeader: 'PalletEth2FinalityVerifierBeaconBlockHeader',
        signature: '[u8;96]',
        signerBits: 'Vec<bool>',
        justifiedProof: 'PalletEth2FinalityVerifierMerkleProof',
        executionPayload: 'PalletEth2FinalityVerifierExecutionPayload',
        payloadProof: 'PalletEth2FinalityVerifierMerkleProof',
        executionRange: 'Vec<PalletEth2FinalityVerifierExecutionHeader>',
      },
      submit_epoch: {
        encodedUpdate: 'Bytes',
      },
      submit_epoch_skipped_slot_debug: {
        beaconHeaders: 'Vec<PalletEth2FinalityVerifierBeaconBlockHeader>',
        signature: '[u8;96]',
        signerBits: 'Vec<bool>',
        justifiedProof: 'PalletEth2FinalityVerifierMerkleProof',
        executionPayload: 'PalletEth2FinalityVerifierExecutionPayload',
        payloadProof: 'PalletEth2FinalityVerifierMerkleProof',
        executionRange: 'Vec<PalletEth2FinalityVerifierExecutionHeader>',
      },
      submit_epoch_skipped_slot: {
        encodedUpdate: 'Bytes',
      },
      submit_fork: {
        encodedNewUpdate: 'Bytes',
        encodedOldUpdate: 'Bytes',
      },
      add_next_sync_committee: {
        nextSyncCommittee: 'PalletEth2FinalityVerifierSyncCommittee',
        proof: 'PalletEth2FinalityVerifierMerkleProof',
        proofSlot: 'u64',
      },
      verify_receipt_inclusion: {
        proof: 'PalletEth2FinalityVerifierEthereumReceiptInclusionProof',
        speedMode: 'T3rnPrimitivesSpeedMode',
      },
      verify_event_inclusion: {
        proof: 'PalletEth2FinalityVerifierEthereumEventInclusionProof',
        speedMode: 'T3rnPrimitivesSpeedMode',
        sourceAddress: 'Option<H160>',
      },
      reset: 'Null',
      set_owner: {
        owner: 'AccountId32'
      }
    }
  },
  /**
   * Lookup268: pallet_eth2_finality_verifier::types::BeaconBlockHeader
   **/
  PalletEth2FinalityVerifierBeaconBlockHeader: {
    slot: 'u64',
    proposerIndex: 'u64',
    parentRoot: '[u8;32]',
    stateRoot: '[u8;32]',
    bodyRoot: '[u8;32]'
  },
  /**
   * Lookup271: pallet_eth2_finality_verifier::types::MerkleProof
   **/
  PalletEth2FinalityVerifierMerkleProof: {
    gIndex: 'u64',
    witness: 'Vec<[u8;32]>'
  },
  /**
   * Lookup273: pallet_eth2_finality_verifier::types::ExecutionPayload
   **/
  PalletEth2FinalityVerifierExecutionPayload: {
    parentHash: '[u8;32]',
    feeRecipient: '[u8;20]',
    stateRoot: '[u8;32]',
    receiptsRoot: '[u8;32]',
    logsBloom: 'EthbloomBloom',
    prevRandao: '[u8;32]',
    blockNumber: 'u64',
    gasLimit: 'u64',
    gasUsed: 'u64',
    timestamp: 'u64',
    extraData: 'Bytes',
    baseFeePerGas: 'U256',
    blockHash: '[u8;32]',
    transactionsRoot: '[u8;32]',
    withdrawalsRoot: '[u8;32]'
  },
  /**
   * Lookup274: ethbloom::Bloom
   **/
  EthbloomBloom: '[u8;256]',
  /**
   * Lookup277: pallet_eth2_finality_verifier::types::ExecutionHeader
   **/
  PalletEth2FinalityVerifierExecutionHeader: {
    parentHash: '[u8;32]',
    ommersHash: '[u8;32]',
    beneficiary: 'H160',
    stateRoot: '[u8;32]',
    transactionsRoot: '[u8;32]',
    receiptsRoot: '[u8;32]',
    logsBloom: 'EthbloomBloom',
    difficulty: 'U256',
    number: 'u64',
    gasLimit: 'u64',
    gasUsed: 'u64',
    timestamp: 'u64',
    extraData: 'Bytes',
    mixHash: '[u8;32]',
    nonce: 'u64',
    baseFeePerGas: 'u64',
    withdrawalsRoot: '[u8;32]'
  },
  /**
   * Lookup279: pallet_eth2_finality_verifier::types::SyncCommittee
   **/
  PalletEth2FinalityVerifierSyncCommittee: {
    pubs: 'Vec<[u8;48]>',
    aggr: '[u8;48]'
  },
  /**
   * Lookup282: pallet_eth2_finality_verifier::types::EthereumReceiptInclusionProof
   **/
  PalletEth2FinalityVerifierEthereumReceiptInclusionProof: {
    blockNumber: 'u64',
    witness: 'Vec<Bytes>',
    index: 'Bytes'
  },
  /**
   * Lookup283: pallet_eth2_finality_verifier::types::EthereumEventInclusionProof
   **/
  PalletEth2FinalityVerifierEthereumEventInclusionProof: {
    blockNumber: 'u64',
    witness: 'Vec<Bytes>',
    index: 'Bytes',
    event: 'Bytes'
  },
  /**
   * Lookup285: pallet_sepolia_finality_verifier::pallet::Call<T>
   **/
  PalletSepoliaFinalityVerifierCall: {
    _enum: {
      submit_epoch_decoded: {
        attestedBeaconHeader: 'PalletSepoliaFinalityVerifierBeaconBlockHeader',
        signature: '[u8;96]',
        signerBits: 'Vec<bool>',
        justifiedProof: 'PalletSepoliaFinalityVerifierMerkleProof',
        executionPayload: 'PalletSepoliaFinalityVerifierExecutionPayload',
        payloadProof: 'PalletSepoliaFinalityVerifierMerkleProof',
        executionRange: 'Vec<PalletSepoliaFinalityVerifierExecutionHeader>',
      },
      submit_epoch: {
        encodedUpdate: 'Bytes',
      },
      submit_epoch_skipped_slot_decoded: {
        beaconHeaders: 'Vec<PalletSepoliaFinalityVerifierBeaconBlockHeader>',
        signature: '[u8;96]',
        signerBits: 'Vec<bool>',
        justifiedProof: 'PalletSepoliaFinalityVerifierMerkleProof',
        executionPayload: 'PalletSepoliaFinalityVerifierExecutionPayload',
        payloadProof: 'PalletSepoliaFinalityVerifierMerkleProof',
        executionRange: 'Vec<PalletSepoliaFinalityVerifierExecutionHeader>',
      },
      submit_epoch_skipped_slot: {
        encodedUpdate: 'Bytes',
      },
      submit_unsigned_epoch_decoded: {
        updates: 'Vec<PalletSepoliaFinalityVerifierEpochUpdate>',
      },
      submit_unsigned_epoch_skipped_slot_decoded: {
        updates: 'Vec<PalletSepoliaFinalityVerifierEpochUpdateSkippedSlot>',
      },
      submit_fork: {
        encodedNewUpdate: 'Bytes',
        encodedOldUpdate: 'Bytes',
      },
      add_next_sync_committee: {
        nextSyncCommittee: 'PalletSepoliaFinalityVerifierSyncCommittee',
        proof: 'PalletSepoliaFinalityVerifierMerkleProof',
        proofSlot: 'u64',
      },
      verify_receipt_inclusion: {
        proof: 'PalletSepoliaFinalityVerifierEthereumReceiptInclusionProof',
        speedMode: 'T3rnPrimitivesSpeedMode',
      },
      verify_event_inclusion: {
        proof: 'PalletSepoliaFinalityVerifierEthereumEventInclusionProof',
        speedMode: 'T3rnPrimitivesSpeedMode',
        sourceAddress: 'Option<H160>',
      },
      reset: 'Null',
      set_owner: {
        owner: 'AccountId32'
      }
    }
  },
  /**
   * Lookup286: pallet_sepolia_finality_verifier::types::BeaconBlockHeader
   **/
  PalletSepoliaFinalityVerifierBeaconBlockHeader: {
    slot: 'u64',
    proposerIndex: 'u64',
    parentRoot: '[u8;32]',
    stateRoot: '[u8;32]',
    bodyRoot: '[u8;32]'
  },
  /**
   * Lookup287: pallet_sepolia_finality_verifier::types::MerkleProof
   **/
  PalletSepoliaFinalityVerifierMerkleProof: {
    gIndex: 'u64',
    witness: 'Vec<[u8;32]>'
  },
  /**
   * Lookup288: pallet_sepolia_finality_verifier::types::ExecutionPayload
   **/
  PalletSepoliaFinalityVerifierExecutionPayload: {
    parentHash: '[u8;32]',
    feeRecipient: '[u8;20]',
    stateRoot: '[u8;32]',
    receiptsRoot: '[u8;32]',
    logsBloom: 'EthbloomBloom',
    prevRandao: '[u8;32]',
    blockNumber: 'u64',
    gasLimit: 'u64',
    gasUsed: 'u64',
    timestamp: 'u64',
    extraData: 'Bytes',
    baseFeePerGas: 'U256',
    blockHash: '[u8;32]',
    transactionsRoot: '[u8;32]',
    withdrawalsRoot: '[u8;32]'
  },
  /**
   * Lookup290: pallet_sepolia_finality_verifier::types::ExecutionHeader
   **/
  PalletSepoliaFinalityVerifierExecutionHeader: {
    parentHash: '[u8;32]',
    ommersHash: '[u8;32]',
    beneficiary: 'H160',
    stateRoot: '[u8;32]',
    transactionsRoot: '[u8;32]',
    receiptsRoot: '[u8;32]',
    logsBloom: 'EthbloomBloom',
    difficulty: 'U256',
    number: 'u64',
    gasLimit: 'u64',
    gasUsed: 'u64',
    timestamp: 'u64',
    extraData: 'Bytes',
    mixHash: '[u8;32]',
    nonce: 'u64',
    baseFeePerGas: 'u64',
    withdrawalsRoot: '[u8;32]'
  },
  /**
   * Lookup293: pallet_sepolia_finality_verifier::types::EpochUpdate
   **/
  PalletSepoliaFinalityVerifierEpochUpdate: {
    attestedBeaconHeader: 'PalletSepoliaFinalityVerifierBeaconBlockHeader',
    signature: '[u8;96]',
    signerBits: 'Vec<bool>',
    justifiedProof: 'PalletSepoliaFinalityVerifierMerkleProof',
    executionPayload: 'PalletSepoliaFinalityVerifierExecutionPayload',
    payloadProof: 'PalletSepoliaFinalityVerifierMerkleProof',
    executionRange: 'Vec<PalletSepoliaFinalityVerifierExecutionHeader>'
  },
  /**
   * Lookup295: pallet_sepolia_finality_verifier::types::EpochUpdateSkippedSlot
   **/
  PalletSepoliaFinalityVerifierEpochUpdateSkippedSlot: {
    attestedBeaconHeader: 'PalletSepoliaFinalityVerifierBeaconBlockHeader',
    checkpointBeaconHeader: 'PalletSepoliaFinalityVerifierBeaconBlockHeader',
    signature: '[u8;96]',
    signerBits: 'Vec<bool>',
    justifiedProof: 'PalletSepoliaFinalityVerifierMerkleProof',
    executionPayload: 'PalletSepoliaFinalityVerifierExecutionPayload',
    payloadProof: 'PalletSepoliaFinalityVerifierMerkleProof',
    executionRange: 'Vec<PalletSepoliaFinalityVerifierExecutionHeader>'
  },
  /**
   * Lookup296: pallet_sepolia_finality_verifier::types::SyncCommittee
   **/
  PalletSepoliaFinalityVerifierSyncCommittee: {
    pubs: 'Vec<[u8;48]>',
    aggr: '[u8;48]'
  },
  /**
   * Lookup297: pallet_sepolia_finality_verifier::types::EthereumReceiptInclusionProof
   **/
  PalletSepoliaFinalityVerifierEthereumReceiptInclusionProof: {
    blockNumber: 'u64',
    witness: 'Vec<Bytes>',
    index: 'Bytes'
  },
  /**
   * Lookup298: pallet_sepolia_finality_verifier::types::EthereumEventInclusionProof
   **/
  PalletSepoliaFinalityVerifierEthereumEventInclusionProof: {
    blockNumber: 'u64',
    witness: 'Vec<Bytes>',
    index: 'Bytes',
    event: 'Bytes'
  },
  /**
   * Lookup299: pallet_identity::pallet::Call<T>
   **/
  PalletIdentityCall: {
    _enum: {
      add_registrar: {
        account: 'MultiAddress',
      },
      set_identity: {
        info: 'PalletIdentityIdentityInfo',
      },
      set_subs: {
        subs: 'Vec<(AccountId32,Data)>',
      },
      clear_identity: 'Null',
      request_judgement: {
        regIndex: 'Compact<u32>',
        maxFee: 'Compact<u128>',
      },
      cancel_request: {
        regIndex: 'u32',
      },
      set_fee: {
        index: 'Compact<u32>',
        fee: 'Compact<u128>',
      },
      set_account_id: {
        _alias: {
          new_: 'new',
        },
        index: 'Compact<u32>',
        new_: 'MultiAddress',
      },
      set_fields: {
        index: 'Compact<u32>',
        fields: 'PalletIdentityBitFlags',
      },
      provide_judgement: {
        regIndex: 'Compact<u32>',
        target: 'MultiAddress',
        judgement: 'PalletIdentityJudgement',
        identity: 'H256',
      },
      kill_identity: {
        target: 'MultiAddress',
      },
      add_sub: {
        sub: 'MultiAddress',
        data: 'Data',
      },
      rename_sub: {
        sub: 'MultiAddress',
        data: 'Data',
      },
      remove_sub: {
        sub: 'MultiAddress',
      },
      quit_sub: 'Null'
    }
  },
  /**
   * Lookup300: pallet_identity::types::IdentityInfo<FieldLimit>
   **/
  PalletIdentityIdentityInfo: {
    additional: 'Vec<(Data,Data)>',
    display: 'Data',
    legal: 'Data',
    web: 'Data',
    riot: 'Data',
    email: 'Data',
    pgpFingerprint: 'Option<[u8;20]>',
    image: 'Data',
    twitter: 'Data'
  },
  /**
   * Lookup336: pallet_identity::types::BitFlags<pallet_identity::types::IdentityField>
   **/
  PalletIdentityBitFlags: {
    _bitLength: 64,
    Display: 1,
    Legal: 2,
    Web: 4,
    Riot: 8,
    Email: 16,
    PgpFingerprint: 32,
    Image: 64,
    Twitter: 128
  },
  /**
   * Lookup337: pallet_identity::types::IdentityField
   **/
  PalletIdentityIdentityField: {
    _enum: ['__Unused0', 'Display', 'Legal', '__Unused3', 'Web', '__Unused5', '__Unused6', '__Unused7', 'Riot', '__Unused9', '__Unused10', '__Unused11', '__Unused12', '__Unused13', '__Unused14', '__Unused15', 'Email', '__Unused17', '__Unused18', '__Unused19', '__Unused20', '__Unused21', '__Unused22', '__Unused23', '__Unused24', '__Unused25', '__Unused26', '__Unused27', '__Unused28', '__Unused29', '__Unused30', '__Unused31', 'PgpFingerprint', '__Unused33', '__Unused34', '__Unused35', '__Unused36', '__Unused37', '__Unused38', '__Unused39', '__Unused40', '__Unused41', '__Unused42', '__Unused43', '__Unused44', '__Unused45', '__Unused46', '__Unused47', '__Unused48', '__Unused49', '__Unused50', '__Unused51', '__Unused52', '__Unused53', '__Unused54', '__Unused55', '__Unused56', '__Unused57', '__Unused58', '__Unused59', '__Unused60', '__Unused61', '__Unused62', '__Unused63', 'Image', '__Unused65', '__Unused66', '__Unused67', '__Unused68', '__Unused69', '__Unused70', '__Unused71', '__Unused72', '__Unused73', '__Unused74', '__Unused75', '__Unused76', '__Unused77', '__Unused78', '__Unused79', '__Unused80', '__Unused81', '__Unused82', '__Unused83', '__Unused84', '__Unused85', '__Unused86', '__Unused87', '__Unused88', '__Unused89', '__Unused90', '__Unused91', '__Unused92', '__Unused93', '__Unused94', '__Unused95', '__Unused96', '__Unused97', '__Unused98', '__Unused99', '__Unused100', '__Unused101', '__Unused102', '__Unused103', '__Unused104', '__Unused105', '__Unused106', '__Unused107', '__Unused108', '__Unused109', '__Unused110', '__Unused111', '__Unused112', '__Unused113', '__Unused114', '__Unused115', '__Unused116', '__Unused117', '__Unused118', '__Unused119', '__Unused120', '__Unused121', '__Unused122', '__Unused123', '__Unused124', '__Unused125', '__Unused126', '__Unused127', 'Twitter']
  },
  /**
   * Lookup338: pallet_identity::types::Judgement<Balance>
   **/
  PalletIdentityJudgement: {
    _enum: {
      Unknown: 'Null',
      FeePaid: 'u128',
      Reasonable: 'Null',
      KnownGood: 'Null',
      OutOfDate: 'Null',
      LowQuality: 'Null',
      Erroneous: 'Null'
    }
  },
  /**
   * Lookup339: pallet_maintenance_mode::pallet::Call<T>
   **/
  PalletMaintenanceModeCall: {
    _enum: ['enter_maintenance_mode', 'resume_normal_operation']
  },
  /**
   * Lookup340: pallet_sudo::pallet::Call<T>
   **/
  PalletSudoCall: {
    _enum: {
      sudo: {
        call: 'Call',
      },
      sudo_unchecked_weight: {
        call: 'Call',
        weight: 'SpWeightsWeightV2Weight',
      },
      set_key: {
        _alias: {
          new_: 'new',
        },
        new_: 'MultiAddress',
      },
      sudo_as: {
        who: 'MultiAddress',
        call: 'Call'
      }
    }
  },
  /**
   * Lookup341: circuit_standalone_runtime::OriginCaller
   **/
  CircuitStandaloneRuntimeOriginCaller: {
    _enum: {
      system: 'FrameSupportDispatchRawOrigin',
      Void: 'SpCoreVoid'
    }
  },
  /**
   * Lookup342: frame_support::dispatch::RawOrigin<sp_core::crypto::AccountId32>
   **/
  FrameSupportDispatchRawOrigin: {
    _enum: {
      Root: 'Null',
      Signed: 'AccountId32',
      None: 'Null'
    }
  },
  /**
   * Lookup343: pallet_utility::pallet::Error<T>
   **/
  PalletUtilityError: {
    _enum: ['TooManyCalls']
  },
  /**
   * Lookup345: pallet_balances::types::BalanceLock<Balance>
   **/
  PalletBalancesBalanceLock: {
    id: '[u8;8]',
    amount: 'u128',
    reasons: 'PalletBalancesReasons'
  },
  /**
   * Lookup346: pallet_balances::types::Reasons
   **/
  PalletBalancesReasons: {
    _enum: ['Fee', 'Misc', 'All']
  },
  /**
   * Lookup349: pallet_balances::types::ReserveData<ReserveIdentifier, Balance>
   **/
  PalletBalancesReserveData: {
    id: '[u8;8]',
    amount: 'u128'
  },
  /**
   * Lookup353: circuit_standalone_runtime::RuntimeHoldReason
   **/
  CircuitStandaloneRuntimeRuntimeHoldReason: 'Null',
  /**
   * Lookup356: pallet_balances::types::IdAmount<Id, Balance>
   **/
  PalletBalancesIdAmount: {
    id: 'Null',
    amount: 'u128'
  },
  /**
   * Lookup358: pallet_balances::pallet::Error<T, I>
   **/
  PalletBalancesError: {
    _enum: ['VestingBalance', 'LiquidityRestrictions', 'InsufficientBalance', 'ExistentialDeposit', 'Expendability', 'ExistingVestingSchedule', 'DeadAccount', 'TooManyReserves', 'TooManyHolds', 'TooManyFreezes']
  },
  /**
   * Lookup360: pallet_transaction_payment::Releases
   **/
  PalletTransactionPaymentReleases: {
    _enum: ['V1Ancient', 'V2']
  },
  /**
   * Lookup361: pallet_assets::types::AssetDetails<Balance, sp_core::crypto::AccountId32, DepositBalance>
   **/
  PalletAssetsAssetDetails: {
    owner: 'AccountId32',
    issuer: 'AccountId32',
    admin: 'AccountId32',
    freezer: 'AccountId32',
    supply: 'u128',
    deposit: 'u128',
    minBalance: 'u128',
    isSufficient: 'bool',
    accounts: 'u32',
    sufficients: 'u32',
    approvals: 'u32',
    status: 'PalletAssetsAssetStatus'
  },
  /**
   * Lookup362: pallet_assets::types::AssetStatus
   **/
  PalletAssetsAssetStatus: {
    _enum: ['Live', 'Frozen', 'Destroying']
  },
  /**
   * Lookup364: pallet_assets::types::AssetAccount<Balance, DepositBalance, Extra, sp_core::crypto::AccountId32>
   **/
  PalletAssetsAssetAccount: {
    balance: 'u128',
    status: 'PalletAssetsAccountStatus',
    reason: 'PalletAssetsExistenceReason',
    extra: 'Null'
  },
  /**
   * Lookup365: pallet_assets::types::AccountStatus
   **/
  PalletAssetsAccountStatus: {
    _enum: ['Liquid', 'Frozen', 'Blocked']
  },
  /**
   * Lookup366: pallet_assets::types::ExistenceReason<Balance, sp_core::crypto::AccountId32>
   **/
  PalletAssetsExistenceReason: {
    _enum: {
      Consumer: 'Null',
      Sufficient: 'Null',
      DepositHeld: 'u128',
      DepositRefunded: 'Null',
      DepositFrom: '(AccountId32,u128)'
    }
  },
  /**
   * Lookup368: pallet_assets::types::Approval<Balance, DepositBalance>
   **/
  PalletAssetsApproval: {
    amount: 'u128',
    deposit: 'u128'
  },
  /**
   * Lookup369: pallet_assets::types::AssetMetadata<DepositBalance, bounded_collections::bounded_vec::BoundedVec<T, S>>
   **/
  PalletAssetsAssetMetadata: {
    deposit: 'u128',
    name: 'Bytes',
    symbol: 'Bytes',
    decimals: 'u8',
    isFrozen: 'bool'
  },
  /**
   * Lookup371: pallet_assets::pallet::Error<T, I>
   **/
  PalletAssetsError: {
    _enum: ['BalanceLow', 'NoAccount', 'NoPermission', 'Unknown', 'Frozen', 'InUse', 'BadWitness', 'MinBalanceZero', 'UnavailableConsumer', 'BadMetadata', 'Unapproved', 'WouldDie', 'AlreadyExists', 'NoDeposit', 'WouldBurn', 'LiveAsset', 'AssetNotLive', 'IncorrectStatus', 'NotFrozen', 'CallbackFailed']
  },
  /**
   * Lookup372: t3rn_primitives::account_manager::RequestCharge<sp_core::crypto::AccountId32, Balance, AssetId>
   **/
  T3rnPrimitivesAccountManagerRequestCharge: {
    payee: 'AccountId32',
    offeredReward: 'u128',
    maybeAssetId: 'Option<u32>',
    chargeFee: 'u128',
    recipient: 'Option<AccountId32>',
    source: 'T3rnPrimitivesClaimableBenefitSource',
    role: 'T3rnPrimitivesClaimableCircuitRole'
  },
  /**
   * Lookup374: t3rn_primitives::common::RoundInfo<BlockNumber>
   **/
  T3rnPrimitivesCommonRoundInfo: {
    index: 'u32',
    head: 'u32',
    term: 'u32'
  },
  /**
   * Lookup375: t3rn_primitives::account_manager::Settlement<sp_core::crypto::AccountId32, Balance, AssetId>
   **/
  T3rnPrimitivesAccountManagerSettlement: {
    requester: 'AccountId32',
    recipient: 'AccountId32',
    settlementAmount: 'u128',
    maybeAssetId: 'Option<u32>',
    outcome: 'T3rnPrimitivesAccountManagerOutcome',
    source: 'T3rnPrimitivesClaimableBenefitSource',
    role: 'T3rnPrimitivesClaimableCircuitRole'
  },
  /**
   * Lookup376: pallet_account_manager::pallet::Error<T>
   **/
  PalletAccountManagerError: {
    _enum: ['PendingChargeNotFoundAtCommit', 'PendingChargeNotFoundAtRefund', 'ExecutionNotRegistered', 'ExecutionAlreadyRegistered', 'SkippingEmptyCharges', 'NoChargeOfGivenIdRegistered', 'ChargeAlreadyRegistered', 'ChargeOrSettlementCalculationOverflow', 'ChargeOrSettlementActualFeesOutgrowReserved', 'DecodingExecutionIDFailed', 'TransferDepositFailedOldChargeNotFound', 'TransferDepositFailedToReleasePreviousCharge']
  },
  /**
   * Lookup377: pallet_treasury::Proposal<sp_core::crypto::AccountId32, Balance>
   **/
  PalletTreasuryProposal: {
    proposer: 'AccountId32',
    value: 'u128',
    beneficiary: 'AccountId32',
    bond: 'u128'
  },
  /**
   * Lookup380: frame_support::PalletId
   **/
  FrameSupportPalletId: '[u8;8]',
  /**
   * Lookup381: pallet_treasury::pallet::Error<T, I>
   **/
  PalletTreasuryError: {
    _enum: ['InsufficientProposersBalance', 'InvalidIndex', 'TooManyApprovals', 'InsufficientPermission', 'ProposalNotApproved']
  },
  /**
   * Lookup386: pallet_clock::pallet::Error<T>
   **/
  PalletClockError: 'Null',
  /**
   * Lookup392: t3rn_primitives::GatewayActivity<BlockNumber>
   **/
  T3rnPrimitivesGatewayActivity: {
    gatewayId: '[u8;4]',
    reportedAt: 'u32',
    justifiedHeight: 'u32',
    finalizedHeight: 'u32',
    updatedHeight: 'u32',
    attestationLatency: 'Option<T3rnPrimitivesAttestersLatencyStatus>',
    securityLvl: 'T3rnTypesSfxSecurityLvl',
    isActive: 'bool'
  },
  /**
   * Lookup395: t3rn_primitives::FinalityVerifierActivity<BlockNumber>
   **/
  T3rnPrimitivesFinalityVerifierActivity: {
    verifier: 'T3rnPrimitivesGatewayVendor',
    reportedAt: 'u32',
    justifiedHeight: 'u32',
    finalizedHeight: 'u32',
    updatedHeight: 'u32',
    epoch: 'u32',
    isActive: 'bool'
  },
  /**
   * Lookup397: t3rn_primitives::xdns::EpochEstimate<BlockNumber>
   **/
  T3rnPrimitivesXdnsEpochEstimate: {
    local: 'u32',
    remote: 'u32',
    movingAverageLocal: 'u32',
    movingAverageRemote: 'u32'
  },
  /**
   * Lookup398: pallet_xdns::pallet::Error<T>
   **/
  PalletXdnsError: {
    _enum: ['GatewayRecordAlreadyExists', 'XdnsRecordNotFound', 'EscrowAccountNotFound', 'RemoteOrderAddressNotFound', 'TokenRecordAlreadyExists', 'TokenRecordNotFoundInAssetsOverlay', 'TokenRecordNotFoundInGateway', 'GatewayRecordNotFound', 'SideEffectABIAlreadyExists', 'SideEffectABINotFound', 'NoParachainInfoFound', 'TokenExecutionVendorMismatch', 'GatewayNotActive', 'TopologyDecodeError', 'EmptyTopologySubmitted']
  },
  /**
   * Lookup399: t3rn_primitives::attesters::AttesterInfo
   **/
  T3rnPrimitivesAttestersAttesterInfo: {
    keyEd: '[u8;32]',
    keyEc: '[u8;33]',
    keySr: '[u8;32]',
    commission: 'Percent',
    index: 'u32'
  },
  /**
   * Lookup401: pallet_attesters::pallet::InfluxMessage<BlockNumber>
   **/
  PalletAttestersInfluxMessage: {
    messageHash: 'H256',
    message: 'H256',
    heightThere: 'u32',
    gateway: '[u8;4]',
    signatures: 'Vec<Bytes>',
    created: 'u32',
    status: 'PalletAttestersBatchStatus'
  },
  /**
   * Lookup411: pallet_attesters::pallet::Error<T>
   **/
  PalletAttestersError: {
    _enum: ['AttesterNotFound', 'ArithmeticOverflow', 'InvalidSignature', 'InvalidMessage', 'InvalidTargetInclusionProof', 'UnexpectedBatchHashRecoveredFromCommitment', 'AlreadyRegistered', 'PublicKeyMissing', 'AttestationSignatureInvalid', 'AttestationDoubleSignAttempt', 'NotActiveSet', 'NotInCurrentCommittee', 'AttesterDidNotAgreeToNewTarget', 'NotRegistered', 'NoNominationFound', 'AlreadyNominated', 'NominatorNotEnoughBalance', 'NominatorBondTooSmall', 'AttesterBondTooSmall', 'MissingNominations', 'BatchHashMismatch', 'BatchNotFound', 'CollusionWithPermanentSlashDetected', 'BatchFoundWithUnsignableStatus', 'RejectingFromSlashedAttester', 'TargetAlreadyActive', 'TargetNotActive', 'XdnsTargetNotActive', 'XdnsGatewayDoesNotHaveEscrowAddressRegistered', 'SfxAlreadyRequested', 'AddAttesterAlreadyRequested', 'RemoveAttesterAlreadyRequested', 'NextCommitteeAlreadyRequested', 'BanAttesterAlreadyRequested', 'BatchAlreadyCommitted', 'CommitteeSizeTooLarge', 'InfluxSignatureAlreadySubmitted', 'InfluxMessageHashIncorrect']
  },
  /**
   * Lookup416: pallet_rewards::pallet::AssetType<AssetId>
   **/
  PalletRewardsAssetType: {
    _enum: {
      Native: 'Null',
      NonNative: 'u32'
    }
  },
  /**
   * Lookup417: pallet_rewards::pallet::TreasuryBalanceSheet<Balance>
   **/
  PalletRewardsTreasuryBalanceSheet: {
    treasury: 'u128',
    escrow: 'u128',
    fee: 'u128',
    slash: 'u128',
    parachain: 'u128'
  },
  /**
   * Lookup419: pallet_rewards::pallet::DistributionRecord<BlockNumber, Balance>
   **/
  PalletRewardsDistributionRecord: {
    blockNumber: 'u32',
    attesterRewards: 'u128',
    collatorRewards: 'u128',
    executorRewards: 'u128',
    treasuryRewards: 'u128',
    available: 'u128',
    distributed: 'u128'
  },
  /**
   * Lookup421: t3rn_primitives::claimable::ClaimableArtifacts<sp_core::crypto::AccountId32, Balance>
   **/
  T3rnPrimitivesClaimableClaimableArtifacts: {
    beneficiary: 'AccountId32',
    role: 'T3rnPrimitivesClaimableCircuitRole',
    totalRoundClaim: 'u128',
    nonNativeAssetId: 'Option<u32>',
    benefitSource: 'T3rnPrimitivesClaimableBenefitSource'
  },
  /**
   * Lookup423: pallet_rewards::pallet::Error<T>
   **/
  PalletRewardsError: {
    _enum: ['DistributionPeriodNotElapsed', 'NoPendingClaims', 'ArithmeticOverflow', 'AttesterNotFound', 'TryIntoConversionU128ToBalanceFailed', 'Halted']
  },
  /**
   * Lookup424: pallet_contracts_registry::pallet::Error<T>
   **/
  PalletContractsRegistryError: {
    _enum: ['ContractAlreadyExists', 'UnknownContract']
  },
  /**
   * Lookup425: t3rn_primitives::circuit::types::XExecSignal<sp_core::crypto::AccountId32, BlockNumber>
   **/
  T3rnPrimitivesCircuitTypesXExecSignal: {
    requester: 'AccountId32',
    requesterNonce: 'u32',
    timeoutsAt: 'T3rnPrimitivesCircuitTypesAdaptiveTimeout',
    speedMode: 'T3rnPrimitivesSpeedMode',
    delayStepsAt: 'Option<Vec<u32>>',
    status: 'T3rnPrimitivesCircuitTypesCircuitStatus',
    stepsCnt: '(u32,u32)'
  },
  /**
   * Lookup427: t3rn_primitives::volatile::LocalState
   **/
  T3rnPrimitivesVolatileLocalState: {
    state: 'BTreeMap<[u8;32], Bytes>'
  },
  /**
   * Lookup434: t3rn_sdk_primitives::signal::ExecutionSignal<primitive_types::H256>
   **/
  T3rnSdkPrimitivesSignalExecutionSignal: {
    step: 'u32',
    kind: 'T3rnSdkPrimitivesSignalSignalKind',
    executionId: 'H256'
  },
  /**
   * Lookup436: pallet_circuit::pallet::Error<T>
   **/
  PalletCircuitError: {
    _enum: ['UpdateAttemptDoubleRevert', 'UpdateAttemptDoubleKill', 'UpdateStateTransitionDisallowed', 'UpdateForcedStateTransitionDisallowed', 'UpdateXtxTriggeredWithUnexpectedStatus', 'ConfirmationFailed', 'InvalidOrderOrigin', 'ApplyTriggeredWithUnexpectedStatus', 'BidderNotEnoughBalance', 'RequesterNotEnoughBalance', 'AssetsFailedToWithdraw', 'SanityAfterCreatingSFXDepositsFailed', 'ContractXtxKilledRunOutOfFunds', 'ChargingTransferFailed', 'ChargingTransferFailedAtPendingExecution', 'XtxChargeFailedRequesterBalanceTooLow', 'XtxChargeBondDepositFailedCantAccessBid', 'FinalizeSquareUpFailed', 'CriticalStateSquareUpCalledToFinishWithoutFsxConfirmed', 'RewardTransferFailed', 'RefundTransferFailed', 'SideEffectsValidationFailed', 'InsuranceBondNotRequired', 'BiddingInactive', 'BiddingRejectedBidBelowDust', 'BiddingRejectedBidTooHigh', 'BiddingRejectedInsuranceTooLow', 'BiddingRejectedBetterBidFound', 'BiddingRejectedFailedToDepositBidderBond', 'BiddingFailedExecutorsBalanceTooLowToReserve', 'InsuranceBondAlreadyDeposited', 'InvalidFTXStateEmptyBidForReadyXtx', 'InvalidFTXStateEmptyConfirmationForFinishedXtx', 'InvalidFTXStateUnassignedExecutorForReadySFX', 'InvalidFTXStateIncorrectExecutorForReadySFX', 'GatewayNotActive', 'SetupFailed', 'SetupFailedXtxNotFound', 'SetupFailedXtxStorageArtifactsNotFound', 'SetupFailedIncorrectXtxStatus', 'SetupFailedDuplicatedXtx', 'SetupFailedEmptyXtx', 'SetupFailedXtxAlreadyFinished', 'SetupFailedXtxWasDroppedAtBidding', 'SetupFailedXtxReverted', 'SetupFailedXtxRevertedTimeout', 'XtxDoesNotExist', 'InvalidFSXBidStateLocated', 'EnactSideEffectsCanOnlyBeCalledWithMin1StepFinished', 'FatalXtxTimeoutXtxIdNotMatched', 'RelayEscrowedFailedNothingToConfirm', 'FatalCommitSideEffectWithoutConfirmationAttempt', 'FatalErroredCommitSideEffectConfirmationAttempt', 'FatalErroredRevertSideEffectConfirmationAttempt', 'FailedToHardenFullSideEffect', 'ApplyFailed', 'DeterminedForbiddenXtxStatus', 'SideEffectIsAlreadyScheduledToExecuteOverXBI', 'FSXNotFoundById', 'XtxNotFound', 'LocalSideEffectExecutionNotApplicable', 'LocalExecutionUnauthorized', 'OnLocalTriggerFailedToSetupXtx', 'UnauthorizedCancellation', 'FailedToConvertSFX2XBI', 'FailedToCheckInOverXBI', 'FailedToCreateXBIMetadataDueToWrongAccountConversion', 'FailedToConvertXBIResult2SFXConfirmation', 'FailedToEnterXBIPortal', 'FailedToExitXBIPortal', 'FailedToCommitFSX', 'XBIExitFailedOnSFXConfirmation', 'UnsupportedRole', 'InvalidLocalTrigger', 'SignalQueueFull', 'ArithmeticErrorOverflow', 'ArithmeticErrorUnderflow', 'ArithmeticErrorDivisionByZero', 'ABIOnSelectedTargetNotFoundForSubmittedSFX', 'ForNowOnlySingleRewardAssetSupportedForMultiSFX', 'TargetAppearsNotToBeActiveAndDoesntHaveFinalizedHeight', 'SideEffectsValidationFailedAgainstABI']
  },
  /**
   * Lookup437: pallet_circuit_vacuum::pallet::Error<T>
   **/
  PalletCircuitVacuumError: 'Null',
  /**
   * Lookup439: pallet_3vm::pallet::Error<T>
   **/
  Pallet3vmError: {
    _enum: ['ExceededSignalBounceThreshold', 'CannotTriggerWithoutSideEffects', 'ContractNotFound', 'InvalidOrigin', 'CannotInstantiateContract', 'ContractCannotRemunerate', 'ContractCannotHaveStorage', 'ContractCannotGenerateSideEffects', 'InvalidPrecompilePointer', 'InvalidPrecompileArgs', 'InvalidArithmeticOverflow', 'DownstreamCircuit']
  },
  /**
   * Lookup441: pallet_contracts::wasm::CodeInfo<T>
   **/
  PalletContractsWasmCodeInfo: {
    owner: 'AccountId32',
    deposit: 'Compact<u128>',
    refcount: 'Compact<u64>',
    determinism: 'PalletContractsWasmDeterminism',
    codeLen: 'u32'
  },
  /**
   * Lookup442: pallet_contracts::storage::ContractInfo<T>
   **/
  PalletContractsStorageContractInfo: {
    trieId: 'Bytes',
    depositAccount: 'AccountId32',
    codeHash: 'H256',
    storageBytes: 'u32',
    storageItems: 'u32',
    storageByteDeposit: 'u128',
    storageItemDeposit: 'u128',
    storageBaseDeposit: 'u128'
  },
  /**
   * Lookup445: pallet_contracts::storage::DeletionQueueManager<T>
   **/
  PalletContractsStorageDeletionQueueManager: {
    insertCounter: 'u32',
    deleteCounter: 'u32'
  },
  /**
   * Lookup447: pallet_contracts::schedule::Schedule<T>
   **/
  PalletContractsSchedule: {
    limits: 'PalletContractsScheduleLimits',
    instructionWeights: 'PalletContractsScheduleInstructionWeights',
    hostFnWeights: 'PalletContractsScheduleHostFnWeights'
  },
  /**
   * Lookup448: pallet_contracts::schedule::Limits
   **/
  PalletContractsScheduleLimits: {
    eventTopics: 'u32',
    globals: 'u32',
    locals: 'u32',
    parameters: 'u32',
    memoryPages: 'u32',
    tableSize: 'u32',
    brTableSize: 'u32',
    subjectLen: 'u32',
    payloadLen: 'u32',
    runtimeMemory: 'u32'
  },
  /**
   * Lookup449: pallet_contracts::schedule::InstructionWeights<T>
   **/
  PalletContractsScheduleInstructionWeights: {
    base: 'u32'
  },
  /**
   * Lookup450: pallet_contracts::schedule::HostFnWeights<T>
   **/
  PalletContractsScheduleHostFnWeights: {
    _alias: {
      r_return: 'r#return'
    },
    caller: 'SpWeightsWeightV2Weight',
    isContract: 'SpWeightsWeightV2Weight',
    codeHash: 'SpWeightsWeightV2Weight',
    ownCodeHash: 'SpWeightsWeightV2Weight',
    callerIsOrigin: 'SpWeightsWeightV2Weight',
    callerIsRoot: 'SpWeightsWeightV2Weight',
    address: 'SpWeightsWeightV2Weight',
    gasLeft: 'SpWeightsWeightV2Weight',
    balance: 'SpWeightsWeightV2Weight',
    valueTransferred: 'SpWeightsWeightV2Weight',
    minimumBalance: 'SpWeightsWeightV2Weight',
    blockNumber: 'SpWeightsWeightV2Weight',
    now: 'SpWeightsWeightV2Weight',
    weightToFee: 'SpWeightsWeightV2Weight',
    input: 'SpWeightsWeightV2Weight',
    inputPerByte: 'SpWeightsWeightV2Weight',
    r_return: 'SpWeightsWeightV2Weight',
    returnPerByte: 'SpWeightsWeightV2Weight',
    terminate: 'SpWeightsWeightV2Weight',
    random: 'SpWeightsWeightV2Weight',
    depositEvent: 'SpWeightsWeightV2Weight',
    depositEventPerTopic: 'SpWeightsWeightV2Weight',
    depositEventPerByte: 'SpWeightsWeightV2Weight',
    debugMessage: 'SpWeightsWeightV2Weight',
    debugMessagePerByte: 'SpWeightsWeightV2Weight',
    setStorage: 'SpWeightsWeightV2Weight',
    setStoragePerNewByte: 'SpWeightsWeightV2Weight',
    setStoragePerOldByte: 'SpWeightsWeightV2Weight',
    setCodeHash: 'SpWeightsWeightV2Weight',
    clearStorage: 'SpWeightsWeightV2Weight',
    clearStoragePerByte: 'SpWeightsWeightV2Weight',
    containsStorage: 'SpWeightsWeightV2Weight',
    containsStoragePerByte: 'SpWeightsWeightV2Weight',
    getStorage: 'SpWeightsWeightV2Weight',
    getStoragePerByte: 'SpWeightsWeightV2Weight',
    takeStorage: 'SpWeightsWeightV2Weight',
    takeStoragePerByte: 'SpWeightsWeightV2Weight',
    transfer: 'SpWeightsWeightV2Weight',
    call: 'SpWeightsWeightV2Weight',
    delegateCall: 'SpWeightsWeightV2Weight',
    callTransferSurcharge: 'SpWeightsWeightV2Weight',
    callPerClonedByte: 'SpWeightsWeightV2Weight',
    instantiate: 'SpWeightsWeightV2Weight',
    instantiateTransferSurcharge: 'SpWeightsWeightV2Weight',
    instantiatePerInputByte: 'SpWeightsWeightV2Weight',
    instantiatePerSaltByte: 'SpWeightsWeightV2Weight',
    hashSha2256: 'SpWeightsWeightV2Weight',
    hashSha2256PerByte: 'SpWeightsWeightV2Weight',
    hashKeccak256: 'SpWeightsWeightV2Weight',
    hashKeccak256PerByte: 'SpWeightsWeightV2Weight',
    hashBlake2256: 'SpWeightsWeightV2Weight',
    hashBlake2256PerByte: 'SpWeightsWeightV2Weight',
    hashBlake2128: 'SpWeightsWeightV2Weight',
    hashBlake2128PerByte: 'SpWeightsWeightV2Weight',
    ecdsaRecover: 'SpWeightsWeightV2Weight',
    ecdsaToEthAddress: 'SpWeightsWeightV2Weight',
    sr25519Verify: 'SpWeightsWeightV2Weight',
    sr25519VerifyPerByte: 'SpWeightsWeightV2Weight',
    reentranceCount: 'SpWeightsWeightV2Weight',
    accountReentranceCount: 'SpWeightsWeightV2Weight',
    instantiationNonce: 'SpWeightsWeightV2Weight'
  },
  /**
   * Lookup451: pallet_contracts::pallet::Error<T>
   **/
  PalletContractsError: {
    _enum: ['InvalidSchedule', 'InvalidCallFlags', 'OutOfGas', 'OutputBufferTooSmall', 'TransferFailed', 'MaxCallDepthReached', 'ContractNotFound', 'CodeTooLarge', 'CodeNotFound', 'CodeInfoNotFound', 'OutOfBounds', 'DecodingFailed', 'ContractTrapped', 'ValueTooLarge', 'TerminatedWhileReentrant', 'InputForwarded', 'RandomSubjectTooLong', 'TooManyTopics', 'NoChainExtension', 'DuplicateContract', 'TerminatedInConstructor', 'ReentranceDenied', 'StorageDepositNotEnoughFunds', 'StorageDepositLimitExhausted', 'CodeInUse', 'ContractReverted', 'CodeRejected', 'Indeterministic', 'MigrationInProgress', 'NoMigrationPerformed']
  },
  /**
   * Lookup452: pallet_evm::CodeMetadata
   **/
  PalletEvmCodeMetadata: {
    _alias: {
      size_: 'size',
      hash_: 'hash'
    },
    size_: 'u64',
    hash_: 'H256'
  },
  /**
   * Lookup454: pallet_evm::pallet::Error<T>
   **/
  PalletEvmError: {
    _enum: ['BalanceLow', 'FeeOverflow', 'PaymentOverflow', 'WithdrawFailed', 'GasPriceTooLow', 'InvalidNonce', 'GasLimitTooLow', 'GasLimitTooHigh', 'Undefined', 'Reentrancy', 'TransactionMustComeFromEOA']
  },
  /**
   * Lookup455: pallet_portal::pallet::Error<T>
   **/
  PalletPortalError: {
    _enum: ['XdnsRecordCreationFailed', 'UnimplementedGatewayVendor', 'LightClientNotFoundByVendor', 'RegistrationError', 'GatewayVendorNotFound', 'SetOwnerError', 'SetOperationalError', 'SubmitHeaderError', 'NoGatewayHeightAvailable', 'SideEffectConfirmationFailed', 'SFXRecodeError']
  },
  /**
   * Lookup456: pallet_grandpa_finality_verifier::bridges::header_chain::AuthoritySet
   **/
  PalletGrandpaFinalityVerifierBridgesHeaderChainAuthoritySet: {
    authorities: 'Vec<(SpConsensusGrandpaAppPublic,u64)>',
    setId: 'u64'
  },
  /**
   * Lookup457: pallet_grandpa_finality_verifier::types::ParachainRegistrationData
   **/
  PalletGrandpaFinalityVerifierParachainRegistrationData: {
    relayGatewayId: '[u8;4]',
    id: 'u32'
  },
  /**
   * Lookup458: pallet_grandpa_finality_verifier::pallet::Error<T, I>
   **/
  PalletGrandpaFinalityVerifierError: {
    _enum: ['EmptyRangeSubmitted', 'RangeToLarge', 'NoFinalizedHeader', 'InvalidAuthoritySet', 'InvalidGrandpaJustification', 'InvalidRangeLinkage', 'InvalidJustificationLinkage', 'ParachainEntryNotFound', 'StorageRootNotFound', 'InclusionDataDecodeError', 'InvalidStorageProof', 'EventNotIncluded', 'HeaderDecodingError', 'HeaderDataDecodingError', 'StorageRootMismatch', 'UnknownHeader', 'UnexpectedEventLength', 'UnexpectedSource', 'EventDecodingFailed', 'UnkownSideEffect', 'UnsupportedScheduledChange', 'Halted', 'BlockHeightConversionError', 'InvalidPayloadSource', 'InvalidSourceFormat']
  },
  /**
   * Lookup461: pallet_eth2_finality_verifier::types::Checkpoint
   **/
  PalletEth2FinalityVerifierCheckpoint: {
    attestedBeacon: 'PalletEth2FinalityVerifierBeaconCheckpoint',
    attestedExecution: 'PalletEth2FinalityVerifierExecutionCheckpoint',
    justifiedBeacon: 'PalletEth2FinalityVerifierBeaconCheckpoint',
    justifiedExecution: 'PalletEth2FinalityVerifierExecutionCheckpoint',
    finalizedBeacon: 'PalletEth2FinalityVerifierBeaconCheckpoint',
    finalizedExecution: 'PalletEth2FinalityVerifierExecutionCheckpoint'
  },
  /**
   * Lookup462: pallet_eth2_finality_verifier::types::BeaconCheckpoint
   **/
  PalletEth2FinalityVerifierBeaconCheckpoint: {
    epoch: 'u64',
    root: '[u8;32]'
  },
  /**
   * Lookup463: pallet_eth2_finality_verifier::types::ExecutionCheckpoint
   **/
  PalletEth2FinalityVerifierExecutionCheckpoint: {
    height: 'u64',
    root: '[u8;32]'
  },
  /**
   * Lookup464: pallet_eth2_finality_verifier::pallet::Error<T>
   **/
  PalletEth2FinalityVerifierError: {
    _enum: ['Halted', 'AlreadyInitialized', 'InvalidInitializationData', 'SSZForkDataHashTreeRootFailed', 'SSZSigningDataHashTreeRootFailed', 'BLSPubkeyAggregationFaild', 'InvalidBLSPublicKeyUsedForVerification', 'InvalidInclusionProof', 'ForkNotDetected', 'ValidSyncCommitteeNotAvailable', 'SubmittedHeaderToOld', 'InvalidBLSSignature', 'InvalidMerkleProof', 'BeaconHeaderHashTreeRootFailed', 'BeaconHeaderNotFound', 'BeaconHeaderNotFinalized', 'ExecutionHeaderHashTreeRootFailed', 'InvalidExecutionRangeLinkage', 'InvalidExecutionRange', 'SyncCommitteeParticipantsNotSupermajority', 'SyncCommitteeInvalid', 'NotPeriodsFirstEpoch', 'InvalidCheckpoint', 'ExecutionHeaderNotFound', 'EventNotInReceipt', 'InvalidEncodedEpochUpdate', 'InvalidSyncCommitteePeriod', 'MathError', 'CurrentSyncCommitteePeriodNotAvailable', 'BeaconCheckpointHashTreeRootFailed', 'InvalidFork', 'ExecutionHeaderNotFinalized', 'InvalidBeaconLinkage', 'InvalidExecutionPayload', 'InvalidSourceAddress', 'InvalidAmountOfBeaconHeaders', 'InvalidEventDecodingToAddressTopicLogsFormat', 'InvalidEventProofDecoding', 'EmptyAccountTrieToVerify']
  },
  /**
   * Lookup465: pallet_sepolia_finality_verifier::types::Checkpoint
   **/
  PalletSepoliaFinalityVerifierCheckpoint: {
    attestedBeacon: 'PalletSepoliaFinalityVerifierBeaconCheckpoint',
    attestedExecution: 'PalletSepoliaFinalityVerifierExecutionCheckpoint',
    justifiedBeacon: 'PalletSepoliaFinalityVerifierBeaconCheckpoint',
    justifiedExecution: 'PalletSepoliaFinalityVerifierExecutionCheckpoint',
    finalizedBeacon: 'PalletSepoliaFinalityVerifierBeaconCheckpoint',
    finalizedExecution: 'PalletSepoliaFinalityVerifierExecutionCheckpoint'
  },
  /**
   * Lookup466: pallet_sepolia_finality_verifier::types::BeaconCheckpoint
   **/
  PalletSepoliaFinalityVerifierBeaconCheckpoint: {
    epoch: 'u64',
    root: '[u8;32]'
  },
  /**
   * Lookup467: pallet_sepolia_finality_verifier::types::ExecutionCheckpoint
   **/
  PalletSepoliaFinalityVerifierExecutionCheckpoint: {
    height: 'u64',
    root: '[u8;32]'
  },
  /**
   * Lookup468: pallet_sepolia_finality_verifier::pallet::Error<T>
   **/
  PalletSepoliaFinalityVerifierError: {
    _enum: ['Halted', 'AlreadyInitialized', 'InvalidInitializationData', 'SSZForkDataHashTreeRootFailed', 'SSZSigningDataHashTreeRootFailed', 'BLSPubkeyAggregationFaild', 'InvalidBLSPublicKeyUsedForVerification', 'InvalidInclusionProof', 'ForkNotDetected', 'ValidSyncCommitteeNotAvailable', 'SubmittedHeaderToOld', 'InvalidBLSSignature', 'InvalidMerkleProof', 'BeaconHeaderHashTreeRootFailed', 'BeaconHeaderNotFound', 'BeaconHeaderNotFinalized', 'ExecutionHeaderHashTreeRootFailed', 'InvalidExecutionRangeLinkage', 'InvalidExecutionRangeLinkageStorageToUnsignedEpoch', 'InvalidExecutionRange', 'SyncCommitteeParticipantsNotSupermajority', 'SyncCommitteeInvalid', 'NotPeriodsFirstEpoch', 'InvalidCheckpoint', 'ExecutionHeaderNotFound', 'EventNotInReceipt', 'InvalidEncodedEpochUpdate', 'InvalidSyncCommitteePeriod', 'MathError', 'CurrentSyncCommitteePeriodNotAvailable', 'BeaconCheckpointHashTreeRootFailed', 'InvalidFork', 'ExecutionHeaderNotFinalized', 'InvalidBeaconLinkage', 'InvalidExecutionPayload', 'InvalidSourceAddress', 'InvalidAmountOfBeaconHeaders', 'InvalidEventDecodingToAddressTopicLogsFormat', 'InvalidEventProofDecoding', 'EmptyAccountTrieToVerify']
  },
  /**
   * Lookup469: pallet_identity::types::Registration<Balance, MaxJudgements, MaxAdditionalFields>
   **/
  PalletIdentityRegistration: {
    judgements: 'Vec<(u32,PalletIdentityJudgement)>',
    deposit: 'u128',
    info: 'PalletIdentityIdentityInfo'
  },
  /**
   * Lookup477: pallet_identity::types::RegistrarInfo<Balance, sp_core::crypto::AccountId32>
   **/
  PalletIdentityRegistrarInfo: {
    account: 'AccountId32',
    fee: 'u128',
    fields: 'PalletIdentityBitFlags'
  },
  /**
   * Lookup479: pallet_identity::pallet::Error<T>
   **/
  PalletIdentityError: {
    _enum: ['TooManySubAccounts', 'NotFound', 'NotNamed', 'EmptyIndex', 'FeeChanged', 'NoIdentity', 'StickyJudgement', 'JudgementGiven', 'InvalidJudgement', 'InvalidIndex', 'InvalidTarget', 'TooManyFields', 'TooManyRegistrars', 'AlreadyClaimed', 'NotSub', 'NotOwned', 'JudgementForDifferentIdentity', 'JudgementPaymentFailed']
  },
  /**
   * Lookup481: pallet_maintenance_mode::pallet::Error<T>
   **/
  PalletMaintenanceModeError: {
    _enum: ['AlreadyInMaintenanceMode', 'NotInMaintenanceMode']
  },
  /**
   * Lookup482: pallet_sudo::pallet::Error<T>
   **/
  PalletSudoError: {
    _enum: ['RequireSudo']
  },
  /**
   * Lookup484: sp_runtime::MultiSignature
   **/
  SpRuntimeMultiSignature: {
    _enum: {
      Ed25519: 'SpCoreEd25519Signature',
      Sr25519: 'SpCoreSr25519Signature',
      Ecdsa: 'SpCoreEcdsaSignature'
    }
  },
  /**
   * Lookup485: sp_core::sr25519::Signature
   **/
  SpCoreSr25519Signature: '[u8;64]',
  /**
   * Lookup486: sp_core::ecdsa::Signature
   **/
  SpCoreEcdsaSignature: '[u8;65]',
  /**
   * Lookup488: frame_system::extensions::check_non_zero_sender::CheckNonZeroSender<T>
   **/
  FrameSystemExtensionsCheckNonZeroSender: 'Null',
  /**
   * Lookup489: frame_system::extensions::check_spec_version::CheckSpecVersion<T>
   **/
  FrameSystemExtensionsCheckSpecVersion: 'Null',
  /**
   * Lookup490: frame_system::extensions::check_tx_version::CheckTxVersion<T>
   **/
  FrameSystemExtensionsCheckTxVersion: 'Null',
  /**
   * Lookup491: frame_system::extensions::check_genesis::CheckGenesis<T>
   **/
  FrameSystemExtensionsCheckGenesis: 'Null',
  /**
   * Lookup494: frame_system::extensions::check_nonce::CheckNonce<T>
   **/
  FrameSystemExtensionsCheckNonce: 'Compact<u32>',
  /**
   * Lookup495: frame_system::extensions::check_weight::CheckWeight<T>
   **/
  FrameSystemExtensionsCheckWeight: 'Null',
  /**
   * Lookup496: pallet_asset_tx_payment::ChargeAssetTxPayment<T>
   **/
  PalletAssetTxPaymentChargeAssetTxPayment: {
    tip: 'Compact<u128>',
    assetId: 'Option<u32>'
  }
};
