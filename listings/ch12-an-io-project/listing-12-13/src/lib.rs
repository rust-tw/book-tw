// ANCHOR: here
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // --省略--
        // ANCHOR_END: here
        if args.len() < 3 {
            return Err("引數不足");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
        // ANCHOR: here
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // --省略--
    // ANCHOR_END: here
    let contents = fs::read_to_string(config.file_path)?;

    println!("文字內容：\n{contents}");

    Ok(())
    // ANCHOR: here
}
// ANCHOR_END: here
