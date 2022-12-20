pub use self::lru::*;

pub mod lru;

pub trait Cache<K: Eq, V> {
    fn get(&mut self, key: &K) -> Option<&V>;

    fn put(&mut self, key: K, value: V) -> Option<V>;

    fn capacity(&self) -> usize;
}
