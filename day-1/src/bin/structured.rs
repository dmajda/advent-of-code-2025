// Alternative solution to the day 1 puzzle. It uses the same algorithm as the
// original solution, but the code is written to be nicely structured.
//
// This is part of a small experiment to see how a structured solution feels
// like and how it compares to a simpler solution (see `simple.rs`).

use std::io;
use std::str::FromStr;

use anyhow::{Error, Result, bail, ensure};

#[derive(Copy, Clone, Debug)]
enum Rotation {
    Left(u32),
    Right(u32),
}

impl FromStr for Rotation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ensure!(!s.is_empty(), "rotation is empty");

        let mut chars = s.chars();
        let dir = chars.next().unwrap();
        let dist = chars.as_str().parse::<u32>()?;

        let rotation = match dir {
            'L' => Rotation::Left(dist),
            'R' => Rotation::Right(dist),
            _ => bail!("invalid direction: {dir:?}"),
        };

        Ok(rotation)
    }
}

struct Dial {
    pos: u32,
    count_1: u32,
    count_2: u32,
}

impl Dial {
    const N: u32 = 100;

    pub fn new() -> Self {
        Self {
            pos: 50,
            count_1: 0,
            count_2: 0,
        }
    }

    pub fn count_1(&self) -> u32 {
        self.count_1
    }

    pub fn count_2(&self) -> u32 {
        self.count_2
    }

    pub fn rotate(&mut self, rotation: Rotation) {
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

        match rotation {
            Rotation::Left(dist) => {
                let raw_pos = (Self::N - self.pos) % Self::N + dist;

                self.pos = (Self::N - raw_pos % Self::N) % Self::N;
                self.count_1 += if self.pos == 0 { 1 } else { 0 };
                self.count_2 += raw_pos / Self::N;
            }

            Rotation::Right(dist) => {
                let raw_pos = self.pos + dist;

                self.pos = raw_pos % Self::N;
                self.count_1 += if self.pos == 0 { 1 } else { 0 };
                self.count_2 += raw_pos / Self::N;
            }
        }
    }
}

fn main() -> Result<()> {
    let mut dial = Dial::new();

    for line in io::stdin().lines() {
        dial.rotate(line?.parse()?);
    }

    println!("{}", dial.count_1());
    println!("{}", dial.count_2());
    Ok(())
}
