fn main() {
    let s1 = gives_ownership();         // gives_ownership 移動它的回傳值給 s1

    let s2 = String::from("hello");     // s2 進入作用域

    let s3 = takes_and_gives_back(s2);  // s2 移入 takes_and_gives_back
                                        // 該函式又將其回傳值移到 s3
} // s3 在此離開作用域並釋放
  // s2 離開作用域但已被移走，沒有任何動作發生
  // s1 離開作用域並釋放

fn gives_ownership() -> String {             // gives_ownership 會將他的回傳值
                                             // 移動給呼叫它的函式

    let some_string = String::from("hello"); // some_string 進入作用域

    some_string                              // 回傳 some_string 並移動給
                                             // 呼叫它的函式
}

// takes_and_gives_back 會取得一個 String 然後回傳它
fn takes_and_gives_back(a_string: String) -> String { // a_string 進入作用域

    a_string  // 回傳 a_string 並移動給呼叫的函式
}
