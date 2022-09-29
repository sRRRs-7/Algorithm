
use crate::algorithm::merge_sort;

pub fn main() {
    // binary_search
    let mut arr = vec!(4,6,8,45,6,7,3,2,3,6,7,8,56,434,564,1);
    let search = 564;

    let b = binary_search(&mut arr, search);
    if b {
        println!("Found {}", search)
    } else {
        println!("Not found {}", search)
    };

    // search
    let mut arr2 = vec!(4,6,8,45,6,7,3,2,3,6,7,8,56,434,564,1);
    let search2 = 6;

    let b = search_number(&mut arr2, search2);
    if b {
        println!("Found {}", search2)
    } else {
        println!("Not found {}", search2)
    };

    // rust simple binary search
    let mut arr3 = vec!(4,6,8,45,6,7,3,2,3,6,7,8,56,434,564,1);
    let search3 = 6;
    simple_binary_search(&mut arr3, search3);

}


pub fn binary_search(arr: &mut Vec<i32>, search: i32) -> bool {
    merge_sort::merge_sort(arr);

    let mut left: i32 = -1;
    let mut right = arr.len();

    while right as i32 - left > 1 {
        let mid = left + (right as i32 - left) / 2;
        if arr[mid as usize] > search {
            right = mid as usize
        } else if arr[mid as usize] < search  {
            left = mid
        } else if arr[mid as usize] == search {
            return true
        }
    };

    false
}


fn simple_binary_search(arr: &mut Vec<i32>, search: i32) {
    let value = arr.binary_search_by(|v| {
        v.partial_cmp(&search).expect("compare error")
    });

    match value {
        Ok(x) => println!("found {}", x),
        Err(x) => println!("Not found {}", x),
    }
}

fn search_number(arr: &mut Vec<i32>, search: i32) -> bool {
    arr.sort();
    arr.contains(&search)
}