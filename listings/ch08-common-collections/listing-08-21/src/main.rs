fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let teams = vec![String::from("藍隊"), String::from("黃隊")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
    // ANCHOR_END: here
}
