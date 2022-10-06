
pub fn main() {
    let mut islands = vec![
        vec![1,1,1,1,0],
        vec![1,1,0,1,0],
        vec![1,1,0,0,0],
        vec![0,0,0,0,0],
    ];

    let res = islands_count(&mut islands);
    println!("islands: {}", res);
}


pub fn islands_count(islands: &mut Vec<Vec<i32>>) -> i32 {
    if islands.is_empty() { return 0 };

    let mut res = 0;

    for h in 0..islands.len() {
        for w in 0..islands[h].len() {
            if islands[h][w] == 1 {
                res += 1;
                dfs_bfs(islands, h, w);
            }
        }
    };

    res
}


fn dfs_bfs(islands: &mut Vec<Vec<i32>>, h: usize, w: usize) {
    if islands[h][w] == 0 { return };

    islands[h][w] = 0;

    // search top
    if h > 0 {
        dfs_bfs(islands, h-1, w)
    };
    // search bottom
    if h <= islands.len() - 1 {
        dfs_bfs(islands, h+1, w)
    };
    // search left
    if w > 0 {
        dfs_bfs(islands, h, w-1)
    };
    // search right
    if w <= islands[0].len() - 1 {
        dfs_bfs(islands, h, w+1)
    };
}


