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

fn main() {
    let coin = Coin::Penny;
    // ANCHOR: here
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("此 25 美分所屬的州爲 {:?}!", state);
    } else {
        count += 1;
    }
    // ANCHOR_END: here
}
