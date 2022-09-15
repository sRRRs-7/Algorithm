
// Symbol       Value
// I             1
// V             5
// X             10
// L             50
// C             100
// D             500
// M             1000

pub fn main() {
    let s = String::from("MCMXCIV");
    let num = roman_to_int(s);

    println!("{}", num);
}


pub fn roman_to_int(s: String) -> i32 {
    let mut s_arr: Vec<&str> = s.trim().split("").collect();
        s_arr.retain(|&x| x != "");
    let mut arr = Vec::new();

    for x in s_arr {
        match x {
            "I" => arr.push(1),
            "V" => arr.push(5),
            "X" => arr.push(10),
            "L" => arr.push(50),
            "C" => arr.push(100),
            "D" => arr.push(500),
            "M" => arr.push(1000),
            _ => panic!("panic"),
        };
    }

    for i in 0..arr.len() - 1 {
        if arr[i] < arr[i + 1] {
            arr[i + 1] = arr[i + 1] - arr[i];
            arr[i] = 0;
        };
    };
    arr.retain(|&x| x != 0);

    let mut sum = 0;
    for x in arr {
        sum += x
    }

    sum as i32

}






