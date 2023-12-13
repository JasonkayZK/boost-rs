use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

pub type RcRefCell<T> = Rc<RefCell<T>>;

pub type OptionRcRefCell<T> = Option<RcRefCell<T>>;

pub type VecRcRefCell<T> = Vec<RcRefCell<T>>;

pub type OptionVecRcRefCell<T> = Option<VecRcRefCell<T>>;

pub type ArcMutex<T> = Arc<Mutex<T>>;
