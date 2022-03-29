#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// ANCHOR: here
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("長方形的寬度不非為零，而是 {}", rect1.width);
    }
}
// ANCHOR_END: here