fn main() {
    // ANCHOR: here
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("是"),
        _ => println!("否"),
    }
    // ANCHOR_END: here
}
