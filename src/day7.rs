use core::panic;
use std::vec;

use itertools::Itertools;
use rand::Rng;

fn has_solution(target: u128, current: u128, values: &[u128]) -> bool {
    if values.is_empty() {
        return target == current;
    }
    has_solution(target, current + values[0], &values[1..])
        || has_solution(target, current * values[0], &values[1..])
}

fn concat_digits(n1: u128, n2: u128) -> u128 {
    let mut digits = 1;
    let mut temp = n2.clone();
    while temp >= 10 {
        digits += 1;
        temp /= 10;
    }

    n1 * 10u128.pow(digits) + n2
}

fn has_solution2(target: u128, current: u128, values: &[u128]) -> bool {
    // println!(
    //     "target: {}, current: {}, values: {:?}",
    //     target, current, values
    // );
    if current > target {
        return false;
    }
    if values.is_empty() {
        return target == current;
    }
    if has_solution2(target, current + values[0], &values[1..]) {
        return true;
    }
    if has_solution2(target, current * values[0], &values[1..]) {
        return true;
    }
    if has_solution2(target, concat_digits(current, values[0]), &values[1..]) {
        return true;
    }
    false
}

fn get_input() -> Vec<(u128, Vec<u128>)> {
    let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    let input = include_str!("../input/day7_1.txt");

    input
        .lines()
        .map(|line| {
            let (target, rest) = line.split_once(": ").unwrap();
            let target = target.parse::<u128>().unwrap();
            let values = rest
                .split(" ")
                .map(|v| v.parse::<u128>().unwrap())
                .collect_vec();
            (target, values)
        })
        .collect_vec()
}

pub fn day7_1() {
    let input = get_input();
    let total = input
        .iter()
        .filter_map(|(target, values)| {
            if has_solution(*target, 0, values) {
                Some(target)
            } else {
                None
            }
        })
        .sum::<u128>();

    println!("{}", total);
}
pub fn day7_2() {
    // loop {
    //     let (target, values, ops) = generate_true_cases();
    //     if !has_solution2(target, values[0], &values[1..]) {
    //         panic!("target: {}, numbers: {:?}, ops: {:?}", target, values, ops);
    //     } else {
    //         // println!("success");
    //     }
    // }
    let input = get_input();
    let mut false_counter = 0;
    let total = input
        .iter()
        .filter_map(|(target, values)| {
            if has_solution2(*target, 0, values) {
                Some(*target)
            } else {
                false_counter += 1;
                None
            }
        })
        .sum::<u128>();
    
    println!("{}", false_counter);
    
    println!("{}", total);
}

fn generate_true_cases() -> (u128, Vec<u128>, Vec<char>) {
    let count = rand::thread_rng().gen_range(2..=10);
    let values = (0..count)
        .map(|_| {
            rand::thread_rng().gen_range(1..1000)
        })
        .collect_vec();

    let mut ops = vec![];
    let val = values
        .clone()
        .into_iter()
        .reduce(|acc, e| {
            let op = rand::thread_rng().gen_range::<u8, _>(0..=2);
            if op == 0 {
                ops.push('+');
                acc + e
            } else if op == 1 {
                ops.push('*');
                acc * e
            } else if op == 2 {
                ops.push('|');
                concat_digits(acc, e)
            } else {
                unreachable!()
            }
        })
        .unwrap();

    (val, values, ops)
}
