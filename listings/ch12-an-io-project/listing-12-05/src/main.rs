use std::env;
use std::fs;

// ANCHOR: here
fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);

    // --省略--
    // ANCHOR_END: here

    println!("搜尋 {}", query);
    println!("目標檔案爲 {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("讀取檔案時發生了錯誤");

    println!("文字內容：\n{}", contents);
    // ANCHOR: here
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
// ANCHOR_END: here
