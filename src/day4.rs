pub fn day4_1() {
    let input = "....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX";
    let input = include_str!("../input/day4_1.txt");
    let mut chars = vec![];

    for line in input.lines() {
        chars.push(line.chars().collect::<Vec<_>>());
    }
    // for v in &chars {
    //     for c in v.iter().intersperse(&',') {
    //         print!("{c}");
    //     }
    //     println!("");
    // }
    let n = chars.len() as i32;
    let m = chars[0].len() as i32;

    // horizontal
    let mut horizontal = 0;
    for i in 0..n {
        let s: String = (0..m).map(|j| chars[i as usize][j as usize]).collect();
        horizontal += s.matches("XMAS").count();
        horizontal += s.matches("SAMX").count();
    }

    let mut vertical = 0;
    for j in 0..m {
        let s: String = (0..n).map(|i| chars[i as usize][j as usize]).collect();
        vertical += s.matches("XMAS").count();
        vertical += s.matches("SAMX").count();
    }

    let mut diag1 = 0;
    for i in -n.max(m) + 1..n.max(m) {
        let s: String = (0..n.max(n))
            .filter_map(|j| chars.get(j as usize).and_then(|v| v.get((i + j) as usize)))
            .collect();
        diag1 += s.matches("XMAS").count();
        diag1 += s.matches("SAMX").count();
    }

    let mut diag2 = 0;
    for i in -n.max(m) + 1..n.max(m) {
        let s: String = (0..n)
            .filter_map(|j| {
                chars
                    .get(j as usize)
                    .and_then(|v| v.get((n - 1 - j + i) as usize))
            })
            .collect();
        diag2 += s.matches("XMAS").count();
        diag2 += s.matches("SAMX").count();
    }

    println!("{}", horizontal + vertical + diag1 + diag2);
}
pub fn day4_2() {
    let input = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........X";
    let input = include_str!("../input/day4_1.txt");
    let mut chars = vec![];

    for line in input.lines() {
        chars.push(line.chars().collect::<Vec<_>>());
    }
    // for v in &chars {
    //     for c in v.iter().intersperse(&',') {
    //         print!("{c}");
    //     }
    //     println!();
    // }
    let n = chars.len();
    let m = chars[0].len();

    let mut total = 0;
    for i in 1..n - 1 {
        for j in 1..m - 1 {
            if chars[i][j] != 'A' {
                continue;
            }
            if !(chars[i - 1][j - 1] == 'S' && chars[i + 1][j + 1] == 'M'
                || chars[i - 1][j - 1] == 'M' && chars[i + 1][j + 1] == 'S')
            {
                continue;
            }
            if !(chars[i - 1][j + 1] == 'S' && chars[i + 1][j - 1] == 'M'
                || chars[i - 1][j + 1] == 'M' && chars[i + 1][j - 1] == 'S')
            {
                continue;
            }
            total += 1;
        }
    }
    println!("{}", total);
}
