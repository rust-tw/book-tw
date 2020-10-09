use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("解析引數時出現問題：{}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("應用程式錯誤：{}", e);

        process::exit(1);
    }
}
