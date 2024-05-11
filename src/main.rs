mod program;
mod state;
mod ui;

use crate::program::Program;
use crate::state::{Dir, State};
use crate::ui::ui;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::{
    error::Error,
    io::{self, Stdout},
    process::{Command, Stdio},
};

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(terminal.show_cursor()?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = setup_terminal()?;
    let res = run(&mut terminal, State::new());
    restore_terminal(&mut terminal)?;

    if let Ok(Some(program)) = res {
        let mut cmd = vec![
            String::from("hyprctl"),
            String::from("dispatch"),
            String::from("exec"),
            String::from("--"),
        ];
        cmd.append(&mut program.cmd.clone());

        Command::new(&cmd[0])
            .args(&cmd[1..])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?
            .wait()?;
    }

    Ok(())
}

fn run(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    mut state: State,
) -> io::Result<Option<Program>> {
    state.load_progams();

    loop {
        terminal.draw(|f| ui(f, &mut state))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('j') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        state.move_index(Dir::Down);
                    }
                    KeyCode::Char('k') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        state.move_index(Dir::Up);
                    }
                    KeyCode::Char('w') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        state.delete_word();
                    }
                    KeyCode::Up => {
                        state.move_index(Dir::Up);
                    }
                    KeyCode::Down => {
                        state.move_index(Dir::Down);
                    }
                    KeyCode::Char(to_insert) => {
                        state.enter_char(to_insert);
                    }
                    KeyCode::Backspace => {
                        state.delete_char();
                    }
                    KeyCode::Enter => {
                        if let Some(i) = state.list_state.selected() {
                            return Ok(Some(state.filtered_programs[i].clone()));
                        }
                    }
                    KeyCode::Esc => {
                        return Ok(None);
                    }
                    _ => {}
                }
            }
        }
    }
}
