
//  error
//  serial double sign + -
//  only string
//  include other than e E
//  after e -> None, float, e, E only

pub fn main() {
    let s = String::from("-123.456e789");

    let res = valid_number(s);
    println!("Is number valid ?: {}", res);
}

pub fn valid_number(s: String) -> bool {
    let arr: Vec<char> = s.chars().collect();

    let mut res = true;

    for i in 0..arr.len() - 1 {
        res = match arr[i] {
            '+' | '-' => {
                if i == 0 { continue; }
                if arr[i - 1] == '+' || arr[i - 1] == '-' { return false }
                continue;
            },
            'e' | 'E' => {
                if i + 1 == arr.len() - 1 { return false }
                for x in i..arr.len() - 1 {
                    if arr[x] == '.' { return false }
                };
                continue;
            },
            'a'..='d'| 'f'..='z' => { false },  // include other than 'e'
            'A'..='D'| 'F'..='Z' => { false },  // include other than 'E'
            _ => { continue; },
        }
    };

    res
}