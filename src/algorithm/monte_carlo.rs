use std::f64::consts::PI;
use random_number::random;


pub fn main() {
    let square_area = 1.0;

    let (res, circle_area) = monte_carlo(square_area);
    println!("monte carlo: {} ≈ circle_area: {}", res, circle_area);
}


pub fn monte_carlo(square_area: f64) -> (f64, f64) {
    let circle_area = (square_area / 2.0).powf(2.0) * PI;

    let mut all_points = 0.0;
    let mut threshold  = 0.0;

    while all_points != 1000.0 {
        let rand: f64 = random();
        if rand <= circle_area{ threshold += 1.0 };
        all_points += 1.0;
    };

    let res = threshold / all_points;

    (res, circle_area)
}