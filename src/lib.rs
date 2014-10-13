#![experimental]
#![feature(tuple_indexing)]

pub use frequency::Frequencies;
pub use minmax::MinMax;
pub use online::{OnlineStats, stddev, variance, mean};
pub use sorted::{Sorted, median, mode};

/// Defines an interface for types that have an identity and can be commuted.
///
/// The value returned by `Default::default` must be its identity with respect
/// to the `merge` operation.
pub trait Commute {
    /// Merges the value `other` into `self`.
    fn merge(&mut self, other: Self);

    /// Merges the values in the iterator into `self`.
    fn consume<I: Iterator<Self>>(&mut self, mut other: I) {
        for v in other {
            self.merge(v);
        }
    }
}

/// Merges all items in the stream.
///
/// If the stream is empty, `None` is returned.
pub fn merge_all<T: Commute, I: Iterator<T>>(mut it: I) -> Option<T> {
    match it.next() {
        None => None,
        Some(mut init) => { init.consume(it); Some(init) }
    }
}

impl<T: Commute> Commute for Option<T> {
    fn merge(&mut self, other: Option<T>) {
        match self {
            &None => { *self = other; }
            &Some(ref mut v1) => { other.map(|v2| v1.merge(v2)); }
        }
    }
}

impl<T: Commute, E> Commute for Result<T, E> {
    fn merge(&mut self, other: Result<T, E>) {
        // Can't figure out how to work around the borrow checker to make
        // this code less awkward.
        if !self.is_err() && other.is_err() {
            *self = other;
            return;
        }
        match self {
            &Err(_) => {},
            &Ok(ref mut v1) => {
                match other {
                    Ok(v2) => { v1.merge(v2); }
                    // This is the awkward part. We can't assign to `*self`
                    // because of the `ref mut v1` borrow. So we catch this
                    // case above and declare that this cannot be reached.
                    Err(_) => { unreachable!(); }
                }
            }
        }
    }
}

impl<T: Commute> Commute for Vec<T> {
    fn merge(&mut self, other: Vec<T>) {
        assert_eq!(self.len(), other.len());
        for (v1, v2) in self.iter_mut().zip(other.into_iter()) {
            v1.merge(v2);
        }
    }
}

mod frequency;
mod minmax;
mod online;
mod sorted;

#[cfg(test)]
mod test {
    use Commute;
    use sorted::Sorted;

    #[test]
    fn options() {
        let v1: Sorted<uint> = vec![2, 1, 3, 2].into_iter().collect();
        let v2: Sorted<uint> = vec![5, 6, 5, 5].into_iter().collect();
        let mut merged = Some(v1);
        merged.merge(Some(v2));
        assert_eq!(merged.unwrap().mode(), Some(5));
    }
}
