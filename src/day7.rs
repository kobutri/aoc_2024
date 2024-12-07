use core::panic;
use std::vec;

use itertools::Itertools;
use num_bigint::{BigUint, RandBigInt, ToBigUint};
use rand::Rng;

fn has_solution(target: &BigUint, current: &BigUint, values: &[BigUint]) -> bool {
    if values.is_empty() {
        return target == current;
    }
    has_solution(target, &(current + &values[0]), &values[1..])
        || has_solution(target, &(current * &values[0]), &values[1..])
}

fn concat_digits(n1: &BigUint, n2: &BigUint) -> BigUint {
    let mut digits = 1;
    let mut temp = n2.clone();
    while temp >= 10.to_biguint().unwrap() {
        digits += 1;
        temp /= 10.to_biguint().unwrap();
    }

    n1 * 10.to_biguint().unwrap().pow(digits) + n2
}

fn has_solution2(target: &BigUint, current: &BigUint, values: &[BigUint]) -> bool {
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
    if has_solution(target, &(current + &values[0]), &values[1..]) {
        return true;
    }
    if has_solution(target, &(current * &values[0]), &values[1..]) {
        return true;
    }
    if has_solution2(target, &concat_digits(current, &values[0]), &values[1..]) {
        return true;
    }
    false
}

fn get_input() -> Vec<(BigUint, Vec<BigUint>)> {
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
            let target = target.parse::<BigUint>().unwrap();
            let values = rest
                .split(" ")
                .map(|v| v.parse::<BigUint>().unwrap())
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
            if has_solution(target, &0.to_biguint().unwrap(), values) {
                Some(target)
            } else {
                None
            }
        })
        .sum::<BigUint>();

    println!("{}", total);
}
pub fn day7_2() {
    let (target, values, ops) = generate_true_cases();
    let target = 139410704293280815u128.to_biguint().unwrap();
    let values = [293, 684, 859, 831, 796, 665, 815].map(|v| v.to_biguint().unwrap());
    let ops = ['|', '*', '*', '+', '*', '|'];
    if has_solution2(&target, &values[0], &values[1..]) {
        println!("success");
    } else {
        println!("failure");
    }
    return;
    let mut current = values[0].clone();
    let mut ops_index = 0;
    for val in &values[1..] {
       let op = ops[ops_index];
        if op == '+' {
            current = current + val;
        } else if op == '*' {
            current = current * val;
        } else if op == '|' {
            current = concat_digits(&current, val);
        } else {
            unreachable!()
        }
        ops_index += 1;
    }
    println!("target: {}, values: {:?}, ops: {:?}, current: {}", target, values, ops, current);
    return;
    loop {
        let (target, values, ops) = generate_true_cases();
        if !has_solution2(&target, &values[0], &values[1..]) {
            panic!("target: {}, numbers: {:?}, ops: {:?}", target, values, ops);
        } else {
            println!("success");
        }
    }
    // let input = get_input();
    // let mut false_counter = 0;
    // let total = input
    //     .iter()
    //     .filter_map(|(target, values)| {
    //         if has_solution2(*target, 0, values) {
    //             Some(*target)
    //         } else {
    //             false_counter += 1;
    //             None
    //         }
    //     })
    //     .sum::<BigUint>();
    //
    // println!("{}", false_counter);
    //
    // println!("{}", total);
}

fn generate_true_cases() -> (BigUint, Vec<BigUint>, Vec<char>) {
    let count = rand::thread_rng().gen_range(2..=10);
    let values = (0..count)
        .map(|_| {
            rand::thread_rng()
                .gen_biguint_range(&1.to_biguint().unwrap(), &1000.to_biguint().unwrap())
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
                concat_digits(&acc, &e)
            } else {
                unreachable!()
            }
        })
        .unwrap();

    (val, values, ops)
}
