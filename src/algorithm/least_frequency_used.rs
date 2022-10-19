

pub fn main() {
    let actions = vec!["LFUCache", "put", "put", "get", "put", "get", "get", "put", "get", "get", "get"];
    let instruments = vec![vec![2], vec![1, 1], vec![2, 2], vec![1], vec![3, 3], vec![2], vec![3], vec![4, 4], vec![1], vec![3], vec![4]];

    let res = least_frequency_used(actions, instruments);
    println!("{:?}", res);
}


#[derive(Debug, Clone, PartialEq)]
pub struct LFUCache {
    list: Vec<Option<KV>>
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct KV {
    frequency: Option<i32>,
    key: Option<i32>,
    value: Option<i32>,
}

impl LFUCache {
    fn new(capacity: usize) -> Self {
        Self { list: vec![ None; capacity ] }
    }
    fn get(&mut self, key: i32) -> i32 {
        let idx = self.list.iter().position(|&x| {
            if let Some(k) = x { k.key == Some(key) } else { false }
        } );
        if idx.is_some() {
            *self.list[idx.unwrap()].as_mut().unwrap().frequency.as_mut().unwrap() += 1;
            return self.list[idx.unwrap()].unwrap().value.unwrap();
        };
        -1
    }
    fn put(&mut self, key: i32, value: i32) {
        let idx = self.list.iter().position(|&x| {
            if let Some(k) = x { k.key == Some(key) } else { false }
        } );
        if idx.is_some() {
            *self.list[idx.unwrap()].as_mut().unwrap().value.as_mut().unwrap() = value;
            *self.list[idx.unwrap()].as_mut().unwrap().frequency.as_mut().unwrap() += 1;
        };

        let i = self.list.iter().position(|x| x == &None);
        if i.is_none() {
            for id in (1..self.list.len()).rev() {
                self.list.swap(id - 1, id)
            };
            self.list[0] = Some( KV { frequency: Some(1), key: Some(key), value: Some(value) } );
        } else {
            self.list[i.unwrap()] = Some( KV { frequency: Some(1), key: Some(key), value: Some(value) } );
        }
    }
}


pub fn least_frequency_used(actions: Vec<&str>, instruments: Vec<Vec<i32>>) -> Vec<i32> {
    let mut cache = LFUCache::new(instruments[0][0] as usize);

    let mut res = Vec::new();

    for i in 0..actions.len() {
        match actions[i] {
            "LFUCache" => {
                res.push(0);
            },
            "get" => {
                let key = instruments[i][0];
                res.push( cache.get(key) );
            },
            "put" => {
                let (key, value) = if instruments[i].len() == 2 { ( instruments[i][0], instruments[i][1] ) } else { ( instruments[i][0], 1 ) };
                cache.put(key, value);
                res.push( 0 );
            },
            _ => {
                res.push(00000);    // error
            }
        }
    }

    res
}