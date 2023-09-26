use circuit_mock_runtime::{Circuit, Sudo, *};
use circuit_runtime_types::{AccountId, Balance};
use codec::Decode;

use hex;
use serde::Deserialize;

use sp_core::{sr25519, Pair};
use sp_runtime::{AccountId32, DispatchError, DispatchErrorWithPostInfo};
pub use t3rn_primitives::SpeedMode;
pub use t3rn_types::{
    bid::SFXBid,
    fsx::FullSideEffect,
    sfx::{ConfirmedSideEffect, HardenedSideEffect, SecurityLvl, SideEffect, SideEffectId},
};

type Call = circuit_mock_runtime::Call;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExtrinsicParam {
    pub signer: String,
    pub section: String,
    pub method: String,
    pub args: Vec<EncodedArg>,
    pub submission_height: Option<u32>,
    pub events: Vec<EncodedEvent>,
    #[serde(with = "hex::serde")]
    pub error: Vec<u8>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EncodedArg {
    pub name: String,
    pub rust_type: String,
    #[serde(with = "hex::serde")]
    pub encoded: Vec<u8>,
    pub decoded: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EncodedEvent {
    pub section: String,
    pub method: String,
    #[serde(with = "hex::serde")]
    pub encoded: Vec<u8>,
    pub decoded: String,
}

pub enum ErrorWrapper {
    Dispatch(DispatchError),
    DispatchPostInfo(DispatchErrorWithPostInfo<frame_support::weights::PostDispatchInfo>),
}

pub fn replay_and_evaluate_extrinsic<Runtime>(param: &ExtrinsicParam) -> Result<(), DispatchError> {
    // update the chain to the submission height, and trigger clock
    advance_to_block(param.submission_height);

    match param.section.as_str() {
        "circuit" => match param.method.as_str() {
            "onExtrinsicTrigger" => {
                let sfxs = decode_side_effect(&param.args[0].encoded)?;
                let speed_mode = decode_speed_mode(&param.args[1].encoded)?;
                match Circuit::on_extrinsic_trigger(
                    get_signer(&param.signer),
                    sfxs,
                    speed_mode,
                    SecurityLvl::Optimistic,
                ) {
                    Ok(_) => verify_extrinsic_success::<Runtime>(param)?,
                    Err(err) =>
                        { verify_extrinsic_error(ErrorWrapper::DispatchPostInfo(err), param) }?,
                };
                verify_event_log::<Runtime>(&param.events)?;
                Ok(())
            },
            _ => panic!("Unknown Method!"),
        },
        "sudo" => match param.method.as_str() {
            "sudo" => {
                let call = decode_call(&param.args[0].encoded)?;
                match Sudo::sudo(get_signer(&param.signer), Box::new(call)) {
                    Ok(_) => verify_extrinsic_success::<Runtime>(param)?,
                    Err(err) => verify_extrinsic_error(ErrorWrapper::DispatchPostInfo(err), param)?,
                }
                verify_event_log::<Runtime>(&param.events)?;
                Ok(())
            },
            _ => panic!("Unknown Method!"),
        },
        "utility" => match param.method.as_str() {
            "batch" => {
                let batch = decode_batch(&param.args[0].encoded)?;
                match Utility::batch(get_signer(&param.signer), batch) {
                    Ok(_) => {
                        println!("Batch success");
                        verify_extrinsic_success::<Runtime>(param)
                    }?,
                    Err(err) => verify_extrinsic_error(ErrorWrapper::DispatchPostInfo(err), param)?,
                };
                Ok(())
            },
            _ => panic!("Unknown Method!"),
        },
        _ => panic!("Invalid Pallet!"),
    }
}

pub fn verify_extrinsic_error(
    error: ErrorWrapper,
    params: &ExtrinsicParam,
) -> Result<(), DispatchError> {
    if params.error.is_empty() {
        return Err(DispatchError::Other("Received Error was not expected!"))
    }
    let received_error = match error {
        ErrorWrapper::Dispatch(error) => error,
        ErrorWrapper::DispatchPostInfo(DispatchErrorWithPostInfo { error, .. }) => error,
    };

    // It seems like client side export is always done as DispatchError
    let expected_error = DispatchError::decode(&mut params.error.as_slice()).unwrap();
    if received_error != expected_error {
        return Err(DispatchError::Other(
            "Received Error does not match expected error!",
        ))
    }

    Ok(())
}

pub fn advance_to_block(block: Option<u32>) {
    System::reset_events();
    if let Some(height) = block {
        System::set_block_number(height);
        <Clock as frame_support::traits::OnInitialize<BlockNumber>>::on_initialize(height);
    }
}

fn verify_extrinsic_success<T>(extrinsic_params: &ExtrinsicParam) -> Result<(), DispatchError> {
    if !extrinsic_params.error.is_empty() {
        return Err(DispatchError::Other("Received Ok was not expected!"))
    }
    Ok(())
}

//ToDo: currently, this function compared the event log with the expected events. This is a result of the mock runtime not emitting any of the TX fee events.
// Once standalone and mock are aligned, we check that every expected event is included
fn verify_event_log<T>(events: &Vec<EncodedEvent>) -> Result<(), DispatchError> {
    let event_log = System::events();
    let expected_events = events
        .iter()
        .filter_map(|event| {
            if event.section == "sudo" {
                None // ignore sudo events for now
            } else {
                Some(
                    circuit_mock_runtime::Event::decode(&mut event.encoded.as_slice())
                        .map_err(|_| DispatchError::Other("Event decoding error!"))
                        .ok()?,
                )
            }
        })
        .collect::<Vec<_>>();

    for chain_event in event_log {
        if let false = expected_events.iter().any(|record| {
            if let Event::Sudo(_) = chain_event.event {
                return true // ignore sudo events for now
            }
            record == &chain_event.event
        }) {
            return Err(DispatchError::Other("Event missmatch!"))
        }
    }
    Ok(())
}

pub fn decode_speed_mode(input: &Vec<u8>) -> Result<SpeedMode, DispatchError> {
    SpeedMode::decode(&mut input.as_slice())
        .map_err(|_| DispatchError::Other("SpeedMode deocding error!"))
}

pub fn decode_side_effect(
    input: &Vec<u8>,
) -> Result<Vec<SideEffect<AccountId, Balance>>, DispatchError> {
    let result: Vec<SideEffect<AccountId, Balance>> = Decode::decode(&mut input.as_slice())
        .map_err(|_| DispatchError::Other("SideEffect decoding error!"))?;
    Ok(result)
}

fn decode_call(input: &Vec<u8>) -> Result<Call, DispatchError> {
    Call::decode(&mut input.as_slice()).map_err(|_| DispatchError::Other("Call decoding error!"))
}

fn decode_batch(input: &Vec<u8>) -> Result<Vec<Call>, DispatchError> {
    let result: Vec<Call> = Decode::decode(&mut input.as_slice())
        .map_err(|_| DispatchError::Other("Batch decoding error!"))?;
    Ok(result)
}

pub fn get_signer(address: &String) -> Origin {
    let seed = match address.as_str() {
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" => "//Alice",
        "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty" => "//Bob",
        "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y" => "//Charlie",
        _ => panic!("Unknown signer!"),
    };

    let private_key = sr25519::Pair::from_string(seed, None).unwrap();
    let public_key = private_key.public();
    Origin::signed(AccountId32::from(public_key))
}
