use std::sync::{Arc, Mutex};

pub type ArcMut<T> = Arc<Mutex<T>>;
