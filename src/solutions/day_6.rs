use crate::Solver;
use color_eyre::eyre::{eyre, Result};

use self::parser::{parse_part_1, parse_part_2};

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

mod parser {
    use color_eyre::eyre::eyre;
    use nom::{
        bytes::complete::tag,
        character::complete::{alpha1, digit1, line_ending, space1},
        combinator::{map_res, opt},
        multi::{many1, separated_list1},
        sequence::{preceded, tuple},
        IResult,
    };

    fn parse_part1_line(input: &str) -> IResult<&str, Vec<usize>> {
        preceded(
            tuple((alpha1, tag(":"), space1)),
            separated_list1(space1, map_res(digit1, str::parse)),
        )(input)
    }

    fn parse_both_lines_part1(input: &str) -> IResult<&str, (Vec<usize>, Vec<usize>)> {
        let (input, time) = parse_part1_line(input)?;
        let (input, _) = line_ending(input)?;
        let (input, distance) = parse_part1_line(input)?;
        Ok((input, (time, distance)))
    }

    pub(super) fn parse_part_1(input: &str) -> super::Result<Vec<(usize, usize)>> {
        let (_, (t, d)) =
            parse_both_lines_part1(input).map_err(|e| eyre!("Error parsing input: {:?}", e))?;
        Ok(t.into_iter().zip(d.into_iter()).collect())
    }

    pub(super) fn parse_part_2(input: &str) -> super::Result<(usize, usize)> {
        let (_, (t, d)) =
            parse_part2_line(input).map_err(|e| eyre!("Error parsing input: {:?}", e))?;
        Ok((t, d))
    }

    fn parse_line_part2(input: &str) -> IResult<&str, usize> {
        let (input, numbers) = preceded(
            tuple((alpha1, tag(":"), space1)),
            separated_list1(space1, digit1),
        )(input)?;
        let collect: String = numbers.into_iter().flat_map(|(num)| num.chars()).collect();
        let collect = str::parse(&collect).unwrap(); // numbers contains only digits
        Ok((input, collect))
    }

    fn parse_part2_line(input: &str) -> IResult<&str, (usize, usize)> {
        let (input, t) = parse_line_part2(input)?;
        let (input, _) = opt(line_ending)(input)?;
        let (input, d) = parse_line_part2(input)?;
        Ok((input, (t, d)))
    }
}

fn solve_part_1(input: &str) -> Result<usize> {
    let races = parse_part_1(input)?;
    Ok(races
        .iter()
        .map(|(t, d)| solve_quadratic(*t, *d))
        .fold(1, |mut acc, r| acc * r))
}

fn solve_part_2(input: &str) -> Result<usize> {
    let (t, d) = parse_part_2(input)?;
    Ok(solve_quadratic(t, d))
}

fn solve_quadratic(t: usize, d: usize) -> usize {
    let (t, d) = (t as f64, d as f64);
    let b = t / 2.0;
    let sqrt = b.mul_add(b, -d).sqrt();
    let min = (b - sqrt).floor() as usize + 1;
    let max = (b + sqrt).ceil() as usize - 1;
    max - min + 1
}

#[cfg(test)]
mod tests {
    use assert_ok::assert_ok;

    use super::parser::parse_part_1;
    use super::{solve_part_1, solve_part_2};

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200
    ";

    #[test]
    fn part_1() {
        let r = assert_ok!(solve_part_1(INPUT));
        assert_eq!(r, 288)
    }
    #[test]
    fn part_2() {
        let r = assert_ok!(solve_part_2(INPUT));
        assert_eq!(r, 71503)
    }

    #[test]
    fn parse_both_lines() {
        let _x = assert_ok!(parse_part_1(INPUT));
    }
}
