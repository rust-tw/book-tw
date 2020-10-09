use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("搜尋 {}", config.query);
    println!("目標檔案爲 {}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("讀取檔案時發生了錯誤");

    println!("文字內容：\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

// ANCHOR: here
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("引數不足");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
// ANCHOR_END: here
