#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::error::Error;
use std::fs;
use std::str::FromStr;
use regex::Regex;

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let mut point_map = PointMap::default();
    for line in input.lines() {
        let p: Point = line.parse()?;
        point_map.add(p);
    }

    loop {

        let (_, h) = point_map.dimension();
        if h == LETTER_HEIGHT {
            let res = point_map.to_string();
            println!("{}", point_map.to_string());
            break;
        }
        point_map.step();
    }
    Ok(())
}

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32
}

impl Point {

    fn next_step(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }
}

impl FromStr for Point {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?x)
            position\=
                <\s*
                    (?<x>\-?\d+)
                    \,\s*
                    (?<y>\-?\d+)
                >
            \s
            velocity\=
                <\s*
                    (?<dx>\-?\d+)
                    \,\s*
                    (?<dy>\-?\d+)
                >"
            ).unwrap();
        }

        let caps = RE.captures(s).ok_or("Cannot parse point")?;
        Ok(Point {
            x: caps["x"].parse()?,
            y: caps["y"].parse()?,
            dx: caps["dx"].parse()?,
            dy: caps["dy"].parse()?,
        })
    }
}

#[derive(Clone, Copy, Debug)]
struct Bound {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32
}

impl Bound {

    fn width(&self) -> usize {
        (self.max_x - self.min_x) as usize + 1
    }

    fn height(&self) -> usize {
        (self.max_y - self.min_y) as usize + 1
    }
}

#[derive(Debug, Default)]
struct PointMap {
    points: Vec<Point>
}

impl PointMap {

    fn add(&mut self, p: Point) {
        self.points.push(p);
    }

    fn step(&mut self) {
        self.points.iter_mut().for_each(|p| p.next_step());
    }

    fn bound(&self) -> Bound {

        let (min_x, max_x) = self.points
            .iter()
            .map(|p| p.x)
            .fold((std::i32::MAX, std::i32::MIN), | (min_x, max_x), cur| {
                (min_x.min(cur), max_x.max(cur))
            });

        let (min_y, max_y) = self.points
            .iter()
            .map(|p| p.y)
            .fold((std::i32::MAX, std::i32::MIN), | (min_y, max_y), cur| {
                (min_y.min(cur), max_y.max(cur))
            });

        Bound {
            min_x,
            max_x,
            min_y,
            max_y
        }
    }

    fn dimension(&self) -> (usize, usize) {

        let bound = self.bound();

        let h = (bound.max_y - bound.min_y) as usize + 1;
        let w = (bound.max_x - bound.min_x) as usize + 1;

        (w, h)
    }
}

const LETTER_HEIGHT: usize = 10;

impl ToString for PointMap {

    fn to_string(&self) -> String {
        let bound = self.bound();
        let (w, h) = (bound.width(), bound.height());
        let mut grid = vec![vec![b'.'; w]; h];

        for p in self.points.iter() {
            let x = (p.x - bound.min_x) as usize;
            let y = (p.y - bound.min_y) as usize;
            grid[y][x] = b'#';
        }

        let mut str = String::new();
        for row in grid {
            str.push_str(&String::from_utf8(row).unwrap());
            str.push('\n');
        }
        str
    }
}
