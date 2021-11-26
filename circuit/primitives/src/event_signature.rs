#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::vec::*;

type StrLike = Vec<u8>;

pub fn ensure_str_err(condition: bool, err_message: &'static str) -> Result<(), &'static str> {
    if !condition {
        return Err(err_message);
    }
    Ok(())
}

pub fn decode_signature(signature: StrLike) -> Result<(StrLike, Vec<StrLike>), &'static str> {
    // Constants
    const WHITESPACE_MATRIX: [u8; 4] = [b' ', b'\t', b'\r', b'\n'];
    const ARGS_SEPARATOR: u8 = b',';
    const ARGS_START: u8 = b'(';
    const ARGS_END: u8 = b')';

    // Mutable variables
    let mut event_name: Option<StrLike> = None;
    let mut event_args: Vec<StrLike> = Vec::new();
    let mut current_word: StrLike = StrLike::new();

    // Helper functions

    // Trims all whitespace chars from io_schedule vector
    fn trim_whitespace(input_string: StrLike) -> StrLike {
        let mut result = input_string.clone();

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

    // Actual signature decoding start
    check_overall_sanity(signature.clone())?;

    for &char in signature.iter() {
        match char {
            // Expect to start with an event name before the arguments start
            ARGS_START => {
                if current_word.is_empty() {
                    return Err("Signature must have non-empty event name");
                }
                event_name = Some(current_word.clone());
                current_word.clear();
            }
            // Before pushing next non-empty argument name make sure the name is already set
            ARGS_SEPARATOR | ARGS_END => {
                if current_word.is_empty() {
                    return Err("Signature's argument name can't be empty");
                }
                if event_name.is_none() {
                    return Err("Signature must start with event name");
                }
                event_args.push(current_word.clone());
                current_word.clear();
            }
            // Push non-special character to the current word
            _ => current_word.push(char),
        };
    }

    // Check sanity of result before returning
    let event_name_res = match event_name {
        Some(name) => Ok(name),
        None => Err("Signature must have non-empty event name"),
    }?;

    return Ok((event_name_res, event_args));
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn successfully_decodes_signature_for_transfer_confirmation_event() {
        let valid_signature_transfer_confirm_event = "Transfer(from,to,value)";
        // Important! If using .encode() instead of .as_bytes() + .to_vec(),
        //  SCALE adds additional byte "92" to event name
        let decode_res =
            decode_signature(valid_signature_transfer_confirm_event.as_bytes().to_vec());
        assert_eq!(
            decode_res,
            Ok((
                b"Transfer".to_vec(),
                vec![b"from".to_vec(), b"to".to_vec(), b"value".to_vec()]
            ))
        )
    }

    #[test]
    fn fails_to_decode_signature_when_does_not_end_with_closing_bracket() {
        let valid_signature_transfer_confirm_event = "Transfer(from,to,value";
        // Important! If using .encode() instead of .as_bytes() + .to_vec(),
        //  SCALE adds additional byte "92" to event name
        let decode_res =
            decode_signature(valid_signature_transfer_confirm_event.as_bytes().to_vec());
        assert_eq!(
            decode_res,
            Err("Signature sanity failed - must end with ')'")
        )
    }

    #[test]
    fn fails_to_decode_signature_when_too_many_closing_brackets() {
        let valid_signature_transfer_confirm_event = "Transfer(from,to,value))))";
        // Important! If using .encode() instead of .as_bytes() + .to_vec(),
        //  SCALE adds additional byte "92" to event name
        let decode_res =
            decode_signature(valid_signature_transfer_confirm_event.as_bytes().to_vec());
        assert_eq!(decode_res, Err("Signature's argument name can't be empty"))
    }

    #[test]
    fn fails_to_decode_signature_when_empty_arg_name() {
        let valid_signature_transfer_confirm_event = "Transfer(from,to,)";
        // Important! If using .encode() instead of .as_bytes() + .to_vec(),
        //  SCALE adds additional byte "92" to event name
        let decode_res =
            decode_signature(valid_signature_transfer_confirm_event.as_bytes().to_vec());
        assert_eq!(decode_res, Err("Signature's argument name can't be empty"))
    }

    #[test]
    fn fails_to_decode_signature_when_no_opening_bracket() {
        let valid_signature_transfer_confirm_event = "Transfer,from,to,value)";
        // Important! If using .encode() instead of .as_bytes() + .to_vec(),
        //  SCALE adds additional byte "92" to event name
        let decode_res =
            decode_signature(valid_signature_transfer_confirm_event.as_bytes().to_vec());
        assert_eq!(decode_res, Err("Signature must start with event name"))
    }

    #[test]
    fn fails_to_decode_signature_when_empty_event_name() {
        let valid_signature_transfer_confirm_event = "(from,to,value)";
        // Important! If using .encode() instead of .as_bytes() + .to_vec(),
        //  SCALE adds additional byte "92" to event name
        let decode_res =
            decode_signature(valid_signature_transfer_confirm_event.as_bytes().to_vec());
        assert_eq!(decode_res, Err("Signature must have non-empty event name"))
    }
}
