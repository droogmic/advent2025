use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    str::FromStr,
};

use crate::{Day, DayCalc, Examples, ParseError, PartOutput};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RowColPos {
    row: usize,
    col: usize,
}

impl RowColPos {
    fn area(&self, other: &RowColPos) -> usize {
        self.row.abs_diff(other.row).checked_add(1).unwrap()
            * self.col.abs_diff(other.col).checked_add(1).unwrap()
    }
    fn neighbours(&self) -> [RowColPos; 4] {
        [
            RowColPos {
                row: self.row,
                col: self.col.checked_add(1).unwrap(),
            },
            RowColPos {
                row: self.row,
                col: self.col.checked_sub(1).unwrap(),
            },
            RowColPos {
                row: self.row.checked_add(1).unwrap(),
                col: self.col,
            },
            RowColPos {
                row: self.row.checked_sub(1).unwrap(),
                col: self.col,
            },
        ]
    }
}

impl FromStr for RowColPos {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (col, row) = s.split_once(',').ok_or(ParseError::Empty)?;
        Ok(Self {
            row: row.parse()?,
            col: col.parse()?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GridManifest {
    red_tiles: Vec<RowColPos>,
}

impl FromStr for GridManifest {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<_>, _>>()
            .map(|red_tiles| Self { red_tiles })
    }
}

pub fn part1(grid: &GridManifest) -> PartOutput<usize> {
    log::debug!("Finding the largest area of any rectangle...");
    PartOutput {
        answer: grid
            .red_tiles
            .iter()
            .flat_map(|first| {
                grid.red_tiles.iter().filter_map(|second| {
                    if first.row <= second.row {
                        Some(first.area(second))
                    } else {
                        None
                    }
                })
            })
            .max()
            .unwrap_or(0),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Red,
    Green,
}

#[derive(Debug, Clone, Default)]
struct Grid(HashMap<RowColPos, Tile>);

impl Grid {
    fn draw_edge(&mut self, first: &RowColPos, second: &RowColPos) {
        self.0.insert(*first, Tile::Red);
        self.0.insert(*second, Tile::Red);
        if first.row == second.row {
            for col in first.col.min(second.col).checked_add(1).unwrap()..first.col.max(second.col)
            {
                assert!(
                    self.0
                        .insert(
                            RowColPos {
                                row: first.row,
                                col,
                            },
                            Tile::Green,
                        )
                        .is_none()
                );
            }
        } else if first.col == second.col {
            for row in first.row.min(second.row).checked_add(1).unwrap()..first.row.max(second.row)
            {
                assert!(
                    self.0
                        .insert(
                            RowColPos {
                                row,
                                col: first.col,
                            },
                            Tile::Green,
                        )
                        .is_none()
                );
            }
        } else {
            panic!("Cannot draw edge between {:?} and {:?}", first, second);
        }
    }
    fn flood_fill_slow(&mut self, midpoint: &RowColPos) {
        log::info!("Flood fill from {:?}", midpoint);
        let mut tsunami: HashSet<RowColPos> = HashSet::from([midpoint.to_owned()]);
        loop {
            // Green the tsunami
            for pos in &tsunami {
                self.0.insert(pos.to_owned(), Tile::Green);
            }
            // Extend the tsunami
            let old_tsunami: Vec<RowColPos> = tsunami.drain().collect();
            for pos in old_tsunami {
                for neighbour in pos.neighbours() {
                    if self.0.get(&neighbour).is_none() {
                        tsunami.insert(neighbour);
                    }
                }
            }
            if tsunami.is_empty() {
                break;
            }
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut min_row = usize::MAX;
        let mut max_row = usize::MIN;
        let mut min_col = usize::MAX;
        let mut max_col = usize::MIN;
        for pos in self.0.keys() {
            min_row = min_row.min(pos.row);
            max_row = max_row.max(pos.row);
            min_col = min_col.min(pos.col);
            max_col = max_col.max(pos.col);
        }
        for row in min_row..=max_row {
            for col in min_col..=max_col {
                let pos = RowColPos { row, col };
                let tile_char = self
                    .0
                    .get(&pos)
                    .map(|t| match t {
                        Tile::Green => 'X',
                        Tile::Red => '#',
                    })
                    .unwrap_or('.');
                write!(f, "{}", tile_char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[allow(unused)]
fn part2_flood_fill(grid_manifest: &GridManifest) -> PartOutput<usize> {
    let mut grid = Grid::default();
    for pair in grid_manifest.red_tiles.windows(2) {
        log::trace!("Grid:\n{}", grid);
        grid.draw_edge(&pair[0], &pair[1]);
    }
    grid.draw_edge(
        &grid_manifest.red_tiles.last().unwrap(),
        &grid_manifest.red_tiles.first().unwrap(),
    );
    log::trace!("Grid:\n{}", grid);
    let min_col = grid_manifest.red_tiles.iter().map(|t| t.col).min().unwrap();
    let mid_row = grid_manifest
        .red_tiles
        .iter()
        .map(|t| t.row)
        .min()
        .unwrap()
        .abs_diff(grid_manifest.red_tiles.iter().map(|t| t.row).max().unwrap())
        .checked_div(2)
        .unwrap();
    let mut col = min_col;
    let midpoint = loop {
        log::info!("Looking for midpoint at row {}, col {}", mid_row, col);
        if grid.0.contains_key(&RowColPos { row: mid_row, col }) {
            if !grid.0.contains_key(&RowColPos {
                row: mid_row,
                col: col + 1,
            }) {
                break RowColPos {
                    row: mid_row,
                    col: col + 1,
                };
            }
        }
        col += 1;
    };
    grid.flood_fill_slow(&midpoint);
    log::trace!("Grid:\n{}", grid);
    let mut possible_areas: Vec<(usize, [RowColPos; 2])> = grid_manifest
        .red_tiles
        .iter()
        .flat_map(|first| {
            grid_manifest.red_tiles.iter().filter_map(|second| {
                if first.row <= second.row {
                    Some((first.area(second), [first.to_owned(), second.to_owned()]))
                } else {
                    None
                }
            })
        })
        .collect();
    possible_areas.sort_unstable_by_key(|area| usize::MAX - area.0);
    for (area, [first, second]) in possible_areas {
        log::trace!(
            "Checking rectangle with area {} at {:?}",
            area,
            [first, second]
        );
        if (first.row.min(second.row)..=first.row.max(second.row)).all(|row| {
            (first.col.min(second.col)..=first.col.max(second.col)).all(|col| {
                log::trace!("Checking if grid contains {:?}", RowColPos { row, col });
                grid.0.contains_key(&RowColPos { row, col })
            })
        }) {
            log::info!(
                "Found rectangle with area {} at {:?}",
                area,
                [first, second]
            );
            return PartOutput { answer: area };
        }
    }
    panic!("No rectangle found");
}

enum Edge {
    Horizontal { row: usize, cols: [usize; 2] },
    Vertical { rows: [usize; 2], col: usize },
}

pub fn part2(grid_manifest: &GridManifest) -> PartOutput<usize> {
    log::debug!("Finding the largest area of any rectangle using only red and green tiles...");
    log::debug!("Drawing edges...");
    let edges: Vec<Edge> = {
        let mut edges = Vec::new();
        let mut add_pair = |first: &RowColPos, second: &RowColPos| {
            if first.row == second.row {
                edges.push(Edge::Horizontal {
                    row: first.row,
                    cols: [first.col.min(second.col), first.col.max(second.col)],
                });
            } else if first.col == second.col {
                edges.push(Edge::Vertical {
                    rows: [first.row.min(second.row), first.row.max(second.row)],
                    col: first.col,
                });
            } else {
                panic!("Edge calculation failed.")
            }
        };
        for pair in grid_manifest.red_tiles.windows(2) {
            add_pair(&pair[0], &pair[1]);
        }
        add_pair(
            &grid_manifest.red_tiles.last().unwrap(),
            &grid_manifest.red_tiles.first().unwrap(),
        );
        edges
    };
    log::debug!("Finding possible areas...");
    let mut possible_areas: Vec<(usize, [RowColPos; 2])> = grid_manifest
        .red_tiles
        .iter()
        .flat_map(|first| {
            grid_manifest.red_tiles.iter().filter_map(|second| {
                if first.row <= second.row {
                    Some((first.area(second), [first.to_owned(), second.to_owned()]))
                } else {
                    None
                }
            })
        })
        .collect();
    log::debug!("Sorting possible areas...");
    possible_areas.sort_unstable_by_key(|area| usize::MAX - area.0);
    log::debug!("Finding first valid possible areas...");
    for (area, [rect_1, rect_2]) in possible_areas {
        log::trace!(
            "Checking rectangle with area {} at {:?} to {:?}...",
            area,
            rect_1,
            rect_2,
        );
        // Invalid if any edge crosses into this rectangle
        let rect_top = rect_1.row.min(rect_2.row);
        let rect_bottom = rect_1.row.max(rect_2.row);
        let rect_left = rect_1.col.min(rect_2.col);
        let rect_right = rect_1.col.max(rect_2.col);
        let invalid_rectangle = edges.iter().any(|edge| match edge {
            Edge::Horizontal {
                row,
                cols: [left, right],
            } => {
                let invalid_horizontal = (*row > rect_top && *row < rect_bottom)
                    && ((*left <= rect_left && *right >= rect_left)
                        || (*left <= rect_right && *right >= rect_right));
                invalid_horizontal
            }
            Edge::Vertical {
                col,
                rows: [top, bottom],
            } => {
                let invalid_vertical = (*col > rect_left && *col < rect_right)
                    && ((*top <= rect_top && *bottom >= rect_top)
                        || (*top <= rect_bottom && *bottom >= rect_bottom));
                invalid_vertical
            }
        });
        if !invalid_rectangle {
            log::info!(
                "Found rectangle with area {} at {:?}",
                area,
                [rect_1, rect_2]
            );
            return PartOutput { answer: area };
        }
    }
    panic!("No rectangle found");
}

pub const DAY: Day<GridManifest, usize, 1, 0, 0> = Day {
    day: 9,
    title: "Movie Theater",
    display: (
        "The largest area of any rectangle is: {answer}",
        "The largest area of any rectangle using only red and green tiles is: {answer}",
    ),
    calc: DayCalc {
        parse: GridManifest::from_str,
        part1,
        part2,
    },
    examples: Examples::single(include_str!("../../examples/day09.txt")),
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
        let grid = parse(DAY.examples().first()).unwrap();
        let result = part1(&grid);
        assert_eq!(result.unwrap(), 50);
    }

    #[test]
    fn test_example_flood_fill() {
        let parse = DAY.calc.parse;
        let grid = parse(DAY.examples().second()).unwrap();
        let result = part2_flood_fill(&grid);
        assert_eq!(result.unwrap(), 24);
    }

    #[test]
    fn test_example_part2() {
        let parse = DAY.calc.parse;
        let grid = parse(DAY.examples().second()).unwrap();
        let result = part2(&grid);
        assert_eq!(result.unwrap(), 24);
    }

    #[test]
    fn test_main() {
        let parse = DAY.calc.parse;
        let grid = parse(&DAY.input()).unwrap();
        assert_eq!(part1(&grid).answer.to_string(), "4771508457");
        assert_eq!(part2(&grid).answer.to_string(), "1539809693");
    }
}
