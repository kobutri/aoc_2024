use std::collections::HashSet;

fn get_input() -> (Vec<[u8; 5]>, Vec<[u8; 5]>) {
    let input = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
    let input = include_str!("../input/day25_1.txt");
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    input.split("\n\n").for_each(|s| {
        let mut counts = [0; 5];
        s.lines().for_each(|l| {
            l.char_indices().for_each(|(i, c)| {
                if c == '#' {
                    counts[i] += 1;
                }
            });
        });
        counts.iter_mut().for_each(|c| *c -= 1);
        if s.starts_with("#") {
            locks.push(counts);
        } else {
            keys.push(counts);
        }
    });
    (locks, keys)
}

fn get_counts<'a>(input: impl Iterator<Item=&'a [u8; 5]>) -> [[HashSet<&'a [u8; 5]>; 6]; 5] {
    let mut counts = core::array::from_fn(|_| core::array::from_fn(|_| HashSet::new()));
    for el in input {
        for (i, c) in el.iter().enumerate() {
            for j in *c..6 {
                counts[i][j as usize].insert(el);
            }
        }
    }
    counts
}

pub fn day25_1() {
    let (locks, keys) = get_input();
    let key_counts = get_counts(keys.iter());
    let mut total = 0;
    for lock in &locks {
        let mut key_candidates = HashSet::from_iter(keys.iter());
        for (i, c) in lock.iter().enumerate() {
            key_candidates = key_counts[i][5-*c as usize].intersection(&key_candidates).copied().collect();
        }
        total += key_candidates.len();
    }
    println!("{}", total);
}
pub fn day25_2() {
    // nothing to do
}
