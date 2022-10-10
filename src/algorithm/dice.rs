
// n : umber of dice
// k : dice rolls
// target : seek sum value


pub fn main() {
    let (n, k, target) = (2, 6, 7);

    let res = dice_sum(n, k, target);
    println!("dice sum: {}", res);
}

const MODULO: usize = 1000000007;   // prevent overflow

pub fn dice_sum(n: i32, k: i32, target: i32) -> i32 {
    let (n, k, target) = (n as usize, k as usize, target as usize);
    let (mut dp_prev, mut dp_curr) = (vec![0; target + 1], vec![0; target + 1]);

    dp_prev[0] = 1;

    for _ in 0..n {
        for j in 1..=target {
            for m in 1..=k {
                if m <= j { dp_curr[j] = (dp_curr[j] + dp_prev[j - m]) % MODULO }
                else { continue };
            }
        };

        std::mem::swap(&mut dp_curr, &mut dp_prev);
        dp_curr.fill(0);
    };

    dp_prev[target] as _

}