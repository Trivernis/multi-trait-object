use crate::{MultitraitObject};

pub trait TryClone: Sized {
    fn try_clone(&self) -> Option<Self>;
}

/// Returns a raw pointer to the cloned data.
/// This will leak the memory of the underlying pointer.
/// This trait is implemented for all types that implement clone
/// so you can pass this trait to the object constructor. This
/// way the given object can be cloned with [TryClone].
pub unsafe trait RawClone {
    #[doc(hidden)]
    #[must_use]
    unsafe fn raw_clone(&self) -> *mut ();
}

unsafe impl<T: Clone> RawClone for T {
    unsafe fn raw_clone(&self) -> *mut () {
        Box::into_raw(Box::new(self.clone())) as *mut ()
    }
}

/// A trait that tries cloning an object and returns an option
/// with the variant depending on the result.
impl TryClone for MultitraitObject {
    fn try_clone(&self) -> Option<Self> {
        let clone = self.downcast_trait::<dyn RawClone>()?;
        let data_ptr = unsafe {
            clone.raw_clone()
        };
        Some(MultitraitObject {
            data: data_ptr,
            original_typeid: self.original_typeid.clone(),
            traits: self.traits.clone(),
        })
    }
}