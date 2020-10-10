fn foo(_: i32, y: i32) {
    println!("此程式碼只使用了參數 y：{}", y);
}

fn main() {
    foo(3, 4);
}
