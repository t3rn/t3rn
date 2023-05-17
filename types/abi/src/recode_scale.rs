use crate::{
    recode::{Codec, Recode},
    to_abi::Abi,
    to_filled_abi::FilledAbi,
    types::Name,
};

use bytes::{Buf, Bytes};
use codec::Decode;
use frame_support::ensure;
use sp_runtime::DispatchError;
use sp_std::{prelude::*, vec::IntoIter};
pub struct RecodeScale;

impl Recode for RecodeScale {
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

impl RecodeScale {
    pub fn try_decode_field_by_name_from_scale<D: Decode>(
        data: Vec<u8>,
        abi_descriptor: Vec<u8>,
        name: Vec<u8>,
    ) -> Result<D, DispatchError> {
        let abi: Abi = abi_descriptor.try_into()?;
        let filled_abi = FilledAbi::try_fill_abi(abi, data, Codec::Scale)?;
        let data_by_name = filled_abi
            .get_data_by_name(&name)
            .ok_or(DispatchError::Other(
                "RecodeScale::try_decode_by_name_from_scale - can't access field by given name",
            ))?;

        D::decode(&mut &data_by_name[..]).map_err(|_| {
            DispatchError::Other("RecodeScale::try_decode_by_name_from_scale - decoding error")
        })
    }
}
