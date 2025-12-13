use hashbrown::hash_map as base;
use base::DefaultHashBuilder;
use core::hash::{BuildHasher,Hash};

pub struct HashMap<K, V, S = DefaultHashBuilder> {
    inner: base::HashMap<K, V, S>,
}

pub struct Iter<'a, K, V> {
    inner: base::Iter<'a, K, V>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);
    
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}


impl<K,V> HashMap<K, V> {
    pub fn new() -> Self {
        Self{
            inner: base::HashMap::new(),
        }
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter { inner: self.inner.iter() }
    }
}

impl<K, V, S> HashMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.inner.insert(k,v)
    }
}

