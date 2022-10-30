
pub fn main() {
    let arr = vec![3];

    let res = jump_game(arr);
    println!("jump game: {}", res);
}

pub fn jump_game(arr: Vec<i32>) -> i32 {
    let first = arr[0];
    let last = arr[arr.len()-1];
    let length = arr.len();

    if first == arr[length-1] { return 1 }

    let mut first_index = Vec::new();
    let mut last_index = Vec::new();

    for i in 1..length-1 {
        if arr[i] == first { first_index.push(i) }
        else if arr[i] == last { last_index.push(i) };
    }

    if last_index.len() == 0 {
        return (length - first_index[first_index.len()-1]) as i32
    }

    if let Some(i) = first_index.pop() {
        if i == length-2 { return 2 }
    }

    3
}