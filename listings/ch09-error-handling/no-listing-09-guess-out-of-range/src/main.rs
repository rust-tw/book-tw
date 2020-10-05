use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("請猜測一個數字！");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // ANCHOR: here
    loop {
        // --snip--

        // ANCHOR_END: here
        println!("請輸入你的猜測數字。");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("讀取行數失敗");

        // ANCHOR: here
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        match guess.cmp(&secret_number) {
            // --snip--
            // ANCHOR_END: here
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        // ANCHOR: here
    }
    // ANCHOR_END: here
}
