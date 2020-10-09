fn main() {
    // ANCHOR: here
    let s = Some(String::from("哈囉！"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);
    // ANCHOR_END: here
}
