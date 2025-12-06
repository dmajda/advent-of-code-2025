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

#[derive(Copy, Clone, Debug)]
enum Op {
    Add,
    Mul,
}

fn compute_problem(numbers: &[u64], op: Op) -> u64 {
    match op {
        Op::Add => numbers.into_iter().sum(),
        Op::Mul => numbers.into_iter().product(),
    }
}

fn main() -> Result<()> {
    let lines = io::stdin().lines().collect::<Result<Vec<_>, _>>()?;
    let bytes = lines.iter().map(|line| line.bytes()).collect::<Vec<_>>();
    let cols = ManyZip::new(bytes);

    let mut numbers = vec![0; lines.len() - 1];
    let mut op = Op::Add;

    let mut results = vec![];

    for col in cols {
        if col.iter().all(|&b| b == b' ') {
            results.push(compute_problem(&numbers, op));
            numbers.fill(0);

            continue;
        }

        for i in 0..col.len() - 1 {
            let b = col[i];
            match b {
                b'0'..=b'9' => numbers[i] = 10 * numbers[i] + (b - b'0') as u64,
                b' ' => (),
                _ => bail!("invalid character: {}", b as char),
            }
        }

        let b = col[col.len() - 1];
        match b {
            b'+' => op = Op::Add,
            b'*' => op = Op::Mul,
            b' ' => (),
            _ => bail!("invalid character: {}", b as char),
        }
    }

    results.push(compute_problem(&numbers, op));

    let total = results.into_iter().sum::<u64>();

    println!("{total}");
    Ok(())
}
