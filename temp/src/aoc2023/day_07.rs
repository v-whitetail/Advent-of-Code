use anyhow::{ Result, anyhow, };
use nom::bytes::streaming::take;
use std::fs::*;
use std::cmp::*;
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


pub fn part_two(input: Input) -> Result<u64> {

    let input = input.read();

    let mut game: Vec<_> = input.lines()
        .map( |line| WildBet::parse(line).map_err( |err| anyhow!(err.to_owned()) ))
        .try_collect()?;

    game.sort_by_key( |&(_, bet)| bet.hand );

    let ans = game.iter()
        .enumerate()
        .map( |(i, (_, bet))| (i as u64 + 1) * bet.value)
        .sum::<u64>();

    Ok(ans)

}
#[test]
fn test_part_two() {
    let ans = part_two(Input::new(DAY_7).test()).unwrap();
    assert_eq!(5905, ans);
}
#[test]
fn ans_part_two() {
    let ans = part_two(Input::new(DAY_7)).unwrap();
    assert_ne!(252334781, ans);
    assert_ne!(252340560, ans);
    assert_ne!(255406852, ans);
    
}




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
struct WildBet {
    hand: WildHand,
    value: u64,
}
impl WildBet {
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
    fn parse_hand(s: &str) -> IResult<&str, WildHand> {
        map(
            Self::parse_cards,
            |(cards, _)|
            WildHand::new(&cards)
           )(s)
    }
    fn parse_cards(s: &str) -> IResult<&str, ([WildCard; 5], usize)> {
        type Acc = ([WildCard; 5], usize);
        fold_many1(
            Card::parse,
            Acc::default,
            |mut acc, card| {
                acc.0[acc.1] = card.into();
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
        let r_cards = s_cards.clone();
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


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum WildHand {
    HighCard([WildCard; 5]),
    Pair([WildCard; 5]),
    TwoPair([WildCard; 5]),
    ThreeOAK([WildCard; 5]),
    FullHouse([WildCard; 5]),
    FourOAK([WildCard; 5]),
    FiveOAK([WildCard; 5]),
}
impl WildHand {
    fn new(cards: &[WildCard; 5]) -> Self {
        let mut s_cards = cards.clone();
        s_cards.sort();
        let (sorted, dupes) = s_cards.partition_dedup();
        match dupes.len() {
            4 => { return Self::FiveOAK(*cards); },
            3 => {
                if dupes[0] == dupes[1] && dupes[1] == dupes[2] {
                    return Self::FourOAK(*cards);
                } else {
                    return Self::FullHouse(*cards);
                };
            },
            2 => { 
                if dupes[0] != dupes[1] { return Self::TwoPair(*cards); };
                if dupes[0] == dupes[1] { return Self::ThreeOAK(*cards); };
            },
            1 => { return Self::Pair(*cards); },
            0 => { return Self::HighCard(*cards); },
            _ => unreachable!()
        };
        unreachable!()
    }
}


#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Default, Debug, Copy, Clone)]
struct WildCard { card: Card }
impl From<Card> for WildCard {
    fn from(card: Card) -> Self {
        Self{card}
    }
}
impl PartialEq for WildCard {
    fn eq(&self, other: &Self) -> bool {
        if Card::J == self.card || Card::J == other.card {
            return true;
        };
        self.card.eq(&other.card)
    }
}
impl Eq for WildCard { }
impl PartialOrd for WildCard {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if Card::J == self.card {
            return Some(Ordering::Less);
        };
        if Card::J == other.card {
            return Some(Ordering::Greater);
        };
        self.card.partial_cmp(&other.card)
    }
}
impl Ord for WildCard {
    fn cmp(&self, other: &Self) -> Ordering {
        if Card::J == self.card {
            return Ordering::Less;
        };
        if Card::J == other.card  {
            return Ordering::Greater;
        };
        self.card.cmp(&other.card)
    }
}
