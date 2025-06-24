use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    symbols,
    text::Span,
    widgets::{Block, Tabs},
};

use crate::app::{App, Screen};

#[derive(Debug)]
struct Theme {
    flox_purple: Style,
    fg_text: Style,
    selected_option: Style,
}

impl Theme {
    fn new() -> Self {
        Theme {
            flox_purple: Style::default().fg(Color::Rgb(175, 135, 255)),
            fg_text: Style::default(),
            selected_option: Style::default().add_modifier(Modifier::BOLD),
        }
    }
}

pub fn draw_ui(app: &mut App, frame: &mut Frame) {
    // This creates the header box and the main box below it.
    let [header_area, body_area] = Layout::new(
        Direction::Vertical,
        [Constraint::Length(3), Constraint::Min(0)],
    )
    .margin(1)
    .areas(frame.area());

    let header_title: Vec<Span> = vec![
        " ".into(),
        env!("CARGO_PKG_NAME").bold(),
        "-".into(),
        env!("CARGO_PKG_VERSION").bold(),
        " ".into(),
    ];
    let header_box = Block::bordered()
        .title(header_title)
        .title_alignment(Alignment::Center);
    frame.render_widget(header_box, header_area);
    let [tabs_area] = Layout::horizontal([Constraint::Percentage(100)])
        .margin(1)
        .areas(header_area);
    let tabs = Tabs::new(
        [
            Screen::Home,
            Screen::Prompt,
            Screen::Vars,
            Screen::Trace,
            Screen::Output,
        ]
        .into_iter()
        .map(|s| s.to_string()),
    )
    .style(Style::default().fg(ratatui::style::Color::Rgb(175, 135, 255)))
    .highlight_style(Style::default().white().add_modifier(Modifier::BOLD))
    .select(0);
    frame.render_widget(tabs, tabs_area);
}
