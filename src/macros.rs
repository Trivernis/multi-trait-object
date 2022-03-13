/// Implements the `IntoMultitrait` trait on the defined type.
/// ```rust
/// use multi_trait_object::prelude::*;
///
/// struct MyStruct {
///     a: u64,
/// }
///
/// trait MyTrait {}
/// trait MyOtherTrait {}
///
/// impl MyTrait for MyStruct{}
/// impl MyOtherTrait for MyStruct {}
///
/// impl_trait_object!(MyStruct, dyn MyTrait, dyn MyOtherTrait);
/// ```
#[macro_export]
macro_rules! impl_trait_object {
    ($obj:ty, $($trt:ty),*) => {
        impl IntoMultitrait for $obj {
            fn into_multitrait(self) -> MultitraitObject {
                let mut mto = MultitraitObject::new(self);
                $(
                    register_traits!(mto, $obj, $trt);
                )*

                mto
            }
        }
    }
}

/// Registers multiple trait_impl on a multitrait object
/// ```rust
/// use multi_trait_object::prelude::*;
/// use std::fmt::{Debug, Display};
///
/// let value = String::new();
/// let mut mto = MultitraitObject::new(value);
/// register_traits!(mto, String, dyn Debug, dyn Display);
/// ```
#[macro_export]
macro_rules! register_traits {
    ($r:expr, $v:ty, $($t:ty), +) => {
        $(
            $r._register::<$t>(__fat_pointer!($v as $t).vptr);
        )+
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __fat_pointer {
    ($v:ty as $t:ty) => {{
        let x = ::std::ptr::null::<$v>() as *const $v as *const $t;
        #[allow(unused_unsafe)]
        unsafe {
            std::mem::transmute::<_, FatPointer>(x)
        }
    }}
}
