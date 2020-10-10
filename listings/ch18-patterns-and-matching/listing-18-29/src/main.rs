fn main() {
    // ANCHOR: here
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("id 在此範圍中：{}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("id 在其他範圍中")
        }
        Message::Hello { id } => println!("找到其他 id：{}", id),
    }
    // ANCHOR_END: here
}
