use crate::{Config, Error, Event, Pallet, Signals};
use codec::Encode;
use frame_support::pallet_prelude::Get;
use t3rn_primitives::threevm::SignalOpcode;
use t3rn_sdk_primitives::signal::ExecutionSignal;

const LOG_TARGET: &str = "3vm::signal";

pub(crate) fn signal<T: Config>(
    signal: &ExecutionSignal<<T as frame_system::Config>::Hash>,
) -> Result<SignalOpcode, Error<T>> {
    // TODO[Style]: use mutate
    let new_nonce = match <Signals<T>>::get(signal.execution_id, signal.step) {
        Some(nonce) =>
            if let Some(v) = nonce.checked_add(1) {
                v
            } else {
                return Err(Error::InvalidArithmeticOverflow)
            },
        None => 1,
    };
    log::debug!(
        target: LOG_TARGET,
        "Updating signal: {:?} with nonce {}",
        signal.encode(),
        new_nonce
    );
    <Signals<T>>::insert(signal.execution_id, signal.step, new_nonce);

    if new_nonce > T::SignalBounceThreshold::get() {
        log::debug!(
            target: LOG_TARGET,
            "Signal bounce exceeded threshold {:?}",
            signal.execution_id
        );
        Pallet::<T>::deposit_event(Event::ExceededBounceThrehold((
            signal.step,
            signal.kind,
            signal.execution_id,
        )));
        Err(Error::<T>::ExceededSignalBounceThreshold)?
    } else if new_nonce == 1 {
        log::trace!(target: LOG_TARGET, "Initiating signal {:?}", signal);
        Ok(SignalOpcode::Initiated)
    } else {
        log::trace!(
            target: LOG_TARGET,
            "Signal bounce within threshold {:?}",
            signal.execution_id
        );
        Pallet::<T>::deposit_event(Event::SignalBounced((
            signal.step,
            signal.kind,
            signal.execution_id,
        )));
        Ok(SignalOpcode::Bounced)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::*;
    use frame_support::{assert_err, assert_ok};
    use sp_runtime::traits::Hash;
    use t3rn_sdk_primitives::signal::SignalKind;

    #[test]
    fn test_signal_happy() {
        new_test_ext().execute_with(|| {
            let signal = ExecutionSignal {
                execution_id: <Test as frame_system::Config>::Hashing::hash(&[1u8]),
                step: 0,
                kind: SignalKind::Complete,
            };
            assert_ok!(crate::signal::signal::<Test>(&signal));
            assert!(<Signals<Test>>::get(signal.execution_id, signal.step).is_some());
        });
    }

    #[test]
    fn test_signal_exceeded() {
        new_test_ext().execute_with(|| {
            let signal = ExecutionSignal {
                execution_id: <Test as frame_system::Config>::Hashing::hash(&[1u8]),
                step: 0,
                kind: SignalKind::Complete,
            };
            assert_ok!(
                crate::signal::signal::<Test>(&signal),
                SignalOpcode::Initiated
            );
            let signal = ExecutionSignal {
                execution_id: <Test as frame_system::Config>::Hashing::hash(&[1u8]),
                step: 0,
                kind: SignalKind::Complete,
            };
            assert_ok!(
                crate::signal::signal::<Test>(&signal),
                SignalOpcode::Bounced
            );
            let signal = ExecutionSignal {
                execution_id: <Test as frame_system::Config>::Hashing::hash(&[1u8]),
                step: 0,
                kind: SignalKind::Complete,
            };
            assert_err!(
                crate::signal::signal::<Test>(&signal),
                Error::<Test>::ExceededSignalBounceThreshold
            );

            assert!(<Signals<Test>>::get(signal.execution_id, signal.step).is_some());
        });
    }
}
