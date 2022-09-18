
pub fn main() {
    let s = "abcd";
    let pat = "a*";

    let result = wildcard_matching(s, pat);
    if result {
        println!("{} matches {}", s, pat);
    } else {
        println!("{} not matches {}", s, pat);
    }

    let result2 = is_match(s, pat);
    if result2 {
        println!("{} matches {}", s, pat);
    } else {
        println!("{} not matches {}", s, pat);
    }
}


pub fn wildcard_matching(s: &str, pat: &str) -> bool {
    let s_chars: Vec<char> = s.chars().collect();
    let pat_chars: Vec<char> = pat.chars().collect();

    if s_chars.len() == pat_chars.len() || pat.contains('*') {
        for i in 0..pat_chars.len() - 1 {
            if s_chars[i] == pat_chars[i] {
                continue
            } else if pat_chars[i] == '?' {
                continue
            } else if pat_chars[i] == '*' {
                continue
            } else {
                return false
            }
        }
    } else {
        return false
    };

    true
}

// fast
pub fn is_match<S: AsRef<str>>(s: S, p: S) -> bool {
    let (s, p) = (s.as_ref().as_bytes(), p.as_ref().as_bytes());
    if s.is_empty() {
        return p.is_empty() || p.iter().all(|x| *x == b'*');
    }
    else if p.is_empty() {
        return false;
    }

    let mut dp = vec![false; p.len() + 1];
    dp[0] = true;
    for j in 1..dp.len() {
        dp[j] = if p[j-1] == b'*' {
                dp[j-1]
            } else {
                break
            };
    }
    for i in 1..=s.len() {
        let mut dp_i_1_j_1 = dp[0];
        for j in 1..dp.len() {
            let saved = dp[j];
            dp[j] = if s[i-1] == p[j-1] || p[j-1] == b'?' {
                dp_i_1_j_1
            } else if p[j-1] == b'*' {
                    dp[j] || dp[j-1]
                } else {
                    false
                };
            dp_i_1_j_1 = saved;
        }
        if i == 1 { dp[0] = false; }
    }
    *dp.last().unwrap()
}