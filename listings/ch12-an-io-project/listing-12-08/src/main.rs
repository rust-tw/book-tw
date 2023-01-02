use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("搜尋 {}", config.query);
    println!("目標檔案為 {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("應該要能夠讀取檔案");

    println!("文字內容：\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // ANCHOR: here
    // --省略--
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("引數不足");
        }
        // --省略--
        // ANCHOR_END: here

        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}
