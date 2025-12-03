use crate::{Day, DayCalc, Examples, ParseError, ParseResult, PartOutput};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Joltage(u8);

#[derive(Debug, Clone)]
pub struct BatteryBank(Vec<Joltage>);

impl FromStr for BatteryBank {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BatteryBank(
            s.chars()
                .map(|c| c.to_string().parse().map(Joltage))
                .collect::<Result<Vec<Joltage>, _>>()?,
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

pub fn part1(banks: &BatteryBanks) -> PartOutput<usize> {
    let retval: usize = banks
        .0
        .iter()
        .map(|bank| {
            let mut first = Joltage(0);
            let mut second = Joltage(0);
            for joltage in bank.0.iter().rev().skip(1).rev() {
                if joltage.0 > first.0 {
                    first = *joltage;
                    second = Joltage(0)
                } else if joltage.0 > second.0 {
                    second = *joltage;
                }
            }
            if bank.0.last().unwrap().0 > second.0 {
                second = *bank.0.last().unwrap();
            }
            10 * usize::from(first.0) + usize::from(second.0)
        })
        .sum();
    PartOutput { answer: retval }
}

pub fn part2(banks: &BatteryBanks) -> PartOutput<usize> {
    let retval = banks
        .0
        .iter()
        .map(|bank| {
            let mut max_bank = BatteryBank(
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
                    .into_iter()
                    .map(Joltage)
                    .collect(),
            );
            for joltage_window in bank.0.windows(12) {
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
            let mut s = String::new();
            for j in max_bank.0 {
                let c = j.0.to_string();
                assert!(c.len() == 1);
                s.push_str(c.as_str())
            }
            let final_joltage: usize = s.parse().unwrap();
            log::info!("{:?}", final_joltage);
            final_joltage
        })
        .sum();
    PartOutput { answer: retval }
}

pub const DAY: Day<BatteryBanks, usize, 1, 0, 0> = Day {
    title: "TITLE",
    display: (
        "Foobar foobar foobar: {answer}",
        "Foobar foobar foobar: {answer}",
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
        assert_eq!(result.val(), 357);
    }

    #[test]
    fn test_example_part2() {
        let banks = parse(DAY.examples().second()).unwrap();
        let result = part2(&banks);
        assert_eq!(result.val(), 3121910778619);
    }

    #[test]
    fn test_main() {
        let something = parse(&get_input(3)).unwrap();
        assert_eq!(part1(&something).answer.to_string(), "17316");
        assert_eq!(part2(&something).answer.to_string(), "171741365473332");
    }
}
