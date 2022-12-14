//! Sort utility functions
//!
//! Such as:
//!
//! - bubble_sort
//! - heap_sort
//! - insertion_sort
//! - merge_sort
//! - quick_sort
//! - selection_sort

pub mod bubble_sort;
pub mod heap_sort;
pub mod insertion_sort;
pub mod merge_sort;
pub mod quick_sort;
pub mod selection_sort;

/// Check the slice is sorted
///
/// # Examples
///
/// ```
/// # fn main() {
/// use boost_rs::sort::is_sorted;
/// assert!(is_sorted(&[1,2,3]));
/// assert!(!is_sorted(&[1,3,2]));
/// # }
/// ```
pub fn is_sorted<T: PartialOrd>(arr: &[T]) -> bool {
    is_sorted_with_comparator(arr, |x, y| x.lt(y))
}

/// Check the slice is sorted with a custom comparator
///
/// # Examples
///
/// ```
/// # fn main() {
/// use boost_rs::sort::is_sorted_with_comparator;
/// assert!(is_sorted_with_comparator(&[1,2,3], |x: &i32, y: &i32| { x.lt(y)} ));
/// assert!(is_sorted_with_comparator(&[3,2,1], |x: &i32, y: &i32| { y.lt(x)} ));
/// assert!(!is_sorted_with_comparator(&[1,3,2], |x: &i32, y: &i32| { x.lt(y)} ));
/// # }
/// ```
pub fn is_sorted_with_comparator<T, F>(arr: &[T], is_less: F) -> bool
where
    T: PartialOrd,
    F: Fn(&T, &T) -> bool,
{
    for i in 1..arr.len() {
        if is_less(&arr[i], &arr[i - 1]) {
            return false;
        }
    }
    true
}
