fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("藍隊");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name 和 field_value 在這之後就不能使用了，你可以試著使用它們並看看編譯器回傳什麼錯誤
    // ANCHOR_END: here
}
