fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("'{}' 的長度為 {}。", s1, len);
}

// ANCHOR: here
fn calculate_length(s: &String) -> usize { // s 是個 String 的參考
    s.len()
} // s 在此離開作用域，但因為它沒有它所指向的資料的所有權
  // 所以不會被釋放掉
// ANCHOR_END: here
