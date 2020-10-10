fn main() {
    // ANCHOR: here
    let x = 1;

    match x {
        1 | 2 => println!("一或二"),
        3 => println!("三"),
        _ => println!("任意數字"),
    }
    // ANCHOR_END: here
}
