use std::str::FromStr;

use crate::{Day, DayCalc, Examples, ParseError, ParseResult, PartOutput};

#[derive(Debug)]
pub struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn ints_chars(&self) -> Vec<(usize, Vec<char>)> {
        let mut retval = Vec::new();
        for i in self.start..=self.end {
            retval.push((i, i.to_string().chars().collect()));
        }
        retval
    }
}

#[derive(Debug)]
pub struct Ranges(Vec<Range>);

impl Ranges {
    fn iter(&self) -> std::slice::Iter<'_, Range> {
        self.0.iter()
    }
}

impl FromStr for Ranges {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Ranges(
            s.trim()
                .split(',')
                .map(|range| -> Result<Range, ParseError> {
                    let (start, end) = range.split_once('-').unwrap();
                    Ok(Range {
                        start: start.parse()?,
                        end: end.parse()?,
                    })
                })
                .collect::<Result<Vec<Range>, ParseError>>()?,
        ))
    }
}

pub fn parse(input: &str) -> ParseResult<Ranges> {
    Ranges::from_str(input)
}

pub fn part1(ranges: &Ranges) -> PartOutput<usize> {
    let retval = ranges
        .iter()
        .map(|range| {
            range
                .ints_chars()
                .into_iter()
                .filter_map(|(int, seq)| {
                    if seq.len().checked_rem(2).unwrap() != 0 {
                        None
                    } else {
                        let midpoint = seq.len().checked_div(2).unwrap();
                        let (first, second) = seq.split_at_checked(midpoint).unwrap();
                        if first == second { Some(int) } else { None }
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>();
    PartOutput { answer: retval }
}

fn find_divisors(n: usize) -> Vec<usize> {
    let mut divisors = Vec::new();
    let sqrt_n = (n as f64).sqrt() as usize;
    for i in 1..=sqrt_n {
        if n % i == 0 {
            divisors.push(i);
            if i != n / i {
                divisors.push(n / i);
            }
        }
    }
    divisors.sort_unstable();
    divisors
}

pub fn part2(ranges: &Ranges) -> PartOutput<usize> {
    let retval = ranges
        .iter()
        .map(|range| {
            range
                .ints_chars()
                .into_iter()
                .filter_map(|(int, seq)| {
                    find_divisors(seq.len())
                        .into_iter()
                        .rev()
                        .skip(1) // skip the number itself
                        .find_map(|div| {
                            if seq.chunks(div).all(|c| c == &seq[..div]) {
                                Some(int)
                            } else {
                                None
                            }
                        })
                })
                .sum::<usize>()
        })
        .sum::<usize>();
    PartOutput { answer: retval }
}

pub const DAY: Day<Ranges, usize, 1, 0, 0> = Day {
    title: "Gift Shop",
    display: (
        "The sum of all the invalid IDs using old rules is: {answer}",
        "The sum of all the invalid IDs using new rules is: {answer}",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    examples: Examples::single(include_str!("../../examples/day02.txt")),
};

#[cfg(test)]
mod tests {
    use test_log::test;

    use super::*;
    use crate::{Printable as _, get_input};

    #[test]
    fn test_find_divisors_12() {
        let divisors = find_divisors(12);
        assert_eq!(divisors, vec![1, 2, 3, 4, 6, 12]);
    }

    #[test]
    fn test_find_divisors_120() {
        let divisors = find_divisors(120);
        assert_eq!(
            divisors,
            vec![1, 2, 3, 4, 5, 6, 8, 10, 12, 15, 20, 24, 30, 40, 60, 120]
        );
    }

    #[test]
    fn test_example_part1() {
        let ranges = parse(DAY.examples().first()).unwrap();
        let result = part1(&ranges);
        assert_eq!(result.unwrap(), 1227775554);
    }

    #[test]
    fn test_example_part2() {
        let ranges = parse(DAY.examples().second()).unwrap();
        let result = part2(&ranges);
        assert_eq!(result.unwrap(), 4174379265);
    }

    #[test]
    fn test_main() {
        let ranges = parse(&get_input(2)).unwrap();
        assert_eq!(part1(&ranges).answer.to_string(), "26255179562");
        assert_eq!(part2(&ranges).answer.to_string(), "31680313976");
    }
}
