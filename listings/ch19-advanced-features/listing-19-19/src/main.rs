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
        String::from("小狗狗")
    }
}

fn main() {
    println!("幼犬被稱作{}", Dog::baby_name());
}
