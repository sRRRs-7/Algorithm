
pub fn main() {
    let instruments = vec!["Fancy", "append", "addAll", "append", "multAll", "getIndex", "addAll", "append", "multAll", "getIndex", "getIndex", "getIndex"];
    let values: Vec<Vec<i32>> = vec![[0], [2], [3], [7], [2], [0], [3], [10], [2], [0], [1], [2]].iter().map(|x| x.to_vec()).collect();

    let res = fancy_sequence(instruments, values);
    println!("{:?}", res);
}


struct Fancy {
    output: Vec<i32>
}

impl Fancy {
    fn new() -> Self { Self { output: Vec::new() } }
    fn append(&mut self, val: i32) { self.output.push(val) }
    fn add_all(&mut self, inc: i32) {
        for x in self.output.iter_mut() { *x = *x + inc; }
    }
    fn mult_all(&mut self, m: i32) {
        for x in self.output.iter_mut() { *x = *x * m; }
    }
    fn get_index(&mut self, idx: i32) -> i32 {
        if let Some(&x) = self.output.get(idx as usize) {
            x
        } else {
            -1
        }
    }
}


pub fn fancy_sequence(instruments: Vec<&str>, values: Vec<Vec<i32>>) -> Vec<i32> {
    let mut out = Fancy::new();
    let mut res = Vec::new();

    for i in 0..instruments.len() {
        match instruments[i] {
            "Fancy" => { out = Fancy::new() },
            "append" => {
                out.append(values[i][0]);
                res.push(0);
            },
            "addAll" => {
                out.add_all(values[i][0]);
                res.push(0);
            },
            "multAll" => {
                out.mult_all(values[i][0]);
                res.push(0);
            },
            "getIndex" => {
                res.push( out.get_index(values[i][0]) );
            },
            _ => { res.push(00000) }
        }
    };

    res
}