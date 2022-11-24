use std::thread;
use std::time::Duration;
use boost_rs_macros::{elapsed, HelloMacro};

pub trait HelloMacro {
    fn hello_macro();
}

#[derive(HelloMacro)]
struct Sunfei;

#[derive(HelloMacro)]
struct Sunface;

#[elapsed]
fn deco(t: u64) {
    let secs = Duration::from_secs(t);
    thread::sleep(secs);
}

fn main() {
    Sunfei::hello_macro();
    Sunface::hello_macro();

    deco(4);
    deco(2);
}
