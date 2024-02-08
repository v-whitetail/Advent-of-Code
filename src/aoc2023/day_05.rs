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





pub const DAY_5: &str = file!();





use crate::aoc2023::Input;
pub fn part_one(input: Input) -> Result<i64> {

    let input = input.read();

    let (_, almanac) = Almanac::parse(&input)
        .map_err(|err| anyhow!(err.to_owned()))?;

    let mut seeds = almanac.seeds.clone();

    for table in almanac.tables {
        for seed in seeds.values.iter_mut() {
            for (range, offset) in table.function_args.iter() {
                if range.contains(seed) {
                    *seed += offset;
                    break;
                };
            };
        };
    };

    Ok(seeds.min())

}
#[test]
fn test_part_one() {
    let ans = part_one(Input::new(DAY_5).test());
    assert_eq!(35, ans.unwrap());
}
pub fn part_two(input: Input) -> Result<i64> {

    let input = input.read();

    let (_, almanac) = Almanac::parse(&input)
        .map_err(|err| anyhow!(err.to_owned()))?;

    let mut seeds = almanac.seeds
        .values
        .chunks(2)
        .map( |bounds| bounds[0] .. bounds[1] )
        .collect::<Vec<_>>();

    for table in almanac.tables {
        for seed in seeds.iter() {
            for (range, offset) in table.function_args.iter() {
            };
        };
    };
    Ok(0)

}
#[test]
fn test_part_two() {
    let ans = part_two(Input::new(DAY_5).test());
    assert_eq!(46, ans.unwrap());
}





#[derive(Debug)]
pub struct Almanac<'a> {
    seeds: Seeds,
    tables: Vec<TransTable<'a>>,
}
impl<'a> Almanac<'a> {
    pub fn parse(s: &'a str) -> IResult<&str, Self> {
        map(
            pair(
                terminated(Seeds::parse, line_ending),
                separated_list1(multispace1, TransTable::parse)
                ), |(seeds, tables)|
            Self {seeds, tables}
           )(s)
    }
}


#[derive(Debug)]
pub struct TransTable<'a> {
    source: &'a str,
    target: &'a str,
    instructions: Vec<Trans>,
    function_args: Vec<(Range<i64>, i64)>,
}
impl<'a> TransTable<'a> {
    pub fn parse(s: &'a str) -> IResult<&str, Self> {
        let (input, mut trans_table) = Self::parse_instructions(s)?;
        &trans_table.instructions
            .iter()
            .for_each( |t_struct| {
                let key = t_struct.source .. (t_struct.source + t_struct.range);
                let val = t_struct.target - t_struct.source;
                trans_table.function_args.push((key, val));
            });
        Ok((input, trans_table))
    }
    pub fn parse_instructions(s: &'a str) -> IResult<&str, Self> {
        map(
            separated_pair(
                separated_pair(alpha1, tag("-to-"), alpha1),
                pair(tag(" map:"), line_ending),
                separated_list1(line_ending, Trans::parse)
                ), |((source, target), instructions)| {
                let function_args = Vec::new();
                Self {source, target, instructions, function_args}
            })(s)
    }
}


#[derive(Debug)]
pub struct Trans {
    source: i64,
    target: i64,
    range: i64,
}
impl Trans {
    pub fn parse(s: &str) -> IResult<&str, Self> {
        map(
            tuple((
                    i64,
                    preceded(space1, i64),
                    preceded(space1, i64),
                    )), |(target, source, range)|
            Self{source, target, range}
           )(s)
    }
}


#[derive(Debug, Default, Clone)]
pub struct Seeds {
    values: Vec<i64>
}
impl Seeds {
    fn min(&self) -> i64 {
        let mut values = self.values.clone();
        values.sort();
        values[0]
    }
    fn parse(s: &str) -> IResult<&str, Self> {
        map(
            delimited(
                tag("seeds: "),
                separated_list1(space1, i64),
                line_ending
                ), |values| Self {values}
           )(s)
    }
}
