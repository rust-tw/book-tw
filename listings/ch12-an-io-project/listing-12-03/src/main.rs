use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("搜尋 {}", query);
    println!("目標檔案為 {}", file_path);
}
