use std::collections::HashMap;
use std::str::FromStr;

const DELIMITERS: [&str; 6] = ["\n\n", "\n", ",", " ", ":", "-"];

pub fn read_vec1<T: FromStr>(input: &str) -> Result<Vec<T>, T::Err> {
    log::trace!("input: {input}");
    let found_delims: Vec<&str> = DELIMITERS
        .into_iter()
        .filter(|&delim| input.contains(delim))
        .collect();
    log::trace!("found delims: {found_delims:?}");
    let &first_delim = found_delims.first().unwrap();
    let list: Vec<&str> = input.split(first_delim).collect();
    log::trace!("parse delimited list");
    list.into_iter()
        .map(FromStr::from_str)
        .collect::<Result<Vec<T>, _>>()
}

pub fn read_vec2<T: FromStr>(input: &str) -> Result<Vec<Vec<T>>, T::Err> {
    log::trace!("input: {input}");
    let found_delims: Vec<&str> = DELIMITERS
        .into_iter()
        .filter(|&delim| input.contains(delim))
        .collect();
    log::trace!("found delims: {found_delims:?}");
    let &first_delim = found_delims.first().unwrap();
    let list: Vec<&str> = input.split(first_delim).collect();
    if let Some(&second_delim) = found_delims.get(1) {
        log::trace!("parse delimited list of delimited lists");
        list.into_iter()
            .map(|el| {
                el.split(second_delim)
                    .map(FromStr::from_str)
                    .collect::<Result<_, _>>()
            })
            .collect::<Result<Vec<Vec<T>>, _>>()
    } else {
        log::trace!("parse undelimited list of delimited lists");
        list.into_iter()
            .map(|el| {
                el.chars()
                    .map(|c| c.to_string())
                    .map(|c| c.parse())
                    .collect::<Result<_, _>>()
            })
            .collect::<Result<Vec<Vec<T>>, _>>()
    }
}

pub trait FromChar: Sized {
    type Err;

    fn from_char(c: char) -> Result<Self, Self::Err>;
}

pub fn read_map<T: FromChar>(input: &str) -> Result<HashMap<(usize, usize), T>, T::Err> {
    log::trace!("input: {input}");
    let mut map = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            // Add 1 to avoid trivial underflows
            let existing = map.insert((col + 1, row + 1), T::from_char(char)?);
            if existing.is_some() {
                panic!("unexpected key found");
            }
        }
    }
    Ok(map)
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use test_log::test;

    use super::*;

    #[test]
    fn test_example_day01() {
        let input = read_to_string("../examples/day01.txt").unwrap();
        assert_eq!(read_vec2::<char>(&input).unwrap()[0], vec!['L', '6', '8'],);
    }
}
