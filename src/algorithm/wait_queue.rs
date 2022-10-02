
use crate::algorithm::poisson_dispersion::poisson_dispersion;

// p : utilization (%) -> lambda / mu
// lambda : poisson dispersion (frequency / hour)
// mu : exponential dispersion (times / hour)
// ts : average process time (minutes)
// w : p / (1-p) * ts (minutes)

const E: f64 = 2.7182818284590452353602874713527;

pub fn main() {
    let lambda = poisson_dispersion(100.0, 0.5, 80.0);
    let mu = exponential_dispersion(10.0, 5.0);
    let ts = 10.0;

    let (wq, p) = wait_queue(lambda, mu, ts);
    println!("mu: {}", mu);
    println!("utilization: {}", p);
    println!("wait for {} minutes", (wq * 3600.0) as i32);
}

pub fn wait_queue(lambda: f64, mu: f64, ts: f64) -> (f64, f64) {
    let p = lambda / mu;
    let w = (p / (1.0 - p)) * ts;
    (w, p)
}


// mu : average ratio
// occur : expect occurring

pub fn exponential_dispersion(mu: f64, occur: f64) -> f64 {
    let x = E.powf(-(occur / mu));
    let expo = (mu.powf(x)) / mu;
    expo
}
