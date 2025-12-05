//! Day 0: Template
//! Find and replace day00_template with dayXX, where XX is the day number.
//! Find and replace something, Something, line, Line, cell, and Cell.

use std::{collections::HashMap, fmt::Display, str::FromStr};

use crate::{Day, DayCalc, Examples, ParseError, PartOutput};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RowColPos {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Cell(char);

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

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

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for cell in &self.0 {
            write!(f, "{}", cell)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Something {
    lines: Vec<Line>,
    map: HashMap<RowColPos, Cell>,
}

impl Something {
    fn len(&self) -> usize {
        self.lines.len()
    }
}

impl FromStr for Something {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let mut splits = lines.split(|l| l.is_empty());
        let lines = splits
            .next()
            .unwrap()
            .into_iter()
            .copied()
            .map(Line::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        let mut map = HashMap::new();
        for (row, line) in splits.next().unwrap().into_iter().copied().enumerate() {
            for (col, cell) in line.chars().enumerate() {
                map.insert(RowColPos { row, col }, Cell(cell));
            }
        }
        Ok(Something { lines, map })
    }
}

impl Display for Something {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.lines {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

pub fn part1(something: &Something) -> PartOutput<usize> {
    PartOutput {
        answer: something.len(),
    }
}

pub fn part2(something: &Something) -> PartOutput<usize> {
    PartOutput {
        answer: something.map.len(),
    }
}

pub const DAY: Day<Something, usize, 1, 0, 0> = Day {
    day: 0,
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
    use crate::DayTrait as _;
    use crate::Printable as _;

    #[test]
    fn test_example_part1() {
        let parse = DAY.calc.parse;
        let something = parse(DAY.examples().first()).unwrap();
        let result = part1(&something);
        assert_eq!(result.unwrap(), 2);
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
        let something = parse(&DAY.input()).unwrap();
        assert_eq!(part1(&something).answer.to_string(), "3");
        assert_eq!(part2(&something).answer.to_string(), "12");
    }
}
