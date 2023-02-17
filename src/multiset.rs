use std::borrow::Borrow;
use std::collections::btree_map;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct MultiSet<T> {
    freq: BTreeMap<T, usize>,
    len: usize,
}

pub struct Iter<'a, T> {
    iter: btree_map::Iter<'a, T, usize>,
    front: Option<&'a T>,
    front_to_dispatch: usize,
    back: Option<&'a T>,
    back_to_dispatch: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front_to_dispatch == 0 {
            if let Some((k, &v)) = self.iter.next() {
                self.front = Some(k);
                self.front_to_dispatch = v;
            } else if self.back_to_dispatch > 0 {
                self.back_to_dispatch -= 1;
                return self.back;
            }
        }
        if self.front_to_dispatch > 0 {
            self.front_to_dispatch -= 1;
            return self.front;
        }
        None
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.back_to_dispatch == 0 {
            if let Some((k, &v)) = self.iter.next_back() {
                self.back = Some(k);
                self.back_to_dispatch = v;
            } else if self.front_to_dispatch > 0 {
                self.front_to_dispatch -= 1;
                return self.front;
            }
        }
        if self.back_to_dispatch > 0 {
            self.back_to_dispatch -= 1;
            return self.back;
        }
        None
    }
}

impl<T: Ord> Default for MultiSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> MultiSet<T> {
    pub fn new() -> Self
    where
        T: Ord,
    {
        Self {
            freq: BTreeMap::new(),
            len: 0,
        }
    }

    pub fn insert(&mut self, val: T)
    where
        T: Ord,
    {
        *self.freq.entry(val).or_insert(0) += 1;
        self.len += 1;
    }

    pub fn contains<Q: ?Sized>(&self, val: &Q) -> bool
    where
        T: Borrow<Q> + Ord,
        Q: Ord,
    {
        self.freq.contains_key(val)
    }

    /// Removes one occurance of value from multiset.
    /// Returns true if value was present in set and
    /// deleted, and false otherwise.
    pub fn remove<Q: ?Sized>(&mut self, val: &Q) -> bool
    where
        T: Borrow<Q> + Ord,
        Q: Ord,
    {
        if self.contains(val) {
            *self.freq.get_mut(val).unwrap() -= 1;
            if self.freq[val] == 0 {
                self.freq.remove(val);
            }
            self.len -= 1;
            return true;
        }
        false
    }

    pub fn iter(&self) -> Iter<T>
    where
        T: Ord,
    {
        Iter {
            iter: self.freq.iter(),
            front: None,
            front_to_dispatch: 0,
            back: None,
            back_to_dispatch: 0,
        }
    }
}
