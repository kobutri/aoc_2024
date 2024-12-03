fn read_reports() -> Vec<Vec<i32>> {
    let reports = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    let reports = include_str!("../input/day_2_1.txt");

    let reports = reports
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|number| number.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    reports
}

pub fn day2_1() {
    let reports = read_reports();

    let mut count_safe = 0;
    for report in reports {
        let diffs = report
            .iter()
            .scan(0, |s, v| {
                let diff = *v - *s;
                *s = *v;
                Some(diff)
            })
            .collect::<Vec<_>>();
        if diffs
            .iter()
            .skip(1)
            .all(|diff| diff.abs() <= 3 && diff.abs() >= 1 && *diff > 0)
            || diffs
                .iter()
                .skip(1)
                .all(|diff| diff.abs() <= 3 && diff.abs() >= 1 && *diff < 0)
        {
            count_safe += 1;
        }
    }
    println!("{count_safe}");
}

pub fn day2_2() {
    let reports = read_reports();

    let mut count_safe = 0;
    for report in reports {
        if (0..report.len()).any(|i| {
            let elements = report
                .iter()
                .enumerate()
                .filter_map(|(index, val)| if index != i { Some(val) } else { None })
                .collect::<Vec<_>>();

            let diffs = elements
                .iter()
                .scan(0, |s, v| {
                    let diff = *v - *s;
                    *s = **v;
                    Some(diff)
                })
                .collect::<Vec<_>>();
            if diffs
                .iter()
                .skip(1)
                .all(|diff| diff.abs() <= 3 && diff.abs() >= 1 && *diff > 0)
                || diffs
                    .iter()
                    .skip(1)
                    .all(|diff| diff.abs() <= 3 && diff.abs() >= 1 && *diff < 0)
            {
                return true;
            }

            // println!("{:?}", elements);

            return false;
        }) {
            count_safe += 1;
        }
        // println!("\n");
    }
    println!("{count_safe}");
}
