use std::error::Error;
use std::fmt;
use std::{io, process};

// In principle, the dial position and rotation distances should be unsigned.
// However, the rotation logic is best expressed using signed arithmetics. To
// avoid many type casts, I just use `i32` everywhere instead of more correct
// `u32`.

const N: i32 = 100;

pub struct Dial {
    pos: i32,
    zero_count: u32,
}

impl Dial {
    pub fn new() -> Dial {
        Dial {
            pos: 50,
            zero_count: 0,
        }
    }

    pub fn pos(&self) -> i32 {
        self.pos
    }

    pub fn zero_count(&self) -> u32 {
        self.zero_count
    }

    pub fn rotate_left(&mut self, n: i32) {
        self.pos = (self.pos - n).rem_euclid(N);

        if self.pos == 0 {
            self.zero_count += 1
        }
    }

    pub fn rotate_right(&mut self, n: i32) {
        self.pos = (self.pos + n).rem_euclid(N);

        if self.pos == 0 {
            self.zero_count += 1
        }
    }
}

enum Rotation {
    Left(i32),
    Right(i32),
}

#[derive(Debug)]
enum ParseLineError {
    MissingInstruction,
    InvalidDirection(char),
    InvalidDistance(String),
}

impl fmt::Display for ParseLineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseLineError::MissingInstruction => {
                write!(f, "missing instruction")
            }
            ParseLineError::InvalidDirection(ch) => {
                write!(f, "invalid direction: {ch:?}")
            }
            ParseLineError::InvalidDistance(s) => {
                write!(f, "invalid distance: {s:?}")
            }
        }
    }
}

impl Error for ParseLineError {}

fn parse_line(line: &str) -> Result<Rotation, ParseLineError> {
    if line.is_empty() {
        return Err(ParseLineError::MissingInstruction);
    }

    let (direction, distance) = line.split_at(1);
    let direction = direction.chars().next().unwrap();

    match direction {
        'L' => {
            let distance = distance
                .parse::<i32>()
                .map_err(|_| ParseLineError::InvalidDistance(distance.to_owned()))?;

            Ok(Rotation::Left(distance))
        }

        'R' => {
            let distance = distance
                .parse::<i32>()
                .map_err(|_| ParseLineError::InvalidDistance(distance.to_owned()))?;

            Ok(Rotation::Right(distance))
        }

        _ => Err(ParseLineError::InvalidDirection(direction)),
    }
}

fn main() {
    let mut dial = Dial::new();

    for line in io::stdin().lines() {
        let line = line.unwrap_or_else(|e| {
            eprintln!("{e}");
            process::exit(1);
        });

        let rotation = parse_line(&line).unwrap_or_else(|e| {
            eprintln!("{e}");
            process::exit(1);
        });

        match rotation {
            Rotation::Left(n) => dial.rotate_left(n),
            Rotation::Right(n) => dial.rotate_right(n),
        }
    }

    println!("{}", dial.zero_count());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dial_works() {
        let mut dial = Dial::new();
        assert_eq!(dial.pos(), 50);
        assert_eq!(dial.zero_count(), 0);

        dial.rotate_left(68);
        assert_eq!(dial.pos(), 82);
        assert_eq!(dial.zero_count(), 0);

        dial.rotate_left(30);
        assert_eq!(dial.pos(), 52);
        assert_eq!(dial.zero_count(), 0);

        dial.rotate_right(48);
        assert_eq!(dial.pos(), 0);
        assert_eq!(dial.zero_count(), 1);

        dial.rotate_left(5);
        assert_eq!(dial.pos(), 95);
        assert_eq!(dial.zero_count(), 1);

        dial.rotate_right(60);
        assert_eq!(dial.pos(), 55);
        assert_eq!(dial.zero_count(), 1);

        dial.rotate_left(55);
        assert_eq!(dial.pos(), 0);
        assert_eq!(dial.zero_count(), 2);

        dial.rotate_left(1);
        assert_eq!(dial.pos(), 99);
        assert_eq!(dial.zero_count(), 2);

        dial.rotate_left(99);
        assert_eq!(dial.pos(), 0);
        assert_eq!(dial.zero_count(), 3);

        dial.rotate_right(14);
        assert_eq!(dial.pos(), 14);
        assert_eq!(dial.zero_count(), 3);

        dial.rotate_left(82);
        assert_eq!(dial.pos(), 32);
        assert_eq!(dial.zero_count(), 3);
    }
}
