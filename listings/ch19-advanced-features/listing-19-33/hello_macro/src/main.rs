use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("你好，巨集！我叫作鬆餅！");
    }
}

fn main() {
    Pancakes::hello_macro();
}
