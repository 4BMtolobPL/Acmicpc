use crate::Slope::{A, Horizontal, Vertical};
use std::fmt::Write;
use std::io;

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(start: Point, end: Point) -> Line {
        Line { start, end }
    }

    fn slope(&self) -> Slope {
        if self.start.x == self.end.x {
            Vertical
        } else if self.start.y == self.end.y {
            Horizontal
        } else {
            A((self.end.y - self.start.y) / (self.end.x - self.start.x))
        }
    }

    fn y_intercept(&self) -> Option<f64> {
        let slope = self.slope();

        match slope {
            Vertical => None,
            Horizontal => Some(self.start.y),
            A(s) => Some(self.start.y - self.start.x * s),
        }
    }
}

enum Slope {
    Vertical,
    Horizontal,
    A(f64),
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let lines = buf.lines();

    let mut out = String::new();
    for line in lines.skip(1) {
        let mut iter = line.split_ascii_whitespace().map(|x| x.parse().unwrap());

        let point1 = Point::new(iter.next().unwrap(), iter.next().unwrap());
        let point2 = Point::new(iter.next().unwrap(), iter.next().unwrap());
        let point3 = Point::new(iter.next().unwrap(), iter.next().unwrap());
        let point4 = Point::new(iter.next().unwrap(), iter.next().unwrap());

        let line1 = Line::new(point1, point2);
        let line2 = Line::new(point3, point4);

        let slope1 = line1.slope();
        let slope2 = line2.slope();

        match slope1 {
            Vertical => match slope2 {
                Vertical => {
                    if line1.start.x == line2.start.x {
                        writeln!(out, "LINE").unwrap();
                    } else {
                        writeln!(out, "NONE").unwrap();
                    }
                }
                Horizontal => {
                    writeln!(out, "POINT {:.2} {:.2}", line1.start.x, line2.start.y).unwrap();
                }
                A(s2) => {
                    let point = get_point_vertical(s2, line2.y_intercept().unwrap(), line1.start.x);
                    writeln!(out, "POINT {:.2} {:.2}", point.x, point.y).unwrap();
                }
            },
            Horizontal => match slope2 {
                Vertical => {
                    writeln!(out, "POINT {:.2} {:.2}", line2.start.x, line1.start.y).unwrap();
                }
                Horizontal => {
                    if line1.start.y == line2.start.y {
                        writeln!(out, "LINE").unwrap();
                    } else {
                        writeln!(out, "NONE").unwrap();
                    }
                }
                A(s2) => {
                    let point =
                        get_point_horizontal(s2, line2.y_intercept().unwrap(), line1.start.y);
                    writeln!(out, "POINT {:.2} {:.2}", point.x, point.y).unwrap();
                }
            },
            A(s1) => match slope2 {
                Vertical => {
                    let point = get_point_vertical(s1, line1.y_intercept().unwrap(), line2.start.x);
                    writeln!(out, "POINT {:.2} {:.2}", point.x, point.y).unwrap();
                }
                Horizontal => {
                    let point =
                        get_point_horizontal(s1, line1.y_intercept().unwrap(), line2.start.y);
                    writeln!(out, "POINT {:.2} {:.2}", point.x, point.y).unwrap();
                }
                A(s2) => {
                    if s1 == s2 {
                        if line1.y_intercept() == line2.y_intercept() {
                            writeln!(out, "LINE").unwrap();
                        } else {
                            writeln!(out, "NONE").unwrap();
                        }
                    } else {
                        let point = get_point(
                            s1,
                            line1.y_intercept().unwrap(),
                            s2,
                            line2.y_intercept().unwrap(),
                        );
                        writeln!(out, "POINT {:.2} {:.2}", point.x, point.y).unwrap();
                    }
                }
            },
        }
    }

    print!("{out}");
}

fn get_point_vertical(slope: f64, intercept: f64, x: f64) -> Point {
    Point::new(x, slope * x + intercept)
}

fn get_point_horizontal(slope: f64, intercept: f64, y: f64) -> Point {
    Point::new((y - intercept) / slope, y)
}

fn get_point(slope1: f64, b1: f64, slope2: f64, b2: f64) -> Point {
    // a1x + b1 = a2x + b2
    // (a1 - a2)x = b2 - b1
    let x = (b2 - b1) / (slope1 - slope2);
    let y = slope1 * x + b1;
    Point::new(x, y)
}
