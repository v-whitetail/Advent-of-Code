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
    let input = TEST_INPUT.to_owned();

    let mut map = BTreeSet::<MapItem>::new();
    map.insert(MapItem::default());

    let seeds = input
        .lines()
        .filter_map( |line| Line::parse(line).ok() )
        .find_map( |(_, line)| line.as_seeds() )
        .expect("seeds parsing");
    input
        .lines()
        .filter_map( |line| Line::parse(line).ok() )
        .filter_map( |(_, line)| line.as_map_table() )
        .for_each( |map_item| {
            map_item.insert_item(&mut map)
        });
    
    println!("{map:#?}");

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
    fn insert_item(self, map: &mut BTreeSet<Self>) {
        if let Some(item) = map.get(&self) {
            let splits = item.split(&self);
            let len = splits.iter().filter(|split|split.is_some()).count();
            if 1 < len {
                splits
                    .into_iter()
                    .filter_map( |split| split )
                    .for_each( |split| split.insert_item(map) )
                    ;
            }
        }
    }
    fn split(&self, other: &Self) -> [Option<Self>; 3] {
        use std::cmp::Ordering::{Less as L, Equal as E, Greater as G};
        let [smin, smax, omin, omax] = [
            self.range[0], self.range[1],
            other.range[0], other.range[1]
        ];
        match ( smin.cmp(&omin), smax.cmp(&omax)) {
            (L,L) => [
                Some(Self::new([smin, omin], self.coeff)),
                Some(Self::new([omin, smax], self.coeff + other.coeff)),
                Some(Self::new([smax, omax], other.coeff)),
            ],
            (L,E) => [
                Some(Self::new([smin, omin], self.coeff)),
                Some(Self::new([omin, smax], self.coeff + other.coeff)),
                None,
            ],
            (L,G) => [
                Some(Self::new([smin, omin], self.coeff)),
                Some(Self::new([omin, omax], self.coeff + other.coeff)),
                Some(Self::new([omax, smax], other.coeff)),
            ],
            (E,L) => [
                Some(Self::new([smin, smax], self.coeff + other.coeff)),
                Some(Self::new([smax, omax], other.coeff)),
                None,
            ],
            (E,E) => panic!("equal, equal"),
//            (E,E) => [
//                Some(Self::new([smin, omax], self.coeff + other.coeff)),
//                None,
//                None,
//            ],
            (E,G) => [
                Some(Self::new([omin, omax], self.coeff + other.coeff)),
                Some(Self::new([omax, smax], self.coeff)),
                None,
            ],
            (G,L) => [
                Some(Self::new([omin, smin], other.coeff)),
                Some(Self::new([smin, smax], self.coeff + other.coeff)),
                Some(Self::new([smax, omax], other.coeff)),
            ],
            (G,E) => [
                Some(Self::new([omin, smin], other.coeff)),
                Some(Self::new([smin, smax], self.coeff + other.coeff)),
                None,
            ],
            (G,G) => [
                Some(Self::new([omin, smin], other.coeff)),
                Some(Self::new([smin, omax], self.coeff + other.coeff)),
                Some(Self::new([omax, smax], self.coeff)),
            ],
        }
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
