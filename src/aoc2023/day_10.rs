use anyhow::{ Result, anyhow, };
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


pub const FILE: &str = file!();


// pub fn part_two(input: Input) -> Result<i32> {
//     let input = input.read();
//     Ok(0)
// }
// #[test]
// fn test_part_two() {
//     let ans = part_two(Input::new(FILE).test()).unwrap();
//     assert_eq!(0, ans);
// }
// #[test]
// fn ans_part_two() {
//     let ans = part_one(Input::new(FILE)).unwrap();
//     assert_eq!(0, ans);
// }


pub fn part_one(input: Input) -> Result<usize> {
    let input = input.read();
    let mut map = Map::parse(&input)?;
    let mut index = map.start();
    map.path.push(index);
    while let Some(next_index) = map.walk(index) {
        index = next_index;
    };
    let ans = map.path.len() / 2;
    Ok(ans)
}
#[test]
fn test_part_one() {
    let ans = part_one(Input::new(FILE).test()).unwrap();
    assert_eq!(8, ans);
}
 #[test]
 fn ans_part_one() {
     let ans = part_one(Input::new(FILE)).unwrap();
     assert_eq!(6903, ans);
 }

#[derive(Debug)]
struct Map {
    rows: usize,
    cols: usize,
    data: Vec<Tile>,
    path: Vec<usize>,
}
impl Map {
    fn walk (&mut self, i: usize) -> Option<usize> {
        let is_first_row = |i: &usize| *i <= self.cols;
        let is_first_col = |i: &usize|  0 == i % self.cols;
        let is_last_row  = |i: &usize| *i >= self.cols * (self.rows - 1);
        let is_last_col  = |i: &usize|  0 == (i + 1) % self.cols;
        let to_north = |i: &usize| *i - self.cols;
        let to_south = |i: &usize| *i + self.cols;
        let to_east  = |i: &usize| *i + 1;
        let to_west  = |i: &usize| *i - 1;
        let [mut north, mut south, mut east, mut west] = [Tile::None; 4];
        let tile = &self.data[i];
        match tile {
            Tile::NS => {
                if !is_first_row(&i) { north = self.data[to_north(&i)] };
                if !is_last_row(&i)  { south = self.data[to_south(&i)] };
            },
            Tile::EW => {
                if !is_first_col(&i) { east  = self.data[to_east(&i)]  };
                if !is_last_col(&i)  { west  = self.data[to_west(&i)]  };
            },
            Tile::NE => {
                if !is_first_row(&i) { north = self.data[to_north(&i)] };
                if !is_last_col(&i)  { east  = self.data[to_east(&i)]  };
            },
            Tile::NW => {
                if !is_first_row(&i) { north = self.data[to_north(&i)] };
                if !is_first_col(&i) { west  = self.data[to_west(&i)]  };
            },
            Tile::SW => {
                if !is_last_row(&i)  { south = self.data[to_south(&i)] };
                if !is_first_col(&i) { west  = self.data[to_west(&i)]  };
            },
            Tile::SE => {
                if !is_last_row(&i)  { south = self.data[to_south(&i)] };
                if !is_last_col(&i)  { east  = self.data[to_east(&i)]  };
            },
            Tile::Start => {
                if !is_first_row(&i) { north = self.data[to_north(&i)] };
                if !is_last_row(&i)  { south = self.data[to_south(&i)] };
                if !is_last_col(&i)  { east  = self.data[to_east(&i)]  };
                if !is_first_col(&i) { west  = self.data[to_west(&i)]  };
            },
            _ => {},
        };
        if north.is_south_joint(tile) && !self.path.contains(&to_north(&i)) {
            self.path.push(to_north(&i));
            return Some(to_north(&i));
        };
        if tile.is_south_joint(&south) && !self.path.contains(&to_south(&i)) {
            self.path.push(to_south(&i));
            return Some(to_south(&i));
        };
        if tile.is_east_joint(&east) && !self.path.contains(&to_east(&i)) {
            self.path.push(to_east(&i));
            return Some(to_east(&i));
        };
        if west.is_east_joint(tile) && !self.path.contains(&to_west(&i)) {
            self.path.push(to_west(&i));
            return Some(to_west(&i));
        };
        return None
    }
    fn start(&self) -> usize {
        self.data
            .iter()
            .position( |tile| *tile == Tile::Start )
            .expect("\'S\' not present in map")
    }
    fn parse(s: &str) -> Result<Self> {
        let (_, rows) = Self::parse_rows(s)
            .map_err(|err| err.to_owned())?;
        let (_, cols) = Self::parse_cols(s)
            .map_err(|err| err.to_owned())?;
        let (_, data) = Self::parse_data(s)
            .map_err(|err| err.to_owned())?;
        let path = Vec::new();
        Ok( Self{ rows, cols, data, path, } )
    }
    fn parse_rows(s: &str) -> IResult<&str, usize> {
        many1_count(terminated(Self::parse_cols, line_ending))(s)
    }
    fn parse_cols(s: &str) -> IResult<&str, usize> {
        many1_count(one_of("|-LJ7F.S"))(s)
    }
    fn parse_data(s: &str) -> IResult<&str, Vec<Tile>> {
        many1(terminated(Tile::parse, opt(line_ending)))(s)
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
enum Tile { NS, EW, NE, NW, SW, SE, #[default] None, Start, }
impl Tile {
    fn to_edge(&self, index: &usize, size: [&usize; 2]) -> Option<[usize; 2]>{
        let is_last_row = |i: &usize, s: &[&usize; 2]| s[1] * (s[0] - 1) <= *i;
        let is_last_col = |i: &usize, s: &[&usize; 2]| (index + 1) % size[1] == 0;
        let to_east  = |i: &usize, s: &[&usize; 2]| *index + 1;
        let to_south = |i: &usize, s: &[&usize; 2]| *index + size[1];
        todo!();
    }
    fn is_south_joint(&self, other: &Self) -> bool {
        let from_north = match self {
            Self::NS => true,
            Self::SW => true,
            Self::SE => true,
            Self::Start => true,
            _ => false,
        };
        let to_south = match other {
            Self::NS => true,
            Self::NE => true,
            Self::NW => true,
            Self::Start => true,
            _ => false,
        };
        from_north && to_south
    }
    fn is_east_joint(&self, other: &Self) -> bool {
        let from_east = match self {
            Self::EW => true,
            Self::NE => true,
            Self::SE => true,
            Self::Start => true,
            _ => false,
        };
        let to_west = match other {
            Self::NW => true,
            Self::SW => true,
            Self::EW => true,
            Self::Start => true,
            _ => false,
        };
        from_east && to_west
    }
    fn parse(s: &str) -> IResult<&str, Self> {
        map(
            anychar,
            |char| match char {
                '|' => Self::NS,
                '-' => Self::EW,
                'L' => Self::NE,
                'J' => Self::NW,
                '7' => Self::SW,
                'F' => Self::SE,
                '.' => Self::None,
                'S' => Self::Start,
                ___ => unreachable!(),
            })(s)
    }
}
