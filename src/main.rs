#![allow(unused, dead_code)]

use advent_of_code::aoc2023::*;
use anyhow::Result;

fn main() -> Result<()>{
    day_04::part_one()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
