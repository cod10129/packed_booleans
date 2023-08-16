//! A crate for packing booleans together.

#![no_std]

use core::{
    iter::FusedIterator,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

/// A type containing 8 `bool` values,
/// while only being a single byte.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct PackedBools(u8);

impl PackedBools {
    /// Creates a new `PackedBools` where all values are false.
    pub fn new() -> Self {
        Self(0)
    }

    /// Sets all the booleans to the ones given.
    pub fn set_all(&mut self, vals: [bool; 8]) {
        for (val, idx) in vals.into_iter().zip(0..=7) {
            self.set(val, idx)
        }
    }

    /// Gets all the booleans.
    pub fn get_all(&self) -> [bool; 8] {
        let mut arr = [false; 8];
        for idx in 0..=7 {
            arr[<u8 as Into<usize>>::into(idx)] = self.get(idx);
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
    /// if the index is less than or equal to 7.
    pub fn try_get(&self, idx: u8) -> Option<bool> {
        match idx {
            0..=7 => Some(((self.0 >> idx) & 1) != 0),
            _ => None,
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
    /// if the index is less than or equal to 7.
    pub fn try_set(&mut self, val: bool, idx: u8) -> Option<()> {
        match idx {
            0..=7 => {
                if val {
                    self.0 |= 1 << idx
                } else {
                    self.0 &= !(1 << idx)
                }
                Some(())
            }
            _ => None,
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
    /// if the index is less than or equal to 7.
    pub fn try_toggle(&mut self, idx: u8) -> Option<()> {
        match idx {
            0..=7 => {
                self.0 ^= 1 << idx;
                Some(())
            }
            _ => None,
        }
    }
}

impl BitAnd for PackedBools {
    type Output = PackedBools;

    fn bitand(self, rhs: Self) -> Self::Output {
        PackedBools(self.0 & rhs.0)
    }
}

impl BitAndAssign for PackedBools {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.bitand(rhs)
    }
}

impl BitOr for PackedBools {
    type Output = PackedBools;

    fn bitor(self, rhs: Self) -> Self::Output {
        PackedBools(self.0 | rhs.0)
    }
}

impl BitOrAssign for PackedBools {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.bitor(rhs)
    }
}

impl BitXor for PackedBools {
    type Output = PackedBools;

    fn bitxor(self, rhs: Self) -> Self::Output {
        PackedBools(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for PackedBools {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.bitxor(rhs)
    }
}

impl Not for PackedBools {
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

/// This struct is a smaller range than `ops::Range<u8>` for `IntoIter`,
/// considering the values will only ever go up to 8.
#[repr(transparent)]
struct PackedU8Range(u8);

impl PackedU8Range {
    fn new(start: u8, end: u8) -> Self {
        let mut val = start << 4;
        val |= end;
        PackedU8Range(val)
    }

    fn get_start(&self) -> u8 {
        self.0 >> 4
    }

    fn get_end(&self) -> u8 {
        self.0 & 0b00001111
    }

    fn iter_next(&mut self) -> Option<u8> {
        let start = self.get_start();
        (self.0 < 0b11110000 && start < self.get_end()).then(|| {
            self.0 += 0b0001000; // increment start
            start
        })
    }

    fn iter_next_back(&mut self) -> Option<u8> {
        let end = self.get_end();
        (end > 0 && self.get_start() < end).then(|| {
            self.0 -= 1; // decrement end
            end
        })
    }

    fn len(&self) -> usize {
        (self.get_end() - self.get_start()).into()
    }
}

/// An iterator over the booleans in a `PackedBools`.
#[repr(C)]
pub struct IntoIter {
    bools: PackedBools,
    range: PackedU8Range,
}

impl IntoIter {
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
        let len = self.range.len();
        (len, Some(len))
    }
}

impl DoubleEndedIterator for IntoIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.range.iter_next_back().map(|idx| self.bools.get(idx))
    }
}

impl ExactSizeIterator for IntoIter {
    fn len(&self) -> usize {
        self.range.len()
    }
}

impl FusedIterator for IntoIter {}

#[cfg(test)]
mod tests {
    use super::PackedBools;

    #[test]
    fn assert_size() {
        // This is the core reason to even use this type
        // over multiple bools, so this should be assured in the tests.
        assert_eq!(1, core::mem::size_of::<PackedBools>());
    }

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
}
