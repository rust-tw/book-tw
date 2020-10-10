// ANCHOR: here
use gui::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // 實際畫出選擇框的程式碼
    }
}
// ANCHOR_END: here

fn main() {}
