//! A implement of LRU Cache based on Doubly-LinkedList and HashMap.

use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::{BuildHasher, Hash};
use std::ptr::NonNull;

use crate::collection::cache::Cache;
use crate::collection::linkedlist::{LinkedList, Node};

const DEFAULT_CAPACITY: usize = 1024;

struct LruEntry<K: Eq + Hash + Clone, V> {
    key: K,
    value: V,
}

impl<K: Eq + Hash + Clone, V> PartialEq<Self> for LruEntry<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.key.eq(&other.key)
    }
}

impl<K: Eq + Hash + Clone, V> Eq for LruEntry<K, V> {}

impl<K: Eq + Hash + Clone, V> LruEntry<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Self { key, value }
    }
}

pub struct LruCache<K: Eq + Hash + Clone, V, S: BuildHasher = RandomState> {
    map: HashMap<K, NonNull<Node<LruEntry<K, V>>>, S>,
    cache: LinkedList<LruEntry<K, V>>,
    cap: usize,
}

impl<K: Eq + Hash + Clone + Debug, V: Debug, S: BuildHasher> LruCache<K, V, S> {
    pub fn traverse(&self) {
        print!("{{ ");
        for entry in self.cache.iter() {
            print!("({:?}, {:?}) -> ", &entry.key, &entry.value);
        }
        println!("None }}");
    }
}

impl<K: Eq + Hash + Clone, V> LruCache<K, V, RandomState> {
    pub fn with_capacity(cap: usize) -> Self {
        LruCache {
            map: HashMap::with_capacity(cap),
            cache: LinkedList::new(),
            cap,
        }
    }
}

impl<K: Eq + Hash + Clone, V, S: BuildHasher> LruCache<K, V, S> {
    pub fn with_hasher(hasher: S) -> Self {
        LruCache {
            map: HashMap::with_capacity_and_hasher(DEFAULT_CAPACITY, hasher),
            cache: Default::default(),
            cap: DEFAULT_CAPACITY,
        }
    }

    pub fn with_capacity_and_hasher(cap: usize, hasher: S) -> Self {
        LruCache {
            map: HashMap::with_capacity_and_hasher(cap, hasher),
            cache: Default::default(),
            cap,
        }
    }
}

impl<K: Eq + Hash + Clone, V> Default for LruCache<K, V, RandomState> {
    fn default() -> Self {
        LruCache {
            map: HashMap::default(),
            cache: LinkedList::default(),
            cap: DEFAULT_CAPACITY,
        }
    }
}

impl<K: Eq + Hash + Clone, V, S: BuildHasher> Cache<K, V> for LruCache<K, V, S> {
    fn get(&mut self, key: &K) -> Option<&V> {
        let node = self.map.get(key)?;

        let val = unsafe { &node.as_ref().val().value };

        self.cache.move_raw_node_to_head(*node);

        Some(val)
    }

    fn put(&mut self, key: K, value: V) -> Option<V> {
        let new_key = key.clone();
        let new_node = LruEntry::new(key, value);
        let new_node = Box::new(Node::new(new_node));
        let new_node = NonNull::new(Box::into_raw(new_node)).unwrap();

        match self.map.get(&new_key) {
            Some(val) => unsafe {
                let removed = self.cache.remove_by_val(val.as_ref().val())?;
                self.cache._push_front_raw(new_node);
                self.map.insert(new_key, new_node);
                Some(removed.value)
            },
            None => {
                // Not found
                let mut val = None;
                if self.cache.length() >= self.cap {
                    // Cache is full, remove
                    if let Some(entry) = self.cache.pop_back() {
                        self.map.remove(&entry.key);
                        val = Some(entry.value);
                    }
                }
                self.cache._push_front_raw(new_node);
                self.map.insert(new_key, new_node);
                val
            }
        }
    }

    fn capacity(&self) -> usize {
        self.cap
    }
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::RandomState;

    use crate::collection::cache::{Cache, LruCache};

    #[test]
    fn test_new() {
        let _l: LruCache<i32, String> = LruCache::default();
        let _l: LruCache<i32, String> = LruCache::with_capacity(10);
        let _l: LruCache<i32, String> = LruCache::with_hasher(RandomState::new());
        let _l: LruCache<i32, String> = LruCache::with_capacity_and_hasher(10, RandomState::new());
    }

    #[test]
    fn test_cache() {
        let mut l = LruCache::with_capacity(4);
        l.put("1".to_string(), 1);
        l.put("2".to_string(), 2);
        l.put("3".to_string(), 3);
        l.put("4".to_string(), 4);
        l.traverse();

        assert_eq!(l.get(&"1".to_string()), Some(&1));
        assert_eq!(l.get(&"2".to_string()), Some(&2));
        assert_eq!(l.get(&"3".to_string()), Some(&3));
        assert_eq!(l.get(&"4".to_string()), Some(&4));
        assert_eq!(l.get(&"5".to_string()), None);
        l.traverse();

        l.put("5".to_string(), 5);
        assert_eq!(l.get(&"5".to_string()), Some(&5));
        assert_eq!(l.get(&"1".to_string()), None); // Cache cleaned
        l.traverse();
    }

    #[test]
    fn test_cache2() {
        let mut l = LruCache::with_capacity(4);
        l.put("1".to_string(), 1);
        l.put("2".to_string(), 2);
        l.put("3".to_string(), 3);
        l.traverse();
        l.put("4".to_string(), 4);
        l.put("5".to_string(), 5);
        l.traverse();
        l.put("6".to_string(), 6);
        l.put("7".to_string(), 7);
        l.put("8".to_string(), 8);
        l.traverse();
    }
}
