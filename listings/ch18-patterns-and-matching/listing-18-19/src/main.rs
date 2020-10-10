fn main() {
    // ANCHOR: here
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("一些數字{}、{}、{}", first, third, fifth)
        }
    }
    // ANCHOR_END: here
}
