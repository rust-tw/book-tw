mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("桃子"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // 點夏季早餐並選擇黑麥麵包
    let mut meal = back_of_house::Breakfast::summer("黑麥");
    // 我們想改成全麥麵包
    meal.toast = String::from("全麥");
    println!("我想要{}麵包，謝謝", meal.toast);

    // 接下來這行取消註解的話，我們就無法編譯通過
    // 我們無法擅自更改餐點搭配的季節水果
    // meal.seasonal_fruit = String::from("藍莓");
}
