
pub fn main() {
    let arr = vec![5,4,3,2,1,6,7];

    let res = max_chunks_to_sorted(arr.clone());
    println!("{}", res);

}


pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
    let mut st = Vec::new();
    let mut cur = -1;
    for x in arr {
        while st.last().map_or(false, |&z| x < z) {
            st.pop();
        }
        cur = cur.max(x);
        st.push(cur);
    }

    st.len() as _
}