use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    ops::RangeInclusive,
    str::FromStr,
};

use crate::{Day, DayCalc, Examples, ParseError, ParseResult, PartOutput};

type IngredientId = usize;

#[derive(Debug, Clone)]
pub struct Database {
    fresh_ranges: Vec<RangeInclusive<IngredientId>>,
    available: Vec<IngredientId>,
}

impl FromStr for Database {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        log::trace!("Parsing database:\n{s}");
        let lines: Vec<_> = s.lines().collect();
        let mut splits = lines.split(|l| l.is_empty());
        let fresh_ranges = splits
            .next()
            .unwrap()
            .into_iter()
            .map(|line| -> ParseResult<_> {
                log::trace!("Parsing range line: {line}");
                let (start, end) = line.split_once('-').unwrap();
                let start = start.parse::<IngredientId>()?;
                let end = end.parse::<IngredientId>()?;
                Ok(start..=end)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let available = splits
            .next()
            .unwrap()
            .iter()
            .map(|l| l.parse::<IngredientId>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Database {
            fresh_ranges,
            available,
        })
    }
}

impl Display for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Database:")?;
        for range in self.fresh_ranges.iter() {
            writeln!(f, "  {range:?}")?;
        }
        Ok(())
    }
}

pub fn part1(db: &Database) -> PartOutput<usize> {
    PartOutput {
        answer: db
            .available
            .iter()
            .filter(|ingredient| {
                db.fresh_ranges
                    .iter()
                    .any(|range| range.contains(ingredient))
            })
            .count(),
    }
}

#[allow(unused)]
pub fn part2_brute_force(db: &Database) -> PartOutput<usize> {
    let fresh: HashSet<IngredientId> = db
        .fresh_ranges
        .iter()
        .flat_map(|range| range.clone())
        .collect();
    PartOutput {
        answer: fresh.len(),
    }
}

#[allow(unused)]
pub fn part2_counter(db: &Database) -> PartOutput<usize> {
    let total_min = db
        .fresh_ranges
        .iter()
        .map(|range| range.start())
        .min()
        .unwrap();
    let total_max = db
        .fresh_ranges
        .iter()
        .map(|range| range.end())
        .max()
        .unwrap();

    let mut starts = HashMap::<IngredientId, HashSet<usize>>::new();
    let mut ends = HashMap::<IngredientId, HashSet<usize>>::new();
    for (idx, range) in db.fresh_ranges.iter().enumerate() {
        starts.entry(*range.start()).or_default().insert(idx);
        ends.entry(*range.end()).or_default().insert(idx);
    }

    let mut counter = 0;
    let mut active_range_indices = HashSet::<usize>::new();
    for ingredient_id in *total_min..=*total_max {
        if let Some(indices) = starts.get(&ingredient_id) {
            active_range_indices.extend(indices);
        };
        log::trace!("{}: {:?}", ingredient_id, active_range_indices);
        if active_range_indices.len() > 0 {
            counter += 1;
        }
        if let Some(indices) = ends.get(&ingredient_id) {
            active_range_indices.retain(|idx| !indices.contains(idx));
        }
    }
    PartOutput { answer: counter }
}

pub fn part2(db: &Database) -> PartOutput<usize> {
    let total_min = db
        .fresh_ranges
        .iter()
        .map(|range| range.start())
        .min()
        .unwrap();
    let total_max = db
        .fresh_ranges
        .iter()
        .map(|range| range.end())
        .max()
        .unwrap();

    let mut counter = 0;
    let mut ingredient_id = *total_min;
    loop {
        if let Some(max_end) = db
            .fresh_ranges
            .iter()
            .filter_map(|range| range.contains(&ingredient_id).then(|| range.end()))
            .max()
        {
            let next_ingredient_id = max_end.checked_add(1).unwrap();
            counter += next_ingredient_id.checked_sub(ingredient_id).unwrap();
            ingredient_id = next_ingredient_id;
            if ingredient_id > *total_max {
                break;
            }
        } else {
            if let Some(next_range) = db
                .fresh_ranges
                .iter()
                .filter(|range| range.start() > &ingredient_id)
                .min_by_key(|range| range.start())
            {
                ingredient_id = *next_range.start();
            } else {
                break;
            }
        }
    }

    PartOutput { answer: counter }
}

pub const DAY: Day<Database, usize, 1, 0, 0> = Day {
    day: 5,
    title: "Cafeteria",
    display: (
        "{answer} available ingredient IDs are fresh.",
        "{answer} ingredient IDs are considered to be fresh according to the fresh ingredient ID ranges",
    ),
    calc: DayCalc {
        parse: Database::from_str,
        part1,
        part2,
    },
    examples: Examples::single(include_str!("../../examples/day05.txt")),
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
        let db = parse(DAY.examples().first()).unwrap();
        let result = part1(&db);
        assert_eq!(result.unwrap(), 3);
    }

    #[test]
    fn test_example_part2_brute_force() {
        let parse = DAY.calc.parse;
        let db = parse(DAY.examples().second()).unwrap();
        let result = part2_brute_force(&db);
        assert_eq!(result.unwrap(), 14);
    }

    #[test]
    fn test_example_part2_counter() {
        let parse = DAY.calc.parse;
        let db = parse(DAY.examples().second()).unwrap();
        let result = part2_counter(&db);
        assert_eq!(result.unwrap(), 14);
    }

    #[test]
    fn test_example_part2() {
        let parse = DAY.calc.parse;
        let db = parse(DAY.examples().second()).unwrap();
        let result = part2(&db);
        assert_eq!(result.unwrap(), 14);
    }

    #[test]
    fn test_main() {
        let parse = DAY.calc.parse;
        let db = parse(&DAY.input()).unwrap();
        assert_eq!(part1(&db).answer.to_string(), "828");
        assert_eq!(part2(&db).answer.to_string(), "352681648086146");
    }
}
