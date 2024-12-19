use itertools::Itertools;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

fn get_input() -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    let input = "r, wr, b, g, bwu, rb, gb, br
brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    let input = include_str!("../input/day19_1.txt");

    let mut lines = input.lines();
    let available = lines
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.chars().collect_vec())
        .collect_vec();
    lines.next().unwrap();
    let requested = lines.map(|s| s.trim().chars().collect_vec()).collect_vec();
    (available, requested)
}

fn is_possible<'a>(
    requested: &'a [char],
    available: &'a [Vec<char>],
    dp: &mut HashMap<&'a [char], u64>,
) -> u64 {
    if requested.is_empty() {
        return 1;
    }
    if let Entry::Occupied(entry) = dp.entry(requested) {
        return *entry.get();
    }
    let total = available
        .iter()
        .filter(|av| requested.starts_with(av))
        .map(|av| is_possible(&requested[av.len()..], available, dp))
        .sum1()
        .unwrap_or(0);
    *dp.entry(requested).or_default() = total;
    total
}
pub fn day19_1() {
    let (available, requested) = get_input();
    let mut dp = HashMap::new();
    let total = requested
        .iter()
        .filter(|req| is_possible(&req, &available, &mut dp) > 0)
        .count();
    println!("{}", total);
}
pub fn day19_2() {
    let (available, requested) = get_input();
    let total = requested
        .iter()
        .filter_map(|req| {
            let mut dp = HashMap::new();
            let res  = is_possible(req, &available, &mut dp) as u128;
            return Some(res);
        })
        .sum::<u128>();
    println!("{}", total);
}