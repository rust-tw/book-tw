fn main() {
    // ANCHOR: here
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("取得：{}", val);
    }
    // ANCHOR_END: here
}
