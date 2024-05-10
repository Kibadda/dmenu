use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use ratatui::{prelude::*, widgets::*};
use std::{
    error::Error,
    io::{self, Stdout},
    process::{Command, Stdio},
};

enum Dir {
    Up,
    Down,
    Same,
}

struct App {
    input: String,
    programs: Vec<Program>,
    filtered_programs: Vec<Program>,
    index: usize,
}

#[derive(Clone, Debug)]
struct Program {
    name: String,
    cmd: Vec<String>,
}

impl App {
    const fn new() -> Self {
        Self {
            input: String::new(),
            programs: vec![],
            filtered_programs: vec![],
            index: 0,
        }
    }

    fn load_progams(&mut self) {
        self.programs = vec![
            Program {
                name: String::from("Spotify"),
                cmd: vec![String::from("spotify")],
            },
            Program {
                name: String::from("Discord"),
                cmd: vec![String::from("discord")],
            },
            Program {
                name: String::from("Neovim"),
                cmd: vec![String::from("nvim")],
            },
        ];

        self.filter();
    }

    fn filter(&mut self) {
        let matcher = SkimMatcherV2::default();

        self.filtered_programs = self
            .programs
            .clone()
            .into_iter()
            .filter(|p| matcher.fuzzy_match(&p.name, &self.input).is_some())
            .collect();

        self.move_index(Dir::Same);
    }

    fn enter_char(&mut self, new_char: char) {
        self.input.insert(self.input.len(), new_char);
        self.filter();
    }

    fn delete_char(&mut self) {
        if !self.input.is_empty() {
            self.input = self.input.chars().take(self.input.len() - 1).collect();
            self.filter();
        }
    }

    fn move_index(&mut self, dir: Dir) {
        let len = self.filtered_programs.len();

        if len == 0 {
            self.index = 0;
        } else {
            self.index = match dir {
                Dir::Up => match self.index == 0 {
                    true => len - 1,
                    false => self.index - 1,
                },
                Dir::Down => match self.index == len - 1 {
                    true => 0,
                    false => self.index + 1,
                },
                Dir::Same => self.index.clamp(0, len - 1),
            }
        };
    }
}

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
    let res = run_app(&mut terminal, App::new());
    restore_terminal(&mut terminal)?;

    if let Ok(Some(program)) = res {
        Command::new(&program.cmd[0])
            .args(program.cmd)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<Option<Program>> {
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
        "Press ".into(),
        "Esc".bold(),
        " to exit editing, ".into(),
        "Enter".bold(),
        " to launch program".into(),
    ]))
    .patch_style(Style::default());
    let help = Paragraph::new(text);
    frame.render_widget(help, help_area);

    let search = Paragraph::new(app.input.as_str())
        .style(Style::default())
        .block(Block::bordered().title("Search"));
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

    let programs = List::new(programs).block(Block::bordered().title("Programs"));

    frame.render_widget(programs, programs_area);
}
