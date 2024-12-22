use itertools::Itertools;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

fn generate(mut secret: i64) -> i64 {
    let x = secret * 64;
    secret ^= x;
    secret %= 16777216;
    let x = secret / 32;
    secret ^= x;
    secret %= 16777216;
    let x = secret * 2048;
    secret ^= x;
    secret %= 16777216;
    secret
}

fn get_price(secret: i64) -> i64 {
    secret % 10
}

fn get_input() -> Vec<i64> {
    let input = "1
2
3
2024";

    let input = include_str!("../input/day22_1.txt");

    let input = input
        .lines()
        .map(|l| l.trim().parse().unwrap())
        .collect_vec();

    input
}

pub fn day22_1() {
    let input = get_input();
    let mut total = 0;
    for secret in &input {
        let mut secret = *secret;
        for _ in 0..2000 {
            secret = generate(secret);
        }
        total += secret;
    }
    println!("{}", total);
}
pub fn day22_2() {
    let mut best_prices = HashMap::new();
    let input = get_input();
    for secret in &input {
        let mut secret = *secret;
        let map = (0..2000)
            .map(|_| {
                let new_secret = generate(secret);
                let price_change = (new_secret % 10) - (secret % 10);
                secret = new_secret;
                (secret % 10, price_change)
            })
            .tuple_windows::<(_, _, _, _)>()
            .fold(HashMap::new(), |mut map, val| {
                let entry = map.entry((val.0 .1, val.1 .1, val.2 .1, val.3 .1));
                match entry {
                    Entry::Occupied(_) => {}
                    Entry::Vacant(entry) => {
                        entry.insert(val.3 .0);
                    }
                }
                map
            });
        map.into_iter().for_each(|(key, val)| {
            *best_prices.entry(key).or_insert(0) += val;
        })
    }
    let best = best_prices.into_iter().max_by_key(|(_, v)| *v).unwrap();
    println!("{:?}", best);
}
