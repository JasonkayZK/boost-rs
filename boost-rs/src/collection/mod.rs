//! Utility functions for collections
//!
//!
//! # Macros
//!
//! Macros for container literals with specific type.
//!
//! ```
//! # fn main() {
//! use boost_rs::hashmap;
//! let map = hashmap!{
//!     "a" => 1,
//!     "b" => 2,
//! };
//! assert_eq!(map.len() , 2);
//! # }
//! ```
//!
//! The **collection** crate uses `=>` syntax to separate the key and value for the
//! mapping macros. (It was not possible to use `:` as separator due to syntactic
//! restrictions in regular `macro_rules!` macros.)
//!
//! Note that rust macros are flexible in which brackets you use for the invocation.
//! You can use them as `hashmap!{}` or `hashmap![]` or `hashmap!()`.

pub mod bloom_filter;
pub mod bst;
pub mod btreemap;
pub mod btreeset;
pub mod cache;
pub mod error;
pub mod hashmap;
pub mod hashset;
pub mod linkedlist;
pub mod skiplist;

/// Identity function. Used as the fallback for conversion.
#[doc(hidden)]
pub fn __id<T>(t: T) -> T {
    t
}

/// Macro that converts the keys or key-value pairs passed to another
/// macro. The default conversion is to use the [`Into`] trait, if no
/// custom conversion is passed.
///
/// The syntax is:
///
/// `convert_args!(` `keys=` *function* `,` `values=` *function* `,`
///     *macro_name* `!(` [ *key* => *value* [, *key* => *value* ... ] ] `))`
///
/// Here *macro_name* is any other macro and either or both of the
/// explicit `keys=` and `values=` parameters can be omitted.
///
/// [`Into`]: https://doc.rust-lang.org/std/convert/trait.Into.html
///
/// **Note** To use `convert_args`, the macro that is being wrapped
/// must itself be brought into the current scope with `#[macro_use]` or `use`.
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate boost_rs;
/// # fn main() {
///
/// use std::collections::HashMap;
/// use std::collections::BTreeSet;
///
/// // a. Use the default conversion with the Into trait.
/// // Here this converts both the key and value string literals to `String`,
/// // but we need to specify the map type exactly!
///
/// let map1: HashMap<String, String> = convert_args!(hashmap!(
///     "a" => "b",
///     "c" => "d",
/// ));
///
/// // b. Specify an explicit custom conversion for the keys. If we don't specify
/// // a conversion for the values, they are not converted at all.
///
/// let map2 = convert_args!(keys=String::from, hashmap!(
///     "a" => 1,
///     "c" => 2,
/// ));
///
/// // Note: map2 is a HashMap<String, i32>, but we didn't need to specify the type
/// let _: HashMap<String, i32> = map2;
///
/// // c. convert_args! works with all the macros -- and macros from other
/// // crates that have the same "signature".
/// // For example, btreeset and conversion from &str to Vec<u8>.
///
/// let set: BTreeSet<Vec<u8>> = convert_args!(btreeset!(
///     "a", "b", "c", "d", "a", "e", "f",
/// ));
/// assert_eq!(set.len(), 6);
///
///
/// # }
/// ```
#[macro_export]
macro_rules! convert_args {
    (keys=$kf:expr, $macro_name:ident !($($k:expr),* $(,)*)) => {
        $macro_name! { $(($kf)($k)),* }
    };
    (keys=$kf:expr, values=$vf:expr, $macro_name:ident !($($k:expr),* $(,)*)) => {
        $macro_name! { $(($kf)($k)),* }
    };
    (keys=$kf:expr, values=$vf:expr, $macro_name:ident !( $($k:expr => $v:expr),* $(,)*)) => {
        $macro_name! { $(($kf)($k) => ($vf)($v)),* }
    };
    (keys=$kf:expr, $macro_name:ident !($($rest:tt)*)) => {
        convert_args! {
            keys=$kf, values=$crate::collection::__id,
            $macro_name !(
                $($rest)*
            )
        }
    };
    (values=$vf:expr, $macro_name:ident !($($rest:tt)*)) => {
        convert_args! {
            keys=$crate::collection::__id, values=$vf,
            $macro_name !(
                $($rest)*
            )
        }
    };
    ($macro_name:ident ! $($rest:tt)*) => {
        convert_args! {
            keys=::std::convert::Into::into, values=::std::convert::Into::into,
            $macro_name !
            $($rest)*
        }
    };
}
