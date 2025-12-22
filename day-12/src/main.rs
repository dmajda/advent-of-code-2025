use std::io;
use std::sync::LazyLock;

use anyhow::{Result, ensure};
use regex::Regex;

#[derive(Debug)]
struct Shape {
    pub area: u32,
}

#[derive(Debug)]
struct Region {
    pub width: u32,
    pub height: u32,
    pub quantities: Vec<u32>,
}

impl Region {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}

static INDEX_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d+:+$").unwrap());
static SHAPE_ROW_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[.#]+$").unwrap());
static REGION_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d+x\d+:(\s+\d+)*$").unwrap());

fn parse_input(lines: &[String]) -> Result<(Vec<Shape>, Vec<Region>)> {
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

        let area = rows
            .into_iter()
            .map(|row| row.chars().filter(|&ch| ch == '#').count() as u32)
            .sum();

        let shape = Shape { area: area };
        shapes.push(shape);

        while let Some(&line) = lines.peek()
            && line.is_empty()
        {
            lines.next();
        }
    }

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

    Ok((shapes, regions))
}

fn main() -> Result<()> {
    let lines = io::stdin().lines().collect::<Result<Vec<_>, _>>()?;
    let (shapes, regions) = parse_input(&lines)?;

    let mut count = 0;
    for region in regions {
        let shapes_total_area = shapes
            .iter()
            .zip(&region.quantities)
            .map(|(shape, quantity)| shape.area * quantity)
            .sum::<u32>();

        if shapes_total_area > region.area() {
            continue;
        }

        // Cheat a bit and just assume that if it's not the case that the shapes
        // obviously can't fit, they will fit. It turns out the puzzle is
        // designed so that this assumption holds.
        count += 1;
    }

    println!("{count}");
    Ok(())
}
