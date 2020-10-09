// ANCHOR: here
use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("緩慢計算中...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
// ANCHOR_END: here

fn main() {}
