use crate::client::board::{Board, Cell, HEIGHT, WIDTH};
use crate::client::tetromino::{Tetromino, TetrominoType};
use crossterm::{cursor::MoveTo, event::KeyCode, execute};
use rand::Rng;
use std::io::stdout;

const PREVIEW_WIDTH: usize = 4; // Each cell is a full-width character
const PREVIEW_HEIGHT: usize = 4; // Keep height the same as tetromino size
const PREVIEW_PADDING: i32 = 2;
const BOX_PADDING: i32 = 1; // Padding inside the box

#[derive(Clone, PartialEq)]
pub enum GameState {
    TitleScreen { selected_option: usize },
    Playing,
    Paused,
    GameOver,
}

pub struct Game {
    board: Board,
    current_piece: Tetromino,
    next_piece: Tetromino,
    piece_x: i32,
    piece_y: i32,
    score: u32,
    held_piece: Option<TetrominoType>,
    can_hold: bool,
    state: GameState,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Self {
            board: Board::new(),
            current_piece: Tetromino::new(TetrominoType::I),
            next_piece: Tetromino::new(TetrominoType::I),
            piece_x: WIDTH as i32 / 2 - 2,
            piece_y: 0,
            score: 0,
            held_piece: None,
            can_hold: true,
            state: GameState::TitleScreen { selected_option: 0 },
        };
        game.spawn_piece();
        game
    }

    pub fn spawn_piece(&mut self) {
        self.current_piece = self.next_piece.clone();
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
        self.next_piece = Tetromino::new(piece_types[rng.gen_range(0..piece_types.len())]);

        self.piece_x = WIDTH as i32 / 2 - 2;
        self.piece_y = 0;

        if self.check_collision() {
            self.state = GameState::GameOver;
        }

        self.can_hold = true;
    }

    pub fn rotate(&mut self, is_clockwise: bool) {
        let original_x = self.piece_x;
        let original_y = self.piece_y;
        let original_cells = self.current_piece.cells;

        if is_clockwise {
            self.current_piece.rotate_clockwise();
        } else {
            self.current_piece.rotate_anticlockwise();
        }

        // Revert if rotation causes collision
        if self.check_collision() {
            self.current_piece.cells = original_cells;
            self.piece_x = original_x;
            self.piece_y = original_y;
        }
    }

    pub fn hard_drop(&mut self) {
        while self.move_piece(0, 1) {} // Move until collision
        self.lock_piece();
    }

    pub fn get_state(&self) -> GameState {
        self.state.clone()
    }

    fn draw_piece_preview(&self, piece: &Tetromino, x: i32, y: i32, title: &str) {
        // Draw top border with title
        execute!(stdout(), MoveTo(x as u16, y as u16)).unwrap();
        println!("┌────────┐"); // 4 cells × 2 chars per cell = 8 chars wide

        execute!(stdout(), MoveTo((x + 1) as u16, y as u16)).unwrap();
        print!(" {} ", title);

        // Create a temporary mini-board
        let mut preview = vec![vec![Cell::Empty; PREVIEW_WIDTH]; PREVIEW_HEIGHT];

        // Calculate centering for piece
        for py in 0..4 {
            for px in 0..4 {
                if piece.cells[py][px] {
                    if py < PREVIEW_HEIGHT && px < PREVIEW_WIDTH {
                        preview[py][px] = Cell::Filled;
                    }
                }
            }
        }

        // Draw the preview contents
        for row in 0..PREVIEW_HEIGHT {
            execute!(stdout(), MoveTo(x as u16, (y + 1 + row as i32) as u16)).unwrap();
            print!("│"); // Left border

            for col in 0..PREVIEW_WIDTH {
                print!("{}", preview[row][col].to_string());
            }

            println!("│"); // Right border
        }

        // Draw bottom border
        execute!(
            stdout(),
            MoveTo(x as u16, (y + PREVIEW_HEIGHT as i32 + 1) as u16)
        )
        .unwrap();
        println!("└────────┘");
    }

    pub fn draw(&self) {
        let (term_width, term_height) = crossterm::terminal::size().unwrap_or((80, 24));
        print!("\x1B[2J");

        match &self.state {
            GameState::TitleScreen { selected_option } => {
                self.draw_title_screen(*selected_option, term_width, term_height);
            }
            GameState::Paused => {
                self.draw_pause_screen(term_width, term_height);
            }
            GameState::Playing | GameState::GameOver => {
                self.draw_game_screen(term_width, term_height);
            }
        }
    }

    fn draw_title_screen(&self, selected_option: usize, term_width: u16, term_height: u16) {
        let logo = vec!["---------------", "   Tet-Rust!   ", "---------------"];

        let menu_options = vec!["Play", "Help", "GitHub", "Quit"];

        let start_y = (term_height as i32) / 3; // Move logo higher up
                                                // Draw logo
        for (i, line) in logo.iter().enumerate() {
            execute!(
                stdout(),
                MoveTo(
                    (term_width as i32 - line.len() as i32) as u16 / 2,
                    (start_y + i as i32) as u16
                ),
            )
            .unwrap();
            println!("{}", line);
        }

        for (i, option) in menu_options.iter().enumerate() {
            execute!(
                stdout(),
                MoveTo(
                    (term_width as i32 - option.len() as i32 - 4) as u16 / 2,
                    (start_y + logo.len() as i32 + 1 + i as i32) as u16
                ),
            )
            .unwrap();
            print!(
                "{} {}",
                if i == selected_option { ">" } else { " " },
                option
            );
        }

        let line = "Created by Han Yi";

        execute!(
            stdout(),
            MoveTo(
                (term_width as i32 - line.len() as i32) as u16 / 2,
                (start_y + logo.len() as i32 + menu_options.len() as i32 + 2) as u16
            ),
        )
        .unwrap();
        println!("{}", line);
    }

    fn draw_game_screen(&self, term_width: u16, term_height: u16) {
        let board_width = WIDTH;
        let board_height = HEIGHT;
        let start_x = (term_width as i32 - board_width as i32 * 2) / 2;
        let start_y = (term_height as i32 - board_height as i32) / 2;

        let mut temp_board = self.board.clone();

        // Draw landing preview
        let landing_y = self.get_landing_position();
        for y in 0..4 {
            for x in 0..4 {
                if self.current_piece.cells[y][x] {
                    let board_x = self.piece_x + x as i32;
                    let board_y = landing_y + y as i32;
                    if board_y >= 0
                        && board_y < HEIGHT as i32
                        && board_x >= 0
                        && board_x < WIDTH as i32
                    {
                        if temp_board.get(board_x as usize, board_y as usize) == Cell::Empty {
                            temp_board.set(board_x as usize, board_y as usize, Cell::Preview);
                        }
                    }
                }
            }
        }

        // Draw current piece
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

        // Calculate preview positions - both on right side
        let preview_x = start_x + board_width as i32 * 2 + PREVIEW_PADDING;

        // Draw next piece (always shown)
        self.draw_piece_preview(&self.next_piece, preview_x, start_y, "NEXT");

        // Draw hold piece if it exists
        if let Some(held_type) = self.held_piece {
            let held_piece = Tetromino::new(held_type);
            let hold_y = start_y + PREVIEW_HEIGHT as i32 + 3;
            self.draw_piece_preview(&held_piece, preview_x, hold_y, "HOLD");
        }

        // Draw main board
        for y in 0..HEIGHT {
            execute!(
                stdout(),
                MoveTo(start_x as u16, (start_y + y as i32) as u16),
            )
            .unwrap();

            for x in 0..WIDTH {
                print!("{}", temp_board.get(x, y).to_string());
            }
        }

        // Draw score
        let score_text = format!("Score: {}", self.score);
        execute!(
            stdout(),
            MoveTo(
                (start_x + (board_width as i32 * 2 - score_text.len() as i32) / 2) as u16,
                (start_y + board_height as i32 + 1) as u16
            ),
        )
        .unwrap();
        println!("{}", score_text);

        // Draw game over message if needed
        if let GameState::GameOver = self.state {
            let game_over_text = "Game Over!";
            execute!(
                stdout(),
                MoveTo(
                    (start_x + (board_width as i32 * 2 - game_over_text.len() as i32) / 2) as u16,
                    (start_y + board_height as i32 + 2) as u16
                ),
            )
            .unwrap();
            println!("{}", game_over_text);

            let restart_text = "Press 'r' to restart or 'q' to quit";
            execute!(
                stdout(),
                MoveTo(
                    (start_x + (board_width as i32 * 2 - restart_text.len() as i32) / 2) as u16,
                    (start_y + board_height as i32 + 3) as u16
                ),
            )
            .unwrap();
            println!("{}", restart_text);
        }
    }

    fn draw_pause_screen(&self, term_width: u16, term_height: u16) {
        let help_text = vec![
            "Controls:",
            "←/→: Move piece",
            "A: Rotate clockwise",
            "D: Rotate anti-clockwise",
            "↓: Soft drop",
            "Space: Hard drop",
            "C: Hold piece",
            "Esc/P: Pause/Unpause",
            "R: Restart game",
            "Q: Quit game",
            "",
            "Press Esc or P to resume",
        ];

        let start_y = (term_height as i32 - help_text.len() as i32) / 2;

        for (i, line) in help_text.iter().enumerate() {
            execute!(
                stdout(),
                MoveTo(
                    (term_width as i32 - line.len() as i32) as u16 / 2,
                    (start_y + i as i32) as u16
                ),
            )
            .unwrap();
            println!("{}", line);
        }
    }

    pub fn handle_title_input(&mut self, key: KeyCode) {
        if let GameState::TitleScreen { selected_option } = &mut self.state {
            match key {
                KeyCode::Up => {
                    *selected_option = selected_option.checked_sub(1).unwrap_or(3);
                }
                KeyCode::Down => {
                    *selected_option = (*selected_option + 1) % 4;
                }
                KeyCode::Enter => match *selected_option {
                    0 => self.state = GameState::Playing,
                    1 => self.state = GameState::Paused,
                    2 => if let Ok(()) = open::that("https://github.com/yhanyi/TetRust") {},
                    3 => std::process::exit(0),
                    _ => {}
                },
                _ => {}
            }
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
        let piece = self.current_piece.cells;
        let piece_x = self.piece_x;
        let piece_y = self.piece_y;

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

    pub fn hold_piece(&mut self) {
        if self.can_hold {
            let current_type = self.current_piece.tetromino_type;
            self.current_piece = match self.held_piece {
                Some(held_type) => Tetromino::new(held_type),
                None => {
                    self.spawn_piece();
                    self.current_piece.clone()
                }
            };
            self.held_piece = Some(current_type);
            self.piece_x = WIDTH as i32 / 2 - 2;
            self.piece_y = 0;
            self.can_hold = false;
        }
    }

    fn get_landing_position(&self) -> i32 {
        let mut test_y = self.piece_y;
        while !self.would_collide(self.piece_x, test_y + 1) {
            test_y += 1;
        }
        test_y
    }

    // Checks for potential collision
    fn would_collide(&self, test_x: i32, test_y: i32) -> bool {
        for y in 0..4 {
            for x in 0..4 {
                if self.current_piece.cells[y][x] {
                    let board_x = test_x + x as i32;
                    let board_y = test_y + y as i32;

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

    pub fn toggle_pause(&mut self) {
        self.state = match self.state {
            GameState::Playing => GameState::Paused,
            GameState::Paused => GameState::Playing,
            _ => self.state.clone(),
        };
    }

    pub fn restart(&mut self) {
        self.board = Board::new();
        self.score = 0;
        self.state = GameState::Playing;
        self.held_piece = None;
        self.can_hold = true;
        self.spawn_piece();
    }
}
