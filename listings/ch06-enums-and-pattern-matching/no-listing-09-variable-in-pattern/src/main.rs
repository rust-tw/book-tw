#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --省略--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// ANCHOR: here
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("此 25 美分所屬的州爲 {:?}!", state);
            25
        }
    }
}
// ANCHOR_END: here

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
}
