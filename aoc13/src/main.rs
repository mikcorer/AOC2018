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

    loop {

        let crashed_points = track_map.step();

        for Coord { x, y } in crashed_points {
            println!("Crash point: {x},{y}");
            if let Some(last_cart) = track_map.last_cart() {
                println!("Last cart: {},{}", last_cart.coord.x, last_cart.coord.y);
                return Ok(());
            }
        }
    }
}

struct TrackMap {
    map: Vec<Vec<TrackPart>>,
    carts_by_coord: HashMap<Coord, Cart>
}

impl TrackMap {

    fn step(&mut self) -> Vec<Coord> {
        let mut heap = BinaryHeap::new();
        let mut crashed = Vec::new();

        for &coord in self.carts_by_coord.keys() {
            heap.push(Reverse((coord.y, coord.x)));
        }

        while let Some(Reverse((y, x))) = heap.pop() {
            if let Some(mut cart) = self.carts_by_coord.remove(&Coord::new(x, y)) {
                let track_part = self.map[cart.coord.y][cart.coord.x];

                if track_part == MainCorner {
                    if cart.dir == Right || cart.dir == Left {
                        cart.turn(Turn::Left);
                    } else {
                        cart.turn(Turn::Right);
                    }
                } else if track_part == SideCorner {
                    if cart.dir == Left || cart.dir == Right {
                        cart.turn(Turn::Right);
                    } else {
                        cart.turn(Turn::Left);
                    }
                } else if track_part == Intersection {
                    cart.intersection_turn();
                }

                cart.next();
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
        let mut cur_y = 0;

        for line in s.lines() {
            let mut cur_x = 0;
            let mut map_row = Vec::new();

            for ch in line.chars() {
                let token = String::from(ch);

                if let Ok(dir) = token.parse::<Direction>() {
                    let coord = Coord::new(cur_x, cur_y);
                    let cart = Cart::new(coord, dir);
                    carts.insert(coord, cart);
                }
                let track_part = token.parse::<TrackPart>()?;
                map_row.push(track_part);
                cur_x += 1;
            }
            track_map.push(map_row);
            cur_y += 1;
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

    fn next(&mut self) {
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

    fn intersection_turn(&mut self) {
        self.turn(self.next_turn);
        self.next_turn = self.next_turn.next().or(Turn::first()).unwrap();
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