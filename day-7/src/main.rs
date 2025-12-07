use std::io;

use anyhow::{Result, bail, ensure};

fn main() -> Result<()> {
    let lines = io::stdin().lines().collect::<Result<Vec<_>, _>>()?;
    let rows = lines
        .iter()
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    ensure!(!rows.is_empty(), "diagram has no rows");
    ensure!(
        rows.windows(2).all(|row| row[0].len() == row[1].len()),
        "diagram rows don't have the same number of columns"
    );

    let mut prev_beams = vec![0; rows[0].len()];
    let mut next_beams = vec![0; rows[0].len()];

    let mut count_1 = 0;

    for i in 0..rows.len() {
        for j in 0..rows[i].len() {
            match rows[i][j] {
                b'S' => next_beams[j] += 1,
                b'.' => next_beams[j] += prev_beams[j],
                b'^' => {
                    if j > 0 {
                        next_beams[j - 1] += prev_beams[j];
                    }
                    if j < next_beams.len() - 1 {
                        next_beams[j + 1] += prev_beams[j];
                    }

                    if prev_beams[j] > 0 {
                        count_1 += 1;
                    }
                }
                _ => bail!("invalid character: {:?}", rows[i][j] as char),
            }
        }

        prev_beams.copy_from_slice(&next_beams);
        next_beams.fill(0);
    }

    let count_2 = prev_beams.into_iter().sum::<u64>();

    println!("{count_1}");
    println!("{count_2}");
    Ok(())
}
