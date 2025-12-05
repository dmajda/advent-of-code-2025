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

    pub fn remove_roll(&mut self, x: usize, y: usize) {
        assert!(x < self.width);
        assert!(y < self.height);
        assert!(self.cell(x, y) == Cell::Roll);

        self.set_cell(x, y, Cell::Empty);
        self.update_adjacent_roll_counts(x, y, -1);
    }

    pub fn accessible_roll_count(&self) -> usize {
        self.accessible_roll_coords().len()
    }

    pub fn remove_accessible_rolls(&mut self) -> usize {
        let coords = self.accessible_roll_coords();
        let coords_len = coords.len();

        for (x, y) in coords {
            self.remove_roll(x, y)
        }

        coords_len
    }

    fn cell(&self, x: usize, y: usize) -> Cell {
        self.cells[y * self.width + x]
    }

    fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        self.cells[y * self.width + x] = cell
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

    fn accessible_roll_coords(&self) -> Vec<(usize, usize)> {
        (0..self.height)
            .flat_map(move |y| {
                (0..self.width)
                    .filter(move |&x| {
                        self.cell(x, y) == Cell::Roll && self.adjacent_roll_count(x, y) < 4
                    })
                    .map(move |x| (x, y))
            })
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

    let count_1 = grid.accessible_roll_count();

    let mut count_2 = 0;
    while let c = grid.remove_accessible_rolls()
        && c > 0
    {
        count_2 += c;
    }

    println!("{}", count_1);
    println!("{}", count_2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_works() {
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

        assert_eq!(grid.remove_accessible_rolls(), 13);
        assert_eq!(grid.remove_accessible_rolls(), 12);
        assert_eq!(grid.remove_accessible_rolls(), 7);
        assert_eq!(grid.remove_accessible_rolls(), 5);
        assert_eq!(grid.remove_accessible_rolls(), 2);
        assert_eq!(grid.remove_accessible_rolls(), 1);
        assert_eq!(grid.remove_accessible_rolls(), 1);
        assert_eq!(grid.remove_accessible_rolls(), 1);
        assert_eq!(grid.remove_accessible_rolls(), 1);
        assert_eq!(grid.remove_accessible_rolls(), 0);
    }
}
