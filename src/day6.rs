use {
    aoc_runner_derive::{
        aoc,
        aoc_generator,
    },
    re_parse::{
        Regex,
        ReParse,
    },
    serde_derive::Deserialize,
    std::str::Split,
};

#[derive(Debug, Deserialize, ReParse)]
#[re_parse(regex = r#"(?P<x>\d{1,10}), (?P<y>\d{1,10})"#)]
struct Coordinate {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct CoordinateParser<'a> {
    input: Split<'a, char>,
}

impl<'a> CoordinateParser<'a> {
    pub fn new(input: Split<'a, char>) -> Self {
        Self {
            input,
        }
    }
}

impl Iterator for CoordinateParser<'_> {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.input.next()?.parse().unwrap())
    }
}

#[aoc_generator(day6)]
fn day6_generator<'a>(input: &'a str) -> CoordinateParser<'a> {
    CoordinateParser::new(input.trim().split('\n'))
}

#[aoc(day6, part1)]
pub fn day6_part1(input: CoordinateParser<'_>) -> u32 {
    println!("input: {:#?}", input);
    0
}
