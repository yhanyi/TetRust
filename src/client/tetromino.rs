#[derive(Clone, Copy)]
pub enum TetrominoType {
    I,
    O,
    T,
    // L,
    // J,
    // S,
    // Z,
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
            // TODO: Add L, J, S, Z blocks
            _ => [[false; 4]; 4],
        };

        Self {
            tetromino_type,
            cells,
        }
    }
}
