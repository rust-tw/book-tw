fn main() {
    // ANCHOR: here
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("比五還小：{}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
    // ANCHOR_END: here
}
