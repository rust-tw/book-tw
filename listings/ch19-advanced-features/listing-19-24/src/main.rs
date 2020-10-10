fn main() {
    // ANCHOR: here
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("嗨"));

    fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
        // --省略--
    }

    fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
        // --省略--
        // ANCHOR_END: here
        Box::new(|| ())
        // ANCHOR: here
    }
    // ANCHOR_END: here
}
