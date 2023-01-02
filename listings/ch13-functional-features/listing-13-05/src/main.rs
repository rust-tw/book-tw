fn main() {
    let mut list = vec![1, 2, 3];
    println!("呼叫閉包前{:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("呼叫閉包後：{:?}", list);
}
