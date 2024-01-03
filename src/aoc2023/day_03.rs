#![allow(unused, dead_code)]

use anyhow::{ Result, anyhow, };
use std::fs::*;
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

    let input = read_to_string("src/aoc2023/input/day_03.log")?;

    let mut queue = Note::parse(&input)?;

    Ok(())

}





pub fn part_two() -> Result<()> {

    Ok(())

}





#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Label {
    Numb(u32),
    Part(char),
}
impl Label {
    fn parse(s: &str) -> IResult<&str, (usize, Option<Self>)> {
        pair(
            many0_count(tag(".")),
            alt((
                    map( line_ending, |_| None ),
                    map( u32, |numb| Some(Self::Numb(numb)) ),
                    map( anychar, |part| Some(Self::Part(part)) ),
                ))
            )(s)
    }
    fn width(&self) -> usize {
        match self {
            Self::Numb(numb) => numb.to_string().len(),
            Self::Part(part) => 1,
        }
    }
    fn height(&self) -> usize {
        match self {
            Self::Numb(numb) => 1,
            Self::Part(part) => 1,
        }
    }
}





#[derive(Copy, Clone, Debug)]
struct Note {
    label: Label,
    origin: [usize; 2],
    bounds: [usize; 4],
}
impl std::cmp::Ord for Note {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).expect("impl ord for note")
    }
}
impl std::cmp::PartialOrd for Note {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering::{
            Greater as Gr,
            Equal as Eq,
            Less as Le,
        };
        use Label::{
            Part as Part,
            Numb as Numb,
        };
        match (self.label, other.label) {
            (Part(_), Part(_)) => Some(self.order_origin(other)),
            (Part(_), Numb(_)) => Some(self.order_bounds(other)),
            (Numb(_), Part(_)) => Some(self.order_bounds(other)),
            (Numb(_), Numb(_)) => Some(self.order_origin(other)),
        }
    }
}
impl std::cmp::Eq for Note {}
impl std::cmp::PartialEq for Note {
    fn eq(&self, other: &Self) -> bool {
        match (self.label, other.label) {
            (Label::Part(_), Label::Part(_)) => self.overlap_origin(other),
            (Label::Part(_), Label::Numb(_)) => self.overlap_bounds(other),
            (Label::Numb(_), Label::Part(_)) => self.overlap_bounds(other),
            (Label::Numb(_), Label::Numb(_)) => self.overlap_origin(other),
        }
    }
}
impl std::fmt::Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}\t{:?}\t\t{:?}", self.label, self.origin, self.bounds)
    }
}
impl Note {
    fn new(label: Label, origin: [usize; 2]) -> Self {
        let bounds = [
            origin[0], origin[0] + label.width(),
            origin[1], origin[1] + label.height(),
        ];
        Self{ label, origin, bounds }
    }
    fn order_origin(&self, other: &Self) -> std::cmp::Ordering {
        let lhs = ( self.origin[0] + self.label.width() ).pow(2)
            + self.origin[1].pow(2);
        let rhs = ( other.origin[0] + other.label.width() ).pow(2)
            + other.origin[1].pow(2);
        lhs.cmp(&rhs)
    }
    fn order_bounds(&self, other: &Self) -> std::cmp::Ordering {
        if self.overlap_bounds(other) { std::cmp::Ordering::Equal }
        else { self.order_origin(other) }
    }
    fn overlap_origin(&self, other: &Self) -> bool {
        let x_overlap = self.origin[0] == other.origin[0];
        let y_overlap = self.origin[1] == other.origin[1];
        x_overlap && y_overlap
    }
    fn overlap_bounds(&self, other: &Self) -> bool {
        let x_overlap =
            ( self.bounds[0] <= other.bounds[0]
              && other.bounds[0] <= self.bounds[1] )
            ||
            ( other.bounds[0] <= self.bounds[0]
              && self.bounds[0] <= other.bounds[1] );
        let y_overlap =
            ( self.bounds[2] <= other.bounds[2]
              && other.bounds[2] <= self.bounds[3] )
            ||
            ( other.bounds[2] <= self.bounds[2]
              && self.bounds[2] <= other.bounds[3] );
        x_overlap && y_overlap
    }
    fn parse(s: &str) -> Result<VecDeque<Self>> {
        let mut queue = VecDeque::new();
        fold_many0(
            Label::parse, move || (0, None),
            |(mut y, mut prev): (usize, Option<Self>), (mut x, label)| {
                if label.is_none() {
                    return (y + 1, None);
                };
                if let Some(note) = prev {
                    x += note.origin[0] + note.label.width();
                };
                let note = Self::new(label.unwrap(), [ x , y ]);
                match note.label {
                    Label::Numb(_) => queue.push_back(note),
                    Label::Part(_) => queue.push_front(note),
                };
                return (y, Some(note));
            })(s)
        .map_err( |err| anyhow!("impl Note::parse(s)\n{:#?}", err) )?;
        Ok(queue)
    }
}
