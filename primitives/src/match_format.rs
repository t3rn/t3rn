use codec::Encode;
use sp_std::{vec, vec::*};
use t3rn_types::Bytes;

pub type StrLike = Vec<u8>;

pub fn ensure_str_err(condition: bool, err_message: &'static str) -> Result<(), &'static str> {
    if !condition {
        return Err(err_message)
    }
    Ok(())
}

// Constants
const WHITESPACE_MATRIX: [u8; 4] = [b' ', b'\t', b'\r', b'\n'];
const ARGS_SEPARATOR: u8 = b',';
const ARGS_START: u8 = b'(';
const ARGS_END: u8 = b')';

// Helper functions

// Checks if signature is no-empty and ends correctly
fn check_overall_sanity(signature: StrLike) -> Result<(), &'static str> {
    let cloned = trim_whitespace(signature);
    // make sure schedule is not empty
    // probably irrelevant since there is already a check for that
    let last_char = cloned.last();
    ensure_str_err(
        last_char.is_some(),
        "Signature sanity failed - can't be empty",
    )?;
    // make sure the schedule ends correctly and remove ending character or panic
    let ends_correctly = last_char.eq(&Some(&ARGS_END));
    ensure_str_err(
        ends_correctly,
        "Signature sanity failed - must end with ')'",
    )
}

// Trims all whitespace chars from io_schedule vector
pub fn trim_whitespace(input_string: StrLike) -> StrLike {
    let mut result = input_string;

    // checks if character is whitespace
    let is_whitespace = |x: &u8| WHITESPACE_MATRIX.contains(x);

    let mut i = 0;
    while i < result.len() {
        if is_whitespace(&result[i]) {
            result.remove(i);
        } else {
            i += 1;
        }
    }
    result
}

pub fn match_side_effect(kind: &StrLike) -> Result<Bytes, &'static str> {
    match &kind[..] {
        b"Transfer" => Ok(b"tran".encode()),
        b"Call" => Ok(b"call".encode()),
        b"Swap" => Ok(b"swap".encode()),
        b"MultiTransfer" => Ok(b"mult".encode()),
        b"AddLiquidity" => Ok(b"aliq".encode()),
        _ => Err("Unknown side effect kind"),
    }
}

pub fn match_signature(signature: StrLike) -> Result<(StrLike, Vec<StrLike>), &'static str> {
    // Mutable variables
    let mut event_name: Option<StrLike> = None;
    let mut event_args: Vec<StrLike> = Vec::new();
    let mut current_word: StrLike = StrLike::new();

    // Actual signature decoding start
    check_overall_sanity(signature.clone())?;

    for &char in signature.iter() {
        match char {
            // Expect to start with an event name before the arguments start
            ARGS_START => {
                if current_word.is_empty() {
                    return Err("Signature must have non-empty event name")
                }
                event_name = Some(current_word.clone());
                current_word.clear();
            },
            // Before pushing next non-empty argument name make sure the name is already set
            ARGS_SEPARATOR | ARGS_END => {
                if current_word.is_empty() {
                    return Err("Signature's argument name can't be empty")
                }
                if event_name.is_none() {
                    return Err("Signature must start with event name")
                }
                event_args.push(current_word.clone());
                current_word.clear();
            },
            // Push non-special character to the current word
            _ => current_word.push(char),
        };
    }

    // Check sanity of result before returning
    let event_name_res = match event_name {
        Some(name) => Ok(name),
        _ => Err("Signature must have non-empty event name"),
    }?;

    Ok((event_name_res, event_args))
}

pub fn match_dfd(generic_dfd: StrLike) -> Result<Vec<Vec<StrLike>>, &'static str> {
    // Mutable variables
    let mut steps: Vec<Vec<StrLike>> = vec![vec![]];
    let mut curr_step_index: usize = 0;
    let mut current_word: StrLike = StrLike::new();

    // Actual generic_dfd decoding start
    let cloned = trim_whitespace(generic_dfd);
    // make sure schedule is not empty
    // probably irrelevant since there is already a check for that
    let last_char = cloned.last();
    ensure_str_err(
        last_char.is_some(),
        "Signature sanity failed - can't be empty",
    )?;

    for &char in cloned.iter() {
        match char {
            ARGS_START | ARGS_SEPARATOR | ARGS_END => {
                if !current_word.is_empty() {
                    if let Some(last_step) = steps.get_mut(curr_step_index) {
                        last_step.push(current_word.clone());
                    } else {
                        return Err("DFD Decoder - attempt to edit step at incorrect depth")
                    }
                    current_word.clear();
                }
                if char == ARGS_START {
                    curr_step_index += 1;
                    steps.push(vec![])
                }
                if char == ARGS_SEPARATOR {
                    current_word.clear();
                }
                if char == ARGS_END {
                    if let Some(new_step_index) = curr_step_index.checked_sub(1) {
                        curr_step_index = new_step_index;
                    } else {
                        return Err("DFD Decoder - attempt to edit step at incorrect depth")
                    }
                }
            },
            // Push non-special character to the current word
            _ => current_word.push(char),
        };
    }

    if curr_step_index != 0 {
        return Err("DFD Decoder - too many opening brackets")
    }

    // If last word didn't end with , or )
    if !current_word.is_empty() {
        if let Some(last_step) = steps.get_mut(curr_step_index) {
            last_step.push(current_word.clone());
        } else {
            return Err("DFD Decoder - attempt to edit step at incorrect depth")
        }
        current_word.clear();
    }

    // Trim empty steps (support additional depths of DFD, like ((((A,B))) )
    steps.retain(|step| !step.is_empty());

    // Reverse the steps order
    steps.reverse();

    Ok(steps)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn successfully_matches_signature_for_transfer_confirmation_event() {
        let valid_signature_transfer_confirm_event = "Transfer(from,to,value)";
        // Important! If using .encode() instead of .as_bytes() + .to_vec(),
        //  SCALE adds additional byte "92" to event name
        let decode_res =
            match_signature(valid_signature_transfer_confirm_event.as_bytes().to_vec());
        assert_eq!(
            decode_res,
            Ok((
                b"Transfer".to_vec(),
                vec![b"from".to_vec(), b"to".to_vec(), b"value".to_vec()]
            ))
        )
    }

    #[test]
    fn fails_to_match_signature_when_does_not_end_with_closing_bracket() {
        let valid_signature_transfer_confirm_event = "Transfer(from,to,value";
        // Important! If using .encode() instead of .as_bytes() + .to_vec(),
        //  SCALE adds additional byte "92" to event name
        let decode_res =
            match_signature(valid_signature_transfer_confirm_event.as_bytes().to_vec());
        assert_eq!(
            decode_res,
            Err("Signature sanity failed - must end with ')'")
        )
    }

    #[test]
    fn fails_to_match_signature_when_too_many_closing_brackets() {
        let valid_signature_transfer_confirm_event = "Transfer(from,to,value))))";
        // Important! If using .encode() instead of .as_bytes() + .to_vec(),
        //  SCALE adds additional byte "92" to event name
        let decode_res =
            match_signature(valid_signature_transfer_confirm_event.as_bytes().to_vec());
        assert_eq!(decode_res, Err("Signature's argument name can't be empty"))
    }

    #[test]
    fn fails_to_match_signature_when_empty_arg_name() {
        let valid_signature_transfer_confirm_event = "Transfer(from,to,)";
        // Important! If using .encode() instead of .as_bytes() + .to_vec(),
        //  SCALE adds additional byte "92" to event name
        let decode_res =
            match_signature(valid_signature_transfer_confirm_event.as_bytes().to_vec());
        assert_eq!(decode_res, Err("Signature's argument name can't be empty"))
    }

    #[test]
    fn fails_to_match_signature_when_no_opening_bracket() {
        let valid_signature_transfer_confirm_event = "Transfer,from,to,value)";
        // Important! If using .encode() instead of .as_bytes() + .to_vec(),
        //  SCALE adds additional byte "92" to event name
        let decode_res =
            match_signature(valid_signature_transfer_confirm_event.as_bytes().to_vec());
        assert_eq!(decode_res, Err("Signature must start with event name"))
    }

    #[test]
    fn fails_to_match_signature_when_empty_event_name() {
        let valid_signature_transfer_confirm_event = "(from,to,value)";
        // Important! If using .encode() instead of .as_bytes() + .to_vec(),
        //  SCALE adds additional byte "92" to event name
        let decode_res =
            match_signature(valid_signature_transfer_confirm_event.as_bytes().to_vec());
        assert_eq!(decode_res, Err("Signature must have non-empty event name"))
    }

    #[test]
    fn successfully_matches_dfd_for_3_parallel_events() {
        let valid_dfd = "A,B,C";
        // Important! If using .encode() instead of .as_bytes() + .to_vec(),
        //  SCALE adds additional byte "92" to event name
        let decode_res = match_dfd(valid_dfd.as_bytes().to_vec());
        assert_eq!(
            decode_res,
            Ok(vec![vec![b"A".to_vec(), b"B".to_vec(), b"C".to_vec()]])
        )
    }

    #[test]
    fn successfully_matches_dfd_for_3_parallel_events_in_single_brackets() {
        let valid_dfd = "(A,B,C)";
        // Important! If using .encode() instead of .as_bytes() + .to_vec(),
        //  SCALE adds additional byte "92" to event name
        let decode_res = match_dfd(valid_dfd.as_bytes().to_vec());
        assert_eq!(
            decode_res,
            Ok(vec![vec![b"A".to_vec(), b"B".to_vec(), b"C".to_vec()]])
        )
    }

    #[test]
    fn successfully_matches_dfd_for_3_parallel_events_in_triple_brackets() {
        let valid_dfd = "(((A,B,C)))";
        // Important! If using .encode() instead of .as_bytes() + .to_vec(),
        //  SCALE adds additional byte "92" to event name
        let decode_res = match_dfd(valid_dfd.as_bytes().to_vec());
        assert_eq!(
            decode_res,
            Ok(vec![vec![b"A".to_vec(), b"B".to_vec(), b"C".to_vec()]])
        )
    }

    #[test]
    fn fails_to_match_dfd_for_3_parallel_events_with_incorrect_closing_brackets() {
        let valid_dfd = "(A,B,C";
        // Important! If using .encode() instead of .as_bytes() + .to_vec(),
        //  SCALE adds additional byte "92" to event name
        let decode_res = match_dfd(valid_dfd.as_bytes().to_vec());
        assert_eq!(decode_res, Err("DFD Decoder - too many opening brackets"))
    }

    #[test]
    fn fails_to_match_dfd_for_3_parallel_events_with_incorrect_opening_brackets() {
        let valid_dfd = "A,B,C)";
        // Important! If using .encode() instead of .as_bytes() + .to_vec(),
        //  SCALE adds additional byte "92" to event name
        let decode_res = match_dfd(valid_dfd.as_bytes().to_vec());
        assert_eq!(
            decode_res,
            Err("DFD Decoder - attempt to edit step at incorrect depth")
        )
    }

    #[test]
    fn successfully_matches_dfd_for_3_sequential_events() {
        let valid_dfd = "(A(B(C)))";
        // Important! If using .encode() instead of .as_bytes() + .to_vec(),
        //  SCALE adds additional byte "92" to event name
        let decode_res = match_dfd(valid_dfd.as_bytes().to_vec());
        assert_eq!(
            decode_res,
            Ok(vec![
                vec![b"C".to_vec()],
                vec![b"B".to_vec()],
                vec![b"A".to_vec()],
            ])
        )
    }

    #[test]
    fn successfully_matches_dfd_for_3_sequential_32b_long_events() {
        let valid_dfd = "(0909090909090909090909090909090909090909090909090909090909090909(\
            0606060606060606060606060606060606060606060606060606060606060606(\
                0303030303030303030303030303030303030303030303030303030303030303)))";
        // Important! If using .encode() instead of .as_bytes() + .to_vec(),
        //  SCALE adds additional byte "92" to event name
        let decode_res = match_dfd(valid_dfd.as_bytes().to_vec());
        assert_eq!(
            decode_res,
            Ok(vec![
                vec![b"0303030303030303030303030303030303030303030303030303030303030303".to_vec()],
                vec![b"0606060606060606060606060606060606060606060606060606060606060606".to_vec()],
                vec![b"0909090909090909090909090909090909090909090909090909090909090909".to_vec()],
            ])
        )
    }

    #[test]
    fn successfully_matches_dfd_for_2_sequential_events_after_2_parallel() {
        let valid_dfd = "(D(C(A,B)))";
        // Important! If using .encode() instead of .as_bytes() + .to_vec(),
        //  SCALE adds additional byte "92" to event name
        let decode_res = match_dfd(valid_dfd.as_bytes().to_vec());
        assert_eq!(
            decode_res,
            Ok(vec![
                vec![b"A".to_vec(), b"B".to_vec()],
                vec![b"C".to_vec()],
                vec![b"D".to_vec()],
            ])
        )
    }

    #[test]
    fn test_match_signature() {
        let sig = "Transfer(from,to,value)";
        let decoded_sig = match_signature(sig.as_bytes().to_vec());
        println!("{:?}", decoded_sig);
    }
}
