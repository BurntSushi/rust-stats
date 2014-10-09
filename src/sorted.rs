use std::collections::PriorityQueue;
use std::default::Default;

use Commute;

/// Compute the exact median on a stream of data.
///
/// (This has time complexity `O(nlogn)` and space complexity `O(n)`.)
pub fn median<T: Ord + ToPrimitive + Clone, I: Iterator<T>>(mut it: I) -> f64 {
    it.collect::<Sorted<T>>().median()
}

/// Compute the exact mode on a stream of data.
///
/// (This has time complexity `O(nlogn)` and space complexity `O(n)`.)
///
/// If the data does not have a mode, then `None` is returned.
pub fn mode<T: Ord + Clone, I: Iterator<T>>(mut it: I) -> Option<T> {
    it.collect::<Sorted<T>>().mode()
}

/// A commutative data structure for sorted sequences of data.
#[deriving(Clone)]
pub struct Sorted<T> {
    data: PriorityQueue<T>,
}

impl<T: Ord> Sorted<T> {
    /// Create initial empty state.
    pub fn new() -> Sorted<T> {
        Default::default()
    }

    /// Add a new element to the set.
    pub fn add(&mut self, v: T) {
        self.data.push(v)
    }
}

impl<T: Ord + Clone> Sorted<T> {
    /// Returns the mode of the data.
    pub fn mode(&self) -> Option<T> {
        // This approach to computing the mode works very nicely when the
        // number of samples is large and is close to its cardinality.
        // In other cases, a hashmap would be much better.
        // But really, how can we know this when given an arbitrary stream?
        // Might just switch to a hashmap to track frequencies. That would also
        // be generally useful for discovering the cardinality of a sample.
        if self.len() == 0 {
            return None;
        }
        let (mut mode, mut next) = (None, None);
        let (mut mode_count, mut next_count) = (0u, 0u);
        for x in self.data.clone().into_sorted_vec().into_iter() {
            if mode.as_ref().map(|y| y == &x).unwrap_or(false) {
                mode_count += 1;
            } else if next.as_ref().map(|y| y == &x).unwrap_or(false) {
                next_count += 1;
            } else {
                next = Some(x);
                next_count = 0;
            }

            if next_count > mode_count {
                mode = next;
                mode_count = next_count;
                next = None;
                next_count = 0;
            } else if next_count == mode_count {
                mode = None;
                mode_count = 0u;
            }
        }
        mode
    }
}

impl<T: Ord + ToPrimitive + Clone> Sorted<T> {
    /// Returns the median of the data.
    pub fn median(&self) -> f64 {
        // Grr. The only way to avoid the alloc here is to take `self` by
        // value. Could return `(f64, Sorted<T>)`, but that seems a bit weird.
        //
        // NOTE: Can `std::mem::swap` help us here?
        let data = self.data.clone().into_sorted_vec();
        if data.len() % 2 == 0 {
            let v1 = data[(data.len() / 2) - 1].to_f64().unwrap();
            let v2 = data[data.len() / 2].to_f64().unwrap();
            (v1 + v2) / 2.0
        } else {
            data[data.len() / 2].to_f64().unwrap()
        }
    }
}

impl<T: Ord> Commute for Sorted<T> {
    fn merge(&mut self, v: Sorted<T>) {
        // should this be `into_sorted_vec`?
        self.extend(v.data.into_vec().into_iter());
    }
}

impl<T: Ord> Default for Sorted<T> {
    fn default() -> Sorted<T> { Sorted { data: PriorityQueue::new() } }
}

impl<T: Ord> Collection for Sorted<T> {
    fn len(&self) -> uint { self.data.len() }
}

impl<T: Ord> Mutable for Sorted<T> {
    fn clear(&mut self) { self.data.clear(); }
}

impl<T: Ord> FromIterator<T> for Sorted<T> {
    fn from_iter<I: Iterator<T>>(it: I) -> Sorted<T> {
        let mut v = Sorted::new();
        v.extend(it);
        v
    }
}

impl<T: Ord> Extendable<T> for Sorted<T> {
    fn extend<I: Iterator<T>>(&mut self, it: I) {
        self.data.extend(it)
    }
}

#[cfg(test)]
mod test {
    use super::{median, mode};

    #[test]
    fn median_stream() {
        assert_eq!(median(vec![3u, 5, 7, 9].into_iter()), 6.0);
        assert_eq!(median(vec![3u, 5, 7].into_iter()), 5.0);
    }

    #[test]
    fn mode_stream() {
        assert_eq!(mode(vec![3u, 5, 7, 9].into_iter()), None);
        assert_eq!(mode(vec![3u, 3, 3, 3].into_iter()), Some(3));
        assert_eq!(mode(vec![3u, 3, 3, 4].into_iter()), Some(3));
        assert_eq!(mode(vec![4u, 3, 3, 3].into_iter()), Some(3));
        assert_eq!(mode(vec![1u, 1, 2, 3, 3].into_iter()), None);
    }
}
