use crate::client::board::{Board, Cell, HEIGHT, WIDTH};
use crate::client::tetromino::{Tetromino, TetrominoType};
use crossterm::{cursor::MoveTo, execute};
use rand::Rng;
use std::io::stdout;

pub struct Game {
    board: Board,
    current_piece: Tetromino,
    piece_x: i32,
    piece_y: i32,
    score: u32,
    game_over: bool,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            current_piece: Tetromino::new(TetrominoType::I),
            piece_x: WIDTH as i32 / 2 - 2,
            piece_y: 0,
            score: 0,
            game_over: false,
        }
    }

    pub fn spawn_piece(&mut self) {
        let mut rng = rand::thread_rng();
        let piece_types = [
            TetrominoType::I,
            TetrominoType::O,
            TetrominoType::T,
            TetrominoType::L,
            TetrominoType::J,
            TetrominoType::S,
            TetrominoType::Z,
        ];
        self.current_piece = Tetromino::new(piece_types[rng.gen_range(0..piece_types.len())]);
        self.piece_x = WIDTH as i32 / 2 - 2;
        self.piece_y = 0;

        if self.check_collision() {
            self.game_over = true;
        }
    }

    pub fn rotate(&mut self, is_clockwise: bool) {
        // Store original position and cells
        let original_x = self.piece_x;
        let original_y = self.piece_y;
        let original_cells = self.current_piece.cells;

        // Try rotation
        if is_clockwise {
            self.current_piece.rotate_clockwise();
        } else {
            self.current_piece.rotate_anticlockwise();
        }

        // If rotation causes collision, revert back
        if self.check_collision() {
            self.current_piece.cells = original_cells;
            self.piece_x = original_x;
            self.piece_y = original_y;
        }
    }

    pub fn draw(&self) {
        // Get terminal size
        let (term_width, term_height) = crossterm::terminal::size().unwrap_or((80, 24));

        // Centre board, +2 for borders
        let board_width = WIDTH + 2;
        let board_height = HEIGHT + 2;
        let start_x = (term_width as i32 - board_width as i32) / 2;
        let start_y = (term_height as i32 - board_height as i32) / 2;

        // Clear screen once
        print!("\x1B[2J");

        // Create temporary board with current piece
        let mut temp_board = self.board.clone();

        // Draw current piece on temporary board
        for y in 0..4 {
            for x in 0..4 {
                if self.current_piece.cells[y][x] {
                    let board_x = self.piece_x + x as i32;
                    let board_y = self.piece_y + y as i32;
                    if board_y >= 0
                        && board_y < HEIGHT as i32
                        && board_x >= 0
                        && board_x < WIDTH as i32
                    {
                        temp_board.set(board_x as usize, board_y as usize, Cell::Filled);
                    }
                }
            }
        }

        // Move cursor and draw top border
        execute!(stdout(), MoveTo(start_x as u16, start_y as u16),).unwrap();
        // println!("┌{}┐", "─".repeat(WIDTH));

        // Draw board contents
        for y in 0..HEIGHT {
            execute!(
                stdout(),
                MoveTo(start_x as u16, (start_y + 1 + y as i32) as u16),
            )
            .unwrap();

            for x in 0..WIDTH {
                match temp_board.get(x, y) {
                    Cell::Empty => print!("⬛"),
                    Cell::Filled => print!("⬜"),
                }
            }
        }

        // Draw bottom border
        execute!(
            stdout(),
            MoveTo(start_x as u16, (start_y + HEIGHT as i32 + 1) as u16),
        )
        .unwrap();

        // Draw score below the board
        execute!(
            stdout(),
            MoveTo(start_x as u16, (start_y + HEIGHT as i32 + 2) as u16),
        )
        .unwrap();
        println!("Score: {}", self.score);

        if self.game_over {
            execute!(
                stdout(),
                MoveTo(start_x as u16, (start_y + HEIGHT as i32 + 3) as u16),
            )
            .unwrap();
            println!("Game Over!");
            execute!(
                stdout(),
                MoveTo(start_x as u16, (start_y + HEIGHT as i32 + 4) as u16),
            )
            .unwrap();
            println!("Press 'r' to restart or 'q' to quit");
        }
    }

    pub fn move_piece(&mut self, dx: i32, dy: i32) -> bool {
        self.piece_x += dx;
        self.piece_y += dy;

        if self.check_collision() {
            self.piece_x -= dx;
            self.piece_y -= dy;
            false
        } else {
            true
        }
    }

    fn check_collision(&self) -> bool {
        for y in 0..4 {
            for x in 0..4 {
                if self.current_piece.cells[y][x] {
                    let board_x = self.piece_x + x as i32;
                    let board_y = self.piece_y + y as i32;

                    if board_x < 0 || board_x >= WIDTH as i32 || board_y >= HEIGHT as i32 {
                        return true;
                    }

                    if board_y >= 0
                        && self.board.get(board_x as usize, board_y as usize) == Cell::Filled
                    {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn lock_piece(&mut self) {
        // Create a temporary copy of the current piece's data
        let piece = self.current_piece.cells;
        let piece_x = self.piece_x;
        let piece_y = self.piece_y;

        // Update the board with the piece's position
        for y in 0..4 {
            for x in 0..4 {
                if piece[y][x] {
                    let board_x = piece_x + x as i32;
                    let board_y = piece_y + y as i32;
                    if board_y >= 0
                        && board_y < HEIGHT as i32
                        && board_x >= 0
                        && board_x < WIDTH as i32
                    {
                        self.board
                            .set(board_x as usize, board_y as usize, Cell::Filled);
                    }
                }
            }
        }

        self.clear_lines();
        self.spawn_piece();
    }

    fn clear_lines(&mut self) {
        let mut lines_cleared = 0;
        for y in (0..HEIGHT).rev() {
            let mut line_filled = true;
            for x in 0..WIDTH {
                if self.board.get(x, y) == Cell::Empty {
                    line_filled = false;
                    break;
                }
            }
            if line_filled {
                lines_cleared += 1;
                self.board.clear_line(y);
            }
        }
        self.score += match lines_cleared {
            1 => 100,
            2 => 300,
            3 => 500,
            4 => 800,
            _ => 0,
        };
    }

    pub fn restart(&mut self) {
        self.board = Board::new();
        self.score = 0;
        self.game_over = false;
        self.spawn_piece();
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }
}
