
pub fn main() {
    recursive_func(3);

    let res = factorial_recursive(5.0);
    println!("factorial recursive: {}", res);
}

fn recursive_func(n: i32) {
    if n > 0 {
        recursive_func(n - 1);
        recursive_func(n - 1);

        // println!("{}", n);
    }
}

// stack
// 1: [3,2,1]
// 2: [3,2,1]
// 112, 1123

// 1: [4,3,2,1]
// 2: [4,3,2,1]
// 112, 1123, 112, 11234


// 1 : [1]
// 2 : [112]
// 3 : [112,112,3]
// 4 : [112,1123,112,1123,4]
// 5 : [112,1123,112,11234,112,1123,112,11234,5]


fn factorial_recursive(n: f64) -> f64 {
    let mut res = 1.0 ;
    if n > 1.0 { res = n * factorial_recursive(n - 1.0); }
    res
 }