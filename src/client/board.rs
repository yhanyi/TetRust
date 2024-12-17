pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 20;

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    Filled,
    Preview,
}

impl Cell {
    pub fn to_string(&self) -> &str {
        match self {
            Cell::Empty => "⬜",
            Cell::Filled => "⬛",
            Cell::Preview => "🟦",
            // Cell::Empty => "◻",
            // Cell::Filled => "◼",
            // Cell::Preview => "⛶",
        }
    }
}

#[derive(Clone)]
pub struct Board {
    cells: [[Cell; WIDTH]; HEIGHT],
}

impl Board {
    pub fn new() -> Self {
        Self {
            cells: [[Cell::Empty; WIDTH]; HEIGHT],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Cell {
        self.cells[y][x]
    }

    pub fn set(&mut self, x: usize, y: usize, cell: Cell) {
        self.cells[y][x] = cell;
    }

    pub fn clear_line(&mut self, y: usize) {
        for row in (1..=y).rev() {
            self.cells[row] = self.cells[row - 1];
        }
        self.cells[0] = [Cell::Empty; WIDTH];
    }
}
