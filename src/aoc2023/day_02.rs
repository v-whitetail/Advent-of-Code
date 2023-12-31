use anyhow::Result;
use std::fs::read_to_string;
use nom::{
    IResult, Parser,
    branch::alt,
    combinator::map,
    bytes::complete::tag,
    multi::separated_list1,
    character::complete::{ u8, space1 },
    sequence::{ delimited, pair, preceded, },
};





pub fn part_one() -> Result<()> {

    let input = read_to_string("src/aoc2023/input/day_02.log")?;

    let rules = Rules::new(12, 13, 14);

    let ans = input.lines()
        .filter_map( |line| Game::parse(line).ok() )
        .filter_map( |(_, game)| game.judge_1(&rules) )
        .map( |int| int as u64 )
        .sum::<u64>();

    println!("{ans:#?}");

    Ok(())
}





pub fn part_two() -> Result<()> {

    let input = read_to_string("src/aoc2023/input/day_02.log")?;

    let ans = input.lines()
        .filter_map( |line| Game::parse(line).ok() )
        .map( |(_, game)| game.judge_2() )
        .map( |min_rule| min_rule.power()  )
        .sum::<u64>();

    println!("{ans:#?}");

    Ok(())
}




#[derive(Default, Debug, Clone, Copy)]
struct Rules{
    red: u8,
    green: u8,
    blue: u8,
}
impl Rules{
    fn new(red: u8, green: u8, blue: u8) -> Self {
        Self{red, green, blue}
    }
    fn power(&self) -> u64 {
        self.red as u64
            * self.green as u64
            * self.blue as u64
    }
}





#[derive(Default, Debug, Clone)]
struct Game{
    id: u8,
    hands: Vec<Hand>,
}
impl Game{
    fn parse(s: &str) -> IResult<&str, Self> {
        map(
            pair(
                delimited(tag("Game "), u8, tag(":")),
                separated_list1(tag(";"), Hand::parse)
                ),
                | (id, hands) |
                Self{id, hands}
           )(s)
    }
    fn judge_1(&self, rules: &Rules) -> Option<u8> {
        for hand in &self.hands {
            if rules.red < hand.red
                || rules.green < hand.green
                    || rules.blue < hand.blue
                    { return None }
        };
        Some(self.id)
    }
    fn judge_2(&self) -> Rules {
        let mut rules = Rules::default();
        for hand in &self.hands {
            rules.red = rules.red.max(hand.red);
            rules.green = rules.green.max(hand.green);
            rules.blue = rules.blue.max(hand.blue);
        };
        rules
    }
}





#[derive(Default, Debug, Clone, Copy)]
struct Hand{
    red: u8,
    green: u8,
    blue: u8,
}
impl Hand{
    fn parse(s: &str) -> IResult<&str, Self> {
        map(
            separated_list1(tag(","), Color::parse),
            | colors | {
                let mut hand = Hand::default();
                for color in colors {
                    match color {
                        Color::Red(value) => hand.red = value,
                        Color::Green(value) => hand.green = value,
                        Color::Blue(value) => hand.blue = value,
                    }
                }
                hand
            })(s)
    }
}
impl std::ops::Add for Hand {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self{
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}




#[derive(Debug, Clone, Copy)]
enum Color{
    Red(u8),
    Green(u8),
    Blue(u8),
}
impl Color {
    fn parse(s: &str) -> IResult<&str, Self> {
        map(
            pair(
                delimited(space1, u8, space1),
                alt((tag("red"),tag("green"),tag("blue"))),
                ),
                | (value, tag) |
                match tag {
                    "red" => Self::Red(value),
                    "green" => Self::Green(value),
                    "blue" => Self::Blue(value),
                    _ => panic!("how can you not count to three, ya dingus"),
                }
           )(s)
    }
}
