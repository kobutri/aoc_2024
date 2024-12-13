use std::collections::{HashMap, HashSet, VecDeque};
use enumset::{EnumSet, EnumSetType};
use itertools::Itertools;

fn get_input() -> Vec<Vec<i32>> {
    let input = "AAAA
BBCD
BBCC
EEEC";
    let input = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
    let input = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
    let input = include_str!("../input/day12_1.txt");
    let input = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let n = input.len() as i32;
    let m = input[0].len() as i32;

    let mut fields = vec![vec![0; m as usize]; n as usize];

    let mut max = 0;
    for i in 0..n {
        for j in 0..m {
            let c = input[i as usize][j as usize];
            let field = fields[i as usize][j as usize];
            if field != 0 {
                continue;
            }
            max += 1;
            fields[i as usize][j as usize] = max;
            let mut queue = VecDeque::new();
            queue.push_back((i, j));
            while !queue.is_empty() {
                let (i, j) = queue.pop_front().unwrap();
                [(-1, 0), (1, 0), (0, -1), (0, 1)].into_iter().for_each(|(k, l)| {
                    let i = k + i;
                    let j = l + j;
                    if !(i < 0 || i >= fields.len() as i32 || j < 0 || j >= fields[i as usize].len() as i32 || fields[i as usize][j as usize] != 0) && input[i as usize][j as usize] == c {
                        fields[i as usize][j as usize] = max;
                        queue.push_back((i, j));
                    }
                });
            }
        }
    }

    fields
}

pub fn day12_1() {
    let fields = get_input();
    let n = fields.len() as i32;
    let m = fields[0].len() as i32;
    let mut areas = HashMap::new();
    let mut perimeters = HashMap::new();
    let mut fields_set = HashSet::new();
    for i in 0..n {
        for j in 0..m {
            let field = fields[i as usize][j as usize];
            fields_set.insert(field);
            *areas.entry(field).or_insert(0) += 1;
            *perimeters.entry(field).or_insert(0) += [(-1, 0), (1, 0), (0, -1), (0, 1)].into_iter().filter(|(k, l)| {
                let i = k + i;
                let j = l + j;
                i < 0 || i >= fields.len() as i32 || j < 0 || j >= fields[i as usize].len() as i32 || fields[i as usize][j as usize] != field
            }).count();
        }
    }
    // println!("areas: {:?}", areas);
    // println!("perimeters: {:?}", perimeters);

    let mut total = 0;
    for field in fields_set {
        // println!("{field}, area: {}, perimeter: {}", areas.get(&field).unwrap(), perimeters.get(&field).unwrap());
        total += areas.get(&field).unwrap() * perimeters.get(&field).unwrap()
    }
    println!("{}", total);
}
#[derive(EnumSetType, Debug)]
pub enum Dir {
    P, M
}
pub fn day12_2() {
    let fields = get_input();
    let n = fields.len() as i32;
    let m = fields[0].len() as i32;
    let mut areas = HashMap::new();
    let mut fields_set = HashSet::new();
    for i in 0..n {
        for j in 0..m {
            let field = fields[i as usize][j as usize];
            fields_set.insert(field);
            *areas.entry(field).or_insert(0) += 1;
        }
    }

    let mut perimeters = HashMap::new();
    for i in 0..n {
        let mut last = 0;
        let mut last_dir = EnumSet::empty();
        for j in 0..m {
            let field = fields[i as usize][j as usize];
            let dir = [-1, 1].into_iter().filter_map(|k| {
                if fields.get((i + k) as usize).and_then(|row| row.get(j as usize).map(|field2| field != *field2)).unwrap_or(true) {
                    Some(match k {
                        1 => Dir::P,
                        -1 => Dir::M,
                        _ => unreachable!()
                    })
                } else {
                    None
                }
            }).fold(EnumSet::empty(), |acc, v| {
                acc | v
            });
            if last != field {
                last_dir = EnumSet::empty();
                last = field;
            }
            *perimeters.entry(field).or_insert(0) += dir.difference(last_dir).len();
            last_dir = dir;
        }
    }


    for j in 0..m {
        let mut last = 0;
        let mut last_dir = EnumSet::empty();
        for i in 0..n {
            let field = fields[i as usize][j as usize];
            let dir = [-1, 1].into_iter().filter_map(|k| {
                if fields.get(i as usize).and_then(|row| row.get((j + k) as usize).map(|field2| field != *field2)).unwrap_or(true) {
                    Some(match k {
                        1 => Dir::P,
                        -1 => Dir::M,
                        _ => unreachable!()
                    })
                } else {
                    None
                }
            }).fold(EnumSet::empty(), |acc, v| {
                acc | v
            });
            if last != field {
                last_dir = EnumSet::empty();
                last = field;
            }
            *perimeters.entry(field).or_insert(0) += dir.difference(last_dir).len();
            last_dir = dir;
        }
    }


    let mut total = 0;
    for field in fields_set {
        total += areas.get(&field).unwrap() * perimeters.get(&field).unwrap()
    }
    println!("{}", total);
}
