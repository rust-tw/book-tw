// ANCHOR: here
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("最大數字為 {}", result);
    // ANCHOR_END: here
    assert_eq!(result, &100);
    // ANCHOR: here

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("最大字元為 {}", result);
    // ANCHOR_END: here
    assert_eq!(result, &'y');
    // ANCHOR: here
}
// ANCHOR_END: here
