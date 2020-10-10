// ANCHOR: here
fn first_word(s: &str) -> &str {
    // ANCHOR_END: here
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// ANCHOR: usage
fn main() {
    let my_string = String::from("hello world");

    // first_word 適用於 `String` 的切片
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word 適用於字串字面值
    let word = first_word(&my_string_literal[..]);

    // 因為字串字面值*本來*就是切片
    // 沒有切片語法也是可行的！
    let word = first_word(my_string_literal);
}
// ANCHOR_END: usage
