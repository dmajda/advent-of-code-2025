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

    let mut numbers_1 = vec![0; lines.len() - 1];
    let mut numbers_2 = vec![];
    let mut op = Op::Add;

    let mut results_1 = vec![];
    let mut results_2 = vec![];

    for col in cols {
        if col.iter().all(|&b| b == b' ') {
            results_1.push(compute_problem(&numbers_1, op));
            results_2.push(compute_problem(&numbers_2, op));

            numbers_1.fill(0);
            numbers_2.clear();

            continue;
        }

        let mut number_2 = 0;

        for i in 0..col.len() - 1 {
            let b = col[i];
            match b {
                b'0'..=b'9' => {
                    let digit = (b - b'0') as u64;

                    numbers_1[i] = 10 * numbers_1[i] + digit;
                    number_2 = 10 * number_2 + digit;
                }
                b' ' => (),
                _ => bail!("invalid character: {:?}", b as char),
            }
        }

        numbers_2.push(number_2);

        let b = col[col.len() - 1];
        match b {
            b'+' => op = Op::Add,
            b'*' => op = Op::Mul,
            b' ' => (),
            _ => bail!("invalid character: {:?}", b as char),
        }
    }

    results_1.push(compute_problem(&numbers_1, op));
    results_2.push(compute_problem(&numbers_2, op));

    let total_1 = results_1.into_iter().sum::<u64>();
    let total_2 = results_2.into_iter().sum::<u64>();

    println!("{total_1}");
    println!("{total_2}");
    Ok(())
}
