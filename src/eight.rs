//! Packing 8 booleans together into a byte.

use core::{
    fmt, iter::FusedIterator,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

use crate::macros::impl_binop;

/// A type containing 8 `bool` values,
/// while only being a single byte.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, PartialEq, Eq, Default, Hash)]
#[repr(transparent)]
pub struct PackedBools8(u8);

impl PackedBools8 {
    /// Creates a new `PackedBools8` where all values are false.
    pub const fn new() -> Self {
        Self(0)
    }

    /// Creates a new `PackedBools8` from the given bits.
    pub fn from_bits(bits: u8) -> Self {
        Self(bits)
    }

    /// Counts how many true values there are.
    pub fn count_true(&self) -> u8 {
        // This cast is valid because the return value
        // of count_ones should never be more than 8
        // (only 8 bits in `u8`)
        self.0.count_ones() as u8
    }

    /// Counts how many false values there are.
    pub fn count_false(&self) -> u8 {
        // See count_true comment
        self.0.count_zeros() as u8
    }

    /// Creates a new `PackedBools8` with the given values.
    pub fn new_vals(vals: [bool; 8]) -> Self {
        Self(vals.into_iter()
            .zip(0..8u8)
            .fold(0, |acc, (b, idx)| acc.bitor((b as u8) << idx)))
    }

    /// Sets all the booleans to the ones given.
    pub fn set_all(&mut self, vals: [bool; 8]) {
        *self = Self::new_vals(vals);
    }

    /// Gets all the booleans.
    pub fn get_all(&self) -> [bool; 8] {
        let mut arr = [false; 8];
        for (idx, b) in arr.iter_mut().enumerate() {
            *b = ((self.0 >> idx) & 1) != 0
        }
        arr
    }

    /// Gets the boolean at the given index.
    ///
    /// # Panics
    ///
    /// Panics if the given index is greater than 7.
    pub fn get(&self, idx: u8) -> bool {
        self.try_get(idx)
            .expect("The index cannot be greater than 7.")
    }

    /// Gets the boolean at the given index,
    /// if the index is less than 8.
    pub fn try_get(&self, idx: u8) -> Option<bool> {
        if idx < 8 {
            Some(((self.0 >> idx) & 1) != 0)
        } else {
            None
        }
    }

    /// Sets the boolean at the given index to val.
    ///
    /// # Panics
    ///
    /// Panics if the given index is greater than 7.
    pub fn set(&mut self, val: bool, idx: u8) {
        self.try_set(val, idx)
            .expect("The index cannot be greater than 7.")
    }

    /// Sets the boolean at the given index to val,
    /// if the index is less than 8.
    pub fn try_set(&mut self, val: bool, idx: u8) -> Option<()> {
        if idx < 8 {
            match val {
                true => self.0 |= 1 << idx,
                false => self.0 &= !(1 << idx),
            };
            Some(())
        } else {
            None
        }
    }

    /// Toggles the boolean at the given index.
    ///
    /// # Panics
    ///
    /// Panics if the given index is greater than 7.
    pub fn toggle(&mut self, idx: u8) {
        self.try_toggle(idx)
            .expect("The index cannot be greater than 7.")
    }

    /// Toggles the boolean at the given index,
    /// if the index is less than 8.
    pub fn try_toggle(&mut self, idx: u8) -> Option<()> {
        if idx < 8 {
            self.0 ^= 1 << idx;
            Some(())
        } else {
            None
        }
    }
}

impl From<[bool; 8]> for PackedBools8 {
    fn from(bools: [bool; 8]) -> Self {
        Self::new_vals(bools)
    }
}

impl_binop! { impl & for PackedBools8: BitAnd bitand BitAndAssign bitand_assign }
impl_binop! { impl | for PackedBools8: BitOr bitor BitOrAssign bitor_assign }
impl_binop! { impl ^ for PackedBools8: BitXor bitxor BitXorAssign bitxor_assign }

impl Not for PackedBools8 {
    type Output = PackedBools8;

    fn not(self) -> Self::Output {
        PackedBools8(!self.0)
    }
}

impl Not for &PackedBools8 {
    type Output = PackedBools8;

    fn not(self) -> Self::Output {
        PackedBools8(!self.0)
    }
}

impl IntoIterator for PackedBools8 {
    type Item = bool;
    type IntoIter = IntoIter8;

    fn into_iter(self) -> IntoIter8 {
        IntoIter8::new(self)
    }
}

impl fmt::Debug for PackedBools8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            // manual pretty-printing
            // this exists because I need to print the binary representation of self.0
            // #[derive] doesn't work because of this, and nor does DebugTuple.
            write!(f, "PackedBools8(\n    {:#010b},\n)", self.0)
        } else {
            // normal printing
            write!(f, "PackedBools8({:#010b})", self.0)
        }
    }
}

/// Displays the PackedBools8 in binary.
/// Note that the order may not be what you expect.
/// The "first" bool will actually be last in the formatting.
/// Note that this impl is not stable.
impl fmt::Binary for PackedBools8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            f.write_str("0b")?;
        }
        write!(f, "{:08b}", self.0)
    }
}

/// Displays the PackedBools8 in lowercase hexadecimal.
/// See notes on fmt::Binary impl.
impl fmt::LowerHex for PackedBools8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            f.write_str("0x")?;
        }
        write!(f, "{:02x}", self.0)
    }
}

/// Displays the PackedBools8 in uppercase hexadecimal.
/// See notes on fmt::Binary impl.
impl fmt::UpperHex for PackedBools8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            f.write_str("0x")?;
        }
        write!(f, "{:02X}", self.0)
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
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct IntoIter8 {
    bools: PackedBools8,
    range: PackedU8Range,
}

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
