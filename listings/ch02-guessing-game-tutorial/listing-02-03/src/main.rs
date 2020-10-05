// ANCHOR: all
use std::io;
// ANCHOR: ch07-04
use rand::Rng;

fn main() {
    // ANCHOR_END: ch07-04
    println!("請猜測一個數字！");

    // ANCHOR: ch07-04
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // ANCHOR_END: ch07-04

    println!("祕密數字爲：{}", secret_number);

    println!("請輸入你的猜測數字。");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("讀取行數失敗");

    println!("你的猜測數字：{}", guess);
    // ANCHOR: ch07-04
}
// ANCHOR_END: ch07-04
// ANCHOR_END: all
