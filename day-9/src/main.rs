use std::io;

use anyhow::{Result, ensure};

#[derive(Debug)]
struct Location {
    pub x: u64,
    pub y: u64,
}

fn parse_locations(lines: &[String]) -> Result<Vec<Location>> {
    lines.iter().map(|line| parse_location(line)).collect()
}

fn parse_location(line: &str) -> Result<Location> {
    let coords = line
        .split(',')
        .map(|x| x.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?;

    ensure!(
        coords.len() == 2,
        "location doesn't have 2 coordinates: {line:?}"
    );

    let location = Location {
        x: coords[0],
        y: coords[1],
    };

    Ok(location)
}

fn main() -> Result<()> {
    let lines = io::stdin().lines().collect::<Result<Vec<_>, _>>()?;
    let locations = parse_locations(&lines)?;

    // There are definitely some optimizations possible, but I can't see any
    // that fundamentally changes the O(n^2) nature of the problem. So let's
    // just brute-force our way through...

    let mut max_area = 0;

    for i in 0..locations.len() - 1 {
        for j in 0..i {
            let Location { x: x1, y: y1 } = locations[i];
            let Location { x: x2, y: y2 } = locations[j];

            let area = (x2.abs_diff(x1) + 1) * (y2.abs_diff(y1) + 1);

            if area > max_area {
                max_area = area;
            }
        }
    }

    println!("{max_area}");
    Ok(())
}
