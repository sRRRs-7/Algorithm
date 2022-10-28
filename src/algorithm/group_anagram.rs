use std::collections::HashMap;


pub fn main() {
    let strs = vec!["eat","tea","tan","ate","nat","bat"];

    let res = group_anagram(strs.clone());
    println!("{:?}", res);

    let res2 = group_anagrams(strs);
    println!("{:?}", res2);
}

pub fn group_anagram(strs: Vec<&str>) -> Vec<Vec<&str>> {
    let mut arr = strs.clone();
    let mut res = Vec::new();

    loop {
        let mut group = Vec::new();
        let mut is_anagram = true;

        let str = arr.pop();

        if let Some(st) = str {
            group.push(st);

            for i in (0..arr.len()).rev() {
                for s in st.chars() {
                    if !strs[i].contains(s) { is_anagram = false; break; };
                }
                if is_anagram {
                    group.push(arr[i]);
                    arr.remove(i);
                };
                is_anagram = true;
            }
            res.push(group);

        } else { break; }
    }

    res
}


pub fn group_anagrams(strs: Vec<&str>) -> Vec<Vec<&str>> {

    let mut map = HashMap::new();
    for s in strs.into_iter() {
        let mut key = [0; 26];
        for ch in s.chars() {
            key[(ch as u32 - 'a' as u32) as usize] +=1;
        }
        map.entry(key).or_insert(Vec::new()).push(s);
    }
    map.into_iter().map(|(_, v)| v).collect()
}