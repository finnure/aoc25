pub fn solve(input: &str) {
    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    let mut count = 0;
    let sections: Vec<&str> = input.split("\n\n").collect();
    let ranges: Vec<(u64, u64)> = sections[0]
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split('-').collect();
            (parts[0].parse().unwrap(), parts[1].parse().unwrap())
        })
        .collect();
    sections[1].lines().for_each(|line| {
        let num: u64 = line.parse().unwrap();
        for (start, end) in &ranges {
            if num >= *start && num <= *end {
                count += 1;
                return;
            }
        }
    });
    count
}

fn part2(input: &str) -> u64 {
    let mut sum = 0;
    let sections: Vec<&str> = input.split("\n\n").collect();
    let ranges: Vec<(u64, u64)> = sections[0]
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split('-').collect();
            (parts[0].parse().unwrap(), parts[1].parse().unwrap())
        })
        .collect();
    let mut done: Vec<i32> = Vec::new();
    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();
    for i in 0..ranges.len() {
        let mut range_start = ranges[i].0;
        let mut range_end = ranges[i].1;
        loop {
            let mut changed = false;
            for j in i + 1..ranges.len() {
                if done.contains(&(j as i32)) {
                    continue;
                }
                let (other_start, other_end) = ranges[j];
                if ((range_start >= other_start) && (range_start <= other_end))
                    || ((range_end >= other_start) && (range_end <= other_end))
                    || ((other_start >= range_start) && (other_start <= range_end))
                    || ((other_end >= range_start) && (other_end <= range_end))
                {
                    range_start = range_start.min(other_start);
                    range_end = range_end.max(other_end);
                    done.push(j as i32);
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        if !done.contains(&(i as i32)) {
            merged_ranges.push((range_start, range_end));
        }
    }
    for (start, end) in &merged_ranges {
        sum += end - start + 1;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let result = part1(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part2() {
        let input = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let result = part2(input);
        assert_eq!(result, 14);
    }
}
