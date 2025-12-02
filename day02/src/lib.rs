pub fn solve(input: &str) {
    println!("part 1: {}", part1(input));
}

fn part1(input: &str) -> u64 {
    let mut sum: u64 = 0;
    for range in input.trim().split(',') {
        let (left, right) = range.split_once('-').unwrap();
        let left: u64 = left.parse().unwrap();
        let right: u64 = right.parse().unwrap();
        for id in left..=right {
            // convert id to string, check if it has even number of digits and check if first half is same as second half
            let id_str = id.to_string();
            let len = id_str.len();
            if len % 2 != 0 {
                continue;
            }
            let half = len / 2;
            if &id_str[..half] == &id_str[half..] {
                sum += id;
            }
        }
    }
    sum
}
