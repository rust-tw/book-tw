struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("無法找到 '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}