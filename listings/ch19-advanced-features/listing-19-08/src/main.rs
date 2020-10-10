extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("依據 C 所判斷 -3 的絕對值爲：{}", abs(-3));
    }
}
