use std::io;
use std::ops::RangeInclusive;

use anyhow::{Result, anyhow};

fn is_fresh(fresh_id_ranges: &[RangeInclusive<u64>], id: u64) -> bool {
    fresh_id_ranges.iter().any(|range| range.contains(&id))
}

fn parse_fresh_id_range(line: String) -> Result<RangeInclusive<u64>> {
    let (start, end) = line
        .split_once('-')
        .ok_or_else(|| anyhow!("invalid fresh ingredient ID range: {line:?}"))?;

    let start = start
        .parse()
        .map_err(|_| anyhow!("invalid fresh ingredient ID range: {line:?}"))?;

    let end = end
        .parse()
        .map_err(|_| anyhow!("invalid fresh ingredient ID range: {line:?}"))?;

    Ok(start..=end)
}

fn parse_available_id(line: String) -> Result<u64> {
    line.parse()
        .map_err(|_| anyhow!("invalid available ingredient ID: {line:?}"))
}

fn main() -> Result<()> {
    let mut fresh_id_ranges = vec![];

    for line in io::stdin().lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        fresh_id_ranges.push(parse_fresh_id_range(line)?);
    }

    let mut count = 0;

    for line in io::stdin().lines() {
        let line = line?;

        if is_fresh(&fresh_id_ranges, parse_available_id(line)?) {
            count += 1;
        }
    }

    println!("{count}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_fresh_works() {
        let fresh_id_ranges = vec![3..=5, 10..=14, 16..=20, 12..=18];

        assert_eq!(is_fresh(&fresh_id_ranges, 1), false);
        assert_eq!(is_fresh(&fresh_id_ranges, 5), true);
        assert_eq!(is_fresh(&fresh_id_ranges, 8), false);
        assert_eq!(is_fresh(&fresh_id_ranges, 11), true);
        assert_eq!(is_fresh(&fresh_id_ranges, 17), true);
        assert_eq!(is_fresh(&fresh_id_ranges, 32), false);
    }
}
