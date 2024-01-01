use anyhow::{ Result, anyhow, };
use itertools::{
    Itertools,
    Either::Left as Left,
    Either::Right as Right,
};
use std::{
    fs::*,
    ops::*,
    collections::*,
    borrow::BorrowMut,
    cmp::Ordering::Less as Less,
    cmp::Ordering::Equal as Equal,
    cmp::Ordering::Greater as Greater,
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

    let items = input
        .lines()
        .enumerate()
        .filter_map( |(y, line)| take_line(line, 0, y).ok())
        .collect::<Vec<_>>();

    println!("{items:#?}");

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
    let (_, mut line) = many1(item)(s).map_err(|_| anyhow!("depth limit"))?;
    let mut mut_windows = line.iter_mut().peekable();
    while let Some(item) = mut_windows.next() {
        if let Some(next_item) = mut_windows.peek_mut() {
            **next_item += item.to_owned();
        };
    };
    Ok(line)
}




#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
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
    fn as_mut_label(&mut self) -> Option<&mut Label> {
        match self {
            Self::Part(_) => None,
            Self::Label(label) => Some(label),
        }
    }
    fn as_mut_part(&mut self) -> Option<&mut Part> {
        match self {
            Self::Part(part) => Some(part),
            Self::Label(_) => None,
        }
    }
    fn is_part(self) -> bool {
        match self {
            Self::Part(_) => true,
            Self::Label(_) => false,
        }
    }
    fn is_label(self) -> bool {
        match self {
            Self::Part(_) => false,
            Self::Label(_) => true,
        }
    }
}
impl Add for Item {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        use Item::Part as Part;
        use Item::Label as Label;
        match (self, rhs) {
            (  Part(lhs),  Part(rhs) ) =>  Part( lhs + rhs ),
            (  Part(lhs), Label(rhs) ) =>  Part( lhs + rhs ),
            ( Label(lhs), Label(rhs) ) => Label( lhs + rhs ),
            ( Label(lhs),  Part(rhs) ) => Label( lhs + rhs ),
        }
    }
}
impl AddAssign for Item {
    fn add_assign(&mut self, rhs: Self) {
        let new = self.clone() + rhs;
        self.clone_from(&new);
    }
}





#[derive(Debug, Clone, PartialEq, Eq, Ord, Hash)]
struct Part {
    item: char,
    size: Range2D,
}
impl Part {
    fn parse(s: &str, x: usize, y: usize) -> IResult<&str, Self> {
        map(
            pair(skip_empty, anychar),
            |(head_space, item)| {
                let size = Range2D::new(
                    [x + head_space, x + head_space ],
                    [y, y]
                    );
                Self{size, item}
            }
           )(s)
    }
}
impl PartialOrd for Part {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.size.partial_cmp(&other.size)
    }
}
impl Add for Part {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let size = self.size + rhs.size;
        let item = self.item;
        Self{item, size}
    }
}
impl AddAssign for Part {
    fn add_assign(&mut self, rhs: Self) {
        self.clone().add(rhs);
    }
}
impl Add<Label> for Part {
    type Output = Self;
    fn add(self, rhs: Label) -> Self::Output {
        let size = self.size + rhs.size;
        let item = self.item;
        Self{item, size}
    }
}
impl AddAssign<Label> for Part {
    fn add_assign(&mut self, rhs: Label) {
        let new = self.clone() + rhs;
        self.clone_from(&new);
    }
}




#[derive(Debug, Clone, PartialEq, Eq, Ord, Hash)]
struct Label {
    item: u32,
    size: Range2D,
}
impl Label {
    fn parse(s: &str, x: usize, y: usize) -> IResult<&str, Self> {
        map(
            pair(skip_empty, u32),
            |(head_space, item)| {
                let size = Range2D::new(
                    [ x + head_space
                    , x + head_space + item.to_string().len() - 1 ],
                    [y, y]
                    );
                Self{size, item}
            }
           )(s)
    }
}
impl PartialOrd for Label {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.size.partial_cmp(&other.size)
    }
}
impl Add for Label {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let size = self.size + rhs.size;
        let item = self.item;
        Self{item, size}
    }
}
impl AddAssign for Label {
    fn add_assign(&mut self, rhs: Self) {
        self.clone().add(rhs);
    }
}
impl Add<Part> for Label {
    type Output = Self;
    fn add(self, rhs: Part) -> Self::Output {
        let size = self.size + rhs.size;
        let item = self.item;
        Self{item, size}
    }
}
impl AddAssign<Part> for Label {
    fn add_assign(&mut self, rhs: Part) {
        let new = self.clone() + rhs;
        self.clone_from(&new);
    }
}




#[derive(Debug, Clone, Eq, Hash)]
struct Range2D {
    x: RangeInclusive<usize>,
    y: RangeInclusive<usize>,
}
impl Range2D {
    fn new(x: [usize;2], y: [usize;2]) -> Self{
        let x = RangeInclusive::new(x[0], x[1]);
        let y = RangeInclusive::new(y[0], y[1]);
        Self{x, y}
    }
    fn partial_include(&self, other: &Self) -> bool {
        let x_contains =
            self.x.contains(other.x.start())
            || self.x.contains(other.x.start());
        let y_contains =
            self.y.contains(other.y.start())
            || self.y.contains(other.y.end());
        let is_x_contained =
            other.x.contains(self.x.start())
            || other.x.contains(self.x.start());
        let is_y_contained =
            other.y.contains(self.y.start())
            || other.y.contains(self.y.end());
        let x_overlap = x_contains || is_x_contained;
        let y_overlap = y_contains || is_y_contained;
        x_overlap && y_overlap
    }
}
impl Add for Range2D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let x = RangeInclusive::new(
            self.x.start() + rhs.x.end() + 1,
            self.x.end()   + rhs.x.end() + 1 );
        let y = self.y;
        Self{x, y}
    }
}
impl AddAssign for Range2D {
    fn add_assign(&mut self, rhs: Self) {
        let new = self.clone() + rhs;
        self.clone_from(&new);
    }
}
impl PartialEq for Range2D {
    fn eq(&self, other: &Self) -> bool {
        self.partial_include(other)
    }
}
impl PartialOrd for Range2D {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (
            self.x.start().partial_cmp(other.x.start()),
            self.y.start().partial_cmp(other.y.start()),
            ) {
            ( _, Some(Greater)) => Some(Greater),
            ( _,    Some(Less)) => Some(Less),
            (Some(Greater) ,_ ) => Some(Greater),
            (Some(Less)    ,_ ) => Some(Less),
            _ => None
        }
    }
}
impl Ord for Range2D {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(Less)
    }
}
