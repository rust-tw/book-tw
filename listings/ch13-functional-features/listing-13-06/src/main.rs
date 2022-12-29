use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("呼叫閉包前：{:?}", list);

    thread::spawn(move || println!("來自執行緒：{:?}", list))
        .join()
        .unwrap();
}