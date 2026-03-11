use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Bound;

use skiplist::ordered_skip_list::OrderedSkipList;
use ordered_float::NotNan;

use crate::ByteString;

#[derive(Clone, Default)]
pub struct SortedSet {
    map: HashMap<ByteString, NotNan<f64>>,
    smap: OrderedSkipList<(NotNan<f64>, ByteString)>,
}

impl SortedSet {
    pub fn new() -> Self {
        Self { map: HashMap::new(), smap: OrderedSkipList::new() }
    }
}

impl SortedSet {
    pub fn insert(&mut self, s: NotNan<f64>, t: ByteString) -> Option<NotNan<f64>> {
        let old_s = self.map.insert(t.clone(), s);
        if let Some(old_s) = &old_s {
            self.smap.remove_by_value(&(*old_s, t.clone()));
        }
        self.smap.insert((s, t));
        old_s
    }

    pub fn remove(&mut self, t: ByteString) -> Option<NotNan<f64>> {
        let s = self.map.remove(&t);
        if let Some(s) = &s {
            self.smap.remove_by_value(&(s.clone(), t));
        }
        s
    }

    pub fn len(&self) -> usize { self.map.len() }

    pub fn get_score(&self, t: &[u8]) -> Option<f64> {
        self.map.get(t).map(|&s| *s)
    }

    pub fn iter(&self) -> impl Iterator<Item=(f64, &[u8])> {
        self.smap.iter().map(|(s, t)| (**s, t.as_slice()))
    }

    pub fn riter(&self) -> impl Iterator<Item=(f64, &[u8])> {
        self.smap.iter().rev().map(|(s, t)| (**s, t.as_slice()))
    }

    pub fn range(&self, min: NotNan<f64>, max: NotNan<f64>) -> impl Iterator<Item=(f64, &[u8])> {
        let min = Bound::Included((min, Vec::new()));
        let max = Bound::Excluded((NotNan::new(max.next_up()).unwrap(), Vec::new()));
        self.smap.range((min, max)).map(|(s, t)| (**s, t.as_slice()))
    }

    pub fn rank(&self, t: ByteString) -> Option<(f64, usize)> {
        let s = self.map.get(&t)?.clone();
        let r = self.smap.rank(&(s.clone(), t))?;
        Some((*s, r))
    }
}

impl Debug for SortedSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SortedSet").field("map", &self.smap).finish()
    }
}

impl PartialEq for SortedSet {
    fn eq(&self, other: &Self) -> bool { self.map == other.map }
}

impl Eq for SortedSet {}
