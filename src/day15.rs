use crossterm::cursor::MoveTo;
use crossterm::event::read;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::io::stdout;

fn print_map(map: &Vec<Vec<char>>) {
    let mut s = "".to_string();
    for row in map {
        s.push_str(&row.iter().collect::<String>());
        s.push('\n');
    }
    println!("{}", s);
}

fn get_input() -> &'static str {
    let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    let input = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    let input = include_str!("../input/day15_1.txt");

    input
}

pub fn day15_1() {
    let input = get_input();

    let (map, moves) = input.split_once("\n\n").unwrap();

    let mut map = map.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let n = map.len();
    let m = map[0].len();
    let mut pos = map
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find_map(|(j, c)| if *c == '@' { Some((i, j)) } else { None })
        })
        .unwrap();

    for (index, mo) in moves.trim().char_indices() {
        match mo {
            '>' => {
                let mut possible = false;
                for j in pos.1 + 1..m {
                    match map[pos.0][j] {
                        '.' => {
                            possible = true;
                            break;
                        }
                        '#' => {
                            possible = false;
                            break;
                        }
                        'O' => {
                            continue;
                        }
                        _ => unreachable!(),
                    };
                }
                if possible {
                    let mut current = '.';
                    for j in pos.1..m {
                        std::mem::swap(&mut map[pos.0][j], &mut current);
                        if current == '.' {
                            break;
                        }
                    }
                    pos.1 += 1;
                }
            }
            '<' => {
                let mut possible = false;
                for j in (0..pos.1).rev() {
                    match map[pos.0][j] {
                        '.' => {
                            possible = true;
                            break;
                        }
                        '#' => {
                            possible = false;
                            break;
                        }
                        'O' => {
                            continue;
                        }
                        _ => unreachable!(),
                    };
                }
                if possible {
                    let mut current = '.';
                    for j in (0..=pos.1).rev() {
                        std::mem::swap(&mut map[pos.0][j], &mut current);
                        if current == '.' {
                            break;
                        }
                    }
                    pos.1 -= 1;
                }
            }
            '^' => {
                let mut possible = false;
                for i in (0..pos.0).rev() {
                    match map[i][pos.1] {
                        '.' => {
                            possible = true;
                            break;
                        }
                        '#' => {
                            possible = false;
                            break;
                        }
                        'O' => {
                            continue;
                        }
                        _ => unreachable!(),
                    };
                }
                if possible {
                    let mut current = '.';
                    for i in (0..=pos.0).rev() {
                        std::mem::swap(&mut map[i][pos.1], &mut current);
                        if current == '.' {
                            break;
                        }
                    }
                    pos.0 -= 1;
                }
            }
            'v' => {
                let mut possible = false;
                for i in pos.0 + 1..n {
                    match map[i][pos.1] {
                        '.' => {
                            possible = true;
                            break;
                        }
                        '#' => {
                            possible = false;
                            break;
                        }
                        'O' => {
                            continue;
                        }
                        _ => unreachable!(),
                    };
                }
                if possible {
                    let mut current = '.';
                    for i in pos.0..n {
                        std::mem::swap(&mut map[i][pos.1], &mut current);
                        if current == '.' {
                            break;
                        }
                    }
                    pos.0 += 1;
                }
            }
            _ => {}
        }
    }
    print_map(&map);

    let total: usize = map
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, c)| if *c == 'O' { i * 100 + j } else { 0 })
                .sum::<usize>()
        })
        .sum();

    println!("{}", total);
}
pub fn day15_2() {
    let input = get_input();

    let (map, moves) = input.split_once("\n\n").unwrap();

    let mut map = map
        .lines()
        .map(|l| {
            l.chars()
                .flat_map(|c| match c {
                    '#' => ['#', '#'],
                    '.' => ['.', '.'],
                    '@' => ['@', '.'],
                    'O' => ['[', ']'],
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();

    let n = map.len();
    let m = map[0].len();
    let mut pos = map
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find_map(|(j, c)| if *c == '@' { Some((i, j)) } else { None })
        })
        .unwrap();

    for (index, mo) in moves.trim().char_indices() {
        // execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
        // println!("{index}, {mo}");
        // print_map(&map);
        // read();
        let mut make_move = |dir: (i32, i32)| {
            let mut visited = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back(pos);
            visited.insert(pos);
            let mut found = true;

            while !queue.is_empty() {
                let pos = queue.pop_front().unwrap();
                let mut insert_in_dir = |pos: (usize, usize)| {
                    let new_pos = (pos.0 as i32 + dir.0, pos.1 as i32 + dir.1);
                    if new_pos.0 < 0
                        || new_pos.1 < 0
                        || new_pos.0 >= n as i32
                        || new_pos.1 >= m as i32
                    {
                        return true;
                    }
                    let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
                    if !visited.contains(&new_pos) {
                        queue.push_back(new_pos);
                        visited.insert(new_pos);
                    }
                    return false;
                };
                let c = map[pos.0][pos.1];
                match c {
                    '#' => {
                        found = false;
                        break;
                    }
                    '@' => {
                        if insert_in_dir(pos) {
                            found = false;
                            break;
                        }
                    }
                    '.' => {}
                    '[' => {
                        if insert_in_dir(pos) || insert_in_dir((pos.0, pos.1 + 1)) {
                            found = false;
                            break;
                        }
                    }
                    ']' => {
                        if insert_in_dir(pos) || insert_in_dir((pos.0, pos.1 - 1)) {
                            found = false;
                        }
                    }
                    _ => unreachable!(),
                }
            }
            if found {
                let mut queue = VecDeque::new();
                let mut visited = HashSet::new();
                queue.push_back((pos, '.'));
                visited.insert(pos);

                while !queue.is_empty() {
                    let (mut pos, mut c) = queue.pop_front().unwrap();
                    std::mem::swap(&mut map[pos.0][pos.1], &mut c);
                    match c {
                        '.' => {}
                        '@' => {
                            pos.0 = (pos.0 as i32 + dir.0) as usize;
                            pos.1 = (pos.1 as i32 + dir.1) as usize;
                            queue.push_back((pos, c));
                            visited.insert(pos);
                        }
                        '[' => {
                            let mut new_pos = (
                                (pos.0 as i32 + dir.0) as usize,
                                (pos.1 as i32 + dir.1) as usize,
                            );
                            queue.push_back((new_pos, c));
                            visited.insert(new_pos);
                            pos.1 += 1;
                            if !visited.contains(&pos) {
                                queue.push_back((pos, '.'));
                                visited.insert(pos);
                            }
                        }
                        ']' => {
                            let mut new_pos = (
                                (pos.0 as i32 + dir.0) as usize,
                                (pos.1 as i32 + dir.1) as usize,
                            );
                            queue.push_back((new_pos, c));
                            visited.insert(new_pos);
 
                            pos.1 -= 1;
                            if !visited.contains(&pos) {
                                queue.push_back((pos, '.'));
                                visited.insert(pos);
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                pos.0 = (pos.0 as i32 + dir.0) as usize;
                pos.1 = (pos.1 as i32 + dir.1) as usize;
            }
        };
        match mo {
            '>' => make_move((0, 1)),
            '<' => make_move((0, -1)),
            '^' => make_move((-1, 0)),
            'v' => make_move((1, 0)),
            _ => {}
        }

    }
    
    print_map(&map);

    let total: usize = map
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, c)| if *c == '[' { i * 100 + j } else { 0 })
                .sum::<usize>()
        })
        .sum();

    println!("{}", total);
}
