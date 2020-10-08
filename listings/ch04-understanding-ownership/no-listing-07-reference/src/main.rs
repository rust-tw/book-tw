// ANCHOR: all
fn main() {
    // ANCHOR: here
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
    // ANCHOR_END: here

    println!("'{}' 的長度爲 {}。", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
// ANCHOR_END: all
