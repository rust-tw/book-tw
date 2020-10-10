fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("最長的字串為 {}", result);
}

// ANCHOR: here
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("超長的字串");
    result.as_str()
}
// ANCHOR_END: here
