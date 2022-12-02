const INPUT: &str = include_str!("input.txt");

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let mut sums = vec![0];
    for line in INPUT.lines() {
        if line.is_empty() {
            sums.push(0);
            continue;
        }
        if let Some(curr) = sums.last_mut() {
            *curr += line.parse::<i32>().expect("line is not a number")
        }
    }
    println!("{}", sums.iter().max().expect("sums is empty"));
}

fn part_two() {
    let mut sums = vec![0];
    for line in INPUT.lines() {
        if line.is_empty() {
            sums.push(0);
            continue;
        }
        if let Some(curr) = sums.last_mut() {
            *curr += line.parse::<i32>().expect("line is not a number")
        }
    }
    sums.sort();
    println!("{}", sums.iter().rev().take(3).sum::<i32>());
}
