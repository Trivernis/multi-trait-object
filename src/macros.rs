/// Implements the `IntoMultitrait` trait on the defined type.
/// ```rust
/// use multi_trait_object::*;
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
        impl $crate::IntoMultitrait for $obj {
            fn into_multitrait(self) -> $crate::MultitraitObject {
                let mut mto = $crate::MultitraitObject::new(self);
                $(
                    unsafe {
                        // SAFETY: We're only passing v-tables associated with the given type
                        mto._register::<$trt>($crate::__fat_pointer!($obj as $trt).vptr);
                    }
                )*

                mto
            }
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __fat_pointer {
    ($v:ty as $t:ty) => {{
        let x = ::std::ptr::null::<$v>() as *const $v as *const $t;
        #[allow(unused_unsafe)]
        unsafe {
            std::mem::transmute::<_, $crate::FatPointer>(x)
        }
    }}
}

/// Registers multiple trait_impl on a multitrait object
/// ```rust
/// use multi_trait_object::*;
/// use std::fmt::{Debug, Display};
///
/// let mto = create_object!(String::new(), dyn Debug, dyn Display);
/// ```
#[macro_export]
macro_rules! create_object {
    ($v:expr, $($t:ty), +) => {
        {
            let null_ptr = unsafe {
                // SAFETY: We're never accessing the null value
                $crate::null_ptr(&$v)
            };
            let mut mto = $crate::MultitraitObject::new($v);
            $(
                unsafe {
                    // SAFETY: We're never accessing the null value
                    let ptr = null_ptr as *const $t;
                    let vptr = std::mem::transmute::<_, $crate::FatPointer>(ptr).vptr;
                    // SAFETY: We're only passing v-tables associated with the given type
                    mto._register::<$t>(vptr);
                }
            )+
            mto
        }
    }
}