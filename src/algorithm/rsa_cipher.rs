
const ALPHABETS: [char; 26] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];

pub fn main() {
    rsa_cipher();
}

pub fn rsa_cipher() {
    let (p, q) = (3, 7);
    let n = (p - 1) * (q - 1);
    let e = 3;
    let k = 2;

    let public = p * q;
    let private = (k * n + 1) / e;

    let message = String::from("hello john");
    let num = convert_letter_to_number(message);

    println!("{}", num)

}


pub fn convert_letter_to_number(message: String) -> i128 {
    let mut res = Vec::new();

    for m in message.chars() {
        let idx = ALPHABETS.binary_search(&m);
        if let Ok(i) = idx {
            res.push(i + 1)
        }
    };

    let res: Vec<String> = res.iter().map(|x| x.to_string()).collect();
    let num: i128 = res.join("").parse().unwrap();
    num
}

