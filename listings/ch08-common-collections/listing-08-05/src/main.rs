fn main() {
    // ANCHOR: here
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("第三個元素為 {}", third);

    match v.get(2) {
        Some(third) => println!("第三個元素為 {}", third),
        None => println!("第三個元素並不存在。"),
    }
    // ANCHOR_END: here
}
