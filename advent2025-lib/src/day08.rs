use std::{collections::HashMap, fmt::Display, num::ParseIntError, str::FromStr};

use crate::{Day, DayCalc, Examples, ParseError, PartOutput};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JunctionBox([usize; 3]);

impl JunctionBox {
    /// Euclidean integer distance.
    fn idistance(&self, other: &Self) -> usize {
        self.0
            .iter()
            .zip(other.0.iter())
            .map(|(a, b)| a.abs_diff(*b).pow(2))
            .sum::<usize>()
            .isqrt()
    }
    /// Euclidean integer distance.
    #[allow(unused)]
    fn distance(&self, other: &Self) -> f64 {
        (self
            .0
            .iter()
            .zip(other.0.iter())
            .map(|(a, b)| a.abs_diff(*b).pow(2))
            .sum::<usize>() as f64)
            .sqrt()
    }
}

impl FromStr for JunctionBox {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coordinates: Vec<usize> = s
            .split(',')
            .map(|pos| pos.parse())
            .collect::<Result<_, ParseIntError>>()?;
        Ok(JunctionBox([
            coordinates[0],
            coordinates[1],
            coordinates[2],
        ]))
    }
}

impl Display for JunctionBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

#[derive(Debug, Clone)]
pub struct JunctionBoxes {
    boxes: Vec<JunctionBox>,
}

impl JunctionBoxes {}

impl FromStr for JunctionBoxes {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(JunctionBoxes {
            boxes: s
                .lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JunctionBoxTree {
    root: JunctionBox,
    nodes: Vec<JunctionBoxTree>,
}

impl JunctionBoxTree {
    pub fn new(node: JunctionBox) -> Self {
        Self {
            root: node,
            nodes: Vec::new(),
        }
    }
    pub fn contains(&self, node: &JunctionBox) -> bool {
        self.root == *node || self.nodes.iter().any(|n| n.contains(node))
    }
    pub fn len(&self) -> usize {
        self.nodes.iter().map(|n| n.len()).sum::<usize>() + 1
    }
}

pub fn distance_matrix(
    diagram: &JunctionBoxes,
    connection_pairs_max: Option<usize>,
) -> (Vec<JunctionBoxTree>, Option<[&JunctionBox; 2]>) {
    let distance_matrix: HashMap<[&JunctionBox; 2], usize> = diagram
        .boxes
        .iter()
        .flat_map(|first| {
            diagram
                .boxes
                .iter()
                .filter_map(|second| {
                    if first == second {
                        None
                    } else {
                        let pair = if first.0 < second.0 {
                            [first, second]
                        } else {
                            [second, first]
                        };
                        Some((pair, first.idistance(second)))
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();
    let pairs_sorted_by_distance = {
        let mut pairs_sorted_by_distance: Vec<(usize, [&JunctionBox; 2])> = distance_matrix
            .iter()
            .map(|(pair, distance)| (*distance, *pair))
            .collect::<Vec<_>>();
        pairs_sorted_by_distance.sort_by_key(|(distance, _)| *distance);
        pairs_sorted_by_distance
    };
    for (distance, pair) in pairs_sorted_by_distance.iter().take(16) {
        log::debug!("{} to {} is a distance of {}", pair[0], pair[1], distance);
    }
    let mut trees: Vec<JunctionBoxTree> = vec![];
    let mut count_connection_pairs = 0;
    let mut last_pair = None;
    for (_distance, pair) in &pairs_sorted_by_distance {
        // Both in a tree
        if let (Some(first_in_tree_idx), Some(second_in_tree_idx)) = (
            trees.iter().position(|tree| tree.contains(pair[0])),
            trees.iter().position(|tree| tree.contains(pair[1])),
        ) {
            if first_in_tree_idx == second_in_tree_idx {
                log::info!(
                    "Nodes {} and {} are already in the same tree.",
                    pair[0],
                    pair[1]
                );
                count_connection_pairs += 1;
                continue;
            }
            let second_tree = trees.remove(second_in_tree_idx);
            let first_tree = trees
                .iter_mut()
                .find(|tree| tree.contains(pair[0]))
                .unwrap();
            log::info!(
                "Merging trees {}({}) and {}({}).",
                &first_tree.root,
                pair[0],
                &second_tree.root,
                pair[1]
            );
            first_tree.nodes.push(second_tree);
            count_connection_pairs += 1;
        }
        // One in a tree
        else if let Some(tree) = trees.iter_mut().find(|tree| tree.contains(pair[0])) {
            tree.nodes.push(JunctionBoxTree::new(pair[1].to_owned()));
            log::info!("Adding {} to tree {}.", pair[1], pair[0]);
            count_connection_pairs += 1;
        } else if let Some(tree) = trees.iter_mut().find(|tree| tree.contains(pair[1])) {
            tree.nodes.push(JunctionBoxTree::new(pair[0].to_owned()));
            log::info!("Adding {} to tree {}.", pair[0], pair[1]);
            count_connection_pairs += 1;
        }
        // Niether in a tree
        else if trees
            .iter()
            .all(|tree| !tree.contains(pair[0]) && !tree.contains(pair[1]))
        {
            let mut new_tree = JunctionBoxTree::new(pair[0].to_owned());
            new_tree
                .nodes
                .push(JunctionBoxTree::new(pair[1].to_owned()));
            trees.push(new_tree);
            log::info!("Creating new tree with {} and {}.", pair[0], pair[1]);
            count_connection_pairs += 1;
        }
        if let Some(connection_pairs_max) = connection_pairs_max
            && count_connection_pairs >= connection_pairs_max
        {
            log::info!("Max connection pairs reached.");
            break;
        }
        if trees.first().map(|t| t.len()).unwrap_or_default() >= diagram.boxes.len() {
            log::info!("All boxes connected.");
            last_pair = Some(pair);
            break;
        }
    }
    trees.sort_by_key(|t| usize::MAX - t.len());
    (trees, last_pair.copied())
}

pub fn distance_matrix_tree(
    diagram: &JunctionBoxes,
    connection_pairs_max: usize,
) -> Vec<JunctionBoxTree> {
    distance_matrix(diagram, Some(connection_pairs_max)).0
}

pub fn distance_matrix_last_connection(
    diagram: &JunctionBoxes,
) -> (JunctionBoxTree, [&JunctionBox; 2]) {
    let (trees, pair) = distance_matrix(diagram, None);
    assert_eq!(trees.len(), 1);
    let tree = trees.into_iter().next().unwrap();
    log::info!(
        "Number of boxes: {}, Tree length: {}",
        diagram.boxes.len(),
        tree.len()
    );
    (tree, pair.unwrap())
}

pub fn part1(diagram: &JunctionBoxes) -> PartOutput<usize> {
    let trees = distance_matrix_tree(diagram, 1000);
    PartOutput {
        answer: trees.iter().take(3).map(|t| t.len()).product(),
    }
}

pub fn part2(diagram: &JunctionBoxes) -> PartOutput<usize> {
    let (tree, last_connection) = distance_matrix_last_connection(diagram);
    log::info!(
        "Number of boxes: {}, Tree length: {}",
        diagram.boxes.len(),
        tree.len()
    );
    PartOutput {
        answer: last_connection[0].0[0]
            .checked_mul(last_connection[1].0[0])
            .unwrap(),
    }
}

pub const DAY: Day<JunctionBoxes, usize, 1, 0, 0> = Day {
    day: 8,
    title: "Playground  ",
    display: (
        "Product of the sizes of the three largest circuits: {answer}",
        "Product of the X coordinates of the last two junction boxes you need to connect: {answer}",
    ),
    calc: DayCalc {
        parse: JunctionBoxes::from_str,
        part1,
        part2,
    },
    examples: Examples::single(include_str!("../../examples/day08.txt")),
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
        let trees = distance_matrix_tree(&diagram, 10);
        log::info!("Trees: {:#?}", trees);
        assert_eq!(trees.iter().take(3).map(|t| t.len()).product::<usize>(), 40);
    }

    #[test]
    fn test_example_part2() {
        let parse = DAY.calc.parse;
        let diagram = parse(DAY.examples().second()).unwrap();
        let result = part2(&diagram);
        assert_eq!(result.unwrap(), 25272);
    }

    #[test]
    fn test_main() {
        let parse = DAY.calc.parse;
        let diagram = parse(&DAY.input()).unwrap();
        assert_eq!(part1(&diagram).answer.to_string(), "171503");
        assert_eq!(part2(&diagram).answer.to_string(), "9069509600");
    }
}
