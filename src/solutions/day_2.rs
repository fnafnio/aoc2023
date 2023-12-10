use std::ops::{Add, AddAssign};

use crate::Solver;
use color_eyre::eyre::Result;
use color_eyre::eyre::{anyhow, eyre};
use derive_more::{Add, AddAssign};

use self::parser::game_parser;

pub struct Day;

impl Solver for Day {
    fn part_1(&self, input: &str) -> Result<String> {
        Ok(solve_part_1(input)?.to_string())
    }

    fn part_2(&self, input: &str) -> Result<String> {
        Ok(solve_part_2(input)?.to_string())
    }
}
#[derive(Debug, Default)]
struct Game {
    id: usize,
    set: Vec<ColorCounts>,
}

impl Game {
    fn is_valid(&self) -> bool {
        self.set.iter().all(|s| s.is_valid())
    }

    fn fewest_cubes(&self) -> ColorCounts {
        let mut fewest = ColorCounts::default();

        for x in self.set.iter() {
            fewest.blue = fewest.blue.max(x.blue);
            fewest.green = fewest.green.max(x.green);
            fewest.red = fewest.red.max(x.red);
        }

        fewest
    }
}

#[derive(Debug, Add, AddAssign, Default)]
struct ColorCounts {
    blue: usize,
    green: usize,
    red: usize,
}

impl ColorCounts {
    fn add_to_color(&mut self, c: Color, num: usize) {
        match c {
            Color::Blue => self.blue += num,
            Color::Green => self.green += num,
            Color::Red => self.red += num,
        }
    }

    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }

    fn product(&self) -> usize {
        self.blue * self.red * self.green
    }

    fn new_max() -> Self {
        Self {
            blue: usize::MAX,
            green: usize::MAX,
            red: usize::MAX,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Color {
    Blue,
    Green,
    Red,
}

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{digit1, multispace0, multispace1},
        combinator::{map, map_res},
        multi::separated_list1,
        sequence::tuple,
        IResult,
    };

    use super::{Color, ColorCounts, Game};

    pub fn parse_color(input: &str) -> IResult<&str, (usize, Color)> {
        let color_parser = alt((
            map(tag("blue"), |_| Color::Blue),
            map(tag("green"), |_| Color::Green),
            map(tag("red"), |_| Color::Red),
        ));

        let count_parser = map_res(digit1, |s: &str| s.parse::<usize>());
        let mut count_and_color_parser = tuple((count_parser, multispace0, color_parser));
        // let counts_parser = separated_list1(tag(","), count_and_color_parser);

        let (input, (cnt, _, color)) = count_and_color_parser(input)?;
        Ok((input, (cnt, color)))
    }

    pub fn parse_set(input: &str) -> IResult<&str, ColorCounts> {
        let mut p = separated_list1(tag(", "), parse_color);
        let (i, r) = p(input)?;
        let mut cc = ColorCounts::default();
        for (cnt, col) in r.into_iter() {
            cc.add_to_color(col, cnt);
        }
        Ok((i, cc))
    }

    pub fn game_parser(input: &str) -> IResult<&str, Game> {
        let mut color_counts = ColorCounts::default();

        let mut id_parser = tuple((
            tag("Game"),
            multispace0,
            map_res(digit1, |s: &str| s.parse::<usize>()),
            tag(": "),
        ));

        let (input, (_, _, id, _)) = id_parser(input)?;

        let (input, set) = separated_list1(tag("; "), parse_set)(input)?;

        Ok((input, Game { id, set }))
    }
}

fn solve_part_1(input: &str) -> color_eyre::eyre::Result<usize> {
    let mut cnt = 0;
    for l in input.lines() {
        let game = match parser::game_parser(l) {
            Ok((_, g)) => g,
            Err(e) => return Err(eyre!("failed to parse a line {l}")),
        };
        if game.is_valid() {
            cnt += game.id;
        }
    }
    Ok(cnt)
}

fn solve_part_2(input: &str) -> Result<usize> {
    let sum = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| game_parser(l).unwrap().1.fewest_cubes().product())
        .sum();
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use crate::solutions::day_2::parser::game_parser;
    use assert_ok::assert_ok;

    use super::{parser::parse_color, *};
    const INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn test_part_1() {
        let cnt = assert_ok!(solve_part_1(INPUT));
        assert_eq!(8, cnt);
    }

    #[test]
    fn test_part_2() {
        let cnt = assert_ok!(solve_part_2(INPUT));
        assert_eq!(2286, cnt);
    }
}
