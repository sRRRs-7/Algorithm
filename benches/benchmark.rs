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

fn bm6(c: &mut Criterion) {
    c.bench_function(
        "reverse_integer", |b| b.iter(
            || algorithm::reverse_int::reverse_integer(-186483673)));
}

fn bm7(c: &mut Criterion) {
    c.bench_function(
        "reverse_integer", |b| b.iter(
            || algorithm::reverse_int::reverse(-186483673)));
}

fn bm8(c: &mut Criterion) {
    c.bench_function(
        "wildcard_match", |b| b.iter(
            || algorithm::wildcard_match::wildcard_matching("hsfduaifh", "*")));
}

fn bm9(c: &mut Criterion) {
    c.bench_function(
        "is_match", |b| b.iter(
            || algorithm::wildcard_match::is_match("hsfduaifh", "*")));
}

fn bm10(c: &mut Criterion) {
    c.bench_function(
        "median_array", |b| b.iter(
            || algorithm::median_array::median(vec!(1, 2, 3, 4, 5), vec!(7, 9, 11, 13))));
}

fn bm11(c: &mut Criterion) {
    c.bench_function(
        "median", |b| b.iter(
            || algorithm::median_array::find_median_sorted_arrays(vec!(1, 2, 3, 4, 5), vec!(7, 9, 11, 13))));
}

fn bm12(c: &mut Criterion) {
    c.bench_function(
        "elevation_map", |b| b.iter(
            || algorithm::elevation_map::elevation_map(vec!(0,1,0,2,1,0,1,3,2,1,2,1))));
}

fn bm13(c: &mut Criterion) {
    c.bench_function(
        "trap", |b| b.iter(
            || algorithm::elevation_map::trap(vec!(0,1,0,2,1,0,1,3,2,1,2,1))));
}


criterion_group!(benches,
    bm1,
    bm2,
    bm3,
    bm4,
    bm5,
    bm6,
    bm7,
    bm8,
    bm9,
    bm10,
    bm11,
    bm12,
    bm13,
);
criterion_main!(benches);
