// ANCHOR: here
use std::env;
use std::fs;

fn main() {
    // --省略--
    // ANCHOR_END: here
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("搜尋 {}", query);
    // ANCHOR: here
    println!("目標檔案為 {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("讀取檔案時發生了錯誤");

    println!("文字內容：\n{}", contents);
}
// ANCHOR_END: here
