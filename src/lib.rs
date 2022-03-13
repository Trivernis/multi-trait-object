#![doc=include_str!("../README.md")]
#[cfg(test)]
mod tests;

pub(crate) mod macros;

use std::any::{Any, TypeId};
use std::collections::HashMap;

#[doc(hidden)]
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct FatPointer {
    pub data: *const (),
    pub vptr: *const (),
}

/// A container to store data with the associated type and trait objects
/// allowing for casting down to traits or the concrete type
/// ```rust
/// use multi_trait_object::prelude::*;
/// use std::fmt::{Debug, Display};
///
/// let mut mto = MultitraitObject::new(String::new());
/// register_traits!(mto, String, dyn Debug, dyn Display);
///
/// let debug = mto.downcast_trait::<dyn Debug>().unwrap();
/// println!("{:?}", debug);
/// let display = mto.downcast_trait::<dyn Display>().unwrap();
/// println!("{}", display);
/// let string = mto.downcast::<String>().unwrap();
/// println!("{}", string);
/// ```
#[derive(Debug)]
pub struct MultitraitObject {
    pub data: *mut (),
    pub original_typeid: TypeId,
    pub any_vtable: *const (),
    pub traits: HashMap<TypeId, *const ()>,
}

impl MultitraitObject {
    pub fn new<T: 'static + Any>(value: T) -> Self {
        let any_vtable = __fat_pointer!(T as dyn Any).vptr;
        let data = Box::into_raw(Box::new(value)) as *mut ();

        let mut this = Self {
            data,
            any_vtable,
            original_typeid: TypeId::of::<T>(),
            traits: Default::default(),
        };
        this._register::<dyn Any>(any_vtable);

        this
    }

    /// Downcasts the object into a reference of the given trait
    pub fn downcast_trait<T1: 'static + ?Sized>(&self) -> Option<&T1> {
        if std::mem::size_of::<&T1>() != std::mem::size_of::<FatPointer>() {
            None
        } else {
            unsafe {
                self._downcast_trait::<T1>()
            }
        }
    }

    /// Downcasts the object into a mutable reference of the given trait
    pub fn downcast_trait_mut<T1: 'static + ?Sized>(&mut self) -> Option<&mut T1> {
        if std::mem::size_of::<&T1>() != std::mem::size_of::<FatPointer>() {
            None
        } else {
            unsafe {
                self._downcast_trait_mut::<T1>()
            }
        }
    }

    /// Downcasts the object to a boxed representation of the given trait
    pub fn downcast_trait_boxed<T1: 'static + ?Sized>(mut self) -> Option<Box<T1>> {
        if std::mem::size_of::<&T1>() != std::mem::size_of::<FatPointer>() {
            None
        } else {
            unsafe {
                self._downcast_boxed_trait::<T1>()
            }
        }
    }

    /// Downcasts the object into a reference of the given type
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        let any = self.downcast_trait::<dyn Any>().unwrap();
        any.downcast_ref::<T>()
    }

    /// Downcasts the object into a mutable reference of the given type
    pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        let any = self.downcast_trait_mut::<dyn Any>().unwrap();
        any.downcast_mut::<T>()
    }

    /// Downcasts the object into the given concrete type
    pub fn downcast<T: Any>(self) -> Option<T> {
        if TypeId::of::<T>() == self.original_typeid {
            unsafe {
                let typed_ptr = std::mem::transmute::<_, *mut T>(self.data);
                let boxed = Box::from_raw(typed_ptr);

                Some(*boxed)
            }
        } else {
            None
        }
    }

    #[doc(hidden)]
    pub fn _register<T2: 'static + ?Sized>(&mut self, vtable_ptr: *const ()) {
        self.traits.insert(TypeId::of::<T2>(), vtable_ptr);
    }

    #[doc(hidden)]
    unsafe fn _downcast_trait<T1: 'static + ?Sized>(&self) -> Option<&T1> {
        let vptr = *self.traits.get(&TypeId::of::<T1>())?;
        let fat_pointer = FatPointer { data: self.data, vptr };
        let value = std::mem::transmute::<_, &&T1>(&fat_pointer);

        Some(*value)
    }

    #[doc(hidden)]
    unsafe fn _downcast_trait_mut<T1: 'static + ?Sized>(&mut self) -> Option<&mut T1> {
        let vptr = *self.traits.get(&TypeId::of::<T1>())?;
        let mut fat_pointer = FatPointer { data: self.data, vptr };
        let value = std::mem::transmute::<_, &mut &mut T1>(&mut fat_pointer);

        Some(*value)
    }

    #[doc(hidden)]
    unsafe fn _downcast_boxed_trait<T1: 'static + ?Sized>(&mut self) -> Option<Box<T1>> {
        let vptr = *self.traits.get(&TypeId::of::<T1>())?;
        let fat_pointer = FatPointer { data: self.data, vptr };
        let value = std::mem::transmute::<_, *const *mut T1>(&fat_pointer);
        let value = Box::from_raw(*value);

        Some(value)
    }
}

impl Drop for MultitraitObject {
    fn drop(&mut self) {
        unsafe {
            let raw = Box::from_raw(self.data);
            std::mem::drop(raw);
        }
    }
}

pub trait IntoMultitrait {
    fn into_multitrait(self) -> MultitraitObject;
}

pub mod prelude {
    pub use crate::*;
}