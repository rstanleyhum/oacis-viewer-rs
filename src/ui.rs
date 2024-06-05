use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{palette::tailwind, Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, HighlightSpacing, List, Paragraph},
    Frame,
};

use crate::app::App;

const SELECTED_STYLE_FG: Color = tailwind::BLUE.c300;

pub fn ui(f: &mut Frame, app: &mut App) {
    // Create the layout sections.

    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(40),
            Constraint::Percentage(50),
            Constraint::Percentage(10),
        ])
        .split(f.size());

    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(outer_layout[0]);

    let pid_items = app.get_pid_items();
    let pid_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let pid_items = List::new(pid_items)
        .block(pid_block)
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::REVERSED)
                .fg(SELECTED_STYLE_FG),
        )
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always);

    let lab_items = app.get_lab_items();

    let labs_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let lab_items = List::new(lab_items)
        .block(labs_block)
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::REVERSED)
                .fg(SELECTED_STYLE_FG),
        )
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always);

    let result_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let result_title = Paragraph::new(Text::styled(
        "Results block",
        Style::default().fg(Color::Green),
    ))
    .block(result_block);

    let status_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let current_pid = app.get_selected_pid();
    let current_lab = app.get_selected_lab();

    let status_title = Paragraph::new(Text::styled(
        format!("Status Bar: PID: {current_pid}   LAB: {current_lab}"),
        Style::default().fg(Color::Green),
    ))
    .block(status_block);

    f.render_stateful_widget(pid_items, inner_layout[0], &mut app.pid_list.state);
    f.render_stateful_widget(lab_items, inner_layout[1], &mut app.lab_list.state);
    f.render_widget(result_title, outer_layout[1]);
    f.render_widget(status_title, outer_layout[2]);
}
