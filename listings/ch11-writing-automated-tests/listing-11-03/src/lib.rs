// ANCHOR: here
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("此測試會失敗");
    }
}
// ANCHOR_END: here

fn main() {}
