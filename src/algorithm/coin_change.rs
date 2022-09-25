
use crate::algorithm::quick_sort::quick_sort;

pub fn main() {
    let mut coins = vec!(2, 5);
    let mut amount = 12;

    let res = coin_change(&mut coins, &mut amount).expect("cannot change coins");
    println!("{}", res);
}

pub fn coin_change(coins: &mut Vec<i32>, amount: &mut i32) -> Option<i32> {
    let mut coin = 0;

    quick_sort(coins);

    for x in coins.iter().rev() {
        coin = coin + *amount / *x;
        *amount = *amount % *x;

        if *amount % *x == 0 { return Some(coin) };
    };

    None
}