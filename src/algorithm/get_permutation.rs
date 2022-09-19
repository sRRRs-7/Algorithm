
pub fn main() {
    let result = get_permutation(4, 4);
    println!("{}", result);
}

pub fn get_permutation(n: i32, k: i32) -> String {
    let mut payload = (1..=n).collect();
    for _ in 0..k-1 {
        next_permutation(&mut payload);
    }
    let result = payload.iter().map(|a| a.to_string()).collect::<String>();

    result
}

fn next_permutation(arr: &mut Vec<i32>) {
    let mut i = arr.len() - 1;
    while i > 0 {
        i -= 1;
        if arr[i] < arr[i + 1] {
            let mut j = arr.len() - 1;
            while arr[i] >= arr[j] {
                j -= 1;
            }
            arr.swap(i, j);

            let mut low = i + 1;
            let mut high = arr.len() - 1;

            while low < high {
                arr.swap(low, high);
                low += 1;
                high -= 1;
            }
            return;
        }
    }
    arr.sort()
}


