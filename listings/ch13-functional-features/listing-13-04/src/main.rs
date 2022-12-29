fn main() {
    let list = vec![1, 2, 3];
    println!("定義閉包前：{:?}", list);

    let only_borrows = || println!("來自閉包：{:?}", list);

    println!("呼叫閉包前：{:?}", list);
    only_borrows();
    println!("呼叫閉包後：{:?}", list);
}
