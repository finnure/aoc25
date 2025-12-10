use std::ops::RangeInclusive;
use std::rc::Rc;

pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: &str, y: &str) -> Self {
        Point {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

impl Clone for Point {
    fn clone(&self) -> Self {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub enum Line {
    Horizontal { y: usize, x: RangeInclusive<usize> },
    Vertical { x: usize, y: RangeInclusive<usize> },
}

impl Line {
    pub fn new(p1: Rc<Point>, p2: Rc<Point>) -> Self {
        if p1.y == p2.y {
            let y = p1.y;
            let x_start = p1.x.min(p2.x);
            let x_end = p1.x.max(p2.x);
            Line::Horizontal {
                y,
                x: RangeInclusive::new(x_start, x_end),
            }
        } else if p1.x == p2.x {
            let x = p1.x;
            let y_start = p1.y.min(p2.y);
            let y_end = p1.y.max(p2.y);
            Line::Vertical {
                x,
                y: RangeInclusive::new(y_start, y_end),
            }
        } else {
            panic!("Points do not form a straight line");
        }
    }

    pub fn crosses(&self, other: &Line) -> bool {
        match (self, other) {
            (Line::Horizontal { y: y1, x: x1 }, Line::Vertical { x: x2, y: y2 }) => {
                // Original check: intersection at any point
                let crosses = x1.contains(x2) && y2.contains(y1);
                // Exclude intersection at start or end points
                let not_at_endpoints = *x2 != *x1.start()
                    && *x2 != *x1.end()
                    && *y1 != *y2.start()
                    && *y1 != *y2.end();
                crosses && not_at_endpoints
            }
            (Line::Vertical { x: x1, y: y1 }, Line::Horizontal { y: y2, x: x2 }) => {
                let crosses = x2.contains(x1) && y1.contains(y2);
                let not_at_endpoints = *x1 != *x2.start()
                    && *x1 != *x2.end()
                    && *y2 != *y1.start()
                    && *y2 != *y1.end();
                crosses && not_at_endpoints
            }
            _ => false,
        }
    }
}

pub struct Box {
    top_left: Rc<Point>,
    top_left_valid: bool,
    top_right: Rc<Point>,
    top_right_valid: bool,
    bottom_left: Rc<Point>,
    bottom_left_valid: bool,
    bottom_right: Rc<Point>,
    bottom_right_valid: bool,
}

impl Box {
    pub fn new(p1: Point, p2: Point) -> Self {
        let top_left = Rc::new(Point {
            x: p1.x.min(p2.x),
            y: p1.y.min(p2.y),
        });
        let bottom_right = Rc::new(Point {
            x: p1.x.max(p2.x),
            y: p1.y.max(p2.y),
        });
        let top_right = Rc::new(Point {
            x: bottom_right.x,
            y: top_left.y,
        });
        let bottom_left = Rc::new(Point {
            x: top_left.x,
            y: bottom_right.y,
        });

        Box {
            top_left_valid: top_left == p1.clone().into() || top_left == p2.clone().into(),
            top_left,
            top_right_valid: top_right == p1.clone().into() || top_right == p2.clone().into(),
            top_right,
            bottom_left_valid: bottom_left == p1.clone().into() || bottom_left == p2.clone().into(),
            bottom_left,
            bottom_right_valid: bottom_right == p1.clone().into()
                || bottom_right == p2.clone().into(),
            bottom_right,
        }
    }

    pub fn area(&self) -> i64 {
        let width = ((self.bottom_right.x as i64 - self.top_left.x as i64).abs() + 1) as i64;
        let height = ((self.bottom_right.y as i64 - self.top_left.y as i64).abs() + 1) as i64;
        width * height
    }

    pub fn intersects(&self, line: &Line) -> bool {
        match line {
            Line::Horizontal { y: _, x: _ } => {
                let left = Line::new(Rc::clone(&self.top_left), Rc::clone(&self.bottom_left));
                let right = Line::new(Rc::clone(&self.top_right), Rc::clone(&self.bottom_right));
                left.crosses(line) || right.crosses(line)
            }
            Line::Vertical { x: _, y: _ } => {
                let top = Line::new(Rc::clone(&self.top_left), Rc::clone(&self.top_right));
                let bottom = Line::new(Rc::clone(&self.bottom_left), Rc::clone(&self.bottom_right));
                top.crosses(line) || bottom.crosses(line)
            }
        }
    }

    pub fn is_valid(&self) -> bool {
        self.top_left_valid
            && self.top_right_valid
            && self.bottom_left_valid
            && self.bottom_right_valid
    }

    pub fn update_corners(&mut self, line: &Line) {
        if self.is_valid() {
            return;
        }
        match line {
            Line::Horizontal { y, x } => {
                if !self.top_left_valid && self.top_left.y == *y && x.contains(&self.top_left.x) {
                    self.top_left_valid = true;
                }
                if !self.top_right_valid && self.top_right.y == *y && x.contains(&self.top_right.x)
                {
                    self.top_right_valid = true;
                }
                if !self.bottom_left_valid
                    && self.bottom_left.y == *y
                    && x.contains(&self.bottom_left.x)
                {
                    self.bottom_left_valid = true;
                }
            }
            Line::Vertical { x, y } => {
                if !self.top_left_valid && self.top_left.x == *x && y.contains(&self.top_left.y) {
                    self.top_left_valid = true;
                }

                if !self.top_right_valid && self.top_right.x == *x && y.contains(&self.top_right.y)
                {
                    self.top_right_valid = true;
                }
                if !self.bottom_right_valid
                    && self.bottom_right.x == *x
                    && y.contains(&self.bottom_right.y)
                {
                    self.bottom_right_valid = true;
                }
            }
        }
    }
}
