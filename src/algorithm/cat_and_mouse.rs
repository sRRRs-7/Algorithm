
pub fn main() {
    let graph: Vec<Vec<i32>> = vec![vec![2,5],vec![3],vec![0,4,5],vec![1,4,5],vec![2,3],vec![0,2,3]];

    let res = cat_and_mouse(graph);
    println!("{}", res);
}


pub fn cat_and_mouse(graph: Vec<Vec<i32>>) -> i32 {
    let mut mouse = &graph[1];
    let mut cat = &graph[2];

    let mut routes = vec![vec![(&graph[0], 0)]];
    let mut is_route = false;
    let mut layer = vec![(&graph[0], 0)];

    while !is_route {
        let mut state = Vec::new();
        for ii in layer {
            for i in ii.0 {
                if graph[*i as usize].contains(&1) { is_route = true };
                state.push((&graph[*i as usize], *i));
            }
        }
        layer = state.clone();
        routes.push(state);
    }

    let mut best_route = Vec::new();
    let mut index = 0;

    for ii in 0..routes.len() {
        for i in (0..routes[ii].len()).rev() {
            if routes[ii][i].0.contains(&1) {
                index = i;
                best_route.push(routes[ii][i]);
            }
            if routes[ii].len() == 1 && routes[ii][i].1 == 0 { break };
            if best_route.len() != 0 {
                if routes[ii][i].0.contains(&best_route[ii-2].1) && routes[ii][i].1 == index as i32 {
                    index = i;
                    best_route.push(routes[ii][i]);
                    continue;
                }
            }
        }
    }
    best_route.push((&graph[0], 0));
    best_route.reverse();
    best_route.push((&graph[1], 1));


    while mouse == &graph[0] || mouse == cat {
        break
    };

    7
}