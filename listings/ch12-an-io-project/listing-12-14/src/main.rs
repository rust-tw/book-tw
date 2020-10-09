// ANCHOR: here
use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // --省略--
    // ANCHOR_END: here
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("解析引數時出現問題：{}", err);
        process::exit(1);
    });

    println!("搜尋 {}", config.query);
    println!("目標檔案爲 {}", config.filename);

    // ANCHOR: here
    if let Err(e) = minigrep::run(config) {
        // --省略--
        // ANCHOR_END: here
        println!("應用程式錯誤：{}", e);

        process::exit(1);
        // ANCHOR: here
    }
}
// ANCHOR_END: here
