use std::collections::HashMap;

pub fn solve(input: &str) {
    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    let mut result = 0;
    let mut running: HashMap<usize, bool> = HashMap::new();
    let lines = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    for line in lines {
        for i in 0..line.len() {
            match line[i] {
                'S' => {
                    running.insert(i, true);
                }
                '^' => {
                    if let Some(is_running) = running.get(&i) {
                        if *is_running {
                            result += 1;
                            running.insert(i, false);
                            running.insert(i - 1, true);
                            running.insert(i + 1, true);
                        }
                    }
                }
                _ => { /* do nothing */ }
            }
        }
    }
    result
}

fn part2(input: &str) -> u64 {
    let mut result = 1;
    let mut gridlevels: HashMap<usize, u64> = HashMap::new();
    let lines = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    for line in (0..lines.len()).rev() {
        for i in 0..lines[line].len() {
            match lines[line][i] {
                'S' => {
                    result = *gridlevels.get(&i).unwrap_or(&1);
                }
                '^' => {
                    let left = gridlevels.get(&(i - 1)).unwrap_or(&1);
                    let right = gridlevels.get(&(i + 1)).unwrap_or(&1);
                    gridlevels.insert(i, left + right);
                }
                _ => { /* do nothing */ }
            }
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
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        let result = part1(input);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_part2() {
        let input = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        let result = part2(input);
        assert_eq!(result, 40);
    }
}
