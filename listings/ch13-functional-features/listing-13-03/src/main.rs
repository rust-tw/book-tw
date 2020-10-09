use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("緩慢計算中...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

// ANCHOR: here
fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "今天請做 {} 下伏地挺身！",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "然後請做 {} 下仰臥起坐！",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("今天休息！別忘了多喝水！");
        } else {
            println!(
                "今天請慢跑 {} 分鐘！",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}
// ANCHOR_END: here

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}
