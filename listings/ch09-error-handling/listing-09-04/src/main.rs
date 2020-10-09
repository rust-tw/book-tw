use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("開啟檔案時發生問題：{:?}", error),
    };
}
