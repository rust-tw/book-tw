use std::env;
use std::fs;

// ANCHOR: here
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    // ANCHOR_END: here

    println!("搜尋 {}", config.query);
    println!("目標檔案爲 {}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("讀取檔案時發生了錯誤");

    println!("文字內容：\n{}", contents);
    // ANCHOR: here

    // --省略--
}

// --省略--

// ANCHOR_END: here
struct Config {
    query: String,
    filename: String,
}

// ANCHOR: here
impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}
// ANCHOR_END: here
