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
    cells: Vec<Vec<Cell>>,
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

    pub fn new(cells: Vec<Vec<Cell>>) -> Grid {
        let height = cells.len();
        let width = cells.first().unwrap().len();

        Grid {
            width,
            height,
            cells,
        }
    }

    pub fn accessible_roll_count(&self) -> usize {
        let mut count = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                if self.cells[y][x] == Cell::Roll && self.adjacent_roll_count(x, y) < 4 {
                    count += 1;
                }
            }
        }

        count
    }

    fn adjacent_roll_count(&self, x: usize, y: usize) -> usize {
        Grid::ADJACENT_OFFSETS
            .iter()
            .filter_map(|&(dx, dy)| self.get(x as isize + dx, y as isize + dy))
            .filter(|&cell| cell == Cell::Roll)
            .count()
    }

    fn get(&self, x: isize, y: isize) -> Option<Cell> {
        if x < 0 || x >= self.width as isize || y < 0 || y >= self.height as isize {
            return None;
        }

        Some(self.cells[y as usize][x as usize])
    }
}

fn parse_lines(lines: Vec<String>) -> Vec<Vec<Cell>> {
    lines.into_iter().map(parse_line).collect()
}

fn parse_line(line: String) -> Vec<Cell> {
    line.chars().map(parse_char).collect()
}

fn parse_char(ch: char) -> Cell {
    match ch {
        '.' => Cell::Empty,
        '@' => Cell::Roll,
        _ => unreachable!(),
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
            .all(|line| line.chars().all(|b| b == '.' || b == '@')),
        "grid cell contains an invalid character"
    );

    let grid = Grid::new(parse_lines(lines));

    println!("{}", grid.accessible_roll_count());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_count_accessible_rolls_works() {
        const E: Cell = Cell::Empty;
        const R: Cell = Cell::Roll;

        let cells = vec![
            vec![E, E, R, R, E, R, R, R, R, E],
            vec![R, R, R, E, R, E, R, E, R, R],
            vec![R, R, R, R, R, E, R, E, R, R],
            vec![R, E, R, R, R, R, E, E, R, E],
            vec![R, R, E, R, R, R, R, E, R, R],
            vec![E, R, R, R, R, R, R, R, E, R],
            vec![E, R, E, R, E, R, E, R, R, R],
            vec![R, E, R, R, R, E, R, R, R, R],
            vec![E, R, R, R, R, R, R, R, R, E],
            vec![R, E, R, E, R, R, R, E, R, E],
        ];
        let grid = Grid::new(cells);

        assert_eq!(grid.accessible_roll_count(), 13);
    }
}
