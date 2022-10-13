

pub fn main() {
    let (word1, word2) = (String::from("intention"), String::from("execution"));

    let res = min_distance(word1, word2);
    println!("{}", res);
}

pub fn min_distance(word1: String, word2: String) -> i32 {
    let mut m = Vec::with_capacity(word2.len());
    m.extend(word2.chars().zip(1..));

    for (i, c) in word1.chars().enumerate() {
        let mut x = i;
        let mut y = i + 1;
        for (d, v) in m.iter_mut() {
            y = std::cmp::min(x + (c != *d) as usize, std::cmp::min(y, *v) + 1);
            x = *v;
            *v = y;
        }
    }

    m.pop().map_or(word1.len(), |x|x.1) as i32
}

