
pub fn main() {
    let heights = vec!(0,1,0,2,1,0,1,3,2,1,2,1);

    let result = elevation_map(heights);
    println!("{}", result);

    let heights2 = vec!(0,1,0,2,1,0,1,3,2,1,2,1);

    let result2 = trap(heights2);
    println!("{}", result2);
}

pub fn elevation_map(heights: Vec<i32>) -> i32 {
    let mut water = 0;
    let mut height = 0;
    let max = heights.iter().max().unwrap();

    for (i, x) in heights.iter().enumerate() {
        if height < *x {
            height = *x;
            continue
        };

        if height > *x && *max != height {
            water += height - x
        };

        if *max == height {
            if *x > heights[i - 1] {
                water += x - heights[i - 1]
            }
        };
    }

    water
}


pub fn trap(heights: Vec<i32>) -> i32 {
    let forward = running_max(heights.iter().cloned());
    let backward = running_max(heights.iter().rev().cloned())
        .collect::<Vec<_>>()
        .into_iter()
        .rev();

    forward
        .zip(backward)
        .map(|(l, r)| l.min(r))
        .zip(heights.iter())
        .map(|(w, &h)| w - h)
        .sum()
}

fn running_max(iter: impl Iterator<Item = i32>) -> impl Iterator<Item = i32> {
    iter.scan(0, |a, b| {
        *a = (*a).max(b);
        Some(*a)
    })
}