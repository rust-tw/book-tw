struct Point {
    x: i32,
    y: i32,
}

// ANCHOR: here
fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("位於 x 軸的 {}", x),
        Point { x: 0, y } => println!("位於 y 軸的 {}", y),
        Point { x, y } => println!("不在任一軸：({}, {})", x, y),
    }
}
// ANCHOR_END: here
