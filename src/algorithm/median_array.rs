
pub fn main() {
    let arr1 = vec!(1, 3);
    let arr2 = vec!(2);

    let result = median(arr1, arr2);
    println!("median: {}", result as f64);

    let arr3 = vec!(1, 3);
    let arr4 = vec!(2);

    let result2 = find_median_sorted_arrays(arr3, arr4);
    println!("median: {}", result2 as f64);
}

pub fn median(arr1: Vec<i32>, arr2: Vec<i32>) -> f64 {
    let mut arr = [arr1, arr2].concat();
    arr.sort();

    let median: f64;
    if arr.len() % 2 == 1 {
        median = arr[arr.len() / 2] as f64;
    } else {
        let former = arr[arr.len() / 2 - 1] as f64;
        let latter = arr[arr.len() / 2] as f64;
        median = ((former + latter) / 2 as f64) as f64;
    }

    median
}


pub fn find_median_sorted_arrays(arr1: Vec<i32>, arr2: Vec<i32>) -> f64 {
    let mut merged = vec![];
    let (mut i, mut j) = (0, 0);

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            merged.push(arr1[i]);
            i += 1;
        } else {
            merged.push(arr2[j]);
            j += 1;
        }
    }

    while i < arr1.len() {
        merged.push(arr1[i]);
        i += 1;
    }

    while j < arr2.len() {
        merged.push(arr2[j]);
        j += 1;
    }

    let mid = merged.len() / 2;
    match merged.len() % 2 {
        0 => return ((merged[mid-1] as f64 + merged[mid] as f64) / 2.0) as f64,
        _ => return merged[mid] as f64
    }
}