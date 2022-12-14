//! Env for runtime environment

/// Check if is debug environment
///
/// # Usage
///
/// ```
/// # fn main() {
/// use boost_rs::env::debug_mode;
/// if debug_mode() {
///     println!("run with debug mode, print this line");
/// } else {
///     println!("not run with debug mode, print this line");
/// }
/// # }
/// ```
pub fn debug_mode() -> bool {
    cfg!(debug_assertion)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_debug_mode() {
        if crate::env::debug_mode() {
            println!("run with debug mode, print this line");
        } else {
            println!("not run with debug mode, print this line");
        }
    }
}
