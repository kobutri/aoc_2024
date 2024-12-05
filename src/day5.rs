use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

fn get_input() -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let input = "47|53
    97|13
    97|61
    97|47
    75|29
    61|13
    75|53
    29|13
    97|29
    53|29
    61|53
    97|53
    61|29
    47|13
    75|47
    97|75
    47|61
    75|61
    47|29
    75|13
    53|13

    75,47,61,53,29
    97,61,53,29,13
    75,29,13
    75,97,47,61,53
    61,13,29
    97,13,75,29,47";

    let input = include_str!("../input/day5_1.txt");

    let (ordering_rules, updates) = input.split_once("\n\n").unwrap();
    let ordering_rules = ordering_rules
        .lines()
        .map(|l| {
            let (n1, n2) = l.trim().split_once("|").unwrap();
            (n1.parse::<i32>().unwrap(), n2.parse::<i32>().unwrap())
        })
        .collect::<Vec<_>>();
    let updates = updates
        .lines()
        .map(|l| {
            l.trim()
                .split(",")
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (ordering_rules, updates)
}

fn is_correct(rules: &[(i32, i32)], values: &[i32]) -> bool {
    let mut correct = true;
    for rule in rules {
        let Some(p1) = values.iter().position(|&n| n == rule.0) else {
            continue;
        };
        let Some(p2) = values.iter().position(|&n| n == rule.1) else {
            continue;
        };
        if p1 >= p2 {
            correct = false;
            break;
        }
    }
    return correct;
}

pub fn day5_1() {
    let (ordering_rules, updates) = get_input();
    let mut total = 0;
    for update in updates {
        if is_correct(&ordering_rules, &update) {
            total += update[update.len() / 2];
        }
    }
    println!("{}", total);
}
pub fn day5_2() {
    let (ordering_rules, updates) = get_input();
    let mut left = HashMap::new();
    let mut right = HashMap::new();
    ordering_rules.iter().for_each(|(n1, n2)| {
        left.entry(*n1).or_insert(HashSet::new()).insert(*n2);
        right.entry(*n2).or_insert(HashSet::new()).insert(*n1);
    });

    let mut total = 0;
    for mut update in updates {
        if !is_correct(&ordering_rules, &update) {
            update.sort_by(|a, b| {
               if left.get(a).map(|set| set.contains(&b)).unwrap_or(false) {
                   Ordering::Less
               } else if right.get(a).map(|set| set.contains(&b)).unwrap_or(false) {
                   Ordering::Greater
               } else {
                  Ordering::Equal 
               }
            });
            total += update[update.len() / 2];
        }
    }
    println!("{}", total);
}
