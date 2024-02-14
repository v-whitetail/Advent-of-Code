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



pub fn part_two(input: Input) -> Result<i32> {
    let input = input.read();
    let (_, file) = parse_file(&input)
        .map_err(|err| anyhow!(err.to_owned()))?;
    let ans = file.iter()
        .map( |layer| r_extrapolate_layer(layer) )
        .sum::<i32>();
    Ok(ans)
}
#[test]
fn test_part_two() {
    let ans = part_two(Input::new(FILE).test()).unwrap();
    assert_eq!(2, ans);
}


pub fn part_one(input: Input) -> Result<i32> {
    let input = input.read();
    let (_, file) = parse_file(&input)
        .map_err(|err| anyhow!(err.to_owned()))?;
    let ans = file.iter()
        .map( |layer| extrapolate_layer(layer) )
        .sum::<i32>();
    Ok(ans)
}
#[test]
fn test_part_one() {
    let ans = part_one(Input::new(FILE).test()).unwrap();
    assert_eq!(114, ans);
}
#[test]
fn ans_part_one() {
    let ans = part_one(Input::new(FILE)).unwrap();
    assert_eq!(1789635132, ans);
}


fn r_extrapolate_layer(layer: &Vec<i32>) -> i32 {
    if is_layer_zero(layer) { return 0 };
    if let Some(next_layer) = next_layer(layer) {
        let seed = r_extrapolate_layer(&next_layer);
        return r_extrapolate_value(&layer, seed);
    };
    unreachable!()
}
fn r_extrapolate_value(layer: &Vec<i32>, seed: i32) -> i32 {
    layer.first().unwrap() - seed
}
fn extrapolate_layer(layer: &Vec<i32>) -> i32 {
    if is_layer_zero(layer) { return 0 };
    if let Some(next_layer) = next_layer(layer) {
        let seed = extrapolate_layer(&next_layer);
        return extrapolate_value(&layer, seed);
    };
    unreachable!()
}
fn extrapolate_value(layer: &Vec<i32>, seed: i32) -> i32 {
    seed + layer.last().unwrap()
}
fn next_layer(layer: &Vec<i32>) -> Option<Vec<i32>> {
    if layer.len() < 2 { return None };
    let next_layer = layer.windows(2)
        .map( |values| values[1] - values[0] )
        .collect();
    Some(next_layer)
}
fn is_layer_zero(layer: &Vec<i32>) -> bool {
    let zeros = layer.iter().filter( |&&value| value == 0).count();
    if zeros == layer.len() { true } else { false }
}
fn parse_file(s: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list1(line_ending, parse_line)(s)
}
fn parse_line(s: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(space1, i32)(s)
}
