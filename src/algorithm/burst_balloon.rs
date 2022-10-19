
pub fn main() {
    let points = vec![1,5];

    let res = burst_balloon(points);
    println!("{}", res);
}

pub fn burst_balloon(points: Vec<i32>) -> i32 {
    let mut balloons = points;
    let mut total_point = 0;

    while balloons.len() != 0 {
        let l = balloons.len();
        let mid = (l - 1) / 2;

        if l > 2 {
            total_point += balloons[mid - 1] * balloons[mid] * balloons[mid + 1];
            balloons.remove(mid);
        } else if l == 2 {
            total_point += balloons[mid] * balloons[mid + 1];
            balloons.remove(mid);
        } else {
            total_point += balloons[0];
            balloons.remove(0);
        }
    };

    total_point
}