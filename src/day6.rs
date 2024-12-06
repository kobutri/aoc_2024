use crate::day6::Dir::*;
use itertools::Itertools;

enum Dir {
    l,
    r,
    u,
    d,
}

impl Dir {
    fn to_char(&self) -> char {
        match self {
            l => 'l',
            r => 'r',
            u => 'u',
            d => 'd',
        }
    }
}

fn get_input() -> Vec<Vec<char>> {
    let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    let input = include_str!("../input/day6_1.txt");

    input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>()
}

fn get_loop_steps(mut input: Vec<Vec<char>>) -> Option<i32> {
    let n = input.len() as i32;
    let m = input[0].len() as i32;
    let mut dir = u;
    let mut total = 1;
    let (mut y, mut x) = (0..n)
        .cartesian_product(0..m)
        .find(|(i, j)| {
            let c = input[*i as usize][*j as usize];
            if c == '^' {
                input[*i as usize][*j as usize] = 'X';
                return true;
            } else {
                return false;
            }
        })
        .unwrap();
    loop {
        let (ny, nx) = match dir {
            u => (y - 1, x),
            r => (y, x + 1),
            d => (y + 1, x),
            l => (y, x - 1),
        };
        if ny >= n || ny < 0 || nx >= m || nx < 0 {
            break;
        }
        let c = input[ny as usize][nx as usize];
        if c == '#' {
            dir = match dir {
                u => r,
                r => d,
                d => l,
                l => u,
            };
        } else if dir.to_char() == c {
            return None;
        } else {
            x = nx;
            y = ny;
            if c == '.' {
                total += 1;
                input[ny as usize][nx as usize] = dir.to_char();
            }
        }
        // if i % 10 == 0 || true {
        //     for i in 0..n {
        //         for j in 0..m {
        //             if i != y || j != x {
        //                 print!("{}", input[i as usize][j as usize]);
        //             } else {
        //                 print!("{}", match dir {
        //                     l => 'l',
        //                     r => 'r',
        //                     u => 'u',
        //                     d => 'd',
        //                 });
        //             }
        //         }
        //         println!();
        //     }
        //     println!("\n");
        // }
    }
    return Some(total);
}

pub fn day6_1() {
    println!("{}", get_loop_steps(get_input()).unwrap());
}
pub fn day6_2() {
    let input = get_input();
    let n = input.len();
    let m = input[0].len();

    let mut total = 0;
    for i in 0..n {
        for j in 0..m {
            if input[i][j] == '.' {
                let mut new_input = input.clone();
                new_input[i][j] = '#';
                if get_loop_steps(new_input).is_none() {
                    total += 1;
                }
            }
        }
    }
    println!("{}", total);
}
