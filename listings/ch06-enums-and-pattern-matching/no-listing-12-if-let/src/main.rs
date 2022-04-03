fn main() {
    // ANCHOR: here
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("最大值被設為 {}", max);
    }
    // ANCHOR_END: here
}
