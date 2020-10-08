fn main() {
    // ANCHOR: here
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() 將字面值加到字串後面

    println!("{}", s); // 這會印出 `hello, world!`
                       // ANCHOR_END: here
}
