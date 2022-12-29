use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("建立檔案時發生問題：{:?}", e),
            },
            other_error => {
                panic!("開啟檔案時發生問題：{:?}", other_error);
            }
        },
    };
}
