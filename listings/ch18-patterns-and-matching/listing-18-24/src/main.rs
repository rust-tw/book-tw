fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("一些數字：{}、{}", first, last);
        }
    }
}
