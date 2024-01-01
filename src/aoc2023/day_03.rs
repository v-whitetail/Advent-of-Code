use anyhow::Result;
use anyhow::anyhow;
use std::ops::Range;
use std::fs::read_to_string;
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

    let ligma = input.lines().next().unwrap();

    println!("{ligma:#?}");

    let sugma = take_line(ligma, 0, 0);

    println!("{sugma:#?}");

    Ok(())
}





pub fn part_two() -> Result<()> {

    Ok(())
}





fn skip_empty(s: &str) -> IResult<&str, usize> {
    many0_count(tag("."))(s)
}
fn take_line(s: &str, x: usize, y: usize) -> IResult<&str, Vec<Item>> {
    let item = |s| Item::parse(s, x, y);
    let wrapped_item = |s| Item::wrapped_parse(s, x, y);
    alt((
            map(
                pair(item, rest), |(item, rest)| {
                    let (_, mut tail) = take_line(rest, item.offset(x), y)
                        .unwrap_or_default();
                    tail.push(item);
                    tail
                }),
               wrapped_item
        ))(s)
}




#[derive(Debug, Clone)]
enum Item {
    Part(Part),
    Label(Label),
}
impl Item {
    fn parse(s: &str, x: usize, y: usize) -> IResult<&str, Self> {
        let label = |s| Self::from_label(s, x, y);
        let part = |s| Self::from_part(s, x, y);
        alt((label, part))(s)
    }
    fn wrapped_parse(s: &str, x: usize, y: usize) -> IResult<&str, Vec<Self>> {
        let label = |s| Self::from_label(s, x, y);
        let part = |s| Self::from_part(s, x, y);
        map(alt((label, part)), |item| vec![item])(s)
    }
    fn from_label(s: &str, x: usize, y: usize) -> IResult<&str, Self> {
        let label = |s| Label::parse(s, x, y);
        map(label, |label| Self::Label(label))(s)
    }
    fn from_part(s: &str, x: usize, y: usize) -> IResult<&str, Self> {
        let part = |s| Part::parse(s, x, y);
        map(part, |part| Self::Part(part))(s)
    }
    fn offset(&self, x: usize) -> usize {
        match self {
            Self::Label(label) => x + label.x.1,
            Self::Part(part) => x + part.x,
        }
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
                let x = x + head_space + 1;
                let y = y.clone();
                Self{x, y, item}
            }
           )(s)
    }
}




#[derive(Debug, Clone)]
struct Label {
    x: (usize, usize),
    y: usize,
    item: u8,
}
impl Label {
    fn parse(s: &str, x: usize, y: usize) -> IResult<&str, Self> {
        map(
            pair(skip_empty, u8),
            |(head_space, item)|
            {
                let x = ( x + head_space + 1,
                          x + head_space + item.to_string().len() );
                let y = y.clone();
                Self{x, y, item}
            }
           )(s)
    }
}

