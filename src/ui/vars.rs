use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style, Styled},
    text::Line,
    widgets::{Block, List, Paragraph, Wrap},
};

use crate::app::{App, vars::VarDetailState};

pub fn render_vars_screen(app: &mut App, frame: &mut Frame, area: Rect) {
    let [var_list_area, detail_area] =
        Layout::horizontal([Constraint::Percentage(33), Constraint::Percentage(67)])
            .spacing(1)
            .areas(area);

    render_var_list(app, frame, var_list_area);
    render_var_detail(app, frame, detail_area);
}

fn render_var_list(app: &mut App, frame: &mut Frame, area: Rect) {
    let theme = app.theme();
    let block = Block::bordered().title(" Variables ");
    let block = if app.env().var_list_focused() {
        block.border_style(theme.flox_purple)
    } else {
        block
    };
    let env = app.env_mut();
    let vars = env.vars().to_vec();
    let var_list = List::new(vars)
        .block(block)
        .highlight_style(theme.highlighted_text);
    frame.render_stateful_widget(var_list, area, env.var_list_state());
}

fn render_var_detail(app: &mut App, frame: &mut Frame, area: Rect) {
    let [var_name_area, detail_sub_area] =
        Layout::vertical([Constraint::Length(3), Constraint::Percentage(100)])
            .spacing(1)
            .areas(area);

    // Render the boxed variable name
    let selected_var = app.env().selected_var();
    frame.render_widget(Block::bordered().title(" Name "), var_name_area);
    let [var_name_area_inner] = Layout::vertical([Constraint::Percentage(100)])
        .margin(1)
        .areas(var_name_area);
    frame.render_widget(
        Paragraph::new(
            selected_var
                .clone()
                .unwrap_or("<No variable selected>".to_string()),
        ),
        var_name_area_inner,
    );

    let selected_detail_item = app
        .env()
        .selected_detail_item()
        .unwrap_or("<No item selected>".to_string());

    let detail_block_title = var_detail_block_title(app.env().var_detail_state());
    let theme = app.theme();
    let block = Block::bordered().title(detail_block_title);
    let block = if app.env().var_list_focused() {
        block
    } else {
        block.border_style(theme.flox_purple)
    };
    match app.env_mut().var_detail_state_mut() {
        VarDetailState::Raw => {
            let [text_area] = Layout::vertical([Constraint::Percentage(100)])
                .margin(1)
                .areas(detail_sub_area);
            let selected_var = app
                .env_mut()
                .selected_var_value()
                .unwrap_or("<No variable selected>".to_string());
            frame.render_widget(block, detail_sub_area);
            frame.render_widget(
                Paragraph::new(selected_var).wrap(Wrap { trim: false }),
                text_area,
            );
        }
        VarDetailState::Split { items, list_state } => {
            let [list_area, value_area] =
                Layout::vertical([Constraint::Fill(1), Constraint::Length(4)])
                    .spacing(1)
                    .areas(detail_sub_area);
            let var_list = List::new(items.iter().map(|s| s.as_str()))
                .block(block)
                .highlight_style(theme.highlighted_text);
            frame.render_stateful_widget(var_list, list_area, list_state);

            frame.render_widget(Block::bordered().title(" Selected "), value_area);
            let [value_area_inner] = Layout::vertical([Constraint::Percentage(100)])
                .margin(1)
                .areas(value_area);
            frame.render_widget(
                Paragraph::new(selected_detail_item).wrap(Wrap { trim: false }),
                value_area_inner,
            );
        }
    }
}

fn var_detail_block_title(state: &VarDetailState) -> Line<'static> {
    match state {
        VarDetailState::Raw => Line::from(vec![
            " ".into(),
            "Raw".set_style(Style::new().add_modifier(Modifier::UNDERLINED)),
            " / ".into(),
            "S".set_style(Style::new().add_modifier(Modifier::UNDERLINED)),
            "plit".into(),
            " ".into(),
        ]),
        VarDetailState::Split { .. } => Line::from(vec![
            " ".into(),
            "R".set_style(Style::new().add_modifier(Modifier::UNDERLINED)),
            "aw".into(),
            " / ".into(),
            "Split".set_style(Style::new().add_modifier(Modifier::UNDERLINED)),
            " ".into(),
        ]),
    }
}
