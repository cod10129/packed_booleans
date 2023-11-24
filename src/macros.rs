macro_rules! impl_binop {
    (impl $op:tt for $type:ty: $tr:ident $method:ident $assign_tr:ident $assign_method:ident) => {
        // base impl
        impl $tr for $type {
            type Output = $type;

            fn $method(self, rhs: Self) -> $type {
                <$type>::from_bits(self.0 $op rhs.0)
            }
        }

        // ref impls
        impl $tr<$type> for &$type {
            type Output = $type;

            fn $method(self, rhs: $type) -> $type {
                $tr::$method(*self, rhs)
            }
        }

        impl $tr<&$type> for $type {
            type Output = $type;

            fn $method(self, rhs: &$type) -> $type {
                $tr::$method(self, *rhs)
            }
        }

        impl $tr<&$type> for &$type {
            type Output = $type;

            fn $method(self, rhs: &$type) -> $type {
                $tr::$method(*self, *rhs)
            }
        }

        // op= impls
        impl $assign_tr<$type> for $type {
            fn $assign_method(&mut self, rhs: Self) {
                *self = self.$method(rhs)
            }
        }

        impl $assign_tr<&$type> for $type {
            fn $assign_method(&mut self, rhs: &$type) {
                *self = self.$method(*rhs)
            }
        }
    }
}

pub(crate) use impl_binop;

macro_rules! impl_binops {
    (impl & | ^ for $type: ty) => {
        use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};
        crate::macros::impl_binop!{ impl & for $type: BitAnd bitand BitAndAssign bitand_assign }
        crate::macros::impl_binop!{ impl | for $type: BitOr bitor BitOrAssign bitor_assign }
        crate::macros::impl_binop!{ impl ^ for $type: BitXor bitxor BitXorAssign bitxor_assign }
    }
}

pub(crate) use impl_binops;

macro_rules! packed_bools_type {
    (
        NAME = $pkd:ident,
        REPR = $repr:ident,
        BOOL_COUNT = $bcount:literal,
        BCOUNT_MINUS1 = $bcountdec:literal,
        BYTE_DESCRIPTION = $bdesc:literal,
        PRETTY_DEBUG = $pdebug:literal,
        DEBUG = $debug:literal,
        BINARY = $binary:literal,
        LOW_HEX = $lohex:literal,
        UPPER_HEX = $uphex:literal
    ) => {
        #[doc = concat!("A type containing ", $bcount, " `bool` values,")]
        #[doc = concat!("while only being ", $bdesc, ".")]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[derive(Clone, Copy, PartialEq, Eq, Default, Hash)]
        #[repr(transparent)]
        pub struct $pkd($repr);

        impl $pkd {
            #[doc = concat!("Creates a new `", stringify!($pkd), "` with all false values.")]
            pub const fn new() -> Self { Self(0) }

            #[doc = concat!("Creates a new `", stringify!($pkd), "` from the given bits.")]
            pub fn from_bits(bits: $repr) -> Self { Self(bits) }

            /// Counts how many true values there are.
            pub fn count_true(&self) -> u8 {
                self.0.count_ones() as u8
            }

            /// Counts how many false values there are.
            pub fn count_false(&self) -> u8 {
                self.0.count_zeros() as u8
            }

            #[doc = concat!("Creates a new `", stringify!($pkd), "` from the given values.")]
            pub fn new_vals(vals: [bool; $bcount]) -> Self {
                let out: $repr = vals.into_iter()
                    .map($repr::from)
                    .zip(0..$bcount)
                    .fold(0, |acc, (b, idx)| acc.bitor(b << idx));
                Self(out)
            }

            /// Sets all the booleans to the ones given.
            pub fn set_all(&mut self, vals: [bool; $bcount]) {
                *self = Self::new_vals(vals);
            }

            /// Gets all the booleans.
            pub fn get_all(&self) -> [bool; $bcount] {
                let mut arr = [false; $bcount];
                for (b, idx) in arr.iter_mut().zip(0..$bcount) {
                    *b = ((self.0 >> idx) & 1) != 0;
                }
                arr
            }
            
            /// Gets the boolean at the given index.
            ///
            /// # Panics
            ///
            #[doc = concat!("Panics if the given index is greater than ", $bcountdec, ".")]
            pub fn get(&self, idx: u8) -> bool {
                self.try_get(idx)
                    .expect(concat!("The index cannot be greater than ", $bcountdec))
            }
            
            /// Gets the boolean at the given index,
            #[doc = concat!("if the index is less than ", $bcount, ".")]
            pub fn try_get(&self, idx: u8) -> Option<bool> {
                if idx < $bcount {
                    Some(((self.0 >> idx) & 1) != 0)
                } else {
                    None
                }
            }

            /// Sets the boolean at the given index to val.
            ///
            /// # Panics
            ///
            #[doc = concat!("Panics if the given index is greater than ", $bcountdec, ".")]
            pub fn set(&mut self, val: bool, idx: u8) {
                self.try_set(val, idx)
                    .expect(concat!("The index cannot be greater than ", $bcountdec))
            }

            /// Sets the boolean at the given index to val,
            #[doc = concat!("if the index is less than ", $bcount, ".")]
            pub fn try_set(&mut self, val: bool, idx: u8) -> Option<()> {
                if idx < $bcount {
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
            #[doc = concat!("Panics if the given index is greater than ", $bcountdec, ".")]
            pub fn toggle(&mut self, idx: u8) {
                self.try_toggle(idx)
                    .expect(concat!("The index cannot be greater than ", $bcountdec))
            }

            /// Toggles the boolean at the given index,
            #[doc = concat!("if the index is less than ", $bcount, ".")]
            pub fn try_toggle(&mut self, idx: u8) -> Option<()> {
                if idx < $bcount {
                    self.0 ^= 1 << idx;
                    Some(())
                } else {
                    None
                }
            }
        }

        impl From<[bool; $bcount]> for $pkd {
            fn from(bools: [bool; $bcount]) -> Self { Self::new_vals(bools) }
        }

        crate::macros::impl_binops!{ impl & | ^ for $pkd }

        impl core::ops::Not for $pkd {
            type Output = $pkd;
            fn not(self) -> Self { Self(!self.0) }
        }

        impl core::ops::Not for &$pkd {
            type Output = $pkd;
            fn not(self) -> $pkd { $pkd(!self.0) }
        }

        impl core::fmt::Debug for $pkd {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                if f.alternate() {
                    // manual pretty-print
                    write!(f, $pdebug, self.0)
                } else {
                    write!(f, $debug, self.0)
                }
            }
        }

        #[doc = concat!("Displays the ", stringify!($pkd), " in binary.")]
        /// Note that the order may not be what you expect.
        /// The "first" bool will actually be last in the formatting.
        /// Note that this impl is not stable.
        impl core::fmt::Binary for $pkd {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                if f.alternate() {
                    f.write_str("0b")?;
                }
                write!(f, $binary, self.0)
            }
        }

        #[doc = concat!("Displays the ", stringify!($pkd), " in lowercase hexadecimal.")]
        /// See the notes on fmt::Binary impl for ordering and stability.
        impl core::fmt::LowerHex for $pkd {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                if f.alternate() {
                    f.write_str("0x")?;
                }
                write!(f, $lohex, self.0)
            }
        }

        #[doc = concat!("Displays the ", stringify!($pkd), " in uppercase hexadecimal.")]
        /// See the notes on fmt::Binary impl for ordering and stability.
        impl core::fmt::UpperHex for $pkd {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                if f.alternate() {
                    f.write_str("0x")?;
                }
                write!(f, $uphex, self.0)
            }
        }
    }
}

pub(crate) use packed_bools_type;
