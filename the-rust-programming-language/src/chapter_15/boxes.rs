use std::ops::Deref;

use crate::chapter_15::boxes::List::{Cons, Nil};

enum List {
    Cons(i32, Box<List>),
    Nil,
}
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
fn hello(name: &str) {
    println!("Hello, {name}!");
}
pub fn boxes() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}
