// ANCHOR: here
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive_test() {
    // 會執行一小時的程式碼
}
// ANCHOR_END: here

fn main() {}
