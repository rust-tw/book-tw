use std::env;
use std::process;

use minigrep::Config;

// ANCHOR: ch13
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("解析引數時出現問題：{}", err);
        process::exit(1);
    });

    // --省略--
    // ANCHOR_END: ch13

    if let Err(e) = minigrep::run(config) {
        eprintln!("應用程式錯誤：{}", e);

        process::exit(1);
    }
    // ANCHOR: ch13
}
// ANCHOR_END: ch13
