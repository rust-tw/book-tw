fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("使用你最喜歡的顏色{}作爲背景", color);
    } else if is_tuesday {
        println!("星期二就用綠色！");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("使用紫色作爲背景顏色");
        } else {
            println!("使用橘色作爲背景顏色");
        }
    } else {
        println!("使用藍色作爲背景顏色");
    }
}
