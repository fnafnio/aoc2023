use std::fmt::Display;

use crate::Solver;
use color_eyre::eyre::{eyre, Error, Result};
use itertools::{Group, Itertools};

pub struct Day;

impl Solver for Day {
    fn part_1(&self, input: &str) -> Result<String> {
        Ok(solve_part_1(input)?.to_string())
    }

    fn part_2(&self, input: &str) -> Result<String> {
        todo!()
    }
}

#[derive(Eq, PartialEq, PartialOrd, Debug)]
struct Hand {
    pub(crate) bet: usize,
    pub(crate) hands: Vec<Groups>,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = self.hands.iter().sorted_by(|a, b| Ord::cmp(b, a));
        let b = other.hands.iter().sorted_by(|a, b| Ord::cmp(b, a));

        for (a, b) in a.zip(b) {
            match a.cmp(b) {
                x @ std::cmp::Ordering::Greater | x @ std::cmp::Ordering::Less => return x,
                std::cmp::Ordering::Equal => continue,
            }
        }
        std::cmp::Ordering::Equal
    }
}

impl TryFrom<&str> for Hand {
    type Error = color_eyre::Report;

    fn try_from(value: &str) -> Result<Self> {
        let a = &value[..5];
        let b = &value[6..];
        // let hand = a.try_into()?;
        let hands = Groups::parse_hand(a)?;
        let bet = b
            .parse()
            .map_err(|e| eyre!("{b} cannot be parsed to an integer: {e:?}"))?;
        Ok(Self { bet, hands })
    }
}

fn solve_part_1(input: &str) -> Result<usize> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| Hand::try_from(l))
        .process_results(|iter| {
            iter.into_iter()
                .sorted_by(|h1, h2| Ord::cmp(h1, h2))
                .enumerate()
                .map(|(i, h)| (i + 1) * h.bet)
                .sum()
        })
}

impl Hand {
    fn new(bet: usize, hands: Vec<Groups>) -> Self {
        Self { bet, hands }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Num(usize),
    T,
    J,
    Q,
    K,
    A,
}

impl TryFrom<char> for Card {
    type Error = color_eyre::Report;

    fn try_from(value: char) -> Result<Self> {
        match value {
            c @ '2'..':' => Ok(Self::Num(c as usize - '0' as usize)),
            'T' => Ok(Self::T),
            'J' => Ok(Self::J),
            'Q' => Ok(Self::Q),
            'K' => Ok(Self::K),
            'A' => Ok(Self::A),
            x => Err(eyre!("{x} is not a valid character")),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Groups {
    HighCard(Card),
    OnePair(Card),
    TwoPair(Card, Card),
    ThreeOfAKind(Card),
    FullHouse(Card, Card),
    FourOfAKind(Card),
    FiveOfAkind(Card),
}

impl Groups {
    fn parse_hand(s: &str) -> Result<Vec<Groups>> {
        let hand: Vec<(_, _)> = s
            .chars()
            .sorted()
            .group_by(|c| *c)
            .into_iter()
            .map(|(c, g)| -> Result<(Card, usize)> {
                let g: Vec<_> = g.collect();
                Ok((Card::try_from(c)?, g.len()))
            })
            .process_results(|iter| iter.into_iter().sorted_by(|a, b| b.1.cmp(&a.1)).collect())?;
        let mut hand = hand.into_iter();

        let new_group = if let Some(h) = hand.next() {
            match h {
                (x, 1) => Ok(Self::HighCard(x)),
                (x, 2) => match hand.next().unwrap() {
                    (y, 2) => {
                        let (x, y) = match x.cmp(&y) {
                            std::cmp::Ordering::Less => (y, x),
                            std::cmp::Ordering::Equal => {
                                unreachable!()
                            }
                            std::cmp::Ordering::Greater => (x, y),
                        };
                        debug_assert!(x > y);
                        Ok(Self::TwoPair(x, y))
                    }
                    _ => Ok(Self::OnePair(x)),
                },
                (x, 3) => match hand.next().unwrap() {
                    (y, 2) => Ok(Self::FullHouse(x, y)),
                    _ => Ok(Self::ThreeOfAKind(x)),
                },
                (x, 4) => Ok(Self::FourOfAKind(x)),
                (x, 5) => Ok(Self::FiveOfAkind(x)),
                (x, n) => Err(eyre!("{} is too many of one card!", n)),
            }
        } else {
            Err(eyre!("This did not parse right"))
        };
        let mut result = vec![new_group?];
        for (c, n) in hand {
            debug_assert_eq!(1, n);
            result.push(Self::HighCard(c))
        }

        result.sort_by(|a,b| Ord::cmp(&b, &a));

        let mut cnt = 0usize;
        result.iter().tuple_windows().for_each(|(a, b)| {
            if a < b {
                println!("{cnt:4}: {a:?}, {b:?}");
                cnt += 1;
            }
        });

        Ok(result)
    }
}

// impl TryFrom<&str> for Groups {
//     type Error = color_eyre::Report;

//     fn try_from(value: &str) -> Result<Self> {
//         if value.len() != 5 {
//             Err(eyre!("wrong input length {}", value.len()))
//         } else {
//             Ok(*Groups::parse_hand(value)?
//                 .first()
//                 .ok_or(eyre!("nothing found in string"))?)
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use assert_ok::assert_ok;
    use itertools::Itertools;
    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    // #[test]
    // fn test_full_part_1() {
    //     let input = include_str!("../../input/day_7");
    //     let mut vec = vec![];
    //     for l in input.lines().filter(|l| !l.is_empty()) {
    //         let h = assert_ok!(Hand::try_from(l));
    //         // println!("{h}");
    //         vec.push(h);
    //     }
    // //     vec.sort_by(|a, b| Ord::cmp(&a.hand, &b.hand));
    // //     // dbg!(&vec);
    // //     for v in vec.iter() {
    // //         // println!("{v}");
    // //     }
    // // }

    #[test]
    fn part_1() {
        let r = assert_ok!(solve_part_1(INPUT));
        assert_eq!(6440, r)
    }

    #[test]
    fn test_parse() {
        for l in INPUT.lines() {
            let h = assert_ok!(Hand::try_from(l));
        }
    }

    #[test]
    fn test_groups() {
        let a = assert_ok!(Groups::parse_hand("TAAKK"));
        let b = assert_ok!(Groups::parse_hand("KTJJT"));
        dbg!((&a, &b));
        assert!(a > b);
    }

    #[test]
    fn card_ord() {
        use Card::*;
        assert!(Num(0) < Num(1));
        assert!(Num(9) < T);
        assert!(Num(9) < A);
        assert!(T < J);
        assert!(J < Q);
        assert!(Q < K);
        assert!(K < A);
    }

    #[test]
    fn group_ord() {
        use Card::*;
        use Groups::*;
        assert!(HighCard(Num(5)) < HighCard(A));
        assert!(HighCard(A) < OnePair(T));
        assert!(FullHouse(K, T) < FullHouse(A, T));
        assert!(FullHouse(A, T) < FullHouse(A, J));
        assert!(FullHouse(A, T) < FiveOfAkind(J));
        assert!(TwoPair(A, T) < TwoPair(A, J));
        assert!(TwoPair(A, J) > TwoPair(A, T));
        assert_eq!(TwoPair(A, J), TwoPair(A, J));
    }
}
