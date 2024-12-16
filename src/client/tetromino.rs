#[derive(Clone, Copy)]
pub enum TetrominoType {
    I,
    O,
    T,
    L,
    J,
    S,
    Z,
}

pub struct Tetromino {
    pub tetromino_type: TetrominoType,
    pub cells: [[bool; 4]; 4],
}

impl Tetromino {
    pub fn new(tetromino_type: TetrominoType) -> Self {
        let cells = match tetromino_type {
            TetrominoType::I => [
                [false, false, false, false],
                [true, true, true, true],
                [false, false, false, false],
                [false, false, false, false],
            ],
            TetrominoType::O => [
                [false, true, true, false],
                [false, true, true, false],
                [false, false, false, false],
                [false, false, false, false],
            ],
            TetrominoType::T => [
                [false, false, false, false],
                [false, true, false, false],
                [true, true, true, false],
                [false, false, false, false],
            ],
            TetrominoType::L => [
                [false, false, true, false],
                [true, true, true, false],
                [false, false, false, false],
                [false, false, false, false],
            ],
            TetrominoType::J => [
                [true, false, false, false],
                [true, true, true, false],
                [false, false, false, false],
                [false, false, false, false],
            ],
            TetrominoType::S => [
                [false, true, true, false],
                [true, true, false, false],
                [false, false, false, false],
                [false, false, false, false],
            ],
            TetrominoType::Z => [
                [true, true, false, false],
                [false, true, true, false],
                [false, false, false, false],
                [false, false, false, false],
            ],
            _ => [[false; 4]; 4],
        };

        Self {
            tetromino_type,
            cells,
        }
    }

    pub fn rotate_clockwise(&mut self) {
        let mut new_cells = [[false; 4]; 4];
        for y in 0..4 {
            for x in 0..4 {
                new_cells[x][3 - y] = self.cells[y][x];
            }
        }
        self.cells = new_cells;
    }

    pub fn rotate_anticlockwise(&mut self) {
        let mut new_cells = [[false; 4]; 4];
        for y in 0..4 {
            for x in 0..4 {
                new_cells[3 - x][y] = self.cells[y][x];
            }
        }
        self.cells = new_cells;
    }
}
