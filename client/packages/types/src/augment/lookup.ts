// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

/* eslint-disable sort-keys */

export default {
  /**
   * Lookup3: frame_system::AccountInfo<Index, pallet_balances::AccountData<Balance>>
   **/
  FrameSystemAccountInfo: {
    nonce: 'u32',
    consumers: 'u32',
    providers: 'u32',
    sufficients: 'u32',
    data: 'PalletBalancesAccountData'
  },
  /**
   * Lookup5: pallet_balances::AccountData<Balance>
   **/
  PalletBalancesAccountData: {
    free: 'u128',
    reserved: 'u128',
    miscFrozen: 'u128',
    feeFrozen: 'u128'
  },
  /**
   * Lookup7: frame_support::weights::PerDispatchClass<T>
   **/
  FrameSupportWeightsPerDispatchClassU64: {
    normal: 'u64',
    operational: 'u64',
    mandatory: 'u64'
  },
  /**
   * Lookup11: sp_runtime::generic::digest::Digest
   **/
  SpRuntimeDigest: {
    logs: 'Vec<SpRuntimeDigestDigestItem>'
  },
  /**
   * Lookup13: sp_runtime::generic::digest::DigestItem
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
   * Lookup16: frame_system::EventRecord<t0rn_parachain_runtime::Event, primitive_types::H256>
   **/
  FrameSystemEventRecord: {
    phase: 'FrameSystemPhase',
    event: 'Event',
    topics: 'Vec<H256>'
  },
  /**
   * Lookup18: frame_system::pallet::Event<T>
   **/
  FrameSystemEvent: {
    _enum: {
      ExtrinsicSuccess: {
        dispatchInfo: 'FrameSupportWeightsDispatchInfo',
      },
      ExtrinsicFailed: {
        dispatchError: 'SpRuntimeDispatchError',
        dispatchInfo: 'FrameSupportWeightsDispatchInfo',
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
   * Lookup19: frame_support::weights::DispatchInfo
   **/
  FrameSupportWeightsDispatchInfo: {
    weight: 'u64',
    class: 'FrameSupportWeightsDispatchClass',
    paysFee: 'FrameSupportWeightsPays'
  },
  /**
   * Lookup20: frame_support::weights::DispatchClass
   **/
  FrameSupportWeightsDispatchClass: {
    _enum: ['Normal', 'Operational', 'Mandatory']
  },
  /**
   * Lookup21: frame_support::weights::Pays
   **/
  FrameSupportWeightsPays: {
    _enum: ['Yes', 'No']
  },
  /**
   * Lookup22: sp_runtime::DispatchError
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
      Arithmetic: 'SpRuntimeArithmeticError',
      Transactional: 'SpRuntimeTransactionalError'
    }
  },
  /**
   * Lookup23: sp_runtime::ModuleError
   **/
  SpRuntimeModuleError: {
    index: 'u8',
    error: '[u8;4]'
  },
  /**
   * Lookup24: sp_runtime::TokenError
   **/
  SpRuntimeTokenError: {
    _enum: ['NoFunds', 'WouldDie', 'BelowMinimum', 'CannotCreate', 'UnknownAsset', 'Frozen', 'Unsupported']
  },
  /**
   * Lookup25: sp_runtime::ArithmeticError
   **/
  SpRuntimeArithmeticError: {
    _enum: ['Underflow', 'Overflow', 'DivisionByZero']
  },
  /**
   * Lookup26: sp_runtime::TransactionalError
   **/
  SpRuntimeTransactionalError: {
    _enum: ['LimitReached', 'NoLayer']
  },
  /**
   * Lookup27: cumulus_pallet_parachain_system::pallet::Event<T>
   **/
  CumulusPalletParachainSystemEvent: {
    _enum: {
      ValidationFunctionStored: 'Null',
      ValidationFunctionApplied: {
        relayChainBlockNum: 'u32',
      },
      ValidationFunctionDiscarded: 'Null',
      UpgradeAuthorized: {
        codeHash: 'H256',
      },
      DownwardMessagesReceived: {
        count: 'u32',
      },
      DownwardMessagesProcessed: {
        weightUsed: 'u64',
        dmqHead: 'H256'
      }
    }
  },
  /**
   * Lookup28: pallet_preimage::pallet::Event<T>
   **/
  PalletPreimageEvent: {
    _enum: {
      Noted: {
        _alias: {
          hash_: 'hash',
        },
        hash_: 'H256',
      },
      Requested: {
        _alias: {
          hash_: 'hash',
        },
        hash_: 'H256',
      },
      Cleared: {
        _alias: {
          hash_: 'hash',
        },
        hash_: 'H256'
      }
    }
  },
  /**
   * Lookup29: pallet_scheduler::pallet::Event<T>
   **/
  PalletSchedulerEvent: {
    _enum: {
      Scheduled: {
        when: 'u32',
        index: 'u32',
      },
      Canceled: {
        when: 'u32',
        index: 'u32',
      },
      Dispatched: {
        task: '(u32,u32)',
        id: 'Option<Bytes>',
        result: 'Result<Null, SpRuntimeDispatchError>',
      },
      CallLookupFailed: {
        task: '(u32,u32)',
        id: 'Option<Bytes>',
        error: 'FrameSupportScheduleLookupError'
      }
    }
  },
  /**
   * Lookup34: frame_support::traits::schedule::LookupError
   **/
  FrameSupportScheduleLookupError: {
    _enum: ['Unknown', 'BadFormat']
  },
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
   * Lookup36: pallet_identity::pallet::Event<T>
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
   * Lookup37: pallet_balances::pallet::Event<T, I>
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
        reserved: 'u128',
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
        amount: 'u128'
      }
    }
  },
  /**
   * Lookup38: frame_support::traits::tokens::misc::BalanceStatus
   **/
  FrameSupportTokensMiscBalanceStatus: {
    _enum: ['Free', 'Reserved']
  },
  /**
   * Lookup39: pallet_transaction_payment::pallet::Event<T>
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
   * Lookup40: pallet_assets::pallet::Event<T, I>
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
        totalSupply: 'u128',
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
        assetId: 'u32'
      }
    }
  },
  /**
   * Lookup42: pallet_account_manager::pallet::Event<T>
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
   * Lookup44: pallet_treasury::pallet::Event<T, I>
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
        beneficiary: 'AccountId32'
      }
    }
  },
  /**
   * Lookup49: pallet_collator_selection::pallet::Event<T>
   **/
  PalletCollatorSelectionEvent: {
    _enum: {
      NewInvulnerables: {
        invulnerables: 'Vec<AccountId32>',
      },
      NewDesiredCandidates: {
        desiredCandidates: 'u32',
      },
      NewCandidacyBond: {
        bondAmount: 'u128',
      },
      CandidateAdded: {
        accountId: 'AccountId32',
        deposit: 'u128',
      },
      CandidateRemoved: {
        accountId: 'AccountId32'
      }
    }
  },
  /**
   * Lookup51: pallet_session::pallet::Event
   **/
  PalletSessionEvent: {
    _enum: {
      NewSession: {
        sessionIndex: 'u32'
      }
    }
  },
  /**
   * Lookup52: cumulus_pallet_xcmp_queue::pallet::Event<T>
   **/
  CumulusPalletXcmpQueueEvent: {
    _enum: {
      Success: {
        messageHash: 'Option<H256>',
        weight: 'u64',
      },
      Fail: {
        messageHash: 'Option<H256>',
        error: 'XcmV2TraitsError',
        weight: 'u64',
      },
      BadVersion: {
        messageHash: 'Option<H256>',
      },
      BadFormat: {
        messageHash: 'Option<H256>',
      },
      UpwardMessageSent: {
        messageHash: 'Option<H256>',
      },
      XcmpMessageSent: {
        messageHash: 'Option<H256>',
      },
      OverweightEnqueued: {
        sender: 'u32',
        sentAt: 'u32',
        index: 'u64',
        required: 'u64',
      },
      OverweightServiced: {
        index: 'u64',
        used: 'u64'
      }
    }
  },
  /**
   * Lookup54: xcm::v2::traits::Error
   **/
  XcmV2TraitsError: {
    _enum: {
      Overflow: 'Null',
      Unimplemented: 'Null',
      UntrustedReserveLocation: 'Null',
      UntrustedTeleportLocation: 'Null',
      MultiLocationFull: 'Null',
      MultiLocationNotInvertible: 'Null',
      BadOrigin: 'Null',
      InvalidLocation: 'Null',
      AssetNotFound: 'Null',
      FailedToTransactAsset: 'Null',
      NotWithdrawable: 'Null',
      LocationCannotHold: 'Null',
      ExceedsMaxMessageSize: 'Null',
      DestinationUnsupported: 'Null',
      Transport: 'Null',
      Unroutable: 'Null',
      UnknownClaim: 'Null',
      FailedToDecode: 'Null',
      MaxWeightInvalid: 'Null',
      NotHoldingFees: 'Null',
      TooExpensive: 'Null',
      Trap: 'u64',
      UnhandledXcmVersion: 'Null',
      WeightLimitReached: 'u64',
      Barrier: 'Null',
      WeightNotComputable: 'Null'
    }
  },
  /**
   * Lookup56: pallet_xcm::pallet::Event<T>
   **/
  PalletXcmEvent: {
    _enum: {
      Attempted: 'XcmV2TraitsOutcome',
      Sent: '(XcmV1MultiLocation,XcmV1MultiLocation,XcmV2Xcm)',
      UnexpectedResponse: '(XcmV1MultiLocation,u64)',
      ResponseReady: '(u64,XcmV2Response)',
      Notified: '(u64,u8,u8)',
      NotifyOverweight: '(u64,u8,u8,u64,u64)',
      NotifyDispatchError: '(u64,u8,u8)',
      NotifyDecodeFailed: '(u64,u8,u8)',
      InvalidResponder: '(XcmV1MultiLocation,u64,Option<XcmV1MultiLocation>)',
      InvalidResponderVersion: '(XcmV1MultiLocation,u64)',
      ResponseTaken: 'u64',
      AssetsTrapped: '(H256,XcmV1MultiLocation,XcmVersionedMultiAssets)',
      VersionChangeNotified: '(XcmV1MultiLocation,u32)',
      SupportedVersionChanged: '(XcmV1MultiLocation,u32)',
      NotifyTargetSendFail: '(XcmV1MultiLocation,u64,XcmV2TraitsError)',
      NotifyTargetMigrationFail: '(XcmVersionedMultiLocation,u64)'
    }
  },
  /**
   * Lookup57: xcm::v2::traits::Outcome
   **/
  XcmV2TraitsOutcome: {
    _enum: {
      Complete: 'u64',
      Incomplete: '(u64,XcmV2TraitsError)',
      Error: 'XcmV2TraitsError'
    }
  },
  /**
   * Lookup58: xcm::v1::multilocation::MultiLocation
   **/
  XcmV1MultiLocation: {
    parents: 'u8',
    interior: 'XcmV1MultilocationJunctions'
  },
  /**
   * Lookup59: xcm::v1::multilocation::Junctions
   **/
  XcmV1MultilocationJunctions: {
    _enum: {
      Here: 'Null',
      X1: 'XcmV1Junction',
      X2: '(XcmV1Junction,XcmV1Junction)',
      X3: '(XcmV1Junction,XcmV1Junction,XcmV1Junction)',
      X4: '(XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction)',
      X5: '(XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction)',
      X6: '(XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction)',
      X7: '(XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction)',
      X8: '(XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction)'
    }
  },
  /**
   * Lookup60: xcm::v1::junction::Junction
   **/
  XcmV1Junction: {
    _enum: {
      Parachain: 'Compact<u32>',
      AccountId32: {
        network: 'XcmV0JunctionNetworkId',
        id: '[u8;32]',
      },
      AccountIndex64: {
        network: 'XcmV0JunctionNetworkId',
        index: 'Compact<u64>',
      },
      AccountKey20: {
        network: 'XcmV0JunctionNetworkId',
        key: '[u8;20]',
      },
      PalletInstance: 'u8',
      GeneralIndex: 'Compact<u128>',
      GeneralKey: 'Bytes',
      OnlyChild: 'Null',
      Plurality: {
        id: 'XcmV0JunctionBodyId',
        part: 'XcmV0JunctionBodyPart'
      }
    }
  },
  /**
   * Lookup62: xcm::v0::junction::NetworkId
   **/
  XcmV0JunctionNetworkId: {
    _enum: {
      Any: 'Null',
      Named: 'Bytes',
      Polkadot: 'Null',
      Kusama: 'Null'
    }
  },
  /**
   * Lookup67: xcm::v0::junction::BodyId
   **/
  XcmV0JunctionBodyId: {
    _enum: {
      Unit: 'Null',
      Named: 'Bytes',
      Index: 'Compact<u32>',
      Executive: 'Null',
      Technical: 'Null',
      Legislative: 'Null',
      Judicial: 'Null'
    }
  },
  /**
   * Lookup68: xcm::v0::junction::BodyPart
   **/
  XcmV0JunctionBodyPart: {
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
   * Lookup69: xcm::v2::Xcm<Call>
   **/
  XcmV2Xcm: 'Vec<XcmV2Instruction>',
  /**
   * Lookup71: xcm::v2::Instruction<Call>
   **/
  XcmV2Instruction: {
    _enum: {
      WithdrawAsset: 'XcmV1MultiassetMultiAssets',
      ReserveAssetDeposited: 'XcmV1MultiassetMultiAssets',
      ReceiveTeleportedAsset: 'XcmV1MultiassetMultiAssets',
      QueryResponse: {
        queryId: 'Compact<u64>',
        response: 'XcmV2Response',
        maxWeight: 'Compact<u64>',
      },
      TransferAsset: {
        assets: 'XcmV1MultiassetMultiAssets',
        beneficiary: 'XcmV1MultiLocation',
      },
      TransferReserveAsset: {
        assets: 'XcmV1MultiassetMultiAssets',
        dest: 'XcmV1MultiLocation',
        xcm: 'XcmV2Xcm',
      },
      Transact: {
        originType: 'XcmV0OriginKind',
        requireWeightAtMost: 'Compact<u64>',
        call: 'XcmDoubleEncoded',
      },
      HrmpNewChannelOpenRequest: {
        sender: 'Compact<u32>',
        maxMessageSize: 'Compact<u32>',
        maxCapacity: 'Compact<u32>',
      },
      HrmpChannelAccepted: {
        recipient: 'Compact<u32>',
      },
      HrmpChannelClosing: {
        initiator: 'Compact<u32>',
        sender: 'Compact<u32>',
        recipient: 'Compact<u32>',
      },
      ClearOrigin: 'Null',
      DescendOrigin: 'XcmV1MultilocationJunctions',
      ReportError: {
        queryId: 'Compact<u64>',
        dest: 'XcmV1MultiLocation',
        maxResponseWeight: 'Compact<u64>',
      },
      DepositAsset: {
        assets: 'XcmV1MultiassetMultiAssetFilter',
        maxAssets: 'Compact<u32>',
        beneficiary: 'XcmV1MultiLocation',
      },
      DepositReserveAsset: {
        assets: 'XcmV1MultiassetMultiAssetFilter',
        maxAssets: 'Compact<u32>',
        dest: 'XcmV1MultiLocation',
        xcm: 'XcmV2Xcm',
      },
      ExchangeAsset: {
        give: 'XcmV1MultiassetMultiAssetFilter',
        receive: 'XcmV1MultiassetMultiAssets',
      },
      InitiateReserveWithdraw: {
        assets: 'XcmV1MultiassetMultiAssetFilter',
        reserve: 'XcmV1MultiLocation',
        xcm: 'XcmV2Xcm',
      },
      InitiateTeleport: {
        assets: 'XcmV1MultiassetMultiAssetFilter',
        dest: 'XcmV1MultiLocation',
        xcm: 'XcmV2Xcm',
      },
      QueryHolding: {
        queryId: 'Compact<u64>',
        dest: 'XcmV1MultiLocation',
        assets: 'XcmV1MultiassetMultiAssetFilter',
        maxResponseWeight: 'Compact<u64>',
      },
      BuyExecution: {
        fees: 'XcmV1MultiAsset',
        weightLimit: 'XcmV2WeightLimit',
      },
      RefundSurplus: 'Null',
      SetErrorHandler: 'XcmV2Xcm',
      SetAppendix: 'XcmV2Xcm',
      ClearError: 'Null',
      ClaimAsset: {
        assets: 'XcmV1MultiassetMultiAssets',
        ticket: 'XcmV1MultiLocation',
      },
      Trap: 'Compact<u64>',
      SubscribeVersion: {
        queryId: 'Compact<u64>',
        maxResponseWeight: 'Compact<u64>',
      },
      UnsubscribeVersion: 'Null'
    }
  },
  /**
   * Lookup72: xcm::v1::multiasset::MultiAssets
   **/
  XcmV1MultiassetMultiAssets: 'Vec<XcmV1MultiAsset>',
  /**
   * Lookup74: xcm::v1::multiasset::MultiAsset
   **/
  XcmV1MultiAsset: {
    id: 'XcmV1MultiassetAssetId',
    fun: 'XcmV1MultiassetFungibility'
  },
  /**
   * Lookup75: xcm::v1::multiasset::AssetId
   **/
  XcmV1MultiassetAssetId: {
    _enum: {
      Concrete: 'XcmV1MultiLocation',
      Abstract: 'Bytes'
    }
  },
  /**
   * Lookup76: xcm::v1::multiasset::Fungibility
   **/
  XcmV1MultiassetFungibility: {
    _enum: {
      Fungible: 'Compact<u128>',
      NonFungible: 'XcmV1MultiassetAssetInstance'
    }
  },
  /**
   * Lookup77: xcm::v1::multiasset::AssetInstance
   **/
  XcmV1MultiassetAssetInstance: {
    _enum: {
      Undefined: 'Null',
      Index: 'Compact<u128>',
      Array4: '[u8;4]',
      Array8: '[u8;8]',
      Array16: '[u8;16]',
      Array32: '[u8;32]',
      Blob: 'Bytes'
    }
  },
  /**
   * Lookup80: xcm::v2::Response
   **/
  XcmV2Response: {
    _enum: {
      Null: 'Null',
      Assets: 'XcmV1MultiassetMultiAssets',
      ExecutionResult: 'Option<(u32,XcmV2TraitsError)>',
      Version: 'u32'
    }
  },
  /**
   * Lookup83: xcm::v0::OriginKind
   **/
  XcmV0OriginKind: {
    _enum: ['Native', 'SovereignAccount', 'Superuser', 'Xcm']
  },
  /**
   * Lookup84: xcm::double_encoded::DoubleEncoded<T>
   **/
  XcmDoubleEncoded: {
    encoded: 'Bytes'
  },
  /**
   * Lookup85: xcm::v1::multiasset::MultiAssetFilter
   **/
  XcmV1MultiassetMultiAssetFilter: {
    _enum: {
      Definite: 'XcmV1MultiassetMultiAssets',
      Wild: 'XcmV1MultiassetWildMultiAsset'
    }
  },
  /**
   * Lookup86: xcm::v1::multiasset::WildMultiAsset
   **/
  XcmV1MultiassetWildMultiAsset: {
    _enum: {
      All: 'Null',
      AllOf: {
        id: 'XcmV1MultiassetAssetId',
        fun: 'XcmV1MultiassetWildFungibility'
      }
    }
  },
  /**
   * Lookup87: xcm::v1::multiasset::WildFungibility
   **/
  XcmV1MultiassetWildFungibility: {
    _enum: ['Fungible', 'NonFungible']
  },
  /**
   * Lookup88: xcm::v2::WeightLimit
   **/
  XcmV2WeightLimit: {
    _enum: {
      Unlimited: 'Null',
      Limited: 'Compact<u64>'
    }
  },
  /**
   * Lookup90: xcm::VersionedMultiAssets
   **/
  XcmVersionedMultiAssets: {
    _enum: {
      V0: 'Vec<XcmV0MultiAsset>',
      V1: 'XcmV1MultiassetMultiAssets'
    }
  },
  /**
   * Lookup92: xcm::v0::multi_asset::MultiAsset
   **/
  XcmV0MultiAsset: {
    _enum: {
      None: 'Null',
      All: 'Null',
      AllFungible: 'Null',
      AllNonFungible: 'Null',
      AllAbstractFungible: {
        id: 'Bytes',
      },
      AllAbstractNonFungible: {
        class: 'Bytes',
      },
      AllConcreteFungible: {
        id: 'XcmV0MultiLocation',
      },
      AllConcreteNonFungible: {
        class: 'XcmV0MultiLocation',
      },
      AbstractFungible: {
        id: 'Bytes',
        amount: 'Compact<u128>',
      },
      AbstractNonFungible: {
        class: 'Bytes',
        instance: 'XcmV1MultiassetAssetInstance',
      },
      ConcreteFungible: {
        id: 'XcmV0MultiLocation',
        amount: 'Compact<u128>',
      },
      ConcreteNonFungible: {
        class: 'XcmV0MultiLocation',
        instance: 'XcmV1MultiassetAssetInstance'
      }
    }
  },
  /**
   * Lookup93: xcm::v0::multi_location::MultiLocation
   **/
  XcmV0MultiLocation: {
    _enum: {
      Null: 'Null',
      X1: 'XcmV0Junction',
      X2: '(XcmV0Junction,XcmV0Junction)',
      X3: '(XcmV0Junction,XcmV0Junction,XcmV0Junction)',
      X4: '(XcmV0Junction,XcmV0Junction,XcmV0Junction,XcmV0Junction)',
      X5: '(XcmV0Junction,XcmV0Junction,XcmV0Junction,XcmV0Junction,XcmV0Junction)',
      X6: '(XcmV0Junction,XcmV0Junction,XcmV0Junction,XcmV0Junction,XcmV0Junction,XcmV0Junction)',
      X7: '(XcmV0Junction,XcmV0Junction,XcmV0Junction,XcmV0Junction,XcmV0Junction,XcmV0Junction,XcmV0Junction)',
      X8: '(XcmV0Junction,XcmV0Junction,XcmV0Junction,XcmV0Junction,XcmV0Junction,XcmV0Junction,XcmV0Junction,XcmV0Junction)'
    }
  },
  /**
   * Lookup94: xcm::v0::junction::Junction
   **/
  XcmV0Junction: {
    _enum: {
      Parent: 'Null',
      Parachain: 'Compact<u32>',
      AccountId32: {
        network: 'XcmV0JunctionNetworkId',
        id: '[u8;32]',
      },
      AccountIndex64: {
        network: 'XcmV0JunctionNetworkId',
        index: 'Compact<u64>',
      },
      AccountKey20: {
        network: 'XcmV0JunctionNetworkId',
        key: '[u8;20]',
      },
      PalletInstance: 'u8',
      GeneralIndex: 'Compact<u128>',
      GeneralKey: 'Bytes',
      OnlyChild: 'Null',
      Plurality: {
        id: 'XcmV0JunctionBodyId',
        part: 'XcmV0JunctionBodyPart'
      }
    }
  },
  /**
   * Lookup95: xcm::VersionedMultiLocation
   **/
  XcmVersionedMultiLocation: {
    _enum: {
      V0: 'XcmV0MultiLocation',
      V1: 'XcmV1MultiLocation'
    }
  },
  /**
   * Lookup96: cumulus_pallet_xcm::pallet::Event<T>
   **/
  CumulusPalletXcmEvent: {
    _enum: {
      InvalidFormat: '[u8;8]',
      UnsupportedVersion: '[u8;8]',
      ExecutedDownward: '([u8;8],XcmV2TraitsOutcome)'
    }
  },
  /**
   * Lookup97: cumulus_pallet_dmp_queue::pallet::Event<T>
   **/
  CumulusPalletDmpQueueEvent: {
    _enum: {
      InvalidFormat: {
        messageId: '[u8;32]',
      },
      UnsupportedVersion: {
        messageId: '[u8;32]',
      },
      ExecutedDownward: {
        messageId: '[u8;32]',
        outcome: 'XcmV2TraitsOutcome',
      },
      WeightExhausted: {
        messageId: '[u8;32]',
        remainingWeight: 'u64',
        requiredWeight: 'u64',
      },
      OverweightEnqueued: {
        messageId: '[u8;32]',
        overweightIndex: 'u64',
        requiredWeight: 'u64',
      },
      OverweightServiced: {
        overweightIndex: 'u64',
        weightUsed: 'u64'
      }
    }
  },
  /**
   * Lookup98: pallet_xbi_portal::pallet::Event<T>
   **/
  PalletXbiPortalEvent: {
    _enum: {
      XbiMessageReceived: {
        request: 'Option<XpFormatXbiFormat>',
        response: 'Option<XpFormatXbiResult>',
      },
      XbiMessageSent: {
        msg: 'XpChannelMessage',
      },
      XbiRequestHandled: {
        result: 'XpFormatXbiResult',
        metadata: 'XpFormatXbiMetadata',
        weight: 'u64',
      },
      XbiInstructionHandled: {
        msg: 'XpFormatXbiFormat',
        weight: 'u64',
      },
      QueueEmpty: 'Null',
      QueuePopped: {
        signal: 'XpChannelQueueQueueSignal',
        msg: 'XpChannelMessage',
      },
      QueuePushed: {
        signal: 'XpChannelQueueQueueSignal',
        msg: 'XpChannelMessage',
      },
      ResponseStored: {
        _alias: {
          hash_: 'hash',
        },
        hash_: 'H256',
        result: 'XpFormatXbiResult'
      }
    }
  },
  /**
   * Lookup100: xp_format::XbiFormat
   **/
  XpFormatXbiFormat: {
    instr: 'XpFormatXbiInstruction',
    metadata: 'XpFormatXbiMetadata'
  },
  /**
   * Lookup101: xp_format::XbiInstruction
   **/
  XpFormatXbiInstruction: {
    _enum: {
      Unknown: {
        identifier: 'u8',
        params: 'Bytes',
      },
      CallNative: {
        payload: 'Bytes',
      },
      CallEvm: {
        source: 'H160',
        target: 'H160',
        value: 'U256',
        input: 'Bytes',
        gasLimit: 'u64',
        maxFeePerGas: 'U256',
        maxPriorityFeePerGas: 'Option<U256>',
        nonce: 'Option<U256>',
        accessList: 'Vec<(H160,Vec<H256>)>',
      },
      CallWasm: {
        dest: 'AccountId32',
        value: 'u128',
        gasLimit: 'u64',
        storageDepositLimit: 'Option<u128>',
        data: 'Bytes',
      },
      CallCustom: {
        caller: 'AccountId32',
        dest: 'AccountId32',
        value: 'u128',
        input: 'Bytes',
        limit: 'u64',
        additionalParams: 'Bytes',
      },
      Transfer: {
        dest: 'AccountId32',
        value: 'u128',
      },
      TransferAssets: {
        currencyId: 'u32',
        dest: 'AccountId32',
        value: 'u128',
      },
      Swap: {
        assetOut: 'u32',
        assetIn: 'u32',
        amount: 'u128',
        maxLimit: 'u128',
        discount: 'bool',
      },
      AddLiquidity: {
        assetA: 'u32',
        assetB: 'u32',
        amountA: 'u128',
        amountBMaxLimit: 'u128',
      },
      RemoveLiquidity: {
        assetA: 'u32',
        assetB: 'u32',
        liquidityAmount: 'u128',
      },
      GetPrice: {
        assetA: 'u32',
        assetB: 'u32',
        amount: 'u128'
      }
    }
  },
  /**
   * Lookup110: xp_format::XbiMetadata
   **/
  XpFormatXbiMetadata: {
    id: 'H256',
    destParaId: 'u32',
    srcParaId: 'u32',
    timeouts: 'XpFormatTimeouts',
    timesheet: 'XpFormatXbiTimeSheet',
    fees: 'XpFormatFees',
    origin: 'Option<AccountId32>'
  },
  /**
   * Lookup111: xp_format::Timeouts
   **/
  XpFormatTimeouts: {
    sent: 'XpFormatActionNotificationTimeouts',
    delivered: 'XpFormatActionNotificationTimeouts',
    executed: 'XpFormatActionNotificationTimeouts',
    responded: 'XpFormatActionNotificationTimeouts'
  },
  /**
   * Lookup112: xp_format::ActionNotificationTimeouts
   **/
  XpFormatActionNotificationTimeouts: {
    action: 'u32',
    notification: 'u32'
  },
  /**
   * Lookup113: xp_format::XbiTimeSheet<BlockNumber>
   **/
  XpFormatXbiTimeSheet: {
    submitted: 'Option<u32>',
    sent: 'Option<u32>',
    delivered: 'Option<u32>',
    executed: 'Option<u32>',
    responded: 'Option<u32>',
    received: 'Option<u32>'
  },
  /**
   * Lookup115: xp_format::Fees
   **/
  XpFormatFees: {
    asset: 'Option<u32>',
    executionCostLimit: 'u128',
    notificationCostLimit: 'u128',
    aggregatedCost: 'u128'
  },
  /**
   * Lookup117: xp_format::XbiResult
   **/
  XpFormatXbiResult: {
    status: 'XpFormatStatus',
    output: 'Bytes',
    witness: 'Bytes'
  },
  /**
   * Lookup118: xp_format::Status
   **/
  XpFormatStatus: {
    _enum: ['Success', 'FailedExecution', 'DispatchFailed', 'ExecutionLimitExceeded', 'NotificationLimitExceeded', 'SendTimeout', 'DeliveryTimeout', 'ExecutionTimeout']
  },
  /**
   * Lookup119: xp_channel::Message
   **/
  XpChannelMessage: {
    _enum: {
      Request: 'XpFormatXbiFormat',
      Response: '(XpFormatXbiResult,XpFormatXbiMetadata)'
    }
  },
  /**
   * Lookup120: xp_channel::queue::QueueSignal
   **/
  XpChannelQueueQueueSignal: {
    _enum: {
      PendingRequest: 'Null',
      PendingExecution: 'Null',
      PendingResponse: 'Null',
      PendingResult: 'Null',
      ProtocolError: 'XpFormatStatus'
    }
  },
  /**
   * Lookup121: pallet_asset_registry::pallet::Event<T>
   **/
  PalletAssetRegistryEvent: {
    _enum: {
      Registered: {
        assetId: 'u32',
        location: 'XcmV1MultiLocation',
      },
      Info: {
        assetId: 'u32',
        location: 'XcmV1MultiLocation'
      }
    }
  },
  /**
   * Lookup122: pallet_xdns::pallet::Event<T>
   **/
  PalletXdnsEvent: {
    _enum: {
      GatewayRecordStored: '[u8;4]',
      NewTokenLinkedToGateway: '(u32,[u8;4])',
      NewTokenAssetRegistered: '(u32,[u8;4])',
      GatewayRecordPurged: '(AccountId32,[u8;4])',
      XdnsRecordPurged: '(AccountId32,[u8;4])',
      XdnsRecordUpdated: '[u8;4]'
    }
  },
  /**
   * Lookup123: pallet_attesters::pallet::Event<T>
   **/
  PalletAttestersEvent: {
    _enum: {
      AttesterRegistered: 'AccountId32',
      AttesterDeregistrationScheduled: '(AccountId32,u32)',
      AttesterDeregistered: 'AccountId32',
      AttestationSubmitted: 'AccountId32',
      NewAttestationBatch: '([u8;4],PalletAttestersBatchMessage)',
      NewAttestationMessageHash: '([u8;4],H256,T3rnPrimitivesExecutionVendor)',
      NewConfirmationBatch: '([u8;4],PalletAttestersBatchMessage,Bytes,H256)',
      Nominated: '(AccountId32,AccountId32,u128)',
      NewTargetActivated: '[u8;4]',
      NewTargetProposed: '[u8;4]',
      AttesterAgreedToNewTarget: '(AccountId32,[u8;4],Bytes)',
      CurrentPendingAttestationBatches: '([u8;4],Vec<(u32,H256)>)'
    }
  },
  /**
   * Lookup124: pallet_attesters::pallet::BatchMessage<BlockNumber>
   **/
  PalletAttestersBatchMessage: {
    committedSfx: 'Option<Vec<H256>>',
    revertedSfx: 'Option<Vec<H256>>',
    nextCommittee: 'Option<Vec<Bytes>>',
    bannedCommittee: 'Option<Vec<Bytes>>',
    index: 'u32',
    signatures: 'Vec<(u32,[u8;65])>',
    created: 'u32',
    status: 'PalletAttestersBatchStatus',
    latency: 'T3rnPrimitivesAttestersLatencyStatus'
  },
  /**
   * Lookup131: pallet_attesters::pallet::BatchStatus
   **/
  PalletAttestersBatchStatus: {
    _enum: ['PendingMessage', 'PendingAttestation', 'ReadyForSubmissionByMajority', 'ReadyForSubmissionFullyApproved', 'Repatriated', 'Expired', 'Committed']
  },
  /**
   * Lookup132: t3rn_primitives::attesters::LatencyStatus
   **/
  T3rnPrimitivesAttestersLatencyStatus: {
    _enum: {
      OnTime: 'Null',
      Late: '(u32,u32)'
    }
  },
  /**
   * Lookup133: t3rn_primitives::ExecutionVendor
   **/
  T3rnPrimitivesExecutionVendor: {
    _enum: ['Substrate', 'EVM']
  },
  /**
   * Lookup136: pallet_rewards::pallet::Event<T>
   **/
  PalletRewardsEvent: {
    _enum: {
      AttesterRewarded: '(AccountId32,u128)',
      CollatorRewarded: '(AccountId32,u128)',
      ExecutorRewarded: '(AccountId32,u128)',
      NewMaxRewardExecutorsKickbackSet: '(Percent,Percent)',
      Claimed: '(AccountId32,u128)',
      PendingClaim: '(AccountId32,u128)'
    }
  },
  /**
   * Lookup138: pallet_contracts_registry::pallet::Event<T>
   **/
  PalletContractsRegistryEvent: {
    _enum: {
      ContractStored: '(AccountId32,H256)',
      ContractPurged: '(AccountId32,H256)'
    }
  },
  /**
   * Lookup139: pallet_circuit::pallet::Event<T>
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
      Result: '(AccountId32,AccountId32,XpFormatXbiResult,Bytes,Bytes)',
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
   * Lookup141: t3rn_types::sfx::SideEffect<sp_core::crypto::AccountId32, BalanceOf>
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
   * Lookup144: t3rn_types::fsx::FullSideEffect<sp_core::crypto::AccountId32, BlockNumber, BalanceOf>
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
   * Lookup146: t3rn_types::sfx::ConfirmedSideEffect<sp_core::crypto::AccountId32, BlockNumber, BalanceOf>
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
   * Lookup148: t3rn_types::sfx::ConfirmationOutcome
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
   * Lookup149: t3rn_types::sfx::SecurityLvl
   **/
  T3rnTypesSfxSecurityLvl: {
    _enum: ['Optimistic', 'Escrow']
  },
  /**
   * Lookup151: t3rn_types::bid::SFXBid<sp_core::crypto::AccountId32, BalanceOf, AssetId>
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
   * Lookup152: pallet_clock::pallet::Event<T>
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
   * Lookup153: pallet_3vm::pallet::Event<T>
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
   * Lookup155: t3rn_sdk_primitives::signal::SignalKind
   **/
  T3rnSdkPrimitivesSignalSignalKind: {
    _enum: {
      Complete: 'Null',
      Kill: 'T3rnSdkPrimitivesSignalKillReason'
    }
  },
  /**
   * Lookup156: t3rn_sdk_primitives::signal::KillReason
   **/
  T3rnSdkPrimitivesSignalKillReason: {
    _enum: ['Unhandled', 'Codec', 'Timeout']
  },
  /**
   * Lookup158: t3rn_primitives::contract_metadata::ContractType
   **/
  T3rnPrimitivesContractMetadataContractType: {
    _enum: ['System', 'VanillaEvm', 'VanillaWasm', 'VolatileEvm', 'VolatileWasm']
  },
  /**
   * Lookup160: pallet_contracts::pallet::Event<T>
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
        oldCodeHash: 'H256'
      }
    }
  },
  /**
   * Lookup161: pallet_evm::pallet::Event<T>
   **/
  PalletEvmEvent: {
    _enum: {
      Log: 'EthereumLog',
      Created: 'H160',
      CreatedFailed: 'H160',
      Executed: 'H160',
      ExecutedFailed: 'H160',
      BalanceDeposit: '(AccountId32,H160,U256)',
      BalanceWithdraw: '(AccountId32,H160,U256)',
      ClaimAccount: {
        accountId: 'AccountId32',
        evmAddress: 'H160'
      }
    }
  },
  /**
   * Lookup162: ethereum::log::Log
   **/
  EthereumLog: {
    address: 'H160',
    topics: 'Vec<H256>',
    data: 'Bytes'
  },
  /**
   * Lookup163: pallet_portal::pallet::Event<T>
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
   * Lookup164: t3rn_primitives::GatewayVendor
   **/
  T3rnPrimitivesGatewayVendor: {
    _enum: ['Polkadot', 'Kusama', 'Rococo', 'Ethereum']
  },
  /**
   * Lookup165: pallet_grandpa_finality_verifier::pallet::Event<T, I>
   **/
  PalletGrandpaFinalityVerifierEvent: {
    _enum: {
      HeadersAdded: 'u32'
    }
  },
  /**
   * Lookup168: pallet_maintenance_mode::pallet::Event
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
   * Lookup169: pallet_sudo::pallet::Event<T>
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
   * Lookup170: frame_system::Phase
   **/
  FrameSystemPhase: {
    _enum: {
      ApplyExtrinsic: 'u32',
      Finalization: 'Null',
      Initialization: 'Null'
    }
  },
  /**
   * Lookup172: frame_system::LastRuntimeUpgradeInfo
   **/
  FrameSystemLastRuntimeUpgradeInfo: {
    specVersion: 'Compact<u32>',
    specName: 'Text'
  },
  /**
   * Lookup174: frame_system::pallet::Call<T>
   **/
  FrameSystemCall: {
    _enum: {
      fill_block: {
        ratio: 'Perbill',
      },
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
   * Lookup178: frame_system::limits::BlockWeights
   **/
  FrameSystemLimitsBlockWeights: {
    baseBlock: 'u64',
    maxBlock: 'u64',
    perClass: 'FrameSupportWeightsPerDispatchClassWeightsPerClass'
  },
  /**
   * Lookup179: frame_support::weights::PerDispatchClass<frame_system::limits::WeightsPerClass>
   **/
  FrameSupportWeightsPerDispatchClassWeightsPerClass: {
    normal: 'FrameSystemLimitsWeightsPerClass',
    operational: 'FrameSystemLimitsWeightsPerClass',
    mandatory: 'FrameSystemLimitsWeightsPerClass'
  },
  /**
   * Lookup180: frame_system::limits::WeightsPerClass
   **/
  FrameSystemLimitsWeightsPerClass: {
    baseExtrinsic: 'u64',
    maxExtrinsic: 'Option<u64>',
    maxTotal: 'Option<u64>',
    reserved: 'Option<u64>'
  },
  /**
   * Lookup182: frame_system::limits::BlockLength
   **/
  FrameSystemLimitsBlockLength: {
    max: 'FrameSupportWeightsPerDispatchClassU32'
  },
  /**
   * Lookup183: frame_support::weights::PerDispatchClass<T>
   **/
  FrameSupportWeightsPerDispatchClassU32: {
    normal: 'u32',
    operational: 'u32',
    mandatory: 'u32'
  },
  /**
   * Lookup184: frame_support::weights::RuntimeDbWeight
   **/
  FrameSupportWeightsRuntimeDbWeight: {
    read: 'u64',
    write: 'u64'
  },
  /**
   * Lookup185: sp_version::RuntimeVersion
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
   * Lookup190: frame_system::pallet::Error<T>
   **/
  FrameSystemError: {
    _enum: ['InvalidSpecName', 'SpecVersionNeedsToIncrease', 'FailedToExtractRuntimeVersion', 'NonDefaultComposite', 'NonZeroRefCount', 'CallFiltered']
  },
  /**
   * Lookup191: polkadot_primitives::v2::PersistedValidationData<primitive_types::H256, N>
   **/
  PolkadotPrimitivesV2PersistedValidationData: {
    parentHead: 'Bytes',
    relayParentNumber: 'u32',
    relayParentStorageRoot: 'H256',
    maxPovSize: 'u32'
  },
  /**
   * Lookup194: polkadot_primitives::v2::UpgradeRestriction
   **/
  PolkadotPrimitivesV2UpgradeRestriction: {
    _enum: ['Present']
  },
  /**
   * Lookup195: sp_trie::storage_proof::StorageProof
   **/
  SpTrieStorageProof: {
    trieNodes: 'BTreeSet<Bytes>'
  },
  /**
   * Lookup197: cumulus_pallet_parachain_system::relay_state_snapshot::MessagingStateSnapshot
   **/
  CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot: {
    dmqMqcHead: 'H256',
    relayDispatchQueueSize: '(u32,u32)',
    ingressChannels: 'Vec<(u32,PolkadotPrimitivesV2AbridgedHrmpChannel)>',
    egressChannels: 'Vec<(u32,PolkadotPrimitivesV2AbridgedHrmpChannel)>'
  },
  /**
   * Lookup200: polkadot_primitives::v2::AbridgedHrmpChannel
   **/
  PolkadotPrimitivesV2AbridgedHrmpChannel: {
    maxCapacity: 'u32',
    maxTotalSize: 'u32',
    maxMessageSize: 'u32',
    msgCount: 'u32',
    totalSize: 'u32',
    mqcHead: 'Option<H256>'
  },
  /**
   * Lookup201: polkadot_primitives::v2::AbridgedHostConfiguration
   **/
  PolkadotPrimitivesV2AbridgedHostConfiguration: {
    maxCodeSize: 'u32',
    maxHeadDataSize: 'u32',
    maxUpwardQueueCount: 'u32',
    maxUpwardQueueSize: 'u32',
    maxUpwardMessageSize: 'u32',
    maxUpwardMessageNumPerCandidate: 'u32',
    hrmpMaxMessageNumPerCandidate: 'u32',
    validationUpgradeCooldown: 'u32',
    validationUpgradeDelay: 'u32'
  },
  /**
   * Lookup207: polkadot_core_primitives::OutboundHrmpMessage<polkadot_parachain::primitives::Id>
   **/
  PolkadotCorePrimitivesOutboundHrmpMessage: {
    recipient: 'u32',
    data: 'Bytes'
  },
  /**
   * Lookup208: cumulus_pallet_parachain_system::pallet::Call<T>
   **/
  CumulusPalletParachainSystemCall: {
    _enum: {
      set_validation_data: {
        data: 'CumulusPrimitivesParachainInherentParachainInherentData',
      },
      sudo_send_upward_message: {
        message: 'Bytes',
      },
      authorize_upgrade: {
        codeHash: 'H256',
      },
      enact_authorized_upgrade: {
        code: 'Bytes'
      }
    }
  },
  /**
   * Lookup209: cumulus_primitives_parachain_inherent::ParachainInherentData
   **/
  CumulusPrimitivesParachainInherentParachainInherentData: {
    validationData: 'PolkadotPrimitivesV2PersistedValidationData',
    relayChainState: 'SpTrieStorageProof',
    downwardMessages: 'Vec<PolkadotCorePrimitivesInboundDownwardMessage>',
    horizontalMessages: 'BTreeMap<u32, Vec<PolkadotCorePrimitivesInboundHrmpMessage>>'
  },
  /**
   * Lookup211: polkadot_core_primitives::InboundDownwardMessage<BlockNumber>
   **/
  PolkadotCorePrimitivesInboundDownwardMessage: {
    sentAt: 'u32',
    msg: 'Bytes'
  },
  /**
   * Lookup214: polkadot_core_primitives::InboundHrmpMessage<BlockNumber>
   **/
  PolkadotCorePrimitivesInboundHrmpMessage: {
    sentAt: 'u32',
    data: 'Bytes'
  },
  /**
   * Lookup217: cumulus_pallet_parachain_system::pallet::Error<T>
   **/
  CumulusPalletParachainSystemError: {
    _enum: ['OverlappingUpgrades', 'ProhibitedByPolkadot', 'TooBig', 'ValidationDataNotAvailable', 'HostConfigurationNotAvailable', 'NotScheduled', 'NothingAuthorized', 'Unauthorized']
  },
  /**
   * Lookup218: pallet_timestamp::pallet::Call<T>
   **/
  PalletTimestampCall: {
    _enum: {
      set: {
        now: 'Compact<u64>'
      }
    }
  },
  /**
   * Lookup219: pallet_preimage::RequestStatus<sp_core::crypto::AccountId32, Balance>
   **/
  PalletPreimageRequestStatus: {
    _enum: {
      Unrequested: 'Option<(AccountId32,u128)>',
      Requested: 'u32'
    }
  },
  /**
   * Lookup223: pallet_preimage::pallet::Call<T>
   **/
  PalletPreimageCall: {
    _enum: {
      note_preimage: {
        bytes: 'Bytes',
      },
      unnote_preimage: {
        _alias: {
          hash_: 'hash',
        },
        hash_: 'H256',
      },
      request_preimage: {
        _alias: {
          hash_: 'hash',
        },
        hash_: 'H256',
      },
      unrequest_preimage: {
        _alias: {
          hash_: 'hash',
        },
        hash_: 'H256'
      }
    }
  },
  /**
   * Lookup224: pallet_preimage::pallet::Error<T>
   **/
  PalletPreimageError: {
    _enum: ['TooLarge', 'AlreadyNoted', 'NotAuthorized', 'NotNoted', 'Requested', 'NotRequested']
  },
  /**
   * Lookup227: pallet_scheduler::ScheduledV3<frame_support::traits::schedule::MaybeHashed<t0rn_parachain_runtime::Call, primitive_types::H256>, BlockNumber, t0rn_parachain_runtime::OriginCaller, sp_core::crypto::AccountId32>
   **/
  PalletSchedulerScheduledV3: {
    maybeId: 'Option<Bytes>',
    priority: 'u8',
    call: 'FrameSupportScheduleMaybeHashed',
    maybePeriodic: 'Option<(u32,u32)>',
    origin: 'T0rnParachainRuntimeOriginCaller'
  },
  /**
   * Lookup228: frame_support::traits::schedule::MaybeHashed<t0rn_parachain_runtime::Call, primitive_types::H256>
   **/
  FrameSupportScheduleMaybeHashed: {
    _enum: {
      Value: 'Call',
      Hash: 'H256'
    }
  },
  /**
   * Lookup230: pallet_scheduler::pallet::Call<T>
   **/
  PalletSchedulerCall: {
    _enum: {
      schedule: {
        when: 'u32',
        maybePeriodic: 'Option<(u32,u32)>',
        priority: 'u8',
        call: 'FrameSupportScheduleMaybeHashed',
      },
      cancel: {
        when: 'u32',
        index: 'u32',
      },
      schedule_named: {
        id: 'Bytes',
        when: 'u32',
        maybePeriodic: 'Option<(u32,u32)>',
        priority: 'u8',
        call: 'FrameSupportScheduleMaybeHashed',
      },
      cancel_named: {
        id: 'Bytes',
      },
      schedule_after: {
        after: 'u32',
        maybePeriodic: 'Option<(u32,u32)>',
        priority: 'u8',
        call: 'FrameSupportScheduleMaybeHashed',
      },
      schedule_named_after: {
        id: 'Bytes',
        after: 'u32',
        maybePeriodic: 'Option<(u32,u32)>',
        priority: 'u8',
        call: 'FrameSupportScheduleMaybeHashed'
      }
    }
  },
  /**
   * Lookup232: pallet_utility::pallet::Call<T>
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
        asOrigin: 'T0rnParachainRuntimeOriginCaller',
        call: 'Call',
      },
      force_batch: {
        calls: 'Vec<Call>'
      }
    }
  },
  /**
   * Lookup234: t0rn_parachain_runtime::OriginCaller
   **/
  T0rnParachainRuntimeOriginCaller: {
    _enum: {
      system: 'FrameSupportDispatchRawOrigin',
      __Unused1: 'Null',
      __Unused2: 'Null',
      Void: 'SpCoreVoid',
      __Unused4: 'Null',
      __Unused5: 'Null',
      __Unused6: 'Null',
      __Unused7: 'Null',
      __Unused8: 'Null',
      __Unused9: 'Null',
      __Unused10: 'Null',
      __Unused11: 'Null',
      __Unused12: 'Null',
      __Unused13: 'Null',
      __Unused14: 'Null',
      __Unused15: 'Null',
      __Unused16: 'Null',
      __Unused17: 'Null',
      __Unused18: 'Null',
      __Unused19: 'Null',
      __Unused20: 'Null',
      __Unused21: 'Null',
      __Unused22: 'Null',
      __Unused23: 'Null',
      __Unused24: 'Null',
      __Unused25: 'Null',
      __Unused26: 'Null',
      __Unused27: 'Null',
      __Unused28: 'Null',
      __Unused29: 'Null',
      __Unused30: 'Null',
      PolkadotXcm: 'PalletXcmOrigin',
      CumulusXcm: 'CumulusPalletXcmOrigin'
    }
  },
  /**
   * Lookup235: frame_support::dispatch::RawOrigin<sp_core::crypto::AccountId32>
   **/
  FrameSupportDispatchRawOrigin: {
    _enum: {
      Root: 'Null',
      Signed: 'AccountId32',
      None: 'Null'
    }
  },
  /**
   * Lookup236: pallet_xcm::pallet::Origin
   **/
  PalletXcmOrigin: {
    _enum: {
      Xcm: 'XcmV1MultiLocation',
      Response: 'XcmV1MultiLocation'
    }
  },
  /**
   * Lookup237: cumulus_pallet_xcm::pallet::Origin
   **/
  CumulusPalletXcmOrigin: {
    _enum: {
      Relay: 'Null',
      SiblingParachain: 'u32'
    }
  },
  /**
   * Lookup238: sp_core::Void
   **/
  SpCoreVoid: 'Null',
  /**
   * Lookup239: pallet_identity::pallet::Call<T>
   **/
  PalletIdentityCall: {
    _enum: {
      add_registrar: {
        account: 'AccountId32',
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
        new_: 'AccountId32',
      },
      set_fields: {
        index: 'Compact<u32>',
        fields: 'PalletIdentityBitFlags',
      },
      provide_judgement: {
        regIndex: 'Compact<u32>',
        target: 'MultiAddress',
        judgement: 'PalletIdentityJudgement',
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
   * Lookup240: pallet_identity::types::IdentityInfo<FieldLimit>
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
   * Lookup276: pallet_identity::types::BitFlags<pallet_identity::types::IdentityField>
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
   * Lookup277: pallet_identity::types::IdentityField
   **/
  PalletIdentityIdentityField: {
    _enum: ['__Unused0', 'Display', 'Legal', '__Unused3', 'Web', '__Unused5', '__Unused6', '__Unused7', 'Riot', '__Unused9', '__Unused10', '__Unused11', '__Unused12', '__Unused13', '__Unused14', '__Unused15', 'Email', '__Unused17', '__Unused18', '__Unused19', '__Unused20', '__Unused21', '__Unused22', '__Unused23', '__Unused24', '__Unused25', '__Unused26', '__Unused27', '__Unused28', '__Unused29', '__Unused30', '__Unused31', 'PgpFingerprint', '__Unused33', '__Unused34', '__Unused35', '__Unused36', '__Unused37', '__Unused38', '__Unused39', '__Unused40', '__Unused41', '__Unused42', '__Unused43', '__Unused44', '__Unused45', '__Unused46', '__Unused47', '__Unused48', '__Unused49', '__Unused50', '__Unused51', '__Unused52', '__Unused53', '__Unused54', '__Unused55', '__Unused56', '__Unused57', '__Unused58', '__Unused59', '__Unused60', '__Unused61', '__Unused62', '__Unused63', 'Image', '__Unused65', '__Unused66', '__Unused67', '__Unused68', '__Unused69', '__Unused70', '__Unused71', '__Unused72', '__Unused73', '__Unused74', '__Unused75', '__Unused76', '__Unused77', '__Unused78', '__Unused79', '__Unused80', '__Unused81', '__Unused82', '__Unused83', '__Unused84', '__Unused85', '__Unused86', '__Unused87', '__Unused88', '__Unused89', '__Unused90', '__Unused91', '__Unused92', '__Unused93', '__Unused94', '__Unused95', '__Unused96', '__Unused97', '__Unused98', '__Unused99', '__Unused100', '__Unused101', '__Unused102', '__Unused103', '__Unused104', '__Unused105', '__Unused106', '__Unused107', '__Unused108', '__Unused109', '__Unused110', '__Unused111', '__Unused112', '__Unused113', '__Unused114', '__Unused115', '__Unused116', '__Unused117', '__Unused118', '__Unused119', '__Unused120', '__Unused121', '__Unused122', '__Unused123', '__Unused124', '__Unused125', '__Unused126', '__Unused127', 'Twitter']
  },
  /**
   * Lookup280: pallet_identity::types::Judgement<Balance>
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
   * Lookup281: pallet_balances::pallet::Call<T, I>
   **/
  PalletBalancesCall: {
    _enum: {
      transfer: {
        dest: 'MultiAddress',
        value: 'Compact<u128>',
      },
      set_balance: {
        who: 'MultiAddress',
        newFree: 'Compact<u128>',
        newReserved: 'Compact<u128>',
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
        amount: 'u128'
      }
    }
  },
  /**
   * Lookup282: pallet_assets::pallet::Call<T, I>
   **/
  PalletAssetsCall: {
    _enum: {
      create: {
        id: 'Compact<u32>',
        admin: 'MultiAddress',
        minBalance: 'u128',
      },
      force_create: {
        id: 'Compact<u32>',
        owner: 'MultiAddress',
        isSufficient: 'bool',
        minBalance: 'Compact<u128>',
      },
      destroy: {
        id: 'Compact<u32>',
        witness: 'PalletAssetsDestroyWitness',
      },
      mint: {
        id: 'Compact<u32>',
        beneficiary: 'MultiAddress',
        amount: 'Compact<u128>',
      },
      burn: {
        id: 'Compact<u32>',
        who: 'MultiAddress',
        amount: 'Compact<u128>',
      },
      transfer: {
        id: 'Compact<u32>',
        target: 'MultiAddress',
        amount: 'Compact<u128>',
      },
      transfer_keep_alive: {
        id: 'Compact<u32>',
        target: 'MultiAddress',
        amount: 'Compact<u128>',
      },
      force_transfer: {
        id: 'Compact<u32>',
        source: 'MultiAddress',
        dest: 'MultiAddress',
        amount: 'Compact<u128>',
      },
      freeze: {
        id: 'Compact<u32>',
        who: 'MultiAddress',
      },
      thaw: {
        id: 'Compact<u32>',
        who: 'MultiAddress',
      },
      freeze_asset: {
        id: 'Compact<u32>',
      },
      thaw_asset: {
        id: 'Compact<u32>',
      },
      transfer_ownership: {
        id: 'Compact<u32>',
        owner: 'MultiAddress',
      },
      set_team: {
        id: 'Compact<u32>',
        issuer: 'MultiAddress',
        admin: 'MultiAddress',
        freezer: 'MultiAddress',
      },
      set_metadata: {
        id: 'Compact<u32>',
        name: 'Bytes',
        symbol: 'Bytes',
        decimals: 'u8',
      },
      clear_metadata: {
        id: 'Compact<u32>',
      },
      force_set_metadata: {
        id: 'Compact<u32>',
        name: 'Bytes',
        symbol: 'Bytes',
        decimals: 'u8',
        isFrozen: 'bool',
      },
      force_clear_metadata: {
        id: 'Compact<u32>',
      },
      force_asset_status: {
        id: 'Compact<u32>',
        owner: 'MultiAddress',
        issuer: 'MultiAddress',
        admin: 'MultiAddress',
        freezer: 'MultiAddress',
        minBalance: 'Compact<u128>',
        isSufficient: 'bool',
        isFrozen: 'bool',
      },
      approve_transfer: {
        id: 'Compact<u32>',
        delegate: 'MultiAddress',
        amount: 'Compact<u128>',
      },
      cancel_approval: {
        id: 'Compact<u32>',
        delegate: 'MultiAddress',
      },
      force_cancel_approval: {
        id: 'Compact<u32>',
        owner: 'MultiAddress',
        delegate: 'MultiAddress',
      },
      transfer_approved: {
        id: 'Compact<u32>',
        owner: 'MultiAddress',
        destination: 'MultiAddress',
        amount: 'Compact<u128>',
      },
      touch: {
        id: 'Compact<u32>',
      },
      refund: {
        id: 'Compact<u32>',
        allowBurn: 'bool'
      }
    }
  },
  /**
   * Lookup283: pallet_assets::types::DestroyWitness
   **/
  PalletAssetsDestroyWitness: {
    accounts: 'Compact<u32>',
    sufficients: 'Compact<u32>',
    approvals: 'Compact<u32>'
  },
  /**
   * Lookup284: pallet_account_manager::pallet::Call<T>
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
   * Lookup285: t3rn_primitives::claimable::BenefitSource
   **/
  T3rnPrimitivesClaimableBenefitSource: {
    _enum: ['BootstrapPool', 'Inflation', 'TrafficFees', 'TrafficRewards', 'Unsettled', 'SlashTreasury']
  },
  /**
   * Lookup286: t3rn_primitives::claimable::CircuitRole
   **/
  T3rnPrimitivesClaimableCircuitRole: {
    _enum: ['Ambassador', 'Executor', 'Attester', 'Staker', 'Collator', 'ContractAuthor', 'Relayer', 'Requester', 'Local']
  },
  /**
   * Lookup287: t3rn_primitives::account_manager::Outcome
   **/
  T3rnPrimitivesAccountManagerOutcome: {
    _enum: ['UnexpectedFailure', 'Revert', 'Commit', 'Slash']
  },
  /**
   * Lookup288: pallet_treasury::pallet::Call<T, I>
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
   * Lookup293: pallet_authorship::pallet::Call<T>
   **/
  PalletAuthorshipCall: {
    _enum: {
      set_uncles: {
        newUncles: 'Vec<SpRuntimeHeader>'
      }
    }
  },
  /**
   * Lookup295: sp_runtime::generic::header::Header<Number, sp_runtime::traits::BlakeTwo256>
   **/
  SpRuntimeHeader: {
    parentHash: 'H256',
    number: 'Compact<u32>',
    stateRoot: 'H256',
    extrinsicsRoot: 'H256',
    digest: 'SpRuntimeDigest'
  },
  /**
   * Lookup296: sp_runtime::traits::BlakeTwo256
   **/
  SpRuntimeBlakeTwo256: 'Null',
  /**
   * Lookup297: pallet_collator_selection::pallet::Call<T>
   **/
  PalletCollatorSelectionCall: {
    _enum: {
      set_invulnerables: {
        _alias: {
          new_: 'new',
        },
        new_: 'Vec<AccountId32>',
      },
      set_desired_candidates: {
        max: 'u32',
      },
      set_candidacy_bond: {
        bond: 'u128',
      },
      register_as_candidate: 'Null',
      leave_intent: 'Null'
    }
  },
  /**
   * Lookup298: pallet_session::pallet::Call<T>
   **/
  PalletSessionCall: {
    _enum: {
      set_keys: {
        _alias: {
          keys_: 'keys',
        },
        keys_: 'T0rnParachainRuntimeParachainConfigSessionKeys',
        proof: 'Bytes',
      },
      purge_keys: 'Null'
    }
  },
  /**
   * Lookup299: t0rn_parachain_runtime::parachain_config::SessionKeys
   **/
  T0rnParachainRuntimeParachainConfigSessionKeys: {
    aura: 'SpConsensusAuraSr25519AppSr25519Public'
  },
  /**
   * Lookup300: sp_consensus_aura::sr25519::app_sr25519::Public
   **/
  SpConsensusAuraSr25519AppSr25519Public: 'SpCoreSr25519Public',
  /**
   * Lookup301: sp_core::sr25519::Public
   **/
  SpCoreSr25519Public: '[u8;32]',
  /**
   * Lookup302: cumulus_pallet_xcmp_queue::pallet::Call<T>
   **/
  CumulusPalletXcmpQueueCall: {
    _enum: {
      service_overweight: {
        index: 'u64',
        weightLimit: 'u64',
      },
      suspend_xcm_execution: 'Null',
      resume_xcm_execution: 'Null',
      update_suspend_threshold: {
        _alias: {
          new_: 'new',
        },
        new_: 'u32',
      },
      update_drop_threshold: {
        _alias: {
          new_: 'new',
        },
        new_: 'u32',
      },
      update_resume_threshold: {
        _alias: {
          new_: 'new',
        },
        new_: 'u32',
      },
      update_threshold_weight: {
        _alias: {
          new_: 'new',
        },
        new_: 'u64',
      },
      update_weight_restrict_decay: {
        _alias: {
          new_: 'new',
        },
        new_: 'u64',
      },
      update_xcmp_max_individual_weight: {
        _alias: {
          new_: 'new',
        },
        new_: 'u64'
      }
    }
  },
  /**
   * Lookup303: pallet_xcm::pallet::Call<T>
   **/
  PalletXcmCall: {
    _enum: {
      send: {
        dest: 'XcmVersionedMultiLocation',
        message: 'XcmVersionedXcm',
      },
      teleport_assets: {
        dest: 'XcmVersionedMultiLocation',
        beneficiary: 'XcmVersionedMultiLocation',
        assets: 'XcmVersionedMultiAssets',
        feeAssetItem: 'u32',
      },
      reserve_transfer_assets: {
        dest: 'XcmVersionedMultiLocation',
        beneficiary: 'XcmVersionedMultiLocation',
        assets: 'XcmVersionedMultiAssets',
        feeAssetItem: 'u32',
      },
      execute: {
        message: 'XcmVersionedXcm',
        maxWeight: 'u64',
      },
      force_xcm_version: {
        location: 'XcmV1MultiLocation',
        xcmVersion: 'u32',
      },
      force_default_xcm_version: {
        maybeXcmVersion: 'Option<u32>',
      },
      force_subscribe_version_notify: {
        location: 'XcmVersionedMultiLocation',
      },
      force_unsubscribe_version_notify: {
        location: 'XcmVersionedMultiLocation',
      },
      limited_reserve_transfer_assets: {
        dest: 'XcmVersionedMultiLocation',
        beneficiary: 'XcmVersionedMultiLocation',
        assets: 'XcmVersionedMultiAssets',
        feeAssetItem: 'u32',
        weightLimit: 'XcmV2WeightLimit',
      },
      limited_teleport_assets: {
        dest: 'XcmVersionedMultiLocation',
        beneficiary: 'XcmVersionedMultiLocation',
        assets: 'XcmVersionedMultiAssets',
        feeAssetItem: 'u32',
        weightLimit: 'XcmV2WeightLimit'
      }
    }
  },
  /**
   * Lookup304: xcm::VersionedXcm<Call>
   **/
  XcmVersionedXcm: {
    _enum: {
      V0: 'XcmV0Xcm',
      V1: 'XcmV1Xcm',
      V2: 'XcmV2Xcm'
    }
  },
  /**
   * Lookup305: xcm::v0::Xcm<Call>
   **/
  XcmV0Xcm: {
    _enum: {
      WithdrawAsset: {
        assets: 'Vec<XcmV0MultiAsset>',
        effects: 'Vec<XcmV0Order>',
      },
      ReserveAssetDeposit: {
        assets: 'Vec<XcmV0MultiAsset>',
        effects: 'Vec<XcmV0Order>',
      },
      TeleportAsset: {
        assets: 'Vec<XcmV0MultiAsset>',
        effects: 'Vec<XcmV0Order>',
      },
      QueryResponse: {
        queryId: 'Compact<u64>',
        response: 'XcmV0Response',
      },
      TransferAsset: {
        assets: 'Vec<XcmV0MultiAsset>',
        dest: 'XcmV0MultiLocation',
      },
      TransferReserveAsset: {
        assets: 'Vec<XcmV0MultiAsset>',
        dest: 'XcmV0MultiLocation',
        effects: 'Vec<XcmV0Order>',
      },
      Transact: {
        originType: 'XcmV0OriginKind',
        requireWeightAtMost: 'u64',
        call: 'XcmDoubleEncoded',
      },
      HrmpNewChannelOpenRequest: {
        sender: 'Compact<u32>',
        maxMessageSize: 'Compact<u32>',
        maxCapacity: 'Compact<u32>',
      },
      HrmpChannelAccepted: {
        recipient: 'Compact<u32>',
      },
      HrmpChannelClosing: {
        initiator: 'Compact<u32>',
        sender: 'Compact<u32>',
        recipient: 'Compact<u32>',
      },
      RelayedFrom: {
        who: 'XcmV0MultiLocation',
        message: 'XcmV0Xcm'
      }
    }
  },
  /**
   * Lookup307: xcm::v0::order::Order<Call>
   **/
  XcmV0Order: {
    _enum: {
      Null: 'Null',
      DepositAsset: {
        assets: 'Vec<XcmV0MultiAsset>',
        dest: 'XcmV0MultiLocation',
      },
      DepositReserveAsset: {
        assets: 'Vec<XcmV0MultiAsset>',
        dest: 'XcmV0MultiLocation',
        effects: 'Vec<XcmV0Order>',
      },
      ExchangeAsset: {
        give: 'Vec<XcmV0MultiAsset>',
        receive: 'Vec<XcmV0MultiAsset>',
      },
      InitiateReserveWithdraw: {
        assets: 'Vec<XcmV0MultiAsset>',
        reserve: 'XcmV0MultiLocation',
        effects: 'Vec<XcmV0Order>',
      },
      InitiateTeleport: {
        assets: 'Vec<XcmV0MultiAsset>',
        dest: 'XcmV0MultiLocation',
        effects: 'Vec<XcmV0Order>',
      },
      QueryHolding: {
        queryId: 'Compact<u64>',
        dest: 'XcmV0MultiLocation',
        assets: 'Vec<XcmV0MultiAsset>',
      },
      BuyExecution: {
        fees: 'XcmV0MultiAsset',
        weight: 'u64',
        debt: 'u64',
        haltOnError: 'bool',
        xcm: 'Vec<XcmV0Xcm>'
      }
    }
  },
  /**
   * Lookup309: xcm::v0::Response
   **/
  XcmV0Response: {
    _enum: {
      Assets: 'Vec<XcmV0MultiAsset>'
    }
  },
  /**
   * Lookup310: xcm::v1::Xcm<Call>
   **/
  XcmV1Xcm: {
    _enum: {
      WithdrawAsset: {
        assets: 'XcmV1MultiassetMultiAssets',
        effects: 'Vec<XcmV1Order>',
      },
      ReserveAssetDeposited: {
        assets: 'XcmV1MultiassetMultiAssets',
        effects: 'Vec<XcmV1Order>',
      },
      ReceiveTeleportedAsset: {
        assets: 'XcmV1MultiassetMultiAssets',
        effects: 'Vec<XcmV1Order>',
      },
      QueryResponse: {
        queryId: 'Compact<u64>',
        response: 'XcmV1Response',
      },
      TransferAsset: {
        assets: 'XcmV1MultiassetMultiAssets',
        beneficiary: 'XcmV1MultiLocation',
      },
      TransferReserveAsset: {
        assets: 'XcmV1MultiassetMultiAssets',
        dest: 'XcmV1MultiLocation',
        effects: 'Vec<XcmV1Order>',
      },
      Transact: {
        originType: 'XcmV0OriginKind',
        requireWeightAtMost: 'u64',
        call: 'XcmDoubleEncoded',
      },
      HrmpNewChannelOpenRequest: {
        sender: 'Compact<u32>',
        maxMessageSize: 'Compact<u32>',
        maxCapacity: 'Compact<u32>',
      },
      HrmpChannelAccepted: {
        recipient: 'Compact<u32>',
      },
      HrmpChannelClosing: {
        initiator: 'Compact<u32>',
        sender: 'Compact<u32>',
        recipient: 'Compact<u32>',
      },
      RelayedFrom: {
        who: 'XcmV1MultilocationJunctions',
        message: 'XcmV1Xcm',
      },
      SubscribeVersion: {
        queryId: 'Compact<u64>',
        maxResponseWeight: 'Compact<u64>',
      },
      UnsubscribeVersion: 'Null'
    }
  },
  /**
   * Lookup312: xcm::v1::order::Order<Call>
   **/
  XcmV1Order: {
    _enum: {
      Noop: 'Null',
      DepositAsset: {
        assets: 'XcmV1MultiassetMultiAssetFilter',
        maxAssets: 'u32',
        beneficiary: 'XcmV1MultiLocation',
      },
      DepositReserveAsset: {
        assets: 'XcmV1MultiassetMultiAssetFilter',
        maxAssets: 'u32',
        dest: 'XcmV1MultiLocation',
        effects: 'Vec<XcmV1Order>',
      },
      ExchangeAsset: {
        give: 'XcmV1MultiassetMultiAssetFilter',
        receive: 'XcmV1MultiassetMultiAssets',
      },
      InitiateReserveWithdraw: {
        assets: 'XcmV1MultiassetMultiAssetFilter',
        reserve: 'XcmV1MultiLocation',
        effects: 'Vec<XcmV1Order>',
      },
      InitiateTeleport: {
        assets: 'XcmV1MultiassetMultiAssetFilter',
        dest: 'XcmV1MultiLocation',
        effects: 'Vec<XcmV1Order>',
      },
      QueryHolding: {
        queryId: 'Compact<u64>',
        dest: 'XcmV1MultiLocation',
        assets: 'XcmV1MultiassetMultiAssetFilter',
      },
      BuyExecution: {
        fees: 'XcmV1MultiAsset',
        weight: 'u64',
        debt: 'u64',
        haltOnError: 'bool',
        instructions: 'Vec<XcmV1Xcm>'
      }
    }
  },
  /**
   * Lookup314: xcm::v1::Response
   **/
  XcmV1Response: {
    _enum: {
      Assets: 'XcmV1MultiassetMultiAssets',
      Version: 'u32'
    }
  },
  /**
   * Lookup328: cumulus_pallet_dmp_queue::pallet::Call<T>
   **/
  CumulusPalletDmpQueueCall: {
    _enum: {
      service_overweight: {
        index: 'u64',
        weightLimit: 'u64'
      }
    }
  },
  /**
   * Lookup329: pallet_xbi_portal::pallet::Call<T>
   **/
  PalletXbiPortalCall: {
    _enum: {
      send: {
        kind: 'XpChannelExecutionType',
        msg: 'XpFormatXbiFormat',
      },
      receive: {
        msg: 'XpChannelMessage',
      },
      process_queue: 'Null'
    }
  },
  /**
   * Lookup330: xp_channel::ExecutionType
   **/
  XpChannelExecutionType: {
    _enum: ['Sync', 'Async']
  },
  /**
   * Lookup331: pallet_asset_registry::pallet::Call<T>
   **/
  PalletAssetRegistryCall: {
    _enum: {
      register: {
        location: 'XcmV1MultiLocation',
        id: 'u32',
      },
      register_info: {
        info: 'PalletAssetRegistryAssetInfo'
      }
    }
  },
  /**
   * Lookup332: pallet_asset_registry::AssetInfo<AssetId, sp_core::crypto::AccountId32, Balance>
   **/
  PalletAssetRegistryAssetInfo: {
    id: 'u32',
    capabilities: 'Vec<PalletAssetRegistryCapability>',
    location: 'XcmV1MultiLocation'
  },
  /**
   * Lookup334: pallet_asset_registry::Capability<sp_core::crypto::AccountId32, Balance>
   **/
  PalletAssetRegistryCapability: {
    _enum: {
      Teleport: 'Option<AccountId32>',
      Reserve: 'Option<AccountId32>',
      Payable: {
        feesPerWeight: 'Option<u128>'
      }
    }
  },
  /**
   * Lookup335: pallet_xdns::pallet::Call<T>
   **/
  PalletXdnsCall: {
    _enum: {
      reboot_self_gateway: {
        vendor: 'T3rnPrimitivesGatewayVendor',
      },
      purge_gateway_record: {
        requester: 'AccountId32',
        gatewayId: '[u8;4]',
      },
      unlink_token: {
        gatewayId: '[u8;4]',
        tokenId: 'u32',
      },
      purge_token_record: {
        tokenId: 'u32'
      }
    }
  },
  /**
   * Lookup336: pallet_attesters::pallet::Call<T>
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
      submit_attestation: {
        message: 'H256',
        signature: 'Bytes',
        target: '[u8;4]',
      },
      commit_batch: {
        target: '[u8;4]',
        targetInclusionProofEncoded: 'Bytes',
      },
      set_confirmation_cost: {
        target: '[u8;4]',
        cost: 'u128',
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
   * Lookup339: pallet_rewards::pallet::Call<T>
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
   * Lookup341: pallet_contracts_registry::pallet::Call<T>
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
   * Lookup342: t3rn_primitives::contracts_registry::RegistryContract<primitive_types::H256, sp_core::crypto::AccountId32, BalanceOf, BlockNumber>
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
   * Lookup343: t3rn_primitives::contracts_registry::AuthorInfo<sp_core::crypto::AccountId32, BalanceOf>
   **/
  T3rnPrimitivesContractsRegistryAuthorInfo: {
    account: 'AccountId32',
    feesPerSingleUse: 'Option<u128>'
  },
  /**
   * Lookup345: t3rn_types::gateway::ContractActionDesc<primitive_types::H256, TargetId, sp_core::crypto::AccountId32>
   **/
  T3rnTypesGatewayContractActionDesc: {
    actionId: 'H256',
    targetId: 'Option<[u8;4]>',
    to: 'Option<AccountId32>'
  },
  /**
   * Lookup348: t3rn_primitives::storage::RawAliveContractInfo<primitive_types::H256, Balance, BlockNumber>
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
   * Lookup350: t3rn_primitives::contract_metadata::ContractMetadata
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
   * Lookup351: pallet_circuit::pallet::Call<T>
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
      on_extrinsic_trigger: {
        sideEffects: 'Vec<T3rnTypesSfxSideEffect>',
        speedMode: 'T3rnPrimitivesSpeedMode',
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
   * Lookup352: t3rn_primitives::SpeedMode
   **/
  T3rnPrimitivesSpeedMode: {
    _enum: ['Fast', 'Rational', 'Finalized']
  },
  /**
   * Lookup353: pallet_3vm::pallet::Call<T>
   **/
  Pallet3vmCall: 'Null',
  /**
   * Lookup354: pallet_contracts::pallet::Call<T>
   **/
  PalletContractsCall: {
    _enum: {
      call: {
        dest: 'MultiAddress',
        value: 'Compact<u128>',
        gasLimit: 'Compact<u64>',
        storageDepositLimit: 'Option<Compact<u128>>',
        data: 'Bytes',
      },
      instantiate_with_code: {
        value: 'Compact<u128>',
        gasLimit: 'Compact<u64>',
        storageDepositLimit: 'Option<Compact<u128>>',
        code: 'Bytes',
        data: 'Bytes',
        salt: 'Bytes',
      },
      instantiate: {
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
      },
      remove_code: {
        codeHash: 'H256'
      }
    }
  },
  /**
   * Lookup356: pallet_evm::pallet::Call<T>
   **/
  PalletEvmCall: {
    _enum: {
      withdraw: {
        address: 'H160',
        value: 'u128',
      },
      call: {
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
        init: 'Bytes',
        value: 'U256',
        gasLimit: 'u64',
        maxFeePerGas: 'U256',
        maxPriorityFeePerGas: 'Option<U256>',
        nonce: 'Option<U256>',
        accessList: 'Vec<(H160,Vec<H256>)>',
      },
      create2: {
        init: 'Bytes',
        salt: 'H256',
        value: 'U256',
        gasLimit: 'u64',
        maxFeePerGas: 'U256',
        maxPriorityFeePerGas: 'Option<U256>',
        nonce: 'Option<U256>',
        accessList: 'Vec<(H160,Vec<H256>)>',
      },
      claim: 'Null'
    }
  },
  /**
   * Lookup357: pallet_portal::pallet::Call<T>
   **/
  PalletPortalCall: {
    _enum: {
      register_gateway: {
        gatewayId: '[u8;4]',
        tokenId: '[u8;4]',
        verificationVendor: 'T3rnPrimitivesGatewayVendor',
        executionVendor: 'T3rnPrimitivesExecutionVendor',
        codec: 'T3rnAbiRecodeCodec',
        registrant: 'Option<AccountId32>',
        escrowAccount: 'Option<AccountId32>',
        allowedSideEffects: 'Vec<([u8;4],Option<u8>)>',
        tokenProps: 'T3rnPrimitivesTokenInfo',
        encodedRegistrationData: 'Bytes'
      }
    }
  },
  /**
   * Lookup358: t3rn_abi::recode::Codec
   **/
  T3rnAbiRecodeCodec: {
    _enum: ['Scale', 'Rlp']
  },
  /**
   * Lookup362: t3rn_primitives::TokenInfo
   **/
  T3rnPrimitivesTokenInfo: {
    _enum: {
      Substrate: 'T3rnPrimitivesSubstrateToken',
      Ethereum: 'T3rnPrimitivesEthereumToken'
    }
  },
  /**
   * Lookup363: t3rn_primitives::SubstrateToken
   **/
  T3rnPrimitivesSubstrateToken: {
    id: 'u32',
    symbol: 'Bytes',
    decimals: 'u8'
  },
  /**
   * Lookup364: t3rn_primitives::EthereumToken
   **/
  T3rnPrimitivesEthereumToken: {
    symbol: 'Bytes',
    decimals: 'u8',
    address: 'Option<[u8;20]>'
  },
  /**
   * Lookup365: pallet_grandpa_finality_verifier::pallet::Call<T, I>
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
   * Lookup366: pallet_grandpa_finality_verifier::bridges::header_chain::justification::GrandpaJustification<sp_runtime::generic::header::Header<Number, sp_runtime::traits::BlakeTwo256>>
   **/
  PalletGrandpaFinalityVerifierBridgesHeaderChainJustificationGrandpaJustification: {
    round: 'u64',
    commit: 'FinalityGrandpaCommit',
    votesAncestries: 'Vec<SpRuntimeHeader>'
  },
  /**
   * Lookup367: finality_grandpa::Commit<primitive_types::H256, N, sp_finality_grandpa::app::Signature, sp_finality_grandpa::app::Public>
   **/
  FinalityGrandpaCommit: {
    targetHash: 'H256',
    targetNumber: 'u32',
    precommits: 'Vec<FinalityGrandpaSignedPrecommit>'
  },
  /**
   * Lookup368: sp_finality_grandpa::app::Signature
   **/
  SpFinalityGrandpaAppSignature: 'SpCoreEd25519Signature',
  /**
   * Lookup369: sp_core::ed25519::Signature
   **/
  SpCoreEd25519Signature: '[u8;64]',
  /**
   * Lookup371: sp_finality_grandpa::app::Public
   **/
  SpFinalityGrandpaAppPublic: 'SpCoreEd25519Public',
  /**
   * Lookup372: sp_core::ed25519::Public
   **/
  SpCoreEd25519Public: '[u8;32]',
  /**
   * Lookup374: finality_grandpa::SignedPrecommit<primitive_types::H256, N, sp_finality_grandpa::app::Signature, sp_finality_grandpa::app::Public>
   **/
  FinalityGrandpaSignedPrecommit: {
    precommit: 'FinalityGrandpaPrecommit',
    signature: 'SpFinalityGrandpaAppSignature',
    id: 'SpFinalityGrandpaAppPublic'
  },
  /**
   * Lookup375: finality_grandpa::Precommit<primitive_types::H256, N>
   **/
  FinalityGrandpaPrecommit: {
    targetHash: 'H256',
    targetNumber: 'u32'
  },
  /**
   * Lookup378: pallet_eth2_finality_verifier::pallet::Call<T>
   **/
  PalletEth2FinalityVerifierCall: {
    _enum: {
      submit_epoch: {
        update: 'PalletEth2FinalityVerifierEpochUpdate',
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
        submissionTargetHeight: 'Option<u32>',
      },
      verify_event_inclusion: {
        proof: 'PalletEth2FinalityVerifierEthereumEventInclusionProof',
        submissionTargetHeight: 'Option<u32>'
      }
    }
  },
  /**
   * Lookup379: pallet_eth2_finality_verifier::types::EpochUpdate
   **/
  PalletEth2FinalityVerifierEpochUpdate: {
    attestedBeaconHeader: 'PalletEth2FinalityVerifierBeaconBlockHeader',
    signature: '[u8;96]',
    signerBits: 'Vec<bool>',
    justifiedProof: 'PalletEth2FinalityVerifierMerkleProof',
    finalizedProof: 'PalletEth2FinalityVerifierMerkleProof',
    executionHeader: 'PalletEth2FinalityVerifierExecutionHeader',
    executionProof: 'PalletEth2FinalityVerifierMerkleProof',
    executionRange: 'Vec<PalletEth2FinalityVerifierExecutionHeader>'
  },
  /**
   * Lookup380: pallet_eth2_finality_verifier::types::BeaconBlockHeader
   **/
  PalletEth2FinalityVerifierBeaconBlockHeader: {
    slot: 'u64',
    proposerIndex: 'u64',
    parentRoot: '[u8;32]',
    stateRoot: '[u8;32]',
    bodyRoot: '[u8;32]'
  },
  /**
   * Lookup383: pallet_eth2_finality_verifier::types::MerkleProof
   **/
  PalletEth2FinalityVerifierMerkleProof: {
    gIndex: 'u64',
    witness: 'Vec<[u8;32]>'
  },
  /**
   * Lookup385: pallet_eth2_finality_verifier::types::ExecutionHeader
   **/
  PalletEth2FinalityVerifierExecutionHeader: {
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
   * Lookup386: ethbloom::Bloom
   **/
  EthbloomBloom: '[u8;256]',
  /**
   * Lookup390: pallet_eth2_finality_verifier::types::SyncCommittee
   **/
  PalletEth2FinalityVerifierSyncCommittee: {
    pubs: 'Vec<[u8;48]>',
    aggr: '[u8;48]'
  },
  /**
   * Lookup393: pallet_eth2_finality_verifier::types::EthereumReceiptInclusionProof
   **/
  PalletEth2FinalityVerifierEthereumReceiptInclusionProof: {
    blockNumber: 'u64',
    witness: 'Vec<Bytes>',
    index: 'Bytes'
  },
  /**
   * Lookup394: pallet_eth2_finality_verifier::types::EthereumEventInclusionProof
   **/
  PalletEth2FinalityVerifierEthereumEventInclusionProof: {
    blockNumber: 'u64',
    witness: 'Vec<Bytes>',
    index: 'Bytes',
    event: 'Bytes'
  },
  /**
   * Lookup395: pallet_maintenance_mode::pallet::Call<T>
   **/
  PalletMaintenanceModeCall: {
    _enum: ['enter_maintenance_mode', 'resume_normal_operation']
  },
  /**
   * Lookup396: pallet_sudo::pallet::Call<T>
   **/
  PalletSudoCall: {
    _enum: {
      sudo: {
        call: 'Call',
      },
      sudo_unchecked_weight: {
        call: 'Call',
        weight: 'u64',
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
   * Lookup397: pallet_scheduler::pallet::Error<T>
   **/
  PalletSchedulerError: {
    _enum: ['FailedToSchedule', 'NotFound', 'TargetBlockNumberInPast', 'RescheduleNoChange']
  },
  /**
   * Lookup398: pallet_utility::pallet::Error<T>
   **/
  PalletUtilityError: {
    _enum: ['TooManyCalls']
  },
  /**
   * Lookup399: pallet_identity::types::Registration<Balance, MaxJudgements, MaxAdditionalFields>
   **/
  PalletIdentityRegistration: {
    judgements: 'Vec<(u32,PalletIdentityJudgement)>',
    deposit: 'u128',
    info: 'PalletIdentityIdentityInfo'
  },
  /**
   * Lookup407: pallet_identity::types::RegistrarInfo<Balance, sp_core::crypto::AccountId32>
   **/
  PalletIdentityRegistrarInfo: {
    account: 'AccountId32',
    fee: 'u128',
    fields: 'PalletIdentityBitFlags'
  },
  /**
   * Lookup409: pallet_identity::pallet::Error<T>
   **/
  PalletIdentityError: {
    _enum: ['TooManySubAccounts', 'NotFound', 'NotNamed', 'EmptyIndex', 'FeeChanged', 'NoIdentity', 'StickyJudgement', 'JudgementGiven', 'InvalidJudgement', 'InvalidIndex', 'InvalidTarget', 'TooManyFields', 'TooManyRegistrars', 'AlreadyClaimed', 'NotSub', 'NotOwned']
  },
  /**
   * Lookup412: pallet_balances::BalanceLock<Balance>
   **/
  PalletBalancesBalanceLock: {
    id: '[u8;8]',
    amount: 'u128',
    reasons: 'PalletBalancesReasons'
  },
  /**
   * Lookup413: pallet_balances::Reasons
   **/
  PalletBalancesReasons: {
    _enum: ['Fee', 'Misc', 'All']
  },
  /**
   * Lookup416: pallet_balances::ReserveData<ReserveIdentifier, Balance>
   **/
  PalletBalancesReserveData: {
    id: '[u8;8]',
    amount: 'u128'
  },
  /**
   * Lookup418: pallet_balances::Releases
   **/
  PalletBalancesReleases: {
    _enum: ['V1_0_0', 'V2_0_0']
  },
  /**
   * Lookup419: pallet_balances::pallet::Error<T, I>
   **/
  PalletBalancesError: {
    _enum: ['VestingBalance', 'LiquidityRestrictions', 'InsufficientBalance', 'ExistentialDeposit', 'KeepAlive', 'ExistingVestingSchedule', 'DeadAccount', 'TooManyReserves']
  },
  /**
   * Lookup421: pallet_transaction_payment::Releases
   **/
  PalletTransactionPaymentReleases: {
    _enum: ['V1Ancient', 'V2']
  },
  /**
   * Lookup422: pallet_assets::types::AssetDetails<Balance, sp_core::crypto::AccountId32, DepositBalance>
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
    isFrozen: 'bool'
  },
  /**
   * Lookup424: pallet_assets::types::AssetAccount<Balance, DepositBalance, Extra>
   **/
  PalletAssetsAssetAccount: {
    balance: 'u128',
    isFrozen: 'bool',
    reason: 'PalletAssetsExistenceReason',
    extra: 'Null'
  },
  /**
   * Lookup425: pallet_assets::types::ExistenceReason<Balance>
   **/
  PalletAssetsExistenceReason: {
    _enum: {
      Consumer: 'Null',
      Sufficient: 'Null',
      DepositHeld: 'u128',
      DepositRefunded: 'Null'
    }
  },
  /**
   * Lookup427: pallet_assets::types::Approval<Balance, DepositBalance>
   **/
  PalletAssetsApproval: {
    amount: 'u128',
    deposit: 'u128'
  },
  /**
   * Lookup428: pallet_assets::types::AssetMetadata<DepositBalance, sp_runtime::bounded::bounded_vec::BoundedVec<T, S>>
   **/
  PalletAssetsAssetMetadata: {
    deposit: 'u128',
    name: 'Bytes',
    symbol: 'Bytes',
    decimals: 'u8',
    isFrozen: 'bool'
  },
  /**
   * Lookup430: pallet_assets::pallet::Error<T, I>
   **/
  PalletAssetsError: {
    _enum: ['BalanceLow', 'NoAccount', 'NoPermission', 'Unknown', 'Frozen', 'InUse', 'BadWitness', 'MinBalanceZero', 'NoProvider', 'BadMetadata', 'Unapproved', 'WouldDie', 'AlreadyExists', 'NoDeposit', 'WouldBurn']
  },
  /**
   * Lookup431: t3rn_primitives::account_manager::RequestCharge<sp_core::crypto::AccountId32, Balance, AssetId>
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
   * Lookup433: t3rn_primitives::common::RoundInfo<BlockNumber>
   **/
  T3rnPrimitivesCommonRoundInfo: {
    index: 'u32',
    head: 'u32',
    term: 'u32'
  },
  /**
   * Lookup434: t3rn_primitives::account_manager::Settlement<sp_core::crypto::AccountId32, Balance>
   **/
  T3rnPrimitivesAccountManagerSettlement: {
    requester: 'AccountId32',
    recipient: 'AccountId32',
    settlementAmount: 'u128',
    outcome: 'T3rnPrimitivesAccountManagerOutcome',
    source: 'T3rnPrimitivesClaimableBenefitSource',
    role: 'T3rnPrimitivesClaimableCircuitRole'
  },
  /**
   * Lookup435: pallet_account_manager::pallet::Error<T>
   **/
  PalletAccountManagerError: {
    _enum: ['PendingChargeNotFoundAtCommit', 'PendingChargeNotFoundAtRefund', 'ExecutionNotRegistered', 'ExecutionAlreadyRegistered', 'SkippingEmptyCharges', 'NoChargeOfGivenIdRegistered', 'ChargeAlreadyRegistered', 'ChargeOrSettlementCalculationOverflow', 'ChargeOrSettlementActualFeesOutgrowReserved', 'DecodingExecutionIDFailed', 'TransferDepositFailedOldChargeNotFound', 'TransferDepositFailedToReleasePreviousCharge']
  },
  /**
   * Lookup436: pallet_treasury::Proposal<sp_core::crypto::AccountId32, Balance>
   **/
  PalletTreasuryProposal: {
    proposer: 'AccountId32',
    value: 'u128',
    beneficiary: 'AccountId32',
    bond: 'u128'
  },
  /**
   * Lookup440: frame_support::PalletId
   **/
  FrameSupportPalletId: '[u8;8]',
  /**
   * Lookup441: pallet_treasury::pallet::Error<T, I>
   **/
  PalletTreasuryError: {
    _enum: ['InsufficientProposersBalance', 'InvalidIndex', 'TooManyApprovals', 'InsufficientPermission', 'ProposalNotApproved']
  },
  /**
   * Lookup447: pallet_authorship::UncleEntryItem<BlockNumber, primitive_types::H256, sp_core::crypto::AccountId32>
   **/
  PalletAuthorshipUncleEntryItem: {
    _enum: {
      InclusionHeight: 'u32',
      Uncle: '(H256,Option<AccountId32>)'
    }
  },
  /**
   * Lookup449: pallet_authorship::pallet::Error<T>
   **/
  PalletAuthorshipError: {
    _enum: ['InvalidUncleParent', 'UnclesAlreadySet', 'TooManyUncles', 'GenesisUncle', 'TooHighUncle', 'UncleAlreadyIncluded', 'OldUncle']
  },
  /**
   * Lookup452: pallet_collator_selection::pallet::CandidateInfo<sp_core::crypto::AccountId32, Balance>
   **/
  PalletCollatorSelectionCandidateInfo: {
    who: 'AccountId32',
    deposit: 'u128'
  },
  /**
   * Lookup454: pallet_collator_selection::pallet::Error<T>
   **/
  PalletCollatorSelectionError: {
    _enum: ['TooManyCandidates', 'TooFewCandidates', 'Unknown', 'Permission', 'AlreadyCandidate', 'NotCandidate', 'TooManyInvulnerables', 'AlreadyInvulnerable', 'NoAssociatedValidatorId', 'ValidatorNotRegistered']
  },
  /**
   * Lookup458: sp_core::crypto::KeyTypeId
   **/
  SpCoreCryptoKeyTypeId: '[u8;4]',
  /**
   * Lookup459: pallet_session::pallet::Error<T>
   **/
  PalletSessionError: {
    _enum: ['InvalidProof', 'NoAssociatedValidatorId', 'DuplicatedKey', 'NoKeys', 'NoAccount']
  },
  /**
   * Lookup464: cumulus_pallet_xcmp_queue::InboundChannelDetails
   **/
  CumulusPalletXcmpQueueInboundChannelDetails: {
    sender: 'u32',
    state: 'CumulusPalletXcmpQueueInboundState',
    messageMetadata: 'Vec<(u32,PolkadotParachainPrimitivesXcmpMessageFormat)>'
  },
  /**
   * Lookup465: cumulus_pallet_xcmp_queue::InboundState
   **/
  CumulusPalletXcmpQueueInboundState: {
    _enum: ['Ok', 'Suspended']
  },
  /**
   * Lookup468: polkadot_parachain::primitives::XcmpMessageFormat
   **/
  PolkadotParachainPrimitivesXcmpMessageFormat: {
    _enum: ['ConcatenatedVersionedXcm', 'ConcatenatedEncodedBlob', 'Signals']
  },
  /**
   * Lookup471: cumulus_pallet_xcmp_queue::OutboundChannelDetails
   **/
  CumulusPalletXcmpQueueOutboundChannelDetails: {
    recipient: 'u32',
    state: 'CumulusPalletXcmpQueueOutboundState',
    signalsExist: 'bool',
    firstIndex: 'u16',
    lastIndex: 'u16'
  },
  /**
   * Lookup472: cumulus_pallet_xcmp_queue::OutboundState
   **/
  CumulusPalletXcmpQueueOutboundState: {
    _enum: ['Ok', 'Suspended']
  },
  /**
   * Lookup474: cumulus_pallet_xcmp_queue::QueueConfigData
   **/
  CumulusPalletXcmpQueueQueueConfigData: {
    suspendThreshold: 'u32',
    dropThreshold: 'u32',
    resumeThreshold: 'u32',
    thresholdWeight: 'u64',
    weightRestrictDecay: 'u64',
    xcmpMaxIndividualWeight: 'u64'
  },
  /**
   * Lookup476: cumulus_pallet_xcmp_queue::pallet::Error<T>
   **/
  CumulusPalletXcmpQueueError: {
    _enum: ['FailedToSend', 'BadXcmOrigin', 'BadXcm', 'BadOverweightIndex', 'WeightOverLimit']
  },
  /**
   * Lookup477: pallet_xcm::pallet::Error<T>
   **/
  PalletXcmError: {
    _enum: ['Unreachable', 'SendFailure', 'Filtered', 'UnweighableMessage', 'DestinationNotInvertible', 'Empty', 'CannotReanchor', 'TooManyAssets', 'InvalidOrigin', 'BadVersion', 'BadLocation', 'NoSubscription', 'AlreadySubscribed']
  },
  /**
   * Lookup478: cumulus_pallet_xcm::pallet::Error<T>
   **/
  CumulusPalletXcmError: 'Null',
  /**
   * Lookup479: cumulus_pallet_dmp_queue::ConfigData
   **/
  CumulusPalletDmpQueueConfigData: {
    maxIndividual: 'u64'
  },
  /**
   * Lookup480: cumulus_pallet_dmp_queue::PageIndexData
   **/
  CumulusPalletDmpQueuePageIndexData: {
    beginUsed: 'u32',
    endUsed: 'u32',
    overweightCount: 'u64'
  },
  /**
   * Lookup483: cumulus_pallet_dmp_queue::pallet::Error<T>
   **/
  CumulusPalletDmpQueueError: {
    _enum: ['Unknown', 'OverLimit']
  },
  /**
   * Lookup486: pallet_xbi_portal::pallet::Error<T>
   **/
  PalletXbiPortalError: {
    _enum: ['FailedToCastValue', 'FailedToCastAddress', 'FailedToCastHash', 'InstructionuctionNotAllowedHere', 'AlreadyCheckedIn', 'NotificationTimeoutDelivery', 'NotificationTimeoutExecution', 'CallbackUnsupported', 'EvmUnsupported', 'WasmUnsupported', 'CallNativeUnsupported', 'CallCustomUnsupported', 'TransferUnsupported', 'AssetsUnsupported', 'DefiUnsupported', 'ArithmeticErrorOverflow', 'TransferFailed', 'ResponseAlreadyStored']
  },
  /**
   * Lookup487: pallet_asset_registry::pallet::Error<T>
   **/
  PalletAssetRegistryError: {
    _enum: ['NotFound', 'LocationUnallowed', 'CapabilitiesNotPermitted', 'ShouldntExecuteMessage']
  },
  /**
   * Lookup488: t3rn_abi::sfx_abi::SFXAbi
   **/
  T3rnAbiSfxAbi: {
    argsNames: 'Vec<(Bytes,bool)>',
    maybePrefixMemo: 'Option<u8>',
    egressAbiDescriptors: 'T3rnAbiSfxAbiPerCodecAbiDescriptors',
    ingressAbiDescriptors: 'T3rnAbiSfxAbiPerCodecAbiDescriptors'
  },
  /**
   * Lookup491: t3rn_abi::sfx_abi::PerCodecAbiDescriptors
   **/
  T3rnAbiSfxAbiPerCodecAbiDescriptors: {
    forRlp: 'Bytes',
    forScale: 'Bytes'
  },
  /**
   * Lookup493: t3rn_primitives::xdns::GatewayRecord<sp_core::crypto::AccountId32>
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
   * Lookup495: t3rn_primitives::xdns::TokenRecord
   **/
  T3rnPrimitivesXdnsTokenRecord: {
    tokenId: 'u32',
    gatewayId: '[u8;4]',
    tokenProps: 'T3rnPrimitivesTokenInfo'
  },
  /**
   * Lookup499: t3rn_primitives::GatewayActivity<BlockNumber>
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
   * Lookup501: pallet_xdns::pallet::Error<T>
   **/
  PalletXdnsError: {
    _enum: ['GatewayRecordAlreadyExists', 'XdnsRecordNotFound', 'TokenRecordAlreadyExists', 'TokenRecordNotFoundInAssetsOverlay', 'GatewayRecordNotFound', 'SideEffectABIAlreadyExists', 'SideEffectABINotFound', 'NoParachainInfoFound', 'TokenExecutionVendorMismatch', 'GatewayNotActive']
  },
  /**
   * Lookup502: t3rn_primitives::attesters::AttesterInfo
   **/
  T3rnPrimitivesAttestersAttesterInfo: {
    keyEd: '[u8;32]',
    keyEc: '[u8;33]',
    keySr: '[u8;32]',
    commission: 'Percent',
    index: 'u32'
  },
  /**
   * Lookup508: pallet_attesters::pallet::Error<T>
   **/
  PalletAttestersError: {
    _enum: ['AttesterNotFound', 'ArithmeticOverflow', 'InvalidSignature', 'InvalidMessage', 'InvalidTargetInclusionProof', 'AlreadyRegistered', 'PublicKeyMissing', 'AttestationSignatureInvalid', 'AttestationDoubleSignAttempt', 'NotActiveSet', 'NotInCurrentCommittee', 'AttesterDidNotAgreeToNewTarget', 'NotRegistered', 'NoNominationFound', 'AlreadyNominated', 'NominatorNotEnoughBalance', 'NominatorBondTooSmall', 'AttesterBondTooSmall', 'MissingNominations', 'BatchHashMismatch', 'BatchNotFound', 'CollusionWithPermanentSlashDetected', 'BatchFoundWithUnsignableStatus', 'RejectingFromSlashedAttester', 'TargetAlreadyActive', 'TargetNotActive', 'XdnsTargetNotActive', 'XdnsGatewayDoesNotHaveEscrowAddressRegistered', 'SfxAlreadyRequested', 'AddAttesterAlreadyRequested', 'RemoveAttesterAlreadyRequested', 'NextCommitteeAlreadyRequested', 'BanAttesterAlreadyRequested', 'BatchAlreadyCommitted', 'CommitteeSizeTooLarge']
  },
  /**
   * Lookup512: pallet_rewards::pallet::TreasuryBalanceSheet<Balance>
   **/
  PalletRewardsTreasuryBalanceSheet: {
    treasury: 'u128',
    escrow: 'u128',
    fee: 'u128',
    slash: 'u128',
    parachain: 'u128'
  },
  /**
   * Lookup514: pallet_rewards::pallet::DistributionRecord<BlockNumber, Balance>
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
   * Lookup516: t3rn_primitives::claimable::ClaimableArtifacts<sp_core::crypto::AccountId32, Balance>
   **/
  T3rnPrimitivesClaimableClaimableArtifacts: {
    beneficiary: 'AccountId32',
    role: 'T3rnPrimitivesClaimableCircuitRole',
    totalRoundClaim: 'u128',
    benefitSource: 'T3rnPrimitivesClaimableBenefitSource'
  },
  /**
   * Lookup517: pallet_rewards::pallet::Error<T>
   **/
  PalletRewardsError: {
    _enum: ['DistributionPeriodNotElapsed', 'NoPendingClaims', 'ArithmeticOverflow', 'AttesterNotFound', 'TryIntoConversionU128ToBalanceFailed', 'Halted']
  },
  /**
   * Lookup518: pallet_contracts_registry::pallet::Error<T>
   **/
  PalletContractsRegistryError: {
    _enum: ['ContractAlreadyExists', 'UnknownContract']
  },
  /**
   * Lookup519: t3rn_primitives::circuit::types::XExecSignal<sp_core::crypto::AccountId32, BlockNumber>
   **/
  T3rnPrimitivesCircuitTypesXExecSignal: {
    requester: 'AccountId32',
    requesterNonce: 'u32',
    timeoutsAt: 'u32',
    speedMode: 'T3rnPrimitivesSpeedMode',
    delayStepsAt: 'Option<Vec<u32>>',
    status: 'T3rnPrimitivesCircuitTypesCircuitStatus',
    stepsCnt: '(u32,u32)'
  },
  /**
   * Lookup521: t3rn_primitives::circuit::types::CircuitStatus
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
   * Lookup522: t3rn_primitives::circuit::types::Cause
   **/
  T3rnPrimitivesCircuitTypesCause: {
    _enum: ['Timeout', 'IntentionalKill']
  },
  /**
   * Lookup523: t3rn_primitives::volatile::LocalState
   **/
  T3rnPrimitivesVolatileLocalState: {
    state: 'BTreeMap<[u8;32], Bytes>'
  },
  /**
   * Lookup529: t3rn_sdk_primitives::signal::ExecutionSignal<primitive_types::H256>
   **/
  T3rnSdkPrimitivesSignalExecutionSignal: {
    step: 'u32',
    kind: 'T3rnSdkPrimitivesSignalSignalKind',
    executionId: 'H256'
  },
  /**
   * Lookup531: pallet_circuit::pallet::Error<T>
   **/
  PalletCircuitError: {
    _enum: ['UpdateAttemptDoubleRevert', 'UpdateAttemptDoubleKill', 'UpdateStateTransitionDisallowed', 'UpdateForcedStateTransitionDisallowed', 'UpdateXtxTriggeredWithUnexpectedStatus', 'ConfirmationFailed', 'ApplyTriggeredWithUnexpectedStatus', 'BidderNotEnoughBalance', 'RequesterNotEnoughBalance', 'SanityAfterCreatingSFXDepositsFailed', 'ContractXtxKilledRunOutOfFunds', 'ChargingTransferFailed', 'ChargingTransferFailedAtPendingExecution', 'XtxChargeFailedRequesterBalanceTooLow', 'XtxChargeBondDepositFailedCantAccessBid', 'FinalizeSquareUpFailed', 'CriticalStateSquareUpCalledToFinishWithoutFsxConfirmed', 'RewardTransferFailed', 'RefundTransferFailed', 'SideEffectsValidationFailed', 'InsuranceBondNotRequired', 'BiddingInactive', 'BiddingRejectedBidBelowDust', 'BiddingRejectedBidTooHigh', 'BiddingRejectedInsuranceTooLow', 'BiddingRejectedBetterBidFound', 'BiddingRejectedFailedToDepositBidderBond', 'BiddingFailedExecutorsBalanceTooLowToReserve', 'InsuranceBondAlreadyDeposited', 'InvalidFTXStateEmptyBidForReadyXtx', 'InvalidFTXStateEmptyConfirmationForFinishedXtx', 'InvalidFTXStateUnassignedExecutorForReadySFX', 'InvalidFTXStateIncorrectExecutorForReadySFX', 'SetupFailed', 'SetupFailedXtxNotFound', 'SetupFailedXtxStorageArtifactsNotFound', 'SetupFailedIncorrectXtxStatus', 'SetupFailedDuplicatedXtx', 'SetupFailedEmptyXtx', 'SetupFailedXtxAlreadyFinished', 'SetupFailedXtxWasDroppedAtBidding', 'SetupFailedXtxReverted', 'SetupFailedXtxRevertedTimeout', 'XtxDoesNotExist', 'InvalidFSXBidStateLocated', 'EnactSideEffectsCanOnlyBeCalledWithMin1StepFinished', 'FatalXtxTimeoutXtxIdNotMatched', 'RelayEscrowedFailedNothingToConfirm', 'FatalCommitSideEffectWithoutConfirmationAttempt', 'FatalErroredCommitSideEffectConfirmationAttempt', 'FatalErroredRevertSideEffectConfirmationAttempt', 'FailedToHardenFullSideEffect', 'ApplyFailed', 'DeterminedForbiddenXtxStatus', 'SideEffectIsAlreadyScheduledToExecuteOverXBI', 'FSXNotFoundById', 'XtxNotFound', 'LocalSideEffectExecutionNotApplicable', 'LocalExecutionUnauthorized', 'OnLocalTriggerFailedToSetupXtx', 'UnauthorizedCancellation', 'FailedToConvertSFX2XBI', 'FailedToCheckInOverXBI', 'FailedToCreateXBIMetadataDueToWrongAccountConversion', 'FailedToConvertXBIResult2SFXConfirmation', 'FailedToEnterXBIPortal', 'FailedToExitXBIPortal', 'FailedToCommitFSX', 'XBIExitFailedOnSFXConfirmation', 'UnsupportedRole', 'InvalidLocalTrigger', 'SignalQueueFull', 'ArithmeticErrorOverflow', 'ArithmeticErrorUnderflow', 'ArithmeticErrorDivisionByZero']
  },
  /**
   * Lookup532: pallet_clock::pallet::Error<T>
   **/
  PalletClockError: 'Null',
  /**
   * Lookup534: pallet_3vm::pallet::Error<T>
   **/
  Pallet3vmError: {
    _enum: ['ExceededSignalBounceThreshold', 'CannotTriggerWithoutSideEffects', 'ContractNotFound', 'InvalidOrigin', 'CannotInstantiateContract', 'ContractCannotRemunerate', 'ContractCannotHaveStorage', 'ContractCannotGenerateSideEffects', 'InvalidPrecompilePointer', 'InvalidPrecompileArgs', 'InvalidArithmeticOverflow', 'DownstreamCircuit']
  },
  /**
   * Lookup535: pallet_contracts::wasm::PrefabWasmModule<T>
   **/
  PalletContractsWasmPrefabWasmModule: {
    instructionWeightsVersion: 'Compact<u32>',
    initial: 'Compact<u32>',
    maximum: 'Compact<u32>',
    code: 'Bytes',
    author: 'Option<T3rnPrimitivesContractsRegistryAuthorInfo>',
    kind: 'T3rnPrimitivesContractMetadataContractType'
  },
  /**
   * Lookup537: pallet_contracts::wasm::OwnerInfo<T>
   **/
  PalletContractsWasmOwnerInfo: {
    owner: 'AccountId32',
    deposit: 'Compact<u128>',
    refcount: 'Compact<u64>'
  },
  /**
   * Lookup538: pallet_contracts::storage::RawContractInfo<primitive_types::H256, Balance>
   **/
  PalletContractsStorageRawContractInfo: {
    trieId: 'Bytes',
    codeHash: 'H256',
    storageDeposit: 'u128'
  },
  /**
   * Lookup540: pallet_contracts::storage::DeletedContract
   **/
  PalletContractsStorageDeletedContract: {
    trieId: 'Bytes'
  },
  /**
   * Lookup541: pallet_contracts::schedule::Schedule<T>
   **/
  PalletContractsSchedule: {
    limits: 'PalletContractsScheduleLimits',
    instructionWeights: 'PalletContractsScheduleInstructionWeights',
    hostFnWeights: 'PalletContractsScheduleHostFnWeights'
  },
  /**
   * Lookup542: pallet_contracts::schedule::Limits
   **/
  PalletContractsScheduleLimits: {
    eventTopics: 'u32',
    stackHeight: 'Option<u32>',
    globals: 'u32',
    parameters: 'u32',
    memoryPages: 'u32',
    tableSize: 'u32',
    brTableSize: 'u32',
    subjectLen: 'u32',
    callDepth: 'u32',
    payloadLen: 'u32',
    codeLen: 'u32'
  },
  /**
   * Lookup543: pallet_contracts::schedule::InstructionWeights<T>
   **/
  PalletContractsScheduleInstructionWeights: {
    _alias: {
      r_if: 'r#if'
    },
    version: 'u32',
    i64const: 'u32',
    i64load: 'u32',
    i64store: 'u32',
    select: 'u32',
    r_if: 'u32',
    br: 'u32',
    brIf: 'u32',
    brTable: 'u32',
    brTablePerEntry: 'u32',
    call: 'u32',
    callIndirect: 'u32',
    callIndirectPerParam: 'u32',
    localGet: 'u32',
    localSet: 'u32',
    localTee: 'u32',
    globalGet: 'u32',
    globalSet: 'u32',
    memoryCurrent: 'u32',
    memoryGrow: 'u32',
    i64clz: 'u32',
    i64ctz: 'u32',
    i64popcnt: 'u32',
    i64eqz: 'u32',
    i64extendsi32: 'u32',
    i64extendui32: 'u32',
    i32wrapi64: 'u32',
    i64eq: 'u32',
    i64ne: 'u32',
    i64lts: 'u32',
    i64ltu: 'u32',
    i64gts: 'u32',
    i64gtu: 'u32',
    i64les: 'u32',
    i64leu: 'u32',
    i64ges: 'u32',
    i64geu: 'u32',
    i64add: 'u32',
    i64sub: 'u32',
    i64mul: 'u32',
    i64divs: 'u32',
    i64divu: 'u32',
    i64rems: 'u32',
    i64remu: 'u32',
    i64and: 'u32',
    i64or: 'u32',
    i64xor: 'u32',
    i64shl: 'u32',
    i64shrs: 'u32',
    i64shru: 'u32',
    i64rotl: 'u32',
    i64rotr: 'u32'
  },
  /**
   * Lookup544: pallet_contracts::schedule::HostFnWeights<T>
   **/
  PalletContractsScheduleHostFnWeights: {
    _alias: {
      r_return: 'r#return'
    },
    caller: 'u64',
    isContract: 'u64',
    codeHash: 'u64',
    ownCodeHash: 'u64',
    callerIsOrigin: 'u64',
    address: 'u64',
    gasLeft: 'u64',
    balance: 'u64',
    valueTransferred: 'u64',
    minimumBalance: 'u64',
    blockNumber: 'u64',
    now: 'u64',
    weightToFee: 'u64',
    gas: 'u64',
    input: 'u64',
    inputPerByte: 'u64',
    r_return: 'u64',
    returnPerByte: 'u64',
    terminate: 'u64',
    random: 'u64',
    depositEvent: 'u64',
    depositEventPerTopic: 'u64',
    depositEventPerByte: 'u64',
    debugMessage: 'u64',
    setStorage: 'u64',
    setStoragePerNewByte: 'u64',
    setStoragePerOldByte: 'u64',
    setCodeHash: 'u64',
    clearStorage: 'u64',
    clearStoragePerByte: 'u64',
    containsStorage: 'u64',
    containsStoragePerByte: 'u64',
    getStorage: 'u64',
    getStoragePerByte: 'u64',
    takeStorage: 'u64',
    takeStoragePerByte: 'u64',
    transfer: 'u64',
    call: 'u64',
    delegateCall: 'u64',
    callTransferSurcharge: 'u64',
    callPerClonedByte: 'u64',
    instantiate: 'u64',
    instantiateTransferSurcharge: 'u64',
    instantiatePerSaltByte: 'u64',
    hashSha2256: 'u64',
    hashSha2256PerByte: 'u64',
    hashKeccak256: 'u64',
    hashKeccak256PerByte: 'u64',
    hashBlake2256: 'u64',
    hashBlake2256PerByte: 'u64',
    hashBlake2128: 'u64',
    hashBlake2128PerByte: 'u64',
    ecdsaRecover: 'u64'
  },
  /**
   * Lookup545: pallet_contracts::pallet::Error<T>
   **/
  PalletContractsError: {
    _enum: ['InvalidScheduleVersion', 'InvalidCallFlags', 'OutOfGas', 'OutputBufferTooSmall', 'TransferFailed', 'MaxCallDepthReached', 'ContractNotFound', 'CodeTooLarge', 'CodeNotFound', 'OutOfBounds', 'DecodingFailed', 'ContractTrapped', 'ValueTooLarge', 'TerminatedWhileReentrant', 'InputForwarded', 'RandomSubjectTooLong', 'TooManyTopics', 'DuplicateTopics', 'NoChainExtension', 'DeletionQueueFull', 'DuplicateContract', 'TerminatedInConstructor', 'DebugMessageInvalidUTF8', 'ReentranceDenied', 'StorageDepositNotEnoughFunds', 'StorageDepositLimitExhausted', 'CodeInUse', 'ContractReverted', 'CodeRejected', 'NoStateReturned']
  },
  /**
   * Lookup547: pallet_evm::ThreeVmInfo<T>
   **/
  PalletEvmThreeVmInfo: {
    author: 'T3rnPrimitivesContractsRegistryAuthorInfo',
    kind: 'T3rnPrimitivesContractMetadataContractType'
  },
  /**
   * Lookup548: pallet_evm::pallet::Error<T>
   **/
  PalletEvmError: {
    _enum: ['BalanceLow', 'FeeOverflow', 'PaymentOverflow', 'WithdrawFailed', 'GasPriceTooLow', 'InvalidNonce', 'InvalidRegistryHash', 'RemunerateAuthor', 'ExecutedFailed', 'CreatedFailed']
  },
  /**
   * Lookup549: pallet_portal::pallet::Error<T>
   **/
  PalletPortalError: {
    _enum: ['XdnsRecordCreationFailed', 'UnimplementedGatewayVendor', 'LightClientNotFoundByVendor', 'RegistrationError', 'GatewayVendorNotFound', 'SetOwnerError', 'SetOperationalError', 'SubmitHeaderError', 'NoGatewayHeightAvailable', 'SideEffectConfirmationFailed', 'SFXRecodeError']
  },
  /**
   * Lookup550: pallet_grandpa_finality_verifier::bridges::header_chain::AuthoritySet
   **/
  PalletGrandpaFinalityVerifierBridgesHeaderChainAuthoritySet: {
    authorities: 'Vec<(SpFinalityGrandpaAppPublic,u64)>',
    setId: 'u64'
  },
  /**
   * Lookup553: pallet_grandpa_finality_verifier::types::ParachainRegistrationData
   **/
  PalletGrandpaFinalityVerifierParachainRegistrationData: {
    relayGatewayId: '[u8;4]',
    id: 'u32'
  },
  /**
   * Lookup554: pallet_grandpa_finality_verifier::pallet::Error<T, I>
   **/
  PalletGrandpaFinalityVerifierError: {
    _enum: ['EmptyRangeSubmitted', 'RangeToLarge', 'NoFinalizedHeader', 'InvalidAuthoritySet', 'InvalidGrandpaJustification', 'InvalidRangeLinkage', 'InvalidJustificationLinkage', 'ParachainEntryNotFound', 'StorageRootNotFound', 'InclusionDataDecodeError', 'InvalidStorageProof', 'EventNotIncluded', 'HeaderDecodingError', 'HeaderDataDecodingError', 'StorageRootMismatch', 'UnknownHeader', 'EventDecodingFailed', 'UnkownSideEffect', 'UnsupportedScheduledChange', 'Halted', 'BlockHeightConversionError']
  },
  /**
   * Lookup557: pallet_eth2_finality_verifier::types::Checkpoint
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
   * Lookup558: pallet_eth2_finality_verifier::types::BeaconCheckpoint
   **/
  PalletEth2FinalityVerifierBeaconCheckpoint: {
    epoch: 'u64',
    root: '[u8;32]'
  },
  /**
   * Lookup559: pallet_eth2_finality_verifier::types::ExecutionCheckpoint
   **/
  PalletEth2FinalityVerifierExecutionCheckpoint: {
    height: 'u64',
    root: '[u8;32]'
  },
  /**
   * Lookup560: pallet_eth2_finality_verifier::pallet::Error<T>
   **/
  PalletEth2FinalityVerifierError: {
    _enum: ['Halted', 'AlreadyInitialized', 'InvalidInitializationData', 'SSZForkDataHashTreeRootFailed', 'SSZSigningDataHashTreeRootFailed', 'BLSPubkeyAggregationFaild', 'InvalidBLSPublicKeyUsedForVerification', 'InvalidInclusionProof', 'ForkNotDetected', 'ValidSyncCommitteeNotAvailable', 'SubmittedHeaderToOld', 'InvalidBLSSignature', 'InvalidMerkleProof', 'BeaconHeaderHashTreeRootFailed', 'BeaconHeaderNotFound', 'BeaconHeaderNotFinalized', 'ExecutionHeaderHashTreeRootFailed', 'InvalidExecutionRangeLinkage', 'InvalidExecutionRange', 'SyncCommitteeParticipantsNotSupermajority', 'SyncCommitteeInvalid', 'NotPeriodsFirstEpoch', 'InvalidCheckpoint', 'ExecutionHeaderNotFound', 'EventNotInReceipt', 'InvalidEncodedEpochUpdate', 'InvalidSyncCommitteePeriod', 'MathError', 'CurrentSyncCommitteePeriodNotAvailable', 'BeaconCheckpointHashTreeRootFailed', 'InvalidFork']
  },
  /**
   * Lookup561: pallet_maintenance_mode::pallet::Error<T>
   **/
  PalletMaintenanceModeError: {
    _enum: ['AlreadyInMaintenanceMode', 'NotInMaintenanceMode']
  },
  /**
   * Lookup562: pallet_sudo::pallet::Error<T>
   **/
  PalletSudoError: {
    _enum: ['RequireSudo']
  },
  /**
   * Lookup564: sp_runtime::MultiSignature
   **/
  SpRuntimeMultiSignature: {
    _enum: {
      Ed25519: 'SpCoreEd25519Signature',
      Sr25519: 'SpCoreSr25519Signature',
      Ecdsa: 'SpCoreEcdsaSignature'
    }
  },
  /**
   * Lookup565: sp_core::sr25519::Signature
   **/
  SpCoreSr25519Signature: '[u8;64]',
  /**
   * Lookup566: sp_core::ecdsa::Signature
   **/
  SpCoreEcdsaSignature: '[u8;65]',
  /**
   * Lookup568: frame_system::extensions::check_non_zero_sender::CheckNonZeroSender<T>
   **/
  FrameSystemExtensionsCheckNonZeroSender: 'Null',
  /**
   * Lookup569: frame_system::extensions::check_spec_version::CheckSpecVersion<T>
   **/
  FrameSystemExtensionsCheckSpecVersion: 'Null',
  /**
   * Lookup570: frame_system::extensions::check_tx_version::CheckTxVersion<T>
   **/
  FrameSystemExtensionsCheckTxVersion: 'Null',
  /**
   * Lookup571: frame_system::extensions::check_genesis::CheckGenesis<T>
   **/
  FrameSystemExtensionsCheckGenesis: 'Null',
  /**
   * Lookup574: frame_system::extensions::check_nonce::CheckNonce<T>
   **/
  FrameSystemExtensionsCheckNonce: 'Compact<u32>',
  /**
   * Lookup575: frame_system::extensions::check_weight::CheckWeight<T>
   **/
  FrameSystemExtensionsCheckWeight: 'Null',
  /**
   * Lookup576: pallet_asset_tx_payment::ChargeAssetTxPayment<T>
   **/
  PalletAssetTxPaymentChargeAssetTxPayment: {
    tip: 'Compact<u128>',
    assetId: 'Option<u32>'
  },
  /**
   * Lookup577: t0rn_parachain_runtime::Runtime
   **/
  T0rnParachainRuntimeRuntime: 'Null'
};
