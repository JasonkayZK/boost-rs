use boost_rs::{btreeset, convert_args, hashmap};
use std::collections::BTreeSet;

fn main() {
    use std::collections::HashMap;
    // a. Use the default conversion with the Into trait.
    // Here this converts both the key and value string literals to `String`,
    // but we need to specify the map type exactly!
    let map1: HashMap<String, String> = convert_args!(hashmap!(
        "a" => "b",
        "c" => "d",
    ));
    println!("map1: {:?}", map1);

    // b. Specify an explicit custom conversion for the keys. If we don't specify
    // a conversion for the values, they are not converted at all.
    let map2 = convert_args!(
        keys = String::from,
        hashmap!(
            "a" => 1,
            "c" => 2,
        )
    );
    // Note: map2 is a HashMap<String, i32>, but we didn't need to specify the type
    let _: HashMap<String, i32> = map2;
    println!("map2: {:?}", map2);

    // c. convert_args! works with all the maplit macros -- and macros from other
    // crates that have the same "signature".
    // For example, btreeset and conversion from &str to Vec<u8>.
    let set: BTreeSet<Vec<u8>> = convert_args!(btreeset!("a", "b", "c", "d", "a", "e", "f",));
    assert_eq!(set.len(), 6);
    println!("set: {:?}", set);
}
