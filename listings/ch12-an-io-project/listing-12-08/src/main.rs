use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("搜尋 {}", config.query);
    println!("目標檔案為 {}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("讀取檔案時發生了錯誤");

    println!("文字內容：\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
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
        let filename = args[2].clone();

        Config { query, filename }
    }
}
