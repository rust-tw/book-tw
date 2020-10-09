use std::env;
use std::error::Error;
use std::fs;
use std::process;

// ANCHOR: here
fn main() {
    // --省略--

    // ANCHOR_END: here
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("解析引數時出現問題：{}", err);
        process::exit(1);
    });

    // ANCHOR: here
    println!("搜尋 {}", config.query);
    println!("目標檔案爲 {}", config.filename);

    if let Err(e) = run(config) {
        println!("應用程式錯誤：{}", e);

        process::exit(1);
    }
}
// ANCHOR_END: here

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("文字內容：\n{}", contents);

    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

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
