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
   * Lookup19: frame_system::EventRecord<t0rn_parachain_runtime::RuntimeEvent, primitive_types::H256>
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
   * Lookup30: cumulus_pallet_parachain_system::pallet::Event<T>
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
        weightUsed: 'SpWeightsWeightV2Weight',
        dmqHead: 'H256',
      },
      UpwardMessageSent: {
        messageHash: 'Option<[u8;32]>'
      }
    }
  },
  /**
   * Lookup32: pallet_preimage::pallet::Event<T>
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
   * Lookup33: pallet_scheduler::pallet::Event<T>
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
        id: 'Option<[u8;32]>',
        result: 'Result<Null, SpRuntimeDispatchError>',
      },
      CallUnavailable: {
        task: '(u32,u32)',
        id: 'Option<[u8;32]>',
      },
      PeriodicFailed: {
        task: '(u32,u32)',
        id: 'Option<[u8;32]>',
      },
      PermanentlyOverweight: {
        task: '(u32,u32)',
        id: 'Option<[u8;32]>'
      }
    }
  },
  /**
   * Lookup37: pallet_utility::pallet::Event
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
   * Lookup38: pallet_identity::pallet::Event<T>
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
   * Lookup39: pallet_balances::pallet::Event<T, I>
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
   * Lookup40: frame_support::traits::tokens::misc::BalanceStatus
   **/
  FrameSupportTokensMiscBalanceStatus: {
    _enum: ['Free', 'Reserved']
  },
  /**
   * Lookup41: pallet_transaction_payment::pallet::Event<T>
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
   * Lookup42: pallet_assets::pallet::Event<T, I>
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
   * Lookup44: pallet_account_manager::pallet::Event<T>
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
   * Lookup46: pallet_asset_tx_payment::pallet::Event<T>
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
   * Lookup48: pallet_treasury::pallet::Event<T, I>
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
   * Lookup53: pallet_collator_selection::pallet::Event<T>
   **/
  PalletCollatorSelectionEvent: {
    _enum: {
      NewInvulnerables: {
        invulnerables: 'Vec<AccountId32>',
      },
      InvulnerableAdded: {
        accountId: 'AccountId32',
      },
      InvulnerableRemoved: {
        accountId: 'AccountId32',
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
        accountId: 'AccountId32',
      },
      InvalidInvulnerableSkipped: {
        accountId: 'AccountId32'
      }
    }
  },
  /**
   * Lookup55: pallet_session::pallet::Event
   **/
  PalletSessionEvent: {
    _enum: {
      NewSession: {
        sessionIndex: 'u32'
      }
    }
  },
  /**
   * Lookup56: cumulus_pallet_xcmp_queue::pallet::Event<T>
   **/
  CumulusPalletXcmpQueueEvent: {
    _enum: {
      Success: {
        messageHash: '[u8;32]',
        messageId: '[u8;32]',
        weight: 'SpWeightsWeightV2Weight',
      },
      Fail: {
        messageHash: '[u8;32]',
        messageId: '[u8;32]',
        error: 'XcmV3TraitsError',
        weight: 'SpWeightsWeightV2Weight',
      },
      BadVersion: {
        messageHash: '[u8;32]',
      },
      BadFormat: {
        messageHash: '[u8;32]',
      },
      XcmpMessageSent: {
        messageHash: '[u8;32]',
      },
      OverweightEnqueued: {
        sender: 'u32',
        sentAt: 'u32',
        index: 'u64',
        required: 'SpWeightsWeightV2Weight',
      },
      OverweightServiced: {
        index: 'u64',
        used: 'SpWeightsWeightV2Weight'
      }
    }
  },
  /**
   * Lookup57: xcm::v3::traits::Error
   **/
  XcmV3TraitsError: {
    _enum: {
      Overflow: 'Null',
      Unimplemented: 'Null',
      UntrustedReserveLocation: 'Null',
      UntrustedTeleportLocation: 'Null',
      LocationFull: 'Null',
      LocationNotInvertible: 'Null',
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
      ExpectationFalse: 'Null',
      PalletNotFound: 'Null',
      NameMismatch: 'Null',
      VersionIncompatible: 'Null',
      HoldingWouldOverflow: 'Null',
      ExportError: 'Null',
      ReanchorFailed: 'Null',
      NoDeal: 'Null',
      FeesNotMet: 'Null',
      LockError: 'Null',
      NoPermission: 'Null',
      Unanchored: 'Null',
      NotDepositable: 'Null',
      UnhandledXcmVersion: 'Null',
      WeightLimitReached: 'SpWeightsWeightV2Weight',
      Barrier: 'Null',
      WeightNotComputable: 'Null',
      ExceedsStackLimit: 'Null'
    }
  },
  /**
   * Lookup59: pallet_xcm::pallet::Event<T>
   **/
  PalletXcmEvent: {
    _enum: {
      Attempted: {
        outcome: 'XcmV3TraitsOutcome',
      },
      Sent: {
        origin: 'XcmV3MultiLocation',
        destination: 'XcmV3MultiLocation',
        message: 'XcmV3Xcm',
        messageId: '[u8;32]',
      },
      UnexpectedResponse: {
        origin: 'XcmV3MultiLocation',
        queryId: 'u64',
      },
      ResponseReady: {
        queryId: 'u64',
        response: 'XcmV3Response',
      },
      Notified: {
        queryId: 'u64',
        palletIndex: 'u8',
        callIndex: 'u8',
      },
      NotifyOverweight: {
        queryId: 'u64',
        palletIndex: 'u8',
        callIndex: 'u8',
        actualWeight: 'SpWeightsWeightV2Weight',
        maxBudgetedWeight: 'SpWeightsWeightV2Weight',
      },
      NotifyDispatchError: {
        queryId: 'u64',
        palletIndex: 'u8',
        callIndex: 'u8',
      },
      NotifyDecodeFailed: {
        queryId: 'u64',
        palletIndex: 'u8',
        callIndex: 'u8',
      },
      InvalidResponder: {
        origin: 'XcmV3MultiLocation',
        queryId: 'u64',
        expectedLocation: 'Option<XcmV3MultiLocation>',
      },
      InvalidResponderVersion: {
        origin: 'XcmV3MultiLocation',
        queryId: 'u64',
      },
      ResponseTaken: {
        queryId: 'u64',
      },
      AssetsTrapped: {
        _alias: {
          hash_: 'hash',
        },
        hash_: 'H256',
        origin: 'XcmV3MultiLocation',
        assets: 'XcmVersionedMultiAssets',
      },
      VersionChangeNotified: {
        destination: 'XcmV3MultiLocation',
        result: 'u32',
        cost: 'XcmV3MultiassetMultiAssets',
        messageId: '[u8;32]',
      },
      SupportedVersionChanged: {
        location: 'XcmV3MultiLocation',
        version: 'u32',
      },
      NotifyTargetSendFail: {
        location: 'XcmV3MultiLocation',
        queryId: 'u64',
        error: 'XcmV3TraitsError',
      },
      NotifyTargetMigrationFail: {
        location: 'XcmVersionedMultiLocation',
        queryId: 'u64',
      },
      InvalidQuerierVersion: {
        origin: 'XcmV3MultiLocation',
        queryId: 'u64',
      },
      InvalidQuerier: {
        origin: 'XcmV3MultiLocation',
        queryId: 'u64',
        expectedQuerier: 'XcmV3MultiLocation',
        maybeActualQuerier: 'Option<XcmV3MultiLocation>',
      },
      VersionNotifyStarted: {
        destination: 'XcmV3MultiLocation',
        cost: 'XcmV3MultiassetMultiAssets',
        messageId: '[u8;32]',
      },
      VersionNotifyRequested: {
        destination: 'XcmV3MultiLocation',
        cost: 'XcmV3MultiassetMultiAssets',
        messageId: '[u8;32]',
      },
      VersionNotifyUnrequested: {
        destination: 'XcmV3MultiLocation',
        cost: 'XcmV3MultiassetMultiAssets',
        messageId: '[u8;32]',
      },
      FeesPaid: {
        paying: 'XcmV3MultiLocation',
        fees: 'XcmV3MultiassetMultiAssets',
      },
      AssetsClaimed: {
        _alias: {
          hash_: 'hash',
        },
        hash_: 'H256',
        origin: 'XcmV3MultiLocation',
        assets: 'XcmVersionedMultiAssets'
      }
    }
  },
  /**
   * Lookup60: xcm::v3::traits::Outcome
   **/
  XcmV3TraitsOutcome: {
    _enum: {
      Complete: 'SpWeightsWeightV2Weight',
      Incomplete: '(SpWeightsWeightV2Weight,XcmV3TraitsError)',
      Error: 'XcmV3TraitsError'
    }
  },
  /**
   * Lookup61: xcm::v3::multilocation::MultiLocation
   **/
  XcmV3MultiLocation: {
    parents: 'u8',
    interior: 'XcmV3Junctions'
  },
  /**
   * Lookup62: xcm::v3::junctions::Junctions
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
   * Lookup63: xcm::v3::junction::Junction
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
   * Lookup66: xcm::v3::junction::NetworkId
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
   * Lookup69: xcm::v3::junction::BodyId
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
   * Lookup70: xcm::v3::junction::BodyPart
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
   * Lookup71: xcm::v3::Xcm<Call>
   **/
  XcmV3Xcm: 'Vec<XcmV3Instruction>',
  /**
   * Lookup73: xcm::v3::Instruction<Call>
   **/
  XcmV3Instruction: {
    _enum: {
      WithdrawAsset: 'XcmV3MultiassetMultiAssets',
      ReserveAssetDeposited: 'XcmV3MultiassetMultiAssets',
      ReceiveTeleportedAsset: 'XcmV3MultiassetMultiAssets',
      QueryResponse: {
        queryId: 'Compact<u64>',
        response: 'XcmV3Response',
        maxWeight: 'SpWeightsWeightV2Weight',
        querier: 'Option<XcmV3MultiLocation>',
      },
      TransferAsset: {
        assets: 'XcmV3MultiassetMultiAssets',
        beneficiary: 'XcmV3MultiLocation',
      },
      TransferReserveAsset: {
        assets: 'XcmV3MultiassetMultiAssets',
        dest: 'XcmV3MultiLocation',
        xcm: 'XcmV3Xcm',
      },
      Transact: {
        originKind: 'XcmV2OriginKind',
        requireWeightAtMost: 'SpWeightsWeightV2Weight',
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
      DescendOrigin: 'XcmV3Junctions',
      ReportError: 'XcmV3QueryResponseInfo',
      DepositAsset: {
        assets: 'XcmV3MultiassetMultiAssetFilter',
        beneficiary: 'XcmV3MultiLocation',
      },
      DepositReserveAsset: {
        assets: 'XcmV3MultiassetMultiAssetFilter',
        dest: 'XcmV3MultiLocation',
        xcm: 'XcmV3Xcm',
      },
      ExchangeAsset: {
        give: 'XcmV3MultiassetMultiAssetFilter',
        want: 'XcmV3MultiassetMultiAssets',
        maximal: 'bool',
      },
      InitiateReserveWithdraw: {
        assets: 'XcmV3MultiassetMultiAssetFilter',
        reserve: 'XcmV3MultiLocation',
        xcm: 'XcmV3Xcm',
      },
      InitiateTeleport: {
        assets: 'XcmV3MultiassetMultiAssetFilter',
        dest: 'XcmV3MultiLocation',
        xcm: 'XcmV3Xcm',
      },
      ReportHolding: {
        responseInfo: 'XcmV3QueryResponseInfo',
        assets: 'XcmV3MultiassetMultiAssetFilter',
      },
      BuyExecution: {
        fees: 'XcmV3MultiAsset',
        weightLimit: 'XcmV3WeightLimit',
      },
      RefundSurplus: 'Null',
      SetErrorHandler: 'XcmV3Xcm',
      SetAppendix: 'XcmV3Xcm',
      ClearError: 'Null',
      ClaimAsset: {
        assets: 'XcmV3MultiassetMultiAssets',
        ticket: 'XcmV3MultiLocation',
      },
      Trap: 'Compact<u64>',
      SubscribeVersion: {
        queryId: 'Compact<u64>',
        maxResponseWeight: 'SpWeightsWeightV2Weight',
      },
      UnsubscribeVersion: 'Null',
      BurnAsset: 'XcmV3MultiassetMultiAssets',
      ExpectAsset: 'XcmV3MultiassetMultiAssets',
      ExpectOrigin: 'Option<XcmV3MultiLocation>',
      ExpectError: 'Option<(u32,XcmV3TraitsError)>',
      ExpectTransactStatus: 'XcmV3MaybeErrorCode',
      QueryPallet: {
        moduleName: 'Bytes',
        responseInfo: 'XcmV3QueryResponseInfo',
      },
      ExpectPallet: {
        index: 'Compact<u32>',
        name: 'Bytes',
        moduleName: 'Bytes',
        crateMajor: 'Compact<u32>',
        minCrateMinor: 'Compact<u32>',
      },
      ReportTransactStatus: 'XcmV3QueryResponseInfo',
      ClearTransactStatus: 'Null',
      UniversalOrigin: 'XcmV3Junction',
      ExportMessage: {
        network: 'XcmV3JunctionNetworkId',
        destination: 'XcmV3Junctions',
        xcm: 'XcmV3Xcm',
      },
      LockAsset: {
        asset: 'XcmV3MultiAsset',
        unlocker: 'XcmV3MultiLocation',
      },
      UnlockAsset: {
        asset: 'XcmV3MultiAsset',
        target: 'XcmV3MultiLocation',
      },
      NoteUnlockable: {
        asset: 'XcmV3MultiAsset',
        owner: 'XcmV3MultiLocation',
      },
      RequestUnlock: {
        asset: 'XcmV3MultiAsset',
        locker: 'XcmV3MultiLocation',
      },
      SetFeesMode: {
        jitWithdraw: 'bool',
      },
      SetTopic: '[u8;32]',
      ClearTopic: 'Null',
      AliasOrigin: 'XcmV3MultiLocation',
      UnpaidExecution: {
        weightLimit: 'XcmV3WeightLimit',
        checkOrigin: 'Option<XcmV3MultiLocation>'
      }
    }
  },
  /**
   * Lookup74: xcm::v3::multiasset::MultiAssets
   **/
  XcmV3MultiassetMultiAssets: 'Vec<XcmV3MultiAsset>',
  /**
   * Lookup76: xcm::v3::multiasset::MultiAsset
   **/
  XcmV3MultiAsset: {
    id: 'XcmV3MultiassetAssetId',
    fun: 'XcmV3MultiassetFungibility'
  },
  /**
   * Lookup77: xcm::v3::multiasset::AssetId
   **/
  XcmV3MultiassetAssetId: {
    _enum: {
      Concrete: 'XcmV3MultiLocation',
      Abstract: '[u8;32]'
    }
  },
  /**
   * Lookup78: xcm::v3::multiasset::Fungibility
   **/
  XcmV3MultiassetFungibility: {
    _enum: {
      Fungible: 'Compact<u128>',
      NonFungible: 'XcmV3MultiassetAssetInstance'
    }
  },
  /**
   * Lookup79: xcm::v3::multiasset::AssetInstance
   **/
  XcmV3MultiassetAssetInstance: {
    _enum: {
      Undefined: 'Null',
      Index: 'Compact<u128>',
      Array4: '[u8;4]',
      Array8: '[u8;8]',
      Array16: '[u8;16]',
      Array32: '[u8;32]'
    }
  },
  /**
   * Lookup82: xcm::v3::Response
   **/
  XcmV3Response: {
    _enum: {
      Null: 'Null',
      Assets: 'XcmV3MultiassetMultiAssets',
      ExecutionResult: 'Option<(u32,XcmV3TraitsError)>',
      Version: 'u32',
      PalletsInfo: 'Vec<XcmV3PalletInfo>',
      DispatchResult: 'XcmV3MaybeErrorCode'
    }
  },
  /**
   * Lookup86: xcm::v3::PalletInfo
   **/
  XcmV3PalletInfo: {
    index: 'Compact<u32>',
    name: 'Bytes',
    moduleName: 'Bytes',
    major: 'Compact<u32>',
    minor: 'Compact<u32>',
    patch: 'Compact<u32>'
  },
  /**
   * Lookup89: xcm::v3::MaybeErrorCode
   **/
  XcmV3MaybeErrorCode: {
    _enum: {
      Success: 'Null',
      Error: 'Bytes',
      TruncatedError: 'Bytes'
    }
  },
  /**
   * Lookup92: xcm::v2::OriginKind
   **/
  XcmV2OriginKind: {
    _enum: ['Native', 'SovereignAccount', 'Superuser', 'Xcm']
  },
  /**
   * Lookup93: xcm::double_encoded::DoubleEncoded<T>
   **/
  XcmDoubleEncoded: {
    encoded: 'Bytes'
  },
  /**
   * Lookup94: xcm::v3::QueryResponseInfo
   **/
  XcmV3QueryResponseInfo: {
    destination: 'XcmV3MultiLocation',
    queryId: 'Compact<u64>',
    maxWeight: 'SpWeightsWeightV2Weight'
  },
  /**
   * Lookup95: xcm::v3::multiasset::MultiAssetFilter
   **/
  XcmV3MultiassetMultiAssetFilter: {
    _enum: {
      Definite: 'XcmV3MultiassetMultiAssets',
      Wild: 'XcmV3MultiassetWildMultiAsset'
    }
  },
  /**
   * Lookup96: xcm::v3::multiasset::WildMultiAsset
   **/
  XcmV3MultiassetWildMultiAsset: {
    _enum: {
      All: 'Null',
      AllOf: {
        id: 'XcmV3MultiassetAssetId',
        fun: 'XcmV3MultiassetWildFungibility',
      },
      AllCounted: 'Compact<u32>',
      AllOfCounted: {
        id: 'XcmV3MultiassetAssetId',
        fun: 'XcmV3MultiassetWildFungibility',
        count: 'Compact<u32>'
      }
    }
  },
  /**
   * Lookup97: xcm::v3::multiasset::WildFungibility
   **/
  XcmV3MultiassetWildFungibility: {
    _enum: ['Fungible', 'NonFungible']
  },
  /**
   * Lookup98: xcm::v3::WeightLimit
   **/
  XcmV3WeightLimit: {
    _enum: {
      Unlimited: 'Null',
      Limited: 'SpWeightsWeightV2Weight'
    }
  },
  /**
   * Lookup99: xcm::VersionedMultiAssets
   **/
  XcmVersionedMultiAssets: {
    _enum: {
      __Unused0: 'Null',
      V2: 'XcmV2MultiassetMultiAssets',
      __Unused2: 'Null',
      V3: 'XcmV3MultiassetMultiAssets'
    }
  },
  /**
   * Lookup100: xcm::v2::multiasset::MultiAssets
   **/
  XcmV2MultiassetMultiAssets: 'Vec<XcmV2MultiAsset>',
  /**
   * Lookup102: xcm::v2::multiasset::MultiAsset
   **/
  XcmV2MultiAsset: {
    id: 'XcmV2MultiassetAssetId',
    fun: 'XcmV2MultiassetFungibility'
  },
  /**
   * Lookup103: xcm::v2::multiasset::AssetId
   **/
  XcmV2MultiassetAssetId: {
    _enum: {
      Concrete: 'XcmV2MultiLocation',
      Abstract: 'Bytes'
    }
  },
  /**
   * Lookup104: xcm::v2::multilocation::MultiLocation
   **/
  XcmV2MultiLocation: {
    parents: 'u8',
    interior: 'XcmV2MultilocationJunctions'
  },
  /**
   * Lookup105: xcm::v2::multilocation::Junctions
   **/
  XcmV2MultilocationJunctions: {
    _enum: {
      Here: 'Null',
      X1: 'XcmV2Junction',
      X2: '(XcmV2Junction,XcmV2Junction)',
      X3: '(XcmV2Junction,XcmV2Junction,XcmV2Junction)',
      X4: '(XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction)',
      X5: '(XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction)',
      X6: '(XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction)',
      X7: '(XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction)',
      X8: '(XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction)'
    }
  },
  /**
   * Lookup106: xcm::v2::junction::Junction
   **/
  XcmV2Junction: {
    _enum: {
      Parachain: 'Compact<u32>',
      AccountId32: {
        network: 'XcmV2NetworkId',
        id: '[u8;32]',
      },
      AccountIndex64: {
        network: 'XcmV2NetworkId',
        index: 'Compact<u64>',
      },
      AccountKey20: {
        network: 'XcmV2NetworkId',
        key: '[u8;20]',
      },
      PalletInstance: 'u8',
      GeneralIndex: 'Compact<u128>',
      GeneralKey: 'Bytes',
      OnlyChild: 'Null',
      Plurality: {
        id: 'XcmV2BodyId',
        part: 'XcmV2BodyPart'
      }
    }
  },
  /**
   * Lookup107: xcm::v2::NetworkId
   **/
  XcmV2NetworkId: {
    _enum: {
      Any: 'Null',
      Named: 'Bytes',
      Polkadot: 'Null',
      Kusama: 'Null'
    }
  },
  /**
   * Lookup109: xcm::v2::BodyId
   **/
  XcmV2BodyId: {
    _enum: {
      Unit: 'Null',
      Named: 'Bytes',
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
   * Lookup110: xcm::v2::BodyPart
   **/
  XcmV2BodyPart: {
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
   * Lookup111: xcm::v2::multiasset::Fungibility
   **/
  XcmV2MultiassetFungibility: {
    _enum: {
      Fungible: 'Compact<u128>',
      NonFungible: 'XcmV2MultiassetAssetInstance'
    }
  },
  /**
   * Lookup112: xcm::v2::multiasset::AssetInstance
   **/
  XcmV2MultiassetAssetInstance: {
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
   * Lookup113: xcm::VersionedMultiLocation
   **/
  XcmVersionedMultiLocation: {
    _enum: {
      __Unused0: 'Null',
      V2: 'XcmV2MultiLocation',
      __Unused2: 'Null',
      V3: 'XcmV3MultiLocation'
    }
  },
  /**
   * Lookup114: cumulus_pallet_xcm::pallet::Event<T>
   **/
  CumulusPalletXcmEvent: {
    _enum: {
      InvalidFormat: '[u8;32]',
      UnsupportedVersion: '[u8;32]',
      ExecutedDownward: '([u8;32],XcmV3TraitsOutcome)'
    }
  },
  /**
   * Lookup115: cumulus_pallet_dmp_queue::pallet::Event<T>
   **/
  CumulusPalletDmpQueueEvent: {
    _enum: {
      InvalidFormat: {
        messageHash: '[u8;32]',
      },
      UnsupportedVersion: {
        messageHash: '[u8;32]',
      },
      ExecutedDownward: {
        messageHash: '[u8;32]',
        messageId: '[u8;32]',
        outcome: 'XcmV3TraitsOutcome',
      },
      WeightExhausted: {
        messageHash: '[u8;32]',
        messageId: '[u8;32]',
        remainingWeight: 'SpWeightsWeightV2Weight',
        requiredWeight: 'SpWeightsWeightV2Weight',
      },
      OverweightEnqueued: {
        messageHash: '[u8;32]',
        messageId: '[u8;32]',
        overweightIndex: 'u64',
        requiredWeight: 'SpWeightsWeightV2Weight',
      },
      OverweightServiced: {
        overweightIndex: 'u64',
        weightUsed: 'SpWeightsWeightV2Weight',
      },
      MaxMessagesExhausted: {
        messageHash: '[u8;32]'
      }
    }
  },
  /**
   * Lookup116: pallet_asset_registry::pallet::Event<T>
   **/
  PalletAssetRegistryEvent: {
    _enum: {
      Registered: {
        assetId: 'u32',
        location: 'XcmV3MultiLocation',
      },
      Info: {
        assetId: 'u32',
        location: 'XcmV3MultiLocation'
      }
    }
  },
  /**
   * Lookup117: pallet_xdns::pallet::Event<T>
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
   * Lookup118: pallet_attesters::pallet::Event<T>
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
   * Lookup122: t3rn_primitives::attesters::BatchingFactor
   **/
  T3rnPrimitivesAttestersBatchingFactor: {
    latestConfirmed: 'u16',
    latestSigned: 'u16',
    currentNext: 'u16',
    upToLast10Confirmed: 'Vec<u16>'
  },
  /**
   * Lookup125: pallet_attesters::pallet::BatchMessage<BlockNumber>
   **/
  PalletAttestersBatchMessage: {
    availableToCommitAt: 'u32',
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
   * Lookup133: pallet_attesters::pallet::BatchStatus
   **/
  PalletAttestersBatchStatus: {
    _enum: ['PendingMessage', 'PendingAttestation', 'ReadyForSubmissionByMajority', 'ReadyForSubmissionFullyApproved', 'Repatriated', 'Expired', 'Committed']
  },
  /**
   * Lookup134: t3rn_primitives::attesters::LatencyStatus
   **/
  T3rnPrimitivesAttestersLatencyStatus: {
    _enum: {
      OnTime: 'Null',
      Late: '(u32,u32)'
    }
  },
  /**
   * Lookup136: t3rn_primitives::ExecutionVendor
   **/
  T3rnPrimitivesExecutionVendor: {
    _enum: ['Substrate', 'EVM']
  },
  /**
   * Lookup141: pallet_rewards::pallet::Event<T>
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
   * Lookup144: pallet_contracts_registry::pallet::Event<T>
   **/
  PalletContractsRegistryEvent: {
    _enum: {
      ContractStored: '(AccountId32,H256)',
      ContractPurged: '(AccountId32,H256)'
    }
  },
  /**
   * Lookup145: pallet_circuit::pallet::Event<T>
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
   * Lookup153: xp_format::XbiResult
   **/
  XpFormatXbiResult: {
    status: 'XpFormatStatus',
    output: 'Bytes',
    witness: 'Bytes'
  },
  /**
   * Lookup154: xp_format::Status
   **/
  XpFormatStatus: {
    _enum: ['Success', 'FailedExecution', 'DispatchFailed', 'ExecutionLimitExceeded', 'NotificationLimitExceeded', 'SendTimeout', 'DeliveryTimeout', 'ExecutionTimeout']
  },
  /**
   * Lookup156: t3rn_types::sfx::SideEffect<sp_core::crypto::AccountId32, BalanceOf>
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
   * Lookup159: t3rn_types::fsx::FullSideEffect<sp_core::crypto::AccountId32, BlockNumber, BalanceOf>
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
   * Lookup161: t3rn_types::sfx::ConfirmedSideEffect<sp_core::crypto::AccountId32, BlockNumber, BalanceOf>
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
   * Lookup163: t3rn_types::sfx::ConfirmationOutcome
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
   * Lookup165: t3rn_types::sfx::SecurityLvl
   **/
  T3rnTypesSfxSecurityLvl: {
    _enum: ['Optimistic', 'Escrow']
  },
  /**
   * Lookup167: t3rn_types::bid::SFXBid<sp_core::crypto::AccountId32, BalanceOf, AssetId>
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
   * Lookup168: pallet_clock::pallet::Event<T>
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
   * Lookup169: pallet_circuit_vacuum::pallet::Event<T>
   **/
  PalletCircuitVacuumEvent: {
    _enum: {
      OrderStatusRead: 'PalletCircuitVacuumOrderStatusRead'
    }
  },
  /**
   * Lookup170: pallet_circuit_vacuum::OrderStatusRead<primitive_types::H256, BlockNumber>
   **/
  PalletCircuitVacuumOrderStatusRead: {
    xtxId: 'H256',
    status: 'T3rnPrimitivesCircuitTypesCircuitStatus',
    allIncludedSfx: 'Vec<(H256,T3rnPrimitivesCircuitTypesCircuitStatus)>',
    timeoutsAt: 'T3rnPrimitivesCircuitTypesAdaptiveTimeout'
  },
  /**
   * Lookup171: t3rn_primitives::circuit::types::CircuitStatus
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
   * Lookup172: t3rn_primitives::circuit::types::Cause
   **/
  T3rnPrimitivesCircuitTypesCause: {
    _enum: ['Timeout', 'IntentionalKill']
  },
  /**
   * Lookup175: t3rn_primitives::circuit::types::AdaptiveTimeout<BlockNumber, TargetId>
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
   * Lookup176: pallet_3vm::pallet::Event<T>
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
   * Lookup178: t3rn_sdk_primitives::signal::SignalKind
   **/
  T3rnSdkPrimitivesSignalSignalKind: {
    _enum: {
      Complete: 'Null',
      Kill: 'T3rnSdkPrimitivesSignalKillReason'
    }
  },
  /**
   * Lookup179: t3rn_sdk_primitives::signal::KillReason
   **/
  T3rnSdkPrimitivesSignalKillReason: {
    _enum: ['Unhandled', 'Codec', 'Timeout']
  },
  /**
   * Lookup181: t3rn_primitives::contract_metadata::ContractType
   **/
  T3rnPrimitivesContractMetadataContractType: {
    _enum: ['System', 'VanillaEvm', 'VanillaWasm', 'VolatileEvm', 'VolatileWasm']
  },
  /**
   * Lookup183: pallet_contracts::pallet::Event<T>
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
   * Lookup184: pallet_contracts::Origin<t0rn_parachain_runtime::Runtime>
   **/
  PalletContractsOrigin: {
    _enum: {
      Root: 'Null',
      Signed: 'AccountId32'
    }
  },
  /**
   * Lookup185: t0rn_parachain_runtime::Runtime
   **/
  T0rnParachainRuntimeRuntime: 'Null',
  /**
   * Lookup186: pallet_evm::pallet::Event<T>
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
   * Lookup187: ethereum::log::Log
   **/
  EthereumLog: {
    address: 'H160',
    topics: 'Vec<H256>',
    data: 'Bytes'
  },
  /**
   * Lookup188: pallet_portal::pallet::Event<T>
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
   * Lookup189: t3rn_primitives::GatewayVendor
   **/
  T3rnPrimitivesGatewayVendor: {
    _enum: ['Polkadot', 'Kusama', 'Rococo', 'Ethereum', 'Sepolia', 'XBI']
  },
  /**
   * Lookup190: pallet_grandpa_finality_verifier::pallet::Event<T, I>
   **/
  PalletGrandpaFinalityVerifierEvent: {
    _enum: {
      HeadersAdded: 'u32'
    }
  },
  /**
   * Lookup193: pallet_eth2_finality_verifier::pallet::Event<T>
   **/
  PalletEth2FinalityVerifierEvent: {
    _enum: {
      EpochUpdate: 'PalletEth2FinalityVerifierEpochSubmitted'
    }
  },
  /**
   * Lookup194: pallet_eth2_finality_verifier::types::EpochSubmitted
   **/
  PalletEth2FinalityVerifierEpochSubmitted: {
    epoch: 'u64',
    beaconHeight: 'u64',
    executionHeight: 'u64'
  },
  /**
   * Lookup195: pallet_sepolia_finality_verifier::pallet::Event<T>
   **/
  PalletSepoliaFinalityVerifierEvent: {
    _enum: {
      EpochUpdate: 'PalletSepoliaFinalityVerifierEpochSubmitted'
    }
  },
  /**
   * Lookup196: pallet_sepolia_finality_verifier::types::EpochSubmitted
   **/
  PalletSepoliaFinalityVerifierEpochSubmitted: {
    epoch: 'u64',
    beaconHeight: 'u64',
    executionHeight: 'u64'
  },
  /**
   * Lookup197: pallet_maintenance_mode::pallet::Event
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
   * Lookup198: pallet_sudo::pallet::Event<T>
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
   * Lookup199: frame_system::Phase
   **/
  FrameSystemPhase: {
    _enum: {
      ApplyExtrinsic: 'u32',
      Finalization: 'Null',
      Initialization: 'Null'
    }
  },
  /**
   * Lookup201: frame_system::LastRuntimeUpgradeInfo
   **/
  FrameSystemLastRuntimeUpgradeInfo: {
    specVersion: 'Compact<u32>',
    specName: 'Text'
  },
  /**
   * Lookup203: frame_system::pallet::Call<T>
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
   * Lookup206: frame_system::limits::BlockWeights
   **/
  FrameSystemLimitsBlockWeights: {
    baseBlock: 'SpWeightsWeightV2Weight',
    maxBlock: 'SpWeightsWeightV2Weight',
    perClass: 'FrameSupportDispatchPerDispatchClassWeightsPerClass'
  },
  /**
   * Lookup207: frame_support::dispatch::PerDispatchClass<frame_system::limits::WeightsPerClass>
   **/
  FrameSupportDispatchPerDispatchClassWeightsPerClass: {
    normal: 'FrameSystemLimitsWeightsPerClass',
    operational: 'FrameSystemLimitsWeightsPerClass',
    mandatory: 'FrameSystemLimitsWeightsPerClass'
  },
  /**
   * Lookup208: frame_system::limits::WeightsPerClass
   **/
  FrameSystemLimitsWeightsPerClass: {
    baseExtrinsic: 'SpWeightsWeightV2Weight',
    maxExtrinsic: 'Option<SpWeightsWeightV2Weight>',
    maxTotal: 'Option<SpWeightsWeightV2Weight>',
    reserved: 'Option<SpWeightsWeightV2Weight>'
  },
  /**
   * Lookup210: frame_system::limits::BlockLength
   **/
  FrameSystemLimitsBlockLength: {
    max: 'FrameSupportDispatchPerDispatchClassU32'
  },
  /**
   * Lookup211: frame_support::dispatch::PerDispatchClass<T>
   **/
  FrameSupportDispatchPerDispatchClassU32: {
    normal: 'u32',
    operational: 'u32',
    mandatory: 'u32'
  },
  /**
   * Lookup212: sp_weights::RuntimeDbWeight
   **/
  SpWeightsRuntimeDbWeight: {
    read: 'u64',
    write: 'u64'
  },
  /**
   * Lookup213: sp_version::RuntimeVersion
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
   * Lookup217: frame_system::pallet::Error<T>
   **/
  FrameSystemError: {
    _enum: ['InvalidSpecName', 'SpecVersionNeedsToIncrease', 'FailedToExtractRuntimeVersion', 'NonDefaultComposite', 'NonZeroRefCount', 'CallFiltered']
  },
  /**
   * Lookup218: polkadot_primitives::v5::PersistedValidationData<primitive_types::H256, N>
   **/
  PolkadotPrimitivesV5PersistedValidationData: {
    parentHead: 'Bytes',
    relayParentNumber: 'u32',
    relayParentStorageRoot: 'H256',
    maxPovSize: 'u32'
  },
  /**
   * Lookup221: polkadot_primitives::v5::UpgradeRestriction
   **/
  PolkadotPrimitivesV5UpgradeRestriction: {
    _enum: ['Present']
  },
  /**
   * Lookup222: sp_trie::storage_proof::StorageProof
   **/
  SpTrieStorageProof: {
    trieNodes: 'BTreeSet<Bytes>'
  },
  /**
   * Lookup224: cumulus_pallet_parachain_system::relay_state_snapshot::MessagingStateSnapshot
   **/
  CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot: {
    dmqMqcHead: 'H256',
    relayDispatchQueueSize: 'CumulusPalletParachainSystemRelayStateSnapshotRelayDispachQueueSize',
    ingressChannels: 'Vec<(u32,PolkadotPrimitivesV5AbridgedHrmpChannel)>',
    egressChannels: 'Vec<(u32,PolkadotPrimitivesV5AbridgedHrmpChannel)>'
  },
  /**
   * Lookup225: cumulus_pallet_parachain_system::relay_state_snapshot::RelayDispachQueueSize
   **/
  CumulusPalletParachainSystemRelayStateSnapshotRelayDispachQueueSize: {
    remainingCount: 'u32',
    remainingSize: 'u32'
  },
  /**
   * Lookup228: polkadot_primitives::v5::AbridgedHrmpChannel
   **/
  PolkadotPrimitivesV5AbridgedHrmpChannel: {
    maxCapacity: 'u32',
    maxTotalSize: 'u32',
    maxMessageSize: 'u32',
    msgCount: 'u32',
    totalSize: 'u32',
    mqcHead: 'Option<H256>'
  },
  /**
   * Lookup230: polkadot_primitives::v5::AbridgedHostConfiguration
   **/
  PolkadotPrimitivesV5AbridgedHostConfiguration: {
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
   * Lookup236: polkadot_core_primitives::OutboundHrmpMessage<polkadot_parachain::primitives::Id>
   **/
  PolkadotCorePrimitivesOutboundHrmpMessage: {
    recipient: 'u32',
    data: 'Bytes'
  },
  /**
   * Lookup237: cumulus_pallet_parachain_system::CodeUpgradeAuthorization<T>
   **/
  CumulusPalletParachainSystemCodeUpgradeAuthorization: {
    codeHash: 'H256',
    checkVersion: 'bool'
  },
  /**
   * Lookup238: cumulus_pallet_parachain_system::pallet::Call<T>
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
        checkVersion: 'bool',
      },
      enact_authorized_upgrade: {
        code: 'Bytes'
      }
    }
  },
  /**
   * Lookup239: cumulus_primitives_parachain_inherent::ParachainInherentData
   **/
  CumulusPrimitivesParachainInherentParachainInherentData: {
    validationData: 'PolkadotPrimitivesV5PersistedValidationData',
    relayChainState: 'SpTrieStorageProof',
    downwardMessages: 'Vec<PolkadotCorePrimitivesInboundDownwardMessage>',
    horizontalMessages: 'BTreeMap<u32, Vec<PolkadotCorePrimitivesInboundHrmpMessage>>'
  },
  /**
   * Lookup241: polkadot_core_primitives::InboundDownwardMessage<BlockNumber>
   **/
  PolkadotCorePrimitivesInboundDownwardMessage: {
    sentAt: 'u32',
    msg: 'Bytes'
  },
  /**
   * Lookup244: polkadot_core_primitives::InboundHrmpMessage<BlockNumber>
   **/
  PolkadotCorePrimitivesInboundHrmpMessage: {
    sentAt: 'u32',
    data: 'Bytes'
  },
  /**
   * Lookup247: cumulus_pallet_parachain_system::pallet::Error<T>
   **/
  CumulusPalletParachainSystemError: {
    _enum: ['OverlappingUpgrades', 'ProhibitedByPolkadot', 'TooBig', 'ValidationDataNotAvailable', 'HostConfigurationNotAvailable', 'NotScheduled', 'NothingAuthorized', 'Unauthorized']
  },
  /**
   * Lookup248: pallet_timestamp::pallet::Call<T>
   **/
  PalletTimestampCall: {
    _enum: {
      set: {
        now: 'Compact<u64>'
      }
    }
  },
  /**
   * Lookup249: parachain_info::pallet::Call<T>
   **/
  ParachainInfoCall: 'Null',
  /**
   * Lookup250: pallet_preimage::RequestStatus<sp_core::crypto::AccountId32, Balance>
   **/
  PalletPreimageRequestStatus: {
    _enum: {
      Unrequested: {
        deposit: '(AccountId32,u128)',
        len: 'u32',
      },
      Requested: {
        deposit: 'Option<(AccountId32,u128)>',
        count: 'u32',
        len: 'Option<u32>'
      }
    }
  },
  /**
   * Lookup255: pallet_preimage::pallet::Call<T>
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
   * Lookup256: pallet_preimage::pallet::Error<T>
   **/
  PalletPreimageError: {
    _enum: ['TooBig', 'AlreadyNoted', 'NotAuthorized', 'NotNoted', 'Requested', 'NotRequested']
  },
  /**
   * Lookup259: pallet_scheduler::Scheduled<Name, frame_support::traits::preimages::Bounded<t0rn_parachain_runtime::RuntimeCall>, BlockNumber, t0rn_parachain_runtime::OriginCaller, sp_core::crypto::AccountId32>
   **/
  PalletSchedulerScheduled: {
    maybeId: 'Option<[u8;32]>',
    priority: 'u8',
    call: 'FrameSupportPreimagesBounded',
    maybePeriodic: 'Option<(u32,u32)>',
    origin: 'T0rnParachainRuntimeOriginCaller'
  },
  /**
   * Lookup260: frame_support::traits::preimages::Bounded<t0rn_parachain_runtime::RuntimeCall>
   **/
  FrameSupportPreimagesBounded: {
    _enum: {
      Legacy: {
        _alias: {
          hash_: 'hash',
        },
        hash_: 'H256',
      },
      Inline: 'Bytes',
      Lookup: {
        _alias: {
          hash_: 'hash',
        },
        hash_: 'H256',
        len: 'u32'
      }
    }
  },
  /**
   * Lookup262: pallet_scheduler::pallet::Call<T>
   **/
  PalletSchedulerCall: {
    _enum: {
      schedule: {
        when: 'u32',
        maybePeriodic: 'Option<(u32,u32)>',
        priority: 'u8',
        call: 'Call',
      },
      cancel: {
        when: 'u32',
        index: 'u32',
      },
      schedule_named: {
        id: '[u8;32]',
        when: 'u32',
        maybePeriodic: 'Option<(u32,u32)>',
        priority: 'u8',
        call: 'Call',
      },
      cancel_named: {
        id: '[u8;32]',
      },
      schedule_after: {
        after: 'u32',
        maybePeriodic: 'Option<(u32,u32)>',
        priority: 'u8',
        call: 'Call',
      },
      schedule_named_after: {
        id: '[u8;32]',
        after: 'u32',
        maybePeriodic: 'Option<(u32,u32)>',
        priority: 'u8',
        call: 'Call'
      }
    }
  },
  /**
   * Lookup264: pallet_utility::pallet::Call<T>
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
        calls: 'Vec<Call>',
      },
      with_weight: {
        call: 'Call',
        weight: 'SpWeightsWeightV2Weight'
      }
    }
  },
  /**
   * Lookup266: t0rn_parachain_runtime::OriginCaller
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
   * Lookup267: frame_support::dispatch::RawOrigin<sp_core::crypto::AccountId32>
   **/
  FrameSupportDispatchRawOrigin: {
    _enum: {
      Root: 'Null',
      Signed: 'AccountId32',
      None: 'Null'
    }
  },
  /**
   * Lookup268: pallet_xcm::pallet::Origin
   **/
  PalletXcmOrigin: {
    _enum: {
      Xcm: 'XcmV3MultiLocation',
      Response: 'XcmV3MultiLocation'
    }
  },
  /**
   * Lookup269: cumulus_pallet_xcm::pallet::Origin
   **/
  CumulusPalletXcmOrigin: {
    _enum: {
      Relay: 'Null',
      SiblingParachain: 'u32'
    }
  },
  /**
   * Lookup270: sp_core::Void
   **/
  SpCoreVoid: 'Null',
  /**
   * Lookup271: pallet_identity::pallet::Call<T>
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
   * Lookup274: pallet_identity::types::IdentityInfo<FieldLimit>
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
   * Lookup310: pallet_identity::types::BitFlags<pallet_identity::types::IdentityField>
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
   * Lookup311: pallet_identity::types::IdentityField
   **/
  PalletIdentityIdentityField: {
    _enum: ['__Unused0', 'Display', 'Legal', '__Unused3', 'Web', '__Unused5', '__Unused6', '__Unused7', 'Riot', '__Unused9', '__Unused10', '__Unused11', '__Unused12', '__Unused13', '__Unused14', '__Unused15', 'Email', '__Unused17', '__Unused18', '__Unused19', '__Unused20', '__Unused21', '__Unused22', '__Unused23', '__Unused24', '__Unused25', '__Unused26', '__Unused27', '__Unused28', '__Unused29', '__Unused30', '__Unused31', 'PgpFingerprint', '__Unused33', '__Unused34', '__Unused35', '__Unused36', '__Unused37', '__Unused38', '__Unused39', '__Unused40', '__Unused41', '__Unused42', '__Unused43', '__Unused44', '__Unused45', '__Unused46', '__Unused47', '__Unused48', '__Unused49', '__Unused50', '__Unused51', '__Unused52', '__Unused53', '__Unused54', '__Unused55', '__Unused56', '__Unused57', '__Unused58', '__Unused59', '__Unused60', '__Unused61', '__Unused62', '__Unused63', 'Image', '__Unused65', '__Unused66', '__Unused67', '__Unused68', '__Unused69', '__Unused70', '__Unused71', '__Unused72', '__Unused73', '__Unused74', '__Unused75', '__Unused76', '__Unused77', '__Unused78', '__Unused79', '__Unused80', '__Unused81', '__Unused82', '__Unused83', '__Unused84', '__Unused85', '__Unused86', '__Unused87', '__Unused88', '__Unused89', '__Unused90', '__Unused91', '__Unused92', '__Unused93', '__Unused94', '__Unused95', '__Unused96', '__Unused97', '__Unused98', '__Unused99', '__Unused100', '__Unused101', '__Unused102', '__Unused103', '__Unused104', '__Unused105', '__Unused106', '__Unused107', '__Unused108', '__Unused109', '__Unused110', '__Unused111', '__Unused112', '__Unused113', '__Unused114', '__Unused115', '__Unused116', '__Unused117', '__Unused118', '__Unused119', '__Unused120', '__Unused121', '__Unused122', '__Unused123', '__Unused124', '__Unused125', '__Unused126', '__Unused127', 'Twitter']
  },
  /**
   * Lookup312: pallet_identity::types::Judgement<Balance>
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
   * Lookup313: pallet_balances::pallet::Call<T, I>
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
   * Lookup314: pallet_assets::pallet::Call<T, I>
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
   * Lookup315: pallet_account_manager::pallet::Call<T>
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
   * Lookup316: t3rn_primitives::claimable::BenefitSource
   **/
  T3rnPrimitivesClaimableBenefitSource: {
    _enum: ['BootstrapPool', 'Inflation', 'TrafficFees', 'TrafficRewards', 'Unsettled', 'SlashTreasury']
  },
  /**
   * Lookup317: t3rn_primitives::claimable::CircuitRole
   **/
  T3rnPrimitivesClaimableCircuitRole: {
    _enum: ['Ambassador', 'Executor', 'Attester', 'Staker', 'Collator', 'ContractAuthor', 'Relayer', 'Requester', 'Local']
  },
  /**
   * Lookup318: t3rn_primitives::account_manager::Outcome
   **/
  T3rnPrimitivesAccountManagerOutcome: {
    _enum: ['UnexpectedFailure', 'Revert', 'Commit', 'Slash']
  },
  /**
   * Lookup319: pallet_treasury::pallet::Call<T, I>
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
   * Lookup324: pallet_collator_selection::pallet::Call<T>
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
      leave_intent: 'Null',
      add_invulnerable: {
        who: 'AccountId32',
      },
      remove_invulnerable: {
        who: 'AccountId32'
      }
    }
  },
  /**
   * Lookup325: pallet_session::pallet::Call<T>
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
   * Lookup326: t0rn_parachain_runtime::parachain_config::SessionKeys
   **/
  T0rnParachainRuntimeParachainConfigSessionKeys: {
    aura: 'SpConsensusAuraSr25519AppSr25519Public'
  },
  /**
   * Lookup327: sp_consensus_aura::sr25519::app_sr25519::Public
   **/
  SpConsensusAuraSr25519AppSr25519Public: 'SpCoreSr25519Public',
  /**
   * Lookup328: sp_core::sr25519::Public
   **/
  SpCoreSr25519Public: '[u8;32]',
  /**
   * Lookup329: cumulus_pallet_xcmp_queue::pallet::Call<T>
   **/
  CumulusPalletXcmpQueueCall: {
    _enum: {
      service_overweight: {
        index: 'u64',
        weightLimit: 'SpWeightsWeightV2Weight',
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
        new_: 'SpWeightsWeightV2Weight',
      },
      update_weight_restrict_decay: {
        _alias: {
          new_: 'new',
        },
        new_: 'SpWeightsWeightV2Weight',
      },
      update_xcmp_max_individual_weight: {
        _alias: {
          new_: 'new',
        },
        new_: 'SpWeightsWeightV2Weight'
      }
    }
  },
  /**
   * Lookup330: pallet_xcm::pallet::Call<T>
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
        maxWeight: 'SpWeightsWeightV2Weight',
      },
      force_xcm_version: {
        location: 'XcmV3MultiLocation',
        version: 'u32',
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
        weightLimit: 'XcmV3WeightLimit',
      },
      limited_teleport_assets: {
        dest: 'XcmVersionedMultiLocation',
        beneficiary: 'XcmVersionedMultiLocation',
        assets: 'XcmVersionedMultiAssets',
        feeAssetItem: 'u32',
        weightLimit: 'XcmV3WeightLimit',
      },
      force_suspension: {
        suspended: 'bool'
      }
    }
  },
  /**
   * Lookup331: xcm::VersionedXcm<RuntimeCall>
   **/
  XcmVersionedXcm: {
    _enum: {
      __Unused0: 'Null',
      __Unused1: 'Null',
      V2: 'XcmV2Xcm',
      V3: 'XcmV3Xcm'
    }
  },
  /**
   * Lookup332: xcm::v2::Xcm<RuntimeCall>
   **/
  XcmV2Xcm: 'Vec<XcmV2Instruction>',
  /**
   * Lookup334: xcm::v2::Instruction<RuntimeCall>
   **/
  XcmV2Instruction: {
    _enum: {
      WithdrawAsset: 'XcmV2MultiassetMultiAssets',
      ReserveAssetDeposited: 'XcmV2MultiassetMultiAssets',
      ReceiveTeleportedAsset: 'XcmV2MultiassetMultiAssets',
      QueryResponse: {
        queryId: 'Compact<u64>',
        response: 'XcmV2Response',
        maxWeight: 'Compact<u64>',
      },
      TransferAsset: {
        assets: 'XcmV2MultiassetMultiAssets',
        beneficiary: 'XcmV2MultiLocation',
      },
      TransferReserveAsset: {
        assets: 'XcmV2MultiassetMultiAssets',
        dest: 'XcmV2MultiLocation',
        xcm: 'XcmV2Xcm',
      },
      Transact: {
        originType: 'XcmV2OriginKind',
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
      DescendOrigin: 'XcmV2MultilocationJunctions',
      ReportError: {
        queryId: 'Compact<u64>',
        dest: 'XcmV2MultiLocation',
        maxResponseWeight: 'Compact<u64>',
      },
      DepositAsset: {
        assets: 'XcmV2MultiassetMultiAssetFilter',
        maxAssets: 'Compact<u32>',
        beneficiary: 'XcmV2MultiLocation',
      },
      DepositReserveAsset: {
        assets: 'XcmV2MultiassetMultiAssetFilter',
        maxAssets: 'Compact<u32>',
        dest: 'XcmV2MultiLocation',
        xcm: 'XcmV2Xcm',
      },
      ExchangeAsset: {
        give: 'XcmV2MultiassetMultiAssetFilter',
        receive: 'XcmV2MultiassetMultiAssets',
      },
      InitiateReserveWithdraw: {
        assets: 'XcmV2MultiassetMultiAssetFilter',
        reserve: 'XcmV2MultiLocation',
        xcm: 'XcmV2Xcm',
      },
      InitiateTeleport: {
        assets: 'XcmV2MultiassetMultiAssetFilter',
        dest: 'XcmV2MultiLocation',
        xcm: 'XcmV2Xcm',
      },
      QueryHolding: {
        queryId: 'Compact<u64>',
        dest: 'XcmV2MultiLocation',
        assets: 'XcmV2MultiassetMultiAssetFilter',
        maxResponseWeight: 'Compact<u64>',
      },
      BuyExecution: {
        fees: 'XcmV2MultiAsset',
        weightLimit: 'XcmV2WeightLimit',
      },
      RefundSurplus: 'Null',
      SetErrorHandler: 'XcmV2Xcm',
      SetAppendix: 'XcmV2Xcm',
      ClearError: 'Null',
      ClaimAsset: {
        assets: 'XcmV2MultiassetMultiAssets',
        ticket: 'XcmV2MultiLocation',
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
   * Lookup335: xcm::v2::Response
   **/
  XcmV2Response: {
    _enum: {
      Null: 'Null',
      Assets: 'XcmV2MultiassetMultiAssets',
      ExecutionResult: 'Option<(u32,XcmV2TraitsError)>',
      Version: 'u32'
    }
  },
  /**
   * Lookup338: xcm::v2::traits::Error
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
   * Lookup339: xcm::v2::multiasset::MultiAssetFilter
   **/
  XcmV2MultiassetMultiAssetFilter: {
    _enum: {
      Definite: 'XcmV2MultiassetMultiAssets',
      Wild: 'XcmV2MultiassetWildMultiAsset'
    }
  },
  /**
   * Lookup340: xcm::v2::multiasset::WildMultiAsset
   **/
  XcmV2MultiassetWildMultiAsset: {
    _enum: {
      All: 'Null',
      AllOf: {
        id: 'XcmV2MultiassetAssetId',
        fun: 'XcmV2MultiassetWildFungibility'
      }
    }
  },
  /**
   * Lookup341: xcm::v2::multiasset::WildFungibility
   **/
  XcmV2MultiassetWildFungibility: {
    _enum: ['Fungible', 'NonFungible']
  },
  /**
   * Lookup342: xcm::v2::WeightLimit
   **/
  XcmV2WeightLimit: {
    _enum: {
      Unlimited: 'Null',
      Limited: 'Compact<u64>'
    }
  },
  /**
   * Lookup351: cumulus_pallet_xcm::pallet::Call<T>
   **/
  CumulusPalletXcmCall: 'Null',
  /**
   * Lookup352: cumulus_pallet_dmp_queue::pallet::Call<T>
   **/
  CumulusPalletDmpQueueCall: {
    _enum: {
      service_overweight: {
        index: 'u64',
        weightLimit: 'SpWeightsWeightV2Weight'
      }
    }
  },
  /**
   * Lookup353: pallet_asset_registry::pallet::Call<T>
   **/
  PalletAssetRegistryCall: {
    _enum: {
      register: {
        location: 'XcmV3MultiLocation',
        id: 'u32',
      },
      register_info: {
        info: 'PalletAssetRegistryAssetInfo'
      }
    }
  },
  /**
   * Lookup354: pallet_asset_registry::AssetInfo<AssetId, sp_core::crypto::AccountId32, Balance>
   **/
  PalletAssetRegistryAssetInfo: {
    id: 'u32',
    capabilities: 'Vec<PalletAssetRegistryCapability>',
    location: 'XcmV3MultiLocation'
  },
  /**
   * Lookup356: pallet_asset_registry::Capability<sp_core::crypto::AccountId32, Balance>
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
   * Lookup357: pallet_xdns::pallet::Call<T>
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
   * Lookup358: pallet_attesters::pallet::Call<T>
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
   * Lookup361: pallet_rewards::pallet::Call<T>
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
   * Lookup363: pallet_contracts_registry::pallet::Call<T>
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
   * Lookup364: t3rn_primitives::contracts_registry::RegistryContract<primitive_types::H256, sp_core::crypto::AccountId32, BalanceOf, BlockNumber>
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
   * Lookup365: t3rn_primitives::contracts_registry::AuthorInfo<sp_core::crypto::AccountId32, BalanceOf>
   **/
  T3rnPrimitivesContractsRegistryAuthorInfo: {
    account: 'AccountId32',
    feesPerSingleUse: 'Option<u128>'
  },
  /**
   * Lookup367: t3rn_types::gateway::ContractActionDesc<primitive_types::H256, TargetId, sp_core::crypto::AccountId32>
   **/
  T3rnTypesGatewayContractActionDesc: {
    actionId: 'H256',
    targetId: 'Option<[u8;4]>',
    to: 'Option<AccountId32>'
  },
  /**
   * Lookup370: t3rn_primitives::storage::RawAliveContractInfo<primitive_types::H256, Balance, BlockNumber>
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
   * Lookup372: t3rn_primitives::contract_metadata::ContractMetadata
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
   * Lookup373: pallet_circuit::pallet::Call<T>
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
   * Lookup374: t3rn_primitives::SpeedMode
   **/
  T3rnPrimitivesSpeedMode: {
    _enum: ['Fast', 'Rational', 'Finalized']
  },
  /**
   * Lookup375: pallet_circuit_vacuum::pallet::Call<T>
   **/
  PalletCircuitVacuumCall: {
    _enum: {
      order: {
        sfxActions: 'Vec<T3rnPrimitivesCircuitTypesOrderSFX>',
        speedMode: 'T3rnPrimitivesSpeedMode',
      },
      read_order_status: {
        xtxId: 'H256'
      }
    }
  },
  /**
   * Lookup377: t3rn_primitives::circuit::types::OrderSFX<sp_core::crypto::AccountId32, Asset, Balance, Destination, Input, MaxCost>
   **/
  T3rnPrimitivesCircuitTypesOrderSFX: {
    sfxAction: 'T3rnPrimitivesCircuitTypesSfxAction',
    maxReward: 'u128',
    rewardAsset: 'u32',
    insurance: 'u128',
    remoteOriginNonce: 'Option<u32>'
  },
  /**
   * Lookup378: t3rn_primitives::circuit::types::SFXAction<sp_core::crypto::AccountId32, Asset, Balance, Destination, Input, MaxCost>
   **/
  T3rnPrimitivesCircuitTypesSfxAction: {
    _enum: {
      Call: '([u8;4],AccountId32,u128,u128,Bytes)',
      Transfer: '([u8;4],u32,AccountId32,u128)'
    }
  },
  /**
   * Lookup379: pallet_3vm::pallet::Call<T>
   **/
  Pallet3vmCall: 'Null',
  /**
   * Lookup380: pallet_contracts::pallet::Call<T>
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
   * Lookup382: pallet_contracts::wasm::Determinism
   **/
  PalletContractsWasmDeterminism: {
    _enum: ['Enforced', 'Relaxed']
  },
  /**
   * Lookup383: pallet_evm::pallet::Call<T>
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
   * Lookup384: pallet_portal::pallet::Call<T>
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
        encodedRegistrationData: 'Bytes'
      }
    }
  },
  /**
   * Lookup385: t3rn_abi::recode::Codec
   **/
  T3rnAbiRecodeCodec: {
    _enum: ['Scale', 'Rlp']
  },
  /**
   * Lookup389: t3rn_primitives::TokenInfo
   **/
  T3rnPrimitivesTokenInfo: {
    _enum: {
      Substrate: 'T3rnPrimitivesSubstrateToken',
      Ethereum: 'T3rnPrimitivesEthereumToken'
    }
  },
  /**
   * Lookup390: t3rn_primitives::SubstrateToken
   **/
  T3rnPrimitivesSubstrateToken: {
    id: 'u32',
    symbol: 'Bytes',
    decimals: 'u8'
  },
  /**
   * Lookup391: t3rn_primitives::EthereumToken
   **/
  T3rnPrimitivesEthereumToken: {
    symbol: 'Bytes',
    decimals: 'u8',
    address: 'Option<[u8;20]>'
  },
  /**
   * Lookup392: pallet_grandpa_finality_verifier::pallet::Call<T, I>
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
   * Lookup394: sp_runtime::generic::header::Header<Number, Hash>
   **/
  SpRuntimeHeader: {
    parentHash: 'H256',
    number: 'Compact<u32>',
    stateRoot: 'H256',
    extrinsicsRoot: 'H256',
    digest: 'SpRuntimeDigest'
  },
  /**
   * Lookup395: pallet_grandpa_finality_verifier::bridges::header_chain::justification::GrandpaJustification<sp_runtime::generic::header::Header<Number, Hash>>
   **/
  PalletGrandpaFinalityVerifierBridgesHeaderChainJustificationGrandpaJustification: {
    round: 'u64',
    commit: 'FinalityGrandpaCommit',
    votesAncestries: 'Vec<SpRuntimeHeader>'
  },
  /**
   * Lookup396: finality_grandpa::Commit<primitive_types::H256, N, sp_consensus_grandpa::app::Signature, sp_consensus_grandpa::app::Public>
   **/
  FinalityGrandpaCommit: {
    targetHash: 'H256',
    targetNumber: 'u32',
    precommits: 'Vec<FinalityGrandpaSignedPrecommit>'
  },
  /**
   * Lookup397: sp_consensus_grandpa::app::Signature
   **/
  SpConsensusGrandpaAppSignature: 'SpCoreEd25519Signature',
  /**
   * Lookup398: sp_core::ed25519::Signature
   **/
  SpCoreEd25519Signature: '[u8;64]',
  /**
   * Lookup400: sp_consensus_grandpa::app::Public
   **/
  SpConsensusGrandpaAppPublic: 'SpCoreEd25519Public',
  /**
   * Lookup401: sp_core::ed25519::Public
   **/
  SpCoreEd25519Public: '[u8;32]',
  /**
   * Lookup403: finality_grandpa::SignedPrecommit<primitive_types::H256, N, sp_consensus_grandpa::app::Signature, sp_consensus_grandpa::app::Public>
   **/
  FinalityGrandpaSignedPrecommit: {
    precommit: 'FinalityGrandpaPrecommit',
    signature: 'SpConsensusGrandpaAppSignature',
    id: 'SpConsensusGrandpaAppPublic'
  },
  /**
   * Lookup404: finality_grandpa::Precommit<primitive_types::H256, N>
   **/
  FinalityGrandpaPrecommit: {
    targetHash: 'H256',
    targetNumber: 'u32'
  },
  /**
   * Lookup407: pallet_eth2_finality_verifier::pallet::Call<T>
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
      reset: 'Null'
    }
  },
  /**
   * Lookup408: pallet_eth2_finality_verifier::types::BeaconBlockHeader
   **/
  PalletEth2FinalityVerifierBeaconBlockHeader: {
    slot: 'u64',
    proposerIndex: 'u64',
    parentRoot: '[u8;32]',
    stateRoot: '[u8;32]',
    bodyRoot: '[u8;32]'
  },
  /**
   * Lookup411: pallet_eth2_finality_verifier::types::MerkleProof
   **/
  PalletEth2FinalityVerifierMerkleProof: {
    gIndex: 'u64',
    witness: 'Vec<[u8;32]>'
  },
  /**
   * Lookup413: pallet_eth2_finality_verifier::types::ExecutionPayload
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
   * Lookup414: ethbloom::Bloom
   **/
  EthbloomBloom: '[u8;256]',
  /**
   * Lookup417: pallet_eth2_finality_verifier::types::ExecutionHeader
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
   * Lookup418: pallet_eth2_finality_verifier::types::SyncCommittee
   **/
  PalletEth2FinalityVerifierSyncCommittee: {
    pubs: 'Vec<[u8;48]>',
    aggr: '[u8;48]'
  },
  /**
   * Lookup421: pallet_eth2_finality_verifier::types::EthereumReceiptInclusionProof
   **/
  PalletEth2FinalityVerifierEthereumReceiptInclusionProof: {
    blockNumber: 'u64',
    witness: 'Vec<Bytes>',
    index: 'Bytes'
  },
  /**
   * Lookup422: pallet_eth2_finality_verifier::types::EthereumEventInclusionProof
   **/
  PalletEth2FinalityVerifierEthereumEventInclusionProof: {
    blockNumber: 'u64',
    witness: 'Vec<Bytes>',
    index: 'Bytes',
    event: 'Bytes'
  },
  /**
   * Lookup424: pallet_sepolia_finality_verifier::pallet::Call<T>
   **/
  PalletSepoliaFinalityVerifierCall: {
    _enum: {
      submit_epoch_debug: {
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
      submit_epoch_skipped_slot: {
        encodedUpdate: 'Bytes',
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
      reset: 'Null'
    }
  },
  /**
   * Lookup425: pallet_sepolia_finality_verifier::types::BeaconBlockHeader
   **/
  PalletSepoliaFinalityVerifierBeaconBlockHeader: {
    slot: 'u64',
    proposerIndex: 'u64',
    parentRoot: '[u8;32]',
    stateRoot: '[u8;32]',
    bodyRoot: '[u8;32]'
  },
  /**
   * Lookup426: pallet_sepolia_finality_verifier::types::MerkleProof
   **/
  PalletSepoliaFinalityVerifierMerkleProof: {
    gIndex: 'u64',
    witness: 'Vec<[u8;32]>'
  },
  /**
   * Lookup427: pallet_sepolia_finality_verifier::types::ExecutionPayload
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
   * Lookup429: pallet_sepolia_finality_verifier::types::ExecutionHeader
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
   * Lookup430: pallet_sepolia_finality_verifier::types::SyncCommittee
   **/
  PalletSepoliaFinalityVerifierSyncCommittee: {
    pubs: 'Vec<[u8;48]>',
    aggr: '[u8;48]'
  },
  /**
   * Lookup431: pallet_sepolia_finality_verifier::types::EthereumReceiptInclusionProof
   **/
  PalletSepoliaFinalityVerifierEthereumReceiptInclusionProof: {
    blockNumber: 'u64',
    witness: 'Vec<Bytes>',
    index: 'Bytes'
  },
  /**
   * Lookup432: pallet_sepolia_finality_verifier::types::EthereumEventInclusionProof
   **/
  PalletSepoliaFinalityVerifierEthereumEventInclusionProof: {
    blockNumber: 'u64',
    witness: 'Vec<Bytes>',
    index: 'Bytes',
    event: 'Bytes'
  },
  /**
   * Lookup433: pallet_maintenance_mode::pallet::Call<T>
   **/
  PalletMaintenanceModeCall: {
    _enum: ['enter_maintenance_mode', 'resume_normal_operation']
  },
  /**
   * Lookup434: pallet_sudo::pallet::Call<T>
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
   * Lookup437: pallet_scheduler::pallet::Error<T>
   **/
  PalletSchedulerError: {
    _enum: ['FailedToSchedule', 'NotFound', 'TargetBlockNumberInPast', 'RescheduleNoChange', 'Named']
  },
  /**
   * Lookup438: pallet_utility::pallet::Error<T>
   **/
  PalletUtilityError: {
    _enum: ['TooManyCalls']
  },
  /**
   * Lookup439: pallet_identity::types::Registration<Balance, MaxJudgements, MaxAdditionalFields>
   **/
  PalletIdentityRegistration: {
    judgements: 'Vec<(u32,PalletIdentityJudgement)>',
    deposit: 'u128',
    info: 'PalletIdentityIdentityInfo'
  },
  /**
   * Lookup447: pallet_identity::types::RegistrarInfo<Balance, sp_core::crypto::AccountId32>
   **/
  PalletIdentityRegistrarInfo: {
    account: 'AccountId32',
    fee: 'u128',
    fields: 'PalletIdentityBitFlags'
  },
  /**
   * Lookup449: pallet_identity::pallet::Error<T>
   **/
  PalletIdentityError: {
    _enum: ['TooManySubAccounts', 'NotFound', 'NotNamed', 'EmptyIndex', 'FeeChanged', 'NoIdentity', 'StickyJudgement', 'JudgementGiven', 'InvalidJudgement', 'InvalidIndex', 'InvalidTarget', 'TooManyFields', 'TooManyRegistrars', 'AlreadyClaimed', 'NotSub', 'NotOwned', 'JudgementForDifferentIdentity', 'JudgementPaymentFailed']
  },
  /**
   * Lookup452: pallet_balances::types::BalanceLock<Balance>
   **/
  PalletBalancesBalanceLock: {
    id: '[u8;8]',
    amount: 'u128',
    reasons: 'PalletBalancesReasons'
  },
  /**
   * Lookup453: pallet_balances::types::Reasons
   **/
  PalletBalancesReasons: {
    _enum: ['Fee', 'Misc', 'All']
  },
  /**
   * Lookup456: pallet_balances::types::ReserveData<ReserveIdentifier, Balance>
   **/
  PalletBalancesReserveData: {
    id: '[u8;8]',
    amount: 'u128'
  },
  /**
   * Lookup460: t0rn_parachain_runtime::RuntimeHoldReason
   **/
  T0rnParachainRuntimeRuntimeHoldReason: 'Null',
  /**
   * Lookup463: pallet_balances::types::IdAmount<Id, Balance>
   **/
  PalletBalancesIdAmount: {
    id: 'Null',
    amount: 'u128'
  },
  /**
   * Lookup465: pallet_balances::pallet::Error<T, I>
   **/
  PalletBalancesError: {
    _enum: ['VestingBalance', 'LiquidityRestrictions', 'InsufficientBalance', 'ExistentialDeposit', 'Expendability', 'ExistingVestingSchedule', 'DeadAccount', 'TooManyReserves', 'TooManyHolds', 'TooManyFreezes']
  },
  /**
   * Lookup467: pallet_transaction_payment::Releases
   **/
  PalletTransactionPaymentReleases: {
    _enum: ['V1Ancient', 'V2']
  },
  /**
   * Lookup468: pallet_assets::types::AssetDetails<Balance, sp_core::crypto::AccountId32, DepositBalance>
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
   * Lookup469: pallet_assets::types::AssetStatus
   **/
  PalletAssetsAssetStatus: {
    _enum: ['Live', 'Frozen', 'Destroying']
  },
  /**
   * Lookup471: pallet_assets::types::AssetAccount<Balance, DepositBalance, Extra, sp_core::crypto::AccountId32>
   **/
  PalletAssetsAssetAccount: {
    balance: 'u128',
    status: 'PalletAssetsAccountStatus',
    reason: 'PalletAssetsExistenceReason',
    extra: 'Null'
  },
  /**
   * Lookup472: pallet_assets::types::AccountStatus
   **/
  PalletAssetsAccountStatus: {
    _enum: ['Liquid', 'Frozen', 'Blocked']
  },
  /**
   * Lookup473: pallet_assets::types::ExistenceReason<Balance, sp_core::crypto::AccountId32>
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
   * Lookup475: pallet_assets::types::Approval<Balance, DepositBalance>
   **/
  PalletAssetsApproval: {
    amount: 'u128',
    deposit: 'u128'
  },
  /**
   * Lookup476: pallet_assets::types::AssetMetadata<DepositBalance, bounded_collections::bounded_vec::BoundedVec<T, S>>
   **/
  PalletAssetsAssetMetadata: {
    deposit: 'u128',
    name: 'Bytes',
    symbol: 'Bytes',
    decimals: 'u8',
    isFrozen: 'bool'
  },
  /**
   * Lookup478: pallet_assets::pallet::Error<T, I>
   **/
  PalletAssetsError: {
    _enum: ['BalanceLow', 'NoAccount', 'NoPermission', 'Unknown', 'Frozen', 'InUse', 'BadWitness', 'MinBalanceZero', 'UnavailableConsumer', 'BadMetadata', 'Unapproved', 'WouldDie', 'AlreadyExists', 'NoDeposit', 'WouldBurn', 'LiveAsset', 'AssetNotLive', 'IncorrectStatus', 'NotFrozen', 'CallbackFailed']
  },
  /**
   * Lookup479: t3rn_primitives::account_manager::RequestCharge<sp_core::crypto::AccountId32, Balance, AssetId>
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
   * Lookup481: t3rn_primitives::common::RoundInfo<BlockNumber>
   **/
  T3rnPrimitivesCommonRoundInfo: {
    index: 'u32',
    head: 'u32',
    term: 'u32'
  },
  /**
   * Lookup482: t3rn_primitives::account_manager::Settlement<sp_core::crypto::AccountId32, Balance, AssetId>
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
   * Lookup483: pallet_account_manager::pallet::Error<T>
   **/
  PalletAccountManagerError: {
    _enum: ['PendingChargeNotFoundAtCommit', 'PendingChargeNotFoundAtRefund', 'ExecutionNotRegistered', 'ExecutionAlreadyRegistered', 'SkippingEmptyCharges', 'NoChargeOfGivenIdRegistered', 'ChargeAlreadyRegistered', 'ChargeOrSettlementCalculationOverflow', 'ChargeOrSettlementActualFeesOutgrowReserved', 'DecodingExecutionIDFailed', 'TransferDepositFailedOldChargeNotFound', 'TransferDepositFailedToReleasePreviousCharge']
  },
  /**
   * Lookup484: pallet_treasury::Proposal<sp_core::crypto::AccountId32, Balance>
   **/
  PalletTreasuryProposal: {
    proposer: 'AccountId32',
    value: 'u128',
    beneficiary: 'AccountId32',
    bond: 'u128'
  },
  /**
   * Lookup487: frame_support::PalletId
   **/
  FrameSupportPalletId: '[u8;8]',
  /**
   * Lookup488: pallet_treasury::pallet::Error<T, I>
   **/
  PalletTreasuryError: {
    _enum: ['InsufficientProposersBalance', 'InvalidIndex', 'TooManyApprovals', 'InsufficientPermission', 'ProposalNotApproved']
  },
  /**
   * Lookup495: pallet_collator_selection::pallet::CandidateInfo<sp_core::crypto::AccountId32, Balance>
   **/
  PalletCollatorSelectionCandidateInfo: {
    who: 'AccountId32',
    deposit: 'u128'
  },
  /**
   * Lookup497: pallet_collator_selection::pallet::Error<T>
   **/
  PalletCollatorSelectionError: {
    _enum: ['TooManyCandidates', 'TooFewEligibleCollators', 'AlreadyCandidate', 'NotCandidate', 'TooManyInvulnerables', 'AlreadyInvulnerable', 'NotInvulnerable', 'NoAssociatedValidatorId', 'ValidatorNotRegistered']
  },
  /**
   * Lookup501: sp_core::crypto::KeyTypeId
   **/
  SpCoreCryptoKeyTypeId: '[u8;4]',
  /**
   * Lookup502: pallet_session::pallet::Error<T>
   **/
  PalletSessionError: {
    _enum: ['InvalidProof', 'NoAssociatedValidatorId', 'DuplicatedKey', 'NoKeys', 'NoAccount']
  },
  /**
   * Lookup507: cumulus_pallet_xcmp_queue::InboundChannelDetails
   **/
  CumulusPalletXcmpQueueInboundChannelDetails: {
    sender: 'u32',
    state: 'CumulusPalletXcmpQueueInboundState',
    messageMetadata: 'Vec<(u32,PolkadotParachainPrimitivesXcmpMessageFormat)>'
  },
  /**
   * Lookup508: cumulus_pallet_xcmp_queue::InboundState
   **/
  CumulusPalletXcmpQueueInboundState: {
    _enum: ['Ok', 'Suspended']
  },
  /**
   * Lookup511: polkadot_parachain::primitives::XcmpMessageFormat
   **/
  PolkadotParachainPrimitivesXcmpMessageFormat: {
    _enum: ['ConcatenatedVersionedXcm', 'ConcatenatedEncodedBlob', 'Signals']
  },
  /**
   * Lookup514: cumulus_pallet_xcmp_queue::OutboundChannelDetails
   **/
  CumulusPalletXcmpQueueOutboundChannelDetails: {
    recipient: 'u32',
    state: 'CumulusPalletXcmpQueueOutboundState',
    signalsExist: 'bool',
    firstIndex: 'u16',
    lastIndex: 'u16'
  },
  /**
   * Lookup515: cumulus_pallet_xcmp_queue::OutboundState
   **/
  CumulusPalletXcmpQueueOutboundState: {
    _enum: ['Ok', 'Suspended']
  },
  /**
   * Lookup517: cumulus_pallet_xcmp_queue::QueueConfigData
   **/
  CumulusPalletXcmpQueueQueueConfigData: {
    suspendThreshold: 'u32',
    dropThreshold: 'u32',
    resumeThreshold: 'u32',
    thresholdWeight: 'SpWeightsWeightV2Weight',
    weightRestrictDecay: 'SpWeightsWeightV2Weight',
    xcmpMaxIndividualWeight: 'SpWeightsWeightV2Weight'
  },
  /**
   * Lookup519: cumulus_pallet_xcmp_queue::pallet::Error<T>
   **/
  CumulusPalletXcmpQueueError: {
    _enum: ['FailedToSend', 'BadXcmOrigin', 'BadXcm', 'BadOverweightIndex', 'WeightOverLimit']
  },
  /**
   * Lookup520: pallet_xcm::pallet::QueryStatus<BlockNumber>
   **/
  PalletXcmQueryStatus: {
    _enum: {
      Pending: {
        responder: 'XcmVersionedMultiLocation',
        maybeMatchQuerier: 'Option<XcmVersionedMultiLocation>',
        maybeNotify: 'Option<(u8,u8)>',
        timeout: 'u32',
      },
      VersionNotifier: {
        origin: 'XcmVersionedMultiLocation',
        isActive: 'bool',
      },
      Ready: {
        response: 'XcmVersionedResponse',
        at: 'u32'
      }
    }
  },
  /**
   * Lookup524: xcm::VersionedResponse
   **/
  XcmVersionedResponse: {
    _enum: {
      __Unused0: 'Null',
      __Unused1: 'Null',
      V2: 'XcmV2Response',
      V3: 'XcmV3Response'
    }
  },
  /**
   * Lookup530: pallet_xcm::pallet::VersionMigrationStage
   **/
  PalletXcmVersionMigrationStage: {
    _enum: {
      MigrateSupportedVersion: 'Null',
      MigrateVersionNotifiers: 'Null',
      NotifyCurrentTargets: 'Option<Bytes>',
      MigrateAndNotifyOldTargets: 'Null'
    }
  },
  /**
   * Lookup532: xcm::VersionedAssetId
   **/
  XcmVersionedAssetId: {
    _enum: {
      __Unused0: 'Null',
      __Unused1: 'Null',
      __Unused2: 'Null',
      V3: 'XcmV3MultiassetAssetId'
    }
  },
  /**
   * Lookup533: pallet_xcm::pallet::RemoteLockedFungibleRecord<ConsumerIdentifier, MaxConsumers>
   **/
  PalletXcmRemoteLockedFungibleRecord: {
    amount: 'u128',
    owner: 'XcmVersionedMultiLocation',
    locker: 'XcmVersionedMultiLocation',
    consumers: 'Vec<(Null,u128)>'
  },
  /**
   * Lookup540: pallet_xcm::pallet::Error<T>
   **/
  PalletXcmError: {
    _enum: ['Unreachable', 'SendFailure', 'Filtered', 'UnweighableMessage', 'DestinationNotInvertible', 'Empty', 'CannotReanchor', 'TooManyAssets', 'InvalidOrigin', 'BadVersion', 'BadLocation', 'NoSubscription', 'AlreadySubscribed', 'InvalidAsset', 'LowBalance', 'TooManyLocks', 'AccountNotSovereign', 'FeesNotMet', 'LockNotFound', 'InUse']
  },
  /**
   * Lookup541: cumulus_pallet_xcm::pallet::Error<T>
   **/
  CumulusPalletXcmError: 'Null',
  /**
   * Lookup542: cumulus_pallet_dmp_queue::ConfigData
   **/
  CumulusPalletDmpQueueConfigData: {
    maxIndividual: 'SpWeightsWeightV2Weight'
  },
  /**
   * Lookup543: cumulus_pallet_dmp_queue::PageIndexData
   **/
  CumulusPalletDmpQueuePageIndexData: {
    beginUsed: 'u32',
    endUsed: 'u32',
    overweightCount: 'u64'
  },
  /**
   * Lookup546: cumulus_pallet_dmp_queue::pallet::Error<T>
   **/
  CumulusPalletDmpQueueError: {
    _enum: ['Unknown', 'OverLimit']
  },
  /**
   * Lookup547: pallet_asset_registry::pallet::Error<T>
   **/
  PalletAssetRegistryError: {
    _enum: ['NotFound', 'LocationUnallowed', 'CapabilitiesNotPermitted', 'ShouldntExecuteMessage']
  },
  /**
   * Lookup548: t3rn_abi::sfx_abi::SFXAbi
   **/
  T3rnAbiSfxAbi: {
    argsNames: 'Vec<(Bytes,bool)>',
    maybePrefixMemo: 'Option<u8>',
    egressAbiDescriptors: 'T3rnAbiSfxAbiPerCodecAbiDescriptors',
    ingressAbiDescriptors: 'T3rnAbiSfxAbiPerCodecAbiDescriptors'
  },
  /**
   * Lookup551: t3rn_abi::sfx_abi::PerCodecAbiDescriptors
   **/
  T3rnAbiSfxAbiPerCodecAbiDescriptors: {
    forRlp: 'Bytes',
    forScale: 'Bytes'
  },
  /**
   * Lookup553: t3rn_primitives::xdns::GatewayRecord<sp_core::crypto::AccountId32>
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
   * Lookup555: t3rn_primitives::xdns::TokenRecord
   **/
  T3rnPrimitivesXdnsTokenRecord: {
    tokenId: 'u32',
    gatewayId: '[u8;4]',
    tokenProps: 'T3rnPrimitivesTokenInfo'
  },
  /**
   * Lookup558: t3rn_primitives::GatewayActivity<BlockNumber>
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
   * Lookup561: t3rn_primitives::FinalityVerifierActivity<BlockNumber>
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
   * Lookup563: t3rn_primitives::xdns::EpochEstimate<BlockNumber>
   **/
  T3rnPrimitivesXdnsEpochEstimate: {
    local: 'u32',
    remote: 'u32',
    movingAverageLocal: 'u32',
    movingAverageRemote: 'u32'
  },
  /**
   * Lookup564: pallet_xdns::pallet::Error<T>
   **/
  PalletXdnsError: {
    _enum: ['GatewayRecordAlreadyExists', 'XdnsRecordNotFound', 'EscrowAccountNotFound', 'TokenRecordAlreadyExists', 'TokenRecordNotFoundInAssetsOverlay', 'GatewayRecordNotFound', 'SideEffectABIAlreadyExists', 'SideEffectABINotFound', 'NoParachainInfoFound', 'TokenExecutionVendorMismatch', 'GatewayNotActive']
  },
  /**
   * Lookup565: t3rn_primitives::attesters::AttesterInfo
   **/
  T3rnPrimitivesAttestersAttesterInfo: {
    keyEd: '[u8;32]',
    keyEc: '[u8;33]',
    keySr: '[u8;32]',
    commission: 'Percent',
    index: 'u32'
  },
  /**
   * Lookup572: pallet_attesters::pallet::Error<T>
   **/
  PalletAttestersError: {
    _enum: ['AttesterNotFound', 'ArithmeticOverflow', 'InvalidSignature', 'InvalidMessage', 'InvalidTargetInclusionProof', 'UnexpectedBatchHashRecoveredFromCommitment', 'AlreadyRegistered', 'PublicKeyMissing', 'AttestationSignatureInvalid', 'AttestationDoubleSignAttempt', 'NotActiveSet', 'NotInCurrentCommittee', 'AttesterDidNotAgreeToNewTarget', 'NotRegistered', 'NoNominationFound', 'AlreadyNominated', 'NominatorNotEnoughBalance', 'NominatorBondTooSmall', 'AttesterBondTooSmall', 'MissingNominations', 'BatchHashMismatch', 'BatchNotFound', 'CollusionWithPermanentSlashDetected', 'BatchFoundWithUnsignableStatus', 'RejectingFromSlashedAttester', 'TargetAlreadyActive', 'TargetNotActive', 'XdnsTargetNotActive', 'XdnsGatewayDoesNotHaveEscrowAddressRegistered', 'SfxAlreadyRequested', 'AddAttesterAlreadyRequested', 'RemoveAttesterAlreadyRequested', 'NextCommitteeAlreadyRequested', 'BanAttesterAlreadyRequested', 'BatchAlreadyCommitted', 'CommitteeSizeTooLarge']
  },
  /**
   * Lookup577: pallet_rewards::pallet::AssetType<AssetId>
   **/
  PalletRewardsAssetType: {
    _enum: {
      Native: 'Null',
      NonNative: 'u32'
    }
  },
  /**
   * Lookup578: pallet_rewards::pallet::TreasuryBalanceSheet<Balance>
   **/
  PalletRewardsTreasuryBalanceSheet: {
    treasury: 'u128',
    escrow: 'u128',
    fee: 'u128',
    slash: 'u128',
    parachain: 'u128'
  },
  /**
   * Lookup580: pallet_rewards::pallet::DistributionRecord<BlockNumber, Balance>
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
   * Lookup582: t3rn_primitives::claimable::ClaimableArtifacts<sp_core::crypto::AccountId32, Balance>
   **/
  T3rnPrimitivesClaimableClaimableArtifacts: {
    beneficiary: 'AccountId32',
    role: 'T3rnPrimitivesClaimableCircuitRole',
    totalRoundClaim: 'u128',
    nonNativeAssetId: 'Option<u32>',
    benefitSource: 'T3rnPrimitivesClaimableBenefitSource'
  },
  /**
   * Lookup584: pallet_rewards::pallet::Error<T>
   **/
  PalletRewardsError: {
    _enum: ['DistributionPeriodNotElapsed', 'NoPendingClaims', 'ArithmeticOverflow', 'AttesterNotFound', 'TryIntoConversionU128ToBalanceFailed', 'Halted']
  },
  /**
   * Lookup585: pallet_contracts_registry::pallet::Error<T>
   **/
  PalletContractsRegistryError: {
    _enum: ['ContractAlreadyExists', 'UnknownContract']
  },
  /**
   * Lookup586: t3rn_primitives::circuit::types::XExecSignal<sp_core::crypto::AccountId32, BlockNumber>
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
   * Lookup588: t3rn_primitives::volatile::LocalState
   **/
  T3rnPrimitivesVolatileLocalState: {
    state: 'BTreeMap<[u8;32], Bytes>'
  },
  /**
   * Lookup595: t3rn_sdk_primitives::signal::ExecutionSignal<primitive_types::H256>
   **/
  T3rnSdkPrimitivesSignalExecutionSignal: {
    step: 'u32',
    kind: 'T3rnSdkPrimitivesSignalSignalKind',
    executionId: 'H256'
  },
  /**
   * Lookup597: pallet_circuit::pallet::Error<T>
   **/
  PalletCircuitError: {
    _enum: ['UpdateAttemptDoubleRevert', 'UpdateAttemptDoubleKill', 'UpdateStateTransitionDisallowed', 'UpdateForcedStateTransitionDisallowed', 'UpdateXtxTriggeredWithUnexpectedStatus', 'ConfirmationFailed', 'InvalidOrderOrigin', 'ApplyTriggeredWithUnexpectedStatus', 'BidderNotEnoughBalance', 'RequesterNotEnoughBalance', 'AssetsFailedToWithdraw', 'SanityAfterCreatingSFXDepositsFailed', 'ContractXtxKilledRunOutOfFunds', 'ChargingTransferFailed', 'ChargingTransferFailedAtPendingExecution', 'XtxChargeFailedRequesterBalanceTooLow', 'XtxChargeBondDepositFailedCantAccessBid', 'FinalizeSquareUpFailed', 'CriticalStateSquareUpCalledToFinishWithoutFsxConfirmed', 'RewardTransferFailed', 'RefundTransferFailed', 'SideEffectsValidationFailed', 'InsuranceBondNotRequired', 'BiddingInactive', 'BiddingRejectedBidBelowDust', 'BiddingRejectedBidTooHigh', 'BiddingRejectedInsuranceTooLow', 'BiddingRejectedBetterBidFound', 'BiddingRejectedFailedToDepositBidderBond', 'BiddingFailedExecutorsBalanceTooLowToReserve', 'InsuranceBondAlreadyDeposited', 'InvalidFTXStateEmptyBidForReadyXtx', 'InvalidFTXStateEmptyConfirmationForFinishedXtx', 'InvalidFTXStateUnassignedExecutorForReadySFX', 'InvalidFTXStateIncorrectExecutorForReadySFX', 'GatewayNotActive', 'SetupFailed', 'SetupFailedXtxNotFound', 'SetupFailedXtxStorageArtifactsNotFound', 'SetupFailedIncorrectXtxStatus', 'SetupFailedDuplicatedXtx', 'SetupFailedEmptyXtx', 'SetupFailedXtxAlreadyFinished', 'SetupFailedXtxWasDroppedAtBidding', 'SetupFailedXtxReverted', 'SetupFailedXtxRevertedTimeout', 'XtxDoesNotExist', 'InvalidFSXBidStateLocated', 'EnactSideEffectsCanOnlyBeCalledWithMin1StepFinished', 'FatalXtxTimeoutXtxIdNotMatched', 'RelayEscrowedFailedNothingToConfirm', 'FatalCommitSideEffectWithoutConfirmationAttempt', 'FatalErroredCommitSideEffectConfirmationAttempt', 'FatalErroredRevertSideEffectConfirmationAttempt', 'FailedToHardenFullSideEffect', 'ApplyFailed', 'DeterminedForbiddenXtxStatus', 'SideEffectIsAlreadyScheduledToExecuteOverXBI', 'FSXNotFoundById', 'XtxNotFound', 'LocalSideEffectExecutionNotApplicable', 'LocalExecutionUnauthorized', 'OnLocalTriggerFailedToSetupXtx', 'UnauthorizedCancellation', 'FailedToConvertSFX2XBI', 'FailedToCheckInOverXBI', 'FailedToCreateXBIMetadataDueToWrongAccountConversion', 'FailedToConvertXBIResult2SFXConfirmation', 'FailedToEnterXBIPortal', 'FailedToExitXBIPortal', 'FailedToCommitFSX', 'XBIExitFailedOnSFXConfirmation', 'UnsupportedRole', 'InvalidLocalTrigger', 'SignalQueueFull', 'ArithmeticErrorOverflow', 'ArithmeticErrorUnderflow', 'ArithmeticErrorDivisionByZero']
  },
  /**
   * Lookup598: pallet_clock::pallet::Error<T>
   **/
  PalletClockError: 'Null',
  /**
   * Lookup599: pallet_circuit_vacuum::pallet::Error<T>
   **/
  PalletCircuitVacuumError: 'Null',
  /**
   * Lookup600: pallet_3vm::pallet::Error<T>
   **/
  Pallet3vmError: {
    _enum: ['ExceededSignalBounceThreshold', 'CannotTriggerWithoutSideEffects', 'ContractNotFound', 'InvalidOrigin', 'CannotInstantiateContract', 'ContractCannotRemunerate', 'ContractCannotHaveStorage', 'ContractCannotGenerateSideEffects', 'InvalidPrecompilePointer', 'InvalidPrecompileArgs', 'InvalidArithmeticOverflow', 'DownstreamCircuit']
  },
  /**
   * Lookup602: pallet_contracts::wasm::CodeInfo<T>
   **/
  PalletContractsWasmCodeInfo: {
    owner: 'AccountId32',
    deposit: 'Compact<u128>',
    refcount: 'Compact<u64>',
    determinism: 'PalletContractsWasmDeterminism',
    codeLen: 'u32'
  },
  /**
   * Lookup603: pallet_contracts::storage::ContractInfo<T>
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
   * Lookup605: pallet_contracts::storage::DeletionQueueManager<T>
   **/
  PalletContractsStorageDeletionQueueManager: {
    insertCounter: 'u32',
    deleteCounter: 'u32'
  },
  /**
   * Lookup607: pallet_contracts::schedule::Schedule<T>
   **/
  PalletContractsSchedule: {
    limits: 'PalletContractsScheduleLimits',
    instructionWeights: 'PalletContractsScheduleInstructionWeights',
    hostFnWeights: 'PalletContractsScheduleHostFnWeights'
  },
  /**
   * Lookup608: pallet_contracts::schedule::Limits
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
   * Lookup609: pallet_contracts::schedule::InstructionWeights<T>
   **/
  PalletContractsScheduleInstructionWeights: {
    base: 'u32'
  },
  /**
   * Lookup610: pallet_contracts::schedule::HostFnWeights<T>
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
   * Lookup611: pallet_contracts::pallet::Error<T>
   **/
  PalletContractsError: {
    _enum: ['InvalidSchedule', 'InvalidCallFlags', 'OutOfGas', 'OutputBufferTooSmall', 'TransferFailed', 'MaxCallDepthReached', 'ContractNotFound', 'CodeTooLarge', 'CodeNotFound', 'CodeInfoNotFound', 'OutOfBounds', 'DecodingFailed', 'ContractTrapped', 'ValueTooLarge', 'TerminatedWhileReentrant', 'InputForwarded', 'RandomSubjectTooLong', 'TooManyTopics', 'NoChainExtension', 'DuplicateContract', 'TerminatedInConstructor', 'ReentranceDenied', 'StorageDepositNotEnoughFunds', 'StorageDepositLimitExhausted', 'CodeInUse', 'ContractReverted', 'CodeRejected', 'Indeterministic', 'MigrationInProgress', 'NoMigrationPerformed']
  },
  /**
   * Lookup612: pallet_evm::CodeMetadata
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
   * Lookup614: pallet_evm::pallet::Error<T>
   **/
  PalletEvmError: {
    _enum: ['BalanceLow', 'FeeOverflow', 'PaymentOverflow', 'WithdrawFailed', 'GasPriceTooLow', 'InvalidNonce', 'GasLimitTooLow', 'GasLimitTooHigh', 'Undefined', 'Reentrancy', 'TransactionMustComeFromEOA']
  },
  /**
   * Lookup615: pallet_portal::pallet::Error<T>
   **/
  PalletPortalError: {
    _enum: ['XdnsRecordCreationFailed', 'UnimplementedGatewayVendor', 'LightClientNotFoundByVendor', 'RegistrationError', 'GatewayVendorNotFound', 'SetOwnerError', 'SetOperationalError', 'SubmitHeaderError', 'NoGatewayHeightAvailable', 'SideEffectConfirmationFailed', 'SFXRecodeError']
  },
  /**
   * Lookup616: pallet_grandpa_finality_verifier::bridges::header_chain::AuthoritySet
   **/
  PalletGrandpaFinalityVerifierBridgesHeaderChainAuthoritySet: {
    authorities: 'Vec<(SpConsensusGrandpaAppPublic,u64)>',
    setId: 'u64'
  },
  /**
   * Lookup619: pallet_grandpa_finality_verifier::types::ParachainRegistrationData
   **/
  PalletGrandpaFinalityVerifierParachainRegistrationData: {
    relayGatewayId: '[u8;4]',
    id: 'u32'
  },
  /**
   * Lookup620: pallet_grandpa_finality_verifier::pallet::Error<T, I>
   **/
  PalletGrandpaFinalityVerifierError: {
    _enum: ['EmptyRangeSubmitted', 'RangeToLarge', 'NoFinalizedHeader', 'InvalidAuthoritySet', 'InvalidGrandpaJustification', 'InvalidRangeLinkage', 'InvalidJustificationLinkage', 'ParachainEntryNotFound', 'StorageRootNotFound', 'InclusionDataDecodeError', 'InvalidStorageProof', 'EventNotIncluded', 'HeaderDecodingError', 'HeaderDataDecodingError', 'StorageRootMismatch', 'UnknownHeader', 'UnexpectedEventLength', 'UnexpectedSource', 'EventDecodingFailed', 'UnkownSideEffect', 'UnsupportedScheduledChange', 'Halted', 'BlockHeightConversionError', 'InvalidPayloadSource', 'InvalidSourceFormat']
  },
  /**
   * Lookup623: pallet_eth2_finality_verifier::types::Checkpoint
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
   * Lookup624: pallet_eth2_finality_verifier::types::BeaconCheckpoint
   **/
  PalletEth2FinalityVerifierBeaconCheckpoint: {
    epoch: 'u64',
    root: '[u8;32]'
  },
  /**
   * Lookup625: pallet_eth2_finality_verifier::types::ExecutionCheckpoint
   **/
  PalletEth2FinalityVerifierExecutionCheckpoint: {
    height: 'u64',
    root: '[u8;32]'
  },
  /**
   * Lookup626: pallet_eth2_finality_verifier::pallet::Error<T>
   **/
  PalletEth2FinalityVerifierError: {
    _enum: ['Halted', 'AlreadyInitialized', 'InvalidInitializationData', 'SSZForkDataHashTreeRootFailed', 'SSZSigningDataHashTreeRootFailed', 'BLSPubkeyAggregationFaild', 'InvalidBLSPublicKeyUsedForVerification', 'InvalidInclusionProof', 'ForkNotDetected', 'ValidSyncCommitteeNotAvailable', 'SubmittedHeaderToOld', 'InvalidBLSSignature', 'InvalidMerkleProof', 'BeaconHeaderHashTreeRootFailed', 'BeaconHeaderNotFound', 'BeaconHeaderNotFinalized', 'ExecutionHeaderHashTreeRootFailed', 'InvalidExecutionRangeLinkage', 'InvalidExecutionRange', 'SyncCommitteeParticipantsNotSupermajority', 'SyncCommitteeInvalid', 'NotPeriodsFirstEpoch', 'InvalidCheckpoint', 'ExecutionHeaderNotFound', 'EventNotInReceipt', 'InvalidEncodedEpochUpdate', 'InvalidSyncCommitteePeriod', 'MathError', 'CurrentSyncCommitteePeriodNotAvailable', 'BeaconCheckpointHashTreeRootFailed', 'InvalidFork', 'ExecutionHeaderNotFinalized', 'InvalidBeaconLinkage', 'InvalidExecutionPayload', 'InvalidSourceAddress']
  },
  /**
   * Lookup627: pallet_sepolia_finality_verifier::types::Checkpoint
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
   * Lookup628: pallet_sepolia_finality_verifier::types::BeaconCheckpoint
   **/
  PalletSepoliaFinalityVerifierBeaconCheckpoint: {
    epoch: 'u64',
    root: '[u8;32]'
  },
  /**
   * Lookup629: pallet_sepolia_finality_verifier::types::ExecutionCheckpoint
   **/
  PalletSepoliaFinalityVerifierExecutionCheckpoint: {
    height: 'u64',
    root: '[u8;32]'
  },
  /**
   * Lookup630: pallet_sepolia_finality_verifier::pallet::Error<T>
   **/
  PalletSepoliaFinalityVerifierError: {
    _enum: ['Halted', 'AlreadyInitialized', 'InvalidInitializationData', 'SSZForkDataHashTreeRootFailed', 'SSZSigningDataHashTreeRootFailed', 'BLSPubkeyAggregationFaild', 'InvalidBLSPublicKeyUsedForVerification', 'InvalidInclusionProof', 'ForkNotDetected', 'ValidSyncCommitteeNotAvailable', 'SubmittedHeaderToOld', 'InvalidBLSSignature', 'InvalidMerkleProof', 'BeaconHeaderHashTreeRootFailed', 'BeaconHeaderNotFound', 'BeaconHeaderNotFinalized', 'ExecutionHeaderHashTreeRootFailed', 'InvalidExecutionRangeLinkage', 'InvalidExecutionRange', 'SyncCommitteeParticipantsNotSupermajority', 'SyncCommitteeInvalid', 'NotPeriodsFirstEpoch', 'InvalidCheckpoint', 'ExecutionHeaderNotFound', 'EventNotInReceipt', 'InvalidEncodedEpochUpdate', 'InvalidSyncCommitteePeriod', 'MathError', 'CurrentSyncCommitteePeriodNotAvailable', 'BeaconCheckpointHashTreeRootFailed', 'InvalidFork', 'ExecutionHeaderNotFinalized', 'InvalidBeaconLinkage', 'InvalidExecutionPayload', 'InvalidSourceAddress']
  },
  /**
   * Lookup631: pallet_maintenance_mode::pallet::Error<T>
   **/
  PalletMaintenanceModeError: {
    _enum: ['AlreadyInMaintenanceMode', 'NotInMaintenanceMode']
  },
  /**
   * Lookup632: pallet_sudo::pallet::Error<T>
   **/
  PalletSudoError: {
    _enum: ['RequireSudo']
  },
  /**
   * Lookup634: sp_runtime::MultiSignature
   **/
  SpRuntimeMultiSignature: {
    _enum: {
      Ed25519: 'SpCoreEd25519Signature',
      Sr25519: 'SpCoreSr25519Signature',
      Ecdsa: 'SpCoreEcdsaSignature'
    }
  },
  /**
   * Lookup635: sp_core::sr25519::Signature
   **/
  SpCoreSr25519Signature: '[u8;64]',
  /**
   * Lookup636: sp_core::ecdsa::Signature
   **/
  SpCoreEcdsaSignature: '[u8;65]',
  /**
   * Lookup638: frame_system::extensions::check_non_zero_sender::CheckNonZeroSender<T>
   **/
  FrameSystemExtensionsCheckNonZeroSender: 'Null',
  /**
   * Lookup639: frame_system::extensions::check_spec_version::CheckSpecVersion<T>
   **/
  FrameSystemExtensionsCheckSpecVersion: 'Null',
  /**
   * Lookup640: frame_system::extensions::check_tx_version::CheckTxVersion<T>
   **/
  FrameSystemExtensionsCheckTxVersion: 'Null',
  /**
   * Lookup641: frame_system::extensions::check_genesis::CheckGenesis<T>
   **/
  FrameSystemExtensionsCheckGenesis: 'Null',
  /**
   * Lookup644: frame_system::extensions::check_nonce::CheckNonce<T>
   **/
  FrameSystemExtensionsCheckNonce: 'Compact<u32>',
  /**
   * Lookup645: frame_system::extensions::check_weight::CheckWeight<T>
   **/
  FrameSystemExtensionsCheckWeight: 'Null',
  /**
   * Lookup646: pallet_asset_tx_payment::ChargeAssetTxPayment<T>
   **/
  PalletAssetTxPaymentChargeAssetTxPayment: {
    tip: 'Compact<u128>',
    assetId: 'Option<u32>'
  }
};
