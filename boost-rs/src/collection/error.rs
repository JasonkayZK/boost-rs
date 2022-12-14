use thiserror::Error;

#[derive(Error, Debug)]
pub enum CollectionError {
    #[error("index out of range")]
    IndexOutOfRange,

    #[error("invalid parameter")]
    InvalidParameter(String),

    #[error("key already exists")]
    DuplicateKey,
}
