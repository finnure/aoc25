pub fn solve(input: &str) {
    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    let mut result = 0;
    let lines: Vec<Vec<&str>> = input
        .lines()
        .map(|line| {
            line.split(' ')
                .filter(|s| !s.is_empty())
                .collect::<Vec<&str>>()
        })
        .collect();
    for i in 0..lines[0].len() {
        let mut sum = 0;
        let mut mult = 1;
        for j in 0..(lines.len() - 1) {
            let num = lines[j][i].parse::<u64>().unwrap();
            sum += num;
            mult *= num;
        }
        match lines.last().unwrap()[i] {
            "+" => result += sum,
            "*" => result += mult,
            _ => panic!("Unknown operator"),
        }
    }
    result
}

fn part2(input: &str) -> u64 {
    let mut result = 0;
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut sum = 0;
    let mut mult = 1;
    for i in (0..lines[0].len()).rev() {
        let mut val = String::new();
        for j in 0..(lines.len() - 1) {
            val.push(lines[j][i]);
        }
        let num = val.trim().parse::<u64>().unwrap_or_else(|_| 0);
        if num != 0 {
            sum += num;
            mult *= num;
            match lines.last().unwrap()[i] {
                '+' => result += sum,
                '*' => result += mult,
                _ => {}
            }
        } else {
            sum = 0;
            mult = 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        let result = part1(input);
        assert_eq!(result, 4277556);
    }

    #[test]
    fn test_part2() {
        let input = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        let result = part2(input);
        assert_eq!(result, 3263827);
    }
}
