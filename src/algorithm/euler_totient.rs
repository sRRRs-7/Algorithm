
// gcd
// relatively prime to n = 1 and self

pub fn main() {
    let arr = vec!(1,2,3,4,5);

    let res = euler_totient(arr);
    println!("relatively prime count: {}", res);
}

pub fn euler_totient(arr: Vec<i32>) -> i32 {
    let mut res = 0;

    for x in arr {
        for y in 2..x {
            if gcd(y, x) == 1 {
                res += 1
            }
        }
    };

    return res
}

// Euclid method
fn gcd(x: i32, y: i32) -> i32 {
    if x == 0 {
        return y
    };
    return gcd(y % x, x)
}

