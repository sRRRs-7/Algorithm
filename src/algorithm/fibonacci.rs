
pub fn main() {
    let index: i128 = 100;

    let result = fibonacci(index);
    println!("fibonacci: index {} is {}", index, result);
}

pub fn fibonacci(index: i128) -> i128 {
    let mut i = 0;
    let mut num = 1;
    let mut former = 1;
    let mut latter = 1;

    while i != index {
        num = former + latter;
        former = latter;
        latter = num;

        i += 1;
    };

    num
}