use crate::Solver;
use color_eyre::eyre::{eyre, Result};

use self::parser::parse;

pub struct Day;

impl Solver for Day {
    fn part_1(&self, input: &str) -> Result<String> {
        let result = solve_part_1(input)?;
        Ok(result.to_string())
    }

    fn part_2(&self, input: &str) -> Result<String> {
        todo!()
    }
}

mod parser {
    use color_eyre::eyre::eyre;
    use nom::{
        bytes::complete::tag,
        character::complete::{alpha1, digit1, line_ending, space1},
        combinator::map_res,
        multi::separated_list1,
        sequence::{preceded, tuple},
        IResult,
    };

    fn parse_line(input: &str) -> IResult<&str, Vec<usize>> {
        preceded(
            tuple((alpha1, tag(":"), space1)),
            separated_list1(space1, map_res(digit1, str::parse)),
        )(input)
    }

    fn parse_both_lines(input: &str) -> IResult<&str, (Vec<usize>, Vec<usize>)> {
        let (input, time) = parse_line(input)?;
        let (input, _) = line_ending(input)?;
        let (input, distance) = parse_line(input)?;
        Ok((input, (time, distance)))
    }

    pub(super) fn parse(input: &str) -> super::Result<Vec<(usize, usize)>> {
        let (_, (t, d)) =
            parse_both_lines(input).map_err(|e| eyre!("Error parsing input: {:?}", e))?;
        Ok(t.into_iter().zip(d.into_iter()).collect())
    }
}

fn solve_part_1(input: &str) -> Result<usize> {
    let races = parse(input)?;

    // n^2 - t*n - d = 0
    // n_12 = t/2 +- sqrt(t^2 / 4  +d)
    use std::num;
    let quad = |t: usize, d: usize| -> usize {
        let (t, d) = (t as f64, d as f64);
        let b = t / 2.0;
        let sqrt = b.mul_add(b, -d).sqrt();
        let min = (b - sqrt).floor() as usize + 1;
        let max = (b + sqrt).ceil() as usize - 1;
        // println!("{:.02}..{:.02}={:.02}", min, max, min - max);
        max - min + 1
    };

    Ok(races
        .iter()
        .map(|(t, d)| quad(*t, *d))
        .fold(1, |mut acc, r| acc * r))
}

#[cfg(test)]
mod tests {
    use assert_ok::assert_ok;

    use super::{parser::parse, *};
    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200
    ";

    #[test]
    fn part_1() {
        let _r = assert_ok!(solve_part_1(INPUT));
        assert_eq!(_r, 288)
    }

    #[test]
    fn parse_both_lines() {
        let _x = assert_ok!(parse(INPUT));
    }
}
