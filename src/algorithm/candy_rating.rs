
pub fn main() {
    let ratings = vec![1,2,2];

    let res = candy_rating(ratings);
    println!("{:?}", res);
}


pub fn candy_rating(ratings: Vec<i32>) -> i32 {
    let mut candies = vec![1; ratings.len()];

    for i in 1..ratings.len() {
        // former larger case
        if ratings[i - 1] > ratings[i] {
            for idx in 0..i { candies[idx] += 1 }
        };

        // former smaller case
        if ratings[i - 1] < ratings[i] {
            candies[i] = candies[i - 1] + 1;
        };

        // equal case
        continue;
    }

    candies.iter().sum()
}