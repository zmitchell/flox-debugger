use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::{Block, Paragraph, Wrap},
};

use crate::app::App;

pub fn render_output_screen(app: &mut App, frame: &mut Frame, area: Rect) {
    let [desc_area, output_area] = Layout::vertical([Constraint::Max(2), Constraint::Fill(1)])
        .margin(1)
        .spacing(1)
        .areas(area);

    let desc_text = Line::from(vec![
        "These commands will be sourced by your shell when the debugger exits.".into(),
    ]);
    let desc = Paragraph::new(desc_text).wrap(Wrap { trim: false });
    frame.render_widget(desc, desc_area);

    let block = Block::bordered().title(" Output ");
    frame.render_widget(block, output_area);

    let [output_area_inner] = Layout::vertical([Constraint::Percentage(100)])
        .margin(1)
        .areas(output_area);
    let output = Paragraph::new(app.output());
    frame.render_widget(output, output_area_inner);
}
