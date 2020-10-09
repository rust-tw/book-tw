enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// ANCHOR: here
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("幸運幣！");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
// ANCHOR_END: here

fn main() {}
