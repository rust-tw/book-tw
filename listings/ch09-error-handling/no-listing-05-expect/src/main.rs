use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("開啟 hello.txt 失敗");
}
