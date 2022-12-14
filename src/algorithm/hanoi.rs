
// hanoi rules
// 3 towers (start, goal, work)
// disks (n)
// bigger disk is not on the small disk

pub fn main() {
    let (n, left, center, right) = (3, "L", "C", "R");
    let mut process = 0;
    hanoi(n, left, center, right, &mut process);
    println!("process: {} times", process);

    hanoi_bench(n, left, center, right);
}

pub fn hanoi(n: i32, left: &str, center: &str, right: &str, p: &mut i32) {
    if n > 0 {
        hanoi(n - 1, left,right, center, p);
        println!("{}th disk {} to {}", n, left, center);
        *p = *p + 1;
        hanoi(n - 1, right, center, left, p);
    }
}

pub fn hanoi_bench(n: i32, left: &str, center: &str, right: &str) {
    if n > 0 {
        hanoi_bench(n - 1, left,right, center);
        hanoi_bench(n - 1, right, center, left);
    }
}

