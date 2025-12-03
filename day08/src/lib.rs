pub fn solve(input: &str) {
    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

fn part1(_input: &str) -> i32 {
    0
}

fn part2(_input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
987654321111111
811111111111119
234234234234278
818181911112111";
        let result = part1(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part2() {
        let input = "\
987654321111111
811111111111119
234234234234278
818181911112111";
        let result = part2(input);
        assert_eq!(result, 0);
    }
}
