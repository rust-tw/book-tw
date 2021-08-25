use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("請輸入陣列索引");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("讀行失敗");

    let index: usize = index
        .trim()
        .parse()
        .expect("輸入的索引並非數字");

    let element = a[index];

    println!(
        "索引 {} 元素的數值爲：{}",
        index, element
    );
}