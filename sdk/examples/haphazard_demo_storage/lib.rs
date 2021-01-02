// Copyright 2018-2020 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
#[cfg(feature = "flipper_contract")]
#[ink::contract]
pub mod flipper {
    /// Emitted when the requirement changed.
    #[ink(event)]
    pub struct FlipperFlip {
        /// The new requirement value.
        new_value: bool,
    }
    /// Emitted when the requirement changed.
    #[ink(event)]
    pub struct FlipperGet {
        /// The new requirement value.
        current_value: bool,
    }
    #[ink(storage)]
    pub struct Flipper {
        value: bool,
    }

    impl Flipper {
        /// Creates a new flipper smart contract initialized with the given value.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// Creates a new flipper smart contract initialized to `false`.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// Flips the current value of the Flipper's bool.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
            self.env().emit_event(FlipperFlip {
                new_value: self.value,
            });
        }

        /// Returns the current value of the Flipper's bool.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.env().emit_event(FlipperGet {
                current_value: self.value,
            });
            self.value
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn default_works() {
            let flipper = Flipper::default();
            assert_eq!(flipper.get(), false);
        }

        #[test]
        fn it_works() {
            let mut flipper = Flipper::new(false);
            assert_eq!(flipper.get(), false);
            flipper.flip();
            assert_eq!(flipper.get(), true);
        }
    }
}

#[cfg(feature = "runtime_demo_storage")]
static RUNTIME_DEMO_STORAGE_WAT_CONTRACT: &str = r#"
    (module
        (import "seal0" "seal_return" (func $seal_return(param i32 i32 i32)))
        (import "seal0" "seal_call" (func $seal_call (param i32 i32 i64 i32 i32 i32 i32 i32 i32) (result i32)))
        (import "seal0" "seal_input" (func $seal_input (param i32 i32)))
        (import "seal0" "seal_deposit_event" (func $seal_deposit_event (param i32 i32 i32 i32)))
        (import "seal0" "seal_get_raw_storage_by_prefix" (func $seal_get_raw_storage_by_prefix (param i32 i32 i32 i32) (result i32)))
        (import "env" "memory" (memory 1 1))

        ;; [32, 64) bytes for a module name
        (data (i32.const 32) "Weights")
        ;; [64, 96) bytes for a function name
        (data (i32.const 64) "store_value")

        ;; [96, 100) buffer where input is copied (u32 value to put into storage)

        ;; [100, 104) size of the input buffer
        (data (i32.const 100) "\04")

        ;; [104, 136) bytes for a module name
        (data (i32.const 104) "Weights")
        ;; [136, 168) bytes for a 2nd function name
        (data (i32.const 136) "double")

        ;; [168, 172) buffer where input is copied (u32 value to put into storage)

        ;; [172, 176) size of the input buffer
        (data (i32.const 172) "\04")

        ;; [176, 208) bytes for a module name
        (data (i32.const 176) "Weights")
        ;; [208, 240) bytes for a 3rd function name
        (data (i32.const 208) "complex_calculations")

        ;; [240, 248) buffer where input is copied (x: u32 , y: u32 values for complex calculation parameters)
        (data (i32.const 240) "\09") ;; x: u32
        (data (i32.const 244) "\08") ;; y: u32

        ;; [244, 248) size of the input buffer
        (data (i32.const 248) "\08")

        (data (i32.const 256) "SimpleMap")

        (data (i32.const 288) "StoredValue")
        ;; [320, 324) buffer for get_storage StoredValue output
        ;; [324, 328) size of the output buffer for get_storage StoredValue (u32)
        (data (i32.const 324) "\04")


        ;; Set topics for deposit_event after cals are made
        (data (i32.const 512) "\04\83\116\111\114\97\103\101\32\45\32\115\101\116\95\118\97\108\117\101") ;; Storage - set_value as utf8 bytes

        (data (i32.const 576) "\04\83\116\111\114\97\103\101\32\45\32\100\111\117\98\108\101") ;; Storage - double" as utf8 bytes

        (data (i32.const 640) "\04\83\116\111\114\97\103\101\32\45\32\99\111\109\112\108\101\120\95\99\97\108\99\117\108\97\116\105\111\110\115") ;; Storage - complex_calculations") as utf8 bytes

        (func $assert (param i32)
            (block $ok
                (br_if $ok
                    (get_local 0)
                )
                (unreachable)
            )
        )

        (func (export "deploy"))

        (func (export "call")
            (local $address_length i32)
            (local $balance_length i32)
            (set_local $address_length (i32.const 32)) ;; 32 bytes of account length for demo-runtime runtime
            (set_local $balance_length (i32.const 16)) ;; u128 for demo-runtime runtime

            (call $seal_input (i32.const 96) (i32.const 100))

            (call $seal_call
                (i32.const 0)	;; Pointer to destination address
                (get_local $address_length)	;; Length of destination address
                (i64.const 0)	;; How much gas to devote for the execution. 0 = all.
                (i32.const 0)	;; Pointer to the buffer with value to transfer
                (get_local $balance_length)	;; Length of the buffer with value to transfer
                (i32.const 32)	;; Pointer to input data buffer address (NOTE THAT FIRST 64B RESERVER FOR FN + MOD NAMES)
                (i32.const 4)	;; Length of input data buffer
                (i32.const 4294967295) ;; Pointer to output data buffer address
                (i32.const 0) ;;Length of output data buffer
            )
            ;; Get current value of StoredValue after "store_value" call and feed into deposit_event
            (call $seal_get_raw_storage_by_prefix
                (i32.const 256)
                (i32.const 280)
                (i32.const 320)
                (i32.const 324)
            )

            (call $seal_deposit_event
                (i32.const 512) ;; The topics buffer
                (i32.const 64) ;; The topics buffer's length
                (i32.const 320) ;; The data buffer
                (i32.const 4) ;;  Length of data buffer
            )

            ;; Copy out the input given by a user to to input for the next "double" call
            (i32.store
                (i32.const 168)
                (i32.load (i32.const 96))
            )

            (call $seal_call
                (i32.const 16)	;; Pointer to destination address
                (get_local $address_length)	;; Length of destination address
                (i64.const 0)	;; How much gas to devote for the execution. 0 = all.
                (i32.const 0)	;; Pointer to the buffer with value to transfer
                (get_local $balance_length)	;; Length of the buffer with value to transfer
                (i32.const 104)	;; Pointer to input data buffer address (NOTE THAT FIRST 64B RESERVER FOR FN + MOD NAMES)
                (i32.const 4)	;; Length of input data buffer
                (i32.const 4294967295) ;; Pointer to output data buffer address
                (i32.const 0) ;;Length of output data buffer
            )

            ;; Get current value of StoredValue after "double" call and feed into 2nd deposit_event
            (call $seal_get_raw_storage_by_prefix
                (i32.const 256)
                (i32.const 280)
                (i32.const 320)
                (i32.const 324)
            )

            (call $seal_deposit_event
                (i32.const 576) ;; The topics buffer
                (i32.const 64) ;; The topics buffer's length
                (i32.const 320) ;; The data buffer
                (i32.const 4) ;;  Length of data buffer
            )

            (call $seal_call
                (i32.const 16)	;; Pointer to destination address
                (get_local $address_length)	;; Length of destination address
                (i64.const 0)	;; How much gas to devote for the execution. 0 = all.
                (i32.const 0)	;; Pointer to the buffer with value to transfer
                (get_local $balance_length)	;; Length of the buffer with value to transfer
                (i32.const 176)	;; Pointer to input data buffer address (NOTE THAT FIRST 64B RESERVER FOR FN + MOD NAMES)
                (i32.const 8)	;; Length of input data buffer
                (i32.const 4294967295) ;; Pointer to output data buffer address
                (i32.const 0) ;;Length of output data buffer
            )

            ;; Get current value of StoredValue after "complex_calulations" call and feed into 3rd deposit_event
            (call $seal_get_raw_storage_by_prefix
                (i32.const 256)
                (i32.const 280)
                (i32.const 320)
                (i32.const 324)
            )

            (call $seal_deposit_event
                (i32.const 640) ;; The topics buffer
                (i32.const 64) ;; The topics buffer's length
                (i32.const 320) ;; The data buffer
                (i32.const 4) ;;  Length of data buffer
            )

            (call $seal_return
                (i32.const 0)
                (i32.const 0)
                (i32.const 0)
            )
            (unreachable)
        )
    )
"#;

#[cfg(feature = "call_flipper")]
#[ink::contract]
pub mod call_flipper {
    use core::convert::TryInto;
    use ink_env::call::{build_call, utils::ReturnType, ExecutionInput, Selector};
    /// Errors that can occur upon calling this contract.
    #[derive(Copy, Clone, Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        /// Returned if the call failed.
        TransactionFailed,
    }

    #[ink(storage)]
    pub struct CallFlipper {}

    impl CallFlipper {
        /// Creates a new flipper smart contract initialized with the given value.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        /// Creates a new flipper smart contract initialized to `false`.
        #[ink(message)]
        pub fn call_flip(&mut self, target_flip_contract: AccountId) -> Result<bool, Error> {
            build_call::<<Self as ::ink_lang::ContractEnv>::Env>()
                .callee(target_flip_contract)
                .gas_limit(Self::env().gas_left().try_into().unwrap())
                .transferred_value(0)
                .exec_input(ExecutionInput::new(Selector::new([0xC0, 0x96, 0xA5, 0xF3])))
                .returns::<ReturnType<bool>>()
                .fire()
                .map_err(|_| Error::TransactionFailed)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn it_works() {
            let mut flipper = CallFlipper::new();
            assert_eq!(flipper.call_flip(), false);
        }
    }
}
