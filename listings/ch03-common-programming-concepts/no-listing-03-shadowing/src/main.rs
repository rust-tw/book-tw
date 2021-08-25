fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("x 在內部範圍的數值為：{}", x);
    }

    println!("x 的數值為：{}", x);
}
