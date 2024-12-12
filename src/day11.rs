use std::collections::{HashMap, VecDeque};
use itertools::Itertools;
use num_bigint::{BigUint, ToBigUint};
use num_traits::One;

fn solve(count: i32) {
    let input = "125 17";
    let input = include_str!("../input/day11_1.txt");
    let input = input.split(" ").map(|s| s.trim().parse::<u128>().unwrap()).collect_vec();
    let mut input = input.into_iter().fold(HashMap::new(), |mut map, val| {
        *map.entry(val).or_insert(0) += 1;
        map
    });
    for i in 0..count {
        input = input.into_iter().fold(HashMap::new(), |mut map, (x, count)| {
            if x == 0 {
                *map.entry(1).or_insert(0) += count;
            } else {
                let mut digits = 1;
                let mut y = x;
                while y >= 10 {
                    y /= 10;
                    digits += 1;
                }
                if digits % 2 == 0 {
                    let factor = 10u128.pow(digits / 2);
                    let left = x / factor;
                    let right = x % factor;
                    *map.entry(left).or_insert(0) += count;
                    *map.entry(right).or_insert(0) += count;
                } else {
                    *map.entry(x * 2024).or_insert(0) += count;
                }
            }

            map
        });
        // println!("{}, {}, {:?}", i+1, input.values().sum::<u128>(), input);
    }

    println!("{}", input.values().sum::<u128>());
}
pub fn day11_1() {
    solve(25);
}

pub fn day11_2() {
    solve(75);
}
