pub fn solve(input: &str) {
    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
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

fn part2(input: &str) -> u64 {
    let mut sum: u64 = 0;
    for range in input.trim().split(',') {
        let (left, right) = range.split_once('-').unwrap();
        let left: u64 = left.parse().unwrap();
        let right: u64 = right.parse().unwrap();
        for id in left..=right {
            let id_str = id.to_string();
            let len = id_str.len();
            if len < 2 {
                continue;
            }
            // split into multiple single digit parts
            let digits: Vec<char> = id_str.chars().collect();
            if is_same(digits[0], digits[1..].to_vec()) {
                sum += id;
                // println!("singles {}", id);
                continue;
            }
            if len <= 2 {
                continue;
            }
            for i in 2..=(len / 2) {
                if len % i != 0 {
                    continue;
                }
                // split into parts of size i
                let mut parts: Vec<String> = Vec::new();
                for j in 0..(len / i) {
                    parts.push(id_str[j * i..(j + 1) * i].to_string());
                }
                if is_same(parts[0].clone(), parts[1..].to_vec()) {
                    sum += id;
                    // println!("parts of size {}: {}", i, id);
                    break;
                }
            }
        }
    }
    sum
}

fn is_same<T>(first: T, list: Vec<T>) -> bool
where
    T: PartialEq,
{
    for item in list {
        if first != item {
            return false;
        }
    }
    true
}
