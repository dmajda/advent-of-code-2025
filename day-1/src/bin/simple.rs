// Alternative solution to the day 1 puzzle. It uses the same algorithm as the
// original solution, but the code is written to be as simple as possible.
//
// This is part of a small experiment to see how a simple solution feels like
// and how it compares to a more structured solution (see `structured.rs`).

use std::io;

use anyhow::{Result, bail, ensure};

fn main() -> Result<()> {
    const N: u32 = 100;

    let mut pos = 50;
    let mut count_1 = 0;
    let mut count_2 = 0;

    for line in io::stdin().lines() {
        let line = line?;

        ensure!(!line.is_empty(), "rotation is empty");

        let mut chars = line.chars();
        let dir = chars.next().unwrap();
        let dist = chars.as_str().parse::<u32>()?;

        // The trick I use below is to convert a left rotation to a right one
        // using a symmetry along the vertical axis of the dial, which can be
        // expressed as:
        //
        //     pos <=> (N - pos) % N
        //
        // This approach simplifies the counting of zero crossings.
        //
        // There may be a simple way to count zero crossings for left rotations
        // even without using the symmetry, but I don't see it at the moment.

        match dir {
            'L' => {
                let raw_pos = (N - pos) % N + dist;

                pos = (N - raw_pos % N) % N;
                count_1 += if pos == 0 { 1 } else { 0 };
                count_2 += raw_pos / N;
            }

            'R' => {
                let raw_pos = pos + dist;

                pos = raw_pos % N;
                count_1 += if pos == 0 { 1 } else { 0 };
                count_2 += raw_pos / N;
            }

            _ => bail!("invalid direction: {dir:?}"),
        }
    }

    println!("{count_1}");
    println!("{count_2}");
    Ok(())
}
