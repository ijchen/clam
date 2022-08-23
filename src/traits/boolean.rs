//! A `Boolean` is a general boolean type.
//!
//! TODO Further description... I do not see how this is at all meaningful when
//! all the number stuff is stripped away, but maybe having this as a trait is
//! helpful in some way I don't understand?

use std::fmt::Debug;
use std::fmt::Display;

use ndarray_npy::ReadableElement;
use ndarray_npy::WritableElement;

/// TODO Doc
pub trait Boolean: Copy + Clone + Send + Sync + Debug + Display + ReadableElement + WritableElement {
    /// Converts the Boolean to a bool
    fn as_bool(&self) -> bool;
}

impl Boolean for bool {
    fn as_bool(&self) -> bool {
        *self
    }
}
