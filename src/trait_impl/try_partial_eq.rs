use std::any::{Any};
use crate::MultitraitObject;

/// Compares the given value with an Any trait object.
/// Register this trait if you want to compare two Multitrait objects
/// with the [TryPartialEq] trait.
pub trait PartialEqAny {
    fn partial_equal_any(&self, other: &dyn Any) -> bool;
}

impl<T: PartialEq + Any> PartialEqAny for T {
    fn partial_equal_any(&self, other: &dyn Any) -> bool {
        if let Some(other) = other.downcast_ref::<T>() {
            self.eq(other)
        } else {
            false
        }
    }
}

/// Tries to compare the MultitraitObject with another object
/// and returns Some(bool) when the underlying type implements PartialEq and
/// has the [PartialEqAny] trait registered on the object.
pub trait TryPartialEq {
    fn try_eq(&self, other: &Self) -> Option<bool>;
}

impl TryPartialEq for MultitraitObject {
    fn try_eq(&self, other: &Self) -> Option<bool> {
        if let Some(eq_any) = self.downcast_trait::<dyn PartialEqAny>() {
            let any = other.downcast_trait::<dyn Any>().unwrap();
            Some(eq_any.partial_equal_any(any))
        } else {
            None
        }
    }
}