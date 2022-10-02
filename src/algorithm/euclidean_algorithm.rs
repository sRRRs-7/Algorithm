
pub fn main() {
    let (mut x, mut y) = (100, 65);

    let res = euclidean_algorithm(&mut x, &mut y);
    println!("euclidean algorithm: {}", res);

    let res = euclidean_algorithm_while(&mut x, &mut y);
    println!("euclidean algorithm: {}", res);
}


pub fn euclidean_algorithm(x: &mut i32, y: &mut i32) -> i32 {
    loop {
        let t = *x % *y;
        if t == 0 { break };
        *x = *y;
        *y = t;
    };

    *y
}

pub fn euclidean_algorithm_while(x: &mut i32, y: &mut i32) -> i32 {
    while *y != 0 {
        let t = *x % *y;
        *x = *y;
        *y = t;
    };

    *x
}