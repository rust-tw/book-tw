fn main() {
    // ANCHOR: here
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("嗨"));

    fn takes_long_type(f: Thunk) {
        // --省略--
    }

    fn returns_long_type() -> Thunk {
        // --省略--
        // ANCHOR_END: here
        Box::new(|| ())
        // ANCHOR: here
    }
    // ANCHOR_END: here
}
