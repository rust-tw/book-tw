fn main() {
    // ANCHOR: here
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意到 s1 被移動因此無法再被使用
                       // ANCHOR_END: here
}
