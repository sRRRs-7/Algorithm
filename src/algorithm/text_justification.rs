
pub fn main() {
    let words: Vec<String> = vec!["This", "is", "an", "example", "of", "text", "justification", "main", "memory", "postgres", "preempt"].iter().map(|&x| x.to_string()).collect();
    let max_width = 16;

    let res = text_justification(words, max_width);
    for x in res { println!("{:?}", x); };
}

pub fn text_justification(words: Vec<String>, max_width: i32) -> Vec<String> {
    // chars count
    let mut counters = Vec::new();
    for word in &words { counters.push( word.chars().count() ) };

    // subdivide
    let mut sentences = Vec::new();

    let mut st = "".to_string();
    let mut num = 0;

    for word in words {
        if st.chars().count() + word.chars().count() < max_width as usize {
            num += 1;
            st = st + &word + &num.to_string();
        } else {
            if st.find("2") != None { st.remove(st.len() - 1); }
            sentences.push(st.to_string());
            num = 1;
            st = "".to_string() + &word + &num.to_string();
        }
    }
    sentences.push(st);

    println!("{:?}", sentences);

    let mut res = Vec::new();

    for w in &mut sentences {
        while (*w).chars().count() as i32 <= max_width as i32 {
            for x in "12345".chars() {
                if w.find(x) != None  {
                    let idx = w.find(x).unwrap();
                    w.insert(idx, ' ');
                }
            }
        };

        for n in "12345".chars() {
            if w.find(n) != None  {
                let idx = w.find(n).unwrap();
                w.remove(idx);
                w.insert(idx, ' ');
            }
        };

        res.push(w);
    };

    res.into_iter().map(|x| x.to_string()).collect()

}