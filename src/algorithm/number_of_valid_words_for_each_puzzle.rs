
pub fn main() {
    let words = vec!["apple","pleas","please"].iter().map(|&x| x.to_string()).collect();
    let puzzles = vec!["aelwxyz","aelpxyz","aelpsxy","saelpxy","xaelpsy"].iter().map(|&x| x.to_string()).collect();

    let res = find_num_of_valid_words(words, puzzles);
    println!("{:?}", res);
}


pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
    let mut res = Vec::new();
    let mut s = String::new();

    let mut state = Vec::new();

    for p in &puzzles {
        for w in &words {
            if w.contains(p.chars().nth(0).unwrap()) == true {
                for c in w.chars() {
                    if p.contains(c) { s += &c.to_string() };
                    if s.eq(w) { state.push(w); break; }
                };
            }
            s = "".to_string();
        };
        res.push( (state.len()) as i32 );
        state.clear();
    };

    res
}