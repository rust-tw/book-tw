fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("藍隊"), 10);

    scores.entry(String::from("黃隊")).or_insert(50);
    scores.entry(String::from("藍隊")).or_insert(50);

    println!("{:?}", scores);
    // ANCHOR_END: here
}
