fn main() {
    let rect1 = (30, 50);

    println!(
        "長方形的面積為 {} 平方像素。",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
