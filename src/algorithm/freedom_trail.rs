
pub fn main() {
    let (ring, key) = (String::from("godding"), String::from("gg"));

    let res = freedom_trail(ring, key);
    println!("freedom trail: {}", res);
}


pub fn freedom_trail(ring: String, key: String) -> i32 {
    let rings: Vec<char> = ring.chars().collect();
    let keys: Vec<char> = key.chars().collect();

    let mut step = 0;

    let mid = (rings.len() - 1) / 2;
    let mut cnt = 0;

    for k in &keys {
        if !rings.contains(&k) { continue; };
        for r in &rings {
            cnt += 1;
            if *k == *r { break; };
        };
        if cnt <= mid { step += cnt } else { step = step + (cnt - mid - 1) };
        cnt = 0;
    };

    step as i32
}