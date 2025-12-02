#[cfg(test)]
mod tests {
    use test_log::test;

    use crate::{Part, PrimaryExample, get_days};

    const EXAMPLE_ANSWERS: [[&'static str; 2]; 1] = [["3", "6"], []];

    #[test]
    fn test_days_examples() {
        let days = get_days();
        for (day_num, day) in days {
            let (part1, part2) = match day.examples() {
                PrimaryExample::Same(example) => day.both(example).unwrap(),
                PrimaryExample::Different([first, second]) => (
                    day.calc(Part::First, first).unwrap(),
                    day.calc(Part::Second, second).unwrap(),
                ),
            };
            let expected = EXAMPLE_ANSWERS.get(day_num - 1).unwrap();
            let expected_part1 = expected[0];
            let expected_part2 = expected[1];
            assert_eq!(
                part1, expected_part1,
                "day {day_num} part 1 example mismatch"
            );
            assert_eq!(
                part2, expected_part2,
                "day {day_num} part 2 example mismatch"
            )
        }
    }
}
