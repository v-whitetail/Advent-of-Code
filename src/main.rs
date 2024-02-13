#![allow(unused, dead_code)]

use advent_of_code::aoc2023::*;
use anyhow::Result;

fn main() -> Result<()>{
    let ans = crate::day_08::part_one(Input::new(crate::day_08::DAY_8))?;
    println!("{ans:#?}");
    Ok(())
}
