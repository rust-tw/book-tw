fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("藍隊"), 10);
    scores.insert(String::from("藍隊"), 25);

    println!("{:?}", scores);
    // ANCHOR_END: here
}
