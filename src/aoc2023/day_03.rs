use anyhow::{ Result, anyhow, };
use std::ops::Range;
use itertools::{
    Itertools,
    Either::Left as Left,
    Either::Right as Right, iproduct,
};
use std::{
    fs::read_to_string,
    collections::HashSet,
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

    input.lines().for_each( |line| {
        println!("{line:#?}");
        });

    let parts = input
        .lines()
        .enumerate()
        .filter_map( |(y, line)| take_line(line, 0, y).ok() )
        .flatten()
        .filter_map( |item| item.as_part() );
    let labels = input
        .lines()
        .enumerate()
        .filter_map( |(y, line)| take_line(line, 0, y).ok() )
        .flatten()
        .filter_map( |item| item.as_label() );
    let ans: u32 = labels
        .filter( |label|
                 parts
                 .clone()
                 .filter( |part| judge_2d(part, label) )
                 .next()
                 .is_some()
               )
        .map( |label| label.item )
        .sum();

    println!("{ans:#?}");

    Ok(())
}





pub fn part_two() -> Result<()> {

    Ok(())
}





fn skip_empty(s: &str) -> IResult<&str, usize> {
    many0_count(tag("."))(s)
}
fn take_line(s: &str, x: usize, y: usize) -> Result<Vec<Item>> {
    let item = |s| Item::parse(s, x, y);
    let (_, mut line) = many1(item)(s)
        .map_err(|err| anyhow!("recursion error:\n{err:#?}"))?;
    let mut iter = line.iter_mut().peekable();
    while let Some(item) = iter.next() {
        if let Some(next) = iter.peek_mut() {
            next.offset(item);
        };
    };
    Ok(line)
}
fn judge_x(part: &Part, label: &Label) -> bool{
    let mut x_bound = label.x.clone();
    x_bound.start = x_bound.start.saturating_sub(1);
    x_bound.end += 2;
    x_bound.contains(&part.x)
}
fn judge_y(part: &Part, label: &Label) -> bool{
    let y_bound = Range{start: label.y.saturating_sub(1), end: label.y+2};
    y_bound.contains(&part.y)
}
fn judge_2d(part: &Part, label: &Label) -> bool{
    judge_x(part, label) && judge_y(part, label)
}




#[derive(Debug, Clone)]
enum Item {
    Label(Label),
    Part(Part),
}
impl Item {
    fn parse(s: &str, x: usize, y: usize) -> IResult<&str, Self> {
        let label = |s| Self::from_label(s, x, y);
        let part = |s| Self::from_part(s, x, y);
        alt((label, part))(s)
    }
    fn from_label(s: &str, x: usize, y: usize) -> IResult<&str, Self> {
        let label = |s| Label::parse(s, x, y);
        map(label, |label| Self::Label(label))(s)
    }
    fn from_part(s: &str, x: usize, y: usize) -> IResult<&str, Self> {
        let part = |s| Part::parse(s, x, y);
        map(part, |part| Self::Part(part))(s)
    }
    fn as_label(self) -> Option<Label> {
        match self {
            Self::Part(_) => None,
            Self::Label(label) => Some(label),
        }
    }
    fn as_part(self) -> Option<Part> {
        match self {
            Self::Part(part) => Some(part),
            Self::Label(_) => None,
        }
    }
    fn offset(&mut self, prev: &mut Self) {
        use Item::Label as L;
        use Item::Part as P;
        match (prev, self) {
            (P(prev), P(next)) => {
                next.x += prev.x + 1;
            },
            (P(prev), L(next)) => {
                next.x.start += prev.x + 1;
                next.x.end += prev.x + 1;
            },
            (L(prev), P(next)) => {
                next.x += prev.x.end;
            },
            (L(prev), L(next)) => {
                next.x.start += prev.x.end + 1;
                next.x.end += prev.x.end + 1;
            },
        };
    }
}





#[derive(Debug, Clone, Copy)]
struct Part {
    x: usize,
    y: usize,
    item: char
}
impl Part {
    fn parse(s: &str, x: usize, y: usize) -> IResult<&str, Self> {
        map(
            pair(skip_empty, anychar),
            |(head_space, item)|
            {
                let x = x + head_space;
                let y = y.clone();
                Self{x, y, item}
            }
           )(s)
    }
}




#[derive(Debug, Clone)]
struct Label {
    x: Range<usize>,
    y: usize,
    item: u32,
}
impl Label {
    fn parse(s: &str, x: usize, y: usize) -> IResult<&str, Self> {
        map(
            pair(skip_empty, u32),
            |(head_space, item)|
            {
                let x = Range{
                    start: x + head_space,
                    end: x + head_space + item.to_string().len() - 1,
                };
                let y = y.clone();
                Self{x, y, item}
            }
           )(s)
    }
}

