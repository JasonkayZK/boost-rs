#[macro_export]
/// Create a **HashMap** from a list of key-value pairs
///
/// ## Example
///
/// ```
/// # fn main() {
/// use boost_rs::hashmap;
/// let map = hashmap!{
///     "a" => 1,
///     "b" => 2,
/// };
/// assert_eq!(map["a"], 1);
/// assert_eq!(map["b"], 2);
/// assert_eq!(map.get("c"), None);
/// # }
/// ```
macro_rules! hashmap {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(hashmap!(@single $rest)),*]));

    ($($key:expr => $value:expr,)+) => { hashmap!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let _cap = hashmap!(@count $($key),*);
            let mut _map = ::std::collections::HashMap::with_capacity(_cap);
            $(
                let _ = _map.insert($key, $value);
            )*
            _map
        }
    };
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::collection::__id;
    use crate::convert_args;

    #[test]
    fn test_hashmap() {
        let names = hashmap! {
            1 => "one",
            2 => "two",
        };
        assert_eq!(names.len(), 2);
        assert_eq!(names[&1], "one");
        assert_eq!(names[&2], "two");
        assert_eq!(names.get(&3), None);

        let empty: HashMap<i32, i32> = hashmap! {};
        assert_eq!(empty.len(), 0);

        let _nested_compiles = hashmap! {
            1 => hashmap!{0 => 1 + 2,},
            2 => hashmap!{1 => 1,},
        };

        let _: HashMap<String, i32> = convert_args!(
            keys = String::from,
            hashmap!(
                "one" => 1,
                "two" => 2,
            )
        );

        let _: HashMap<String, i32> = convert_args!(
            keys = String::from,
            values = __id,
            hashmap!(
                "one" => 1,
                "two" => 2,
            )
        );
    }
}
