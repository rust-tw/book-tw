use add_one;

fn main() {
    let num = 10;
    println!(
        "你好，世界！{} 加一會是 {}！",
        num,
        add_one::add_one(num)
    );
}
