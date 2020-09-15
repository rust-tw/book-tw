trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("小不點")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("小狗崽")
    }
}

// ANCHOR: here
fn main() {
    println!("幼犬被稱作{}", <Dog as Animal>::baby_name());
}
// ANCHOR_END: here
