use itertools::Itertools;
use regex::{Captures, Regex};

use crate::Solver;
use std::fmt::format;

pub struct Day;

impl Solver for Day {
    fn part_1(&self, input: &str) -> String {
        let num: i32 = input
            .lines()
            .filter(|l| !l.is_empty())
            .map(get_cal_value)
            .sum();
        format!("{num}")
    }

    fn part_2(&self, input: &str) -> String {
        let num: i32 = input
            .lines()
            .filter(|l| !l.is_empty())
            .map(replace_line)
            .map(get_cal_value)
            .sum();
        format!("{num}")
    }
}

fn get_cal_value<S: AsRef<str>>(l: S) -> i32 {
    let l = l.as_ref();
    let first = l.chars().find(char::is_ascii_digit).unwrap();
    let last = l.chars().rev().find(char::is_ascii_digit).unwrap();

    let s: String = [first, last].iter().collect();

    s.parse().unwrap()
}

// slower than the iterator by about ~1000x
fn get_cal_value_regex(l: &str) -> i32 {
    let re = Regex::new(r"^\D*(\d).*?(\d){0,1}\D*$").unwrap();
    // let s = re.replace_all(l, "$1$2");
    let c = re.captures(l).unwrap();
    let mut s = String::new();
    let first = c.get(1).unwrap().as_str();
    s.push_str(first);
    s.push_str(match c.get(2) {
        Some(x) => x.as_str(),
        None => first,
    });
    s.parse().unwrap()
}

// this seems to be slower as well
fn get_cal_value_2(l: &str) -> i32 {
    let v = l
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<Vec<char>>();
    let &first = v.first().unwrap();
    let &last = v.last().unwrap();

    let s: String = [first, last].iter().collect();

    s.parse().unwrap()
}

fn replace_line(l: &str) -> String {
    let re = Regex::new(r#"\W*(zero|one|two|three|four|five|six|seven|eight|nine)\W*"#).unwrap();
    let mut new = String::new();

    // Use the regex replace method to replace the matched words with their corresponding digits
    let result = re.replace_all(l, |captures: &regex::Captures| {
        // if let Some(x) = captures.get(1) {
        //     return x.as_str().to_owned();
        // }
        match captures.get(1) {
            Some(x) => {
                match x.as_str() {
                    "one" => "1",
                    "two" => "2",
                    "three" => "3",
                    "four" => "4",
                    "five" => "5",
                    "six" => "6",
                    "seven" => "7",
                    "eight" => "8",
                    "nine" => "9",
                    // Add more mappings as needed
                    _ => "", // _ => captures.get(0).unwrap().as_str(), // Return the original if no match
                }
            }
            None => "",
        }
        .to_string()
    });

    result.into()
}
#[cfg(test)]
mod tests {

    use super::*;
    const INPUT: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

    const INPUT_2: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

    #[test]
    fn test_part_1() {
        let a = INPUT
            .lines()
            .filter(|l| !l.is_empty())
            .map(get_cal_value)
            .sum();
        assert_eq!(142, a);
    }
    #[test]
    fn test_part2() {
        let num: i32 = INPUT_2
            .lines()
            .filter(|l| !l.is_empty())
            .map(replace_line)
            .map(get_cal_value)
            .sum();
        assert_eq!(281, num);
    }

    // #[bench]
    // fn bench_variant_1(b: &mut Bencher) {
    //     // Specify the code to benchmark inside the closure passed to iter
    //     let input = include_str!("../../input/day_1");
    //     b.iter(|| {
    //         let num = .lines().map(|l| get_cal_value(l)).sum();
    //     });
    // }
}
