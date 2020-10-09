// ANCHOR: here
pub fn greeting(name: &str) -> String {
    String::from("哈囉！")
}
// ANCHOR_END: here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("卡爾");
        assert!(result.contains("卡爾"));
    }
}

fn main() {}
