use clap::Parser;
use regex::Regex;

pub fn day3_1() {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let input = include_str!("../input/day3_1.txt");
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let total = re.captures_iter(input).fold(0, |acc, e| {
        let (_, [m1, m2]) = e.extract();
        let n1 = m1.parse::<i32>().unwrap();
        let n2 = m2.parse::<i32>().unwrap();
        acc + n1 * n2
    });
    println!("{}", total)
}

pub fn day3_2() {
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let input = include_str!("../input/day3_1.txt");
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(do\(\))|(don't\(\))").unwrap();
    let mut total = 0;
    let mut enable = true;
    for c in re.captures_iter(input) {
        let m = c.get(0).unwrap();
        if m.as_str() == "do()" {
            enable = true;
        } else if m.as_str() == "don't()" {
            enable = false;
        } else if enable {
            let n1 = c.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let n2 = c.get(2).unwrap().as_str().parse::<i32>().unwrap();
            total += n1 * n2;
        }
    }
    println!("{}", total)
}
