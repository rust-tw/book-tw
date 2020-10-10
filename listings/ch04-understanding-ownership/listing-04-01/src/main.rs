fn main() {
    // ANCHOR: here
    {                      // s 在此處無效，因為它還沒宣告
        let s = "hello";   // s 在此開始視為有效

        // 使用 s
    }                      // 此作用域結束， s 不再有效
    // ANCHOR_END: here
}