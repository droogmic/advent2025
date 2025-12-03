use crate::{Day, DayCalc, Examples, ParseError, ParseResult, PartOutput};
use std::{fmt::Display, iter::Sum, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Joltage(usize);

impl Sum for Joltage {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Joltage(0), |acc, item| Joltage(acc.0 + item.0))
    }
}

impl Display for Joltage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JoltageRating(u8);

#[derive(Debug, Clone)]
pub struct BatteryBank(Vec<JoltageRating>);

impl BatteryBank {
    fn joltage(&self) -> Joltage {
        Joltage(self.0.iter().rev().enumerate().fold(
            0usize,
            |acc, (rating_index, joltage_rating)| {
                acc.checked_add(
                    usize::from(joltage_rating.0) * 10usize.pow(rating_index.try_into().unwrap()),
                )
                .unwrap()
            },
        ))
    }
}

impl FromStr for BatteryBank {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BatteryBank(
            s.chars()
                .map(|c| c.to_string().parse().map(JoltageRating))
                .collect::<Result<Vec<JoltageRating>, _>>()?,
        ))
    }
}

#[derive(Debug, Clone)]
pub struct BatteryBanks(Vec<BatteryBank>);

impl FromStr for BatteryBanks {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BatteryBanks(
            s.lines()
                .map(|l| BatteryBank::from_str(l))
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

pub fn parse(input: &str) -> ParseResult<BatteryBanks> {
    BatteryBanks::from_str(input)
}

fn largest_joltage(banks: &BatteryBanks, battery_count: usize) -> Joltage {
    banks
        .0
        .iter()
        .map(|bank| {
            let mut max_bank = BatteryBank(
                vec![0; battery_count]
                    .into_iter()
                    .map(JoltageRating)
                    .collect(),
            );
            for joltage_window in bank.0.windows(battery_count) {
                let mut new_set = false;
                for (idx, joltage) in joltage_window.iter().enumerate() {
                    if new_set {
                        max_bank.0[idx] = *joltage;
                    } else if joltage.0 > max_bank.0[idx].0 {
                        max_bank.0[idx] = *joltage;
                        new_set = true;
                    }
                }
            }
            let bank_joltage: Joltage = max_bank.joltage();
            log::info!("Bank Joltage: {bank_joltage:?}");
            bank_joltage
        })
        .sum()
}

pub fn part1(banks: &BatteryBanks) -> PartOutput<Joltage> {
    PartOutput {
        answer: largest_joltage(banks, 2),
    }
}

pub fn part2(banks: &BatteryBanks) -> PartOutput<Joltage> {
    PartOutput {
        answer: largest_joltage(banks, 12),
    }
}

pub const DAY: Day<BatteryBanks, Joltage, 1, 0, 0> = Day {
    title: "TITLE",
    display: (
        "The total output joltage is: {answer}",
        "The new output joltage is: {answer}",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    examples: Examples::single(include_str!("../../examples/day03.txt")),
};

#[cfg(test)]
mod tests {
    use test_log::test;

    use super::*;
    use crate::{Printable as _, get_input};

    #[test]
    fn test_example_part1() {
        let banks = parse(DAY.examples().first()).unwrap();
        let result = part1(&banks);
        assert_eq!(result.unwrap(), Joltage(357));
    }

    #[test]
    fn test_example_part2() {
        let banks = parse(DAY.examples().second()).unwrap();
        let result = part2(&banks);
        assert_eq!(result.unwrap(), Joltage(3121910778619));
    }

    #[test]
    fn test_main() {
        let something = parse(&get_input(3)).unwrap();
        assert_eq!(part1(&something).answer.to_string(), "17316");
        assert_eq!(part2(&something).answer.to_string(), "171741365473332");
    }
}
