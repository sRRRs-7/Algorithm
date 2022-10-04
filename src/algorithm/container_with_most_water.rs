
pub fn main() {
    let heights = vec![1,1];

    let res = container_with_most_water(heights);
    println!("most water: {}", res);
}

pub fn container_with_most_water(heights: Vec<u32>) -> i32 {
    let mut res = 0;
    let max = heights.iter().max().unwrap();
    let max_idx = heights.iter().position(|e| e == max).unwrap();

    for (i, h) in heights.iter().enumerate() {
        let width = ( max_idx as i32 - i as i32 ).abs();

        let area = *h as i32 * width;

        if res <= area { res = area };
    }

    res as i32
}