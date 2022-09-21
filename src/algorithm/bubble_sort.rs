
pub fn main() {
    let mut arr = vec!(5,6,3,4);

    let result = bubble_sort(&mut arr);
    println!("bubble sort: {:?}", result);


    let mut arr2 = vec!(5,6,3,4);

    let result2 = simple_sort(&mut arr2);
    println!("rust simple sort: {:?}", result2);
}

pub fn bubble_sort(arr: &mut Vec<i32>) -> Vec<i32> {
    for x in 0..arr.len() - 1 {
        for y in 0..arr.len() - x - 1 {
            if arr[y] > arr[y + 1] {
                arr.swap(y, y + 1 );
            }
        }
    };

    arr.to_vec()
}

pub fn simple_sort(arr: &mut Vec<i32>) -> Vec<i32> {
    arr.sort();
    arr.to_vec()
}