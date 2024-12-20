use itertools::Itertools;
use petgraph::algo::dijkstra;
use petgraph::graph::{DiGraph, UnGraph};
use petgraph::prelude::{NodeIndex, StableUnGraph};
use petgraph::visit::NodeRef;
use std::collections::{HashMap, HashSet};
use petgraph::data::DataMap;
use petgraph::dot::{Config, Dot};
use crate::day12::Dir::P;

fn get_input() -> Vec<Vec<char>> {
    let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    let input = include_str!("../input/day20_1.txt");

    let map = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    map
}

pub fn day20_1() {
    let map = get_input();
    let n = map.len();
    let m = map[0].len();
    let mut node_map = vec![vec![None; m]; n];

    let mut start = Default::default();
    let mut end = Default::default();
    let mut graph = StableUnGraph::default();
    let mut cheat_pos = HashSet::new();
    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            let node: Option<NodeIndex<u32>> = match c {
                '.' | 'S' | 'E' => Some(graph.add_node([i, j])),
                _ => None,
            };
            if *c == 'S' {
                start = node.unwrap();
            } else if *c == 'E' {
                end = node.unwrap();
            }
            node_map[i][j] = node;
            if let Some(node) = node {
                if i > 0 {
                    if let Some(neighbor) = node_map[i - 1][j] {
                        graph.add_edge(node, neighbor, 1);
                    }
                }
                if j > 0 {
                    if let Some(neighbor) = node_map[i][j - 1] {
                        graph.add_edge(node, neighbor, 1);
                    }
                }
                if i > 1 {
                    if let Some(neighbor) = node_map[i - 2][j] {
                        cheat_pos.insert((node, neighbor));
                    }
                }
                if j > 1 {
                    if let Some(neighbor) = node_map[i][j - 2] {
                        cheat_pos.insert((node, neighbor));
                    }
                }
            }
        }
    }
    let start_dist = dijkstra(&graph, start, None, |e| *e.weight());
    let end_dist = dijkstra(&graph, end, None, |e| *e.weight());

    let fair_time = dijkstra(&graph, start, Some(end), |e| *e.weight())[&end];
    let mut total = 0;
    let mut advantage_map = HashMap::new();
    for (a, b) in cheat_pos {
        let d1 = start_dist[&a] + end_dist[&b] + 2;
        let d2 = start_dist[&b] + end_dist[&a] + 2;
        let cheat_time = d1.min(d2).min(fair_time);
        // let edge = graph.add_edge(a, b, 2);
        // let cheat_time = dijkstra(&graph, start, Some(end), |e| *e.weight())[&end];
        let advantage = fair_time - cheat_time;
        *advantage_map.entry(advantage).or_insert(0) += 1;
        if advantage >= 100 {
            total += 1;
        }
        // graph.remove_edge(edge);
    }
    // println!("{:?}", advantage_map.iter().sorted());
    println!("{}", total);
}

pub fn day20_2() {
    let map = get_input();
    let n = map.len();
    let m = map[0].len();
    let mut pmap = vec![vec![None; m]; n];

    let mut start: NodeIndex<usize> = Default::default();
    let mut end = Default::default();
    let mut pgraph = UnGraph::default();

    // init graph
    for i in 0..n {
        for j in 0..m {
            if map[i][j] != '#' {
                let node = pgraph.add_node((i, j));
                if map[i][j] == 'S' {
                    start = node;
                }
                if map[i][j] == 'E' {
                    end = node;
                }
                pmap[i][j] = Some(node);

                if i > 0 && pmap[i - 1][j].is_some() {
                    pgraph.add_edge(node, pmap[i - 1][j].unwrap(), 1);
                }
                if j > 0 && pmap[i][j - 1].is_some() {
                    pgraph.add_edge(node, pmap[i][j - 1].unwrap(), 1);
                }
            }
        }
    }

    let start_dist = dijkstra(&pgraph, start, None, |e| *e.weight());
    let end_dist = dijkstra(&pgraph, end, None, |e| *e.weight());
    let mut shortcuts = HashSet::new();
    for w1 in pgraph.node_weights() {
        for w2 in pgraph.node_weights() {
            let d = w1.0.abs_diff(w2.0) + w1.1.abs_diff(w2.1);
            if d > 0 && d <= 20{
               if w1 < w2 {
                   shortcuts.insert((*w1, *w2, d));
               } else {
                   shortcuts.insert((*w2, *w1, d));
               }
            }
        }
    }

    let fair_time = dijkstra(&pgraph, start, Some(end), |e| *e.weight())[&end];
    let mut total = 0;
    let mut advantage_map = HashMap::new();
    for (a, b, d) in shortcuts {
        let a = pmap[a.0][a.1].unwrap();
        let b = pmap[b.0][b.1].unwrap();
        let d1 = start_dist[&a] + end_dist[&b] + d;
        let d2 = start_dist[&b] + end_dist[&a] + d;
        let cheat_time = d1.min(d2).min(fair_time);
        // let edge = graph.add_edge(a, b, 2);
        // let cheat_time = dijkstra(&graph, start, Some(end), |e| *e.weight())[&end];
        let advantage = fair_time - cheat_time;
        if advantage >= 50 {
            *advantage_map.entry(advantage).or_insert(0) += 1;
        }
        if advantage >= 100 {
            total += 1;
        }
        // graph.remove_edge(edge);
    }
    // println!("{:?}", advantage_map.iter().sorted());
    println!("{}", total);
}
