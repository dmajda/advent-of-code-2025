use std::error::Error;
use std::fmt;
use std::ops::RangeInclusive;
use std::vec::Vec;
use std::{io, process};

fn find_invalid_ids_1(range: RangeInclusive<u64>) -> Vec<u64> {
    range.filter(|&id| is_invalid_id_1(id)).collect()
}

fn is_invalid_id_1(id: u64) -> bool {
    let id_string = id.to_string();
    if !id_string.len().is_multiple_of(2) {
        return false;
    }

    let part_1 = &id_string[..id_string.len() / 2];
    let part_2 = &id_string[id_string.len() / 2..];

    part_1 == part_2
}

fn find_invalid_ids_2(range: RangeInclusive<u64>) -> Vec<u64> {
    range.filter(|&id| is_invalid_id_2(id)).collect()
}

fn is_invalid_id_2(id: u64) -> bool {
    let id_string = id.to_string();

    (1..=id_string.len() / 2).any(|i| is_repeated(&id_string, i))
}

fn is_repeated(id_string: &str, n: usize) -> bool {
    if !id_string.len().is_multiple_of(n) {
        return false;
    }

    let chunks = (0..id_string.len() / n)
        .map(|i| &id_string[i * n..(i + 1) * n])
        .collect::<Vec<_>>();

    chunks.windows(2).all(|window| window[0] == window[1])
}

#[derive(Debug)]
enum ParseInputError {
    InvalidRange(String),
}

impl fmt::Display for ParseInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseInputError::InvalidRange(s) => {
                write!(f, "invalid range: {s:?}")
            }
        }
    }
}

impl Error for ParseInputError {}

fn parse_input(input: &str) -> Result<Vec<RangeInclusive<u64>>, ParseInputError> {
    input
        .trim()
        .split(',')
        .map(|range| {
            let (start, end) = range
                .split_once("-")
                .ok_or_else(|| ParseInputError::InvalidRange(range.to_owned()))?;

            let start: u64 = start
                .parse::<u64>()
                .map_err(|_| ParseInputError::InvalidRange(range.to_owned()))?;

            let end = end
                .parse::<u64>()
                .map_err(|_| ParseInputError::InvalidRange(range.to_owned()))?;

            Ok(start..=end)
        })
        .collect()
}

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    });

    let ranges = parse_input(&input).unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    });

    let sum_1 = ranges
        .iter()
        .flat_map(|range| find_invalid_ids_1(range.clone()))
        .sum::<u64>();
    let sum_2 = ranges
        .iter()
        .flat_map(|range| find_invalid_ids_2(range.clone()))
        .sum::<u64>();

    println!("{sum_1}");
    println!("{sum_2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_invalid_ids_1_works() {
        assert_eq!(find_invalid_ids_1(11..=22), vec![11, 22]);
        assert_eq!(find_invalid_ids_1(95..=115), vec![99]);
        assert_eq!(find_invalid_ids_1(998..=1012), vec![1010]);
        assert_eq!(
            find_invalid_ids_1(1188511880..=1188511890),
            vec![1188511885]
        );
        assert_eq!(find_invalid_ids_1(222220..=222224), vec![222222]);
        assert_eq!(find_invalid_ids_1(1698522..=1698528), vec![]);
        assert_eq!(find_invalid_ids_1(446443..=446449), vec![446446]);
        assert_eq!(find_invalid_ids_1(38593856..=38593862), vec![38593859]);
        assert_eq!(find_invalid_ids_1(565653..=565659), vec![]);
        assert_eq!(find_invalid_ids_1(824824821..=824824827), vec![]);
        assert_eq!(find_invalid_ids_1(2121212118..=2121212124), vec![]);
    }

    #[test]
    fn find_invalid_ids_2_works() {
        assert_eq!(find_invalid_ids_2(11..=22), vec![11, 22]);
        assert_eq!(find_invalid_ids_2(95..=115), vec![99, 111]);
        assert_eq!(find_invalid_ids_2(998..=1012), vec![999, 1010]);
        assert_eq!(
            find_invalid_ids_2(1188511880..=1188511890),
            vec![1188511885]
        );
        assert_eq!(find_invalid_ids_2(222220..=222224), vec![222222]);
        assert_eq!(find_invalid_ids_2(1698522..=1698528), vec![]);
        assert_eq!(find_invalid_ids_2(446443..=446449), vec![446446]);
        assert_eq!(find_invalid_ids_2(38593856..=38593862), vec![38593859]);
        assert_eq!(find_invalid_ids_2(565653..=565659), vec![565656]);
        assert_eq!(find_invalid_ids_2(824824821..=824824827), vec![824824824]);
        assert_eq!(
            find_invalid_ids_2(2121212118..=2121212124),
            vec![2121212121]
        );
    }
}
