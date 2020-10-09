fn main() {
    // ANCHOR: here
    {
        let v = vec![1, 2, 3, 4];

        // 使用 v 做些事情
    } // <- v 在此離開作用域並釋放
      // ANCHOR_END: here
}
