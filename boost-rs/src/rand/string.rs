use rand::{Rng};
use rand::distributions::Alphanumeric;

/// Generate random String with given charset of len length
///
/// # Example
///
/// ```
/// const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
///                     abcdefghijklmnopqrstuvwxyz\
///                     0123456789)(*&^%$#@!~";
/// const PASSWORD_LEN: usize = 30;
/// let password = boost_rs::rand::string::get_random_charset_string(PASSWORD_LEN, CHARSET);
/// assert_eq!(password.len(), PASSWORD_LEN);
/// for x in password.bytes() {
///     assert!(CHARSET.contains(&x));
/// }
/// ```
pub fn get_random_charset_string(len: usize, charset: &[u8]) -> String {
    (0..len)
        .map(|_| {
            let idx = rand::thread_rng().gen_range(0..charset.len());
            // It's safe, because `idx` is in `CHARSET`'s range.
            char::from(unsafe { *charset.get_unchecked(idx) })
        })
        .collect()
}

/// Generate random String with alphanumeric(uppercase, lowercase & number) charset of len length
///
/// # Example
///
/// ```
///const GEN_ASCII_STR_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
///         abcdefghijklmnopqrstuvwxyz\
///         0123456789";
/// const PASSWORD_LEN: usize = 30;
/// let password = boost_rs::rand::string::get_random_alphanumeric_string(PASSWORD_LEN);
/// assert_eq!(password.len(), PASSWORD_LEN);
/// for x in password.bytes() {
///     assert!(GEN_ASCII_STR_CHARSET.contains(&x));
/// }
/// ```
pub fn get_random_alphanumeric_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_random_charset_string_test() {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
        const PASSWORD_LEN: usize = 30;

        let password = get_random_charset_string(PASSWORD_LEN, CHARSET);

        assert_eq!(password.len(), PASSWORD_LEN);
        for x in password.bytes() {
            assert!(CHARSET.contains(&x));
        }
    }

    #[test]
    fn get_random_alphanumeric_string_test() {
        const GEN_ASCII_STR_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                abcdefghijklmnopqrstuvwxyz\
                0123456789";
        const PASSWORD_LEN: usize = 30;

        let password = get_random_alphanumeric_string(PASSWORD_LEN);
        assert_eq!(password.len(), PASSWORD_LEN);
        for x in password.bytes() {
            assert!(GEN_ASCII_STR_CHARSET.contains(&x));
        }
    }
}
