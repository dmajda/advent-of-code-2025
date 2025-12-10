use std::cmp::Ordering;
use std::fmt;
use std::io;
use std::mem;

use anyhow::{Result, ensure};

// `Box` is a commonly used type in Rust and `box` is a reserved word, so we use
// `JBox` and `jbox` as an abbreviation for "junction box".

#[derive(Debug)]
struct JBox {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}

impl JBox {
    pub fn distance(&self, other: &JBox) -> f64 {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        let dz = self.z.abs_diff(other.z);

        ((dx * dx + dy * dy + dz * dz) as f64).sqrt()
    }
}

impl fmt::Display for JBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

#[derive(Debug)]
struct Dist {
    pub dist: f64,
    pub jbox_index_1: usize,
    pub jbox_index_2: usize,
}

// We need to implement `Ord` for `Dist` so we can use it in a binary heap. The
// problem is that `f64` used for the `dist` field only implements `PartialOrd`.
// We deal with this by:
//
//   1. Using `f64::total_cmp` on `dist` to implement `Ord` and `PartialOrd`.
//
//   2. Defining `PartialEq` and `Eq` in terms of bit equivalence on `dist`
//      (which makes them consistent with `Ord` and `PartialOrd`).
//
// For our purposes, where there are no `NaN`s and other exotic values, this
// approach works fine.

impl Ord for Dist {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.total_cmp(&other.dist)
    }
}

impl PartialOrd for Dist {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Dist {
    fn eq(&self, other: &Self) -> bool {
        self.dist.to_bits() == other.dist.to_bits()
    }
}

impl Eq for Dist {}

struct Playground {
    #[cfg(debug_assertions)]
    jboxes: Vec<JBox>,
    circuits: Vec<Vec<usize>>,
    index: Vec<usize>,
    dists: Vec<Dist>,
}

impl Playground {
    pub fn new(jboxes: Vec<JBox>) -> Playground {
        let circuits = (0..jboxes.len()).map(|i| vec![i]).collect();
        let index = (0..jboxes.len()).collect();

        let dists = Self::compute_dists(&jboxes);

        Playground {
            #[cfg(debug_assertions)]
            jboxes,
            circuits,
            index,
            dists,
        }
    }

    fn compute_dists(jboxes: &[JBox]) -> Vec<Dist> {
        // If there are `n` distinct juntion boxes, then there are `n * (n - 1)
        // / 2` mutual distances (we ignore the zero distance each box has to
        // itself). We compute them and return them sorted from the longest to
        // the shortest.

        let mut dists = Vec::with_capacity(jboxes.len() * (jboxes.len() - 1) / 2);

        for i in 0..jboxes.len() {
            for j in 0..i {
                let dist = Dist {
                    dist: jboxes[i].distance(&jboxes[j]),
                    jbox_index_1: j,
                    jbox_index_2: i,
                };

                dists.push(dist);
            }
        }

        dists.sort();
        dists.reverse();
        dists
    }

    pub fn dists(&self) -> &Vec<Dist> {
        &self.dists
    }

    pub fn connect_jboxes(&mut self, k: usize) -> Vec<usize> {
        for _ in 0..k {
            self.connect_closest();
        }

        self.circuit_sizes()
    }

    pub fn connect_closest(&mut self) {
        // Get the two closest boxes.
        let dist = self.dists.pop().unwrap();

        // Get indices of circuits the two boxes belong to.
        let circuit_index_1 = self.index[dist.jbox_index_1];
        let circuit_index_2 = self.index[dist.jbox_index_2];

        // If the circuits are the same, there is nothing to do.
        if circuit_index_1 == circuit_index_2 {
            #[cfg(debug_assertions)]
            println!(
                "{}-{} ({:.2}): already connected to circuit {}",
                self.jboxes[dist.jbox_index_1],
                self.jboxes[dist.jbox_index_2],
                dist.dist,
                circuit_index_1
            );

            return;
        }

        // First, update the index.
        let circuit_2 = &self.circuits[circuit_index_2];
        for &jbox_index in circuit_2 {
            self.index[jbox_index] = circuit_index_1;
        }

        // Now merge the circuits.
        let circuit_2 = &mut mem::take(&mut self.circuits[circuit_index_2]);
        let circuit_1 = &mut self.circuits[circuit_index_1];
        circuit_1.append(circuit_2);

        #[cfg(debug_assertions)]
        println!(
            "{}-{} ({:.2}): merged circuit {} into circuit {}",
            self.jboxes[dist.jbox_index_1],
            self.jboxes[dist.jbox_index_2],
            dist.dist,
            circuit_index_2,
            circuit_index_1
        );
    }

    fn circuit_sizes(&self) -> Vec<usize> {
        self.circuits
            .iter()
            .filter_map(|circuit| {
                if !circuit.is_empty() {
                    Some(circuit.len())
                } else {
                    None
                }
            })
            .collect()
    }
}

fn parse_jboxes(lines: &[String]) -> Result<Vec<JBox>> {
    lines.iter().map(|line| parse_jbox(line)).collect()
}

fn parse_jbox(line: &str) -> Result<JBox> {
    let coords = line
        .split(',')
        .map(|x| x.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?;

    ensure!(
        coords.len() == 3,
        "junction box doesn't have 3 coordinates: {line:?}"
    );

    let jbox = JBox {
        x: coords[0],
        y: coords[1],
        z: coords[2],
    };

    Ok(jbox)
}

const SHORTEST_CONNECTION_MIN: usize = 1000;
const TOP_CIRCUIT_MIN: usize = 3;

fn main() -> Result<()> {
    let lines = io::stdin().lines().collect::<Result<Vec<_>, _>>()?;
    let jboxes = parse_jboxes(&lines)?;

    let mut playground = Playground::new(jboxes);

    ensure!(
        playground.dists().len() >= SHORTEST_CONNECTION_MIN,
        "playground doesn't have enough possible connections"
    );

    let mut circuit_sizes = playground.connect_jboxes(SHORTEST_CONNECTION_MIN);
    circuit_sizes.sort();
    circuit_sizes.reverse();

    ensure!(
        circuit_sizes.len() >= TOP_CIRCUIT_MIN,
        "playground doesn't have enough circuits after connecting the boxes"
    );

    let result = circuit_sizes[..TOP_CIRCUIT_MIN].iter().product::<usize>();

    println!("{:?}", result);
    Ok(())
}
