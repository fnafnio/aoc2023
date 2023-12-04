use core::borrow;
use std::{fmt::Display, ops::Range};

use color_eyre::eyre::eyre;
use pathfinding::num_traits::Num;
use regex::{Match, Regex};

use crate::{Result, Solver};

pub struct Day;

impl Solver for Day {
    fn part_1(&self, input: &str) -> Result<String> {
        let schem = Schematic::create(input);
        let nums = schem.get_numbers();
        let cnt = count_part_numbers(nums, schem);
        Ok(cnt.to_string())
    }

    fn part_2(&self, input: &str) -> Result<String> {
        todo!()
    }
}

#[derive(Debug)]
struct Schematic<'a> {
    data: Vec<&'a str>,
    size: (usize, usize),
}

impl<'a> Display for Schematic<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &l in self.data.iter() {
            f.write_fmt(format_args!("{l}\n"));
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
struct Number {
    line: usize,
    start: usize,
    end: usize,
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { line, start, end } = self;
        write!(f, "{line:3}: ({start:2}..{end:<2})")
    }
}

impl<'a> Schematic<'a> {
    fn create(input: &'a str) -> Self {
        let data: Vec<_> = input.lines().filter(|l| !l.is_empty()).collect();
        let x = data.first().unwrap().len();
        let y = data.len();
        Self { data, size: (x, y) }
    }

    fn get_numbers(&self) -> Vec<Number> {
        let mut v = vec![];
        let re = Regex::new(r#"(\d+)"#).unwrap();

        for (i, &l) in self.data.iter().enumerate() {
            for c in re.captures_iter(l) {
                if let Some(m) = c.get(1) {
                    let range = m.range();

                    let n = Number {
                        line: i,
                        start: m.start(),
                        end: m.end(),
                    };

                    v.push(n);
                }
            }
        }
        v
    }

    fn get_value(&self, n: &Number) -> usize {
        let &s = self.data.get(n.line).unwrap();
        (s[n.start..n.end].parse()).unwrap()
    }

    fn get_neighbors(&self, n: &Number) -> Vec<&str> {
        let start = n.start.saturating_sub(1);
        let end = (n.end + 1).min(self.size.0);
        let top = n.line.saturating_sub(1);
        let btm = (n.line + 1).min(self.size.1);
        let mut v = vec![];
        for i in top..btm + 1 {
            if let Some(&s) = self.data.get(i) {
                v.push(&s[start..end]);
            }
        }
        v
    }
}

fn count_part_numbers(nums: Vec<Number>, schem: Schematic<'_>) -> usize {
    let mut cnt = 0;
    for n in nums.iter() {
        let area = schem.get_neighbors(n);
        if area
            .iter()
            .flat_map(|s| s.chars())
            .filter(|c| !c.is_ascii_digit())
            .any(|c| c != '.')
        {
            cnt += schem.get_value(n);
        }
    }
    cnt
}

#[cfg(test)]
mod tests {
    use assert_ok::assert_ok;

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
        let nums = schem.get_numbers();
        let cnt = count_part_numbers(nums, schem);
        assert_eq!(4361, cnt)
    }
}
