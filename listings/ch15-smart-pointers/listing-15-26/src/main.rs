use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

// ANCHOR: here
fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a 初始引用計數 = {}", Rc::strong_count(&a));
    println!("a 下個項目 = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a 在 b 建立後的引用計數 = {}", Rc::strong_count(&a));
    println!("b 初始引用計數 = {}", Rc::strong_count(&b));
    println!("b 下個項目 = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b 在變更 a 後的引用計數 = {}", Rc::strong_count(&b));
    println!("a 在變更 a 後的引用計數 = {}", Rc::strong_count(&a));

    // 取消下一行的註解可以看到循環產生
    // 這會讓堆疊溢位
    // println!("a 下個項目 = {:?}", a.tail());
}
// ANCHOR_END: here
