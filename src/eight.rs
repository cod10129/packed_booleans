//! Packing 8 booleans together into a byte.

use core::iter::FusedIterator;

crate::macros::packed_bools_type!{
    NAME = PackedBools8,
    REPR = u8,
    BOOL_COUNT = 8,
    BCOUNT_MINUS1 = 7,
    BYTE_DESCRIPTION = "a single byte",
    PRETTY_DEBUG = "PackedBools8(\n    {:#010b},\n)",
    DEBUG = "PackedBools8({:#010b})",
    BINARY = "{:08b}",
    LOW_HEX = "{:02x}",
    UPPER_HEX = "{:02X}"
}

impl IntoIterator for PackedBools8 {
    type Item = bool;
    type IntoIter = IntoIter8;

    fn into_iter(self) -> IntoIter8 {
        IntoIter8::new(self)
    }
}

/// This struct is a smaller range than `ops::Range<u8>` for `IntoIter8`,
/// considering the values will only ever go up to 8.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
struct PackedU8Range(u8);

impl PackedU8Range {
    #[inline]
    fn new(start: u8, end: u8) -> Self {
        Self((start << 4) | (end - 1))
    }

    #[inline]
    fn get_start(&self) -> u8 {
        self.0 >> 4
    }

    #[inline]
    fn get_end(&self) -> u8 {
        self.0 & 0b00001111
    }

    /// Note: this method does no guarding against overflows.
    #[inline]
    fn add_to_start(&mut self, val: u8) {
        self.0 += val << 4
    }

    /// Note: this method does no guarding against underflows.
    #[inline]
    fn sub_from_end(&mut self, val: u8) {
        self.0 -= val
    }

    fn iter_next(&mut self) -> Option<u8> {
        let start = self.get_start();
        if self.0 < 0b11110000 && start <= self.get_end() {
            self.add_to_start(1);
            Some(start)
        } else {
            None
        }
    }

    fn iter_next_back(&mut self) -> Option<u8> {
        let end = self.get_end();
        if end > 0 && self.get_start() <= end {
            self.sub_from_end(1);
            Some(end)
        } else {
            None
        }
    }

    #[inline]
    fn len(&self) -> u8 {
        (self.get_end() + 1) - self.get_start()
    }
}

/// An iterator over the booleans in a `PackedBools8`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct IntoIter8 {
    bools: PackedBools8,
    range: PackedU8Range,
}

// Can't even deprecate a trait impl. FIXME REMEMBER TO REMOVE THIS!
// #[deprecated = "Copy iterators are generally a footgun. Use clone()."]
impl Copy for IntoIter8 {}

impl IntoIter8 {
    #[inline]
    fn new(bools: PackedBools8) -> Self {
        Self {
            bools,
            range: PackedU8Range::new(0, 8),
        }
    }
}

impl Iterator for IntoIter8 {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        self.range.iter_next().map(|idx| self.bools.get(idx))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.range.len().into();
        (len, Some(len))
    }

    fn nth(&mut self, n: usize) -> Option<bool> {
        let n = u8::try_from(n).ok().filter(|&n| n < self.range.len())?;
        self.range.add_to_start(n);
        self.next()
    }

    fn last(mut self) -> Option<bool> {
        self.next_back()
    }
}

impl DoubleEndedIterator for IntoIter8 {
    fn next_back(&mut self) -> Option<bool> {
        self.range.iter_next_back().map(|idx| self.bools.get(idx))
    }

    fn nth_back(&mut self, n: usize) -> Option<bool> {
        let n = u8::try_from(n).ok().filter(|&n| n < self.range.len())?;
        self.range.sub_from_end(n);
        self.next_back()
    }
}

impl ExactSizeIterator for IntoIter8 {
    fn len(&self) -> usize {
        self.range.len().into()
    }
}

impl FusedIterator for IntoIter8 {}

#[cfg(test)]
mod tests {
    extern crate alloc;
    use alloc::format;

    use super::PackedBools8;

    #[test]
    fn set_get() {
        let mut pkd = PackedBools8::new();

        pkd.set_all([false, true, false, true, true, false, false, true]);
        pkd.set(false, 3);
        pkd.set(true, 4);

        assert_eq!(
            pkd.get_all(),
            [false, true, false, false, true, false, false, true]
        );
    }

    #[test]
    fn new_vals() {
        let mut pkd = PackedBools8::new();
        let arr = [false, true, false, false, true, false, true, true];

        pkd.set_all(arr);

        assert_eq!(pkd, PackedBools8::new_vals(arr));
    }

    #[test]
    fn formatting() {
        // formats like 11010100
        let pkd = PackedBools8::from([false, false, true, false, true, false, true, true]);
        assert_eq!(format!("{pkd:?}"), "PackedBools8(0b11010100)");
        assert_eq!(format!("{pkd:#?}"), "PackedBools8(\n    0b11010100,\n)");
        assert_eq!(format!("{pkd:b}"), "11010100");
        assert_eq!(format!("{pkd:#b}"), "0b11010100");
        assert_eq!(format!("{pkd:x}"), "d4");
        assert_eq!(format!("{pkd:#x}"), "0xd4");
        assert_eq!(format!("{pkd:X}"), "D4");
        assert_eq!(format!("{pkd:#X}"), "0xD4");
    }

    #[test]
    fn iter() {
        let pkd = PackedBools8::new();
        assert_eq!(pkd.into_iter().len(), 8);
        for b in pkd.into_iter() {
            assert!(!b);
        }
        let arr = [false, true, false, true, false, false, false, true];

        PackedBools8::new_vals(arr)
            .into_iter()
            .zip(arr.into_iter())
            .for_each(|(b1, b2)| assert_eq!(b1, b2));
    }

    #[test]
    fn iter_back() {
        let arr = [true, false, false, true, true, false, false, false];
        PackedBools8::from(arr)
            .into_iter()
            .rev()
            .zip(arr.into_iter().rev())
            .for_each(|(b1, b2)| assert_eq!(b1, b2));
    }

    #[test]
    #[allow(clippy::iter_nth_zero)]
    fn iter_nth() {
        let mut iter =
            PackedBools8::from([true, false, false, true, false, false, true, true]).into_iter();

        assert_eq!(iter.nth(0), Some(true)); // state = [_, false, false, true, false, false, true, true]
        assert_eq!(iter.nth_back(1), Some(true)); // state = [_, false, false, true, false, false, _, _]
        assert_eq!(iter.nth_back(0), Some(false)); // state = [_, false, false, true, false, _, _, _]
        assert_eq!(iter.nth(3), Some(false)); // state = [_, _, _, _, _, _, _, _]
        assert_eq!(iter.nth(0), None);
        assert_eq!(iter.nth_back(0), None);
        assert_eq!(iter.nth(12), None);
        assert_eq!(iter.nth_back(100), None);
    }
}
