use ratatui::style::{Color, Modifier, Style};

#[derive(Debug, Clone)]
pub struct Theme {
    pub flox_purple: Style,
    pub fg: Style,
    pub fg_dim: Style,
    pub selected_tab: Style,
    pub highlighted_text: Style,
}

impl Theme {
    fn new() -> Self {
        Theme {
            flox_purple: Style::default().fg(Color::Rgb(175, 135, 255)),
            fg: Style::default(),
            fg_dim: Style::default().add_modifier(Modifier::DIM),
            selected_tab: Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::UNDERLINED)
                .add_modifier(Modifier::BOLD),
            highlighted_text: Style::default().bg(Color::White).fg(Color::Black),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme::new()
    }
}
