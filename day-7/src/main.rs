use std::io;

use anyhow::{Result, bail, ensure};

fn main() -> Result<()> {
    let lines = io::stdin().lines().collect::<Result<Vec<_>, _>>()?;
    let mut rows = lines
        .iter()
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    ensure!(!rows.is_empty(), "diagram has no rows");
    ensure!(
        rows.windows(2).all(|row| row[0].len() == row[1].len()),
        "diagram rows don't have the same number of columns"
    );

    let mut count = 0;

    for i in 1..rows.len() {
        for j in 0..rows[i].len() {
            match rows[i - 1][j] {
                b'S' | b'|' => match rows[i][j] {
                    b'.' => rows[i][j] = b'|',
                    b'^' => {
                        if j > 0 {
                            rows[i][j - 1] = b'|';
                        }
                        if j < rows[i].len() - 1 {
                            rows[i][j + 1] = b'|';
                        }

                        count += 1;
                    }
                    b'S' | b'|' => (),
                    _ => bail!("invalid character: {:?}", rows[i][j] as char),
                },
                b'.' | b'^' => (),
                _ => bail!("invalid character: {:?}", rows[i - 1][j] as char),
            }
        }
    }

    println!("{count}");
    Ok(())
}
