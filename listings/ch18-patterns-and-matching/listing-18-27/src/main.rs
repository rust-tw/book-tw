fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("取得 50"),
        Some(n) if n == y => println!("配對成功，n = {}", n),
        _ => println!("預設情形，x = {:?}", x),
    }

    println!("最後結果：x = {:?}, y = {}", x, y);
}
