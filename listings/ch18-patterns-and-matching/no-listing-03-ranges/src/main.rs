fn main() {
    // ANCHOR: here
    let x = 5;

    match x {
        1..=5 => println!("一到五"),
        _ => println!("其他"),
    }
    // ANCHOR_END: here
}
