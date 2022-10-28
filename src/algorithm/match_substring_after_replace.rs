
pub fn main() {
    let s = "Fool33tbaR".to_string();
    let sub = "leetd".to_string();
    let mappings = vec![vec!['e','3'],vec!['t','7'],vec!['t','8'],vec!['d','b'],vec!['p','b']];

    let res = match_replacement(s, sub, mappings);
    println!("{}", res)
}


pub fn match_replacement(s: String, sub: String, mappings: Vec<Vec<char>>) -> bool {
    let s1: Vec<char> = s.chars().collect();
    let mut s2: Vec<char> = sub.chars().collect();

    for c2 in s2.clone() {
        let idx = s1.iter().position(|&x| x == c2);
        if let Some(i) = idx {
            if  i > s.len() - sub.len() { break };
            let arr = s1[ i..( i+sub.len() ) ].to_vec();

            for ii in 0..arr.len() {
                for c in sub.chars() {

                    if arr[ii] != c {
                        for m in &mappings {
                            if m[1] == arr[ii] { s2[ii] = m[1] }
                        }
                    }
                    if arr == s2 { return true }
                }
            };
        } else { continue; };
    };

    false
}