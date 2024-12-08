use std::{
    collections::{HashMap, HashSet},
    mem::swap,
};

use itertools::Itertools;

fn gcd(mut a: i32, mut b: i32) -> i32 {
    assert!(a >= 0 && b >= 0);
    if b > a {
        swap(&mut a, &mut b);
    }
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    return a;
}

fn get_input() -> (
    i32,
    i32,
    impl Fn(i32, i32) -> bool,
    HashMap<char, Vec<(i32, i32)>>,
) {
    let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    let input = include_str!("../input/day8_1.txt");

    let n = input.lines().count() as i32;
    let m = input.lines().next().unwrap().chars().count() as i32;

    let valid = move |i: i32, j: i32| i >= 0 && i < n && j >= 0 && j < m;
    let mut antennas = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.char_indices() {
            if c.is_ascii_alphanumeric() {
                antennas
                    .entry(c)
                    .or_insert(vec![])
                    .push((i as i32, j as i32));
            }
        }
    }

    (n, m, valid, antennas)
}

pub fn day8_1() {
    let (n, m, valid, antennas) = get_input();
    let mut locations = HashSet::new();

    for positions in antennas.values() {
        positions
            .iter()
            .cartesian_product(positions.iter())
            .for_each(|(pos1, pos2)| {
                if pos1 == pos2 {
                    return;
                }
                let d_x = pos2.1 - pos1.1;
                let d_y = pos2.0 - pos1.0;

                let (n1_y, n1_x) = (pos1.0 + 2 * d_y, pos1.1 + 2 * d_x);
                let (n2_y, n2_x) = (pos1.0 - d_y, pos1.1 - d_x);

                if valid(n1_y, n1_x) {
                    locations.insert((n1_y, n1_x));
                }
                if valid(n2_y, n2_x) {
                    locations.insert((n2_y, n2_x));
                }
            });
    }
    println!("{}", locations.len());
}
pub fn day8_2() {
    let (n, m, valid, antennas) = get_input();
    let mut locations = HashSet::new();

    for positions in antennas.values() {
        positions
            .iter()
            .cartesian_product(positions.iter())
            .for_each(|(pos1, pos2)| {
                if pos1 == pos2 {
                    return;
                }
                let d_x = pos2.1 - pos1.1;
                let d_y = pos2.0 - pos1.0;
                let divisor = gcd(d_y.abs(), d_x.abs());
                let d_x = d_x / divisor;
                let d_y = d_y / divisor;
                let mut current = *pos1;
                while valid(current.0, current.1) {
                    locations.insert(current);
                    current.0 -= d_y;
                    current.1 -= d_x;
                }
                let mut current = *pos1;
                while valid(current.0, current.1) {
                    locations.insert(current);
                    current.0 += d_y;
                    current.1 += d_x;
                }
            });
    }

    println!("{}", locations.len());
}
