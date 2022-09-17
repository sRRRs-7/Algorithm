
pub fn main() {
    let num = -1274395702;

    let result = reverse_integer(num);
    println!("{}", result);

    let result2 = reverse(num);
    println!("{}", result2);
}

pub fn reverse_integer(num: i32) -> i32 {
    let mut result = 0;
    let mut sign = 1;

    if num < 0 {
        sign = -1;
    };

    let mut x = num.abs();
    while x > 0 {
        let v = result * 10 + x % 10;
        // check overflow
        if (v - x % 10) / 10 != result {
            return 0
        }
        result = v;
        x = x / 10;
    }

    result * sign
}

// slow
pub fn reverse(x: i32) -> i32 {
    fn helper(mut n: i32) -> Option<i32> {
        let mut res = 0i32;
        while n.abs() != 0 {
            res = res.checked_mul(10)?.checked_add(n % 10)?;
            n /= 10;
        }
        Some(res)
    }
    helper(x).unwrap_or_default()
}