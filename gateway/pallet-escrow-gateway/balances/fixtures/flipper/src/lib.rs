#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::{
    decl_module, decl_storage, dispatch::DispatchResult,
};
use frame_system::{Event, self as system, ensure_signed};

pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as Token {
		pub Value get(fn get_value): bool = false;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		/// Flip!
		#[weight = 10_000]
		pub fn flip(origin) -> DispatchResult {
			ensure_signed(origin)?;
			Value::put(!Value::get());
			Ok(())
		}
	}
}
