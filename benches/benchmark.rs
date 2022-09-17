extern crate criterion;
use criterion::{criterion_group, criterion_main, Criterion};

use lib::algorithm;

fn bm1(c: &mut Criterion) {
    c.bench_function(
        "roma_numeric", |b| b.iter(
            || algorithm::roma_numeric::roman_to_int(String::from("MCMXCIV"))));
}

fn bm2(c: &mut Criterion) {
    c.bench_function(
        "prime judge", |b| b.iter(
            || algorithm::prime_number::prime_judge(101 as f64)));
}

fn bm3(c: &mut Criterion) {
    c.bench_function(
        "greater common divide", |b| b.iter(
            || algorithm::lcm_gcd::gcd(&32, &64)));
}

fn bm4(c: &mut Criterion) {
    c.bench_function(
        "least common multiple", |b| b.iter(
            || algorithm::lcm_gcd::lcm(32, 64)));
}

fn bm5(c: &mut Criterion) {
    c.bench_function(
        "n_root", |b| b.iter(
            || algorithm::n_root::n_root(65536.0)));
}


criterion_group!(benches,
    bm1,
    bm2,
    bm3,
    bm4,
    bm5,
);
criterion_main!(benches);
