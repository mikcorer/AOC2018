use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs;
use std::str::FromStr;
use enum_iterator::Sequence;
use crate::Direction::{Down, Left, Right};
use crate::TrackPart::{Empty, HorizontalPath, Intersection, MainCorner, SideCorner, VerticalPath};

type Error = Box<dyn std::error::Error>;
type Result<A> = std::result::Result<A, Error>;

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let mut track_map = input.parse::<TrackMap>()?;

    while track_map.count_carts() > 1 {

        let crashed_points = track_map.step();

        for Coord { x, y } in crashed_points {
            println!("Crash point: {x},{y}");
        }

        if let Some(last_cart) = track_map.last_cart() {
            println!("The last cart: {},{}", last_cart.coord.x, last_cart.coord.y);
        } else if track_map.count_carts() == 0 {
            println!("All carts are crashed");
        }
    }

    return Ok(());
}

struct TrackMap {
    map: Vec<Vec<TrackPart>>,
    carts_by_coord: HashMap<Coord, Cart>
}

impl TrackMap {

    fn step(&mut self) -> Vec<Coord> {
        let mut crashed = Vec::new();
        let mut heap: BinaryHeap<_> = self.carts_by_coord
            .keys()
            .map(|&Coord {x, y}| Reverse((y, x)))
            .collect();

        while let Some(Reverse((y, x))) = heap.pop() {
            if let Some(mut cart) = self.carts_by_coord.remove(&Coord::new(x, y)) {

                let track_part = self.map[cart.coord.y][cart.coord.x];

                let next_turn = match (track_part, cart.dir) {
                    (MainCorner, Right | Left)                 => Turn::Left,
                    (MainCorner, _)                            => Turn::Right,
                    (SideCorner, Right | Left)                 => Turn::Right,
                    (SideCorner, _)                            => Turn::Left,
                    (Intersection, _)                          => cart.next_turn,
                    (VerticalPath | HorizontalPath | Empty, _) => Turn::Straight
                };

                if track_part == Intersection {
                    cart.next_turn = cart.next_turn.next().or(Turn::first()).unwrap();
                }

                cart.turn(next_turn);
                cart.step();

                if self.carts_by_coord.contains_key(&cart.coord) {
                    crashed.push(cart.coord);
                    self.carts_by_coord.remove(&cart.coord);
                } else {
                    self.carts_by_coord.insert(cart.coord, cart);
                }
            }
        }

        crashed
    }
    
    fn count_carts(&self) -> usize {
        self.carts_by_coord.len()
    }

    fn last_cart(&self) -> Option<&Cart> {
        if self.count_carts() == 1 {
            self.carts_by_coord.values().next()
        } else {
            None
        }
    }
}

impl FromStr for TrackMap {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {

        let mut track_map = Vec::new();
        let mut carts = HashMap::new();

        for (y, line) in s.lines().enumerate() {
            let mut map_row = Vec::new();

            for (x, ch) in line.chars().enumerate() {
                let token = String::from(ch);
                let track_part = token.parse::<TrackPart>()?;

                map_row.push(track_part);

                if let Ok(dir) = token.parse::<Direction>() {
                    let coord = Coord::new(x, y);
                    let cart = Cart::new(coord, dir);
                    carts.insert(coord, cart);
                }
            }
            track_map.push(map_row);
        }

        Ok(TrackMap {
            map: track_map,
            carts_by_coord: carts
        })
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum TrackPart {
    VerticalPath,
    HorizontalPath,
    MainCorner,
    SideCorner,
    Intersection,
    Empty
}

impl FromStr for TrackPart {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
         match s {
            "|" | "^" | "v" => Ok(VerticalPath),
            "-" | "<" | ">" => Ok(HorizontalPath),
            "/"             => Ok(MainCorner),
            "\\"            => Ok(SideCorner),
            "+"             => Ok(Intersection),
            " "             => Ok(Empty),
            _               => Err(Error::from("Unknown track part"))
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Sequence)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "^" => Ok(Direction::Up),
            "v" => Ok(Down),
            "<" => Ok(Left),
            ">" => Ok(Right),
            _   => Err(Error::from("Unknown track part"))
        }
    }
}

struct Cart {
    coord: Coord,
    dir: Direction,
    next_turn: Turn
}

impl Cart {

    fn new(coord: Coord, dir: Direction) -> Self {
        Cart {
            coord,
            dir,
            next_turn: Turn::Left
        }
    }

    fn step(&mut self) {
        use Direction::*;

        match self.dir {
            Up => self.coord.y -= 1,
            Down => self.coord.y += 1,
            Left => self.coord.x -= 1,
            Right => self.coord.x += 1
        };
    }

    fn turn(&mut self, turn_dir: Turn) {
        self.dir = match turn_dir {
            Turn::Right => self.dir.next().or(Direction::first()).unwrap(),
            Turn::Left => self.dir.previous().or(Direction::last()).unwrap(),
            Turn::Straight => self.dir
        };
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Sequence)]
enum Turn {
    Left,
    Straight,
    Right
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
struct Coord {
    x: usize,
    y: usize
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Coord {x, y}
    }
}