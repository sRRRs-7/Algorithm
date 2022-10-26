
pub fn main() {
    let num1 = vec![2,0,1,3];
    let num2 = vec![0,1,2,3];

    let res = triplet_array(num1, num2);
    println!("triplet_array: {}", res)
}

pub fn triplet_array(num1: Vec<i32>, num2: Vec<i32>) -> i64 {
    let mut index = Vec::new();
    for n2 in &num2 {
        let idx = num1.iter().position(|&x| x == *n2);
        if let Some(x) = idx { index.push(x); }
    };

    let mut candidate = Vec::new();
    for x in &index {
        for y in &index {
            if x >= y { continue; }
            for z in &index {
                if y >= z { continue; }
                if x<y && y<z && x<z { candidate.push( (num2[*x], num2[*y], num2[*z]) ) };
            }
        }
    };

    let mut res = Vec::new();
    for c1 in &candidate {
        let idx1 = num1.iter().position(|&x| x == c1.0);
        let idx2 = num1.iter().position(|&x| x == c1.1);
        let idx3 = num1.iter().position(|&x| x == c1.2);

        if idx1 < idx2 && idx2 < idx3 && idx1 < idx3 { res.push(c1) };
    };

    res.len() as i64
}