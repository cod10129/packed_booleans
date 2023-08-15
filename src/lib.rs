//! A crate for packing booleans together.

#![no_std]

use core::ops::{BitAnd, BitOr, BitXor, Not, BitAndAssign, BitOrAssign, BitXorAssign};

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
        self.try_get(idx).expect("The index cannot be greater than 7.")
    }

    /// Gets the boolean at the given index, 
    /// if the index is less than or equal to 7.
    pub fn try_get(&self, idx: u8) -> Option<bool> {
        match idx {
            0..=7 => Some(((self.0 >> idx) & 1) != 0),
            _ => None
        }
    }

    /// Sets the boolean at the given index to val.
    /// 
    /// # Panics
    /// 
    /// Panics if the given index is greater than 7.
    pub fn set(&mut self, val: bool, idx: u8) {
        self.try_set(val, idx).expect("The index cannot be greater than 7.")
    }

    /// Sets the boolean at the given index to val,
    /// if the index is less than or equal to 7.
    pub fn try_set(&mut self, val: bool, idx: u8) -> Option<()> {
        match idx {
            0..=7 => {
                if val { self.0 |= 1 << idx }
                else { self.0 &= !(1 << idx) }
                Some(())
            },
            _ => None
        }
    }

    /// Toggles the boolean at the given index.
    /// 
    /// # Panics
    /// 
    /// Panics if the given index is greater than 7.
    pub fn toggle(&mut self, idx: u8) {
        self.try_toggle(idx).expect("The index cannot be greater than 7.")
    }

    /// Toggles the boolean at the given index,
    /// if the index is less than or equal to 7.
    pub fn try_toggle(&mut self, idx: u8) -> Option<()> {
        match idx {
            0..=7 => {
                self.0 ^= 1 << idx;
                Some(())
            },
            _ => None
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
        IntoIter { bools: self, idx: 0 }
    }
}

pub struct IntoIter {
    bools: PackedBools,
    idx: u8
}

impl Iterator for IntoIter {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx > 7 { return None }
        let val = self.bools.try_get(self.idx);
        self.idx += 1;
        val
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = 7u8.saturating_sub(self.idx).into();
        (len, Some(len))
    }

    // This function is defined here for optimization
    fn last(self) -> Option<Self::Item>
        where
            Self: Sized, {
        if self.idx > 7 { None }
        else { Some(self.bools.get(7)) }
    }
}

#[cfg(test)]
mod tests {
    use super::PackedBools;

    #[test]
    fn assert_size() {
        // This is the core reason to even use this type
        // over multiple bools, so this should be assured in the tests.
        assert_eq!(1, core::mem::size_of::<PackedBools>())
    }

    #[test]
    fn set_get() {
        let mut pkd = PackedBools::new();

        pkd.set_all([
            false, true, false, true, true, false, false, true
        ]);
        pkd.set(false, 3);
        pkd.set(true, 4);

        assert_eq!(
            pkd.get_all(),
            [false, true, false, false, true, false, false, true]
        );
    }
}
