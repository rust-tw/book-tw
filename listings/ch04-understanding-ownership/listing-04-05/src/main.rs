fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("'{}' 的長度爲 {}。", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 回傳 String 的長度

    (s, length)
}
