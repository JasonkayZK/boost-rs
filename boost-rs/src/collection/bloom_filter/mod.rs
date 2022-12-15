//! Bloom filter implementation for rust.
//!
//! Wikipedia:
//!  - https://en.wikipedia.org/wiki/Bloom_filter

use std::collections::hash_map::{DefaultHasher, RandomState};
use std::hash::{BuildHasher, Hash, Hasher};
use std::marker::PhantomData;

use bitvec::vec::BitVec;

const DEFAULT_CAPACITY: usize = 10240;

type HasherArray = Box<[Box<dyn BuildHasher<Hasher=DefaultHasher>>]>;

pub struct BloomFilter<T: ?Sized + Hash> {
    cap: usize,
    bit_array: BitVec,
    hashers: HasherArray,
    _phantom: PhantomData<T>,
}

impl<T: ?Sized + Hash> BloomFilter<T> {
    pub fn with_capacity(cap: usize) -> Self {
        let v: Vec<Box<dyn BuildHasher<Hasher=DefaultHasher>>> = vec![
            Box::new(RandomState::new()),
            Box::new(RandomState::new()),
        ];
        let hash_arr = HasherArray::from(v);
        BloomFilter {
            cap,
            bit_array: BitVec::repeat(false, cap),
            hashers: hash_arr,
            _phantom: Default::default(),
        }
    }

    pub fn with_hashers<const N: usize>(hashers: [Box<dyn BuildHasher<Hasher=DefaultHasher>>; N]) -> Self {
        let hash_arr = HasherArray::from(hashers);
        BloomFilter {
            cap: DEFAULT_CAPACITY,
            bit_array: BitVec::repeat(false, DEFAULT_CAPACITY),
            hashers: hash_arr,
            _phantom: Default::default(),
        }
    }

    pub fn with_cap_and_hashers<const N: usize>(cap: usize, hashers: [Box<dyn BuildHasher<Hasher=DefaultHasher>>; N]) -> Self {
        let hash_arr = HasherArray::from(hashers);
        BloomFilter {
            cap,
            bit_array: BitVec::repeat(false, cap),
            hashers: hash_arr,
            _phantom: Default::default(),
        }
    }

    pub fn set(&mut self, item: &T) {
        for i in 0..self.hashers.len() {
            let bit_offset = self.calculate_hash(i, item) as usize;
            self.bit_array.set(bit_offset, true);
        }
    }

    pub fn might_contain(&self, item: &T) -> bool {
        for i in 0..self.hashers.len() {
            let bit_offset = self.calculate_hash(i, item) as usize;
            match self.bit_array.get(bit_offset) {
                None => return false,
                Some(res) => {
                    if !res {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn cap(&self) -> usize {
        self.cap
    }

    fn calculate_hash(&self, idx: usize, item: &T) -> u64 {
        let mut hasher = self.hashers[idx].build_hasher();
        item.hash(&mut hasher);
        hasher.finish() % (self.cap as u64)
    }
}

impl<T: ?Sized + Hash> Default for BloomFilter<T> {
    fn default() -> Self {
        let v: Vec<Box<dyn BuildHasher<Hasher=DefaultHasher>>> = vec![
            Box::new(RandomState::new()),
            Box::new(RandomState::new()),
        ];
        let hash_arr = HasherArray::from(v);
        BloomFilter {
            bit_array: BitVec::repeat(false, DEFAULT_CAPACITY),
            cap: DEFAULT_CAPACITY,
            hashers: hash_arr,
            _phantom: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::RandomState;

    use crate::collection::bloom_filter::BloomFilter;

    #[test]
    fn test_new() {
        let _f: BloomFilter<String> = BloomFilter::default();
        let _f: BloomFilter<String> = BloomFilter::with_capacity(4);
        let _f: BloomFilter<String> = BloomFilter::with_hashers([
            Box::new(RandomState::new()),
            Box::new(RandomState::new()),
        ]);
        let _f: BloomFilter<String> = BloomFilter::with_cap_and_hashers(4, [
            Box::new(RandomState::new()),
            Box::new(RandomState::new()),
        ]);
    }

    #[test]
    fn test_filter() {
        let mut f: BloomFilter<String> = BloomFilter::with_capacity(102400);
        for x in 0..10000 {
            f.set(&x.to_string());
        }

        for x in 5000..15000 {
            if x < 1000 {
                assert!(f.might_contain(&x.to_string()));
            }
        }
    }
}
