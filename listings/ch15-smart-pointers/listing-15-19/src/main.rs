enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

// ANCHOR: here
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("建立 a 後的計數 = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("建立 b 後的計數 = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("建立 c 後的計數 = {}", Rc::strong_count(&a));
    }
    println!("c 離開作用域後的計數 = {}", Rc::strong_count(&a));
}
// ANCHOR_END: here
