
pub fn main() {
    let s = "great";
    let s2 = "rgeat";

    let res = scramble_string(s, s2);
    println!("{}", res);
}

pub fn scramble_string(s: &str, s2: &str) -> bool {
    let mut s1: Vec<char> = s.chars().collect();
    let mut l = s1.len() - 1;

    if l <= 1 { return true };

    while l != 0 {
        s1.swap(l, l-1);
        let sw: String = s1.clone().into_iter().collect();

        if sw == s2.to_string() { return true };

        s1.swap(l-1, l);
        l -= 1;
    }

    false
}