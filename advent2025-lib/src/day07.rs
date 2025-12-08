use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::{Day, DayCalc, Examples, ParseError, PartOutput};

#[derive(Debug, Clone)]
pub struct SplitterLocations(Vec<isize>);

#[derive(Debug, Clone)]
pub struct Diagram {
    splitter_locations: Vec<SplitterLocations>,
}

impl FromStr for Diagram {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
        let start_idx = lines
            .first()
            .unwrap()
            .iter()
            .position(|c| *c == 'S')
            .expect(&format!(
                "input line {:?} should contain the letter S",
                lines.first().unwrap()
            ));
        Ok(Diagram {
            splitter_locations: lines
                .into_iter()
                .skip(1)
                .filter_map(|line| {
                    line.iter().any(|c| *c == '^').then(|| {
                        SplitterLocations(
                            line.into_iter()
                                .enumerate()
                                .filter_map(|(idx, c)| {
                                    (c == '^').then(|| {
                                        isize::try_from(idx)
                                            .unwrap()
                                            .checked_sub(isize::try_from(start_idx).unwrap())
                                            .unwrap()
                                    })
                                })
                                .collect(),
                        )
                    })
                })
                .collect(),
        })
    }
}

pub fn part1(diagram: &Diagram) -> PartOutput<usize> {
    let mut beam_locations: Vec<HashSet<isize>> = vec![HashSet::from([0])];
    let mut split_count = 0;
    for splitter_locations in &diagram.splitter_locations {
        let mut next_beam_locations = HashSet::new();
        for beam_location in beam_locations.last().unwrap() {
            if splitter_locations.0.contains(beam_location) {
                next_beam_locations.insert(beam_location.checked_sub(1).unwrap());
                next_beam_locations.insert(beam_location.checked_add(1).unwrap());
                split_count += 1;
            } else {
                next_beam_locations.insert(*beam_location);
            }
        }
        log::info!("Beam locations: {next_beam_locations:?}");
        beam_locations.push(next_beam_locations);
    }
    PartOutput {
        answer: split_count,
    }
}

#[allow(unused)]
pub fn part2_brute_force(diagram: &Diagram) -> PartOutput<usize> {
    let mut timelines_history: Vec<Vec<isize>> = vec![vec![0]];
    for splitter_locations in &diagram.splitter_locations {
        let mut next_timeline = Vec::new();
        for timeline_location in timelines_history.last().unwrap() {
            if splitter_locations.0.contains(timeline_location) {
                next_timeline.push(timeline_location.checked_sub(1).unwrap());
                next_timeline.push(timeline_location.checked_add(1).unwrap());
            } else {
                next_timeline.push(*timeline_location);
            }
        }
        timelines_history.push(next_timeline);
    }
    PartOutput {
        answer: timelines_history.last().unwrap().len(),
    }
}

pub fn part2_counter(diagram: &Diagram) -> PartOutput<usize> {
    let mut timelines_history: Vec<HashMap<isize, usize>> = vec![HashMap::from([(0, 1)])];
    for splitter_locations in &diagram.splitter_locations {
        let mut next_count = HashMap::new();
        for (timeline_location, count) in timelines_history.last().unwrap() {
            if splitter_locations.0.contains(timeline_location) {
                next_count
                    .entry(timeline_location.checked_sub(1).unwrap())
                    .and_modify(|c| *c += count)
                    .or_insert(*count);
                next_count
                    .entry(timeline_location.checked_add(1).unwrap())
                    .and_modify(|c| *c += count)
                    .or_insert(*count);
            } else {
                next_count
                    .entry(*timeline_location)
                    .and_modify(|c| *c += count)
                    .or_insert(*count);
            }
        }
        timelines_history.push(next_count);
    }
    PartOutput {
        answer: timelines_history.last().unwrap().values().sum(),
    }
}

pub const DAY: Day<Diagram, usize, 1, 0, 0> = Day {
    day: 7,
    title: "Laboratories",
    display: (
        "The beam will be split {answer} times.",
        "A single particle would end up on {answer} different timelines.",
    ),
    calc: DayCalc {
        parse: Diagram::from_str,
        part1,
        part2: part2_counter,
    },
    examples: Examples::single(include_str!("../../examples/day07.txt")),
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
        let diagram = parse(DAY.examples().first()).unwrap();
        let result = part1(&diagram);
        assert_eq!(result.unwrap(), 21);
    }

    #[test]
    fn test_example_part2_brute_force() {
        let parse = DAY.calc.parse;
        let diagram = parse(DAY.examples().second()).unwrap();
        let result = part2_brute_force(&diagram);
        assert_eq!(result.unwrap(), 40);
    }

    #[test]
    fn test_example_part2() {
        let parse = DAY.calc.parse;
        let diagram = parse(DAY.examples().second()).unwrap();
        let result = part2_counter(&diagram);
        assert_eq!(result.unwrap(), 40);
    }

    #[test]
    fn test_main() {
        let parse = DAY.calc.parse;
        let diagram = parse(&DAY.input()).unwrap();
        assert_eq!(part1(&diagram).answer.to_string(), "1546");
        assert_eq!(part2_counter(&diagram).answer.to_string(), "13883459503480");
    }
}
