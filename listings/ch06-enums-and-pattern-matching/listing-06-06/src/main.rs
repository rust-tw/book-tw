fn main() {
    // ANCHOR: here
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("三"),
        _ => (),
    }
    // ANCHOR_END: here
}
