use std::collections::LinkedList;

use rand::Rng;

use boost_rs::collection::skiplist::OrdSkipList;
use boost_rs_macros::elapsed;

#[elapsed]
fn skiplist_random_test(l: &OrdSkipList<i32>, search_val: &Vec<i32>) {
    for i in search_val {
        assert!(l.contains(i));
    }
}

#[elapsed]
fn linkedlist_random_test(l: &LinkedList<i32>, search_val: &Vec<i32>) {
    for i in search_val {
        assert!(l.contains(i));
    }
}

fn gen_random(cap: usize, search_cap: usize) -> Vec<i32> {
    let mut v = vec![];
    for _ in 0..search_cap {
        let mut rng = rand::thread_rng();
        v.push(rng.gen_range(0..cap) as i32);
    }
    v
}

fn main() {
    let mut sl = OrdSkipList::default();
    let mut ll = LinkedList::new();
    let cap = 100000;
    let search_cap = 10000;
    let search_val = gen_random(cap, search_cap);

    for x in 0..cap {
        let x = x as i32;
        sl.insert(x).unwrap();
        ll.push_back(x);
    }

    skiplist_random_test(&sl, &search_val);
    linkedlist_random_test(&ll, &search_val);
}
