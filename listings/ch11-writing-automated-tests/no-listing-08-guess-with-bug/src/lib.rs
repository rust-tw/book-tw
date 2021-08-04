pub struct Guess {
    value: i32,
}

// ANCHOR: here
// --省略--
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("猜測數字必須介於 1 到 100 之間，你輸入的是 {}。", value);
        }

        Guess { value }
    }
}
// ANCHOR_END: here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
