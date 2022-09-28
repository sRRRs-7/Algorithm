use std::collections::BinaryHeap;

// network node
//     1
// 6 5 0 2
//    4 3


pub struct Edge {
    node: usize,
    cost: usize,
}

pub fn main() {
    let start = 2;
    let goal = 1;

    let graph = vec!(
        // node0
        vec!(
            Edge{node: 1, cost: 5},
            Edge{node: 2, cost: 5},
            Edge{node: 3, cost: 5},
            Edge{node: 4, cost: 5},
            Edge{node: 5, cost: 5},
        ),
        // node1
        vec!(
            Edge{node: 2, cost: 5},
        ),
        // node2
        vec!(
            Edge{node: 3, cost: 5},
        ),
        // node3
        vec!(
            Edge{node: 4, cost: 5},
        ),
        // node4
        vec!(
            Edge{node: 5, cost: 5},
        ),
        // node5
        vec!(
            Edge{node: 1, cost: 5},
            Edge{node: 6, cost: 1},
        ),
        // node6
        vec!(),
    );

    let res = dijkstra(&graph, start, goal);
    println!("cost: {:?}", res.unwrap());

    assert_eq!(dijkstra(&graph, 0, 1), Some(5));
    assert_eq!(dijkstra(&graph, 0, 2), Some(5));
    assert_eq!(dijkstra(&graph, 1, 6), Some(21));
    assert_eq!(dijkstra(&graph, 2, 1), Some(20));
    assert_eq!(dijkstra(&graph, 2, 4), Some(10));
    assert_eq!(dijkstra(&graph, 6, 1), None);
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost).then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn dijkstra(graph: &Vec<Vec<Edge>>, s: usize, g: usize) -> Option<usize> {
    let mut dist: Vec<_> = (0..graph.len()).map(|_| usize::MAX).collect();
    let mut heap = BinaryHeap::new();

    dist[s] = 0;
    heap.push(State{cost: 0, position: s});

    while let Some(State { cost, position }) = heap.pop() {
        if position == g { return Some(cost) };
        if cost > dist[position] { continue };

        for edge in &graph[position] {
            let next = State{
                cost: cost + edge.cost,
                position: edge.node,
            };

            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        };
    };

    None
}