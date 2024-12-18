use crate::day17::Res::Halt;
use itertools::Itertools;
use regex::Regex;

struct Input {
    a: usize,
    b: usize,
    c: usize,
    instructions: Vec<usize>,
}

struct Program<'a> {
    registers: [usize; 7],
    instructions: &'a [usize],
    i_p: usize,
}

enum Res {
    None,
    Halt,
    Output(usize),
}

fn simulate2(mut a: usize) -> impl Iterator<Item = usize> {
    std::iter::from_fn(move || {
        if a == 0 {
            return None;
        }
        let mut b = a % 8;
        b = b ^ 2;
        let mut c = a / 2usize.pow(b as u32);
        b = b ^ c;
        b = b ^ 3;
        a = a / 8;
        Some(b % 8)
    })
    .chain(std::iter::once(0))
}

fn find(instructions: &[usize]) -> Option<usize> {
    let target = instructions.iter().rev().collect_vec();
    let mut queue = vec![(0, 0)];
    let mut max = 0;
    while !queue.is_empty() {
        let (val, count) = queue.pop().unwrap();
        if count == target.len() {
            return Some(val);
        }
        for i in 0..=7 {
            let mut outputs = vec![];
            let val = (val << 3) + i;
            let mut a = val;

            loop {
                let mut b = a % 8;
                b = b ^ 2;
                let c = a / 2usize.pow(b as u32);
                b = b ^ c;
                b = b ^ 3;
                outputs.push(b % 8);
                a = a / 8;
                if a == 0 {
                    break;
                }
            }
            if outputs
                .iter()
                .rev()
                .zip(target.iter())
                .all(|(a, b)| *a == **b)
            {
                if outputs.len() > max {
                    max = outputs.len();
                }
                queue.push((val, outputs.len()));
            }
        }
    }
    None
}

impl<'a> Program<'a> {
    fn new(input: &'a Input) -> Self {
        Self {
            registers: [0, 1, 2, 3, input.a, input.b, input.c],
            instructions: &input.instructions,
            i_p: 0,
        }
    }

    fn step(&mut self) -> Res {
        if self.i_p >= self.instructions.len() - 1 {
            return Halt;
        }
        let Self {
            registers,
            instructions,
            i_p,
        } = self;
        let instruction = instructions[*i_p];
        let op_idx = instructions[*i_p + 1];
        let operand = registers[op_idx as usize];
        match instruction {
            0 => {
                registers[4] = registers[4] / 2usize.pow(operand as u32);
                *i_p += 2;
            }
            1 => {
                registers[5] = registers[5] ^ operand;
                *i_p += 2;
            }
            2 => {
                registers[5] = operand % 8;
                *i_p += 2;
            }
            3 => {
                if registers[4] == 0 {
                    *i_p += 2;
                } else {
                    *i_p = operand as usize;
                }
            }
            4 => {
                registers[5] = registers[5] ^ registers[6];
                *i_p += 2;
            }
            5 => {
                *i_p += 2;
                return Res::Output(operand % 8);
            }
            6 => {
                registers[5] = registers[4] / 2usize.pow(operand as u32);
                *i_p += 2;
            }
            7 => {
                registers[6] = registers[4] / 2usize.pow(operand as u32);
                *i_p += 2;
            }
            _ => unreachable!(),
        }
        Res::None
    }
}

fn get_input() -> Input {
    let input = "Register A: 117440
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    let input = include_str!("../input/day17_1.txt");

    let regex = Regex::new(
        r"(Register A: (?<a>\d+))\n(Register B: (?<b>\d+))\n(Register C: (?<c>\d+))\n\n(Program: (?<p>(\d+,)*\d+))",
    )
        .unwrap();
    let caps = regex.captures(input).unwrap();
    let a = caps["a"].parse::<usize>().unwrap();
    let b = caps["b"].parse::<usize>().unwrap();
    let c = caps["c"].parse::<usize>().unwrap();
    let instructions = caps["p"]
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();

    Input {
        a,
        b,
        c,
        instructions,
    }
}

pub fn day17_1() {
    let input = get_input();
    let mut program = Program::new(&input);
    let mut output = vec![];
    loop {
        match program.step() {
            Res::None => {}
            Halt => {
                break;
            }
            Res::Output(i) => {
                output.push(i);
            }
        }
    }
    println!("{}", output.iter().map(|i| i.to_string()).join(","));
}
pub fn day17_2() {
    let mut input = get_input();

    println!("{:?}", find(&input.instructions).unwrap());
}
