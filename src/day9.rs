fn print_disk(vs: &[i32]) {
    for v in vs {
        if *v == -1 {
            print!(".");
        } else {
            print!("{v}");
        }
    }
    println!();
}

fn get_input() -> Vec<i32> {
    let input = "2333133121414131402";
    let input = include_str!("../input/day9_1.txt");
    let mut disk = vec![];
    for (i, c) in input.char_indices() {
        if !c.is_digit(10) {
            println!("{}", c as u8);
        }
        let count = c.to_digit(10).unwrap();
        if i % 2 == 0 {
            for _ in 0..count {
                disk.push(i as i32 / 2);
            }
        } else {
            for _ in 0..count {
                disk.push(-1);
            }
        }
    } 
    disk
}

fn get_check_sum(disk: &[i32]) -> usize {
    let mut check_sum = 0usize;
    for (i, &v) in disk.iter().enumerate() {
        if v != -1 {
            check_sum += i * v as usize;
        }
    } 
    check_sum
}

pub fn day9_1() {
    let mut disk = get_input();
    let mut first = 0;
    let mut last = disk.len() -1;
    loop {
        while disk[last] == -1 {
            last -= 1;
        }
        while disk[first] != -1 {
            first += 1;
        }
        if first >= last {
            break;
        }
        disk[first] = disk[last];
        disk[last] = -1;
    }
    let check_sum = get_check_sum(&disk);
    println!("{}", check_sum);
}

enum Entry {
    File(i32, i32),
    Free(i32)
}
pub fn day9_2() {
    let mut disk = get_input();
   
    let mut last = disk.len() - 1;
    loop {
       while disk[last] == -1 {
           last -= 1;
       }
        let mut file_count = 1;
        while last > 0 && disk[last-1] == disk[last] {
            file_count += 1;
            last -= 1;
        }
        if last == 0 {
            break;
        }
        let mut first = 0;
        'outer: while first < last {
            while disk[first] != -1 {
                first += 1;
            }
            let mut free_count = 0;
            while free_count < file_count {
                if disk[first + free_count] != -1 {
                    first = first + free_count;
                    continue 'outer;
                }
                free_count += 1;
            }
            break 'outer;
        }
        if first >= last {
            last -= 1;
            continue;
        }
        for i in 0..file_count {
            disk[first+i] = disk[last+i];
            disk[last+i] = -1;
        }
    }


    let check_sum = get_check_sum(&disk);
    // println!("{:?}", disk);
    println!("{}", check_sum);
}
