use codec::{Decode, Encode};
use frame_support::RuntimeDebug;
#[cfg(test)]
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, ConstU32, IdentityLookup},
};

pub type AccountId = sp_runtime::AccountId32;
pub type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<MiniRuntime>;
pub type Block = frame_system::mocking::MockBlock<MiniRuntime>;
pub type Balance = u128;

frame_support::construct_runtime!(
    pub enum MiniRuntime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        System: frame_system = 1,
        Balances: pallet_balances = 2,
    }
);

impl pallet_balances::Config for MiniRuntime {
    type AccountStore = System;
    /// The type for recording an account's balance.
    type Balance = Balance;
    type DustRemoval = ();
    /// The ubiquitous event type.
    type Event = ();
    type ExistentialDeposit = ();
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type WeightInfo = ();
}

impl frame_system::Config for MiniRuntime {
    type AccountData = pallet_balances::AccountData<u128>;
    type AccountId = AccountId;
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockHashCount = ();
    type BlockLength = ();
    type BlockNumber = u64;
    type BlockWeights = ();
    type DbWeight = ();
    type Event = ();
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type Header = Header;
    type Index = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type MaxConsumers = ConstU32<16>;
    type OnKilledAccount = ();
    type OnNewAccount = ();
    type OnSetCode = ();
    type PalletInfo = PalletInfo;
    type RuntimeCall = RuntimeCall;
    type RuntimeOrigin = RuntimeOrigin;
    type SS58Prefix = ();
    type SystemWeightInfo = ();
    type Version = ();
}

pub type AssetId = u32;

// Mock from pallet events
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub enum MockedAssetEvent<T: frame_system::Config + pallet_balances::Config> {
    /// Some asset class was created.
    Created {
        asset_id: AssetId,
        creator: T::AccountId,
        owner: T::AccountId,
    },
    /// Some assets were issued.
    Issued {
        asset_id: AssetId,
        owner: T::AccountId,
        amount: T::Balance,
    },
    /// Some assets were transferred.
    Transferred {
        asset_id: AssetId,
        from: T::AccountId,
        to: T::AccountId,
        amount: T::Balance,
    },
    /// Some assets were destroyed.
    Burned {
        asset_id: AssetId,
        owner: T::AccountId,
        balance: T::Balance,
    },
    /// The management team changed.
    TeamChanged {
        asset_id: AssetId,
        issuer: T::AccountId,
        admin: T::AccountId,
        freezer: T::AccountId,
    },
    /// The owner changed.
    OwnerChanged {
        asset_id: AssetId,
        owner: T::AccountId,
    },
    /// Some account `who` was frozen.
    Frozen {
        asset_id: AssetId,
        who: T::AccountId,
    },
    /// Some account `who` was thawed.
    Thawed {
        asset_id: AssetId,
        who: T::AccountId,
    },
    /// Some asset `asset_id` was frozen.
    AssetFrozen { asset_id: AssetId },
    /// Some asset `asset_id` was thawed.
    AssetThawed { asset_id: AssetId },
    /// Accounts were destroyed for given asset.
    AccountsDestroyed {
        asset_id: AssetId,
        accounts_destroyed: u32,
        accounts_remaining: u32,
    },
    /// Approvals were destroyed for given asset.
    ApprovalsDestroyed {
        asset_id: AssetId,
        approvals_destroyed: u32,
        approvals_remaining: u32,
    },
    /// An asset class is in the process of being destroyed.
    DestructionStarted { asset_id: AssetId },
    /// An asset class was destroyed.
    Destroyed { asset_id: AssetId },
    /// Some asset class was force-created.
    ForceCreated {
        asset_id: AssetId,
        owner: T::AccountId,
    },
    /// New metadata has been set for an asset.
    MetadataSet {
        asset_id: AssetId,
        name: Vec<u8>,
        symbol: Vec<u8>,
        decimals: u8,
        is_frozen: bool,
    },
    /// Metadata has been cleared for an asset.
    MetadataCleared { asset_id: AssetId },
    /// (Additional) funds have been approved for transfer to a destination account.
    ApprovedTransfer {
        asset_id: AssetId,
        source: T::AccountId,
        delegate: T::AccountId,
        amount: T::Balance,
    },
    /// An approval for account `delegate` was cancelled by `owner`.
    ApprovalCancelled {
        asset_id: AssetId,
        owner: T::AccountId,
        delegate: T::AccountId,
    },
    /// An `amount` was transferred in its entirety from `owner` to `destination` by
    /// the approved `delegate`.
    TransferredApproved {
        asset_id: AssetId,
        owner: T::AccountId,
        delegate: T::AccountId,
        destination: T::AccountId,
        amount: T::Balance,
    },
    /// An asset has had its attributes changed by the `Force` origin.
    AssetStatusChanged { asset_id: AssetId },
}

type CodeHash<T> = <T as frame_system::Config>::Hash;

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub enum MockWasmContractsEvent<T: frame_system::Config + pallet_balances::Config> {
    /// Contract deployed by address at the specified address.
    Instantiated {
        deployer: T::AccountId,
        contract: T::AccountId,
    },

    /// Contract has been removed.
    ///
    /// # Note
    ///
    /// The only way for a contract to be removed and emitting this event is by calling
    /// `seal_terminate`.
    Terminated {
        /// The contract that was terminated.
        contract: T::AccountId,
        /// The account that received the contracts remaining balance
        beneficiary: T::AccountId,
    },

    /// Code with the specified hash has been stored.
    CodeStored { code_hash: T::Hash },

    /// A custom event emitted by the contract.
    ContractEmitted {
        /// The contract that emitted the event.
        contract: T::AccountId,
        /// Data supplied by the contract. Metadata generated during contract compilation
        /// is needed to decode it.
        data: Vec<u8>,
    },

    /// A code with the specified hash was removed.
    CodeRemoved { code_hash: T::Hash },

    /// A contract's code was updated.
    ContractCodeUpdated {
        /// The contract that has been updated.
        contract: T::AccountId,
        /// New code hash that was set for the contract.
        new_code_hash: T::Hash,
        /// Previous code hash of the contract.
        old_code_hash: T::Hash,
    },

    /// A contract was called either by a plain account or another contract.
    ///
    /// # Note
    ///
    /// Please keep in mind that like all events this is only emitted for successful
    /// calls. This is because on failure all storage changes including events are
    /// rolled back.
    Called {
        /// The account that called the `contract`.
        caller: T::AccountId,
        /// The contract that was called.
        contract: T::AccountId,
    },

    /// A contract delegate called a code hash.
    ///
    /// # Note
    ///
    /// Please keep in mind that like all events this is only emitted for successful
    /// calls. This is because on failure all storage changes including events are
    /// rolled back.
    DelegateCalled {
        /// The contract that performed the delegate call and hence in whose context
        /// the `code_hash` is executed.
        contract: T::AccountId,
        /// The code hash that was delegate called.
        code_hash: CodeHash<T>,
    },
}
