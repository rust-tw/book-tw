fn main() {
    // ANCHOR: here
    let mut s = String::from("hello");

    let r1 = &s; // 沒問題
    let r2 = &s; // 沒問題
    println!("{} and {}", r1, r2);
    // 變數 r1 和 r2 將不再使用
    
    let r3 = &mut s; // 沒問題
    println!("{}", r3);
    // ANCHOR_END: here
}
