// I used the following visualization on CodePen to help me see the shape of the
// problem.
//
// HTML
//
//     <canvas id="canvas" width="1000" height="1000"></canvas>
//
// JS
//
//     const SCALE = 100;
//
//     const tiles = [
//       // ...
//     ];
//
//     const canvas = document.getElementById("canvas");
//     const ctx = canvas.getContext("2d");
//
//     ctx.beginPath();
//
//     ctx.moveTo(tiles[0].x / SCALE, tiles[0].y / SCALE);
//     for (let tile of tiles.slice(1)) {
//       ctx.lineTo(tile.x / SCALE, tile.y / SCALE);
//     }
//     ctx.lineTo(tiles[0].x / SCALE, tiles[0].y / SCALE);
//
//     ctx.closePath();
//     ctx.stroke();
//

use std::collections::{HashMap, HashSet};
use std::io;
use std::iter::Peekable;

use anyhow::{Result, ensure};

#[derive(Debug)]
struct Tile {
    pub x: u64,
    pub y: u64,
}

#[derive(Copy, Clone, Debug)]
struct Span {
    pub start: u64,
    pub end: u64,
}

impl Span {
    pub fn covers(&self, edge: &Span) -> bool {
        // Don't assume anything about the edge orientation.
        edge.start >= self.start
            && edge.start <= self.end
            && edge.end >= self.start
            && edge.end <= self.end
    }
}

enum Axis {
    X,
    Y,
}

// The following few functions need to work both in horizontal and vertical
// direction. To avoid using direction-specific terminology, we talk about
// *primary* and *secondary* coordinates instead of *x* and *y*.

fn group_tiles_by_axis(tiles: &[Tile], axis: Axis) -> HashMap<u64, HashSet<u64>> {
    let mut grouped_tiles = HashMap::new();

    for tile in tiles {
        let (primary, secondary) = match axis {
            Axis::X => (tile.x, tile.y),
            Axis::Y => (tile.y, tile.x),
        };

        grouped_tiles
            .entry(primary)
            .or_insert_with(HashSet::new)
            .insert(secondary);
    }

    grouped_tiles
}

fn compute_spans(grouped_tiles: &HashMap<u64, HashSet<u64>>) -> Result<HashMap<u64, Vec<Span>>> {
    let mut spans = HashMap::new();

    let mut primaries = grouped_tiles.keys().copied().collect::<Vec<_>>();
    primaries.sort_unstable();

    let mut prev_flips = HashSet::new();
    let mut prev_spans = vec![];

    for primary in primaries {
        let next_flips = prev_flips
            .symmetric_difference(&grouped_tiles.get(&primary).unwrap())
            .copied()
            .collect::<HashSet<_>>();
        let next_spans = flips_to_spans(&next_flips)?;

        spans.insert(primary, merge_spans(&prev_spans, &next_spans));

        prev_flips = next_flips;
        prev_spans = next_spans;
    }

    Ok(spans)
}

fn flips_to_spans(flips: &HashSet<u64>) -> Result<Vec<Span>> {
    let mut flips = flips.iter().copied().collect::<Vec<_>>();
    flips.sort_unstable();

    ensure!(
        flips.len().is_multiple_of(2),
        "floor doesn't contain an even number of red tiles in every row/column"
    );

    let spans = flips
        .chunks(2)
        .map(|chunk| Span {
            start: chunk[0],
            end: chunk[1],
        })
        .collect();

    Ok(spans)
}

fn merge_spans(spans_1: &[Span], spans_2: &[Span]) -> Vec<Span> {
    let mut merged_spans = vec![];

    let mut spans_1 = spans_1.iter().copied().peekable();
    let mut spans_2 = spans_2.iter().copied().peekable();

    let Some(mut current) = next_span(&mut spans_1, &mut spans_2) else {
        return vec![];
    };

    loop {
        let Some(next) = next_span(&mut spans_1, &mut spans_2) else {
            break;
        };

        if next.start <= current.end + 1 {
            current.end = current.end.max(next.end);
        } else {
            merged_spans.push(current);
            current = next;
        }
    }

    merged_spans.push(current);

    merged_spans
}

fn next_span(
    spans_1: &mut Peekable<impl Iterator<Item = Span>>,
    spans_2: &mut Peekable<impl Iterator<Item = Span>>,
) -> Option<Span> {
    match (spans_1.peek(), spans_2.peek()) {
        (Some(span_1), Some(span_2)) => {
            if span_1.start < span_2.start {
                spans_1.next()
            } else {
                spans_2.next()
            }
        }
        (Some(_), None) => spans_1.next(),
        (None, Some(_)) => spans_2.next(),
        (None, None) => None,
    }
}

fn parse_tiles(lines: &[String]) -> Result<Vec<Tile>> {
    lines.iter().map(|line| parse_tile(line)).collect()
}

fn parse_tile(line: &str) -> Result<Tile> {
    let coords = line
        .split(',')
        .map(|x| x.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?;

    ensure!(
        coords.len() == 2,
        "tile doesn't have 2 coordinates: {line:?}"
    );

    let tile = Tile {
        x: coords[0],
        y: coords[1],
    };

    Ok(tile)
}

fn main() -> Result<()> {
    let lines = io::stdin().lines().collect::<Result<Vec<_>, _>>()?;
    let tiles = parse_tiles(&lines)?;

    ensure!(
        tiles.len() >= 4,
        "floor doesn't have have at least 4 red tiles"
    );

    let tiles_by_rows = group_tiles_by_axis(&tiles, Axis::Y);
    let tiles_by_cols = group_tiles_by_axis(&tiles, Axis::X);
    let h_spans = compute_spans(&tiles_by_rows)?;
    let v_spans = compute_spans(&tiles_by_cols)?;

    let mut max_area_1 = 0;
    let mut max_area_2 = 0;

    for i in 0..tiles.len() - 1 {
        for j in 0..i {
            let Tile { x: x1, y: y1 } = tiles[i];
            let Tile { x: x2, y: y2 } = tiles[j];

            let area = (x2.abs_diff(x1) + 1) * (y2.abs_diff(y1) + 1);

            if area > max_area_1 {
                max_area_1 = area;
            }

            if area > max_area_2 {
                let h_edge = Span { start: x1, end: x2 };
                let v_edge = Span { start: y1, end: y2 };

                let h_edge_1_covered = h_spans[&y1].iter().any(|span| span.covers(&h_edge));
                let h_edge_2_covered = h_spans[&y2].iter().any(|span| span.covers(&h_edge));
                let v_edge_1_covered = v_spans[&x1].iter().any(|span| span.covers(&v_edge));
                let v_edge_2_covered = v_spans[&x2].iter().any(|span| span.covers(&v_edge));

                if h_edge_1_covered && h_edge_2_covered && v_edge_1_covered && v_edge_2_covered {
                    max_area_2 = area;
                }
            }
        }
    }

    println!("{max_area_1}");
    println!("{max_area_2}");
    Ok(())
}
