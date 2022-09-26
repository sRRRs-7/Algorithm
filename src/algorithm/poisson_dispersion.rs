
// prerequisite
// binominal dispersion
// n = frequency -> infinite
// p = probability -> 1 / infinite
// lambda = np
// formula = (lambda^k) * (e^-lambda) / k!

//poisson dispersion
    // n = average frequency
    // p = average probability
    // k = future probability


const E: f64 = 2.7182818284590452353602874713527;

pub fn main() {
    let res = binominal_dispersion(10.0, 0.5, 10.0);
    println!("binominal: {} %", res);

    // if tell average 2 time per half hour, will tell 6 time per a hour ?
    let res2 = poisson_dispersion(2.0, 2.0, 6.0);
    println!("poisson dispersion: {} %", res2);

}

pub fn poisson_dispersion(n: f64, p: f64, k: f64) -> f64 {
    let lambda = n * p;
    let fac = factorial(k);
    let res = (lambda.powf(k) * E.powf(-lambda)) / fac;

    res * 100.0
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