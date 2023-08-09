use crate::Vec;
use arrayvec::ArrayVec;
use codec::{Compact, Decode, Encode, EncodeLike, MaxEncodedLen, Output};

// We had to make our own version of this since the one in frame_support is super coupled to the runtime crates
// Test to prove feature parity in 3vm wasm-contracts.

/// A bounded vector using ArrayVec as an underlying data structure.
///
#[derive(scale_info::TypeInfo, Debug, Clone, PartialEq, Eq)]
pub struct BoundedVec<T: Encode + Decode, const CAP: usize>(pub ArrayVec<T, CAP>);

impl<T, const CAP: usize> BoundedVec<T, CAP>
where
    T: Encode + Decode,
{
    pub fn try_push(&mut self, item: T) -> Result<(), T> {
        if self.0.remaining_capacity() > 0 {
            self.0.push(item);
            Ok(())
        } else {
            Err(item)
        }
    }

    pub fn try_pop(&mut self) -> Option<T> {
        self.0.pop()
    }
}

impl<T, const CAP: usize> FromIterator<T> for BoundedVec<T, CAP>
where
    T: Encode + Decode,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let inner = ArrayVec::from_iter(iter);
        Self(inner)
    }
}

impl<T, const CAP: usize> Default for BoundedVec<T, CAP>
where
    T: Encode + Decode,
{
    fn default() -> Self {
        Self(ArrayVec::new())
    }
}

impl<T, const CAP: usize> MaxEncodedLen for BoundedVec<T, CAP>
where
    T: MaxEncodedLen + Encode + Decode,
{
    fn max_encoded_len() -> usize {
        Compact(CAP as u32)
            .encoded_size()
            .saturating_add(CAP.saturating_mul(T::max_encoded_len()))
    }
}

impl<T, const CAP: usize> scale_info::prelude::ops::Deref for BoundedVec<T, CAP>
where
    T: Decode + Encode,
{
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Decode + Encode, const CAP: usize> Decode for BoundedVec<T, CAP> {
    fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
        let inner = Vec::<T>::decode(input)?;
        if inner.len() > CAP {
            return Err("BoundedVec exceeds its limit".into())
        }
        let inner = ArrayVec::from_iter(inner);
        Ok(Self(inner))
    }

    fn skip<I: codec::Input>(input: &mut I) -> Result<(), codec::Error> {
        Vec::<T>::skip(input)
    }
}

impl<T: Decode + Encode, const CAP: usize> Encode for BoundedVec<T, CAP> {
    fn size_hint(&self) -> usize {
        self.0.iter().map(|x| x.size_hint()).sum()
    }

    fn encode_to<W: Output + ?Sized>(&self, dest: &mut W) {
        let bits = self.0.len();
        Compact(bits as u32).encode_to(dest);
        self.0.iter().for_each(|x| x.encode_to(dest));
    }
}

// `BoundedVec`s encode to something which will always decode as a `Vec`.
impl<T: Encode + Decode, const CAP: usize> EncodeLike<Vec<T>> for BoundedVec<T, CAP> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boundec_vec_options() {
        let x = BoundedVec::<_, 3>::from_iter(vec![Some(1_u8), Some(2_u8), Some(3_u8)]);
        assert_eq!(x.encode(), vec![12, 1, 1, 1, 2, 1, 3]);
    }

    #[test]
    fn test_can_push_to_iter() {
        let mut x = BoundedVec::<_, 3>::default();
        x.0.try_push(1_u8).unwrap();
        x.0.try_push(3_u8).unwrap();
        x.0.push(2_u8);
    }
}
