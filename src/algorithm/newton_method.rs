
// sqrt2

pub fn main() {
    let sqrt = 3.0;
    let n = 5.0;

    let res = newton_method(sqrt, n);
    println!("√{} ≈ {}", sqrt, res);
}

pub fn newton_method(sqrt: f64, n: f64) -> f64 {
    let mut x = n;
    loop {
        let x2 = x - (x * x - sqrt) / (x * sqrt);
        let num = x2 - x;

        if num.abs() < 0.0001 {
            return x
        }
        x = x2;
    }
}