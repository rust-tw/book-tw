pub fn greeting(name: &str) -> String {
    String::from("哈囉！")
}

#[cfg(test)]
mod tests {
    use super::*;

    // ANCHOR: here
    #[test]
    fn greeting_contains_name() {
        let result = greeting("卡爾");
        assert!(
            result.contains("卡爾"),
            "打招呼時並沒有喊出名稱，其數值為 `{}`",
            result
        );
    }
    // ANCHOR_END: here
}
