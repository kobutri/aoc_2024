use itertools::Itertools;

fn solve(count: i32) {
    let input = "125 17";
    let input = include_str!("../input/day11_1.txt");
    let mut input = input.split(" ").map(|s| s.trim().parse::<u128>().unwrap()).collect_vec();
    for i in 0..count {
        println!("{i}, {}", input.len());
        input = input.into_iter().flat_map(|x| {
            if x == 0 {
                [Some(1), None].into_iter()
            } else {
                let mut digits = 1;
                let mut y = x;
                while y >= 10 {
                    y /= 10;
                    digits += 1;
                }
                if digits % 2 == 0 {
                    let factor = 10u128.pow(digits / 2);
                    let left = x / factor;
                    let right = x % factor;
                    [Some(left), Some(right)].into_iter()
                } else {
                    [Some(x * 2024), None].into_iter()
                }
            }
        }).filter_map(|x| x).collect_vec();
    }

    println!("{}", input.len());
}
pub fn day11_1() {
    solve(25);
}
pub fn day11_2() {
    solve(75);
}