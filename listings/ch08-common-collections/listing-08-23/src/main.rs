fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("藍隊"), 10);
    scores.insert(String::from("黃隊"), 50);

    let team_name = String::from("藍隊");
    let score = scores.get(&team_name);
    // ANCHOR_END: here
}
