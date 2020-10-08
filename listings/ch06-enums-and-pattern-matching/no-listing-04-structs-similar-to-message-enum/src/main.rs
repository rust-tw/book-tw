// ANCHOR: here
struct QuitMessage; // 類單元結構體
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // 元組結構體
struct ChangeColorMessage(i32, i32, i32); // 元組結構體
                                          // ANCHOR_END: here

fn main() {}
