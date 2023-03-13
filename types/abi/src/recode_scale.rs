use crate::{
    recode::{split_bytes, Codec, Recode},
    to_abi::Abi,
    to_filled_abi::FilledAbi,
    types::Name,
};

use sp_runtime::DispatchError;
use sp_std::{prelude::*, vec::IntoIter};

pub struct RecodeScale;

impl Recode for RecodeScale {
    // Removes the first byte of the input data for SCALE encoded data and chops the data into fields by the given ABI size.
    fn chop_encoded(
        field_data: &[u8],
        fields_iter_clone: IntoIter<Box<Abi>>,
    ) -> Result<(IntoIter<Vec<u8>>, u8), DispatchError> {
        let (memo_prefix, right) = split_bytes(field_data, 1)?;
        let mut no_strut_prefix_data = right;
        let fields_iter = fields_iter_clone.peekable();
        let chopped_field_data: Vec<Vec<u8>> =
            // Make sure original fields iterator won't be consumed
            // let fields_iter_clone = fields_descriptors.iter().cloned();
            fields_iter
                .map(|field_descriptor| {
                    let field_size = field_descriptor.get_size();
                    let (left, right) = split_bytes(no_strut_prefix_data, field_size)?;
                    no_strut_prefix_data = right;
                    Ok(left.to_vec())
                })
                .collect::<Result<Vec<Vec<u8>>, DispatchError>>()?;

        Ok((
            chopped_field_data.into_iter(),
            *memo_prefix
                .first()
                .expect("encoded_struct_chopper - Memo cannot be empty for structs"),
        ))
    }

    fn event_to_filled(
        field_data: &[u8],
        name: Option<Name>,
        fields_iter_clone: IntoIter<Box<Abi>>,
    ) -> Result<(FilledAbi, usize), DispatchError> {
        FilledAbi::recursive_fill_abi(
            Abi::Enum(name, fields_iter_clone.collect()),
            field_data,
            Codec::Scale,
        )
    }
}
