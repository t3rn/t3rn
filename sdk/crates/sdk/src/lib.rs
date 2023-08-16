#![cfg_attr(not(feature = "std"), no_std)]

extern crate core;

pub use t3rn_sdk_primitives as primitives;

use crate::executor::{Executor, StateHandler, Submitter};
use codec::{Decode, Encode};
use error::Error;
use t3rn_sdk_primitives::{
    signal::{ExecutionSignal, KillReason, SignalKind, Signaller},
    state::{ExecutionState, GetSteps, SideEffects},
    storage::BoundedVec,
    xc::Chain,
    Box, Debug, DEFAULT_MAX_STEPS_IN_EXECUTION, MAX_PARAMETERS_IN_FUNCTION,
};

pub mod error;
pub mod executor;

/// Information for the next step to be sent to the virtual machine.
#[derive(Clone)]
pub struct Step<AccountId, Balance, Hash>
where
    AccountId: Encode + Decode + Debug + Clone,
    Balance: Encode + Decode + Debug + Clone,
    Hash: Encode + Decode + Debug + Clone,
{
    pub side_effects: BoundedVec<Chain<AccountId, Balance, Hash>, MAX_PARAMETERS_IN_FUNCTION>,
}

impl<AccountId, Balance, Hash> FromIterator<Chain<AccountId, Balance, Hash>>
    for Step<AccountId, Balance, Hash>
where
    AccountId: Encode + Decode + Debug + Clone,
    Balance: Encode + Decode + Debug + Clone,
    Hash: Encode + Decode + Debug + Clone,
{
    fn from_iter<T: IntoIterator<Item = Chain<AccountId, Balance, Hash>>>(iter: T) -> Self {
        Step {
            side_effects: BoundedVec::from_iter(iter),
        }
    }
}

impl<AccountId, Balance, Hash> Step<AccountId, Balance, Hash>
where
    AccountId: Encode + Decode + Debug + Clone,
    Balance: Encode + Decode + Debug + Clone,
    Hash: Encode + Decode + Debug + Clone,
{
    /// Try to push another side effect to the step, this would fail if the underlying data structure
    /// met max_capacity
    pub fn try_push(
        &mut self,
        side_effect: Chain<AccountId, Balance, Hash>,
    ) -> Result<&mut Self, Error> {
        log_msg!(
            "Checking that {} is not more than {}",
            self.side_effects.0.len() + 1,
            self.side_effects.0.capacity()
        );
        if self.side_effects.0.len() + 1 >= self.side_effects.0.capacity() {
            return Err(Error::TooManyFunctionParams)
        }
        self.side_effects.0.push(side_effect);
        Ok(self)
    }

    /// Same functionality as `Iter::pop`
    pub fn try_pop(&mut self) -> Option<Chain<AccountId, Balance, Hash>> {
        self.side_effects.0.pop()
    }

    /// Take a look at the last element of the Vector, if it exists, otherwise none.
    pub fn peek(&self) -> Option<&Chain<AccountId, Balance, Hash>> {
        self.side_effects.0.last()
    }
}

impl<AccountId, Balance, Hash> Default for Step<AccountId, Balance, Hash>
where
    AccountId: Encode + Decode + Debug + Clone,
    Balance: Encode + Decode + Debug + Clone,
    Hash: Encode + Decode + Debug + Clone,
{
    fn default() -> Self {
        Self {
            side_effects: BoundedVec::default(),
        }
    }
}

/// A user provided function that takes a hash and returns some steps to be appended to local state on-chain.
pub type UserStepHandler<Hash, AccountId, Balance> = Box<
    dyn Fn(
        &ExecutionState<Hash, AccountId, u64, Balance>,
    ) -> Result<Step<AccountId, Balance, Hash>, Error>,
>;

/// Execute all steps, providing the user function to apply to the steps.
///
/// This function signals on continuation for each step.
pub fn execute<Hash, AccountId, Balance>(
    execution_id: Option<Hash>,
    f: UserStepHandler<Hash, AccountId, Balance>,
) -> Result<(), Error>
where
    Hash: Encode + Decode + Clone + Debug + PartialEq + Eq + Default + Copy,
    AccountId: Encode + Decode + Clone + Debug,
    Balance: Encode + Decode + Clone + Debug,
{
    match run_steps(execution_id, f) {
        Ok(signal) => {
            Executor::signal(&signal)?;
            Ok(())
        },
        Err(e) => Err(e),
    }
}

/// Run the steps, applying the user function.
///
/// On each step, we retrieve execution state, either new(if execution id is None) or existing and run some validation on it.
///
/// Then apply the user function:
///     If ok and steps are empty:
///         complete execution
///     If ok and new steps:
///         submit them and call again
///     If err:
///         signal the err
fn run_steps<Hash, AccountId, Balance>(
    execution_id: Option<Hash>,
    f: UserStepHandler<Hash, AccountId, Balance>,
) -> Result<ExecutionSignal<Hash>, Error>
where
    Hash: Encode + Decode + Clone + Debug + Default + Copy,
    AccountId: Encode + Decode + Clone + Debug,
    Balance: Encode + Decode + Clone + Debug,
{
    let prev_state = Executor::get_state(execution_id)?;

    let execution_id = match execution_id {
        None => prev_state.xtx_id,
        Some(hash) => hash,
    };

    // We get step count here, we can just break if the step is greater than the limit.
    if prev_state.get_index() >= DEFAULT_MAX_STEPS_IN_EXECUTION as u32 {
        log_msg!("[SDK] Execution limit reached");
        return Ok(ExecutionSignal::new(
            &execution_id,
            Some(prev_state.get_index()),
            SignalKind::Complete,
        ))
    }
    // apply user function
    match f(&prev_state) {
        Ok(state) => {
            log_msg!("[SDK] user function handled successfully");
            let new_step_state: Step<AccountId, Balance, Hash> = state;

            // if ok and no side effects, send complete and break
            // Guard for no new steps and reached limit
            if prev_state.reached_end() && new_step_state.side_effects.is_empty() {
                return Ok(ExecutionSignal::new(
                    &execution_id,
                    Some(prev_state.get_index()),
                    SignalKind::Complete,
                ))
            }

            Executor::submit(SideEffects {
                execution_id,
                side_effects: new_step_state.side_effects,
            })
            .map_err(|e| {
                let _: Result<(), executor::Error> = Executor::signal(&ExecutionSignal::new(
                    &execution_id,
                    Some(prev_state.get_index()),
                    SignalKind::Kill(KillReason::Unhandled),
                ));
                e
            })?;
            // call self ready for next step
            run_steps(Some(execution_id), f)
        },
        Err(err) => {
            log_msg!("[SDK] error calling user provided function {:?}", err);

            // if err and error handler is defined, call error handler and break TODO: user provided error handler
            // if err and error handler is not defined, send default signal and break TODO: user provided error handler
            Executor::signal(&ExecutionSignal::new(
                &execution_id,
                Some(prev_state.get_index()),
                SignalKind::Kill(KillReason::Unhandled),
            ))?;

            Err(err)
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::executor::Submitter;
    use codec::alloc::sync::Mutex;
    use lazy_static::lazy_static;
    use scale_info::prelude::{collections::HashMap, vec};
    use t3rn_sdk_primitives::{
        state::{GetExecutionId, GetSteps, Getters},
        xc::{Chain, Operation},
        Vec,
    };

    type Hash = [u8; 32];
    type AccountId = [u8; 32];

    const CALLER: AccountId = [1_u8; 32];
    const CONTRACT: AccountId = [2_u8; 32];
    const EXECUTION_ID: Hash = [50_u8; 32];

    #[derive(Encode, Decode, Default)]
    struct State(Vec<u8>);

    struct TestProvider;

    lazy_static! {
        static ref STATE: Mutex<HashMap<Hash, Vec<u8>>> = Mutex::new(HashMap::new());
        static ref SIGNALS: Mutex<HashMap<Hash, Vec<u8>>> = Mutex::new(HashMap::new());
    }

    impl StateHandler<Hash, State> for TestProvider {
        fn get_state(execution_id: Option<Hash>) -> Result<State, executor::Error> {
            match STATE.lock().unwrap().get(&execution_id.unwrap()) {
                Some(bytes) => Ok(State(bytes.clone())),
                None => Ok(State(Vec::new())),
            }
        }
    }

    impl Submitter<SideEffects<AccountId, u128, Hash>> for TestProvider {
        fn submit(state: SideEffects<AccountId, u128, Hash>) -> Result<(), executor::Error> {
            STATE
                .lock()
                .unwrap()
                .insert(state.execution_id, state.encode());
            Ok(())
        }
    }

    impl GetExecutionId<Hash> for State {
        fn get_execution_id(&self) -> &Hash {
            &EXECUTION_ID
        }
    }

    impl GetSteps for State {
        fn get_index(&self) -> u32 {
            let guard = STATE.lock().unwrap();
            let state = guard.get(&EXECUTION_ID);
            if state.is_some() {
                1
            } else {
                0
            }
        }

        fn get_len(&self) -> u32 {
            1
        }

        fn reached_end(&self) -> bool {
            self.get_index() == self.get_len()
        }
    }

    impl Getters<Hash> for State {}

    impl Signaller<Hash> for TestProvider {
        type Result = Result<(), executor::Error>;

        fn signal(signal: &ExecutionSignal<Hash>) -> Result<(), executor::Error> {
            SIGNALS
                .lock()
                .unwrap()
                .insert(signal.execution_id, signal.encode());
            Ok(())
        }
    }

    // Ignored since we disallow executors, need to have a test executor
    #[ignore]
    #[test]
    fn test_run_step_provides_state_in_a_deterministic_way() {
        let mut state = Step::default();
        state
            .try_push(Chain::<_, _, _>::Polkadot(Operation::Transfer {
                caller: CALLER,
                to: CONTRACT,
                amount: 500,
                insurance: None,
            }))
            .unwrap();
        let f = Box::new(move |_state: &ExecutionState<Hash, AccountId, _, u64>| Ok(state.clone()));

        execute(Some(EXECUTION_ID), f).unwrap();
        let guard = STATE.lock().unwrap();
        let state = guard.get(&EXECUTION_ID).unwrap();
        assert_eq!(
            state,
            &vec![
                // 1, // option some TODO: check why this is removed
                50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50,
                50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, // exec id
                112, 111, 108, 107, 16, // polk
                116, 114, 97, 110, // tran
                128, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, // caller
                128, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
                2, 2, 2, 2, 2, 2, //contract
                16, 244, 1, // amt
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, // uber padding
            ]
        );
    }
}
