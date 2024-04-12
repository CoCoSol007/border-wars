use std::hash::Hash;
use std::ops::{Deref, DerefMut};

use dashmap::mapref::multiple::RefMutMulti;
use dashmap::mapref::one::RefMut;

pub enum GlobalRefMut<'a, K: Eq + Hash, V> {
    Single(RefMut<'a, K, V>),
    Multi(RefMutMulti<'a, K, V>),
}

impl<'a, K: Eq + Hash, V> From<RefMut<'a, K, V>> for GlobalRefMut<'a, K, V> {
    fn from(v: RefMut<'a, K, V>) -> Self {
        Self::Single(v)
    }
}

impl<'a, K: Eq + Hash, V> From<RefMutMulti<'a, K, V>> for GlobalRefMut<'a, K, V> {
    fn from(v: RefMutMulti<'a, K, V>) -> Self {
        Self::Multi(v)
    }
}

impl<'a, K: Eq + Hash, V> Deref for GlobalRefMut<'a, K, V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Single(v) => v.value(),
            Self::Multi(v) => v.value(),
        }
    }
}

impl<'a, K: Eq + Hash, V> DerefMut for GlobalRefMut<'a, K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Single(v) => v.value_mut(),
            Self::Multi(v) => v.value_mut(),
        }
    }
}
