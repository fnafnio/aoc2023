use itertools::Itertools;
use regex::Regex;

use crate::Solver;
use std::fmt::format;

pub struct Day;

impl Solver for Day {
    fn part_1(&self, input: &str) -> String {
        let num: i32 = input.lines().map(|l| get_cal_value(l)).sum();
        format!("{num}")
    }

    fn part_2(&self, input: &str) -> String {
        let num: i32 = input
            .lines()
            .map(|l| replace_line(l))
            .map(|l| get_cal_value(&l))
            .sum();
        format!("{num}")
    }
}

fn get_cal_value(l: &str) -> i32 {
    let first = l.chars().filter(|c| c.is_digit(10)).next().unwrap();
    let last = l.chars().rev().filter(|c| c.is_digit(10)).next().unwrap();

    let s: String = [first, last].iter().collect();

    s.parse().unwrap()
}

// this seems to be slower
// fn get_cal_value_2(l: &str) -> i32 {
//     let v = l.chars().filter(|c| c.is_digit(10)).collect::<Vec<char>>();
//     let &first = v.first().unwrap();
//     let &last = v.last().unwrap();

//     let s: String = [first, last].iter().collect();

//     s.parse().unwrap()
// }

fn replace_line(l: &str) -> String {
    let re = Regex::new(r#"\b(one|two|three|four|five|six|seven|eight|nine|\d)"#).unwrap();
    let mut new = String::new();
    re.captures_iter(l).for_each(|x| {
        for i in x.iter().filter_map(|c| c) {
            let next = match i.as_str() {
                "0" | "zero" => "0",
                "1" | "one" => "1",
                "2" | "two" => "2",
                "3" | "three" => "3",
                "4" | "four" => "4",
                "5" | "five" => "5",
                "6" | "six" => "6",
                "7" | "seven" => "7",
                "8" | "eight" => "8",
                "9" | "nine" => "9",
                // Add more mappings as needed
                _ => continue, // Return nothing if no match
            };
            new.push_str(dbg!(next));
        }
    });

    // let result = re.captures_iter(l, {
    //     for Some(x) in captures.iter() {
    //         match x.as_str() {
    //             "0" | "zero" => "0",
    //             "1" | "one" => "1",
    //             "2" | "two" => "2",
    //             "3" | "three" => "3",
    //             "4" | "four" => "4",
    //             "5" | "five" => "5",
    //             "6" | "six" => "6",
    //             "7" | "seven" => "7",
    //             "8" | "eight" => "8",
    //             "9" | "nine" => "9",
    //             // Add more mappings as needed
    //             _ => "", // Return nothing if no match
    //         }
    //         .to_string();
    //     }
    // });
    dbg!(new)
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
    fn test_parser() {
        let a = INPUT.lines().map(|l| dbg!(get_cal_value(l))).sum();
        assert_eq!(142, a);
    }
    #[test]
    fn test_part2() {
        let num: i32 = INPUT_2
            .lines()
            .map(|l| replace_line(l))
            .map(|l| get_cal_value(&l))
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
