mod state;

use crate::state::{App, Dir, Program};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};
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
    let res = run(&mut terminal, App::new());
    restore_terminal(&mut terminal)?;

    if let Ok(Some(program)) = res {
        let mut cmd = vec![
            String::from("hyprctl"),
            String::from("dispatch"),
            String::from("exec"),
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
    mut app: App,
) -> io::Result<Option<Program>> {
    app.load_progams();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('j') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.move_index(Dir::Down);
                    }
                    KeyCode::Char('k') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.move_index(Dir::Up);
                    }
                    KeyCode::Char('w') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.delete_word();
                    }
                    KeyCode::Up => {
                        app.move_index(Dir::Up);
                    }
                    KeyCode::Down => {
                        app.move_index(Dir::Down);
                    }
                    KeyCode::Char(to_insert) => {
                        app.enter_char(to_insert);
                    }
                    KeyCode::Backspace => {
                        app.delete_char();
                    }
                    KeyCode::Enter => {
                        return Ok(Some(app.filtered_programs[app.index].clone()));
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

fn ui(frame: &mut Frame, app: &App) {
    let vertical = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(3),
        Constraint::Min(0),
    ]);
    let [help_area, search_area, programs_area] = vertical.areas(frame.size());

    let text = Text::from(Line::from(vec![
        "Esc".bold(),
        " (exit), ".into(),
        "Enter".bold(),
        " (launch)".into(),
    ]))
    .patch_style(Style::default());
    let help = Paragraph::new(text);
    frame.render_widget(help, help_area);

    let search = Paragraph::new(app.input.as_str())
        .style(Style::default())
        .block(Block::bordered().title(" Search "));
    frame.render_widget(search, search_area);
    frame.set_cursor(
        search_area.x + app.input.len() as u16 + 1,
        search_area.y + 1,
    );

    let programs: Vec<ListItem> = app
        .filtered_programs
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let content = Line::from(match app.index == i {
                true => Span::raw(&p.name).style(Style::default().fg(Color::LightBlue)),
                false => Span::raw(&p.name),
            });
            ListItem::new(content)
        })
        .collect();

    let programs = List::new(programs).block(Block::bordered().title(" Programs "));

    frame.render_widget(programs, programs_area);
}
