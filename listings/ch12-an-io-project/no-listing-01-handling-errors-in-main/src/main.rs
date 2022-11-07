use std::env;
use std::error::Error;
use std::fs;
use std::process;

// ANCHOR: here
fn main() {
    // --省略--

    // ANCHOR_END: here
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("解析引數時出現問題：{err}");
        process::exit(1);
    });

    // ANCHOR: here
    println!("搜尋 {}", config.query);
    println!("目標檔案為 {}", config.file_path);

    if let Err(e) = run(config) {
        println!("應用程式錯誤：{e}");
        process::exit(1);
    }
}
// ANCHOR_END: here

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("文字內容：\n{contents}");

    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("引數不足");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
