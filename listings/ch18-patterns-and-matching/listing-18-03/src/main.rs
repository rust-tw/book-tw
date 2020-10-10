fn main() {
    // ANCHOR: here
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} 位於索引 {}", value, index);
    }
    // ANCHOR_END: here
}
