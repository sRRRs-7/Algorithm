
pub fn main() {
    let mut arr = vec!(5,6,3,4);

    merge_sort(&mut arr);
    println!("merge sort: {:?}", arr);
}

pub fn merge_sort(arr: &mut Vec<i32>) {
    if arr.len() > 1 {
        let mid = arr.len() / 2;
        let mut left = arr[..mid].to_vec();
        let mut right = arr[mid..].to_vec();

         // stack buffer
        merge_sort(&mut left);
        merge_sort(&mut right);

        let (mut l, mut r, mut k) = (0, 0, 0);
        while l < left.len() && r < right.len() {
            if left[l] <= right[r] {
                arr[k] = left[l];
                l += 1;
            } else {
                arr[k] = right[r];
                r += 1;
            }
            k += 1;
        };

        while l < left.len() {
            arr[k] = left[l];
            l += 1;
            k += 1;
        };

        while r < right.len() {
            arr[k] = right[r];
            r += 1;
            k += 1;
        };
    }
}