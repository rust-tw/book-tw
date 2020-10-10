fn main() {
    // ANCHOR: here
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("一"),
        3 => println!("三"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
    // ANCHOR_END: here
}
