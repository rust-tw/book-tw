use std::env;
use std::fs;

// ANCHOR: here
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("搜尋 {}", config.query);
    println!("目標檔案為 {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("應該要能夠讀取檔案");

    // --省略--
    // ANCHOR_END: here

    println!("文字內容：\n{contents}");
    // ANCHOR: here
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}
// ANCHOR_END: here
