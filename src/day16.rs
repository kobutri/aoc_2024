use std::collections::{HashSet, VecDeque};
use itertools::Itertools;
use petgraph::algo::{astar, dijkstra};
use petgraph::data::Build;
use petgraph::visit::{IntoEdges, NodeRef, Visitable};
use petgraph::prelude::*;

fn get_input() -> (Graph<(usize, usize, char), i32, Undirected>, NodeIndex, Vec<NodeIndex>) {
    let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    let input = include_str!("../input/day16_1.txt");

    let map = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let n = map.len();
    let m = map[0].len();
    // dir order t l r d
    let mut graph_map = vec![vec![vec![]; m]; n];
    let mut start = None;
    let mut end = vec![];
    let mut graph = Graph::new_undirected();
    map.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, c)| match c {
            'S' | 'E' | '.' => {
                let t = graph.add_node((i, j, 't'));
                let l = graph.add_node((i, j, 'l'));
                let r = graph.add_node((i, j, 'r'));
                let d = graph.add_node((i, j, 'd'));
                graph.add_edge(t, l, 1000);
                graph.add_edge(l, d, 1000);
                graph.add_edge(d, r, 1000);
                graph.add_edge(r, t, 1000);
                graph.add_edge(t, d, 0);
                graph.add_edge(l, r, 0);
                graph_map[i][j] = vec![t, l, r, d];
                if *c == 'S' {
                    start = Some(l);
                }
                if *c == 'E' {
                    end = graph_map[i][j].clone();
                }
            }
            _ => {}
        });
    });
    graph_map.iter().enumerate().skip(1).for_each(|(i, row)| {
        row.iter().enumerate().skip(1).for_each(|(j, c)| {
            if !c.is_empty() {
                let v =&graph_map[i - 1][j];
                if !v.is_empty() {
                    graph.add_edge(v[3], c[0], 1);
                }
                let v =&graph_map[i][j-1];
                if !v.is_empty() {
                    graph.add_edge(v[2], c[1], 1);
                }
            }
        });
    });

    let start = start.unwrap();

    (graph, start, end)
}
pub fn day16_1() {
    let (graph, start, end) = get_input();
    let (cost, path) = astar(&graph, start, |finish| end.contains(&finish), |e| {
        *e.weight()
    }, |_| 0).unwrap();

    for node in path {
        let weight = graph.node_weight(node).unwrap();
    }
    println!("{}", cost);
}
pub fn day16_2() {
    let (graph, start, end) = get_input();
    let cost = dijkstra(&graph, start, None, |e| *e.weight());
    let min_dist = end.iter().map(|e| cost[e]).min().unwrap();
    let mut set = HashSet::new();
    set.insert(start);
    let mut queue = VecDeque::new();
    for e in end.iter().filter(|e| cost[*e] == min_dist) {
        queue.push_back(*e);
    }

    let mut visited = HashSet::new();
    while let Some(node) = queue.pop_front() {
        if visited.contains(&node) {
            continue;
        }
        visited.insert(node);
        set.insert(node);
        graph.neighbors(node).filter(|n| {
            let edge = graph.find_edge(node, *n).unwrap();
            graph.edge_weight(edge).unwrap() + cost[n] == cost[&node]
        }).for_each(|n| {
            queue.push_back(n);
        });
    }
    let set = set.into_iter().fold(HashSet::new(), |mut acc, e| {
        let (a, b, _) = graph.node_weight(e).unwrap();
        acc.insert((a,b));
        acc
    });
    println!("{}", set.len());
}
