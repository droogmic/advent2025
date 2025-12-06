use std::str::FromStr;

use crate::{Day, DayCalc, Examples, ParseError, ParseResult, PartOutput};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn from_char(c: char) -> ParseResult<Self> {
        match c {
            '+' => Ok(Self::Add),
            '*' => Ok(Self::Multiply),
            _ => Err(ParseError::Empty),
        }
    }
}

impl FromStr for Operation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let first = chars.next().ok_or(ParseError::Empty)?;
        let op = Self::from_char(first)?;
        if chars.next().is_some() {
            return Err(ParseError::Empty);
        }
        Ok(op)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Problem {
    operands: Vec<usize>,
    operation: Operation,
}

impl Problem {
    fn solve(&self) -> usize {
        match self.operation {
            Operation::Add => self.operands.iter().sum(),
            Operation::Multiply => self.operands.iter().product(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HomeworkPart1 {
    problems: Vec<Problem>,
}

impl FromStr for HomeworkPart1 {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let (operation_line, operand_lines) = lines.split_last().unwrap();
        let operands_by_line: Vec<Vec<usize>> = operand_lines
            .iter()
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|num| num.parse())
                    .collect::<Result<Vec<usize>, _>>()
            })
            .collect::<Result<Vec<Vec<usize>>, _>>()?;
        let operands_by_column: Vec<Vec<usize>> = (0usize..operands_by_line[0].len())
            .map(|col_idx| operands_by_line.iter().map(|line| line[col_idx]).collect())
            .collect();
        let operations: Vec<Operation> = operation_line
            .split_ascii_whitespace()
            .map(|op| op.parse())
            .collect::<Result<Vec<Operation>, ParseError>>()?;
        Ok(HomeworkPart1 {
            problems: operands_by_column
                .iter()
                .zip(operations.iter())
                .map(|(operands, operation)| Problem {
                    operands: operands.to_owned(),
                    operation: operation.to_owned(),
                })
                .collect(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct HomeworkPart2 {
    problems: Vec<Problem>,
}

impl FromStr for HomeworkPart2 {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows_cols: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
        let longest_row = rows_cols.iter().map(|row| row.len()).max().unwrap_or(0);
        let cols_rows: Vec<Vec<char>> = (0..longest_row)
            .map(|col_idx| {
                rows_cols
                    .iter()
                    .map(|line| line.get(col_idx).unwrap_or(&' '))
                    .copied()
                    .collect()
            })
            .rev()
            .collect();
        let problems: Vec<Problem> = cols_rows
            .split(|line| line.iter().all(|c| c.is_whitespace()))
            .map(|chars: &[Vec<char>]| {
                let operands: Vec<usize> = chars
                    .iter()
                    .map(|line| {
                        line.iter()
                            .rev()
                            .skip(1)
                            .rev()
                            .collect::<String>()
                            .trim_ascii()
                            .parse()
                    })
                    .collect::<Result<_, _>>()?;
                let operation: Operation =
                    Operation::from_char(*chars.last().unwrap().last().unwrap())?;
                Ok(Problem {
                    operands,
                    operation,
                })
            })
            .collect::<ParseResult<Vec<Problem>>>()?;
        Ok(Self { problems })
    }
}

pub fn parse(s: &str) -> ParseResult<String> {
    // Do nothing
    Ok(s.to_string())
}

pub fn part1(s: &String) -> PartOutput<usize> {
    let homework = HomeworkPart1::from_str(s).unwrap();
    PartOutput {
        answer: homework
            .problems
            .iter()
            .map(|problem| problem.solve())
            .sum(),
    }
}

pub fn part2(s: &String) -> PartOutput<usize> {
    let homework = HomeworkPart2::from_str(s).unwrap();
    PartOutput {
        answer: homework
            .problems
            .iter()
            .map(|problem| problem.solve())
            .sum(),
    }
}

pub const DAY: Day<String, usize, 1, 0, 0> = Day {
    day: 6,
    title: "Trash Compactor",
    display: (
        "The grand total reading row-wise is: {answer}",
        "The grand total reading column-wise is: {answer}",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    examples: Examples::single(include_str!("../../examples/day06.txt")),
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
        let homework = parse(DAY.examples().first()).unwrap();
        let result = part1(&homework);
        assert_eq!(result.unwrap(), 4277556);
    }

    #[test]
    fn test_example_part2() {
        let parse = DAY.calc.parse;
        let homework = parse(DAY.examples().second()).unwrap();
        let result = part2(&homework);
        assert_eq!(result.unwrap(), 3263827);
    }

    #[test]
    fn test_main() {
        let parse = DAY.calc.parse;
        let homework = parse(&DAY.input()).unwrap();
        assert_eq!(part1(&homework).answer.to_string(), "5977759036837");
        assert_eq!(part2(&homework).answer.to_string(), "9630000828442");
    }
}
