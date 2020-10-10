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
    match coin {
        Coin::Quarter(state) => println!("此 25 美分所屬的州為 {:?}!", state),
        _ => count += 1,
    }
    // ANCHOR_END: here
}
