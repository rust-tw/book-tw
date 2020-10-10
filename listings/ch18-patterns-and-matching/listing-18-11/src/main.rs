fn main() {
    // ANCHOR: here
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("取得 50"),
        Some(y) => println!("配對成功，y = {:?}", y),
        _ => println!("預設情形，x = {:?}", x),
    }

    println!("最後結果：x = {:?}, y = {:?}", x, y);
    // ANCHOR_END: here
}
