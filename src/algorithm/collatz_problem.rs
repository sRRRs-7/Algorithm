
pub fn main() {
    let mut start = 27;
    let result = collatz_problem(&mut start);
    let max = result.iter().max().unwrap();
    println!("collatz {:?}", result);
    println!("collatz max: {}", max);
}

pub fn collatz_problem(n: &mut i32) -> Vec<i32> {
    let mut arr = Vec::new();
    while *n != 1 {
        if *n % 2 == 0 {
            *n = *n / 2;
            arr.push(*n);
        } else if *n % 2 == 1 {
            *n = 3 * *n + 1;
            arr.push(*n);
        }
    };

    arr
}