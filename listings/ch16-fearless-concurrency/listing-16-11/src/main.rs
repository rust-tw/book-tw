use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // ANCHOR: here
    // --省略--

    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("執行緒"),
            String::from("傳來"),
            String::from("的"),
            String::from("嗨"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("更多"),
            String::from("給你"),
            String::from("的"),
            String::from("訊息"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("取得：{}", received);
    }

    // --省略--
    // ANCHOR_END: here
}
