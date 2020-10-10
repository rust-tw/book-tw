fn main() {
    // ANCHOR: here
    let s = Some(String::from("哈囉！"));

    if let Some(_) = s {
        println!("發現字串");
    }

    println!("{:?}", s);
    // ANCHOR_END: here
}
