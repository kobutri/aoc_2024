use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

fn get_input() -> Vec<(usize, Vec<char>)> {
    let input = "029A
980A
179A
456A
379A";
    let input = include_str!("../input/day21_1.txt");
    let regex = Regex::new(r"\d+").unwrap();
    let input = input
        .lines()
        .map(|line| {
            let cap = regex.captures(line).unwrap();
            let num = cap.get(0).unwrap().as_str().parse::<usize>().unwrap();
            (num, line.chars().collect_vec())
        })
        .collect_vec();
    input
}

fn numeric_to_coordinate(c: char) -> (i32, i32) {
    match c {
        '7' => (0, 0),
        '8' => (0, 1),
        '9' => (0, 2),
        '4' => (1, 0),
        '5' => (1, 1),
        '6' => (1, 2),
        '1' => (2, 0),
        '2' => (2, 1),
        '3' => (2, 2),
        '0' => (3, 1),
        'A' => (3, 2),
        _ => unreachable!(),
    }
}

fn directional_to_coordinate(c: char) -> (i32, i32) {
    match c {
        '^' => (0, 1),
        'A' => (0, 2),
        '<' => (1, 0),
        'v' => (1, 1),
        '>' => (1, 2),
        _ => unreachable!(),
    }
}

fn coordinate_to_directional(coord: (i32, i32)) -> char {
    match coord {
        (0, 1) => '^',
        (0, 2) => 'A',
        (1, 0) => '<',
        (1, 1) => 'v',
        (1, 2) => '>',
        _ => unreachable!(),
    }
}

fn coordinate_to_numeric(coord: (i32, i32)) -> char {
    match coord {
        (0, 0) => '7',
        (0, 1) => '8',
        (0, 2) => '9',
        (1, 0) => '4',
        (1, 1) => '5',
        (1, 2) => '6',
        (2, 0) => '1',
        (2, 1) => '2',
        (2, 2) => '3',
        (3, 1) => '0',
        (3, 2) => 'A',
        _ => unreachable!(),
    }
}

fn simulate_directional(input: Vec<char>) -> Vec<char> {
    let mut current_coord = directional_to_coordinate('A');
    let mut output = vec![];
    for s in input {
        match s {
            'A' => output.push(coordinate_to_directional(current_coord)),
            '>' => current_coord.1 += 1,
            'v' => current_coord.0 += 1,
            '<' => current_coord.1 -= 1,
            '^' => current_coord.0 -= 1,
            _ => unreachable!(),
        }
    }
    output
}

fn simulate_numeric(input: Vec<char>) -> Vec<char> {
    let mut current_coord = numeric_to_coordinate('A');
    let mut output = vec![];
    for s in input {
        match s {
            'A' => output.push(coordinate_to_numeric(current_coord)),
            '>' => current_coord.1 += 1,
            'v' => current_coord.0 += 1,
            '<' => current_coord.1 -= 1,
            '^' => current_coord.0 -= 1,
            _ => unreachable!(),
        }
    }
    output
}

fn nex_possible_steps_numeric(current: char, target: char) -> Vec<char> {
    let current_coord = numeric_to_coordinate(current);
    let target_coord = numeric_to_coordinate(target);
    let lr = (target_coord.1 - current_coord.1).signum();
    let ud = (target_coord.0 - current_coord.0).signum();
    let mut vs = vec![];
    if (current_coord.0 + ud, current_coord.1) != (3, 0) {
        match ud {
            1 => vs.push('v'),
            -1 => vs.push('^'),
            _ => {}
        }
    }
    if (current_coord.0, current_coord.1 + lr) != (3, 0) {
        match lr {
            1 => vs.push('>'),
            -1 => vs.push('<'),
            _ => {}
        }
    }
    vs
}

fn apply_dir_directional(current: char, dir: char) -> char {
    let mut current_coord = directional_to_coordinate(current);
    match dir {
        '>' => current_coord.1 += 1,
        '<' => current_coord.1 -= 1,
        '^' => current_coord.0 -= 1,
        'v' => current_coord.0 += 1,
        _ => unreachable!(),
    };

    coordinate_to_directional(current_coord)
}

fn apply_dir_numeric(current: char, dir: char) -> char {
    let mut current_coord = numeric_to_coordinate(current);
    match dir {
        '>' => current_coord.1 += 1,
        '<' => current_coord.1 -= 1,
        '^' => current_coord.0 -= 1,
        'v' => current_coord.0 += 1,
        _ => unreachable!(),
    };

    coordinate_to_numeric(current_coord)
}

fn nex_possible_steps_directional(current: char, target: char) -> Vec<char> {
    let current_coord = directional_to_coordinate(current);
    let target_coord = directional_to_coordinate(target);
    let lr = (target_coord.1 - current_coord.1).signum();
    let ud = (target_coord.0 - current_coord.0).signum();
    let mut vs = vec![];
    if (current_coord.0 + ud, current_coord.1) != (0, 0) {
        match ud {
            1 => vs.push('v'),
            -1 => vs.push('^'),
            _ => {}
        }
    }
    if (current_coord.0, current_coord.1 + lr) != (0, 0) {
        match lr {
            1 => vs.push('>'),
            -1 => vs.push('<'),
            _ => {}
        }
    }
    vs
}

fn count_steps(current: char, target: char) -> usize {
    let current2 = 'A';
    let current3 = 'A';

    todo!()
}

fn count_steps_numerical(
    current: char,
    currents: &mut [char],
    target: char,
    map: &mut HashMap<(char, Vec<char>, char), usize>,
) ->usize {
    if let Some(count) = map.get(&(current, currents.to_vec(), target)) {
        return *count;
    }
    let count = if current == target {
        count_steps_directional(currents[0], &mut currents[1..], 'A', map)
    } else {
        nex_possible_steps_numeric(current, target)
            .into_iter()
            .map(|d| {
                let next = apply_dir_numeric(current, d);
                let mut v = count_steps_directional(currents[0], &mut currents[1..], d, map);
                let temp = currents[0];
                currents[0] = d;
                v += count_steps_numerical(next, currents, target, map);
                currents[0] = temp;
                v
            })
            .min()
            .unwrap()
    };
    map.insert((current, currents.to_vec(), target), count.clone());
    return count;
}

fn count_steps_directional(
    current: char,
    currents: &mut [char],
    target: char,
    map: &mut HashMap<(char, Vec<char>, char), usize>,
) -> usize {
    // if currents.len() == 0 && '>' == current && target == 'A' {
    //     println!("here");
    // }
    if let Some(count) = map.get(&(current, currents.to_vec(), target)) {
        return *count;
    }
    let count = if current == target {
        if currents.is_empty() {
            1
        } else {
            count_steps_directional(currents[0], &mut currents[1..], 'A', map)
        }
    } else {
        nex_possible_steps_directional(current, target)
            .into_iter()
            .map(|dir| {
                let next = apply_dir_directional(current, dir);
                let mut v = if currents.is_empty() {
                    1
                } else {
                    count_steps_directional(currents[0], &mut currents[1..], dir, map)
                };
                if !currents.is_empty() {
                    let temp = currents[0];
                    currents[0] = dir;
                    v += count_steps_directional(next, currents, target, map);
                    currents[0] = temp;
                } else {
                    v += count_steps_directional(next, currents, target, map);
                }
                v
            })
            .min()
            .unwrap()
    };

    map.insert((current, currents.to_vec(), target), count.clone());
    return count;
}


pub fn day21_1() {
    let mut map = HashMap::new();
    let input = get_input();
    let mut total = 0;
    for (num, input) in input {
        let mut current1 = 'A';
        let mut seq = 0;
        for s in input {
            seq += count_steps_numerical(
                current1, &mut ['A', 'A'], s, &mut map,
            );
            current1 = s;
        }
        total += num * seq;
        println!("{}", seq);
    }
    println!("{}", total);
}

pub fn day21_2() {
    let mut map = HashMap::new();
    let input = get_input();
    let mut total = 0;
    for (num, input) in input {
        let mut current1 = 'A';
        let mut seq = 0;
        for s in input {
            seq += count_steps_numerical(
                current1, &mut ['A';25], s, &mut map,
            );
            current1 = s;
        }
        total += num * seq;
    }
    println!("{}", total);

}
