use std::collections::HashMap;

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

use crate::app::{AppEvent, Event, ExitState, NavEvent, Screen, vars::VarsEvent};

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
    prompt: PromptKeyBindings,
    vars: VarsKeyBindings,
    trace: TraceKeyBindings,
    output: OutputKeyBindings,
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

    /// Returns the configured key bindings for the prompt screen.
    pub fn prompt(&self) -> PromptKeyBindings {
        self.prompt.clone()
    }

    /// Returns the configured key bindings for the vars screen.
    pub fn vars(&self) -> VarsKeyBindings {
        self.vars.clone()
    }

    /// Returns the configured key bindings for the trace screen.
    pub fn trace(&self) -> TraceKeyBindings {
        self.trace.clone()
    }

    /// Returns the configured key bindings for the output screen.
    pub fn output(&self) -> OutputKeyBindings {
        self.output.clone()
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
            // prev_tab,
        } = self.global;
        let existing = keymap.insert(exit, Event::App(AppEvent::ExitRequested));
        debug_assert!(existing.is_none());
        let existing = keymap.insert(next_tab, Event::App(AppEvent::NextTab));
        debug_assert!(existing.is_none());
        // let existing = keymap.insert(prev_tab, Event::App(AppEvent::PrevTab));
        // debug_assert!(existing.is_none());

        // Match on the screen and apply screen-specific keybindings once
        // we have them
        match screen {
            Screen::Home => {
                let HomeKeyBindings {} = self.home;
            }
            Screen::Vars => {
                let VarsKeyBindings {
                    next_var,
                    previous_var,
                    focus_var_list,
                    focus_var_detail,
                    raw_detail,
                    split_detail,
                } = self.vars;
                keymap.insert(next_var, Event::Nav(NavEvent::Down));
                keymap.insert(previous_var, Event::Nav(NavEvent::Up));
                keymap.insert(focus_var_list, Event::Nav(NavEvent::Left));
                keymap.insert(focus_var_detail, Event::Nav(NavEvent::Right));
                keymap.insert(raw_detail, Event::Vars(VarsEvent::RawDetail));
                keymap.insert(split_detail, Event::Vars(VarsEvent::SplitDetail));
            }
            Screen::Trace => {
                let TraceKeyBindings {
                    next_frame,
                    previous_frame,
                } = self.trace;
                keymap.insert(next_frame, Event::Nav(NavEvent::Down));
                keymap.insert(previous_frame, Event::Nav(NavEvent::Up));
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
        // FIXME: this doesn't work for some reason, haven't looked into why
        // let prev_tab = KeyEvent {
        //     code: KeyCode::Tab,
        //     modifiers: KeyModifiers::SHIFT,
        //     kind: KeyEventKind::Press,
        //     state: KeyEventState::NONE,
        // };
        Self {
            exit,
            next_tab,
            // prev_tab,
        }
    }
}

impl DisplayKeyBindings for GlobalKeyBindings {
    fn displayable(&self) -> Vec<(String, &'static str)> {
        vec![
            (self.exit.display_key_combo(), "Exit"),
            (self.next_tab.display_key_combo(), "Next Tab"),
            // (self.prev_tab.display_key_combo(), "Prev"),
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

#[derive(Debug, Clone, Default)]
pub struct PromptKeyBindings {}

impl DisplayKeyBindings for PromptKeyBindings {
    fn displayable(&self) -> Vec<(String, &'static str)> {
        vec![]
    }
}

#[derive(Debug, Clone)]
pub struct VarsKeyBindings {
    next_var: KeyEvent,
    previous_var: KeyEvent,
    focus_var_list: KeyEvent,
    focus_var_detail: KeyEvent,
    raw_detail: KeyEvent,
    split_detail: KeyEvent,
}

impl Default for VarsKeyBindings {
    fn default() -> Self {
        let next_var = KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        };
        let previous_var = KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        };
        let focus_var_list = KeyEvent {
            code: KeyCode::Left,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        };
        let focus_var_detail = KeyEvent {
            code: KeyCode::Right,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        };
        let raw_detail = KeyEvent {
            code: KeyCode::Char('r'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        };
        let split_detail = KeyEvent {
            code: KeyCode::Char('s'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        };
        Self {
            next_var,
            previous_var,
            focus_var_list,
            focus_var_detail,
            raw_detail,
            split_detail,
        }
    }
}

impl DisplayKeyBindings for VarsKeyBindings {
    fn displayable(&self) -> Vec<(String, &'static str)> {
        vec![("↑↓←→".to_string(), "Nav")]
    }
}

#[derive(Debug, Clone)]
pub struct TraceKeyBindings {
    pub next_frame: KeyEvent,
    pub previous_frame: KeyEvent,
}

impl DisplayKeyBindings for TraceKeyBindings {
    fn displayable(&self) -> Vec<(String, &'static str)> {
        vec![("↑↓".to_string(), "Nav")]
    }
}

impl Default for TraceKeyBindings {
    fn default() -> Self {
        let next_frame = KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        };
        let previous_frame = KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        };
        Self {
            next_frame,
            previous_frame,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct OutputKeyBindings {}

impl DisplayKeyBindings for OutputKeyBindings {
    fn displayable(&self) -> Vec<(String, &'static str)> {
        vec![]
    }
}
