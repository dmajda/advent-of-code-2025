use std::error::Error;
use std::fmt;
use std::{io, process};

const N: u32 = 100;

pub struct Dial {
    pos: u32,
    zero_count_1: u32,
    zero_count_2: u32,
}

impl Dial {
    pub fn new() -> Dial {
        Dial {
            pos: 50,
            zero_count_1: 0,
            zero_count_2: 0,
        }
    }

    pub fn pos(&self) -> u32 {
        self.pos
    }

    pub fn zero_count_1(&self) -> u32 {
        self.zero_count_1
    }

    pub fn zero_count_2(&self) -> u32 {
        self.zero_count_2
    }

    pub fn rotate_left(&mut self, n: u32) {
        // The trick I use here is to convert a left rotation to a right one
        // using a symmetry along the vertical axis of the dial, which can be
        // expressed as:
        //
        //     pos <=> (N - pos) % N
        //
        // This approach simplifies the counting of zero crossings.
        //
        // There may be a simple way to count zero crossings for left rotations
        // even without using the symmetry, but I don't see it at the moment.

        let (pos, zero_count_1, zero_count_2) = Dial::rotate(Dial::symmetrical_pos(self.pos), n);

        self.pos = Dial::symmetrical_pos(pos);
        self.zero_count_1 += zero_count_1;
        self.zero_count_2 += zero_count_2;
    }

    pub fn rotate_right(&mut self, n: u32) {
        let (pos, zero_count_1, zero_count_2) = Dial::rotate(self.pos, n);

        self.pos = pos;
        self.zero_count_1 += zero_count_1;
        self.zero_count_2 += zero_count_2;
    }

    fn symmetrical_pos(pos: u32) -> u32 {
        (N - pos) % N
    }

    fn rotate(pos: u32, n: u32) -> (u32, u32, u32) {
        let p = pos + n;

        let pos = p % N;
        let zero_count_1 = if pos == 0 { 1 } else { 0 };
        let zero_count_2 = p / N;

        (pos, zero_count_1, zero_count_2)
    }
}

enum Rotation {
    Left(u32),
    Right(u32),
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
                .parse::<u32>()
                .map_err(|_| ParseLineError::InvalidDistance(distance.to_owned()))?;

            Ok(Rotation::Left(distance))
        }

        'R' => {
            let distance = distance
                .parse::<u32>()
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

    println!("{}", dial.zero_count_1());
    println!("{}", dial.zero_count_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dial_works() {
        let mut dial = Dial::new();
        assert_eq!(dial.pos(), 50);
        assert_eq!(dial.zero_count_1(), 0);
        assert_eq!(dial.zero_count_2(), 0);

        dial.rotate_left(68);
        assert_eq!(dial.pos(), 82);
        assert_eq!(dial.zero_count_1(), 0);
        assert_eq!(dial.zero_count_2(), 1);

        dial.rotate_left(30);
        assert_eq!(dial.pos(), 52);
        assert_eq!(dial.zero_count_1(), 0);
        assert_eq!(dial.zero_count_2(), 1);

        dial.rotate_right(48);
        assert_eq!(dial.pos(), 0);
        assert_eq!(dial.zero_count_1(), 1);
        assert_eq!(dial.zero_count_2(), 2);

        dial.rotate_left(5);
        assert_eq!(dial.pos(), 95);
        assert_eq!(dial.zero_count_1(), 1);
        assert_eq!(dial.zero_count_2(), 2);

        dial.rotate_right(60);
        assert_eq!(dial.pos(), 55);
        assert_eq!(dial.zero_count_1(), 1);
        assert_eq!(dial.zero_count_2(), 3);

        dial.rotate_left(55);
        assert_eq!(dial.pos(), 0);
        assert_eq!(dial.zero_count_1(), 2);
        assert_eq!(dial.zero_count_2(), 4);

        dial.rotate_left(1);
        assert_eq!(dial.pos(), 99);
        assert_eq!(dial.zero_count_1(), 2);
        assert_eq!(dial.zero_count_2(), 4);

        dial.rotate_left(99);
        assert_eq!(dial.pos(), 0);
        assert_eq!(dial.zero_count_1(), 3);
        assert_eq!(dial.zero_count_2(), 5);

        dial.rotate_right(14);
        assert_eq!(dial.pos(), 14);
        assert_eq!(dial.zero_count_1(), 3);
        assert_eq!(dial.zero_count_2(), 5);

        dial.rotate_left(82);
        assert_eq!(dial.pos(), 32);
        assert_eq!(dial.zero_count_1(), 3);
        assert_eq!(dial.zero_count_2(), 6);
    }
}
