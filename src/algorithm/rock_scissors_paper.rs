use std::io::stdin;

use random_number::random;


pub fn main() {
    let input = input_value();

    println!("player: {}", input);

    let res = rock_scissor_paper(input);
    if res { println!("you are winner!!") }
    else { println!("you are loser..") }
}


fn input_value() -> String {
    println!("rock.. scissor.. paper..");

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    if !input.is_empty() { input.truncate(input.len() - 1) };

    input
}


pub fn rock_scissor_paper(input: String) -> bool {
    let mut p = match input.as_str() {
        "rock" => 0,
        "scissor" => 2,
        "paper" => 5,
        _ => 0
    };

    loop {
        let rand: u8 = random!(..=8);
        let np = match rand {
            0..=2 => 0,
            3..=5 => 2,
            6..=8 => 5,
            _ => panic!("non player error"),
        };

        let np_sign = match np {
            0 => "rock",
            2 => "scissor",
            5 => "paper",
            _ => panic!("please select rock, scissor or paper only"),
        };

        println!("non player: {}", np_sign);

        if p == np {
            p = even();
            continue;
        };

        if p - np == -2 || p - np == -3 || p - np == 5 { return true }
        else { return false };
    }
}

fn even() -> i32 {
    let inp = input_value();
    let p = match inp.as_str() {
        "rock" => 0,
        "scissor" => 2,
        "paper" => 5,
        _ => panic!("please select rock, scissor or paper only"),
    };

    p
}