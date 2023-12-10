use crate::{solutions::day_5::parser::parse_input, Solver};
use color_eyre::eyre::{eyre, Result};

pub struct Day;

impl Solver for Day {
    fn part_1(&self, input: &str) -> Result<String> {
        let result = solve_part_1(input)?;
        Ok(result.to_string())
    }

    fn part_2(&self, input: &str) -> Result<String> {
        let result = solve_part_2(input)?;
        Ok(result.to_string())
    }
}

#[derive(Debug)]
struct Map<'a> {
    src: &'a str,
    dst: &'a str,
    ranges: Vec<Range>,
}

impl<'a> Map<'a> {
    fn transform_all(&self, input: usize) -> usize {
        let fold = self
            .ranges
            .iter()
            .filter(|r| r.in_range(input))
            .fold(input, |mut i, r| r.transform(i));
        fold
    }
}

#[derive(Debug)]
struct Range {
    dst: usize,
    src: usize,
    len: usize,
}

impl Range {
    fn in_range(&self, other: usize) -> bool {
        let Range { dst, src, len } = &self;
        other >= *src && other < *src + *len
    }

    fn transform(&self, input: usize) -> usize {
        let Range { dst, src, len } = &self;
        if self.in_range(input) {
            input - *src + *dst
        } else {
            input
        }
    }
}

fn solve_part_1(input: &str) -> Result<usize> {
    let (_, (seeds, maps)) =
        parse_input(input).map_err(|e| eyre!("Error parsing input: {:?}", e))?;

    let min = seeds
        .iter()
        .map(|&s| {
            let fold = maps.iter().fold(s, |s, m| m.transform_all(s));
            fold
        })
        .min()
        .ok_or(eyre!("no min element found"))?;

    Ok(min)
}

fn solve_part_2(input: &str) -> Result<usize> {
    let (_, (seeds, maps)) =
        parse_input(input).map_err(|e| eyre!("Error parsing input: {:?}", e))?;

    let seeds = seeds
        .iter()
        .step_by(2)
        .zip(seeds.iter().skip(1).step_by(2))
        .map(|(&s, &l)| s..s + l)
        .flatten();

    let min = seeds
        .map(|s| {
            let fold = maps.iter().fold(s, |s, m| m.transform_all(s));
            fold
        })
        .min()
        .ok_or(eyre!("no min element found"))?;

    Ok(min)
}

mod parser {
    use super::{Map, Range, Result};
    use std::collections::HashSet;

    use color_eyre::eyre::eyre;
    use nom::{
        bytes::complete::tag,
        character::complete::{alpha1, char, digit1, line_ending, space0, space1},
        combinator::{map_res, opt},
        multi::{many0, many1, separated_list1},
        sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
        IResult,
    };

    pub(super) fn parse_seeds(input: &str) -> IResult<&str, Vec<usize>> {
        let (input, _) = pair(tag("seeds:"), space1)(input)?;
        let (input, seeds): (_, Vec<usize>) =
            separated_list1(space1, map_res(digit1, str::parse))(input)?;
        let (input, _) = space0(input)?; // eat space
        let (input, _) = line_ending(input)?; // eat line_ending

        Ok((input, seeds))
    }

    pub(super) fn parse_range(input: &str) -> IResult<&str, Range> {
        let (input, (dst, src, len)) = tuple((
            terminated(map_res(digit1, str::parse), space0),
            terminated(map_res(digit1, str::parse), space0),
            terminated(map_res(digit1, str::parse), space0),
        ))(input)?;

        Ok((input, Range { src, dst, len }))
    }

    pub(super) fn parse_map(input: &str) -> IResult<&str, Map> {
        let (input, (src, dst)) = terminated(
            separated_pair(alpha1, tag("-to-"), alpha1),
            tuple((space0, tag("map:"), space0)),
        )(input)?;
        let (input, _) = space0(input)?; // eat space
        let (input, _) = line_ending(input)?; // eat line_ending
        let (input, ranges) = separated_list1(line_ending, parse_range)(input)?;
        let (input, _) = opt(line_ending)(input)?; // eat line_ending

        Ok((input, Map { src, dst, ranges }))
    }

    pub(super) fn parse_input(input: &str) -> IResult<&str, (Vec<usize>, Vec<Map>)> {
        let (input, seeds) = terminated(parse_seeds, many1(line_ending))(input)?;
        let (input, maps) = separated_list1(line_ending, parse_map)(input)?;

        Ok((input, (seeds, maps)))
    }
}

#[cfg(test)]
mod tests {
    use assert_ok::assert_ok;

    use super::{
        parser::{parse_input, parse_map, parse_seeds},
        *,
    };
    const INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    #[test]
    fn part_1() {
        let x = assert_ok!(solve_part_1(INPUT));
        assert_eq!(x, 35)
    }
    #[test]
    fn part_2() {
        let x = assert_ok!(solve_part_2(INPUT));
        assert_eq!(x, 46)
    }

    #[test]
    fn parser() {
        let (input, _maps) = assert_ok!(parse_input(INPUT));
    }

    #[test]
    fn map() {
        let input = "soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15";
        let (input, map) = assert_ok!(parse_map(input));
    }
}
