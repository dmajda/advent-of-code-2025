use std::collections::HashMap;
use std::io;
use std::sync::LazyLock;

use anyhow::{Result, bail};
use regex::Regex;

fn count_paths(devices: &HashMap<String, Vec<String>>, start: &str, end: &str) -> u32 {
    do_count_paths(devices, &mut HashMap::new(), start, end)
}

fn do_count_paths(
    devices: &HashMap<String, Vec<String>>,
    counts: &mut HashMap<String, u32>,
    start: &str,
    end: &str,
) -> u32 {
    let Some(outputs) = devices.get(start) else {
        return 0;
    };

    if let Some(&count) = counts.get(start) {
        return count;
    }

    let count = outputs
        .iter()
        .map(|output| {
            if output == end {
                return 1;
            }

            do_count_paths(devices, counts, output, end)
        })
        .sum();

    counts.insert(start.to_owned(), count);
    count
}

static DEVICE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\w+:(\s+\w+)+$").unwrap());

fn parse_devices(lines: &[String]) -> Result<HashMap<String, Vec<String>>> {
    let mut devices = HashMap::new();

    for line in lines {
        if !DEVICE_RE.is_match(line) {
            bail!("machine description has invalid format: {:?}", line);
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

    let count = count_paths(&devices, "you", "out");

    println!("{count}");
    Ok(())
}
