
const ALPHABETS: [char; 26] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];

pub fn main() {
    rsa_cipher();
    convert_letter_to_number(String::from("hello"));
}

pub fn rsa_cipher() {
    let (p, q) = (3, 7);    // prime numbers

    let n = p * q;  // public key
    let mut phi = (p - 1) * (q - 1);

    let mut e = 2;
    while e < phi {
        if gcd(&mut e, &mut phi) == 1 { break };
        e += 1
    };

    let k = 2;  // constant value
    let d = (1 + (k * phi)) / e;    // private key

    let msg: i64 = 20;
    // let num = convert_letter_to_number(msg);
    println!("raw data {}", msg);

    let c = msg.pow(e as u32) % n as i64;
    println!("encrypt data: {}", c);

    let m = c.pow(d as u32) % n as i64;
    println!("decrypt data: {}", m);

    println!("p{} q{} n{} e{} phi{} k{} d{} msg{} c{} m{}", p, q, n, e, phi, k, d, msg, c, m);

}


pub fn gcd(e: &mut i64, phi: &mut i64) -> i64 {
    let mut temp;

    loop {
        temp = *e % *phi;
        if temp == 0 { return *phi };
        *e = *phi;
        *phi = temp;
    };
 }


pub fn convert_letter_to_number(message: String) -> f64 {
    let mut res = Vec::new();

    for m in message.chars() {
        let idx = ALPHABETS.binary_search(&m);
        if let Ok(i) = idx {
            res.push(i + 1)
        }
    };

    let res: Vec<String> = res.iter().map(|x| x.to_string()).collect();
    let num: f64 = res.join("").parse().unwrap();
    num
}

