use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("解析引數時出現問題：{err}");
        process::exit(1);
    });

    println!("搜尋 {}", config.query);
    println!("目標檔案為 {}", config.file_path);

    if let Err(e) = run(config) {
        println!("應用程式錯誤：{e}");
        process::exit(1);
    }
}
