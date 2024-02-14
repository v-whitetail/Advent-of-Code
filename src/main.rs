#![allow(unused, dead_code)]

use advent_of_code::aoc2023::*;
use anyhow::Result;

fn main() -> Result<()>{
    let input = Input::new(crate::day_09::FILE);
    let ans = crate::day_09::part_two(input)?;
    println!("{ans:#?}");
    Ok(())
}
