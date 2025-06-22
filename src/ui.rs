use ratatui::Frame;

use crate::app::App;

pub fn draw_ui(app: &mut App, frame: &mut Frame) {
    frame.render_widget("Hello, World!", frame.area());
}
