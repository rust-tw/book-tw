// ANCHOR: here
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
// ANCHOR_END: here

fn main() {
    let my_string = String::from("hello world");

    // first_word 能用在`String` 的切片
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word 能用在字串字面值
    let word = first_word(&my_string_literal[..]);

    // 因為字串字面值已經是字串切片了
    // 所以也可以不用加上字串語法！
    let word = first_word(my_string_literal);
}
