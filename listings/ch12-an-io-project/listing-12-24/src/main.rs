use std::env;
use std::process;

use minigrep::Config;

// ANCHOR: here
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("解析引數時出現問題：{}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("應用程式錯誤：{}", e);

        process::exit(1);
    }
}
// ANCHOR_END: here
