use std::{collections::HashMap, fmt::Display, str::FromStr};

use crate::{Day, DayCalc, Examples, ParseError, PartOutput};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RowColPos {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RollPaper {
    Absent,
    Present,
}

#[derive(Debug, Clone)]
pub struct Line(Vec<RollPaper>);

impl FromStr for Line {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Line(
            s.chars()
                .map(|c| match c {
                    '.' => RollPaper::Absent,
                    '@' => RollPaper::Present,
                    _ => panic!("Invalid char: {}", c),
                })
                .collect(),
        ))
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for roll_paper in &self.0 {
            match roll_paper {
                RollPaper::Absent => write!(f, ".")?,
                RollPaper::Present => write!(f, "@")?,
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Diagram {
    lines: Vec<Line>,
    map: HashMap<RowColPos, RollPaper>,
}

impl Diagram {
    fn adjacent_3_3(&self, pos: &RowColPos) -> [[Option<RollPaper>; 3]; 3] {
        let mut result = [[None; 3]; 3];
        for row_offset in -1isize..=1isize {
            for col_offset in -1isize..=1isize {
                let row = isize::try_from(pos.row)
                    .unwrap()
                    .checked_add(row_offset)
                    .unwrap();
                let col = isize::try_from(pos.col)
                    .unwrap()
                    .checked_add(col_offset)
                    .unwrap();
                if row < 0 || col < 0 {
                    continue;
                }
                let row = usize::try_from(row).unwrap();
                let col = usize::try_from(col).unwrap();
                if row >= self.lines.len() || col >= self.lines[0].0.len() {
                    continue;
                }
                result[usize::try_from(row_offset + 1).unwrap()]
                    [usize::try_from(col_offset + 1).unwrap()] =
                    Some(self.map[&RowColPos { row, col }]);
            }
        }
        result
    }
    fn adjacent_8(&self, pos: &RowColPos) -> [Option<RollPaper>; 8] {
        let adjacents = self.adjacent_3_3(pos);
        [
            adjacents[0][0],
            adjacents[0][1],
            adjacents[0][2],
            adjacents[1][0],
            adjacents[1][2],
            adjacents[2][0],
            adjacents[2][1],
            adjacents[2][2],
        ]
    }
}

impl FromStr for Diagram {
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
        let diagram = Diagram { lines, map };
        log::debug!("Parsed diagram:\n{diagram}");
        Ok(diagram)
    }
}

impl Display for Diagram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.lines {
            writeln!(f, "{line}")?;
        }
        Ok(())
    }
}

pub fn part1(diagram: &Diagram) -> PartOutput<usize> {
    let forklift_accessible = diagram
        .map
        .keys()
        .filter(|pos| {
            diagram.map.get(&pos) == Some(&RollPaper::Present)
                && diagram
                    .adjacent_8(pos)
                    .into_iter()
                    .filter(|&adjacent_val| adjacent_val == Some(RollPaper::Present))
                    .count()
                    < 4
        })
        .inspect(|pos| log::debug!("Can remove: {:?}", pos))
        .count();
    PartOutput {
        answer: forklift_accessible,
    }
}

pub fn part2(diagram: &Diagram) -> PartOutput<usize> {
    let mut diagram = diagram.clone();
    let mut total_rolls_removed = 0;
    let mut loop_rolls_removed = usize::MAX;
    while loop_rolls_removed > 0 {
        let removed_positions: Vec<RowColPos> = diagram
            .map
            .keys()
            .filter(|pos| {
                diagram.map.get(&pos) == Some(&RollPaper::Present)
                    && diagram
                        .adjacent_8(pos)
                        .into_iter()
                        .filter(|&adjacent_val| adjacent_val == Some(RollPaper::Present))
                        .count()
                        < 4
            })
            .inspect(|pos| log::debug!("Can remove: {:?}", pos))
            .copied()
            .collect();
        loop_rolls_removed = removed_positions.len();
        total_rolls_removed += loop_rolls_removed;
        for pos in removed_positions {
            *diagram.map.get_mut(&pos).unwrap() = RollPaper::Absent;
        }
    }
    PartOutput {
        answer: total_rolls_removed,
    }
}

pub const DAY: Day<Diagram, usize, 1, 0, 0> = Day {
    title: "Printing Department",
    display: (
        "{answer} rolls of paper can be accessed by a forklift.",
        "{answer} rolls of paper in total can be removed by the Elves and their forklifts.",
    ),
    calc: DayCalc {
        parse: Diagram::from_str,
        part1,
        part2,
    },
    examples: Examples::single(include_str!("../../examples/day04.txt")),
};

#[cfg(test)]
mod tests {
    use test_log::test;

    use super::*;
    use crate::{Printable as _, get_input};

    #[test]
    fn test_example_part1() {
        let parse = DAY.calc.parse;
        let diagram = parse(DAY.examples().first()).unwrap();
        let result = part1(&diagram);
        assert_eq!(result.unwrap(), 13);
    }

    #[test]
    fn test_example_part2() {
        let parse = DAY.calc.parse;
        let diagram = parse(DAY.examples().second()).unwrap();
        let result = part2(&diagram);
        assert_eq!(result.unwrap(), 43);
    }

    #[test]
    fn test_main() {
        let parse = DAY.calc.parse;
        let diagram = parse(&get_input(0)).unwrap();
        assert_eq!(part1(&diagram).answer.to_string(), "1451");
        assert_eq!(part2(&diagram).answer.to_string(), "8701");
    }
}
