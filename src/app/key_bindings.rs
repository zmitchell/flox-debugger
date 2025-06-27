use std::collections::HashMap;

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

use crate::app::{AppEvent, Event, ExitState, NavEvent, Screen};

pub trait DisplayKeyBindings {
    /// Returns an iterator over (keys, description) pairs
    /// for each keybinding.
    fn displayable(&self) -> Vec<(String, &'static str)>;
}

pub trait DisplayKeyCombo {
    /// Returns a user-facing representation of a key combination.
    fn display_key_combo(&self) -> String;
}

impl DisplayKeyCombo for KeyEvent {
    fn display_key_combo(&self) -> String {
        let mut parts = vec![];
        if self.modifiers.contains(KeyModifiers::SHIFT) {
            parts.push("⇧".to_string());
        }
        if self.modifiers.contains(KeyModifiers::CONTROL) {
            parts.push("Ctrl".to_string());
        }
        if self.modifiers.contains(KeyModifiers::ALT) {
            parts.push("Alt".to_string());
        }
        if self.modifiers.contains(KeyModifiers::SUPER) {
            parts.push("Super".to_string());
        }
        let key = match self.code {
            KeyCode::Backspace => "Backspace".to_string(),
            KeyCode::Enter => "Enter".to_string(),
            KeyCode::Left => "←".to_string(),
            KeyCode::Right => "→".to_string(),
            KeyCode::Up => "↑".to_string(),
            KeyCode::Down => "↓".to_string(),
            KeyCode::Home => "Home".to_string(),
            KeyCode::End => "End".to_string(),
            KeyCode::PageUp => "PgUp".to_string(),
            KeyCode::PageDown => "PgDown".to_string(),
            KeyCode::Tab => "Tab".to_string(),
            KeyCode::BackTab => "⇧+Tab".to_string(),
            KeyCode::Delete => "Del".to_string(),
            KeyCode::F(n) => format!("F{n}"),
            KeyCode::Char(c) => format!("{}", c.to_uppercase()),
            KeyCode::Esc => "Esc".to_string(),
            _ => "".to_string(),
        };
        parts.push(key);
        parts.join("+")
    }
}

/// The complete set of configured key bindings for the application.
#[derive(Debug, Clone, Default)]
pub struct KeyBindings {
    /// The key bindings available at all times.
    global: GlobalKeyBindings,
    home: HomeKeyBindings,
}

impl KeyBindings {
    /// Returns the configured global key bindings.
    pub fn global(&self) -> GlobalKeyBindings {
        self.global.clone()
    }

    /// Returns the configured key bindings for the home screen.
    pub fn home(&self) -> HomeKeyBindings {
        self.home.clone()
    }

    /// Returns a mapping from keycode to application event given the
    /// current screen.
    pub fn current_keymap(
        &self,
        screen: &Screen,
        exit_state: &ExitState,
    ) -> HashMap<KeyEvent, Event> {
        let mut keymap = HashMap::new();

        // Handle modal-specific bindings
        if let ExitState::PresentModal { .. } = exit_state {
            // When modal is presented, only add navigation keys (no global keys)
            let left_arrow = KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            };
            let existing = keymap.insert(left_arrow, Event::Nav(NavEvent::Left));
            debug_assert!(existing.is_none());

            let right_arrow = KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            };
            let existing = keymap.insert(right_arrow, Event::Nav(NavEvent::Right));
            debug_assert!(existing.is_none());

            // Add Enter key for selection
            let enter = KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            };
            let existing = keymap.insert(enter, Event::Nav(NavEvent::Select));
            debug_assert!(existing.is_none());

            // Return early, no other keys should work when modal is shown
            return keymap;
        }

        // Global keybindings (only when modal is not shown)
        let GlobalKeyBindings {
            exit,
            next_tab,
            prev_tab,
        } = self.global;
        let existing = keymap.insert(exit, Event::App(AppEvent::ExitRequested));
        debug_assert!(existing.is_none());
        let existing = keymap.insert(next_tab, Event::App(AppEvent::NextTab));
        debug_assert!(existing.is_none());
        let existing = keymap.insert(prev_tab, Event::App(AppEvent::PrevTab));
        debug_assert!(existing.is_none());

        match screen {
            Screen::Home => {
                let HomeKeyBindings {} = self.home;
            }
            _ => {}
        }
        keymap
    }
}

#[derive(Debug, Clone)]
pub struct GlobalKeyBindings {
    exit: KeyEvent,
    next_tab: KeyEvent,
    prev_tab: KeyEvent,
}

impl Default for GlobalKeyBindings {
    fn default() -> Self {
        let exit = KeyEvent {
            code: KeyCode::Char('q'),
            // modifiers: KeyModifiers::CONTROL,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        };
        let next_tab = KeyEvent {
            code: KeyCode::Tab,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        };
        let prev_tab = KeyEvent {
            code: KeyCode::BackTab,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        };
        Self {
            exit,
            next_tab,
            prev_tab,
        }
    }
}

impl DisplayKeyBindings for GlobalKeyBindings {
    fn displayable(&self) -> Vec<(String, &'static str)> {
        vec![
            (self.exit.display_key_combo(), "Exit"),
            (self.next_tab.display_key_combo(), "Next"),
            (self.prev_tab.display_key_combo(), "Prev"),
        ]
    }
}

#[derive(Debug, Clone, Default)]
pub struct HomeKeyBindings {}

impl DisplayKeyBindings for HomeKeyBindings {
    fn displayable(&self) -> Vec<(String, &'static str)> {
        vec![]
    }
}
