#![allow(unused, dead_code)]

use advent_of_code::aoc2023::*;
use anyhow::Result;

fn main() -> Result<()>{
    let ans = crate::day_05::part_two(Input::new(crate::day_05::DAY_5))?;
    println!("{ans:#?}");
    Ok(())
}
