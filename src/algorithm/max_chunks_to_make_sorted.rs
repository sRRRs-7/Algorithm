
pub fn main() {
    let arr = vec![5,4,3,2,1];

    let res = max_chunks_to_make_sorted(arr);
    println!("{}", res);
}

pub fn max_chunks_to_make_sorted(arr: Vec<i32>) -> i32 {
    let mut array = arr.clone();
    array.reverse();

    let mut chunk = 0;

    for i in 1..array.len() - 1 {
        for ii in 1..array.len() - 1 {
            if array[i-1] > array[ii] {
                array.swap(i-1, ii);
                chunk += 1
            }
        }
    };

    chunk
}