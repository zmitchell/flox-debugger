mod home;
mod output;
mod vars;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Styled, Stylize},
    text::{Line, Span},
    widgets::{Block, Clear, Tabs},
};

use crate::{
    app::{App, ExitOption, ExitState, Screen, key_bindings::DisplayKeyBindings},
    ui::{home::render_home_screen, output::render_output_screen, vars::render_vars_screen},
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
        Screen::Prompt => render_dummy_screen("Prompt Screen", frame, body_area),
        Screen::Vars => render_vars_screen(app, frame, body_area),
        Screen::Trace => render_dummy_screen("Trace Screen", frame, body_area),
        Screen::Output => render_output_screen(app, frame, body_area),
    }
    if matches!(app.exit_state(), ExitState::PresentModal { .. }) {
        render_exit_modal(app, frame);
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
    .highlight_style(theme.selected_tab)
    .divider("|".set_style(theme.fg_dim))
    .select(app.screen().tab_index());
    frame.render_widget(tabs, tabs_area);
}

fn render_footer(app: &App, frame: &mut Frame, area: Rect) {
    let screen_bindings = match app.screen() {
        Screen::Home => app.key_bindings().home().displayable(),
        Screen::Prompt => app.key_bindings().prompt().displayable(),
        Screen::Vars => app.key_bindings().vars().displayable(),
        Screen::Trace => app.key_bindings().trace().displayable(),
        Screen::Output => app.key_bindings().output().displayable(),
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

fn render_exit_modal(app: &App, frame: &mut Frame) {
    let area = frame.area();
    let ExitState::PresentModal { highlighted_option } = app.exit_state() else {
        panic!("tried to draw exit modal in wrong state");
    };
    let theme = app.theme();

    // First clear the entire screen
    frame.render_widget(Clear, area);

    // Now create a layout to place the popup inside of
    let [vertical_area] = Layout::vertical([Constraint::Length(5)])
        .flex(Flex::Center)
        .areas(area);
    let [popup_area] = Layout::horizontal([Constraint::Length(30)])
        .flex(Flex::Center)
        .areas(vertical_area);

    // Draw the popup border
    frame.render_widget(Block::bordered(), popup_area);

    // Create the internal layout of the popup
    let [desc_area, buttons_area] =
        Layout::vertical([Constraint::Length(1), Constraint::Length(3)])
            .margin(1)
            .spacing(1)
            .areas(popup_area);

    let desc = Line::from("Exit?").alignment(Alignment::Center);
    frame.render_widget(desc, desc_area);

    // Create the areas for each button and determine the style based on
    // which button is selected
    let [ok_area, cancel_area] =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .flex(Flex::Center)
            .spacing(1)
            .areas(buttons_area);
    let (ok_style, cancel_style) = if highlighted_option == ExitOption::Ok {
        (theme.highlighted_text, theme.fg)
    } else {
        (theme.fg, theme.highlighted_text)
    };

    let ok_button = Line::from("[   Ok   ]".set_style(ok_style)).alignment(Alignment::Center);
    let cancel_button =
        Line::from("[ Cancel ]".set_style(cancel_style)).alignment(Alignment::Center);

    frame.render_widget(ok_button, ok_area);
    frame.render_widget(cancel_button, cancel_area);
}

fn render_dummy_screen(name: &str, frame: &mut Frame, area: Rect) {
    frame.render_widget(Line::from(name).alignment(Alignment::Center), area);
}
