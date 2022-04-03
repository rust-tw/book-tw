fn main() {
    // ANCHOR: here
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("最大值被設為 {}", max),
        _ => (),
    }
    // ANCHOR_END: here
}
