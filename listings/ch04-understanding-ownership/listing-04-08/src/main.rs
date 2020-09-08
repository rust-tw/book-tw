fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// ANCHOR: here
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word 取得數值 5

    s.clear(); // 這會清空 String，這就等於 ""

    // word 仍然是數值 5 ，但是我們已經沒有相等意義的字串了
    // 擁有 5 的變數 word 現在完全沒意義！
}
// ANCHOR_END: here
