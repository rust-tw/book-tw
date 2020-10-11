// ANCHOR: all
// ANCHOR: io
use std::io;
// ANCHOR_END: io

// ANCHOR: main
fn main() {
    // ANCHOR_END: main
    // ANCHOR: print
    println!("請猜測一個數字！");

    println!("請輸入你的猜測數字。");
    // ANCHOR_END: print

    // ANCHOR: string
    let mut guess = String::new();
    // ANCHOR_END: string

    // ANCHOR: read
    io::stdin()
        .read_line(&mut guess)
        // ANCHOR_END: read
        // ANCHOR: expect
        .expect("讀取該行失敗");
    // ANCHOR_END: expect

    // ANCHOR: print_guess
    println!("你的猜測數字：{}", guess);
    // ANCHOR_END: print_guess
}
// ANCHOR: all
