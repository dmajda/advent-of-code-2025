use std::io;
use std::ops::RangeInclusive;

use anyhow::{Result, anyhow};

struct FreshIngredients {
    // We maintain these invariants:
    //
    //   1. The ranges are non-empty.
    //   2. The ranges are non-overlapping.
    //   3. The ranges and sorted by their start/end (the first two invariants
    //      imply that sorting by either leads to the same ordering).
    //
    // We rely on these invariants heavily in the implementation.
    ranges: Vec<RangeInclusive<u64>>,
}

impl FreshIngredients {
    pub fn new() -> FreshIngredients {
        FreshIngredients { ranges: vec![] }
    }

    pub fn add_range(&mut self, range: RangeInclusive<u64>) {
        assert!(range.start() <= range.end());

        // Index of the first range the newly added range might overlap with, or
        // |self.ranges.len()| if there isn't any.
        let first = match self
            .ranges
            .binary_search_by_key(range.start(), |range| *range.end())
        {
            Ok(index) => index,
            Err(index) => index,
        };

        // Index of the range *after* the last range the newly added range might
        // overlap with, or |0| if there isn't any.
        let last = match self
            .ranges
            .binary_search_by_key(range.end(), |range| *range.start())
        {
            Ok(index) => index + 1,
            Err(index) => index,
        };

        debug_assert!(first <= last);

        // If there is no overlapping, insert the newly added range. Otherwise,
        // build a new range from newly added range and the ranges it overlaps
        // with, and replace them with it.
        if first == last {
            self.ranges.insert(first, range);
        } else {
            let start = *self.ranges[first].start().min(range.start());
            let end = *self.ranges[last - 1].end().max(range.end());

            self.ranges.splice(first..last, [start..=end]);
        }
    }

    pub fn is_fresh(&self, id: u64) -> bool {
        // Index of the first range the ID might fall into, or
        // |self.ranges.len()| if there isn't any.
        let first = match self.ranges.binary_search_by_key(&id, |range| *range.end()) {
            Ok(index) => index,
            Err(index) => index,
        };

        // Index of the range *after* the last range the ID might fall into, or
        // |0| if there isn't any.
        let last = match self
            .ranges
            .binary_search_by_key(&id, |range| *range.start())
        {
            Ok(index) => index + 1,
            Err(index) => index,
        };

        debug_assert!(first <= last);

        // See if the the ID falls into one of the ranges.
        first < last
    }
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
    let mut fresh_ingredients = FreshIngredients::new();

    for line in io::stdin().lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        fresh_ingredients.add_range(parse_fresh_id_range(line)?);
    }

    let mut count = 0;

    for line in io::stdin().lines() {
        let line = line?;

        if fresh_ingredients.is_fresh(parse_available_id(line)?) {
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
    fn fresh_ingredients_works() {
        let mut fresh_ingredients = FreshIngredients::new();
        fresh_ingredients.add_range(3..=5);
        fresh_ingredients.add_range(10..=14);
        fresh_ingredients.add_range(16..=20);
        fresh_ingredients.add_range(12..=18);

        assert_eq!(fresh_ingredients.is_fresh(1), false);
        assert_eq!(fresh_ingredients.is_fresh(5), true);
        assert_eq!(fresh_ingredients.is_fresh(8), false);
        assert_eq!(fresh_ingredients.is_fresh(11), true);
        assert_eq!(fresh_ingredients.is_fresh(17), true);
        assert_eq!(fresh_ingredients.is_fresh(32), false);
    }
}
