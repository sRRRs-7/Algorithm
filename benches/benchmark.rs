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
            || algorithm::prime_number::prime_judge(101 as i128)));
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

fn bm14(c: &mut Criterion) {
    c.bench_function(
        "bubble_sort", |b| b.iter(
            || algorithm::bubble_sort::bubble_sort(&mut vec!(0,1,0,2,1,0,1,3,2,1,2,1))));
}

fn bm15(c: &mut Criterion) {
    c.bench_function(
        "rust_simple_sort", |b| b.iter(
            || algorithm::bubble_sort::simple_sort(&mut vec!(0,1,0,2,1,0,1,3,2,1,2,1))));
}

fn bm16(c: &mut Criterion) {
    c.bench_function(
        "quick_sort", |b| b.iter(
            || algorithm::quick_sort::quick_sort(&mut vec!(0,1,0,2,1,0,1,3,2,1,2,1))));
}

fn bm17(c: &mut Criterion) {
    c.bench_function(
        "merge_sort", |b| b.iter(
            || algorithm::quick_sort::quick_sort(&mut vec!(0,1,0,2,1,0,1,3,2,1,2,1))));
}

fn bm18(c: &mut Criterion) {
    c.bench_function(
        "radix_convert", |b| b.iter(
            || algorithm::radix_convert::radix_convert(&mut 120, &mut 11, &mut String::from(""))));
}

fn bm19(c: &mut Criterion) {
    c.bench_function(
        "fibonacci", |b| b.iter(
            || algorithm::fibonacci::fibonacci(100)));
}

fn bm20(c: &mut Criterion) {
    c.bench_function(
        "binary_search", |b| b.iter(
            || algorithm::binary_search::binary_search(&mut vec!(4,6,8,45,6,7,3,2,3,6,7,8,56,434,564,1), 56)));
}

fn bm21(c: &mut Criterion) {
    c.bench_function(
        "gaussian_calc", |b| b.iter(
            || algorithm::gaussian_calc::gaussian_calc(1000)));
}

fn bm22(c: &mut Criterion) {
    c.bench_function(
        "collatz_problem", |b| b.iter(
            || algorithm::collatz_problem::collatz_problem(&mut 27)));
}

fn bm23(c: &mut Criterion) {
    c.bench_function(
        "pascal_triangle", |b| b.iter(
            || algorithm::pascal_triangle::pascal_triangle(10)));
}

fn bm24(c: &mut Criterion) {
    c.bench_function(
        "coin_change", |b| b.iter(
            || algorithm::coin_change::coin_change(&mut vec!(1,3,7), &mut 100)));
}

fn bm25(c: &mut Criterion) {
    c.bench_function(
        "poisson_dispersion", |b| b.iter(
            || algorithm::poisson_dispersion::poisson_dispersion(2.0, 2.0, 10.0)));
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
    bm14,
    bm15,
    bm16,
    bm17,
    bm18,
    bm19,
    bm20,
    bm21,
    bm22,
    bm23,
    bm24,
    bm25,
);
criterion_main!(benches);
