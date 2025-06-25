use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Styled, Stylize},
    symbols,
    text::{Line, Text},
    widgets::{Block, Paragraph, Wrap},
};
use tui_big_text::{BigTextBuilder, PixelSize};

use crate::app::App;

pub fn render_home_screen(app: &App, frame: &mut Frame, area: Rect) {
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
