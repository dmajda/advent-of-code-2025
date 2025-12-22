use std::collections::HashMap;
use std::io;
use std::iter::Sum;
use std::ops::Add;
use std::sync::LazyLock;

use anyhow::{Result, bail};
use regex::Regex;

const DEVICE_OUT: &str = "out";
const DEVICE_DAC: &str = "dac";
const DEVICE_FFT: &str = "fft";

// The code below could be generalized to handle any number of "via" devices,
// but this wasn't needed to solve the puzzle, so I didn't do it.

#[derive(Copy, Clone, Debug)]
struct Counts {
    pub via_none: u64,
    pub via_dac: u64,
    pub via_fft: u64,
    pub via_dac_and_fft: u64,
}

impl Counts {
    pub fn zero() -> Self {
        Self {
            via_none: 0,
            via_dac: 0,
            via_fft: 0,
            via_dac_and_fft: 0,
        }
    }

    pub fn out() -> Self {
        Self {
            via_none: 1,
            via_dac: 0,
            via_fft: 0,
            via_dac_and_fft: 0,
        }
    }

    pub fn with_dac(self) -> Self {
        Self {
            via_none: 0,
            via_dac: self.via_none,
            via_fft: 0,
            via_dac_and_fft: self.via_fft,
        }
    }

    pub fn with_fft(self) -> Self {
        Self {
            via_none: 0,
            via_dac: 0,
            via_fft: self.via_none,
            via_dac_and_fft: self.via_dac,
        }
    }
}

impl Add for Counts {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Counts {
            via_none: self.via_none + rhs.via_none,
            via_dac: self.via_dac + rhs.via_dac,
            via_fft: self.via_fft + rhs.via_fft,
            via_dac_and_fft: self.via_dac_and_fft + rhs.via_dac_and_fft,
        }
    }
}

impl Sum for Counts {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Counts::zero(), |a, b| a + b)
    }
}

fn count_paths(devices: &HashMap<String, Vec<String>>, start: &str) -> Counts {
    do_count_paths(devices, &mut HashMap::new(), start)
}

fn do_count_paths(
    devices: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<String, Counts>,
    start: &str,
) -> Counts {
    let Some(outputs) = devices.get(start) else {
        return Counts::zero();
    };

    if let Some(&counts) = memo.get(start) {
        return counts;
    }

    let mut counts = outputs
        .iter()
        .map(|output| {
            if output == DEVICE_OUT {
                return Counts::out();
            }

            do_count_paths(devices, memo, output)
        })
        .sum::<Counts>();

    if start == DEVICE_DAC {
        counts = counts.with_dac();
    }

    if start == DEVICE_FFT {
        counts = counts.with_fft();
    }

    memo.insert(start.to_owned(), counts);
    counts
}

static DEVICE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\w+:(\s+\w+)+$").unwrap());

fn parse_devices(lines: &[String]) -> Result<HashMap<String, Vec<String>>> {
    let mut devices = HashMap::new();

    for line in lines {
        if !DEVICE_RE.is_match(line) {
            bail!("device description has invalid format: {:?}", line);
        };

        let (label_part, outputs_part) = line.split_once(':').unwrap();

        let label = label_part.to_owned();
        let outputs = outputs_part
            .split_whitespace()
            .map(|output| output.to_owned())
            .collect();

        devices.insert(label, outputs);
    }

    Ok(devices)
}

fn main() -> Result<()> {
    let lines = io::stdin().lines().collect::<Result<Vec<_>, _>>()?;
    let devices = parse_devices(&lines)?;

    let counts_1 = count_paths(&devices, "you");
    let counts_2 = count_paths(&devices, "svr");

    let count_1 =
        counts_1.via_none + counts_1.via_dac + counts_1.via_fft + counts_1.via_dac_and_fft;
    let count_2 = counts_2.via_dac_and_fft;

    println!("{count_1}");
    println!("{count_2}");
    Ok(())
}
