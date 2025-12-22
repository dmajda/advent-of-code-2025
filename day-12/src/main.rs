use std::io;
use std::sync::LazyLock;

use anyhow::{Result, bail, ensure};
use regex::Regex;

#[derive(Debug)]
struct Shape {
    pub width: u32,
    pub height: u32,
    pub area: u32,
}

#[derive(Debug)]
struct Region {
    pub width: u32,
    pub height: u32,
    pub quantities: Vec<u32>,
}

#[derive(Debug)]
struct Problem {
    pub shapes: Vec<Shape>,
    pub shape_width: u32,
    pub shape_height: u32,
    pub regions: Vec<Region>,
}

impl Region {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn total_quantity(&self) -> u32 {
        self.quantities.iter().sum()
    }
}

static INDEX_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d+:+$").unwrap());
static SHAPE_ROW_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[.#]+$").unwrap());
static REGION_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d+x\d+:(\s+\d+)*$").unwrap());

fn parse_problem(lines: &[String]) -> Result<Problem> {
    let mut shapes = vec![];
    let mut regions = vec![];

    let mut lines = lines.iter().peekable();

    while let Some(&line) = lines.peek()
        && INDEX_RE.is_match(line)
    {
        lines.next();

        let index = line[0..line.len() - 1].parse::<usize>()?;

        ensure!(index == shapes.len(), "index has invalid value: {line:?}");

        let mut rows = vec![];

        while let Some(&line) = lines.peek()
            && !line.is_empty()
        {
            lines.next();

            ensure!(
                SHAPE_ROW_RE.is_match(line),
                "shape row has invalid format: {line:?}"
            );

            rows.push(line);
        }

        ensure!(!rows.is_empty(), "shape doesn't have any rows");
        ensure!(
            rows.windows(2).all(|row| row[0].len() == row[1].len()),
            "shape rows don't have the same number of columns"
        );

        let width = rows[0].len() as u32;
        let height = rows.len() as u32;
        let area = rows
            .into_iter()
            .map(|row| row.chars().filter(|&ch| ch == '#').count() as u32)
            .sum();

        let shape = Shape {
            width,
            height,
            area,
        };

        shapes.push(shape);

        while let Some(&line) = lines.peek()
            && line.is_empty()
        {
            lines.next();
        }
    }

    ensure!(!shapes.is_empty(), "problem doesn't have any shapes");

    let shape_widths = shapes.iter().map(|shape| shape.width).collect::<Vec<_>>();
    let shape_heights = shapes.iter().map(|shape| shape.height).collect::<Vec<_>>();

    ensure!(
        shape_widths.windows(2).all(|width| width[0] == width[1]),
        "shapes don't have the same width"
    );
    ensure!(
        shape_heights
            .windows(2)
            .all(|height| height[0] == height[1]),
        "shapes don't have the same height"
    );

    let shape_width = shape_widths[0];
    let shape_height = shape_heights[0];

    while let Some(&line) = lines.peek() {
        lines.next();

        ensure!(
            REGION_RE.is_match(line),
            "region description has invalid format: {line:?}"
        );

        let (dims, quantities) = line.split_once(':').unwrap();
        let (width, height) = dims.split_once('x').unwrap();

        let width = width.parse()?;
        let height = height.parse()?;
        let quantities = quantities
            .split_whitespace()
            .map(|quantity| quantity.parse())
            .collect::<Result<Vec<_>, _>>()?;

        ensure!(
            quantities.len() == shapes.len(),
            "region quantities don't correspond to shapes: {line:?}"
        );

        let region = Region {
            width,
            height,
            quantities,
        };

        regions.push(region);
    }

    let problem = Problem {
        shapes,
        shape_width,
        shape_height,
        regions,
    };

    Ok(problem)
}

fn main() -> Result<()> {
    let lines = io::stdin().lines().collect::<Result<Vec<_>, _>>()?;
    let problem = parse_problem(&lines)?;

    let mut count = 0;
    for region in problem.regions {
        let shapes_total_area = problem
            .shapes
            .iter()
            .zip(&region.quantities)
            .map(|(shape, quantity)| shape.area * quantity)
            .sum::<u32>();

        if shapes_total_area > region.area() {
            continue;
        }

        let simple_quantity =
            (region.width / problem.shape_width) * (region.height / problem.shape_height);
        if simple_quantity >= region.total_quantity() {
            count += 1;
        } else {
            bail!("problem is NP-complete");
        }
    }

    println!("{count}");
    Ok(())
}
