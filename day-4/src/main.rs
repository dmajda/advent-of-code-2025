use std::io;

use anyhow::{Result, ensure};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Cell {
    Empty,
    Roll,
}

struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
    adjacent_roll_counts: Vec<usize>,
}

impl Grid {
    const ADJACENT_OFFSETS: [(isize, isize); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    pub fn new(width: usize, height: usize) -> Grid {
        Grid {
            width,
            height,
            cells: vec![Cell::Empty; width * height],
            adjacent_roll_counts: vec![0; width * height],
        }
    }

    pub fn add_roll(&mut self, x: usize, y: usize) {
        assert!(x < self.width);
        assert!(y < self.height);
        assert!(self.cell(x, y) == Cell::Empty);

        self.set_cell(x, y, Cell::Roll);
        self.update_adjacent_roll_counts(x, y, 1);
    }

    pub fn accessible_roll_count(&self) -> usize {
        let mut count = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                if self.cell(x, y) == Cell::Roll && self.adjacent_roll_count(x, y) < 4 {
                    count += 1;
                }
            }
        }

        count
    }

    fn cell(&self, x: usize, y: usize) -> Cell {
        self.cells[y * self.height + x]
    }

    fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        self.cells[y * self.height + x] = cell
    }

    fn adjacent_roll_count(&self, x: usize, y: usize) -> usize {
        self.adjacent_roll_counts[y * self.height + x]
    }

    fn set_adjacent_roll_count(&mut self, x: usize, y: usize, count: usize) {
        self.adjacent_roll_counts[y * self.height + x] = count
    }

    fn update_adjacent_roll_counts(&mut self, x: usize, y: usize, delta: isize) {
        for (x, y) in self.adjacent_cell_coords(x, y) {
            let count = (self.adjacent_roll_count(x, y) as isize + delta) as usize;

            self.set_adjacent_roll_count(x, y, count);
        }
    }

    fn adjacent_cell_coords(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        Grid::ADJACENT_OFFSETS
            .iter()
            .map(move |&(dx, dy)| (x as isize + dx, y as isize + dy))
            .filter(move |&(x, y)| {
                x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize
            })
            .map(|(x, y)| (x as usize, y as usize))
            .collect()
    }
}

fn main() -> Result<()> {
    let lines = io::stdin().lines().collect::<Result<Vec<_>, _>>()?;

    ensure!(!lines.is_empty(), "grid has no rows");
    ensure!(
        lines.iter().all(|line| !line.is_empty()),
        "grid row has no columns"
    );
    ensure!(
        lines.windows(2).all(|line| line[0].len() == line[1].len()),
        "grid rows don't have the same number of columns"
    );
    ensure!(
        lines
            .iter()
            .all(|line| line.chars().all(|ch| ch == '.' || ch == '@')),
        "grid cell contains an invalid character"
    );

    let mut grid = Grid::new(lines.first().unwrap().chars().count(), lines.len());

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '@' {
                grid.add_roll(x, y);
            }
        }
    }

    println!("{}", grid.accessible_roll_count());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_count_accessible_rolls_works() {
        let lines = vec![
            "..@@.@@@@.",
            "@@@.@.@.@@",
            "@@@@@.@.@@",
            "@.@@@@..@.",
            "@@.@@@@.@@",
            ".@@@@@@@.@",
            ".@.@.@.@@@",
            "@.@@@.@@@@",
            ".@@@@@@@@.",
            "@.@.@@@.@.",
        ];

        let mut grid = Grid::new(10, 10);

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '@' {
                    grid.add_roll(x, y);
                }
            }
        }

        assert_eq!(grid.accessible_roll_count(), 13);
    }
}
