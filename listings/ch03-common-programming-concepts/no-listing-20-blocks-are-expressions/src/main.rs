fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("y 的數值為：{}", y);
}
