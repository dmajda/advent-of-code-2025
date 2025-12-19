use std::collections::VecDeque;
use std::io;
use std::sync::LazyLock;

use anyhow::{Result, anyhow, bail, ensure};
use microlp::{OptimizationDirection, Problem};
use regex::Regex;

// I chose to represent indicator light configurations as well as button wirings
// as bit sets backed by `usize`. This means that in part 1:
//
//   1. Pressing a button can be expressed as XORing an indicator light
//      configuration by the button wiring.
//
//   2. The mapping from an indicator light configuration to a minimum number of
//      button presses required for it can be implemented using a simple vector
//      indexed by the indicator light configuration.
//
// The representation limits the number of indicator lights to `usize::BITS`,
// but this is not a problem as the machines have at most 10 indicator lights.

const INITIAL_LIGHTS: usize = 0;

#[derive(Debug)]
struct Machine {
    size: usize,
    lights: usize,
    wirings: Vec<usize>,
    joltages: Vec<u32>,
}

fn find_min_presses_1(machine: &Machine) -> Option<u32> {
    let mut min_presses = vec![u32::MAX; 1 << machine.size];
    let mut queue = VecDeque::new();

    min_presses[INITIAL_LIGHTS] = 0;
    queue.push_back(INITIAL_LIGHTS);

    while !queue.is_empty() {
        let lights = queue.pop_front().unwrap();
        let presses = min_presses[lights];

        if lights == machine.lights {
            return Some(presses);
        }

        let new_presses = presses + 1;

        for &wiring in &machine.wirings {
            let new_lights = lights ^ wiring;

            if min_presses[new_lights] <= new_presses {
                continue;
            }

            min_presses[new_lights] = new_presses;
            queue.push_back(new_lights);
        }
    }

    None
}

fn find_min_presses_2(machine: &Machine) -> Option<u32> {
    let mut problem = Problem::new(OptimizationDirection::Minimize);

    let vars = (0..machine.wirings.len())
        .map(|_| problem.add_integer_var(1.0, (0, i32::MAX)))
        .collect::<Vec<_>>();

    for i in 0..machine.size {
        let expr = machine
            .wirings
            .iter()
            .zip(&vars)
            .filter_map(|(&wiring, &var)| {
                if wiring & (1 << i) != 0 {
                    Some((var, 1.0))
                } else {
                    None
                }
            });

        problem.add_constraint(expr, microlp::ComparisonOp::Eq, machine.joltages[i] as f64);
    }

    let Ok(solution) = problem.solve() else {
        return None;
    };

    Some(solution.objective().round() as u32)
}

static MACHINE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?x)^
        \[[.\#]+\]          # indicator light diagram
        (
            \s+
            \(\d+(,\d+)*\)  # button wiring schematic
        )+
        \s+
        \{\d+(,\d+)*\}      # joltage requirements
        $",
    )
    .unwrap()
});

fn parse_machines(lines: &[String]) -> Result<Vec<Machine>> {
    lines.iter().map(|line| parse_machine(line)).collect()
}

fn parse_machine(line: &str) -> Result<Machine> {
    if !MACHINE_RE.is_match(line) {
        bail!("machine description has invalid format: {:?}", line);
    };

    let parts = line.split_whitespace().collect::<Vec<_>>();

    let (size, lights) = parse_lights(parts[0])?;
    let wirings = parts[1..parts.len() - 1]
        .iter()
        .map(|&part| parse_wiring(part, size))
        .collect::<Result<Vec<_>, _>>()?;
    let joltages = parse_joltages(parts[parts.len() - 1])?;

    if joltages.len() != size {
        bail!(
            "indicator light diagram and joltage requirements have a different size: {:?}",
            line
        );
    }

    let machine = Machine {
        size,
        lights,
        wirings,
        joltages,
    };

    Ok(machine)
}

fn parse_lights(part: &str) -> Result<(usize, usize)> {
    let size = part.len() - 2;

    ensure!(
        size <= usize::BITS as usize,
        "indicator light diagram contains too many lights",
    );

    let lights = part[1..part.len() - 1]
        .chars()
        .enumerate()
        .fold(0, |acc, (i, ch)| match ch {
            '.' => acc,
            '#' => acc | (1 << i),
            _ => unreachable!(),
        });

    Ok((size, lights))
}

fn parse_wiring(part: &str, size: usize) -> Result<usize> {
    let indices = part[1..part.len() - 1]
        .split(",")
        .map(|index| index.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    ensure!(
        indices.iter().all(|&index| index < size),
        "button wiring schematic contains invalid indices"
    );

    let wiring = indices.into_iter().fold(0, |acc, index| acc | (1 << index));

    Ok(wiring)
}

fn parse_joltages(part: &str) -> Result<Vec<u32>> {
    let joltages = part[1..part.len() - 1]
        .split(",")
        .map(|joltage| joltage.parse::<u32>().unwrap())
        .collect();

    Ok(joltages)
}

fn main() -> Result<()> {
    let lines = io::stdin().lines().collect::<Result<Vec<_>, _>>()?;
    let machines = parse_machines(&lines)?;

    let min_presses_1 = machines
        .iter()
        .map(|machine| find_min_presses_1(machine))
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| anyhow!("indicator lights can't be configured on some machines"))?;
    let min_presses_2 = machines
        .iter()
        .map(|machine| find_min_presses_2(machine))
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| anyhow!("joltage level counters can't be configured on some machines"))?;

    let total_min_presses_1 = min_presses_1.iter().sum::<u32>();
    let total_min_presses_2 = min_presses_2.iter().sum::<u32>();

    println!("{:?}", total_min_presses_1);
    println!("{:?}", total_min_presses_2);
    Ok(())
}
