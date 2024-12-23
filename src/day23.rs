use itertools::Itertools;
use petgraph::graph::NodeIndex;
use petgraph::{Graph, Undirected};
use std::collections::{BTreeMap, HashSet};
use petgraph::data::DataMap;
use petgraph::prelude::{StableGraph, StableUnGraph};

fn get_input() -> (BTreeMap<&'static str, NodeIndex<u32>>, StableGraph<&'static str, (), Undirected, u32>) {
    let input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";
    let input = include_str!("../input/day23_1.txt");
    let mut graph = StableUnGraph::default();
    let map = input.lines().fold(BTreeMap::new(), |mut map, line| {
        let (c1, c2) = line.trim().split_once("-").unwrap();
        map.entry(c1).or_insert(graph.add_node(c1));
        map.entry(c2).or_insert(graph.add_node(c2));

        map
    });
    input.lines().for_each(|line| {
        let (c1, c2) = line.trim().split_once("-").unwrap();
        graph.add_edge(map[c1], map[c2], ());
    });

    (map, graph)
}

pub fn day23_1() {
    let (map, graph) = get_input();
    let mut triplets = HashSet::new();
    map.range("t".."u").for_each(|(weight, node)| {
        graph
            .neighbors(*node)
            .cartesian_product(graph.neighbors(*node))
            .filter(|&(c1, c2)| graph.contains_edge(c1, c2))
            .for_each(|(c2, c1)| {
                let mut key = [node.index(), c1.index(), c2.index()];
                key.sort();
                triplets.insert(key);
            })
    });
    println!("{}", triplets.len());
}

pub fn maximum_clique(
    graph: &StableGraph<&'static str, (), Undirected, u32>,
    candidates: HashSet<NodeIndex>,
    best: &mut HashSet<NodeIndex>,
) {
    if candidates.len() < best.len() {
        return;
    } else if candidates
        .iter()
        .cartesian_product(candidates.iter())
        .all(|(a, b)| {
            graph.contains_edge(*a, *b) || a == b
        })
    {
        if candidates.len() > best.len() {
            *best = candidates;
        }
        return;
    }
    for node in &candidates {
        let mut subcandidates = graph
            .neighbors(*node)
            .filter(|c| {
                let neighbor = graph.node_weight(*c).unwrap();
                candidates.contains(c)
            })
            .collect::<HashSet<_>>();
        subcandidates.insert(*node);
        if subcandidates.len() == candidates.len() {
             continue;
        }
        maximum_clique(graph, subcandidates, best);
    }
}
pub fn day23_2() {
    let (map, graph) = get_input();
    let mut best = HashSet::new();
    let candidates = HashSet::from_iter(graph.node_indices());
    maximum_clique(&graph, candidates, &mut best);
    let res = best.into_iter().map(|c| graph.node_weight(c).unwrap()).sorted().join(",");
    println!("{}", res);
}
