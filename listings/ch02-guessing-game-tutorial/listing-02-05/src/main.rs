use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("請猜測一個數字！");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("請輸入你的猜測數字。");

        let mut guess = String::new();

        // ANCHOR: here
        // --snip--

        io::stdin()
            .read_line(&mut guess)
            .expect("讀取行數失敗");

        // ANCHOR: ch19
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // ANCHOR_END: ch19

        println!("你的猜測數字：{}", guess);

        // --snip--
        // ANCHOR_END: here

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
