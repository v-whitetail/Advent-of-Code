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


pub const DAY_6: &str = file!();

pub fn part_one(input: Input) -> Result<i32> {

    let input = input.read();

    let (_, score_chart) = ScoreChart::parse(&input)
        .map_err( |err| anyhow!(err.to_owned()))?;

    let ans = score_chart.into_iter()
        .map( |(dur, dis)| optimize(&dur, &dis) )
        .map( |[min_hold, max_hold]| max_hold + 1 - min_hold )
        .product::<i32>();

    Ok(ans)

}
#[test]
fn test_part_one() {
    let ans = part_one(Input::new(DAY_6).test());
    assert_eq!(288, ans.unwrap());
}
pub fn part_two(input: Input) -> Result<i64> {

    let input = input.read();

    let (_, score_chart) = ScoreChart::parse(&input)
        .map_err( |err| anyhow!(err.to_owned()))?;

    let duration = score_chart.durations.concat()?;
    let distance = score_chart.distances.concat()?;
    let [min_hold, max_hold] = optimize2(&duration, &distance);

    let ans = max_hold + 1 - min_hold;

    Ok(ans)

}
#[test]
fn test_part_two() {
    let ans = part_two(Input::new(DAY_6).test());
    assert_eq!(71503, ans.unwrap());
}

fn optimize2(duration: &i64, distance: &i64) -> [i64; 2] {
    let f_time = *duration as f64;
    let f_dist = *distance as f64;
    let discriminant = ( f_time.powi(2) - 4.0*f_dist ).sqrt();
    let zeros = [ 0.5*(f_time - discriminant), 0.5*(f_time + discriminant) ];
    let holds = [ zeros[0].floor() as i64 + 1, zeros[1].ceil() as i64 - 1];
    return holds
}
fn optimize(duration: &i32, distance: &i32) -> [i32; 2] {
    // UNKNOWN: hold time
    // UNKNOWN: run time
    // UNKNOWN: speed
    // speed = hold time
    // run time = duration - hold time
    // distance = run time * speed
    // distance = ( duration - hold time ) * hold time
    // 0 = ( duration - hold time ) * hold time - distance
    // 0 = hold time ^2 - duration * hold time + distance
    // hold time = (duration +- sqrt(duration^2 - 4*1*distance)) / (2*1)
    let f_time = *duration as f64;
    let f_dist = *distance as f64;
    let discriminant = ( f_time.powi(2) - 4.0*f_dist ).sqrt();
    let zeros = [ 0.5*(f_time - discriminant), 0.5*(f_time + discriminant) ];
    let holds = [ zeros[0].floor() as i32 + 1, zeros[1].ceil() as i32 - 1];
    return holds
}


#[derive(Debug)]
struct ScoreChart {
    durations: Durations,
    distances: Distances,
}
impl ScoreChart {
    fn parse(s: &str) -> IResult<&str, Self> {
        map(
            separated_pair(Durations::parse, newline, Distances::parse),
            |(durations, distances)|
            Self{ durations, distances }
           )(s)
    }
}
impl IntoIterator for ScoreChart {
    type Item = (i32, i32);
    type IntoIter = Box<dyn Iterator<Item = Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        let durations = self.durations.values.into_iter();
        let distances = self.distances.values.into_iter();
        Box::new(durations.zip(distances))
    }
}


#[derive(Debug)]
struct Distances {
    values: Vec<i32>
}
impl Distances {
    fn concat(&self) -> Result<i64> {
        let concat = self.values
            .iter()
            .map( |value| value.to_string() )
            .collect::<String>()
            .parse::<i64>()?;
        Ok(concat)
    }
    fn parse(s: &str) -> IResult<&str, Self> {
        map(
            preceded(
                pair(tag("Distance:"), space1),
                separated_list1(space1, i32)
                ),
                |values|
                Self{ values }
           )(s)
    }
}

#[derive(Debug)]
struct Durations {
    values: Vec<i32>
}
impl Durations {
    fn concat(&self) -> Result<i64> {
        let concat = self.values
            .iter()
            .map( |value| value.to_string() )
            .collect::<String>()
            .parse::<i64>()?;
        Ok(concat)
    }
    fn parse(s: &str) -> IResult<&str, Self> {
        map(
            preceded(
                pair(tag("Time:"), space1),
                separated_list1(space1, i32)
                ),
                |values|
                Self{ values }
           )(s)
    }
}
