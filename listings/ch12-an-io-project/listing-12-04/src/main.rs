// ANCHOR: here
use std::env;
use std::fs;

fn main() {
    // --省略--
    // ANCHOR_END: here
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("搜尋 {}", query);
    // ANCHOR: here
    println!("目標檔案為 {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("應該要能夠讀取檔案");

    println!("文字內容：\n{contents}");
}
// ANCHOR_END: here
