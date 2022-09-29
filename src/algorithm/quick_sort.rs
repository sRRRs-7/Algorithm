
pub fn main() {
    let mut arr = vec!(5,6,3,4);

    quick_sort(&mut arr);
    println!("quick sort: {:?}", arr)
}

pub fn quick_sort(arr: &mut Vec<i32>) {
    let start = 0;
    let end = arr.len() - 1;

    quick_sort_partition(arr, start, end as isize);
}

fn quick_sort_partition(arr: &mut Vec<i32>, start: isize, end: isize) {
    if start < end && end - start >= 1 {
        let pivot = partition(arr, start as isize, end as isize);
        quick_sort_partition(arr, start, pivot - 1);
        quick_sort_partition(arr, pivot + 1, end);
    }
}

fn partition(arr: &mut Vec<i32>, s: isize, e: isize) -> isize {
    let pivot = arr[e as usize];
    let mut i = s - 1;

    for j in s..e {
        if arr[j as usize] <= pivot {
            i = i + 1;
            arr.swap(i as usize, j as usize);
        }
    }
    arr.swap((i + 1) as usize, e as usize);

    i + 1
}