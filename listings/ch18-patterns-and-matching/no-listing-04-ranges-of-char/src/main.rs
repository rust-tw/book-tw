fn main() {
    // ANCHOR: here
    let x = 'c';

    match x {
        'a'..='j' => println!("前半部 ASCII 字母"),
        'k'..='z' => println!("後半部 ASCII 字母"),
        _ => println!("其他"),
    }
    // ANCHOR_END: here
}
