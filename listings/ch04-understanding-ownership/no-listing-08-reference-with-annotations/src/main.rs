fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

// ANCHOR: here
fn calculate_length(s: &String) -> usize { // s 個 String 的引用
    s.len()
} // s 在此離開作用域，但因爲它沒有它所指向的資料的所有權
  // 沒有任何動作發生
// ANCHOR_END: here
