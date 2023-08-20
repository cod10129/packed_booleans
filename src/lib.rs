//! A crate for packing booleans together.

#![no_std]

use core::{
    cmp, fmt,
    iter::FusedIterator,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

/// A type containing 8 `bool` values,
/// while only being a single byte.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, PartialEq, Eq, Default, Hash)]
#[repr(transparent)]
pub struct PackedBools(u8);

impl PackedBools {
    /// Creates a new `PackedBools` where all values are false.
    pub fn new() -> Self {
        Self(0)
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

    /// Creates a new `PackedBools` with the given values.
    pub fn new_vals(vals: [bool; 8]) -> PackedBools {
        let mut out = 0;
        vals.into_iter()
            .zip(0..8)
            .for_each(|(b, idx)| out |= u8::from(b) << idx);
        PackedBools(out)
    }

    /// Sets all the booleans to the ones given.
    pub fn set_all(&mut self, vals: [bool; 8]) {
        vals.into_iter()
            .zip(0..8)
            .for_each(|(val, idx)| self.set(val, idx))
    }

    /// Gets all the booleans.
    pub fn get_all(&self) -> [bool; 8] {
        let mut arr = [false; 8];
        arr.iter_mut()
            .zip(0..8)
            .for_each(|(b, idx)| *b = self.get(idx));
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

impl From<[bool; 8]> for PackedBools {
    fn from(bools: [bool; 8]) -> Self {
        Self::new_vals(bools)
    }
}

impl PartialOrd for PackedBools {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PackedBools {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.get_all().cmp(&other.get_all())
    }
}

macro_rules! impl_binop {
    (impl $op:tt $tr:ident $method:ident $assign_tr:ident $assign_method:ident) => {
        // base impl
        impl $tr for PackedBools {
            type Output = PackedBools;

            fn $method(self, rhs: Self) -> PackedBools {
                PackedBools(self.0 $op rhs.0)
            }
        }

        // ref impls
        impl $tr<PackedBools> for &PackedBools {
            type Output = PackedBools;

            fn $method(self, rhs: PackedBools) -> PackedBools {
                $tr::$method(*self, rhs)
            }
        }

        impl $tr<&PackedBools> for PackedBools {
            type Output = PackedBools;

            fn $method(self, rhs: &PackedBools) -> PackedBools {
                $tr::$method(self, *rhs)
            }
        }

        impl $tr<&PackedBools> for &PackedBools {
            type Output = PackedBools;

            fn $method(self, rhs: &PackedBools) -> PackedBools {
                $tr::$method(*self, *rhs)
            }
        }

        // op= impls
        impl $assign_tr<PackedBools> for PackedBools {
            fn $assign_method(&mut self, rhs: Self) {
                *self = self.$method(rhs)
            }
        }

        impl $assign_tr<&PackedBools> for PackedBools {
            fn $assign_method(&mut self, rhs: &PackedBools) {
                *self = self.$method(*rhs)
            }
        }
    }
}

impl_binop! { impl & BitAnd bitand BitAndAssign bitand_assign }
impl_binop! { impl | BitOr bitor BitOrAssign bitor_assign }
impl_binop! { impl ^ BitXor bitxor BitXorAssign bitxor_assign }

impl Not for PackedBools {
    type Output = PackedBools;

    fn not(self) -> Self::Output {
        PackedBools(!self.0)
    }
}

impl Not for &PackedBools {
    type Output = PackedBools;

    fn not(self) -> Self::Output {
        PackedBools(!self.0)
    }
}

impl IntoIterator for PackedBools {
    type Item = bool;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

impl fmt::Debug for PackedBools {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#b}", self.0)
    }
}

impl fmt::Binary for PackedBools {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Binary::fmt(&self.0, f)
    }
}

/// This struct is a smaller range than `ops::Range<u8>` for `IntoIter`,
/// considering the values will only ever go up to 8.
#[repr(transparent)]
struct PackedU8Range(u8);

impl PackedU8Range {
    #[inline]
    fn new(start: u8, end: u8) -> Self {
        Self((start << 4) | end)
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
        if self.0 < 0b11110000 && start < self.get_end() {
            self.add_to_start(1);
            Some(start)
        } else {
            None
        }
    }

    fn iter_next_back(&mut self) -> Option<u8> {
        let end = self.get_end();
        if end > 0 && self.get_start() < end {
            self.sub_from_end(1);
            Some(end)
        } else {
            None
        }
    }

    #[inline]
    fn len(&self) -> u8 {
        self.get_end() - self.get_start()
    }
}

/// An iterator over the booleans in a `PackedBools`.
#[repr(C)]
pub struct IntoIter {
    bools: PackedBools,
    range: PackedU8Range,
}

impl IntoIter {
    #[inline]
    fn new(bools: PackedBools) -> Self {
        Self {
            bools,
            range: PackedU8Range::new(0, 8),
        }
    }
}

impl Iterator for IntoIter {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        self.range.iter_next().map(|idx| self.bools.get(idx))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.range.len().into();
        (len, Some(len))
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let n = u8::try_from(n).ok().filter(|&n| n < self.range.len())?;
        self.range.add_to_start(n);
        self.next()
    }

    fn last(mut self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.next_back()
    }
}

impl DoubleEndedIterator for IntoIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.range.iter_next_back().map(|idx| self.bools.get(idx))
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let n = u8::try_from(n).ok().filter(|&n| n < self.range.len())?;
        self.range.sub_from_end(n);
        self.next_back()
    }
}

impl ExactSizeIterator for IntoIter {
    fn len(&self) -> usize {
        self.range.len().into()
    }
}

impl FusedIterator for IntoIter {}

#[cfg(test)]
mod tests {
    use super::PackedBools;

    #[test]
    fn set_get() {
        let mut pkd = PackedBools::new();

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
        let mut pkd = PackedBools::new();
        let arr = [false, true, false, false, true, false, true, true];

        pkd.set_all(arr);

        assert_eq!(pkd, PackedBools::new_vals(arr));
    }

    #[test]
    fn iter() {
        let pkd = PackedBools::new();
        assert_eq!(pkd.into_iter().len(), 8);
        for b in pkd.into_iter() {
            assert!(!b);
        }
        let arr = [false, true, false, true, false, false, false, true];

        PackedBools::new_vals(arr)
            .into_iter()
            .zip(arr.into_iter())
            .for_each(|(b1, b2)| assert_eq!(b1, b2));
    }

    #[test]
    fn iter_back() {
        let arr = [true, false, false, true, true, false, false, false];
        PackedBools::from(arr)
            .into_iter()
            .rev()
            .zip(arr.into_iter().rev())
            .for_each(|(b1, b2)| assert_eq!(b1, b2));
    }

    #[test]
    #[allow(clippy::iter_nth_zero)]
    fn iter_nth() {
        let mut iter =
            PackedBools::from([true, false, false, true, false, false, true, true]).into_iter();

        assert_eq!(iter.nth(0), Some(true));
        assert_eq!(iter.nth_back(1), Some(true));
        assert_eq!(iter.nth_back(0), Some(false));
        assert_eq!(iter.nth(3), Some(false));
        assert_eq!(iter.nth(0), None);
        assert_eq!(iter.nth_back(0), None);
        assert_eq!(iter.nth(12), None);
        assert_eq!(iter.nth_back(100), None);
    }
}
