// ANCHOR: here
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();

    assert!(mid <= len);

    (&mut slice[..mid], &mut slice[mid..])
}
// ANCHOR_END: here

fn main() {
    let mut向量 = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut向量, 3);
}
