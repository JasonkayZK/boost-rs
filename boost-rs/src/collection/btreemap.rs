#[macro_export]
/// Create a **BTreeMap** from a list of key-value pairs
///
/// ## Example
///
/// ```
/// # fn main() {
/// use boost_rs::btreemap;
/// let map = btreemap!{
///     "a" => 1,
///     "b" => 2,
/// };
/// assert_eq!(map["a"], 1);
/// assert_eq!(map["b"], 2);
/// assert_eq!(map.get("c"), None);
/// # }
/// ```
macro_rules! btreemap {
    // trailing comma case
    ($($key:expr => $value:expr,)+) => (btreemap!($($key => $value),+));

    ( $($key:expr => $value:expr),* ) => {
        {
            let mut _map = ::std::collections::BTreeMap::new();
            $(
                let _ = _map.insert($key, $value);
            )*
            _map
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_btreemap() {
        use std::collections::BTreeMap;
        let names = btreemap! {
            1 => "one",
            2 => "two",
        };
        assert_eq!(names.len(), 2);
        assert_eq!(names[&1], "one");
        assert_eq!(names[&2], "two");
        assert_eq!(names.get(&3), None);

        let empty: BTreeMap<i32, i32> = btreemap! {};
        assert_eq!(empty.len(), 0);

        let _nested_compiles = btreemap! {
            1 => btreemap!{0 => 1 + 2,},
            2 => btreemap!{1 => 1,},
        };
    }
}
