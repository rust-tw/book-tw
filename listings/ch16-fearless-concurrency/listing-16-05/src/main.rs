use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("這是個向量：{:?}", v);
    });

    handle.join().unwrap();
}
