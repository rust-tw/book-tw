struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("釋放 CustomSmartPointer 的資料 `{}`！", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("我的東東"),
    };
    let d = CustomSmartPointer {
        data: String::from("其他東東"),
    };
    println!("CustomSmartPointers 建立完畢。");
}
