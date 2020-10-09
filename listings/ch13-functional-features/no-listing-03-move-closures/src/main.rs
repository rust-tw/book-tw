fn main() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    println!("無法在此使用 x：{:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}
