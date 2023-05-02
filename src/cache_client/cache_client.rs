use std::collections::HashMap;

pub trait Cache<K: Eq, V> {
    fn write_key(&mut self, key: K, data: V) -> Result<(), ()>;
    fn read_key(&self, key: &K) -> Option<&V>;
    fn read_key_mut(&mut self, key: &K) -> Option<&mut V>;
    fn clear_key(&mut self, key: &K) -> Option<V>;
}

pub struct MemoryCache<K: Eq + std::hash::Hash + Clone, V> {
    hash_map: HashMap<K, V>,
}

impl<K: Eq + std::hash::Hash + Clone, V> MemoryCache<K, V> {
    pub fn new() -> Self {
        Self {
            hash_map: HashMap::new(),
        }
    }
}

impl<K, V> Cache<K, V> for MemoryCache<K, V>
where
    K: Eq + std::hash::Hash + Clone,
{
    fn write_key(&mut self, key: K, data: V) -> Result<(), ()> {
        self.hash_map.insert(key, data);
        Ok(())
    }

    fn read_key(&self, key: &K) -> Option<&V> {
        self.hash_map.get(&key)
    }

    fn read_key_mut(&mut self, key: &K) -> Option<&mut V> {
        self.hash_map.get_mut(&key)
    }

    fn clear_key(&mut self, key: &K) -> Option<V> {
        self.hash_map.remove(&key)
    }
}
