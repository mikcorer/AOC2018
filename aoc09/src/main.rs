#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use std::fs;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::str::FromStr;
use regex::Regex;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let game: GameSetting = input.parse()?;
    let mut scores: HashMap<u32, u32> = HashMap::new();
    let mut zipper = ListZipper::new();
    for (value, player) in (1..=game.last_marble).zip((0..game.players).cycle()) {
        if value % 23 != 0 {
            zipper.next();
            zipper.push_next(value);
            zipper.next();
        } else {
            (0..7).for_each(|_| zipper.prev());
            let entry = scores.entry(player).or_insert(0);
            *entry += value + zipper.get_num();
            zipper.remove();
        }
    }

    let max_score = scores.values().max().unwrap_or(&0);
    println!("{max_score}");
    assert_eq!(*max_score, 3482394794);

    Ok(())
}

#[derive(Debug)]
struct GameSetting {
    players: u32,
    last_marble: u32
}

impl FromStr for GameSetting {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {

        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?<p>[0-9]+) players; last marble is worth (?<m>[0-9]+) point").unwrap();
        }

        let caps = RE.captures(s).ok_or("Cannot parse game data")?;
        Ok(GameSetting {
            players: caps["p"].parse()?,
            last_marble: caps["m"].parse()?
        })
    }
}

type Cursor = Rc<RefCell<Node>>;

#[derive(Debug)]
struct Node {
    num: u32,
    next: Option<Cursor>,
    prev: Option<Weak<RefCell<Node>>>,
}

//TODO: Will be safer rid out from cycle

#[derive(Debug)]
struct ListZipper {
    cursor: Cursor
}

impl ListZipper {

    fn new() -> ListZipper {

        let node =  Rc::new(RefCell::new(Node {
            num: 0,
            next: None,
            prev: None
        }));

        {
            let mut _node = node.borrow_mut();
            _node.next = Some(node.clone());
            _node.prev = Some(Rc::downgrade(&node));
        }

        ListZipper {
            cursor: node
        }
    }

    fn next(&mut self) {
        self.cursor = {
            let cur = self.cursor.borrow();
            cur.next.as_ref().unwrap().clone()
        };
    }

    fn prev(&mut self) {
        self.cursor = {
            let cur = self.cursor.borrow();
            cur.prev.as_ref().unwrap().upgrade().unwrap()
        };
    }

    fn get_num(&self) -> u32 {
        self.cursor.borrow().num
    }

    fn push_next(&mut self, num: u32) {
        let next = self.cursor.borrow_mut().next.take().unwrap();
        let prev = next.borrow_mut().prev.take().unwrap();
        let new_node = Rc::new(RefCell::new(Node {
            num,
            next: Some(next.clone()),
            prev: Some(prev.clone())
        }));

        next.borrow_mut().prev = Some(Rc::downgrade(&new_node));
        prev.upgrade().unwrap().borrow_mut().next = Some(new_node)
    }

    fn remove(&mut self) {

        let (next, prev) = {
            let mut cur = self.cursor.borrow_mut();
            (cur.next.take().unwrap(), cur.prev.take().unwrap())
        };

        prev.upgrade().unwrap().borrow_mut().next = Some(next.clone());
        next.borrow_mut().prev = Some(prev);
        self.cursor = next;
    }
}