// ANCHOR: here
pub fn greeting(name: &str) -> String {
    format!("哈囉{}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("卡爾");
        assert!(result.contains("卡爾"));
    }
}
// ANCHOR_END: here

fn main() {}
