use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::hash::Hash;
use std::str::FromStr;

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

const GENERATIONS: u32 = 500000;

fn main() -> Result<()> {
    let line = fs::read_to_string("input.txt")?;

    let mut lines = line.lines();
    let mut state = lines
        .next()
        .and_then(|s| s.split_once(": "))
        .map(|(p, s)| s.to_string())
        .unwrap();

    lines.next();
    let mut map: HashMap<&str, char> = HashMap::new();
    while let Some(next) = lines.next() {

        let (note, pot) = next
            .split_once(" => ")
            .map(|(note, pot)| (note, pot.chars().next().unwrap()))
            .unwrap();

        map.insert(note, pot);
    }

    let mut start_pos: i32 = 0;

    for _ in 0..(GENERATIONS as usize) {

        let mut tmp_state = String::from("....");
        tmp_state.push_str(&state);
        tmp_state.push_str("....");

        start_pos -= 2;

        let n = tmp_state.chars().count();
        let mut seen_first = false;
        let mut new_state = String::new();
        for i in 0..(n-5) {
            let c = &tmp_state[i..(i+5)];
            let pot = map.get(c).unwrap_or(&'.');

            if *pot == '#' {
                if !seen_first {
                    seen_first = true;
                }
                new_state.push(*pot);
            } else {
                if seen_first {
                    new_state.push(*pot);
                } else {
                    start_pos += 1;
                }
            }
        }
        let mut it = 0;
        for ch in tmp_state.chars() {
            if ch == '.' {
                it += 1;
            } else {
                break;
            }
        }

        state = new_state;
        println!("answer = {}", sum_pots_positions(&state, start_pos));
    }

    println!("answer = {}", sum_pots_positions(&state, start_pos));

    Ok(())
}

fn sum_pots_positions(state: &str, mut start_pos: i32) -> i32 {
    let mut sum = 0;
    for pot in state.chars() {
        if pot == '#' {
            sum += start_pos;
        }
        start_pos += 1;
    }
    sum
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Pot {
    Empty,
    Plant
}

struct PotsState {
    pots: Vec<Pot>
}

