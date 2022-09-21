
pub fn main() {
    let mut decimal: i32 = 120;
    let mut to_radix:i32 = 11;
    let mut value = String::from("");

    radix_convert(&mut decimal, &mut to_radix, &mut value);
    println!("{}", value);
}

pub fn radix_convert(decimal: &mut i32, to: &mut  i32, value: &mut String) {
    if decimal == to {
        return
    }

    let mut arr = Vec::new();
    loop {
        let surplus = *decimal % *to;
        let div = *decimal / *to;

        match surplus {
            1..=9 =>  arr.push(format!("{}", surplus)),
            10 =>  arr.push("A".to_string()),
            11 =>  arr.push("B".to_string()),
            12 =>  arr.push("C".to_string()),
            13 =>  arr.push("D".to_string()),
            14 =>  arr.push("E".to_string()),
            15 => arr.push("F".to_string()),
            _ => panic!("error")
        }

        if div == 0 {
            break
        };

        *decimal = div;
    }

    let s: String = arr.iter().rev().map(|x| x.to_string()).collect();
    *value = s;
}