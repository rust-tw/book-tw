enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("變更顏色為紅色 {r}、綠色 {g} 與藍色 {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("變更顏色為色相 {h}、飽和度 {s} 與明度 {v}");
        }
        _ => (),
    }
}
