
pub fn main() {
    let n: f64 = 101.0;
    let result = prime_judge(n);

    if result {
        println!("{} is prime number", n);
    } else {
        println!("{} is not prime number", n);
    }
}

pub fn prime_judge(n: f64) -> bool {
    let sq = n.sqrt().round() as i32;

    for i in 2..sq {
        if n as i32 % i == 0 {
            return false;
        }
    };

    return true
}