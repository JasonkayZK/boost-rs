use boost_rs::collection::cache::{Cache, LruCache};

fn main() {
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
