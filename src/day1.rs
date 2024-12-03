use std::collections::HashMap;

pub fn day1_1() {
    let (vec1, vec2) = read_numbers();
    let mut diff = 0;
    for (n1, n2) in vec1.iter().zip(vec2.iter()) {
        diff += n1.abs_diff(*n2);
    }
    println!("{}", diff);
}

pub fn day1_2() {
    let (vec1, vec2) = read_numbers();
    let mut map = HashMap::new();
    for v in vec2 {
        *map.entry(v).or_insert(0i32) += 1;
    }
    let mut sim = 0;
    for v in vec1 {
        sim += v * map.get(&v).unwrap_or(&0);
    }
    println!("{}", sim);
}

fn read_numbers() -> (Vec<i32>, Vec<i32>) {
    let str = include_str!("../input/day_1_1.txt");
    //     let str = "3   4
    // 4   3
    // 2   5
    // 1   3
    // 3   9
    // 3   3";
    let mut vec1 = vec![];
    let mut vec2 = vec![];
    for line in str.lines() {
        let (s1, s2) = line.split_once(" ").unwrap();
        vec1.push(s1.trim().parse::<i32>().unwrap());
        vec2.push(s2.trim().parse::<i32>().unwrap());
    }
    vec1.sort();
    vec2.sort();
    (vec1, vec2)
}
