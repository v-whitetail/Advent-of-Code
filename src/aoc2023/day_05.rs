use anyhow::{ Result, anyhow, };
use std::fs::*;
use std::collections::*;
use itertools::Itertools;
use nom::{
    IResult,
    multi::*,
    branch::*,
    sequence::*,
    combinator::*,
    bytes::complete::*,
    character::complete::*,
};





pub const DAY_5: &str = file!();





use crate::aoc2023::Input;
pub fn part_one(input: Input) -> Result<u32> {

    Input::new(file!()).read();

    let input = read_to_string("src/aoc2023/inputs/day_5.nu")?;

    Ok(0)

}
#[test]
fn test_part_one() {
    let ans = part_one(Input::new(DAY_5).test());
    assert_eq!(35, ans.unwrap());
}




pub fn part_two() -> Result<()> {

    let input = read_to_string("src/aoc2023/input/day_5.nu")?;

    Ok(())

}
