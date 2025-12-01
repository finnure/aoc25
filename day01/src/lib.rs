pub fn solve(input: &str) {
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let mut pos = 50;
    let mut countzero = 0;
    for line in input.lines() {
        let (dir, dist) = line.split_at(1);
        let dist: i32 = dist.parse().unwrap();
        match dir {
            "L" => {
                pos -= dist;
                while pos < 0 {
                    pos += 100;
                }
            }
            "R" => {
                pos += dist;
                while pos >= 100 {
                    pos -= 100;
                }
            }
            _ => panic!("Invalid direction"),
        }
        if pos == 0 {
            countzero += 1;
        }
    }
    println!("Part 1: {}", countzero);
}

fn part2(input: &str) {
    let mut pos = 50;
    let mut countzero = 0;
    for line in input.lines() {
        let (dir, dist) = line.split_at(1);
        let dist: i32 = dist.parse().unwrap();
        match dir {
            "L" => {
                if pos == 0 {
                    // This 0 has already been counted
                    countzero -= 1;
                }
                pos -= dist;
                while pos < 0 {
                    pos += 100;
                    countzero += 1;
                }
                if pos == 0 {
                    countzero += 1;
                }
            }
            "R" => {
                pos += dist;
                while pos >= 100 {
                    pos -= 100;
                    countzero += 1;
                }
            }
            _ => panic!("Invalid direction"),
        }
    }
    println!("Part 2: {}", countzero);
}
