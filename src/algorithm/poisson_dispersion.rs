
// prerequisite
// binominal dispersion
// n = frequency -> infinite
// p = probability -> 1 / infinite
// lambda = np

// formula = (lambda^k) * (e^-lambda) / k!


const e: f64 = 2.7182818284590452353602874713527;

pub fn main() {
    binominal_dispersion(10.0, 0.5, 1.0);
}

pub fn poisson_dispersion() {

}


fn factorial(n: f64) -> f64 {
    let mut res = 1.0;
    for x in 2..=n as i64 {
        res = res * x as f64
    }
    res
 }


 fn binominal_dispersion(n: f64, p: f64, k: f64) -> f64 {
    let c = combination(n, k);
    let rp = 1.0 - p;

    let mut res = c * (p.powf(k)) * (rp.powf(n-k));

    if res > 100.0 {
        res = 99.999
    } else if res < 0.00000001 {
        res = 0.00001
    };

    println!("{} %", res);

    res
 }


 fn combination(n: f64, k: f64) -> f64 {
    let mut p = 1.0;
    let mut c = 1.0;

    // permutation
    for x in k as i32 + 1..=n as i32 {
        p = p * x as f64;
    };

    // combination
    for x in 2..=k as i64 {
        c = c * x as f64
    };

    p / c
 }