use anyhow::{ Result, anyhow, };
use nom::bytes::streaming::take;
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


pub const DAY_7: &str = file!();


pub fn part_one(input: Input) -> Result<u64> {

    let input = input.read();

    let mut game: Vec<_> = input.lines()
        .map( |line| Bet::parse(line).map_err( |err| anyhow!(err.to_owned()) ))
        .try_collect()?;

    game.sort_unstable_by_key( |&(_, bet)| bet.hand );

    let ans = game.iter()
        .enumerate()
        .map( |(i, (_, bet))| (i as u64 + 1) * bet.value)
        .sum::<u64>();

    Ok(ans)

}
#[test]
fn test_part_one() {
    let ans = part_one(Input::new(DAY_7).test());
    assert_eq!(6440, ans.unwrap());
}
#[test]
fn ans_part_one() {
    let ans = part_one(Input::new(DAY_7));
    assert_eq!(250898830, ans.unwrap());
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Bet {
    hand: Hand,
    value: u64,
}
impl Bet {
    fn parse(s: &str) -> IResult<&str, Self> {
        map(
            separated_pair(
                Self::parse_hand,
                space1,
                u64,
                ), |(hand, value)|
            Self {hand, value}
           )(s)
    }
    fn parse_hand(s: &str) -> IResult<&str, Hand> {
        map(
            Self::parse_cards,
            |(cards, _)|
            Hand::new(&cards)
           )(s)
    }
    fn parse_cards(s: &str) -> IResult<&str, ([Card; 5], usize)> {
        type Acc = ([Card; 5], usize);
        fold_many1(
            Card::parse,
            Acc::default,
            |mut acc, card| {
                acc.0[acc.1] = card;
                acc.1 += 1;
                acc
            })(s)
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Hand {
    HighCard([Card; 5]),
    Pair([Card; 5]),
    TwoPair([Card; 5]),
    ThreeOAK([Card; 5]),
    FullHouse([Card; 5]),
    FourOAK([Card; 5]),
    FiveOAK([Card; 5]),
}
impl Hand {
    fn new(cards: &[Card; 5]) -> Self {
        let mut s_cards = cards.clone();
        s_cards.sort_unstable();
        let (sorted, dupes) = s_cards.partition_dedup();
        match dupes.len() {
            0 => { return Self::HighCard(*cards); },
            1 => { return Self::Pair(*cards); },
            2 => { 
                if dupes[0] != dupes[1] { return Self::TwoPair(*cards); };
                if dupes[0] == dupes[1] { return Self::ThreeOAK(*cards); };
            },
            3 => {
                if dupes[0] == dupes[1] && dupes[1] == dupes[2] {
                    return Self::FourOAK(*cards);
                } else {
                    return Self::FullHouse(*cards);
                };
            },
            4 => { return Self::FiveOAK(*cards); },
            _ => unreachable!()
        };
        unreachable!()
    }
}


#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    #[default]
    None,
    N(u8),
    T,
    J,
    Q,
    K,
    A,
}
impl Card {
    fn parse(s: &str) -> IResult<&str, Self> {
        alt((Self::parse_face_card, Self::parse_numeric))(s)
    }
    fn parse_face_card(s: &str) -> IResult<&str, Self> {
        map(alt((tag("A"), tag("K"), tag("Q"), tag("J"), tag("T"))),
        |tag| match tag {
            "A" => Self::A, "K" => Self::K, "Q" => Self::Q,
            "J" => Self::J, "T" => Self::T,  _  => panic!("invalid tag")
        })(s)
    }
    fn parse_numeric(s: &str) -> IResult<&str, Self> {
        map(map_parser(take(1usize), u8), |n| Self::N(n) )(s)
    }
}
