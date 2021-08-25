// ANCHOR: here
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        // --省略--
        // ANCHOR_END: here
        if args.len() < 3 {
            return Err("引數不足");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
        // ANCHOR: here
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // --省略--
    // ANCHOR_END: here
    let contents = fs::read_to_string(config.filename)?;

    println!("文字內容：\n{}", contents);

    Ok(())
    // ANCHOR: here
}
// ANCHOR_END: here
