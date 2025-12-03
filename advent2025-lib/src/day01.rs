use std::str::FromStr;

use crate::{Day, DayCalc, Examples, ParseError, ParseResult, PartOutput};

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Rotation {
    direction: Direction,
    steps: usize,
}

impl Rotation {
    pub(crate) fn val(&self) -> isize {
        let sign: isize = match self.direction {
            Direction::Left => -1,
            Direction::Right => 1,
        };
        sign.checked_mul(self.steps as isize).unwrap()
    }
}

impl FromStr for Rotation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = match s.chars().next().unwrap() {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            c => Err(ParseError::Str(format!("invalid direction {c}"))),
        }?;
        let steps = s.get(1..).unwrap().parse()?;
        Ok(Self { direction, steps })
    }
}

#[derive(Debug)]
pub struct Rotations(Vec<Rotation>);

impl Rotations {
    pub fn iter(&self) -> impl IntoIterator<Item = &Rotation> {
        &self.0
    }

    fn sequence(&self, start: usize) -> Vec<usize> {
        let mut sequence = vec![start];
        for rotation in self.iter() {
            let last_dial: isize = sequence.last().unwrap().to_owned().try_into().unwrap();
            let next_dial = last_dial.checked_add(rotation.val()).unwrap();
            let next_dial: usize = next_dial
                .checked_rem_euclid(100)
                .unwrap()
                .try_into()
                .unwrap();
            sequence.push(next_dial)
        }
        sequence
    }
}

impl FromStr for Rotations {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rotations = s
            .lines()
            .map(|s| s.trim().parse())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(rotations))
    }
}

pub fn parse(input: &str) -> ParseResult<Rotations> {
    Rotations::from_str(input)
}

pub fn part1(rotations: &Rotations) -> PartOutput<usize> {
    let sequence = rotations.sequence(50);
    let zeros = sequence.iter().filter(|&dial| *dial == 0).count();
    PartOutput { answer: zeros }
}

pub fn part2(rotations: &Rotations) -> PartOutput<usize> {
    let mut zero_count = 0usize;
    let mut sequence = vec![50];
    for rotation in rotations.iter() {
        let last_dial: isize = sequence.last().unwrap().to_owned().try_into().unwrap();
        let next_dial = last_dial.checked_add(rotation.val()).unwrap();
        // Zero counting
        {
            if last_dial == 0 {
                zero_count += usize::try_from(next_dial.checked_div(100).unwrap().abs()).unwrap();
            } else if next_dial <= 0 {
                zero_count += usize::try_from(
                    next_dial
                        .checked_sub(1)
                        .unwrap()
                        .checked_div_euclid(100)
                        .unwrap()
                        .abs(),
                )
                .unwrap();
            } else {
                zero_count += usize::try_from(next_dial.checked_div_euclid(100).unwrap()).unwrap();
            }
        }
        let next_dial: usize = next_dial
            .checked_rem_euclid(100)
            .unwrap()
            .try_into()
            .unwrap();
        sequence.push(next_dial);
        log::debug!(
            "{} by {} to {} with {} zeros",
            last_dial,
            rotation.val(),
            next_dial,
            zero_count
        )
    }
    PartOutput { answer: zero_count }
}

pub const DAY: Day<Rotations, usize, 1, 0, 0> = Day {
    title: "Secret Entrance",
    display: (
        "The password to open the door is maybe {answer}.",
        "The password to open the door is actually {answer}.",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    examples: Examples::single(include_str!("../../examples/day01.txt")),
};

#[cfg(test)]
mod tests {
    use test_log::test;

    use super::*;
    use crate::{Printable as _, get_input};

    #[test]
    fn test_example_part1() {
        let rotations = parse(DAY.examples().first()).unwrap();
        for rotation in rotations.iter() {
            log::info!("{:?} has val {}", rotation, rotation.val());
        }
        log::info!("Sequence: {:?}", rotations.sequence(50));
        let result = part1(&rotations);
        assert_eq!(result.unwrap(), 3);
    }

    #[test]
    fn test_example_part2() {
        let something = parse(DAY.examples().second()).unwrap();
        let result = part2(&something);
        assert_eq!(result.unwrap(), 6);
    }

    #[test]
    fn test_main() {
        let something = parse(&get_input(1)).unwrap();
        assert_eq!(part1(&something).answer.to_string(), "1034");
        assert_eq!(part2(&something).answer.to_string(), "6166");
    }
}
