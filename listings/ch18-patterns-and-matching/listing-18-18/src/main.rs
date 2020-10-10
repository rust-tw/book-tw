fn main() {
    // ANCHOR: here
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("無法覆寫已經存在的自訂數值");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("設定為 {:?}", setting_value);
    // ANCHOR_END: here
}
