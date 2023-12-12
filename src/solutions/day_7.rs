use crate::Solver;
use color_eyre::eyre::Result;

pub struct Day;

impl Solver for Day {
    fn part_1(&self, input: &str) -> Result<String> {
        todo!()
    }

    fn part_2(&self, input: &str) -> Result<String> {
        todo!()
    }
}

struct Hand<'a> {
    cards: &'a str,
    bet: usize,
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

#[derive(Debug)]
enum Groups {
    FiveOfAkind(Card),
    FourOfAKind(Card),
    FullHouse(Card, Card),
    ThreeOfAKind(Card),
    TwoPair(Card, Card),
    OnePair(Card),
    HighCard(Card),
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
}
