use anyhow::{ Result, anyhow, };
use std::fs::*;
use std::ops::*;
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

use crate::aoc2023::Input;


pub const DAY_7: &str = file!();


pub fn part_one(input: Input) -> Result<i32> {

    input.read();

    let ans = 0;

    Ok(ans)

}
#[test]
fn test_part_one() {
    let ans = part_one(Input::new(DAY_7).test());
    assert_eq!(0, ans.unwrap());
}
