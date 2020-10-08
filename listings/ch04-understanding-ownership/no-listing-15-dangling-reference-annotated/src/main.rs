fn main() {
    let reference_to_nothing = dangle();
}

// ANCHOR: here
fn dangle() -> &String { // 回傳 String 的迷途引用

    let s = String::from("hello"); // s 是個新 String

    &s // 我們回傳 String s 的引用
} // s 在此會離開作用域並釋放，它的記憶體就不見了。
  // 危險！
// ANCHOR_END: here
