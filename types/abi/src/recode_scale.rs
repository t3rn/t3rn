use crate::{
    recode::{Codec, Recode},
    to_abi::Abi,
    to_filled_abi::FilledAbi,
    types::Name,
};

use bytes::{Buf, Bytes};
use frame_support::ensure;
use sp_runtime::DispatchError;
use sp_std::{prelude::*, vec::IntoIter};
pub struct RecodeScale;

impl Recode for RecodeScale {
    fn fill_abi(abi: Abi, field_data: Vec<u8>) -> Result<FilledAbi, DispatchError> {
        let (filled_abi, _size) = FilledAbi::recursive_fill_abi(abi, &field_data, Codec::Scale)?;
        Ok(filled_abi)
    }

    // Removes the first byte of the input data for SCALE encoded data and chops the data into fields by the given ABI size.
    fn chop_encoded(
        field_data: &[u8],
        fields_iter_clone: IntoIter<Box<Abi>>,
    ) -> Result<(IntoIter<Vec<u8>>, u8), DispatchError> {
        let mut buf = Bytes::copy_from_slice(field_data);
        ensure!(
            !buf.is_empty(),
            DispatchError::Other("RecodeScale::chop_encoded - no data to decode")
        );
        let memo_prefix = buf.get_u8();

        let mut no_strut_prefix_data = buf;

        let fields_iter = fields_iter_clone.peekable();
        let chopped_field_data: Vec<Vec<u8>> = fields_iter
            .map(|field_descriptor| {
                let field_size = field_descriptor.get_size();

                let mut field_bytes = vec![0; field_size];
                ensure!(
                    no_strut_prefix_data.len() >= field_size,
                    DispatchError::Other("RecodeScale::chop_encoded - not enough data to decode")
                );
                no_strut_prefix_data.copy_to_slice(&mut field_bytes);

                Ok(field_bytes)
            })
            .collect::<Result<Vec<Vec<u8>>, DispatchError>>()?;

        Ok((chopped_field_data.into_iter(), memo_prefix))
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
