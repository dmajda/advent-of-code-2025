use std::io;
use std::ops::RangeInclusive;

use anyhow::{Result, anyhow};

struct FreshIngredients {
    ranges: Vec<RangeInclusive<u64>>,
}

impl FreshIngredients {
    pub fn new() -> FreshIngredients {
        FreshIngredients { ranges: vec![] }
    }

    pub fn add_range(&mut self, range: RangeInclusive<u64>) {
        self.ranges.push(range);
    }

    pub fn is_fresh(&self, id: u64) -> bool {
        self.ranges.iter().any(|range| range.contains(&id))
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
