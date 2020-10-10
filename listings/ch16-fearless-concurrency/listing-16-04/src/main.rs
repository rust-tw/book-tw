use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("這是個向量：{:?}", v);
    });

    drop(v); // 喔不！

    handle.join().unwrap();
}
