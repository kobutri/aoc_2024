use itertools::Itertools;
use regex::{Regex, RegexSet};

#[derive(Debug)]
struct Input {
    ax: f64,
    bx: f64,
    ay: f64,
    by: f64,
    px: f64,
    py: f64,
}

fn get_input() -> Vec<Input> {
    let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    let input = include_str!("../input/day13_1.txt");

    let regex = Regex::new(r"Button A: X\+(?<ax>\d+), Y\+(?<ay>\d+)\nButton B: X\+(?<bx>\d+), Y\+(?<by>\d+)\nPrize: X=(?<px>\d+), Y=(?<py>\d+)").unwrap();
    let inputs = regex.captures_iter(input).map(|captures| {
        Input {
            ax: captures.name("ax").unwrap().as_str().parse().unwrap(),
            bx: captures.name("bx").unwrap().as_str().parse().unwrap(),
            ay: captures.name("ay").unwrap().as_str().parse().unwrap(),
            by: captures.name("by").unwrap().as_str().parse().unwrap(),
            px: captures.name("px").unwrap().as_str().parse().unwrap(),
            py: captures.name("py").unwrap().as_str().parse().unwrap(),
        }
    }).collect_vec();
    
    inputs
}

fn solve(inputs: &[Input]) {
    let mut cost = vec![];
    for input in inputs {
        let Input { ax, bx, ay, by, px, py } = *input;
        let ny = py/ay;
        let bny = -by/ay;
        let nx = px - ny*ax;
        let bnx = bx + bny*ax;
        let b = nx / bnx;
        let a = (py - b*by) / ay;
        let a = a.round() as i128;
        let b= b.round() as i128;
        if a * ax as i128 + b * bx as i128 == px as i128 && a * ay as i128 + b * by as i128 == py as i128 {
            cost.push(3*a+b);
        }
    }
    println!("total: {}", cost.iter().sum::<i128>());
}

pub fn day13_1() {
  solve(&get_input())

}
pub fn day13_2() {
    let mut inputs = get_input();
    inputs.iter_mut().for_each(|input| {
        input.px += 10000000000000.0;
        input.py += 10000000000000.0;
    });
    solve(&inputs);
}