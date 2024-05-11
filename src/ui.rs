use crate::state::State;

use ratatui::{prelude::*, widgets::*};

pub fn ui(frame: &mut Frame, state: &mut State) {
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

    let search = Paragraph::new(state.input.as_str())
        .style(Style::default())
        .block(Block::bordered().title(" Search "));
    frame.render_widget(search, search_area);
    frame.set_cursor(
        search_area.x + state.input.len() as u16 + 1,
        search_area.y + 1,
    );

    let programs: Vec<ListItem> = state
        .filtered_programs
        .iter()
        .map(|p| ListItem::new(Line::from(Span::raw(&p.name))))
        .collect();

    let programs = List::new(programs)
        .block(Block::bordered().title(" Programs "))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::LightBlue),
        )
        .highlight_symbol("> ");

    frame.render_stateful_widget(programs, programs_area, &mut state.list_state);
}
