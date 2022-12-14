use criterion::{criterion_group, criterion_main, Criterion};

mod btreeset;
mod linkedlist;
mod ordskiplist;
mod vec;

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets =
    crate::linkedlist::push_front,
    crate::linkedlist::rand_access,
    crate::linkedlist::iter,
    crate::ordskiplist::insert,
    crate::ordskiplist::rand_access,
    crate::ordskiplist::iter,
    crate::vec::insert,
    crate::vec::rand_access,
    crate::vec::iter,
    crate::btreeset::insert,
    crate::btreeset::rand_access,
    crate::btreeset::iter,
);

criterion_main!(benches);
