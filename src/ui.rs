use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style, Styled, Stylize},
    text::{Line, Span},
    widgets::{Block, Tabs},
};

use crate::app::{
    App, Screen,
    key_bindings::{DisplayKeyBindings, GlobalKeyBindings, HomeKeyBindings},
};

pub fn draw_ui(app: &mut App, frame: &mut Frame) {
    // This creates the header box and the main box below it.
    let [header_area, body_area, footer_area] = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ],
    )
    .margin(1)
    .areas(frame.area());

    render_header(app, frame, header_area);
    render_footer(app, frame, footer_area);
    match app.screen() {
        Screen::Home => render_home_screen(app, frame, body_area),
        _ => unreachable!(),
    }
}

fn render_header(app: &App, frame: &mut Frame, area: Rect) {
    let theme = app.theme();
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
    frame.render_widget(header_box, area);
    let [tabs_area] = Layout::horizontal([Constraint::Percentage(100)])
        .margin(1)
        .areas(area);
    let tabs = Tabs::new(
        [
            Screen::Home,
            Screen::Prompt,
            Screen::Vars,
            Screen::Trace,
            Screen::Output,
        ]
        .into_iter()
        .map(|s| s.to_string().set_style(theme.flox_purple)),
    )
    .highlight_style(theme.selected_option)
    .divider("|".set_style(theme.fg_dim))
    .select(app.screen().tab_index());
    frame.render_widget(tabs, tabs_area);
}

fn render_footer(app: &App, frame: &mut Frame, area: Rect) {
    let screen_bindings = match app.screen() {
        Screen::Home => app.key_bindings().home().displayable(),
        _ => unreachable!(),
    };
    let applicable_bindings = {
        let mut bindings = app.key_bindings().global().displayable();
        bindings.extend_from_slice(&screen_bindings);
        bindings
    };
    let theme = app.theme();
    let mut formatted_bindings = applicable_bindings
        .into_iter()
        .flat_map(|(keys, desc)| {
            // foo
            vec![
                " [".set_style(theme.fg_dim),
                keys.set_style(theme.flox_purple),
                ": ".set_style(theme.fg_dim),
                desc.set_style(theme.fg),
                "]".set_style(theme.fg_dim),
            ]
        })
        .collect::<Vec<Span>>();
    formatted_bindings.push(" ".into());
    let line = Line::from(formatted_bindings).alignment(Alignment::Center);
    let [line_area] = Layout::horizontal([Constraint::Percentage(100)])
        .margin(1)
        .areas(area);
    frame.render_widget(Block::bordered(), area);
    frame.render_widget(line, line_area);
}

fn render_home_screen(app: &App, frame: &mut Frame, area: Rect) {
    frame.render_widget(Block::bordered(), area);
}
