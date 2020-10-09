use std::thread;
use std::time::Duration;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

// ANCHOR: here
fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("緩慢計算中...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("今天請做 {} 下伏地挺身！", expensive_result.value(intensity));
        println!("然後請做 {} 下仰臥起坐！", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("今天休息！別忘了多喝水！");
        } else {
            println!(
                "今天請慢跑 {} 分鐘！",
                expensive_result.value(intensity)
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
