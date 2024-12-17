mod client;

use crate::client::game::{Game, GameState};
use crossterm::{
    cursor::{Hide, Show},
    event::{read, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io::stdout, thread, time::Duration};

const TICK_RATE: u64 = 500;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;

    let mut game = Game::new();
    game.spawn_piece();

    let mut last_tick = std::time::Instant::now();

    loop {
        if crossterm::event::poll(Duration::from_millis(50))? {
            match read()? {
                Event::Key(key_event) => match &game.get_state() {
                    GameState::TitleScreen { .. } => {
                        game.handle_title_input(key_event.code);
                    }
                    GameState::Playing => match key_event.code {
                        KeyCode::Left => {
                            game.move_piece(-1, 0);
                        }
                        KeyCode::Right => {
                            game.move_piece(1, 0);
                        }
                        KeyCode::Down => {
                            game.move_piece(0, 1);
                        }
                        KeyCode::Char('a') => {
                            game.rotate(false);
                        }
                        KeyCode::Char('s') => {
                            game.rotate(true);
                        }
                        KeyCode::Char('r') => {
                            game.restart();
                        }
                        KeyCode::Char('c') => {
                            game.hold_piece();
                        }
                        KeyCode::Char(' ') => {
                            game.hard_drop();
                        }
                        KeyCode::Esc | KeyCode::Char('p') => {
                            game.toggle_pause();
                        }
                        _ => {}
                    },
                    GameState::Paused => match key_event.code {
                        KeyCode::Esc | KeyCode::Char('p') => game.toggle_pause(),
                        _ => {}
                    },
                    GameState::GameOver => match key_event.code {
                        KeyCode::Char('r') => game = Game::new(),
                        KeyCode::Char('q') => break,
                        _ => {}
                    },
                },
                _ => {}
            }
        }

        if matches!(game.get_state(), GameState::Playing)
            && last_tick.elapsed() >= Duration::from_millis(TICK_RATE)
        {
            if !game.move_piece(0, 1) {
                game.lock_piece();
            }
            last_tick = std::time::Instant::now();
        }

        game.draw();
        thread::sleep(Duration::from_millis(50));
    }

    execute!(stdout, Show, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
