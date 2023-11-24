//! Packing 16 booleans together into 2 bytes.

use core::{iter::FusedIterator, ops};

crate::macros::packed_bools_type!{
    NAME = PackedBools16,
    REPR = u16,
    BOOL_COUNT = 16,
    BCOUNT_MINUS1 = 15,
    BYTE_DESCRIPTION = "two bytes",
    PRETTY_DEBUG = "PackedBools16(\n    {:#018b},\n)",
    DEBUG = "PackedBools16({:#018b})",
    BINARY = "{:016b}",
    LOW_HEX = "{:04x}",
    UPPER_HEX = "{:04X}"
}

impl IntoIterator for PackedBools16 {
    type Item = bool;
    type IntoIter = IntoIter16;

    fn into_iter(self) -> IntoIter16 {
        IntoIter16::new(self)
    }
}

/// An iterator over the booleans in a [`PackedBools16`].
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(not(doc), repr(C))]
pub struct IntoIter16 {
    bools: PackedBools16,
    range: ops::Range<u8>
}

impl IntoIter16 {
    fn new(bools: PackedBools16) -> Self {
        Self { bools, range: 0..16 }
    }
}

impl Iterator for IntoIter16 {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        // try_get here because it strips the panicking path entirely
        // None should never be returned
        // but it should hopefully optimize the unreachable paths out
        self.range.next().and_then(|idx| self.bools.try_get(idx))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }

    fn nth(&mut self, n: usize) -> Option<bool> {
        self.range.nth(n).and_then(|idx| self.bools.try_get(idx))
    }
}

impl DoubleEndedIterator for IntoIter16 {
    fn next_back(&mut self) -> Option<bool> {
        self.range.next_back().and_then(|idx| self.bools.try_get(idx))
    }

    fn nth_back(&mut self, n: usize) -> Option<bool> {
        self.range.nth_back(n).and_then(|idx| self.bools.try_get(idx))
    }
}

impl ExactSizeIterator for IntoIter16 {
    fn len(&self) -> usize {
        self.range.len()
    }
}

impl FusedIterator for IntoIter16 {}

#[cfg(test)]
mod tests {
    use super::PackedBools16;

    const F: bool = false;
    const T: bool = true;

    #[test]
    fn set_get() {
        let mut pkd = PackedBools16::new();
        pkd.set(true, 4);
        pkd.set(true, 8);
        pkd.set(true, 15);
        assert!(!pkd.get(5));
        pkd.set(true, 5);
        assert!(pkd.get(5));
        pkd.set(true, 13);

        assert_eq!(
            pkd.get_all(), [F,F,F,F,T,T,F,F,T,F,F,F,F,T,F,T]
        )
    }

    #[test]
    fn iter() {
        let pkd = PackedBools16::new();
        for b in pkd {
            assert!(!b);
        }

        let arr = [F,F,T,T,T,F,T,F,F,F,F,T,F,T,T,T];
        PackedBools16::from(arr).into_iter()
            .zip(arr.into_iter())
            .for_each(|(a, b)| assert_eq!(a, b));
    }

    #[test]
    fn iter_back() {
        let arr = [F,F,T,T,T,T,F,T,F,F,T,F,F,F,T,T];
        PackedBools16::from(arr).into_iter().rev()
            .zip(arr.into_iter().rev())
            .for_each(|(a, b)| assert_eq!(a, b));
    }
}
