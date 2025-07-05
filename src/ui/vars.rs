use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, List},
};

use crate::app::App;

pub fn render_vars_screen(app: &mut App, frame: &mut Frame, area: Rect) {
    let theme = app.theme();
    let [var_list_area, detail_area] =
        Layout::horizontal([Constraint::Percentage(33), Constraint::Percentage(67)])
            .margin(1)
            .areas(area);

    let env = app.env_mut();
    let vars = env.vars().to_vec();
    let values = env.var_values().to_vec();

    let var_list = List::new(vars)
        .block(Block::bordered())
        .highlight_style(theme.highlighted_text);
    frame.render_stateful_widget(var_list, var_list_area, env.var_list_state());
}
