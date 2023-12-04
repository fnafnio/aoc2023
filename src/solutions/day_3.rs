use std::{fmt::Display, ops::Range};

use regex::{Match, Regex};

use crate::Solver;

pub struct Day;

impl Solver for Day {
    fn part_1(&self, input: &str) -> String {
        todo!()
    }

    fn part_2(&self, input: &str) -> String {
        todo!()
    }
}

#[derive(Debug)]
struct Schematic<'a> {
    data: Vec<&'a str>,
}

impl<'a> Display for Schematic<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &l in self.data.iter() {
            f.write_fmt(format_args!("{l}\n"));
        }
        Ok(())
    }
}

impl<'a> Schematic<'a> {
    fn create(input: &'a str) -> Self {
        Self {
            data: input.lines().filter(|l| !l.is_empty()).collect(),
        }
    }

    fn get_numbers(&self) -> Vec<(usize, Match)> {
        let mut v = vec![];
        let re = Regex::new(r#"\D*([0-9]+)\D*"#).unwrap();

        for (i, &l) in self.data.iter().enumerate() {
            if let Some(c)= re.captures(l) {
            for m in c.get(1) {
                let range = m.range();
                v.push((i, m));
            }
        }}
        dbg!(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    #[test]
    fn part_1() {
        let schem = Schematic::create(INPUT);
        schem.get_numbers();
        println!("{schem}");
    }
}
