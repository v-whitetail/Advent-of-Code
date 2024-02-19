use anyhow::{ Result, anyhow, };
use std::collections::*;
use petgraph::{
    algo::*,
    prelude::*,
    graph::Graph,
    data::FromElements,
    stable_graph::node_index,
};
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
    let graph = Map::parse(&input)?;
    let start = graph.start();
    let paths = dijkstra(&graph.graph, start, None, |_| 1);
    let ans = paths.values().max().unwrap();
    Ok(*ans)
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
    graph: Graph<Tile, bool, Undirected>,
}
impl Map {
    fn start(&self) -> NodeIndex<u32> {
        self.graph.node_indices()
            .find(|index| self.graph[*index] == Tile::Start)
            .expect("start node not found")
    }
    fn parse(s: &str) -> Result<Self> {
        let (_, rows) = Self::parse_rows(s)
            .map_err(|err| err.to_owned())?;
        let (_, cols) = Self::parse_cols(s)
            .map_err(|err| err.to_owned())?;
        let (_, mut graph) = Self::parse_list(s)
            .map_err(|err| err.to_owned())?;
        graph.node_indices()
            .chunks(cols)
            .into_iter()
            .enumerate()
            .for_each( |(r, indices)| { indices.into_iter().enumerate()
                .for_each( |(c, index)| {
                    let north = (0 < r).then(||index.index() - cols);
                    let south = (r < rows-1).then(||index.index() + cols);
                    let east  = (c < cols-1).then(||index.index() + 1);
                    let west  = (0 < c).then(||index.index() - 1);
                    let tile  = graph[index];
                    if let Some(north) = north {
                        let north_index = node_index(north);
                        let north_tile  = graph[north_index];
                        if north_tile.is_south_joint(&tile) {
                            graph.update_edge(north_index, index, true);
                        };
                    };
                    if let Some(south) = south {
                        let south_index = node_index(south);
                        let south_tile  = graph[south_index];
                        if tile.is_south_joint(&south_tile) {
                            graph.update_edge(index, south_index, true);
                        };
                    };
                    if let Some(west) = west {
                        let west_index = node_index(west);
                        let west_tile  = graph[west_index];
                        if west_tile.is_east_joint(&tile) {
                            graph.update_edge(west_index, index, true);
                        };
                    };
                    if let Some(east) = east {
                        let east_index = node_index(east);
                        let east_tile  = graph[east_index];
                        if tile.is_east_joint(&east_tile) {
                            graph.update_edge(index, east_index, true);
                        };
                    };
                });
            });
        Ok( Self{ rows, cols, graph, } )
    }
    fn parse_rows(s: &str) -> IResult<&str, usize> {
        many1_count(terminated(Self::parse_cols, line_ending))(s)
    }
    fn parse_cols(s: &str) -> IResult<&str, usize> {
        many1_count(one_of("|-LJ7F.S"))(s)
    }
    fn parse_list(s: &str) -> IResult<&str, Graph::<Tile, bool, Undirected>> {
        fold_many1(
            terminated(Tile::parse, opt(line_ending)),
            Graph::<Tile, bool, Undirected>::default,
            |mut graph, tile| {
                graph.add_node(tile);
                graph
            })(s)
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
enum Tile { NS, EW, NE, NW, SW, SE, #[default] None, Start, }
impl Tile {
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
