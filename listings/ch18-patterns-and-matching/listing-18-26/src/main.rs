fn main() {
    // ANCHOR: here
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("數字 {x} 是偶數"),
        Some(x) => println!("數字 {x} 是基數"),
        None => (),
    }
    // ANCHOR_END: here
}
