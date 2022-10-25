
pub fn main() {
    let n = 1234;

    let res = count_special_integer(n);
    println!("special integer: {}", res);
}

pub fn count_special_integer(n: i32) -> i32 {
    if n % 10 == n { return n  }

    let mut subtract = 0;

    for i in 10..=n {
        let mut arr: Vec<i32> = i.to_string().into_bytes().into_iter().map(|x| x as i32 - 48).collect();
        let digit = arr.len();
        arr.sort();
        arr.dedup();

        if arr.len() < digit { subtract += 1 };
    };

    n - subtract
}