use boost_rs::collection::skiplist::SkipList;

fn add_test(l: &mut SkipList<i32>, i: i32) {
    l.add(i).unwrap();
}

fn contains_test(l: &mut SkipList<i32>, i: &i32) {
    l.contains(i);
}

fn main() {
    let mut l = SkipList::new();

    for x in 0..10 {
        add_test(&mut l, x);
        contains_test(&mut l, &x);
    }
    l.print();

    l.add(-1).unwrap();
    l.print();

    l.remove(&3);
    l.print();
}
