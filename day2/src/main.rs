const INPUT: &str = include_str!("input.txt");

fn main() {
    part_one();
    part_two();
}

fn encode(mv: &str) -> i32 {
    match mv {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        "X" => 0,
        "Y" => 1,
        "Z" => 2,
        _ => panic!("bad move {}", mv),
    }
}

fn play(line: &str) -> i32 {
    let (p1, p2) = line
        .split_once(' ')
        .map(|(p1, p2)| (encode(p1), encode(p2)))
        .expect("bad line");

    // Add 1 to each score to account for move-based scoring, plus 0 for a loss, 3 for
    // a draw, 6 for a win.
    if (p1 + 1) % 3 == p2 {
        p2 + 7
    } else if p1 == p2 {
        p2 + 4
    } else {
        p2 + 1
    }
}

fn play_two(line: &str) -> i32 {
    let (p1, res) = line
        .split_once(' ')
        .map(|(p1, res)| (encode(p1), res))
        .expect("bad line");

    // Add 1 to each score to account for move-based scoring, plus 0 for a loss, 3 for
    // a draw, 6 for a win.
    match res {
        "X" => (p1 + 2) % 3 + 1,
        "Y" => p1 + 4,
        "Z" => (p1 + 1) % 3 + 7,
        _ => panic!("bad result {}", res),
    }
}

fn part_one() {
    println!("{}", INPUT.lines().map(|line| play(line)).sum::<i32>())
}

fn part_two() {
    println!("{}", INPUT.lines().map(|line| play_two(line)).sum::<i32>())
}
