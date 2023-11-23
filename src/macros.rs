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
