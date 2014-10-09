use std::default::Default;

pub use frequency::Frequencies;
pub use online::{OnlineStats, stddev, variance, mean};
pub use sorted::{Sorted, median, mode};

/// Defines an interface for types that have an identity and can be commuted.
pub trait Commute : Default {
    /// Merges the value `other` into `self`.
    fn merge(&mut self, other: Self);
}

/// Merges all items in the stream.
pub fn merge_all<T: Commute, I: Iterator<T>>(mut it: I) -> T {
    let init: T = Default::default();
    it.fold(init, |mut v1, v2| { v1.merge(v2); v1 })
}

mod frequency;
mod online;
mod sorted;
