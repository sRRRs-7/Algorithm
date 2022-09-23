
pub fn main() {
    let n: i128 = 100;
    let result = prime_judge(n);

    if result {
        println!("{} is prime number", n);
    } else {
        println!("{} is not prime number", n);
    }
}

pub fn prime_judge(n: i128) -> bool {
    let mut i = 2;

    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i = i + i;
    };

    return true
}