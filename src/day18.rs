use itertools::Itertools;
use petgraph::algo::dijkstra;
use petgraph::prelude::*;

fn get_input() -> Vec<(u32, u32)> {
    let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    let input = include_str!("../input/day18_1.txt");

    let input = input
        .lines()
        .map(|l| {
            let (a, b) = l.trim().split_once(",").unwrap();
            return (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap());
        })
        .collect_vec();

    input
}

pub fn day18_1() {
    let input = get_input();
    const size: usize = 7;
    const take: usize = 22;
    let mut field = [[0; size]; size];
    for (x, y) in input.iter().take(take) {
        field[*y as usize][*x as usize] = 1;
    }

    let mut nodes = vec![vec![None; size]; size];
    let mut graph = Graph::new_undirected();
    for i in 0..size {
        for j in 0..size {
            if field[i][j] != 1 {
                let node = graph.add_node((i, j));
                nodes[i][j] = Some(node);
                if i > 0 {
                    if let Some(neighbor) = nodes[i - 1][j] {
                        graph.add_edge(neighbor, node, ());
                    }
                }

                if j > 0 {
                    if let Some(neighbor) = nodes[i][j - 1] {
                        graph.add_edge(neighbor, node, ());
                    }
                }
            }
        }
    }
    let start = nodes[0][0].unwrap();
    let end = nodes[size - 1][size - 1].unwrap();
    let res = dijkstra(&graph, start, Some(end), |_| 1);
    println!("{}", res[&end]);
}
pub fn day18_2() {
    let input = get_input();
    const size: usize = 71;

    let mut nodes = vec![vec![]; size];
    let mut graph = StableUnGraph::with_capacity(size*size, 2*size*size);
    for i in 0..size {
        for j in 0..size {
            let node: petgraph::prelude::NodeIndex<u32> = graph.add_node((i, j));
            nodes[i].push(node);
            if i > 0 {
                graph.add_edge(node, nodes[i - 1][j], ());
            }
            if j > 0 {
                graph.add_edge(node, nodes[i][j - 1], ());
            }
        }
    }
    let start = nodes[0][0];
    let end = nodes[size - 1][size-1];
    for (i, j) in input {
        graph.remove_node(nodes[i as usize][j as usize]);
        let res = dijkstra(&graph, start, Some(end), |_| 1);
        if !res.contains_key(&end) {
            println!("{},{}", i, j);
            break;
        }
    }
}
