// ANCHOR: here
fn main() {
    let string1 = String::from("很長的長字串");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("最長的字串爲 {}", result);
}
// ANCHOR_END: here

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
