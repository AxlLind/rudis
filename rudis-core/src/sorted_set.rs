use std::{collections::{BTreeMap, HashMap, HashSet}, fmt::Debug, hash::Hash};

pub struct SortedSet<T, S> {
    map: HashMap<T, S>,
    smap: BTreeMap<S, HashSet<T>>,
}

impl<T, S> SortedSet<T, S> {
    pub fn new() -> Self {
        Self { map: HashMap::new(), smap: BTreeMap::new() }
    }
}

impl<T: Clone + Eq + Hash, S: Clone + Eq + Hash + Ord> SortedSet<T, S> {

    pub fn insert(&mut self, t: T, s: S) -> Option<S> {
        let old_s = self.map.insert(t.clone(), s.clone());
        if let Some(old_s) = &old_s {
            self.smap.get_mut(old_s).unwrap().remove(&t);
        }
        self.smap.entry(s).or_default().insert(t);
        old_s
    }

    pub fn remove(&mut self, t: &T) -> Option<S> {
        let s = self.map.remove(t);
        if let Some(s) = &s {
            self.smap.get_mut(s).unwrap().remove(t);
        }
        s
    }

    pub fn len(&self) -> usize { self.map.len() }

    pub fn get_score(&self, t: &T) -> Option<S> {
        self.map.get(t).cloned()
    }

    pub fn iter(&self) -> impl Iterator<Item=(&T, &S)> {
        self.smap.iter().flat_map(|(s, set)| set.iter().map(move |t| (t, s)))
    }

    pub fn riter(&self) -> impl Iterator<Item=(&T, &S)> {
        self.smap.iter().rev().flat_map(|(s, set)| set.iter().map(move |t| (t, s)))
    }

    pub fn range(&self, min: S, max: S) -> impl Iterator<Item=(&T, &S)> {
        self.smap.range(min..=max).flat_map(|(s, set)| set.iter().map(move |t| (t, s)))
    }
}

impl<T: Default, S: Default> Default for SortedSet<T, S> {
    fn default() -> Self { Self::new() }
}

impl<T: Clone, S: Clone> Clone for SortedSet<T, S> {
    fn clone(&self) -> Self {
        Self {
            map: self.map.clone(),
            smap: self.smap.clone(),
        }
    }
}

impl<T: Debug, S: Debug> Debug for SortedSet<T, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SortedSet").field("map", &self.map).finish()
    }
}

impl<T: PartialEq + Eq + Hash, S: PartialEq + Eq + Hash> PartialEq for SortedSet<T, S> {
    fn eq(&self, other: &Self) -> bool { self.map == other.map }
}

impl<T: PartialEq + Eq + Hash, S: PartialEq + Eq + Hash> Eq for SortedSet<T, S> {}
