
pub fn main() {
    let max_num = 1000;
    let result = gaussian_calc(max_num);
    println!("gaussian {}", result);
}

// 1001 * 500
pub fn gaussian_calc(n: i32) -> i32 {
    let sum = (1 + n) * (n / 2);
    sum
}