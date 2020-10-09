struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("叫我以實瑪利。多年以前...");
    let first_sentence = novel.split('.').next().expect("找不到'.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
