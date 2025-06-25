use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style, Styled, Stylize},
    symbols,
    text::{Line, Span, Text},
    widgets::{Block, Paragraph, Tabs, Wrap},
};
use tui_big_text::{BigTextBuilder, PixelSize};

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
    let theme = app.theme();
    frame.render_widget(Block::bordered(), area);
    let [_blank, splash_area, info_area, description_area] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(7),
        Constraint::Length(5),
        Constraint::Percentage(100),
    ])
    .margin(1)
    .areas(area);

    // Renders the big "flox-debugger" pixel text
    let splash_text = BigTextBuilder::default()
        .pixel_size(PixelSize::Sextant)
        .lines(["flox-".into(), "debugger".into()])
        .alignment(Alignment::Center)
        .build();
    frame.render_widget(splash_text, splash_area);

    // Renders the short description below the "flox-debugger" text
    let info_text = Text::from(vec![
        Line::from("Debug and inspect a Flox environment"),
        Line::from(symbols::line::HORIZONTAL.repeat(33).set_style(theme.fg_dim)),
        Line::from("https://github.com/flox/flox".italic()),
        Line::from(vec![
            "[".set_style(theme.fg_dim),
            "with ".into(),
            "♥".set_style(theme.flox_purple),
            " from ".into(),
            "@zmitchell".set_style(theme.flox_purple),
            "]".set_style(theme.fg_dim),
        ]),
    ]);
    frame.render_widget(Paragraph::new(info_text).centered(), info_area);

    // Renders the description on the home page.
    let [prose_area] = Layout::vertical([Constraint::Percentage(100)])
        .margin(2)
        .areas(description_area);
    let description = Paragraph::new(Text::from(vec![
        Line::from(vec![
            "This debugger allows you to pause the activation of an environment, ".into(),
            "inspect its state, *modify* its state, and determine whether/where to ".into(),
            "pause execution when the debugger closes.".into(),
        ]),
        Line::default(),
        Line::from("The debugger has capabilities separated out into different tabs:"),
        Line::from("- Home: you are here"),
        Line::from("- Prompt: run commands to set breakpoints, etc"),
        Line::from("- Vars: inspect and modify environment variables"),
        Line::from("- Trace: see a stack trace of shell execution"),
        Line::from("- Output: see the commands that will be sourced when the debugger exits"),
    ]))
    .wrap(Wrap::default())
    .left_aligned();
    frame.render_widget(description, prose_area);
}
