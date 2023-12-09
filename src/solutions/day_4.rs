use std::collections::HashSet;

use crate::Solver;
use color_eyre::eyre::{eyre, Result};
pub struct Day;

impl Solver for Day {
    fn part_1(&self, input: &str) -> Result<String> {
        let sum: usize = input
            .lines()
            .filter_map(|l| Card::parse(l).ok())
            .map(|c| match c.count_winners() {
                x @ 0..1 => x,
                x => 1 << x - 1,
            })
            .sum();
        Ok(sum.to_string())
    }

    fn part_2(&self, input: &str) -> Result<String> {
        todo!()
    }
}

#[derive(Debug)]
struct Card<'a> {
    num: usize,
    winners: HashSet<&'a str>,
    have: Vec<&'a str>,
}

impl<'a> Card<'a> {
    fn parse(input: &'a str) -> Result<Self> {
        match parser::parser(input) {
            Ok((_, c)) => Ok(c),
            Err(e) => Err(eyre!("Error while parsing line: {:?}", e)),
        }
    }

    fn count_winners(&self) -> usize {
        let &Self { num, winners, have } = &self;
        have.iter().filter_map(|h| winners.get(h)).count()
    }
}

mod parser {
    use super::{Card, Result};
    use std::collections::HashSet;

    use color_eyre::eyre::eyre;
    use nom::{
        bytes::complete::tag,
        character::complete::{char, digit1, line_ending, space0, space1},
        multi::{many1, separated_list1},
        sequence::{delimited, pair, preceded, separated_pair, terminated},
        IResult,
    };

    pub(super) fn parser(input: &str) -> IResult<&str, Card> {
        let (input, num) = delimited(tag("Card "), digit1, tag(":"))(input)?;
        let num = num.parse().unwrap(); // because we are sure to only have digits
        let (input, _) = space0(input)?;
        let (input, (winners, have)) = separated_pair(
            separated_list1(space1, digit1),
            tag(" | "),
            separated_list1(space1, digit1),
        )(input)?;
        let winners = winners.into_iter().collect();

        Ok((input, Card { num, winners, have }))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use assert_ok::assert_ok;
    const INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    const DAY: Day = Day;

    #[test]
    fn parser() {
        for l in INPUT.lines() {
            let c = assert_ok!(Card::parse(l));
        }
    }

    #[test]
    fn winners() {
        let sum = assert_ok!(Day.part_1(INPUT));
        assert_eq!(13, sum)
    }
}
