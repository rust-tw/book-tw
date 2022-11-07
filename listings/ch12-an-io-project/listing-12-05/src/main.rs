use std::env;
use std::fs;

// ANCHOR: here
fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, file_path) = parse_config(&args);

    // --省略--
    // ANCHOR_END: here

    println!("搜尋 {}", query);
    println!("目標檔案為 {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("應該要能夠讀取檔案");

    println!("文字內容：\n{contents}");
    // ANCHOR: here
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}
// ANCHOR_END: here
