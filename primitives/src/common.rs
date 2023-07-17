use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;

pub const SECONDS_PER_HOUR: u32 = 3600;
pub const SECONDS_PER_YEAR: u32 = 31557600;
pub const SECONDS_PER_BLOCK: u32 = 12;
pub const BLOCKS_PER_HOUR: u32 = SECONDS_PER_HOUR / SECONDS_PER_BLOCK;
pub const BLOCKS_PER_DAY: u32 = 24 * BLOCKS_PER_HOUR;
pub const BLOCKS_PER_YEAR: u32 = SECONDS_PER_YEAR / SECONDS_PER_BLOCK;
pub const DEFAULT_ROUND_TERM: u32 = 6 * BLOCKS_PER_HOUR;

/// A range consisting of min, ideal, and max.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(
    Eq,
    PartialEq,
    Clone,
    Copy,
    Encode,
    Decode,
    Default,
    RuntimeDebug,
    MaxEncodedLen,
    TypeInfo,
    PartialOrd,
    Ord,
)]
pub struct Range<T> {
    pub min: T,
    pub ideal: T,
    pub max: T,
}

impl<T: Ord> Range<T> {
    pub fn is_valid(&self) -> bool {
        self.min <= self.ideal && self.ideal <= self.max
    }
}

/// Round identifier (one-based).
pub type RoundIndex = u32;

/// General round information consisting ofindex (one-based), head
/// (beginning block number), and term (round length in number of blocks).
#[derive(Copy, Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct RoundInfo<BlockNumber> {
    /// Current round index.
    pub index: RoundIndex,
    /// The first block of the current round.
    pub head: BlockNumber,
    /// The length of the current round in number of blocks.
    pub term: BlockNumber,
}

impl<
        B: Copy + sp_std::ops::Add<Output = B> + sp_std::ops::Sub<Output = B> + From<u32> + PartialOrd,
    > RoundInfo<B>
{
    pub fn new(index: RoundIndex, head: B, term: B) -> RoundInfo<B> {
        RoundInfo { index, head, term }
    }

    /// Check if the round should be updated
    pub fn should_update(&self, now: B) -> bool {
        now - self.head >= self.term
    }

    /// New round
    pub fn update(&mut self, now: B) {
        self.index = self.index.saturating_add(1u32);
        self.head = now;
    }
}

impl<
        B: Copy + sp_std::ops::Add<Output = B> + sp_std::ops::Sub<Output = B> + From<u32> + PartialOrd,
    > Default for RoundInfo<B>
{
    fn default() -> RoundInfo<B> {
        RoundInfo::new(1u32, 1u32.into(), 299u32.into())
    }
}

/// An ordered set backed by `Vec`.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(RuntimeDebug, PartialEq, Eq, Encode, Decode, Default, Clone, TypeInfo)]
pub struct OrderedSet<T>(pub Vec<T>);

impl<T: Ord> OrderedSet<T> {
    /// Create a new empty set
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Create a set from a `Vec`.
    /// `v` will be sorted and dedup first.
    pub fn from(mut v: Vec<T>) -> Self {
        v.sort();
        v.dedup();
        Self::from_sorted_set(v)
    }

    /// Create a set from a `Vec`.
    /// Assume `v` is sorted and contain unique elements.
    pub fn from_sorted_set(v: Vec<T>) -> Self {
        Self(v)
    }

    /// Insert an element.
    /// Return true if insertion happened.
    pub fn insert(&mut self, value: T) -> bool {
        match self.0.binary_search(&value) {
            Ok(_) => false,
            Err(loc) => {
                self.0.insert(loc, value);
                true
            },
        }
    }

    /// Remove an element.
    /// Return true if removal happened.
    pub fn remove(&mut self, value: &T) -> bool {
        match self.0.binary_search(value) {
            Ok(loc) => {
                self.0.remove(loc);
                true
            },
            Err(_) => false,
        }
    }

    /// Return if the set contains `value`
    pub fn contains(&self, value: &T) -> bool {
        self.0.binary_search(value).is_ok()
    }

    /// Clear the set
    pub fn clear(&mut self) {
        self.0.clear();
    }
}

impl<T: Ord> From<Vec<T>> for OrderedSet<T> {
    fn from(v: Vec<T>) -> Self {
        Self::from(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_update() {
        let mut r = RoundInfo::<u32>::new(1, 1, 400);
        assert_eq!(r.should_update(402), true);
        assert_eq!(r.should_update(401), true);
        r.head = 50;
        assert_eq!(r.should_update(50), false);
    }
}
