/// Implement an empty trait for batch types
///
/// # Examples
///
/// ```
/// use boost_rs::empty_trait_impl;
///
/// trait EmptyTrait{}
/// struct Foo{}
/// struct Bar{}
/// empty_trait_impl!(EmptyTrait for Foo Bar);
/// ```
#[macro_export]
macro_rules! empty_trait_impl {
  ($name:ident for $($t:ty)*) => ($(
    impl $name for $t {}
  )*)
}
