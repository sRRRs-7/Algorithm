
// reverse poland
// stack

use std::collections::VecDeque;


pub fn main() {
    let s = String::from("-1+(-1-2)-2");

    let res = basic_calculator(s);
    println!("{}", res);
}


pub fn basic_calculator(s: String) -> i32 {
    const SPACE: char = ' ';
    const SIGN_PLUS: char = '+';
    const SIGN_MINUS: char = '-';
    const SIGN_MULTIPLY: char = '*';
    const SIGN_DIVIDE: char = '/';
    const PAREN_OPEN: char = '(';
    const PAREN_CLOSED: char = ')';

    let len_s: usize = s.len();
    let mut num: i32 = 0;
    let mut ans: i32 = 0;
    let mut sign: i32 = 1;
    let mut stk: VecDeque<i32> = VecDeque::with_capacity(len_s);    // stack

    stk.push_back(sign);

    for ch in s.chars() {
        match ch {
            '0'..='9' => { num = num * 10 + (ch as i32 - '0' as i32); },    // digit carry
            SIGN_PLUS | SIGN_MINUS => {
                ans += sign * num;  //  add sign
                sign = stk.back().unwrap() * if ch == SIGN_PLUS { 1 } else { -1 };  // prepare next calc
                num = 0;    // num reset
            },
            PAREN_OPEN => { stk.push_back(sign); },
            PAREN_CLOSED => { stk.pop_back(); },
            _ => {continue;}
        }
    }

    ans += sign * num;
    ans

}