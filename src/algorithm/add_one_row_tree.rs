
// 0 is None

pub fn main() {
    let root = vec![4,2,6,3,1,5];
    let (value, depth) = (1, 2);

    let res = add_one_row_tree(root, value, depth);

    let mut state = 2;
    for (i, x) in res.iter().enumerate() {
        print!("{}", x);

        if i + 2 == state {
            println!();
            state = state * 2;
        }

        if res.len() -1 == i { println!() }
    };

    println!("add bread first tree: {:?}", res);
}

pub fn add_one_row_tree(root: Vec<i32>, value: i32, depth: i32) -> Vec<i32> {
    let mut res = root;

    let idx = 2_i32.pow((depth - 1) as u32) - 1;

    res.insert(idx as usize + 1, 0);
    res.insert(idx as usize + 1, 0);

    res.insert(idx as usize, value);
    res.insert(idx as usize + 1, value);

    res
}