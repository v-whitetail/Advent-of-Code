
use std::fs::read_to_string;
use anyhow::Result;

pub fn part_one() -> Result<()> {

    let input = read_to_string("src/aoc2023/input/day_01.log")?;

    let ans: u32 = input
        .lines()
        .map( |line|
              line
              .chars()
              .filter_map( |c| c.to_digit(10) )
              .collect::<Vec<_>>()
              )
        .map( |vect|
              match (vect.first(), vect.last()) {
                  (Some(&f), Some(&l)) => 10*f + l,
                  (Some(&f), None) => 10*f + f,
                  _ => 0,
              }
            )
        .sum();

    println!("{ans:#?}");

    Ok(())

}

pub fn part_two() -> Result<()> {

    let input = read_to_string("src/aoc2023/input/day_01.log")?;

    let mut tags = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9",
        "one", "two", "three",
        "four", "five", "six",
        "seven", "eight", "nine",
    ];
    tags.sort_by_key(|tag| tag.len());

    let window_size = tags.last().unwrap().len();

    let ans = input.lines()
        .map( |line| 
              to_buffer(line)
              .windows(window_size)
              .filter_map( |window| tags
                           .into_iter()
                           .filter_map( |tag|
                                        if is_match(window, tag) { Some(tag) }
                                        else { None }
                                      )
                           .fuse()
                           .next()
                         )
              .map( |tag| 
                    match tag {
                        "1" => 1, "2" => 2, "3" => 3,
                        "4" => 4, "5" => 5, "6" => 6,
                        "7" => 7, "8" => 8, "9" => 9,
                        "one" => 1, "two" => 2, "three" => 3,
                        "four" => 4, "five" => 5, "six" => 6,
                        "seven" => 7, "eight" => 8, "nine" => 9,
                        _ => u32::MAX
                    }
                  )
              .collect::<Vec<_>>()
              )
              .map( |line| line_score(line) )
              .sum::<u32>();

    println!("{ans:#?}");

    Ok(())

}

fn is_match(window: &[u8], tag: &str) -> bool {
    window.get(0..tag.len()) == Some(tag.as_bytes())
}
fn to_buffer(window: &str) -> Vec<u8> {
    let mut buffer = window.as_bytes().to_owned();
    buffer.extend_from_slice(&[0;5]);
    buffer
}
fn line_score(line: Vec<u32>) -> u32{
    match (line.first(), line.last()) {
        (Some(f), Some(l)) => 10*f + l,
        (Some(f), None) => 10*f + f,
        _ => panic!("rip lmao")
    }
}
