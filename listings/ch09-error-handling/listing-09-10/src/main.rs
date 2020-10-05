use rand::Rng;
use std::cmp::Ordering;
use std::io;

// ANCHOR: here
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
// ANCHOR_END: here

fn main() {
    println!("請猜測一個數字！");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("請輸入你的猜測數字。");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("讀取行數失敗");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let guess = Guess::new(guess);

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
