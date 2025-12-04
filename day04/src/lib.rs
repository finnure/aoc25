pub fn solve(input: &str) {
    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    let lines: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let mut sum = 0;
    for line in 0..lines.len() {
        for col in 0..lines[line].len() {
            let c = lines[line][col];
            match c {
                '@' => {
                    if count_adjacent(&lines, line, col) < 4 {
                        sum += 1
                    }
                }
                _ => {}
            }
        }
    }
    sum
}

fn part2(input: &str) -> i32 {
    let mut lines: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let mut sum = 0;
    loop {
        let count = count_and_update(&mut lines);
        if count == 0 {
            break;
        }
        sum += count;
    }
    sum
}

fn count_and_update(lines: &mut Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    let copy_lines = lines.clone();
    for line in 0..lines.len() {
        for col in 0..lines[line].len() {
            let c = lines[line][col];
            match c {
                '@' => {
                    if count_adjacent(&copy_lines, line, col) < 4 {
                        count += 1;
                        lines[line][col] = 'O';
                    }
                }
                _ => {}
            }
        }
    }
    count
}

fn count_adjacent(lines: &Vec<Vec<char>>, line: usize, col: usize) -> i32 {
    let mut count = 0;
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    for (dl, dc) in directions.iter() {
        let new_line = line as isize + dl;
        let new_col = col as isize + dc;
        if new_line >= 0
            && new_line < lines.len() as isize
            && new_col >= 0
            && new_col < lines[0].len() as isize
        {
            if lines[new_line as usize][new_col as usize] == '@' {
                count += 1;
                if count >= 4 {
                    return count;
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let result = part1(input);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part2() {
        let input = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let result = part2(input);
        assert_eq!(result, 43);
    }
}
