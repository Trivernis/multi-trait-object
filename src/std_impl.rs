use std::fmt::{Debug, Display};
use std::fmt::Write as FmtWrite;

use crate::*;

impl_trait_object!(String, dyn Debug, dyn Display, dyn RawClone, dyn FmtWrite);
