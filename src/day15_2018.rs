use itertools::Itertools;
use pheap::PairingHeap;
use std::array::IntoIter;
use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};
use std::iter::{successors, FilterMap};

#[derive(Copy, Clone)]
enum Field {
    Goblin(i32, i32),
    Elf(i32, i32),
    Wall,
    Empty,
}

impl Field {
    fn is_opposite(&self, other: &Field) -> bool {
        match self {
            Field::Goblin(_, _) => matches!(other, Field::Elf(_, _)),
            Field::Elf(_, _) => matches!(other, Field::Goblin(_, _)),
            _ => false,
        }
    }

    fn is_same(&self, other: &Field) -> bool {
        match self {
            Field::Goblin(_, _) => matches!(other, Field::Goblin(_, _)),
            Field::Elf(_, _) => matches!(other, Field::Elf(_, _)),
            Field::Wall => matches!(other, Field::Wall),
            Field::Empty => matches!(other, Field::Empty),
        }
    }

    fn is_character(&self) -> bool {
        self.is_goblin() || self.is_elf()
    }

    fn is_goblin(&self) -> bool {
        matches!(self, Field::Goblin(_, _))
    }

    fn is_elf(&self) -> bool {
        matches!(self, Field::Elf(_, _))
    }

    fn is_empty(&self) -> bool {
        matches!(self, Field::Empty)
    }

    fn is_wall(&self) -> bool {
        matches!(self, Field::Wall)
    }

    fn get_count(&self) -> i32 {
        match self {
            Field::Goblin(_, count) => *count,
            Field::Elf(_, count) => *count,
            Field::Wall => 0,
            Field::Empty => 0,
        }
    }

    fn increase_count(&mut self) {
        match self {
            Field::Goblin(_, count) | Field::Elf(_, count) => {
                *count += 1;
            }
            _ => {}
        }
    }

    fn to_char(&self) -> char {
        match self {
            Field::Goblin(x, _) => 'G',
            Field::Elf(x, _) => 'E',
            Field::Wall => '#',
            Field::Empty => '.',
        }
    }

    fn get_health_mut(&mut self) -> &mut i32 {
        match self {
            Field::Goblin(h, _) => h,
            Field::Elf(h, _) => h,
            _ => unreachable!(),
        }
    }

    fn get_health(&self) -> i32 {
        match self {
            Field::Goblin(h, _) => *h,
            Field::Elf(h, _) => *h,
            _ => unreachable!(),
        }
    }
}

fn is_enemy_adjacent(fields: &Vec<Vec<Field>>, pos: (i32, i32)) -> bool {
    let here = fields[pos.0 as usize][pos.1 as usize];
    adjacent_fields(fields, pos).any(|(_, _, other)| here.is_opposite(&other))
}

fn adjacent_fields<'a>(
    fields: &'a Vec<Vec<Field>>,
    pos: (i32, i32),
) -> impl Iterator<Item = (i32, i32, Field)> + use<'a> {
    [(-1, 0), (0, -1), (0, 1), (1, 0)]
        .into_iter()
        .filter_map(move |offset| {
            fields
                .get((pos.0 + offset.0) as usize)
                .and_then(|row| row.get((pos.1 + offset.1) as usize))
                .map(|&field| (pos.0 + offset.0, pos.1 + offset.1, field))
        })
}

fn towards_min_distance(
    fields: &Vec<Vec<Field>>,
    empty_positions: &HashSet<(i32, i32)>,
    empty_target_positions: &HashSet<(i32, i32)>,
    pos: (i32, i32),
) -> Option<(i32, i32)> {
    let mut distance_cache = fields
        .iter()
        .map(|row| row.iter().map(|_| i32::MAX).collect_vec())
        .collect_vec();
    let mut queue = PairingHeap::new();
    distance_cache[pos.0 as usize][pos.1 as usize] = 0;
    queue.insert(pos, 0);
    let mut target_distance: Option<i32> = None;
    let mut minimal_targets = HashSet::new();
    while !queue.is_empty() {
        let (pos, prio) = queue.delete_min().unwrap();
        if let Some(dist) = target_distance {
            if prio > dist {
                break;
            }
        }
        adjacent_fields(fields, pos).for_each(|(target_y, target_x, field)| {
            if field.is_empty() {
                if distance_cache[target_y as usize][target_x as usize] > prio + 1
                    && prio + 1 <= target_distance.unwrap_or(i32::MAX)
                {
                    distance_cache[target_y as usize][target_x as usize] = prio + 1;
                    if empty_target_positions.contains(&(target_y, target_x)) {
                        target_distance =
                            Some(target_distance.map(|d| d.min(prio + 1)).unwrap_or(prio + 1));
                        minimal_targets.insert((target_y, target_x));
                    }
                    // for (i, row) in distance_cache.iter().enumerate() {
                    //     for (j, d) in row.iter().enumerate() {
                    //         if *d == i32::MAX {
                    //             print!("{}", &fields[i as usize][j as usize].to_char());
                    //         }  else {
                    //             print!("{d}");
                    //         }
                    //     }
                    //     println!();
                    // }
                    // println!();

                    queue.insert((target_y, target_x), prio + 1);
                }
            }
        })
    }
    // print_fields(fields);
    let mut next_step_queue = VecDeque::new();
    if minimal_targets.is_empty() {
        return None;
    }
    let minimal_distance_target = min_reading_order(minimal_targets.iter().copied());
    next_step_queue.push_back(minimal_distance_target);
    let mut next_step_candidates = HashSet::new();
    while !next_step_queue.is_empty() {
        let next_step = next_step_queue.pop_front().unwrap();
        let dist = distance_cache[next_step.0 as usize][next_step.1 as usize];
        if dist == 1 {
            next_step_candidates.insert(next_step);
        } else {
            adjacent_fields(fields, next_step).for_each(|(target_y, target_x, _)| {
                if distance_cache[target_y as usize][target_x as usize] == dist - 1 {
                    next_step_queue.push_back((target_y, target_x));
                }
            });
        }
    }
    Some(min_reading_order(next_step_candidates.into_iter()))
}

fn min_reading_order(pos: impl Iterator<Item = (i32, i32)>) -> (i32, i32) {
    pos.min_by(|p1, p2| match p1.0.cmp(&p2.0) {
        Ordering::Equal => p1.1.cmp(&p2.1),
        v => v,
    })
    .unwrap()
}

fn print_fields(fields: &Vec<Vec<Field>>) {
    for row in fields {
        for field in row {
            print!("{}", field.to_char());
        }
        print!("    ");
        for field in row {
            if field.is_character() {
                print!("{}({}), ", field.to_char(), field.get_health());
            }
        }
        println!();
    }
}

fn is_goblin_adjacent(fields: &Vec<Vec<Field>>, pos: (i32, i32)) -> bool {
    adjacent_fields(fields, pos).any(|(_, _, field)| field.is_goblin())
}

fn is_elf_adjacent(fields: &Vec<Vec<Field>>, pos: (i32, i32)) -> bool {
    adjacent_fields(fields, pos).any(|(_, _, field)| field.is_elf())
}

pub fn day15_1_2018() {
    let inputs = [
        "#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######",
        "#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######",
        "#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######",
        "#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######",
        "#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######",
        "#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########",
    ];

    // let input = include_str!("../input/day15_1_2018.txt");

    let inputs = [
        "#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######",
        "#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######",
        "#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######",
        "#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######",
        "#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########",
    ];

    for input in inputs {
        for i in 0..200 {
            let res = solve(input, i, true);
            if res.1 {
                println!("{}", res.0);
                break;
            }
        }
    }

    let input = include_str!("../input/day15_1_2018.txt");
    for i in 0..200 {
        let res = solve(input, i, true);
        if res.1 {
            println!("{}", res.0);
            break;
        }
    }
}

fn solve(input: &str, elf_strength: i32, break_on_elf_death: bool) -> (i32, bool) {
    let mut fields = input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| match c {
                    '#' => Field::Wall,
                    '.' => Field::Empty,
                    'G' => Field::Goblin(200, 0),
                    'E' => Field::Elf(200, 0),
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();

    let n = fields.len() as i32;
    let m = fields[0].len() as i32;

    let mut goblin_positions = HashSet::new();
    let mut elf_positions = HashSet::new();
    let mut goblin_adjacent = HashSet::new();
    let mut elf_adjacent = HashSet::new();
    let mut empty_positions = HashSet::new();
    for i in 0..n {
        for j in 0..m {
            match fields[i as usize][j as usize] {
                Field::Goblin(_, _) => {
                    goblin_positions.insert((i, j));
                    adjacent_fields(&fields, (i, j))
                        .filter(|&(_, _, field)| field.is_empty())
                        .for_each(|(i, j, _)| {
                            goblin_adjacent.insert((i, j));
                        });
                }
                Field::Elf(_, _) => {
                    elf_positions.insert((i, j));
                    adjacent_fields(&fields, (i, j))
                        .filter(|&(_, _, field)| field.is_empty())
                        .for_each(|(i, j, _)| {
                            elf_adjacent.insert((i, j));
                        });
                }
                Field::Empty => {
                    empty_positions.insert((i, j));
                }
                _ => {}
            }
        }
    }

    // print_fields(&fields);
    // println!();

    let mut counter = 0i32;
    'outer: loop {
        counter += 1;
        for i in 0..n {
            for j in 0..m {
                let mut i = i;
                let mut j = j;
                let mut field = &mut fields[i as usize][j as usize];
                if field.is_goblin() && elf_positions.is_empty()
                    || field.is_elf() && goblin_positions.is_empty()
                {
                    break 'outer;
                }
                if !field.is_character() {
                    continue;
                }
                if field.get_count() >= counter {
                    continue;
                }
                field.increase_count();
                let mut field = fields[i as usize][j as usize];
                let mut enemy_adjacent = is_enemy_adjacent(&fields, (i, j));
                if !enemy_adjacent {
                    if let Some(target) = towards_min_distance(
                        &fields,
                        &empty_positions,
                        if field.is_goblin() {
                            &elf_adjacent
                        } else {
                            &goblin_adjacent
                        },
                        (i, j),
                    ) {
                        fields[target.0 as usize][target.1 as usize] = field;
                        fields[i as usize][j as usize] = Field::Empty;

                        empty_positions.remove(&target);
                        empty_positions.insert((i, j));

                        goblin_adjacent.remove(&target);
                        elf_adjacent.remove(&target);

                        if is_goblin_adjacent(&fields, (i, j)) {
                            goblin_adjacent.insert((i, j));
                        }

                        if is_elf_adjacent(&fields, (i, j)) {
                            elf_adjacent.insert((i, j));
                        }

                        adjacent_fields(&fields, (i, j)).for_each(|(i, j, field)| {
                            if field.is_empty() {
                                if is_goblin_adjacent(&fields, (i, j)) {
                                    goblin_adjacent.insert((i, j));
                                } else {
                                    goblin_adjacent.remove(&(i, j));
                                }

                                if is_elf_adjacent(&fields, (i, j)) {
                                    elf_adjacent.insert((i, j));
                                } else {
                                    elf_adjacent.remove(&(i, j));
                                }
                            }
                        });

                        adjacent_fields(&fields, target).for_each(|(i, j, f)| {
                            if f.is_empty() {
                                if field.is_goblin() {
                                    goblin_adjacent.insert((i, j));
                                } else {
                                    elf_adjacent.insert((i, j));
                                }
                            }
                        });
                        if field.is_goblin() {
                            goblin_positions.remove(&(i, j));
                            goblin_positions.insert(target);
                        } else {
                            elf_positions.remove(&(i, j));
                            elf_positions.insert(target);
                        }
                        enemy_adjacent = is_enemy_adjacent(&fields, target);
                        i = target.0;
                        j = target.1;
                        field = fields[i as usize][j as usize];
                    }
                }

                /*{
                    let mut goblin_positions2 = HashSet::new();
                    let mut elf_positions2 = HashSet::new();
                    let mut goblin_adjacent2 = HashSet::new();
                    let mut elf_adjacent2 = HashSet::new();
                    let mut empty_positions2 = HashSet::new();
                    for i in 0..n {
                        for j in 0..m {
                            match fields[i as usize][j as usize] {
                                Field::Goblin(_, _) => {
                                    goblin_positions2.insert((i, j));
                                    adjacent_fields(&fields, (i, j))
                                        .filter(|&(_, _, field)| field.is_empty())
                                        .for_each(|(i, j, _)| {
                                            goblin_adjacent2.insert((i, j));
                                        });
                                }
                                Field::Elf(_, _) => {
                                    elf_positions2.insert((i, j));
                                    adjacent_fields(&fields, (i, j))
                                        .filter(|&(_, _, field)| field.is_empty())
                                        .for_each(|(i, j, _)| {
                                            elf_adjacent2.insert((i, j));
                                        });
                                }
                                Field::Empty => {
                                    empty_positions2.insert((i, j));
                                }
                                _ => {}
                            }
                        }
                    }
                    // println!("{:?}", elf_adjacent.difference(&elf_adjacent2));
                    // println!("{:?}", elf_adjacent));
                    // println!("{:?}", goblin_adjacent.difference(&goblin_adjacent2));
                    // println!("{:?}", goblin_adjacent2.difference(&goblin_adjacent));
                    println!("counter: {}, pos: {:?}", counter, (i, j));
                    if (elf_adjacent != elf_adjacent2 || true) {
                        println!("elf_adjacent.difference(&elfe_adjacent2));
                        println!("elf_adjacent2.difference(&elf_adjacent));
                    }
                    if (goblin_adjacent != goblin_adjacent2 || true) {
                        println!("goblin {:?}", goblin_adjacent.difference(&goblin_adjacent2));
                        println!("goblin2 {:?}", goblin_adjacent2.difference(&goblin_adjacent));
                    }
                    print_fields(&fields);
                    println!();
                    assert_eq!(goblin_positions, goblin_positions2);
                    assert_eq!(elf_positions, elf_positions2);
                    assert_eq!(goblin_adjacent, goblin_adjacent2);
                    assert_eq!(elf_adjacent, elf_adjacent2);
                    assert_eq!(empty_positions, empty_positions2);
                }*/

                if enemy_adjacent {
                    let attack_target_pos = min_reading_order({
                        let mut min_health = i32::MAX;
                        let mut min_health_pos = HashSet::new();
                        adjacent_fields(&fields, (i, j)).for_each(|(i, j, f)| {
                            if field.is_opposite(&f) {
                                if f.get_health() == min_health {
                                    min_health_pos.insert((i, j));
                                } else if f.get_health() < min_health {
                                    min_health = f.get_health();
                                    min_health_pos.clear();
                                    min_health_pos.insert((i, j));
                                }
                            }
                        });

                        // println!("{:?} attacks {:?}", (i, j), min_health_pos);
                        min_health_pos.into_iter()
                    });
                    let attack_target =
                        &mut fields[attack_target_pos.0 as usize][attack_target_pos.1 as usize];
                    if attack_target.is_goblin() {
                        *attack_target.get_health_mut() -= elf_strength;
                    } else {
                        *attack_target.get_health_mut() -= 3;
                    }
                    if attack_target.get_health() <= 0 {
                        let is_goblin = attack_target.is_goblin();
                        if !is_goblin && break_on_elf_death {
                            return (0, false);
                        }
                        *attack_target = Field::Empty;
                        empty_positions.insert(attack_target_pos);
                        if is_goblin {
                            goblin_positions.remove(&attack_target_pos);
                        } else {
                            elf_positions.remove(&attack_target_pos);
                        }
                        adjacent_fields(&fields, attack_target_pos).for_each(|(i, j, f)| {
                            if f.is_empty() {
                                if is_goblin_adjacent(&fields, (i, j)) {
                                    goblin_adjacent.insert((i, j));
                                } else {
                                    goblin_adjacent.remove(&(i, j));
                                }

                                if is_elf_adjacent(&fields, (i, j)) {
                                    elf_adjacent.insert((i, j));
                                } else {
                                    elf_adjacent.remove(&(i, j));
                                }
                            }
                        });
                        if is_goblin_adjacent(&fields, attack_target_pos) {
                            goblin_adjacent.insert(attack_target_pos);
                        }
                        if is_elf_adjacent(&fields, attack_target_pos) {
                            elf_adjacent.insert(attack_target_pos);
                        }
                    }
                }
            }
        }
    }
    // print_fields(&fields);
    // println!();
    let remaining_health = goblin_positions
        .iter()
        .map(|(i, j)| fields[*i as usize][*j as usize].get_health())
        .sum::<i32>()
        + elf_positions
            .iter()
            .map(|(i, j)| fields[*i as usize][*j as usize].get_health())
            .sum::<i32>();
    // println!("counter: {}, remaining: {}", counter, remaining_health);
    let total = remaining_health * (counter - 1);
    (total, goblin_positions.is_empty())
}
