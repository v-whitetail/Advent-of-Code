use anyhow::{ Result, anyhow, };
use itertools::{
    Itertools,
    Either::Left as Left,
    Either::Right as Right,
};
use std::{
    fs::*,
    fmt::*,
    ops::*,
    collections::*,
    borrow::BorrowMut,
    cmp::Ordering::Less as Less,
    cmp::Ordering::Equal as Equal,
    cmp::Ordering::Greater as Greater,
};
use nom::{
    IResult,
    multi::*,
    branch::*,
    sequence::*,
    combinator::*,
    bytes::complete::*,
    character::complete::*,
};




pub fn part_one() -> Result<()> {

    let input = read_to_string("src/aoc2023/input/day_03.log")?;

    Ok(())

}





pub fn part_two() -> Result<()> {

    Ok(())

}
