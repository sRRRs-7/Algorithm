

pub fn main() {
    let n: i128 = 100;
    let res = prime_judge(n);

    if res { println!("{} is prime number", n) }
    else { println!("{} is not prime number", n) };

    let n2 = 1000;
    let res2 = sieve_of_eratosthenes(n2);
    println!("prime list: {:?}", res2);
}

pub fn prime_judge(n: i128) -> bool {
    let mut i = 2;

    while i * i <= n {
        if n % i == 0 { return false };
        i += 1;
    };

    true
}

pub fn sieve_of_eratosthenes(n: i32) -> Vec<usize> {
    let mut list = vec![1; n as usize];
    let sq = (n as f64).sqrt();

    for i in 2..sq as i32 {
        for ii in ( (2 * i)..n ).step_by(i as usize) {
            list[ii as usize] = 0;
        }
    };

    let mut res = Vec::new();

    for i in 0..list.len() {
        if list[i] == 1 { res.push(i) }
    };

    res
}