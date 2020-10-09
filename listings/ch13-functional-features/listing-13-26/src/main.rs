use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("解析引數時出現問題：{}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("應用程式錯誤：{}", e);

        process::exit(1);
    }
}
