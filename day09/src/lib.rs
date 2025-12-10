mod grid;
use std::rc::Rc;

use crate::grid::{Box, Line, Point};

pub fn solve(input: &str) {
    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

fn part1(input: &str) -> i64 {
    let mut result = 0;
    let lines: Vec<(i64, i64)> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x: i64 = parts.next().unwrap().parse().unwrap();
            let y: i64 = parts.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect();
    for i in 0..lines.len() {
        for j in i + 1..lines.len() {
            let (x1, y1) = lines[i];
            let (x2, y2) = lines[j];
            let size = ((x1 - x2).abs() + 1) * ((y1 - y2).abs() + 1);
            if size > result {
                result = size;
            }
        }
    }
    result
}
fn part2(input: &str) -> i64 {
    let mut result = 0;
    let points = input
        .lines()
        .map(|line| {
            let parts = line.split(',').collect::<Vec<&str>>();
            Point::new(parts[0], parts[1])
        })
        .collect::<Vec<Point>>();
    let mut lines = Vec::new();
    for i in 1..points.len() {
        let line = Line::new(Rc::new(points[i - 1].clone()), Rc::new(points[i].clone()));
        lines.push(line);
    }
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let mut box_ = Box::new(points[i].clone(), points[j].clone());
            if box_.area() <= result {
                continue;
            }
            let mut crosses = false;
            for line in &lines {
                box_.update_corners(line);
                if box_.intersects(line) {
                    crosses = true;
                    break;
                }
            }
            if !crosses && box_.is_valid() && box_.area() > result {
                result = box_.area();
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_part1() {
        let result = part1(INPUT);
        assert_eq!(result, 50);
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);
        assert_eq!(result, 24);
    }
}
