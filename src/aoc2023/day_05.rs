use anyhow::{ Result, anyhow, };
use std::fs::*;
use itertools::any;
use std::collections::*;
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

    println!("hello, day 05 part 1");

    let input = read_to_string("src/aoc2023/input/day_05.nu")?;

    Ok(())

}
pub fn part_two() -> Result<()> {

    let input = read_to_string("src/aoc2023/input/day_05.nu")?;

    println!("hello, day 05 part 2");

    Ok(())

}





#[derive(Debug, Clone)]
struct MapItem {
    range: [u64; 2],
    coeff: i64,
}
impl std::cmp::Ord for MapItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).expect("impl ord for mapitem")
    }
}
impl std::cmp::PartialOrd for MapItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            Some(std::cmp::Ordering::Equal)
        } else {
            self.range[0].partial_cmp(&other.range[0])
        }
    }
}
impl std::cmp::PartialOrd<u64> for MapItem {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        if self == other {
            Some(std::cmp::Ordering::Equal)
        } else {
            self.range[0].partial_cmp(&other)
        }
    }
}
impl std::cmp::Eq for MapItem {}
impl std::cmp::PartialEq for MapItem {
    fn eq(&self, other: &Self) -> bool {
        self.is_overlapping(other)
    }
}
impl std::cmp::PartialEq<u64> for MapItem {
    fn eq(&self, other: &u64) -> bool {
        self.range[0] <= *other && *other <= self.range[1]
    }
}
impl MapItem {
    fn separate(&self, other: &Self) -> [Self; 3] {
        if self != other {panic!("attempted to separate without overlap")};
        todo!()
    }
    fn is_overlapping(&self, other: &Self) -> bool {
        self == &other.range[0]
            || self == &other.range[1]
            || other == &self.range[0]
            || other == &self.range[1]
    }
    fn new(target: u64, source: u64, width: u64) -> Self {
        let range = [source, source + width];
        let coeff = target as i64 - source as i64;
        Self{range, coeff}
    }
}





#[derive(Debug, Clone)]
enum Line<'a>{
    Seeds(Vec<u64>),
    MapLabel(&'a str),
    MapTable(MapItem),
}
impl<'a> Line<'a> {
    fn parse(s:&'a str) -> IResult<&str, Self> {
        alt(( Self::seeds, Self::map_label, Self::map_table, ))(s)
    }
    fn seeds(s:&'a str) -> IResult<&str, Self> {
        map(
            preceded( tag("seeds: "), many1(preceded(space1, u64)) ),
            |vec| Self::Seeds(vec)
           )(s)
    }
    fn map_label(s:&'a str) -> IResult<&str, Self> {
        map(
            take_until(" map:"),
            |name| Self::MapLabel(name)
           )(s)
    }
    fn map_table(s:&'a str) -> IResult<&str, Self> {
        map(
            tuple((
                    u64,
                    preceded(space1, u64),
                    preceded(space1, u64)
                  )),
                  |(source, target, width)|
                  Self::MapTable(MapItem::new(source, target, width))
           )(s)
    }
}
