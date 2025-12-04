use std::{collections::HashMap, str::FromStr};

/// Day 0: Template
/// Find and replace day00_template with dayXX, where XX is the day number.
/// Find and replace something, Something, line, Line, cell, and Cell.
use crate::{Day, DayCalc, Examples, ParseError, PartOutput};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RowColPos {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Cell(char);

#[derive(Debug, Clone)]
pub struct Line(Vec<Cell>);

impl FromStr for Line {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 10 {
            return Err(ParseError::Empty);
        }
        Ok(Line(s.chars().map(Cell).collect()))
    }
}

#[derive(Debug, Clone)]
pub struct Something {
    lines: Vec<Line>,
    map: HashMap<RowColPos, Cell>,
}

impl FromStr for Something {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s
            .lines()
            .map(Line::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        let mut map = HashMap::new();
        for (row, line) in lines.iter().enumerate() {
            for (col, cell) in line.0.iter().enumerate() {
                map.insert(RowColPos { row, col }, *cell);
            }
        }
        Ok(Something { lines, map })
    }
}

pub fn part1(something: &Something) -> PartOutput<usize> {
    PartOutput {
        answer: something.lines.len(),
    }
}

pub fn part2(something: &Something) -> PartOutput<usize> {
    PartOutput {
        answer: something.map.len(),
    }
}

pub const DAY: Day<Something, usize, 1, 0, 0> = Day {
    title: "TITLE",
    display: (
        "Foobar foobar foobar: {answer}",
        "Foobar foobar foobar: {answer}",
    ),
    calc: DayCalc {
        parse: Something::from_str,
        part1,
        part2,
    },
    examples: Examples::single(include_str!("../../examples/day00_template.txt")),
};

#[cfg(test)]
mod tests {
    use test_log::test;

    use super::*;
    use crate::{Printable as _, get_input};

    #[test]
    fn test_example_part1() {
        let parse = DAY.calc.parse;
        let something = parse(DAY.examples().first()).unwrap();
        let result = part1(&something);
        assert_eq!(result.unwrap(), 3);
    }

    #[test]
    fn test_example_part2() {
        let parse = DAY.calc.parse;
        let something = parse(DAY.examples().second()).unwrap();
        let result = part2(&something);
        assert_eq!(result.unwrap(), 9);
    }

    #[test]
    fn test_main() {
        let parse = DAY.calc.parse;
        let something = parse(&get_input(0)).unwrap();
        assert_eq!(part1(&something).answer.to_string(), "3");
        assert_eq!(part2(&something).answer.to_string(), "12");
    }
}
