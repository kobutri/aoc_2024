use std::collections::{HashMap, HashSet};
use itertools::Itertools;

fn get_input() -> (Vec<Vec<u8>>, [Vec<(i32, i32)>; 10]) {
    let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    let input = include_str!("../input/day10_1.txt");

    let input = input
        .lines()
        .map(|l| {
            l.trim().chars()
                .map(|c| {
                    if c.to_digit(10).is_none() {
                        println!("{}", c as u8);
                    }
                    c.to_digit(10).unwrap() as u8
                })
                .collect_vec()
        })
        .collect_vec();
    let mut positions = std::array::from_fn::<_, 10, _>(|_| Vec::new());
    for (i, vs) in input.iter().enumerate() {
        for (j, h) in vs.iter().enumerate() {
            positions[*h as usize].push((i as i32, j as i32));
        }
    }

    (input, positions)  
}

pub fn day10_1() {
    let (input, positions) = get_input();



    let n = input.len();
    let m = input[0].len();

    let mut ds = vec![vec![HashSet::new(); m]; n];

    let mut global_total = 0;
    for h in (0..10).rev() {
        for (i, j) in &positions[h] {
            if h == 9 {
                ds[*i as usize][*j as usize].insert((i, j));
            } else {
                let is = [*i+1, *i-1].into_iter().filter(|&i| i >= 0 && i < n as i32).collect_vec();
                let js = [*j+1, *j-1].into_iter().filter(|&j| j >= 0 && j < m as i32).collect_vec();

                let mut total = HashSet::new();
                for i in is {
                    if input[i as usize][*j as usize] as usize == h + 1 {
                       total = total.union(&ds[i as usize][*j as usize]).copied().collect();
                    }
                }
                for j in js {
                    if input[*i as usize][j as usize] as usize == h + 1 {
                        total = total.union(&ds[*i as usize][j as usize]).copied().collect();
                    }
                }
                if h == 0 {
                    global_total += total.len();
                }
                ds[*i as usize][*j as usize] = total;
            }
        }
    }
    println!("{}", global_total);
}
pub fn day10_2() {
    let (input, positions) = get_input();

    let n = input.len();
    let m = input[0].len();
    
    let mut ds = vec![vec![0; m]; n];

    let mut global_total = 0;
    for h in (0..10).rev() {
        for (i, j) in &positions[h] {
            if h == 9 {
                ds[*i as usize][*j as usize] = 1;
            } else {
                let is = [*i+1, *i-1].into_iter().filter(|&i| i >= 0 && i < n as i32).collect_vec();
                let js = [*j+1, *j-1].into_iter().filter(|&j| j >= 0 && j < m as i32).collect_vec();

                let mut total = 0;
                for i in is {
                    if input[i as usize][*j as usize] as usize == h + 1 {
                        total += ds[i as usize][*j as usize];
                    }
                }
                for j in js {
                    if input[*i as usize][j as usize] as usize == h + 1 {
                        total += ds[*i as usize][j as usize];
                    }
                }
                if h == 0 {
                    global_total += total;
                }
                ds[*i as usize][*j as usize] = total;
            }
        }
    }
    println!("{}", global_total);
}
