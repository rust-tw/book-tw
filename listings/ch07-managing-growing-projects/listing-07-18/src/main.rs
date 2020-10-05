use rand::Rng;
// ANCHOR: here
// --snip--
use std::{cmp::Ordering, io};
// --snip--
// ANCHOR_END: here

fn main() {
    println!("請猜測一個數字！");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("請輸入你的猜測數字。");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("讀取行數失敗");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("你的猜測數字：{}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
