use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a向量: {:?}", v);
    });

    drop(v); // oh no!

    handle.join().unwrap();
}
