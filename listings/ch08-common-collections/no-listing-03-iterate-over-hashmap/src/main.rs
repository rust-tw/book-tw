fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("藍隊"), 10);
    scores.insert(String::from("黃隊"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    // ANCHOR_END: here
}
