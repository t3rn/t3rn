#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use std::{thread, time};

use frame_support::{
    debug, decl_error, decl_event, decl_module, decl_storage, dispatch, ensure,
    storage::child::kill_storage,
    traits::{Currency, Time},
};
use frame_system::{ensure_signed};
use pallet_balances as balances;
use reduce::Reduce;
use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize, Serializer};
use sp_runtime::{
    traits::{Hash, Saturating},
    DispatchResult,
};
use t3rn_primitives::Compose;
use regex::bytes::Regex;
use sp_std::convert::TryInto;
use sp_std::vec;
use sp_std::vec::Vec;


// pub mod watchtower;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

type Gas = u64;

// pub type BalanceOf<T> =
//     <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;

pub trait Trait: balances::Trait + frame_system::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}


decl_event!(
	pub enum Event {

	}
);

decl_storage! {
    trait Store for Module<T: Trait> as ChildStorage {

    }
}

decl_error! {
    pub enum Error for Module<T: Trait> {

        RequesterNotEnoughBalance,

        BalanceTransferFailed,

        FeesOverflow,

        FeesRefundFailed,

        CleanupFailedAfterUnsuccessfulExecution,

        UnknownIOScheduleCompose,

        IOScheduleNoEndingSemicolon,
    }
}

// ToDo: Encode errors properly before storing making the below enum obsolete.
#[derive(Clone)]
#[repr(u8)]
pub enum ErrCodes {
    RequesterNotEnoughBalance = 0,

    BalanceTransferFailed = 1,

    FeesRefundFailed = 2,

    PutCodeFailure = 3,

    InitializationFailure = 4,

    ExecutionFailure = 5,

    CallFailure = 6,

    TerminateFailure = 7,
}

// Specifying serde path as `alt_serde`
// ref: https://serde.rs/container-attrs.html#crate
// #[derive(Deserialize, Encode, Decode, Clone, Default, Debug)]
// #[serde(crate = "alt_serde")]
#[derive(Encode, Decode, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterExecReq {
    // Specify our own deserializing function to convert JSON string to vector of bytes
    components: Vec<Compose>,
    io: Vec<u8>,
}

#[derive(Encode, Decode, Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ExecPhase {
    steps: Vec<ExecStep>,
}

#[derive(Encode, Decode, Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ExecStep {
    compose: Compose,
    stamp: Option<Vec<u8>>,
    witness: Option<Vec<u8>>,
}
#[derive(Encode, Decode, Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct InterExecSchedule {
    // Specify our own deserializing function to convert JSON string to vector of bytes
    phases: Vec<ExecPhase>,
}


pub fn decompose_io_schedule<T: Trait>(
    exec_req: &InterExecReq,
) -> Result<InterExecSchedule, Error<T>> {
    let mut inter_schedule = InterExecSchedule::default();

    for caps in Regex::new(
        r"(?P<compose_name>[\w]+)|(?P<next_phase>[>]+)|(?P<parallel_step>[||]+)|(?P<end>[;]+)",
    )
    .unwrap()
    .captures_iter(&exec_req.io[..])
    {
        println!("caps {:?}", caps);
        if let Some(name) = caps.name("compose_name") {
            if let Some(selected_compose) = exec_req.components.clone().into_iter().find(|comp| {
                // println!("IF FIND compose_name {:?} vs {:?} vs {:?} vs {:?}", comp.name.clone(), comp.name.clone().encode(), name.clone().as_bytes(), name.clone().as_bytes().encode());
                // println!("IF FIND UTF8 {:?} vs {:?} vs {:?} vs {:?}", core::str::from_utf8(&comp.name.clone()), core::str::from_utf8(&comp.name.clone().encode()), core::str::from_utf8(name.clone().as_bytes()), core::str::from_utf8(&name.clone().as_bytes().encode()));
                comp.name == name.as_bytes().encode()
            }) {
                let new_step = ExecStep {
                    compose: selected_compose.clone(),
                    stamp: None,
                    witness: None,
                };
                if let Some(last_phase) = inter_schedule.phases.last_mut() {
                    println!(
                        "ADD NEW STEP! {:?} {:?}",
                        new_step.clone(),
                        core::str::from_utf8(&new_step.compose.name.clone())
                    );
                    last_phase.steps.push(new_step);
                } else {
                    println!("NEW EXEC PHASE! inter_schedule.phases.push(ExecPhase");
                    inter_schedule.phases.push(ExecPhase {
                        steps: vec![new_step],
                    });
                }
            } else {
                println!(
                    "ERR UnknownIOScheduleCompose {:?} {:?} ",
                    name,
                    core::str::from_utf8(name.clone().as_bytes())
                );
                return Err(Error::<T>::UnknownIOScheduleCompose);
            }
        }
        if let Some(name) = caps.name("next_phase") {
            inter_schedule.phases.push(ExecPhase::default());
            println!("caps next_phase {:?}", name);
        }
        if let Some(name) = caps.name("parallel_step") {
            println!("caps parallel_step {:?}", name);
        }
        if let Some(name) = caps.name("end") {
            println!("caps EOF {:?}", name);
            return Ok(inter_schedule);
        }
    }
    Err(Error::<T>::IOScheduleNoEndingSemicolon)
}


decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: <T as frame_system::Trait>::Origin {
        // Initializing errors
        // this includes information about your errors in the node's metadata.
        // it is needed only if you are using errors in your pallet
        type Error = Error<T>;
        // Initializing events
        // this is needed only if you are using events in your pallet
        fn deposit_event() = default;

        /// **composable_execution**
        /// ---
        /// **NOTE:**
        /// ---
       #[weight = *gas_limit]
        pub fn composable_execution(
            origin,
            escrow_account: <T as frame_system::Trait>::AccountId,
            code: Vec<u8>,
            // execution_request: InterExecReq,
            execution_request: InterExecReq,
            #[compact] value: T::Balance,
            #[compact] gas_limit: Gas,
            input_data: Vec<u8>,
        ) -> dispatch::DispatchResult {
            // Ensure that the caller is a regular keypair account
            let requester = ensure_signed(origin)?;


            // let s: String = Decode::decode(&mut execution_request.components[0].name).unwrap();
            println!("Rec InterExecReq {:?}", execution_request);
            if let Ok(utf8) = core::str::from_utf8(&execution_request.components[0].name[..]) {
                println!("Rec InterExecReq component #1 name {:?} {:?}", utf8, execution_request.components[0].name == "component1".encode());
            }
            // Decode execution schedule from JSON request
            let mut inter_schedule = decompose_io_schedule::<T>(&execution_request)?;
            // For each execution phase
            env_logger::init();

            let mut i = 0;

            for phase in inter_schedule.phases.clone() {
                println!("Circuit new phase start");
                // Order execution on gateway

                if i == 1 {
                    println!("subscribe for transfer events");
                    // let block2 = watchtower::subscribe_for_gateway_events();
                } else {
                    println!("fetch_kusama_block block ");
                    // let block = watchtower::sync_fetch_kusama_block();
                }
                let ten_millis = time::Duration::from_millis(10000);
                thread::sleep(ten_millis);

                // Collect execution stamp

                // Collect witness

                // Re-execute on Circuit

                // Validate execution

                i += 1;
            }

            println!("ALL GOOD; BYE BYE {:?}", inter_schedule);
            Ok(())
        }
    }
}
