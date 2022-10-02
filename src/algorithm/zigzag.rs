
pub fn main() {
    let (s, rows) = ("helloworld", 3);

    let s = zigzag(s, rows);
    println!("zigzag: {}", s);
}

pub fn zigzag(s: &str, rows: i32) -> String {
    if &rows <= &2 { return s.to_string() }

    let mut strings = vec![String::new(); rows as usize];
    let mut i = 0;
    let mut down = true;

    for c in s.chars() {
        strings[i].push(c);
        i = if down { i + 1 } else { i - 1 };
        down = down == ( 0 < i && i < rows as usize - 1 );
    };

    strings.concat()
}