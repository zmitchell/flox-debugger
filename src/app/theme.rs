use ratatui::style::{Color, Modifier, Style};

#[derive(Debug, Clone)]
pub struct Theme {
    pub flox_purple: Style,
    pub fg: Style,
    pub fg_dim: Style,
    pub selected_option: Style,
}

impl Theme {
    fn new() -> Self {
        Theme {
            flox_purple: Style::default().fg(Color::Rgb(175, 135, 255)),
            fg: Style::default(),
            fg_dim: Style::default().add_modifier(Modifier::DIM),
            selected_option: Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme::new()
    }
}
