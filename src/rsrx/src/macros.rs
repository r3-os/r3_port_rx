macro_rules! zero_sized_ref {
    (
        $( #[$meta:meta] )*
        pub struct $Name:ident: &$ty:ty = $address:expr
    ) => {
        #[doc = concat!("[`", stringify!($ty), "`][] at `", stringify!($address) ,"`")]
        $( #[$meta] )*
        pub struct $Name {
            _marker: ::core::marker::PhantomData<*const ()>,
        }

        unsafe impl Send for $Name {}

        impl $Name {
            pub(crate) const INSTANCE: Self = Self {
                _marker: ::core::marker::PhantomData,
            };

            /// Returns a pointer to the register block
            #[inline(always)]
            pub const fn ptr() -> *const $ty {
                $address as *const _
            }

            /// Returns a pointer to the register block as `usize`
            #[inline(always)]
            pub const fn ptr_usize() -> usize {
                $address
            }
        }

        impl core::ops::Deref for $Name {
            type Target = $ty;

            #[inline(always)]
            fn deref(&self) -> &$ty {
                unsafe { &*Self::ptr() }
            }
        }
    };
}

macro_rules! peripheral_set {
    (
        $( #[$meta:meta] )*
        pub struct Peripherals {
            $( pub $field:ident: $Zsr:ty, )*
        }
    ) => {
        $( #[$meta] )*
        #[allow(non_snake_case)]
        pub struct Peripherals {
            $( pub $field: $Zsr, )*
        }

        impl Peripherals {
            #[inline(always)]
            pub const unsafe fn steal() -> Self {
                Self {
                    $( $field: <$Zsr>::INSTANCE, )*
                }
            }
        }
    };
}
