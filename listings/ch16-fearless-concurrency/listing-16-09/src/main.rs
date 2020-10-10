use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("嗨");
        tx.send(val).unwrap();
        println!("val 爲 {}", val);
    });

    let received = rx.recv().unwrap();
    println!("取得：{}", received);
}
