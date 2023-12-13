use boost_rs::types::as_any::{AsAny, Downcast};

trait Custom: AsAny {
    fn hello(&self) -> String;
}

struct Test {
    age: i32,
}

impl Custom for Test {
    fn hello(&self) -> String {
        String::from("This is Test!")
    }
}

fn main() {
    let y: Box<dyn Custom> = Box::new(Test { age: 1 });
    println!("age: {:?}", (*y).downcast_ref::<Test>().unwrap().age)
}
