use std::char;
use std::collections::HashSet;
use image::{GrayImage, Luma};
use itertools::{Itertools, Position};
use regex::Regex;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Input {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}

fn print_positions(positions: &[Input], width: i32, height: i32) -> GrayImage {
    let mut board = vec![vec![0; width as usize]; height as usize];
    for pos in positions {
        board[pos.py as usize][pos.px as usize] += 1;
    }
    let mut img = GrayImage::new(width as u32, height as u32);
    for (y, row) in board.iter().enumerate() {
        for (x, &value) in row.iter().enumerate() {
            let pixel = if value == 0 {
                Luma([0]) // Black
            } else {
                Luma([255]) // White
            };
            img.put_pixel(x as u32, y as u32, pixel);
        }
    }
    img
}

pub fn day14_1() {
    let positions = simulate(100, false);
    let width = 101;
    let height = 103;
    let mut quadrants = [0; 4];
    for pos in &positions {
        if pos.px < width / 2 {
            if pos.py < height / 2 {
                quadrants[0] += 1;
            } else if pos.py > height / 2 {
                quadrants[1] += 1;
            }
        } else if pos.px > width / 2 {
            if pos.py < height / 2 {
                quadrants[2] += 1;
            } else if pos.py > height / 2 {
                quadrants[3] += 1;
            }
        }
    }
    let total = quadrants.iter().product::<i32>();
    // print_positions(&positions, width, height);
    println!("{}", total);
}

fn simulate(steps: i32, print: bool) -> Vec<Input> {
    let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    let input = include_str!("../input/day14_1.txt");
    let regex = Regex::new(r"p=(?<px>-?\d+),(?<py>-?\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)").unwrap();
    let mut positions = regex
        .captures_iter(&input)
        .map(|c| Input {
            px: c.name("px").unwrap().as_str().parse().unwrap(),
            py: c.name("py").unwrap().as_str().parse().unwrap(),
            vx: c.name("vx").unwrap().as_str().parse().unwrap(),
            vy: c.name("vy").unwrap().as_str().parse().unwrap(),
        })
        .collect_vec();

    let width = 101;
    let height = 103;
    let mut set = HashSet::new();
    for i in 0..steps {
        for pos in &mut positions {
            pos.px = (pos.px + pos.vx).rem_euclid(width);
            pos.py = (pos.py + pos.vy).rem_euclid(height);
        }
       if print {
           if set.contains(&positions) {
               println!("already seen this: {}", i+1);
           } else {
               set.insert(positions.clone());
           }
           print_positions(&positions, width, height).save(format!("output/output_{i}.png")).unwrap();
       }
    }

    positions
}
pub fn day14_2() {
    simulate(10403, true);
}
