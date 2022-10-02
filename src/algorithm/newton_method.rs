
// square root
// y = x^2 - seek = 0
// value on x axis

pub fn main() {
    let seek = 3.0;
    let mut init = 5.0;    // prediction value

    let res = newton_method(seek, &mut init);
    println!("√{} ≈ {}", seek, res);
}

pub fn newton_method(seek: f64, init: &mut f64) -> f64 {
    loop {
        let tangent = *init - f(*init, seek) / df(*init, seek);   //  y2 = y1 - f(x) / f'(x) -> tangent
        let diff = tangent - *init;

        if diff.abs() < 0.0000000001 {
            return *init
        }
        *init = tangent;
    }
}

fn f(init: f64, seek: f64) -> f64 {
    init * init - seek
}

fn df(init: f64, seek: f64) -> f64 {
    init * seek
}