declare const _default: {
    /**
     * Lookup3: frame_system::AccountInfo<Index, pallet_balances::AccountData<Balance>>
     **/
    FrameSystemAccountInfo: {
        nonce: string;
        consumers: string;
        providers: string;
        sufficients: string;
        data: string;
    };
    /**
     * Lookup5: pallet_balances::AccountData<Balance>
     **/
    PalletBalancesAccountData: {
        free: string;
        reserved: string;
        miscFrozen: string;
        feeFrozen: string;
    };
    /**
     * Lookup7: frame_support::weights::PerDispatchClass<T>
     **/
    FrameSupportWeightsPerDispatchClassU64: {
        normal: string;
        operational: string;
        mandatory: string;
    };
    /**
     * Lookup11: sp_runtime::generic::digest::Digest
     **/
    SpRuntimeDigest: {
        logs: string;
    };
    /**
     * Lookup13: sp_runtime::generic::digest::DigestItem
     **/
    SpRuntimeDigestDigestItem: {
        _enum: {
            Other: string;
            __Unused1: string;
            __Unused2: string;
            __Unused3: string;
            Consensus: string;
            Seal: string;
            PreRuntime: string;
            __Unused7: string;
            RuntimeEnvironmentUpdated: string;
        };
    };
    /**
     * Lookup16: frame_system::EventRecord<circuit_standalone_runtime::Event, primitive_types::H256>
     **/
    FrameSystemEventRecord: {
        phase: string;
        event: string;
        topics: string;
    };
    /**
     * Lookup18: frame_system::pallet::Event<T>
     **/
    FrameSystemEvent: {
        _enum: {
            ExtrinsicSuccess: {
                dispatchInfo: string;
            };
            ExtrinsicFailed: {
                dispatchError: string;
                dispatchInfo: string;
            };
            CodeUpdated: string;
            NewAccount: {
                account: string;
            };
            KilledAccount: {
                account: string;
            };
            Remarked: {
                _alias: {
                    hash_: string;
                };
                sender: string;
                hash_: string;
            };
        };
    };
    /**
     * Lookup19: frame_support::weights::DispatchInfo
     **/
    FrameSupportWeightsDispatchInfo: {
        weight: string;
        class: string;
        paysFee: string;
    };
    /**
     * Lookup20: frame_support::weights::DispatchClass
     **/
    FrameSupportWeightsDispatchClass: {
        _enum: string[];
    };
    /**
     * Lookup21: frame_support::weights::Pays
     **/
    FrameSupportWeightsPays: {
        _enum: string[];
    };
    /**
     * Lookup22: sp_runtime::DispatchError
     **/
    SpRuntimeDispatchError: {
        _enum: {
            Other: string;
            CannotLookup: string;
            BadOrigin: string;
            Module: string;
            ConsumerRemaining: string;
            NoProviders: string;
            TooManyConsumers: string;
            Token: string;
            Arithmetic: string;
            Transactional: string;
        };
    };
    /**
     * Lookup23: sp_runtime::ModuleError
     **/
    SpRuntimeModuleError: {
        index: string;
        error: string;
    };
    /**
     * Lookup24: sp_runtime::TokenError
     **/
    SpRuntimeTokenError: {
        _enum: string[];
    };
    /**
     * Lookup25: sp_runtime::ArithmeticError
     **/
    SpRuntimeArithmeticError: {
        _enum: string[];
    };
    /**
     * Lookup26: sp_runtime::TransactionalError
     **/
    SpRuntimeTransactionalError: {
        _enum: string[];
    };
    /**
     * Lookup27: pallet_grandpa::pallet::Event
     **/
    PalletGrandpaEvent: {
        _enum: {
            NewAuthorities: {
                authoritySet: string;
            };
            Paused: string;
            Resumed: string;
        };
    };
    /**
     * Lookup30: sp_finality_grandpa::app::Public
     **/
    SpFinalityGrandpaAppPublic: string;
    /**
     * Lookup31: sp_core::ed25519::Public
     **/
    SpCoreEd25519Public: string;
    /**
     * Lookup32: pallet_sudo::pallet::Event<T>
     **/
    PalletSudoEvent: {
        _enum: {
            Sudid: {
                sudoResult: string;
            };
            KeyChanged: {
                oldSudoer: string;
            };
            SudoAsDone: {
                sudoResult: string;
            };
        };
    };
    /**
     * Lookup36: pallet_utility::pallet::Event
     **/
    PalletUtilityEvent: {
        _enum: {
            BatchInterrupted: {
                index: string;
                error: string;
            };
            BatchCompleted: string;
            BatchCompletedWithErrors: string;
            ItemCompleted: string;
            ItemFailed: {
                error: string;
            };
            DispatchedAs: {
                result: string;
            };
        };
    };
    /**
     * Lookup37: pallet_identity::pallet::Event<T>
     **/
    PalletIdentityEvent: {
        _enum: {
            IdentitySet: {
                who: string;
            };
            IdentityCleared: {
                who: string;
                deposit: string;
            };
            IdentityKilled: {
                who: string;
                deposit: string;
            };
            JudgementRequested: {
                who: string;
                registrarIndex: string;
            };
            JudgementUnrequested: {
                who: string;
                registrarIndex: string;
            };
            JudgementGiven: {
                target: string;
                registrarIndex: string;
            };
            RegistrarAdded: {
                registrarIndex: string;
            };
            SubIdentityAdded: {
                sub: string;
                main: string;
                deposit: string;
            };
            SubIdentityRemoved: {
                sub: string;
                main: string;
                deposit: string;
            };
            SubIdentityRevoked: {
                sub: string;
                main: string;
                deposit: string;
            };
        };
    };
    /**
     * Lookup38: pallet_balances::pallet::Event<T, I>
     **/
    PalletBalancesEvent: {
        _enum: {
            Endowed: {
                account: string;
                freeBalance: string;
            };
            DustLost: {
                account: string;
                amount: string;
            };
            Transfer: {
                from: string;
                to: string;
                amount: string;
            };
            BalanceSet: {
                who: string;
                free: string;
                reserved: string;
            };
            Reserved: {
                who: string;
                amount: string;
            };
            Unreserved: {
                who: string;
                amount: string;
            };
            ReserveRepatriated: {
                from: string;
                to: string;
                amount: string;
                destinationStatus: string;
            };
            Deposit: {
                who: string;
                amount: string;
            };
            Withdraw: {
                who: string;
                amount: string;
            };
            Slashed: {
                who: string;
                amount: string;
            };
        };
    };
    /**
     * Lookup39: frame_support::traits::tokens::misc::BalanceStatus
     **/
    FrameSupportTokensMiscBalanceStatus: {
        _enum: string[];
    };
    /**
     * Lookup40: pallet_transaction_payment::pallet::Event<T>
     **/
    PalletTransactionPaymentEvent: {
        _enum: {
            TransactionFeePaid: {
                who: string;
                actualFee: string;
                tip: string;
            };
        };
    };
    /**
     * Lookup41: pallet_treasury::pallet::Event<T, I>
     **/
    PalletTreasuryEvent: {
        _enum: {
            Proposed: {
                proposalIndex: string;
            };
            Spending: {
                budgetRemaining: string;
            };
            Awarded: {
                proposalIndex: string;
                award: string;
                account: string;
            };
            Rejected: {
                proposalIndex: string;
                slashed: string;
            };
            Burnt: {
                burntFunds: string;
            };
            Rollover: {
                rolloverBalance: string;
            };
            Deposit: {
                value: string;
            };
            SpendApproved: {
                proposalIndex: string;
                amount: string;
                beneficiary: string;
            };
        };
    };
    /**
     * Lookup42: pallet_assets::pallet::Event<T, I>
     **/
    PalletAssetsEvent: {
        _enum: {
            Created: {
                assetId: string;
                creator: string;
                owner: string;
            };
            Issued: {
                assetId: string;
                owner: string;
                totalSupply: string;
            };
            Transferred: {
                assetId: string;
                from: string;
                to: string;
                amount: string;
            };
            Burned: {
                assetId: string;
                owner: string;
                balance: string;
            };
            TeamChanged: {
                assetId: string;
                issuer: string;
                admin: string;
                freezer: string;
            };
            OwnerChanged: {
                assetId: string;
                owner: string;
            };
            Frozen: {
                assetId: string;
                who: string;
            };
            Thawed: {
                assetId: string;
                who: string;
            };
            AssetFrozen: {
                assetId: string;
            };
            AssetThawed: {
                assetId: string;
            };
            Destroyed: {
                assetId: string;
            };
            ForceCreated: {
                assetId: string;
                owner: string;
            };
            MetadataSet: {
                assetId: string;
                name: string;
                symbol: string;
                decimals: string;
                isFrozen: string;
            };
            MetadataCleared: {
                assetId: string;
            };
            ApprovedTransfer: {
                assetId: string;
                source: string;
                delegate: string;
                amount: string;
            };
            ApprovalCancelled: {
                assetId: string;
                owner: string;
                delegate: string;
            };
            TransferredApproved: {
                assetId: string;
                owner: string;
                delegate: string;
                destination: string;
                amount: string;
            };
            AssetStatusChanged: {
                assetId: string;
            };
        };
    };
    /**
     * Lookup44: pallet_xdns::pallet::Event<T>
     **/
    PalletXdnsEvent: {
        _enum: {
            XdnsRecordStored: string;
            XdnsRecordPurged: string;
            XdnsRecordUpdated: string;
        };
    };
    /**
     * Lookup45: pallet_contracts_registry::pallet::Event<T>
     **/
    PalletContractsRegistryEvent: {
        _enum: {
            ContractStored: string;
            ContractPurged: string;
        };
    };
    /**
     * Lookup46: pallet_circuit::pallet::Event<T>
     **/
    PalletCircuitEvent: {
        _enum: {
            Transfer: string;
            TransferAssets: string;
            TransferORML: string;
            AddLiquidity: string;
            Swap: string;
            CallNative: string;
            CallEvm: string;
            CallWasm: string;
            CallCustom: string;
            Result: string;
            XTransactionReceivedForExec: string;
            SFXNewBidReceived: string;
            SideEffectConfirmed: string;
            XTransactionReadyForExec: string;
            XTransactionStepFinishedExec: string;
            XTransactionXtxFinishedExecAllSteps: string;
            XTransactionXtxRevertedAfterTimeOut: string;
            XTransactionXtxDroppedAtBidding: string;
            NewSideEffectsAvailable: string;
            CancelledSideEffects: string;
            SideEffectsConfirmed: string;
            EscrowTransfer: string;
        };
    };
    /**
     * Lookup56: xbi_format::XbiCheckOutStatus
     **/
    XbiFormatXbiCheckOutStatus: {
        _enum: string[];
    };
    /**
     * Lookup58: t3rn_types::sfx::SideEffect<sp_core::crypto::AccountId32, BalanceOf>
     **/
    T3rnTypesSfxSideEffect: {
        target: string;
        maxReward: string;
        insurance: string;
        encodedAction: string;
        encodedArgs: string;
        signature: string;
        enforceExecutor: string;
        rewardAssetId: string;
    };
    /**
     * Lookup63: t3rn_types::fsx::FullSideEffect<sp_core::crypto::AccountId32, BlockNumber, BalanceOf>
     **/
    T3rnTypesFsxFullSideEffect: {
        input: string;
        confirmed: string;
        securityLvl: string;
        submissionTargetHeight: string;
        bestBid: string;
        index: string;
    };
    /**
     * Lookup65: t3rn_types::sfx::ConfirmedSideEffect<sp_core::crypto::AccountId32, BlockNumber, BalanceOf>
     **/
    T3rnTypesSfxConfirmedSideEffect: {
        err: string;
        output: string;
        inclusionData: string;
        executioner: string;
        receivedAt: string;
        cost: string;
    };
    /**
     * Lookup67: t3rn_types::sfx::ConfirmationOutcome
     **/
    T3rnTypesSfxConfirmationOutcome: {
        _enum: {
            Success: string;
            MisbehaviourMalformedValues: {
                key: string;
                expected: string;
                received: string;
            };
            TimedOut: string;
        };
    };
    /**
     * Lookup69: t3rn_types::sfx::SecurityLvl
     **/
    T3rnTypesSfxSecurityLvl: {
        _enum: string[];
    };
    /**
     * Lookup71: t3rn_types::bid::SFXBid<sp_core::crypto::AccountId32, BalanceOf, AssetId>
     **/
    T3rnTypesBidSfxBid: {
        amount: string;
        insurance: string;
        reservedBond: string;
        rewardAssetId: string;
        executor: string;
        requester: string;
    };
    /**
     * Lookup72: pallet_clock::pallet::Event<T>
     **/
    PalletClockEvent: {
        _enum: {
            NewRound: {
                index: string;
                head: string;
                term: string;
            };
        };
    };
    /**
     * Lookup73: pallet_3vm::pallet::Event<T>
     **/
    Pallet3vmEvent: {
        _enum: {
            SignalBounced: string;
            ExceededBounceThrehold: string;
            ModuleInstantiated: string;
            AuthorStored: string;
            AuthorRemoved: string;
        };
    };
    /**
     * Lookup75: t3rn_sdk_primitives::signal::SignalKind
     **/
    T3rnSdkPrimitivesSignalSignalKind: {
        _enum: {
            Complete: string;
            Kill: string;
        };
    };
    /**
     * Lookup76: t3rn_sdk_primitives::signal::KillReason
     **/
    T3rnSdkPrimitivesSignalKillReason: {
        _enum: string[];
    };
    /**
     * Lookup78: t3rn_primitives::contract_metadata::ContractType
     **/
    T3rnPrimitivesContractMetadataContractType: {
        _enum: string[];
    };
    /**
     * Lookup80: pallet_contracts::pallet::Event<T>
     **/
    PalletContractsEvent: {
        _enum: {
            Instantiated: {
                deployer: string;
                contract: string;
            };
            Terminated: {
                contract: string;
                beneficiary: string;
            };
            CodeStored: {
                codeHash: string;
            };
            ContractEmitted: {
                contract: string;
                data: string;
            };
            CodeRemoved: {
                codeHash: string;
            };
            ContractCodeUpdated: {
                contract: string;
                newCodeHash: string;
                oldCodeHash: string;
            };
        };
    };
    /**
     * Lookup81: pallet_evm::pallet::Event<T>
     **/
    PalletEvmEvent: {
        _enum: {
            Log: string;
            Created: string;
            CreatedFailed: string;
            Executed: string;
            ExecutedFailed: string;
            BalanceDeposit: string;
            BalanceWithdraw: string;
            ClaimAccount: {
                accountId: string;
                evmAddress: string;
            };
        };
    };
    /**
     * Lookup82: ethereum::log::Log
     **/
    EthereumLog: {
        address: string;
        topics: string;
        data: string;
    };
    /**
     * Lookup83: pallet_account_manager::pallet::Event<T>
     **/
    PalletAccountManagerEvent: {
        _enum: {
            ContractsRegistryExecutionFinalized: {
                executionId: string;
            };
            Issued: {
                recipient: string;
                amount: string;
            };
            DepositReceived: {
                chargeId: string;
                payee: string;
                recipient: string;
                amount: string;
            };
        };
    };
    /**
     * Lookup84: pallet_portal::pallet::Event<T>
     **/
    PalletPortalEvent: {
        _enum: {
            GatewayRegistered: string;
            SetOwner: string;
            SetOperational: string;
            HeaderSubmitted: string;
        };
    };
    /**
     * Lookup85: frame_system::Phase
     **/
    FrameSystemPhase: {
        _enum: {
            ApplyExtrinsic: string;
            Finalization: string;
            Initialization: string;
        };
    };
    /**
     * Lookup88: frame_system::LastRuntimeUpgradeInfo
     **/
    FrameSystemLastRuntimeUpgradeInfo: {
        specVersion: string;
        specName: string;
    };
    /**
     * Lookup91: frame_system::pallet::Call<T>
     **/
    FrameSystemCall: {
        _enum: {
            fill_block: {
                ratio: string;
            };
            remark: {
                remark: string;
            };
            set_heap_pages: {
                pages: string;
            };
            set_code: {
                code: string;
            };
            set_code_without_checks: {
                code: string;
            };
            set_storage: {
                items: string;
            };
            kill_storage: {
                _alias: {
                    keys_: string;
                };
                keys_: string;
            };
            kill_prefix: {
                prefix: string;
                subkeys: string;
            };
            remark_with_event: {
                remark: string;
            };
        };
    };
    /**
     * Lookup95: frame_system::limits::BlockWeights
     **/
    FrameSystemLimitsBlockWeights: {
        baseBlock: string;
        maxBlock: string;
        perClass: string;
    };
    /**
     * Lookup96: frame_support::weights::PerDispatchClass<frame_system::limits::WeightsPerClass>
     **/
    FrameSupportWeightsPerDispatchClassWeightsPerClass: {
        normal: string;
        operational: string;
        mandatory: string;
    };
    /**
     * Lookup97: frame_system::limits::WeightsPerClass
     **/
    FrameSystemLimitsWeightsPerClass: {
        baseExtrinsic: string;
        maxExtrinsic: string;
        maxTotal: string;
        reserved: string;
    };
    /**
     * Lookup99: frame_system::limits::BlockLength
     **/
    FrameSystemLimitsBlockLength: {
        max: string;
    };
    /**
     * Lookup100: frame_support::weights::PerDispatchClass<T>
     **/
    FrameSupportWeightsPerDispatchClassU32: {
        normal: string;
        operational: string;
        mandatory: string;
    };
    /**
     * Lookup101: frame_support::weights::RuntimeDbWeight
     **/
    FrameSupportWeightsRuntimeDbWeight: {
        read: string;
        write: string;
    };
    /**
     * Lookup102: sp_version::RuntimeVersion
     **/
    SpVersionRuntimeVersion: {
        specName: string;
        implName: string;
        authoringVersion: string;
        specVersion: string;
        implVersion: string;
        apis: string;
        transactionVersion: string;
        stateVersion: string;
    };
    /**
     * Lookup108: frame_system::pallet::Error<T>
     **/
    FrameSystemError: {
        _enum: string[];
    };
    /**
     * Lookup110: pallet_timestamp::pallet::Call<T>
     **/
    PalletTimestampCall: {
        _enum: {
            set: {
                now: string;
            };
        };
    };
    /**
     * Lookup113: sp_consensus_aura::sr25519::app_sr25519::Public
     **/
    SpConsensusAuraSr25519AppSr25519Public: string;
    /**
     * Lookup114: sp_core::sr25519::Public
     **/
    SpCoreSr25519Public: string;
    /**
     * Lookup117: pallet_grandpa::StoredState<N>
     **/
    PalletGrandpaStoredState: {
        _enum: {
            Live: string;
            PendingPause: {
                scheduledAt: string;
                delay: string;
            };
            Paused: string;
            PendingResume: {
                scheduledAt: string;
                delay: string;
            };
        };
    };
    /**
     * Lookup118: pallet_grandpa::StoredPendingChange<N, Limit>
     **/
    PalletGrandpaStoredPendingChange: {
        scheduledAt: string;
        delay: string;
        nextAuthorities: string;
        forced: string;
    };
    /**
     * Lookup120: pallet_grandpa::pallet::Call<T>
     **/
    PalletGrandpaCall: {
        _enum: {
            report_equivocation: {
                equivocationProof: string;
                keyOwnerProof: string;
            };
            report_equivocation_unsigned: {
                equivocationProof: string;
                keyOwnerProof: string;
            };
            note_stalled: {
                delay: string;
                bestFinalizedBlockNumber: string;
            };
        };
    };
    /**
     * Lookup121: sp_finality_grandpa::EquivocationProof<primitive_types::H256, N>
     **/
    SpFinalityGrandpaEquivocationProof: {
        setId: string;
        equivocation: string;
    };
    /**
     * Lookup122: sp_finality_grandpa::Equivocation<primitive_types::H256, N>
     **/
    SpFinalityGrandpaEquivocation: {
        _enum: {
            Prevote: string;
            Precommit: string;
        };
    };
    /**
     * Lookup123: finality_grandpa::Equivocation<sp_finality_grandpa::app::Public, finality_grandpa::Prevote<primitive_types::H256, N>, sp_finality_grandpa::app::Signature>
     **/
    FinalityGrandpaEquivocationPrevote: {
        roundNumber: string;
        identity: string;
        first: string;
        second: string;
    };
    /**
     * Lookup124: finality_grandpa::Prevote<primitive_types::H256, N>
     **/
    FinalityGrandpaPrevote: {
        targetHash: string;
        targetNumber: string;
    };
    /**
     * Lookup125: sp_finality_grandpa::app::Signature
     **/
    SpFinalityGrandpaAppSignature: string;
    /**
     * Lookup126: sp_core::ed25519::Signature
     **/
    SpCoreEd25519Signature: string;
    /**
     * Lookup129: finality_grandpa::Equivocation<sp_finality_grandpa::app::Public, finality_grandpa::Precommit<primitive_types::H256, N>, sp_finality_grandpa::app::Signature>
     **/
    FinalityGrandpaEquivocationPrecommit: {
        roundNumber: string;
        identity: string;
        first: string;
        second: string;
    };
    /**
     * Lookup130: finality_grandpa::Precommit<primitive_types::H256, N>
     **/
    FinalityGrandpaPrecommit: {
        targetHash: string;
        targetNumber: string;
    };
    /**
     * Lookup132: sp_core::Void
     **/
    SpCoreVoid: string;
    /**
     * Lookup133: pallet_grandpa::pallet::Error<T>
     **/
    PalletGrandpaError: {
        _enum: string[];
    };
    /**
     * Lookup134: pallet_sudo::pallet::Call<T>
     **/
    PalletSudoCall: {
        _enum: {
            sudo: {
                call: string;
            };
            sudo_unchecked_weight: {
                call: string;
                weight: string;
            };
            set_key: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            sudo_as: {
                who: string;
                call: string;
            };
        };
    };
    /**
     * Lookup136: pallet_utility::pallet::Call<T>
     **/
    PalletUtilityCall: {
        _enum: {
            batch: {
                calls: string;
            };
            as_derivative: {
                index: string;
                call: string;
            };
            batch_all: {
                calls: string;
            };
            dispatch_as: {
                asOrigin: string;
                call: string;
            };
            force_batch: {
                calls: string;
            };
        };
    };
    /**
     * Lookup138: circuit_standalone_runtime::OriginCaller
     **/
    CircuitStandaloneRuntimeOriginCaller: {
        _enum: {
            system: string;
            Void: string;
        };
    };
    /**
     * Lookup139: frame_support::dispatch::RawOrigin<sp_core::crypto::AccountId32>
     **/
    FrameSupportDispatchRawOrigin: {
        _enum: {
            Root: string;
            Signed: string;
            None: string;
        };
    };
    /**
     * Lookup140: pallet_identity::pallet::Call<T>
     **/
    PalletIdentityCall: {
        _enum: {
            add_registrar: {
                account: string;
            };
            set_identity: {
                info: string;
            };
            set_subs: {
                subs: string;
            };
            clear_identity: string;
            request_judgement: {
                regIndex: string;
                maxFee: string;
            };
            cancel_request: {
                regIndex: string;
            };
            set_fee: {
                index: string;
                fee: string;
            };
            set_account_id: {
                _alias: {
                    new_: string;
                };
                index: string;
                new_: string;
            };
            set_fields: {
                index: string;
                fields: string;
            };
            provide_judgement: {
                regIndex: string;
                target: string;
                judgement: string;
            };
            kill_identity: {
                target: string;
            };
            add_sub: {
                sub: string;
                data: string;
            };
            rename_sub: {
                sub: string;
                data: string;
            };
            remove_sub: {
                sub: string;
            };
            quit_sub: string;
        };
    };
    /**
     * Lookup141: pallet_identity::types::IdentityInfo<FieldLimit>
     **/
    PalletIdentityIdentityInfo: {
        additional: string;
        display: string;
        legal: string;
        web: string;
        riot: string;
        email: string;
        pgpFingerprint: string;
        image: string;
        twitter: string;
    };
    /**
     * Lookup179: pallet_identity::types::BitFlags<pallet_identity::types::IdentityField>
     **/
    PalletIdentityBitFlags: {
        _bitLength: number;
        Display: number;
        Legal: number;
        Web: number;
        Riot: number;
        Email: number;
        PgpFingerprint: number;
        Image: number;
        Twitter: number;
    };
    /**
     * Lookup180: pallet_identity::types::IdentityField
     **/
    PalletIdentityIdentityField: {
        _enum: string[];
    };
    /**
     * Lookup183: pallet_identity::types::Judgement<Balance>
     **/
    PalletIdentityJudgement: {
        _enum: {
            Unknown: string;
            FeePaid: string;
            Reasonable: string;
            KnownGood: string;
            OutOfDate: string;
            LowQuality: string;
            Erroneous: string;
        };
    };
    /**
     * Lookup184: pallet_balances::pallet::Call<T, I>
     **/
    PalletBalancesCall: {
        _enum: {
            transfer: {
                dest: string;
                value: string;
            };
            set_balance: {
                who: string;
                newFree: string;
                newReserved: string;
            };
            force_transfer: {
                source: string;
                dest: string;
                value: string;
            };
            transfer_keep_alive: {
                dest: string;
                value: string;
            };
            transfer_all: {
                dest: string;
                keepAlive: string;
            };
            force_unreserve: {
                who: string;
                amount: string;
            };
        };
    };
    /**
     * Lookup185: pallet_treasury::pallet::Call<T, I>
     **/
    PalletTreasuryCall: {
        _enum: {
            propose_spend: {
                value: string;
                beneficiary: string;
            };
            reject_proposal: {
                proposalId: string;
            };
            approve_proposal: {
                proposalId: string;
            };
            spend: {
                amount: string;
                beneficiary: string;
            };
            remove_approval: {
                proposalId: string;
            };
        };
    };
    /**
     * Lookup186: pallet_assets::pallet::Call<T, I>
     **/
    PalletAssetsCall: {
        _enum: {
            create: {
                id: string;
                admin: string;
                minBalance: string;
            };
            force_create: {
                id: string;
                owner: string;
                isSufficient: string;
                minBalance: string;
            };
            destroy: {
                id: string;
                witness: string;
            };
            mint: {
                id: string;
                beneficiary: string;
                amount: string;
            };
            burn: {
                id: string;
                who: string;
                amount: string;
            };
            transfer: {
                id: string;
                target: string;
                amount: string;
            };
            transfer_keep_alive: {
                id: string;
                target: string;
                amount: string;
            };
            force_transfer: {
                id: string;
                source: string;
                dest: string;
                amount: string;
            };
            freeze: {
                id: string;
                who: string;
            };
            thaw: {
                id: string;
                who: string;
            };
            freeze_asset: {
                id: string;
            };
            thaw_asset: {
                id: string;
            };
            transfer_ownership: {
                id: string;
                owner: string;
            };
            set_team: {
                id: string;
                issuer: string;
                admin: string;
                freezer: string;
            };
            set_metadata: {
                id: string;
                name: string;
                symbol: string;
                decimals: string;
            };
            clear_metadata: {
                id: string;
            };
            force_set_metadata: {
                id: string;
                name: string;
                symbol: string;
                decimals: string;
                isFrozen: string;
            };
            force_clear_metadata: {
                id: string;
            };
            force_asset_status: {
                id: string;
                owner: string;
                issuer: string;
                admin: string;
                freezer: string;
                minBalance: string;
                isSufficient: string;
                isFrozen: string;
            };
            approve_transfer: {
                id: string;
                delegate: string;
                amount: string;
            };
            cancel_approval: {
                id: string;
                delegate: string;
            };
            force_cancel_approval: {
                id: string;
                owner: string;
                delegate: string;
            };
            transfer_approved: {
                id: string;
                owner: string;
                destination: string;
                amount: string;
            };
            touch: {
                id: string;
            };
            refund: {
                id: string;
                allowBurn: string;
            };
        };
    };
    /**
     * Lookup187: pallet_assets::types::DestroyWitness
     **/
    PalletAssetsDestroyWitness: {
        accounts: string;
        sufficients: string;
        approvals: string;
    };
    /**
     * Lookup188: pallet_authorship::pallet::Call<T>
     **/
    PalletAuthorshipCall: {
        _enum: {
            set_uncles: {
                newUncles: string;
            };
        };
    };
    /**
     * Lookup190: sp_runtime::generic::header::Header<Number, sp_runtime::traits::BlakeTwo256>
     **/
    SpRuntimeHeader: {
        parentHash: string;
        number: string;
        stateRoot: string;
        extrinsicsRoot: string;
        digest: string;
    };
    /**
     * Lookup191: sp_runtime::traits::BlakeTwo256
     **/
    SpRuntimeBlakeTwo256: string;
    /**
     * Lookup192: pallet_xdns::pallet::Call<T>
     **/
    PalletXdnsCall: {
        _enum: {
            add_side_effect: {
                id: string;
                name: string;
                argumentAbi: string;
                argumentToStateMapper: string;
                confirmEvents: string;
                escrowedEvents: string;
                commitEvents: string;
                revertEvents: string;
            };
            update_ttl: {
                gatewayId: string;
                lastFinalized: string;
            };
            purge_xdns_record: {
                requester: string;
                xdnsRecordId: string;
            };
        };
    };
    /**
     * Lookup194: t3rn_types::abi::Type
     **/
    T3rnTypesAbiType: {
        _enum: {
            Address: string;
            DynamicAddress: string;
            Bool: string;
            Int: string;
            Uint: string;
            Bytes: string;
            DynamicBytes: string;
            String: string;
            Enum: string;
            Struct: string;
            Mapping: string;
            Contract: string;
            Ref: string;
            Option: string;
            OptionalInsurance: string;
            OptionalReward: string;
            StorageRef: string;
            Value: string;
            Slice: string;
            Hasher: string;
            Crypto: string;
        };
    };
    /**
     * Lookup195: t3rn_types::abi::HasherAlgo
     **/
    T3rnTypesAbiHasherAlgo: {
        _enum: string[];
    };
    /**
     * Lookup196: t3rn_types::abi::CryptoAlgo
     **/
    T3rnTypesAbiCryptoAlgo: {
        _enum: string[];
    };
    /**
     * Lookup197: pallet_contracts_registry::pallet::Call<T>
     **/
    PalletContractsRegistryCall: {
        _enum: {
            add_new_contract: {
                requester: string;
                contract: string;
            };
            purge: {
                requester: string;
                contractId: string;
            };
        };
    };
    /**
     * Lookup198: t3rn_primitives::contracts_registry::RegistryContract<primitive_types::H256, sp_core::crypto::AccountId32, BalanceOf, BlockNumber>
     **/
    T3rnPrimitivesContractsRegistryRegistryContract: {
        codeTxt: string;
        bytes: string;
        author: string;
        abi: string;
        actionDescriptions: string;
        info: string;
        meta: string;
    };
    /**
     * Lookup199: t3rn_primitives::contracts_registry::AuthorInfo<sp_core::crypto::AccountId32, BalanceOf>
     **/
    T3rnPrimitivesContractsRegistryAuthorInfo: {
        account: string;
        feesPerSingleUse: string;
    };
    /**
     * Lookup201: t3rn_types::abi::ContractActionDesc<primitive_types::H256, TargetId, sp_core::crypto::AccountId32>
     **/
    T3rnTypesAbiContractActionDesc: {
        actionId: string;
        targetId: string;
        to: string;
    };
    /**
     * Lookup204: t3rn_primitives::storage::RawAliveContractInfo<primitive_types::H256, Balance, BlockNumber>
     **/
    T3rnPrimitivesStorageRawAliveContractInfo: {
        trieId: string;
        storageSize: string;
        pairCount: string;
        codeHash: string;
        rentAllowance: string;
        rentPaid: string;
        deductBlock: string;
        lastWrite: string;
        reserved: string;
    };
    /**
     * Lookup206: t3rn_primitives::contract_metadata::ContractMetadata
     **/
    T3rnPrimitivesContractMetadata: {
        metadataVersion: string;
        name: string;
        contractType: string;
        version: string;
        authors: string;
        description: string;
        documentation: string;
        repository: string;
        homepage: string;
        license: string;
    };
    /**
     * Lookup207: pallet_circuit::pallet::Call<T>
     **/
    PalletCircuitCall: {
        _enum: {
            on_local_trigger: {
                trigger: string;
            };
            on_xcm_trigger: string;
            on_remote_gateway_trigger: string;
            cancel_xtx: {
                xtxId: string;
            };
            revert: {
                xtxId: string;
            };
            on_extrinsic_trigger: {
                sideEffects: string;
                sequential: string;
            };
            bid_sfx: {
                sfxId: string;
                bidAmount: string;
            };
            confirm_side_effect: {
                sfxId: string;
                confirmation: string;
            };
        };
    };
    /**
     * Lookup208: pallet_3vm::pallet::Call<T>
     **/
    Pallet3vmCall: string;
    /**
     * Lookup209: pallet_contracts::pallet::Call<T>
     **/
    PalletContractsCall: {
        _enum: {
            call: {
                dest: string;
                value: string;
                gasLimit: string;
                storageDepositLimit: string;
                data: string;
            };
            instantiate_with_code: {
                value: string;
                gasLimit: string;
                storageDepositLimit: string;
                code: string;
                data: string;
                salt: string;
            };
            instantiate: {
                value: string;
                gasLimit: string;
                storageDepositLimit: string;
                codeHash: string;
                data: string;
                salt: string;
            };
            upload_code: {
                code: string;
                storageDepositLimit: string;
            };
            remove_code: {
                codeHash: string;
            };
        };
    };
    /**
     * Lookup211: pallet_evm::pallet::Call<T>
     **/
    PalletEvmCall: {
        _enum: {
            withdraw: {
                address: string;
                value: string;
            };
            call: {
                target: string;
                input: string;
                value: string;
                gasLimit: string;
                maxFeePerGas: string;
                maxPriorityFeePerGas: string;
                nonce: string;
                accessList: string;
            };
            create: {
                init: string;
                value: string;
                gasLimit: string;
                maxFeePerGas: string;
                maxPriorityFeePerGas: string;
                nonce: string;
                accessList: string;
            };
            create2: {
                init: string;
                salt: string;
                value: string;
                gasLimit: string;
                maxFeePerGas: string;
                maxPriorityFeePerGas: string;
                nonce: string;
                accessList: string;
            };
            claim: string;
        };
    };
    /**
     * Lookup212: pallet_account_manager::pallet::Call<T>
     **/
    PalletAccountManagerCall: {
        _enum: {
            deposit: {
                chargeId: string;
                payee: string;
                chargeFee: string;
                offeredReward: string;
                source: string;
                role: string;
                recipient: string;
                maybeAssetId: string;
            };
            finalize: {
                chargeId: string;
                outcome: string;
                maybeRecipient: string;
                maybeActualFees: string;
            };
        };
    };
    /**
     * Lookup213: t3rn_primitives::claimable::BenefitSource
     **/
    T3rnPrimitivesClaimableBenefitSource: {
        _enum: string[];
    };
    /**
     * Lookup214: t3rn_primitives::claimable::CircuitRole
     **/
    T3rnPrimitivesClaimableCircuitRole: {
        _enum: string[];
    };
    /**
     * Lookup215: t3rn_primitives::account_manager::Outcome
     **/
    T3rnPrimitivesAccountManagerOutcome: {
        _enum: string[];
    };
    /**
     * Lookup216: pallet_portal::pallet::Call<T>
     **/
    PalletPortalCall: {
        _enum: {
            register_gateway: {
                url: string;
                gatewayId: string;
                gatewayAbi: string;
                gatewayVendor: string;
                gatewayType: string;
                gatewayGenesis: string;
                gatewaySysProps: string;
                allowedSideEffects: string;
                encodedRegistrationData: string;
            };
            set_owner: {
                gatewayId: string;
                encodedNewOwner: string;
            };
            set_operational: {
                gatewayId: string;
                operational: string;
            };
            submit_headers: {
                gatewayId: string;
                encodedHeaderData: string;
            };
        };
    };
    /**
     * Lookup217: t3rn_types::abi::GatewayABIConfig
     **/
    T3rnTypesAbiGatewayABIConfig: {
        blockNumberTypeSize: string;
        hashSize: string;
        hasher: string;
        crypto: string;
        addressLength: string;
        valueTypeSize: string;
        decimals: string;
        structs: string;
    };
    /**
     * Lookup219: t3rn_types::abi::StructDecl
     **/
    T3rnTypesAbiStructDecl: {
        name: string;
        fields: string;
        offsets: string;
    };
    /**
     * Lookup221: t3rn_types::abi::Parameter
     **/
    T3rnTypesAbiParameter: {
        name: string;
        ty: string;
        no: string;
        indexed: string;
    };
    /**
     * Lookup224: t3rn_primitives::GatewayVendor
     **/
    T3rnPrimitivesGatewayVendor: {
        _enum: string[];
    };
    /**
     * Lookup225: t3rn_primitives::GatewayType
     **/
    T3rnPrimitivesGatewayType: {
        _enum: {
            ProgrammableInternal: string;
            ProgrammableExternal: string;
            TxOnly: string;
            OnCircuit: string;
        };
    };
    /**
     * Lookup226: t3rn_primitives::GatewayGenesisConfig
     **/
    T3rnPrimitivesGatewayGenesisConfig: {
        modulesEncoded: string;
        extrinsicsVersion: string;
        genesisHash: string;
    };
    /**
     * Lookup227: t3rn_primitives::GatewaySysProps
     **/
    T3rnPrimitivesGatewaySysProps: {
        ss58Format: string;
        tokenSymbol: string;
        tokenDecimals: string;
    };
    /**
     * Lookup229: pallet_sudo::pallet::Error<T>
     **/
    PalletSudoError: {
        _enum: string[];
    };
    /**
     * Lookup230: pallet_utility::pallet::Error<T>
     **/
    PalletUtilityError: {
        _enum: string[];
    };
    /**
     * Lookup231: pallet_identity::types::Registration<Balance, MaxJudgements, MaxAdditionalFields>
     **/
    PalletIdentityRegistration: {
        judgements: string;
        deposit: string;
        info: string;
    };
    /**
     * Lookup240: pallet_identity::types::RegistrarInfo<Balance, sp_core::crypto::AccountId32>
     **/
    PalletIdentityRegistrarInfo: {
        account: string;
        fee: string;
        fields: string;
    };
    /**
     * Lookup242: pallet_identity::pallet::Error<T>
     **/
    PalletIdentityError: {
        _enum: string[];
    };
    /**
     * Lookup244: pallet_balances::BalanceLock<Balance>
     **/
    PalletBalancesBalanceLock: {
        id: string;
        amount: string;
        reasons: string;
    };
    /**
     * Lookup245: pallet_balances::Reasons
     **/
    PalletBalancesReasons: {
        _enum: string[];
    };
    /**
     * Lookup248: pallet_balances::ReserveData<ReserveIdentifier, Balance>
     **/
    PalletBalancesReserveData: {
        id: string;
        amount: string;
    };
    /**
     * Lookup250: pallet_balances::Releases
     **/
    PalletBalancesReleases: {
        _enum: string[];
    };
    /**
     * Lookup251: pallet_balances::pallet::Error<T, I>
     **/
    PalletBalancesError: {
        _enum: string[];
    };
    /**
     * Lookup253: pallet_transaction_payment::Releases
     **/
    PalletTransactionPaymentReleases: {
        _enum: string[];
    };
    /**
     * Lookup254: pallet_treasury::Proposal<sp_core::crypto::AccountId32, Balance>
     **/
    PalletTreasuryProposal: {
        proposer: string;
        value: string;
        beneficiary: string;
        bond: string;
    };
    /**
     * Lookup258: frame_support::PalletId
     **/
    FrameSupportPalletId: string;
    /**
     * Lookup259: pallet_treasury::pallet::Error<T, I>
     **/
    PalletTreasuryError: {
        _enum: string[];
    };
    /**
     * Lookup260: pallet_assets::types::AssetDetails<Balance, sp_core::crypto::AccountId32, DepositBalance>
     **/
    PalletAssetsAssetDetails: {
        owner: string;
        issuer: string;
        admin: string;
        freezer: string;
        supply: string;
        deposit: string;
        minBalance: string;
        isSufficient: string;
        accounts: string;
        sufficients: string;
        approvals: string;
        isFrozen: string;
    };
    /**
     * Lookup262: pallet_assets::types::AssetAccount<Balance, DepositBalance, Extra>
     **/
    PalletAssetsAssetAccount: {
        balance: string;
        isFrozen: string;
        reason: string;
        extra: string;
    };
    /**
     * Lookup263: pallet_assets::types::ExistenceReason<Balance>
     **/
    PalletAssetsExistenceReason: {
        _enum: {
            Consumer: string;
            Sufficient: string;
            DepositHeld: string;
            DepositRefunded: string;
        };
    };
    /**
     * Lookup265: pallet_assets::types::Approval<Balance, DepositBalance>
     **/
    PalletAssetsApproval: {
        amount: string;
        deposit: string;
    };
    /**
     * Lookup266: pallet_assets::types::AssetMetadata<DepositBalance, sp_runtime::bounded::bounded_vec::BoundedVec<T, S>>
     **/
    PalletAssetsAssetMetadata: {
        deposit: string;
        name: string;
        symbol: string;
        decimals: string;
        isFrozen: string;
    };
    /**
     * Lookup268: pallet_assets::pallet::Error<T, I>
     **/
    PalletAssetsError: {
        _enum: string[];
    };
    /**
     * Lookup270: pallet_authorship::UncleEntryItem<BlockNumber, primitive_types::H256, sp_core::crypto::AccountId32>
     **/
    PalletAuthorshipUncleEntryItem: {
        _enum: {
            InclusionHeight: string;
            Uncle: string;
        };
    };
    /**
     * Lookup272: pallet_authorship::pallet::Error<T>
     **/
    PalletAuthorshipError: {
        _enum: string[];
    };
    /**
     * Lookup273: t3rn_types::interface::SideEffectInterface
     **/
    T3rnTypesInterfaceSideEffectInterface: {
        id: string;
        name: string;
        argumentAbi: string;
        argumentToStateMapper: string;
        confirmEvents: string;
        escrowedEvents: string;
        commitEvents: string;
        revertEvents: string;
    };
    /**
     * Lookup274: t3rn_primitives::xdns::XdnsRecord<sp_core::crypto::AccountId32>
     **/
    T3rnPrimitivesXdnsXdnsRecord: {
        url: string;
        gatewayAbi: string;
        gatewayGenesis: string;
        gatewayVendor: string;
        gatewayType: string;
        gatewayId: string;
        parachain: string;
        gatewaySysProps: string;
        registrant: string;
        securityCoordinates: string;
        lastFinalized: string;
        allowedSideEffects: string;
    };
    /**
     * Lookup276: t3rn_primitives::xdns::Parachain
     **/
    T3rnPrimitivesXdnsParachain: {
        relayChainId: string;
        id: string;
    };
    /**
     * Lookup277: pallet_xdns::pallet::Error<T>
     **/
    PalletXdnsError: {
        _enum: string[];
    };
    /**
     * Lookup278: pallet_contracts_registry::pallet::Error<T>
     **/
    PalletContractsRegistryError: {
        _enum: string[];
    };
    /**
     * Lookup279: pallet_circuit::state::XExecSignal<sp_core::crypto::AccountId32, BlockNumber>
     **/
    PalletCircuitStateXExecSignal: {
        requester: string;
        requesterNonce: string;
        timeoutsAt: string;
        delayStepsAt: string;
        status: string;
        stepsCnt: string;
    };
    /**
     * Lookup281: pallet_circuit::state::CircuitStatus
     **/
    PalletCircuitStateCircuitStatus: {
        _enum: {
            Requested: string;
            Reserved: string;
            PendingBidding: string;
            InBidding: string;
            Killed: string;
            Ready: string;
            PendingExecution: string;
            Finished: string;
            FinishedAllSteps: string;
            Reverted: string;
            Committed: string;
        };
    };
    /**
     * Lookup282: pallet_circuit::state::Cause
     **/
    PalletCircuitStateCause: {
        _enum: string[];
    };
    /**
     * Lookup283: t3rn_primitives::volatile::LocalState
     **/
    T3rnPrimitivesVolatileLocalState: {
        state: string;
    };
    /**
     * Lookup289: t3rn_sdk_primitives::signal::ExecutionSignal<primitive_types::H256>
     **/
    T3rnSdkPrimitivesSignalExecutionSignal: {
        step: string;
        kind: string;
        executionId: string;
    };
    /**
     * Lookup291: pallet_circuit::pallet::Error<T>
     **/
    PalletCircuitError: {
        _enum: string[];
    };
    /**
     * Lookup292: t3rn_primitives::common::RoundInfo<BlockNumber>
     **/
    T3rnPrimitivesCommonRoundInfo: {
        index: string;
        head: string;
        term: string;
    };
    /**
     * Lookup294: t3rn_primitives::claimable::ClaimableArtifacts<sp_core::crypto::AccountId32, Balance>
     **/
    T3rnPrimitivesClaimableClaimableArtifacts: {
        beneficiary: string;
        role: string;
        totalRoundClaim: string;
        benefitSource: string;
    };
    /**
     * Lookup295: pallet_clock::pallet::Error<T>
     **/
    PalletClockError: string;
    /**
     * Lookup297: pallet_3vm::pallet::Error<T>
     **/
    Pallet3vmError: {
        _enum: string[];
    };
    /**
     * Lookup298: pallet_contracts::wasm::PrefabWasmModule<T>
     **/
    PalletContractsWasmPrefabWasmModule: {
        instructionWeightsVersion: string;
        initial: string;
        maximum: string;
        code: string;
        author: string;
        kind: string;
    };
    /**
     * Lookup300: pallet_contracts::wasm::OwnerInfo<T>
     **/
    PalletContractsWasmOwnerInfo: {
        owner: string;
        deposit: string;
        refcount: string;
    };
    /**
     * Lookup301: pallet_contracts::storage::RawContractInfo<primitive_types::H256, Balance>
     **/
    PalletContractsStorageRawContractInfo: {
        trieId: string;
        codeHash: string;
        storageDeposit: string;
    };
    /**
     * Lookup303: pallet_contracts::storage::DeletedContract
     **/
    PalletContractsStorageDeletedContract: {
        trieId: string;
    };
    /**
     * Lookup304: pallet_contracts::schedule::Schedule<T>
     **/
    PalletContractsSchedule: {
        limits: string;
        instructionWeights: string;
        hostFnWeights: string;
    };
    /**
     * Lookup305: pallet_contracts::schedule::Limits
     **/
    PalletContractsScheduleLimits: {
        eventTopics: string;
        stackHeight: string;
        globals: string;
        parameters: string;
        memoryPages: string;
        tableSize: string;
        brTableSize: string;
        subjectLen: string;
        callDepth: string;
        payloadLen: string;
        codeLen: string;
    };
    /**
     * Lookup306: pallet_contracts::schedule::InstructionWeights<T>
     **/
    PalletContractsScheduleInstructionWeights: {
        _alias: {
            r_if: string;
        };
        version: string;
        i64const: string;
        i64load: string;
        i64store: string;
        select: string;
        r_if: string;
        br: string;
        brIf: string;
        brTable: string;
        brTablePerEntry: string;
        call: string;
        callIndirect: string;
        callIndirectPerParam: string;
        localGet: string;
        localSet: string;
        localTee: string;
        globalGet: string;
        globalSet: string;
        memoryCurrent: string;
        memoryGrow: string;
        i64clz: string;
        i64ctz: string;
        i64popcnt: string;
        i64eqz: string;
        i64extendsi32: string;
        i64extendui32: string;
        i32wrapi64: string;
        i64eq: string;
        i64ne: string;
        i64lts: string;
        i64ltu: string;
        i64gts: string;
        i64gtu: string;
        i64les: string;
        i64leu: string;
        i64ges: string;
        i64geu: string;
        i64add: string;
        i64sub: string;
        i64mul: string;
        i64divs: string;
        i64divu: string;
        i64rems: string;
        i64remu: string;
        i64and: string;
        i64or: string;
        i64xor: string;
        i64shl: string;
        i64shrs: string;
        i64shru: string;
        i64rotl: string;
        i64rotr: string;
    };
    /**
     * Lookup307: pallet_contracts::schedule::HostFnWeights<T>
     **/
    PalletContractsScheduleHostFnWeights: {
        _alias: {
            r_return: string;
        };
        caller: string;
        isContract: string;
        codeHash: string;
        ownCodeHash: string;
        callerIsOrigin: string;
        address: string;
        gasLeft: string;
        balance: string;
        valueTransferred: string;
        minimumBalance: string;
        blockNumber: string;
        now: string;
        weightToFee: string;
        gas: string;
        input: string;
        inputPerByte: string;
        r_return: string;
        returnPerByte: string;
        terminate: string;
        random: string;
        depositEvent: string;
        depositEventPerTopic: string;
        depositEventPerByte: string;
        debugMessage: string;
        setStorage: string;
        setStoragePerNewByte: string;
        setStoragePerOldByte: string;
        setCodeHash: string;
        clearStorage: string;
        clearStoragePerByte: string;
        containsStorage: string;
        containsStoragePerByte: string;
        getStorage: string;
        getStoragePerByte: string;
        takeStorage: string;
        takeStoragePerByte: string;
        transfer: string;
        call: string;
        delegateCall: string;
        callTransferSurcharge: string;
        callPerClonedByte: string;
        instantiate: string;
        instantiateTransferSurcharge: string;
        instantiatePerSaltByte: string;
        hashSha2256: string;
        hashSha2256PerByte: string;
        hashKeccak256: string;
        hashKeccak256PerByte: string;
        hashBlake2256: string;
        hashBlake2256PerByte: string;
        hashBlake2128: string;
        hashBlake2128PerByte: string;
        ecdsaRecover: string;
    };
    /**
     * Lookup308: pallet_contracts::pallet::Error<T>
     **/
    PalletContractsError: {
        _enum: string[];
    };
    /**
     * Lookup310: pallet_evm::ThreeVmInfo<T>
     **/
    PalletEvmThreeVmInfo: {
        author: string;
        kind: string;
    };
    /**
     * Lookup311: pallet_evm::pallet::Error<T>
     **/
    PalletEvmError: {
        _enum: string[];
    };
    /**
     * Lookup312: t3rn_primitives::account_manager::RequestCharge<sp_core::crypto::AccountId32, Balance, AssetId>
     **/
    T3rnPrimitivesAccountManagerRequestCharge: {
        payee: string;
        offeredReward: string;
        maybeAssetId: string;
        chargeFee: string;
        recipient: string;
        source: string;
        role: string;
    };
    /**
     * Lookup314: t3rn_primitives::account_manager::Settlement<sp_core::crypto::AccountId32, Balance>
     **/
    T3rnPrimitivesAccountManagerSettlement: {
        requester: string;
        recipient: string;
        settlementAmount: string;
        outcome: string;
        source: string;
        role: string;
    };
    /**
     * Lookup315: pallet_account_manager::pallet::Error<T>
     **/
    PalletAccountManagerError: {
        _enum: string[];
    };
    /**
     * Lookup316: pallet_portal::pallet::Error<T>
     **/
    PalletPortalError: {
        _enum: string[];
    };
    /**
     * Lookup320: pallet_grandpa_finality_verifier::bridges::header_chain::AuthoritySet
     **/
    PalletGrandpaFinalityVerifierBridgesHeaderChainAuthoritySet: {
        authorities: string;
        setId: string;
    };
    /**
     * Lookup321: pallet_grandpa_finality_verifier::types::Parachain
     **/
    PalletGrandpaFinalityVerifierParachain: {
        relayChainId: string;
        id: string;
    };
    /**
     * Lookup322: pallet_grandpa_finality_verifier::pallet::Error<T, I>
     **/
    PalletGrandpaFinalityVerifierError: {
        _enum: string[];
    };
    /**
     * Lookup324: sp_runtime::MultiSignature
     **/
    SpRuntimeMultiSignature: {
        _enum: {
            Ed25519: string;
            Sr25519: string;
            Ecdsa: string;
        };
    };
    /**
     * Lookup325: sp_core::sr25519::Signature
     **/
    SpCoreSr25519Signature: string;
    /**
     * Lookup326: sp_core::ecdsa::Signature
     **/
    SpCoreEcdsaSignature: string;
    /**
     * Lookup329: frame_system::extensions::check_non_zero_sender::CheckNonZeroSender<T>
     **/
    FrameSystemExtensionsCheckNonZeroSender: string;
    /**
     * Lookup330: frame_system::extensions::check_spec_version::CheckSpecVersion<T>
     **/
    FrameSystemExtensionsCheckSpecVersion: string;
    /**
     * Lookup331: frame_system::extensions::check_tx_version::CheckTxVersion<T>
     **/
    FrameSystemExtensionsCheckTxVersion: string;
    /**
     * Lookup332: frame_system::extensions::check_genesis::CheckGenesis<T>
     **/
    FrameSystemExtensionsCheckGenesis: string;
    /**
     * Lookup335: frame_system::extensions::check_nonce::CheckNonce<T>
     **/
    FrameSystemExtensionsCheckNonce: string;
    /**
     * Lookup336: frame_system::extensions::check_weight::CheckWeight<T>
     **/
    FrameSystemExtensionsCheckWeight: string;
    /**
     * Lookup337: pallet_asset_tx_payment::ChargeAssetTxPayment<T>
     **/
    PalletAssetTxPaymentChargeAssetTxPayment: {
        tip: string;
        assetId: string;
    };
    /**
     * Lookup338: circuit_standalone_runtime::Runtime
     **/
    CircuitStandaloneRuntimeRuntime: string;
};
export default _default;
