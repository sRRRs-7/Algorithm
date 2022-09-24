
pub fn main() {
    let arr = vec!(2,4,7,3);
    let search = 10;

    let result = two_sum_search(arr, search);
    if result {
        println!("Found {}", search)
    } else {
        println!("Not found {}", search)
    }
}

pub fn two_sum_search(arr: Vec<i32>, search: i32) -> bool {
    for x in &arr {
        for (y, _) in arr.iter().enumerate() {
            if *x == arr[y] { continue };

            if *x + arr[y] == search {
                return true
            };
        }
    }

    false
}