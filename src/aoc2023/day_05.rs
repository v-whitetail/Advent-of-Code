use crate::aoc2023::Input;
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




pub fn part_one() -> Result<()> {

    Input::new(file!()).read();

//    let input = read_to_string("src/aoc2023/input/day_05.nu")?;
//    let input = TEST_INPUT.to_owned();
//
//    let mut map = BTreeSet::<MapItem>::new();
//    map.insert(MapItem::default());
//
//    let seeds = input
//        .lines()
//        .filter_map( |line| Line::parse(line).ok() )
//        .find_map( |(_, line)| line.as_seeds() )
//        .expect("seeds parsing");
//    input
//        .lines()
//        .filter_map( |line| Line::parse(line).ok() )
//        .filter_map( |(_, line)| line.as_map_table() )
//        .for_each( |map_item| {
//            map_item.insert_item(&mut map)
//        });
//    
//    println!("{map:?}");

    Ok(())

}
pub fn part_two() -> Result<()> {

    let input = read_to_string("src/aoc2023/input/day_05.nu")?;

    println!("hello, day 05 part 2");

    Ok(())

}





#[derive(Debug, Clone, Copy)]
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
    fn insert_item(self, map: &mut BTreeSet<Self>) {
        todo!()
    }
    fn split(&self, other: &Self) -> Vec<Self> {
        [ 
            self.range[0], self.range[1],
            other.range[0], other.range[1],
        ]
            .into_iter()
            .sorted()
            .dedup()
            .map_windows(
                |&[lhs, rhs]|
                Self::new(
                    [lhs, rhs],
                    self.eq(&lhs).then_some(self.coeff).unwrap_or(0)
                    + other.eq(&lhs).then_some(other.coeff).unwrap_or(0)
                    ))
            .collect()
    }
    fn is_overlapping(&self, other: &Self) -> bool {
        self == &other.range[0]
            || self == &other.range[1]
            || other == &self.range[0]
            || other == &self.range[1]
    }
    fn from_input(target: u64, source: u64, width: u64) -> Self {
        let range = [source, source + width];
        let coeff = target as i64 - source as i64;
        Self{range, coeff}
    }
    fn new(range: [u64; 2], coeff: i64) -> Self {
        Self{range, coeff}
    }
    fn default() -> Self {
        let range = [u64::MIN, u64::MAX];
        let coeff = 0;
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
    fn as_seeds(self) -> Option<Vec<u64>> {
        if let Self::Seeds(seeds) = self { Some(seeds) }
        else { None }
    }
    fn as_map_label(self) -> Option<&'a str> {
        if let Self::MapLabel(map_label) = self { Some(map_label) }
        else { None }
    }
    fn as_map_table(self) -> Option<MapItem> {
        if let Self::MapTable(map_label) = self { Some(map_label) }
        else { None }
    }
    fn parse(s:&'a str) -> IResult<&str, Self> {
        alt(( Self::seeds, Self::map_label, Self::map_table, ))(s)
    }
    fn seeds(s:&'a str) -> IResult<&str, Self> {
        map(
            preceded( tag("seeds:"), many1(preceded(space1, u64)) ),
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
                  Self::MapTable(MapItem::from_input(source, target, width))
           )(s)
    }
}




const TEST_INPUT: &str = r#"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;
