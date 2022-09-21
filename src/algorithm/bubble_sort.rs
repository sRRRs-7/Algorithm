
pub fn main() {
    let mut arr = vec!(5,6,3);

    let result = bubble_sort(&mut arr);
    println!("bubble sort: {:?}", result)
}

pub fn bubble_sort(arr: &mut Vec<i32>) -> Vec<i32> {
    for x in 0..arr.len() - 1 {
        for y in 0..arr.len() - x - 1 {
            if arr[y + 1] < arr[y] {
                arr.swap(y, y + 1 )
            }
        }
    };

    arr.to_vec()
}