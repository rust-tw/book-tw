struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("釋放 CustomSmartPointer 的資料 `{}`！", self.data);
    }
}

// ANCHOR: here
fn main() {
    let c = CustomSmartPointer {
        data: String::from("某些資料"),
    };
    println!("CustomSmartPointer 建立完畢。");
    c.drop();
    println!("CustomSmartPointer 在 main 結束前就被釋放了。");
}
// ANCHOR_END: here
