use anyhow::{ Result, anyhow, };
use nom::bytes::streaming::take;
use std::fs::*;
use std::cmp::*;
use std::ops::*;
use std::sync::*;
use std::borrow::*;
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


pub const DAY_8: &str = file!();


pub fn part_two(input: Input) -> Result<u64> {
    let input_string = input.read();
    let (_, instructions) = Instructions::parse(&input_string)
        .map_err(|err| anyhow!(err.to_owned()))?;
    let mut positions = instructions.map.data
        .iter()
        .map( |(position, _)| position.clone() )
        .filter( |position| position.as_bytes()[2] == b'A' )
        .collect::<Vec<_>>();
    let mut targets = instructions.map.data
        .iter()
        .map( |(position, _)| position.clone() )
        .filter( |position| position.as_bytes()[2] == b'Z' )
        .collect::<Vec<_>>();
    positions.sort();
    targets.sort();

    let mut count = 0u64;
    for direction in instructions.directions.iter().cycle() {
        if positions == targets { break; };
        for position in positions.iter_mut() {
            *position = match direction {
                Direction::L => instructions.map.data[position][0].clone(),
                Direction::R => instructions.map.data[position][1].clone(),
            };
        };
        positions.sort();
        count += 1;
    };
    Ok(count)
}
#[test]
fn test_part_two() {
    let ans = part_two(Input::new(DAY_8).test()).unwrap();
    assert_eq!(6, ans);
}


const SOURCE: &str = "AAA";
const TARGET: &str = "ZZZ";


pub fn part_one(input: Input, source: &str, target: &str) -> Result<u64> {
    let input = input.read();
    let (_, instructions) = Instructions::parse(&input)
        .map_err(|err| anyhow!(err.to_owned()))?;
    let mut count = 0u64;
    let mut position = source.into();
    for direction in instructions.directions.iter().cycle() {
        if position == target { break; };
        position = match direction {
            Direction::L => &instructions.map.data[position][0],
            Direction::R => &instructions.map.data[position][1],
        };
        count += 1;
    };
    Ok(count)
}
#[test]
fn test_part_one() {
    let ans = part_one(Input::new(DAY_8).test(), SOURCE, TARGET).unwrap();
    assert_eq!(2, ans);
}
#[test]
fn ans_part_one() {
    let ans = part_one(Input::new(DAY_8), SOURCE, TARGET).unwrap();
    assert_eq!(21797, ans);
}


#[derive(Debug, Clone)]
struct Instructions {
    directions: Vec<Direction>,
    map: Map,
}
impl Instructions {
    fn parse(s: &str) -> IResult<&str, Self> {
        map(
            separated_pair(
                many1(Direction::parse),
                multispace1,
                Map::parse
                ),
                |(directions, map)|
                Self{ directions, map }
           )(s)
    }
}


#[derive(Debug, Copy, Clone)]
enum Direction { L, R }
impl Direction {
    fn parse(s: &str) -> IResult<&str, Self> {
        map(
            alt((tag("L"), tag("R"))),
            |tag| match tag{
                "L" => Self::L,
                "R" => Self::R,
                _ => unreachable!(),
            })(s)
    }
}


#[derive(Debug, Clone)]
struct Map {
    data: HashMap<Arc<str>, [Arc<str>; 2]>,
}
impl Map {
    fn parse(s: &str) -> IResult<&str, Self> {
        map(
            fold_many1(
                terminated(Turn::parse, newline),
                HashMap::<Arc<str>, [Arc<str>; 2]>::new,
                |mut map, turn| {
                    map.insert(turn.position, [turn.left, turn.right]);
                    map
                }),
                |data| Self{data}
           )(s)
    }
}


#[derive(Debug, Clone)]
struct Turn {
    position: Arc<str>,
    left: Arc<str>,
    right: Arc<str>,
}
impl Turn {
    fn parse(s: &str) -> IResult<&str, Self> {
        map(
            Self::line,
            |(position, (left, right))| Self{
                position: position.into(),
                left: left.into(),
                right: right.into()
            })(s)
    }
    fn line(s: &str) -> IResult<&str, (&str, (&str, &str))> {
        separated_pair(alphanumeric1, tag(" = "), Self::duple)(s)
    }
    fn duple(s: &str) -> IResult<&str, (&str, &str)> {
        delimited(tag("("), Self::comma, tag(")"))(s)
    }
    fn comma(s: &str) -> IResult<&str, (&str, &str)> {
        separated_pair(alphanumeric1, tag(", "), alphanumeric1)(s)
    }
}
