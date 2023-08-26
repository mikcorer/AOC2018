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




    Ok(())
}






