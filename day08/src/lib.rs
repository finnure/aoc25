use std::rc::Rc;

pub fn solve(input: &str) {
    println!("part 1: {}", part1(input, None));
    println!("part 2: {}", part2(input));
}

fn part1(input: &str, iters: Option<bool>) -> i32 {
    let mut result = 1;
    let iters = if iters.is_none() { 1000 } else { 10 };
    let points: Vec<Rc<Point>> = input
        .lines()
        .map(|line| Rc::new(Point::from_str(line)))
        .collect();
    let mut pairs: Vec<Pair> = Vec::new();
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let pair = Pair::new(Rc::clone(&points[i]), Rc::clone(&points[j]));
            pairs.push(pair);
        }
    }
    let mut circuits: Vec<Circuit> = Vec::new();
    pairs.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
    for p in pairs.iter().take(iters) {
        let c1 = circuits.iter().position(|c| c.contains(&p.p1));
        let c2 = circuits.iter().position(|c| c.contains(&p.p2));
        if c1.is_some() && c2.is_some() && c1 != c2 {
            let idx1 = c1.unwrap();
            let idx2 = c2.unwrap();
            let circuit2 = circuits.remove(idx2);
            let new_idx1 = if idx2 < idx1 { idx1 - 1 } else { idx1 };
            circuits[new_idx1].merge(&circuit2);
            continue;
        }
        if let Some(idx) = circuits.iter().position(|c| c.contains_either(p)) {
            circuits[idx].add_point(p);
        } else {
            let circuit = Circuit::new(p);
            circuits.push(circuit);
        }
    }
    circuits.sort_by(|a, b| a.size().partial_cmp(&b.size()).unwrap());
    circuits.reverse();
    circuits
        .iter()
        .take(3)
        .for_each(|c| result *= c.size() as i32);
    result
}

fn part2(input: &str) -> i64 {
    let mut result = 0;
    let points: Vec<Rc<Point>> = input
        .lines()
        .map(|line| Rc::new(Point::from_str(line)))
        .collect();
    let mut pairs: Vec<Pair> = Vec::new();
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let pair = Pair::new(Rc::clone(&points[i]), Rc::clone(&points[j]));
            pairs.push(pair);
        }
    }
    let mut circuits: Vec<Circuit> = Vec::new();
    pairs.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
    for p in pairs.iter() {
        let c1 = circuits.iter().position(|c| c.contains(&p.p1));
        let c2 = circuits.iter().position(|c| c.contains(&p.p2));
        if c1.is_some() && c2.is_some() && c1 != c2 {
            let idx1 = c1.unwrap();
            let idx2 = c2.unwrap();
            let circuit2 = circuits.remove(idx2);
            let new_idx1 = if idx2 < idx1 { idx1 - 1 } else { idx1 };
            circuits[new_idx1].merge(&circuit2);
            continue;
        }
        if let Some(idx) = circuits.iter().position(|c| c.contains_either(p)) {
            circuits[idx].add_point(p);
        } else {
            let circuit = Circuit::new(p);
            circuits.push(circuit);
        }
        if circuits.len() == 1 && circuits[0].size() == points.len() {
            result = p.p1.x * p.p2.x;
            break;
        }
    }
    result
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn from_str(s: &str) -> Self {
        let coords: Vec<i64> = s
            .split(',')
            .map(|part| part.parse::<i64>().unwrap())
            .collect();
        Point {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        }
    }

    fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        ((dx * dx + dy * dy + dz * dz) as f64).sqrt()
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Pair {
    p1: Rc<Point>,
    p2: Rc<Point>,
    distance: f64,
}

impl Pair {
    fn new(p1: Rc<Point>, p2: Rc<Point>) -> Self {
        let distance = p1.distance(&p2);
        Pair { p1, p2, distance }
    }
}

struct Circuit {
    points: Vec<Rc<Point>>,
}

impl Circuit {
    fn new(pair: &Pair) -> Self {
        Circuit {
            points: vec![Rc::clone(&pair.p1), Rc::clone(&pair.p2)],
        }
    }

    fn add_point(&mut self, pair: &Pair) {
        if !self.points.contains(&pair.p1) {
            self.points.push(Rc::clone(&pair.p1));
        }
        if !self.points.contains(&pair.p2) {
            self.points.push(Rc::clone(&pair.p2));
        }
    }

    fn merge(&mut self, other: &Circuit) {
        for point in &other.points {
            if !self.points.contains(point) {
                self.points.push(Rc::clone(point));
            }
        }
    }

    fn contains(&self, point: &Rc<Point>) -> bool {
        self.points.contains(point)
    }

    fn contains_either(&self, pair: &Pair) -> bool {
        self.contains(&pair.p1) || self.contains(&pair.p2)
    }

    fn size(&self) -> usize {
        self.points.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        let result = part1(input, Some(true));
        assert_eq!(result, 40);
    }

    #[test]
    fn test_part2() {
        let input = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        let result = part2(input);
        assert_eq!(result, 25272);
    }
}
