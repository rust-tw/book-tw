struct ImportantExcerpt<'a> {
    part: &'a str,
}

// ANCHOR: 1st
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
// ANCHOR_END: 1st

// ANCHOR: 3rd
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("請注意：{}", announcement);
        self.part
    }
}
// ANCHOR_END: 3rd

fn main() {
    let novel = String::from("叫我以實瑪利。多年以前...");
    let first_sentence = novel.split('.').next().expect("找不到'.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
