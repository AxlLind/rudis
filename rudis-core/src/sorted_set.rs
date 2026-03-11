use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Bound;

use skiplist::ordered_skip_list::OrderedSkipList;

pub struct SortedSet<S: Ord, T: Ord> {
    map: HashMap<T, S>,
    smap: OrderedSkipList<(S, T)>,
}

impl<S: Ord, T: Ord> SortedSet<S, T> {
    pub fn new() -> Self {
        Self { map: HashMap::new(), smap: OrderedSkipList::new() }
    }
}

impl<S: Ord + Clone + Eq + Hash + Ord, T: Ord + Clone + Eq + Hash + Default> SortedSet<S, T> {
    pub fn insert(&mut self, s: S, t: T) -> Option<S> {
        let old_s = self.map.insert(t.clone(), s.clone());
        if let Some(old_s) = &old_s {
            self.smap.remove_by_value(&(old_s.clone(), t.clone()));
        }
        self.smap.insert((s, t));
        old_s
    }

    pub fn remove(&mut self, t: &T) -> Option<S> {
        let s = self.map.remove(t);
        if let Some(s) = &s {
            self.smap.remove_by_value(&(s.clone(), t.clone()));
        }
        s
    }

    pub fn len(&self) -> usize { self.map.len() }

    pub fn get_score(&self, t: &T) -> Option<S> {
        self.map.get(t).cloned()
    }

    pub fn iter(&self) -> impl Iterator<Item=(&S, &T)> {
        self.smap.iter().map(|(s, t)| (s, t))
    }

    pub fn riter(&self) -> impl Iterator<Item=(&S, &T)> {
        self.smap.iter().rev().map(|(s, t)| (s, t))
    }

    pub fn range(&self, min: S, max: S) -> impl Iterator<Item=(&S, &T)> {
        let min = Bound::Included((min, T::default()));
        let max = Bound::Included((max, T::default()));
        self.smap.range((min, max)).map(|(s, t)| (s, t))
    }

    pub fn rank(&self, t: T) -> Option<(S, usize)> {
        let s = self.map.get(&t)?.clone();
        let r = self.smap.rank(&(s.clone(), t))?;
        Some((s, r))
    }
}

impl<S: Ord + Default, T: Ord + Default> Default for SortedSet<S, T> {
    fn default() -> Self { Self::new() }
}

impl<S: Ord + Clone, T: Ord + Clone> Clone for SortedSet<S, T> {
    fn clone(&self) -> Self {
        Self {
            map: self.map.clone(),
            smap: self.smap.clone(),
        }
    }
}

impl<S: Ord + Debug, T: Ord + Debug> Debug for SortedSet<S, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SortedSet").field("map", &self.map).finish()
    }
}

impl<S: Ord + PartialEq + Eq + Hash, T: Ord + PartialEq + Eq + Hash> PartialEq for SortedSet<S, T> {
    fn eq(&self, other: &Self) -> bool { self.map == other.map }
}

impl<S: Ord + PartialEq + Eq + Hash, T: Ord + PartialEq + Eq + Hash> Eq for SortedSet<S, T> {}
