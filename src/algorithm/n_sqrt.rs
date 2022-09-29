
pub fn main() {
    let num = 65538.0;
    let (result ,rest) = n_root(num);

    println!("{} includes 2's: {}, rest: {}", num, result, rest);
}

pub fn n_root(num: f64) -> (i32, i32) {
    let mut arr = Vec::new();
    let mut state = num;

    while state as i32 / 2 != 1 {
        let sq = state / 2.0;
        arr.push(sq);
        state = sq;
    }

    let result = arr.len() + 1;
    let rest = num as i32 % (2 * result as i32);

    (result as i32, rest)
}
