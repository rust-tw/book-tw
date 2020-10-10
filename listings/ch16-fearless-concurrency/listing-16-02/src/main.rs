use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("數字 {} 出現在產生的執行緒中！", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("數字 {} 出現在主執行緒中！", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
