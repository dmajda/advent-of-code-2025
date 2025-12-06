use std::io;

use anyhow::{Result, bail};

pub struct ManyZip<I> {
    iters: Vec<I>,
}

impl<I> ManyZip<I>
where
    I: Iterator,
{
    pub fn new(iters: Vec<I>) -> Self {
        Self { iters }
    }
}

impl<I> Iterator for ManyZip<I>
where
    I: Iterator,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iters.is_empty() {
            return None;
        }

        self.iters.iter_mut().map(|it| it.next()).collect()
    }
}

fn main() -> Result<()> {
    let lines = io::stdin().lines().collect::<Result<Vec<_>, _>>()?;
    let cells = lines
        .iter()
        .map(|line| line.split_whitespace())
        .collect::<Vec<_>>();
    let cols = ManyZip::new(cells);

    let total = cols
        .map(|col| {
            let numbers = col[..col.len() - 1]
                .iter()
                .map(|cell| cell.parse::<u64>())
                .collect::<Result<Vec<_>, _>>()?;
            let op = col[col.len() - 1];

            let result = match op {
                "+" => numbers.into_iter().sum::<u64>(),
                "*" => numbers.into_iter().product::<u64>(),
                _ => bail!("invalid operation: {op:?}"),
            };

            Ok(result)
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sum::<u64>();

    println!("{total}");
    Ok(())
}
