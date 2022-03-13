use std::fmt::{Debug, Formatter};
use crate::MultitraitObject;

impl Debug for MultitraitObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(debug) = self.downcast_trait::<dyn Debug>() {
            debug.fmt(f)
        } else {
            write!(f, "<unavailable>")
        }
    }
}