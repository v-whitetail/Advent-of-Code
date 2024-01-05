#![feature(extend_one)]

use std::borrow::*;
use std::collections::*;
use std::fs::read_to_string;
use itertools::Itertools;
use anyhow::{ Result, anyhow, };




/// part one ///
pub fn part_one() -> Result<()> {

    let input = read_to_string("src/aoc2023/input/day_04.nu")?;

    let ans = Buffer::parse_one(&input);

    println!("{ans:#?}");

    Ok(())

}
#[test]
fn test_part_one() {
    assert_eq!(13, Buffer::parse_one(TEST_INPUT));
}





/// part two ///
pub fn part_two() -> Result<()> {

    let input = read_to_string("src/aoc2023/input/day_04.nu")?;

    let ans = Buffer::parse_two(&input);

    println!("{ans:#?}");

    Ok(())

}
#[test]
fn test_part_two() {
    assert_eq!(30, Buffer::parse_two(TEST_INPUT));
}





#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Buffer{
    idx: usize,
    lhs: [u8; 10],
    rhs: [u8; 26],
}
impl Buffer{
    fn parse_one(s: &str) -> u128 {
        let mut score = 0;
        let mut buffer = Self::default();
        for line in s.lines() {
            buffer.parse_line(line);
            score += buffer.count_hits_one();
        }
        score
    }
    fn parse_two(s: &str) -> u128 {
        let mut buffer = Self::default();
        let mut record = [1; 256];
        let mut sum = 0;
        for line in s.lines() {
            buffer.parse_line(line);
            sum  += record[buffer.idx];
            let c = record[buffer.idx];
            record[ buffer.idx + 1 ..= buffer.idx + buffer.count_hits_two() ]
                .iter_mut()
                .for_each(|entry| {*entry += c;} );
        }
        sum
    }
    fn parse_line(&mut self, line: &str) {
        let mut split_line = line.split(&[':', '|']);
        self.idx = split_line
            .next()
            .unwrap()
            .trim_start_matches("Card")
            .trim()
            .parse::<usize>()
            .unwrap();
        split_line
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .filter( |&entry| !entry.is_empty() )
            .fold( 0, |i: usize, entry| {
                self.lhs[i] = entry.parse::<u8>().unwrap();
                i + 1
            });
        split_line
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .filter( |&entry| entry!="" )
            .fold( 0, |i: usize, entry| {
                self.rhs[i] = entry.parse::<u8>().unwrap();
                i + 1
            });
    }
    fn count_hits_one(&self) -> u128 {
        let mut hits = 0;
        for lhs in self.lhs.iter().filter(|&&v| 0 < v ) {
            for rhs in self.rhs.iter().filter(|&&v| 0 < v ) {
                if lhs == rhs {
                    if hits == 0 {
                        hits += 1;
                    } else {
                        hits <<= 1;
                    }
                }
            }
        }
        hits
    }
    fn count_hits_two(&self) -> usize {
        let mut hits = 0;
        for lhs in self.lhs.iter().filter(|&&v| 0 < v ) {
            for rhs in self.rhs.iter().filter(|&&v| 0 < v ) {
                if lhs == rhs {
                    hits += 1;
                }
            }
        }
        hits as usize
    }
}





const TEST_INPUT: &str = 
r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
