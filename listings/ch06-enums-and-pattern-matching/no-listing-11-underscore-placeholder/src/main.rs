fn main() {
    // ANCHOR: here
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("一"),
        3 => println!("三"),
        5 => println!("五"),
        7 => println!("七"),
        _ => (),
    }
    // ANCHOR_END: here
}
