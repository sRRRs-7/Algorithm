
pub fn main() {
    let depth = 10;

    pascal_triangle(depth);
}

pub fn pascal_triangle(d: i32) {
    for i in 0..d {
        let mut c = 1;

        for _ in 1..2 * (d-1-i) + 1 {
            // print!(" ")
        }

        for k in 0..i+1 {
            // print!("{:2} ", c);
            c = c * (i-k) / (k+1);
        }
        // println!();
    }
}