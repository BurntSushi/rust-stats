use std::default::Default;

pub trait Commute : Default {
    fn merge(&mut self, other: Self);
}

pub fn merge_all<T: Commute, I: Iterator<T>>(mut it: I) -> T {
    let init: T = Default::default();
    it.fold(init, |mut v1, v2| { v1.merge(v2); v1 })
}

pub mod frequency;
pub mod online;
pub mod sorted;
