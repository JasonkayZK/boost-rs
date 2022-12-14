use boost_rs::collection::skiplist::{Options, SkipList};

fn add_test(l: &mut SkipList<i32>, i: i32) {
    l.add(i).unwrap();
}

fn contains_test(l: &mut SkipList<i32>, i: &i32) {
    l.contains(i);
}

fn main() {
    let mut l = SkipList::ord_with_options(Options{
        cmp: None,
        level_bound: Some(4),
        level_generator: None,
    }).unwrap();

    for x in 0..10 {
        add_test(&mut l, x);
        contains_test(&mut l, &x);
    }
    l.print();

    l.add(-1).unwrap();
    l.print();

    let res = l.remove(&3);
    println!("res: {:?}", res);
    l.print();
}
