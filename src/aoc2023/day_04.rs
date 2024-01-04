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





/// part one ///
pub fn part_one() -> Result<()> {

    let input = read_to_string("src/aoc2023/input/day_04.log")?;

    Ok(())

}





/// part two ///
pub fn part_two() -> Result<()> {

    let input = read_to_string("src/aoc2023/input/day_04.log")?;

    Ok(())

}





/// tests ///
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
