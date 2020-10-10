fn main() {
    let s = String::from("hello");  // s 進入作用域

    takes_ownership(s);             // s 的值進入函式
                                    // 所以 s 也在此無效

    let x = 5;                      // x 進入作用域

    makes_copy(x);                  // x 本該移動進函式裡
                                    // 但 i32 有 Copy，所以 x 可繼續使用

} // x 在此離開作用域，接著是 s。但因為 s 的值已經被移動了
  // 它不會有任何動作

fn takes_ownership(some_string: String) { // some_string 進入作用域
    println!("{}", some_string);
} // some_string 在此離開作用域並呼叫 `drop`
  // 佔用的記憶體被釋放

fn makes_copy(some_integer: i32) { // some_integer 進入作用域
    println!("{}", some_integer);
} // some_integer 在此離開作用域，沒有任何動作發生
