/// Create a **HashSet** from a list of elements.
///
/// ## Example
///
/// ```
/// # fn main() {
/// use boost_rs::hashset;
/// let set = hashset!{"a", "b"};
/// assert!(set.contains("a"));
/// assert!(set.contains("b"));
/// assert!(!set.contains("c"));
/// # }
/// ```
#[macro_export]
macro_rules! hashset {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(hashset!(@single $rest)),*]));

    ($($key:expr,)+) => { hashset!($($key),+) };
    ($($key:expr),*) => {
        {
            let _cap = hashset!(@count $($key),*);
            let mut _set = ::std::collections::HashSet::with_capacity(_cap);
            $(
                let _ = _set.insert($key);
            )*
            _set
        }
    };
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::convert_args;

    #[test]
    fn test_hashmap() {
        let names: HashSet<String> = convert_args!(hashset!("one", "two",));
        assert!(names.contains("one"));
        assert!(names.contains("two"));

        let lengths: HashSet<usize> = convert_args!(keys = str::len, hashset!("one", "two",));
        assert_eq!(lengths.len(), 1);

        let _no_trailing: HashSet<usize> = convert_args!(keys = str::len, hashset!("one", "two"));
    }
}
