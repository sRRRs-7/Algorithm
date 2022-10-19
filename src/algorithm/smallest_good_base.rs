
//  line up surplus 1
//  base : 2,3,4,5,6,7,8,9
//  pow : base^0..n + N -> value

pub fn main() {
    let s = "13";

    let res = smallest_good_base(s);
    println!("{}", res);
}


pub fn smallest_good_base(s: &str) -> String {
    let n = s.parse::<i64>().unwrap();
    let longest: u32 = (n as f64).log(2.0) as u32 + 1;

    for m in (2..=longest).rev() {
        let k = (n as f64).powf( 1.0 / (m as f64 - 1.0) ) as i64;
        if check(n, k, m) {
            return k.to_string();
        }
    };

    (n - 1).to_string()
}

fn check(n: i64, k: i64, m: u32) -> bool {
    let k_pow_m = (k as i128).pow(m);
    (k_pow_m - 1) % (k as i128 - 1) == 0 && n as i128 == (k_pow_m - 1) / (k as i128 - 1)
}