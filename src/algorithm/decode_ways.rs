
// A -> 1
// B -> 2
//.
// Z -> 26

pub fn main() {
    let code = "165";

    let res = decode_ways(code);
    println!("decode ways: {}", res);
}

pub fn decode_ways(code: &str) -> i32 {
    let codes: Vec<char> = code.chars().collect();
    let mut res = 1;

    for (i, c) in codes.iter().enumerate() {
        if *c == '0' { continue };
        if i + 1 == codes.len() { break };

        let num = ( (*c as i32 - 48) * 10 ) + (codes[i + 1] as i32 - 48);
        if num <= 26 { res += 1 };
    };

    res as i32
}