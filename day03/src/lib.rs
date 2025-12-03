pub fn solve(input: &str) {
    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    let mut total = 0;
    for line in input.lines() {
        let digits: Vec<i32> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();
        let (first, index) = largest(&digits[..(digits.len() - 1)]);
        let (second, _) = largest(&digits[(index as usize + 1)..]);
        total += first * 10 + second;
    }
    total
}

fn part2(input: &str) -> u64 {
    let mut total = 0;
    for line in input.lines() {
        let digits: Vec<u64> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect();
        let base: u64 = 10;
        let mut num: u64 = 0;
        let mut index: usize = 0;
        for i in (1..12).rev() {
            let (value, new_index) = largest(&digits[index..(digits.len() - i)]);
            index += new_index + 1;
            num += value * base.pow(i as u32);
        }
        let (second, _) = largest(&digits[index..]);
        num += second;
        total += num;
    }
    total
}

fn largest<T>(numbers: &[T]) -> (T, usize)
where
    T: PartialEq + PartialOrd + Copy,
{
    let mut max_value = numbers[0];
    let mut max_index = 0;
    for (index, &value) in numbers.iter().enumerate() {
        if value > max_value {
            max_value = value;
            max_index = index;
        }
    }
    (max_value, max_index)
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
        assert_eq!(result, 357);
    }

    #[test]
    fn test_part2() {
        let input = "\
987654321111111
811111111111119
234234234234278
818181911112111";
        let result = part2(input);
        assert_eq!(result, 3121910778619);
    }
}
