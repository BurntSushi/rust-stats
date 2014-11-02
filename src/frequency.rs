use std::collections::hashmap::{HashMap, Occupied, Vacant};
use std::hash::Hash;
use std::default::Default;

use Commute;

/// A commutative data structure for exact frequency counts.
#[deriving(Clone)]
pub struct Frequencies<T> {
    data: HashMap<T, u64>,
}

impl<T: Eq + Hash> Frequencies<T> {
    /// Create a new frequency table with no samples.
    pub fn new() -> Frequencies<T> {
        Default::default()
    }

    /// Add a sample to the frequency table.
    pub fn add(&mut self, v: T) {
        match self.data.entry(v) {
            Vacant(count) => { count.set(1); },
            Occupied(mut count) => { *count.get_mut() += 1; },
        }
    }

    /// Return the number of occurrences of `v` in the data.
    pub fn count(&self, v: &T) -> u64 {
        self.data.find(v).map(|&v| v).unwrap_or(0)
    }

    /// Return the cardinality (number of unique elements) in the data.
    pub fn cardinality(&self) -> u64 {
        self.len() as u64
    }

    /// Returns the mode if one exists.
    pub fn mode(&self) -> Option<&T> {
        let counts = self.most_frequent();
        if counts.is_empty() {
            None
        } else if counts.len() >= 2 && counts[0].val1() == counts[1].val1() {
            None
        } else {
            Some(counts[0].val0())
        }
    }

    /// Return a `Vec` of elements and their corresponding counts in
    /// descending order.
    pub fn most_frequent(&self) -> Vec<(&T, u64)> {
        let mut counts: Vec<_> = self.data.iter()
                                          .map(|(k, &v)| (k, v))
                                          .collect();
        counts.sort_by(|&(_, c1), &(_, c2)| c2.cmp(&c1));
        counts
    }

    /// Return a `Vec` of elements and their corresponding counts in
    /// ascending order.
    pub fn least_frequent(&self) -> Vec<(&T, u64)> {
        let mut counts: Vec<_> = self.data.iter()
                                          .map(|(k, &v)| (k, v))
                                          .collect();
        counts.sort_by(|&(_, c1), &(_, c2)| c1.cmp(&c2));
        counts
    }

    /// Returns the cardinality of the data.
    pub fn len(&self) -> uint {
        self.data.len()
    }
}

impl<T: Eq + Hash> Commute for Frequencies<T> {
    fn merge(&mut self, v: Frequencies<T>) {
        self.data.extend(v.data.into_iter());
    }
}

impl<T: Eq + Hash> Default for Frequencies<T> {
    fn default() -> Frequencies<T> {
        Frequencies { data: HashMap::with_capacity(100000) }
    }
}

impl<T: Eq + Hash> FromIterator<T> for Frequencies<T> {
    fn from_iter<I: Iterator<T>>(it: I) -> Frequencies<T> {
        let mut v = Frequencies::new();
        v.extend(it);
        v
    }
}

impl<T: Eq + Hash> Extendable<T> for Frequencies<T> {
    fn extend<I: Iterator<T>>(&mut self, mut it: I) {
        for sample in it {
            self.add(sample);
        }
    }
}

#[cfg(test)]
mod test {
    use super::Frequencies;

    #[test]
    fn ranked() {
        let mut counts = Frequencies::new();
        counts.extend(vec![1u, 1, 2, 2, 2, 2, 2, 3, 4, 4, 4].into_iter());
        assert_eq!(counts.most_frequent()[0], (&2, 5));
        assert_eq!(counts.least_frequent()[0], (&3, 1));
    }
}
