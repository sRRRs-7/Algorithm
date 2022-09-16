
pub fn main() {
    let (a, b) = (16, 64);

    let gcd_num = gcd(&a, &b);
    println!("greater common divide: {}", gcd_num);

    let lcm_num = lcm(a, b);
    println!("least common multiple: {}", lcm_num);

}

pub fn gcd(a: &i32, b: &i32) -> i32 {
    let mut m = a % b;

    if m == 0 {
        return *b
    } else if *a != 0 || *b != 0 {
        m = gcd(b, &(*a % *b));
    } else if *a == 0 {
        return *b
    } else if *b == 0 {
        return *a
    };

    m
 }


pub fn lcm(a: i32, b: i32) -> i32 {
    let gcd_num = gcd(&a, &b);
    let lcm_num = a / gcd_num * b;
    lcm_num
}

