fn main() {
    // ANCHOR: here
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 離開作用域，所以建立新的引用也不會有問題

    let r2 = &mut s;
    // ANCHOR_END: here
}
