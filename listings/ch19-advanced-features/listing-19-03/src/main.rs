fn main() {
    // ANCHOR: here
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 為：{}", *r1);
        println!("r2 為：{}", *r2);
    }
    // ANCHOR_END: here
}
