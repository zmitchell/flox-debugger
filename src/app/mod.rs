pub mod key_bindings;
pub mod theme;
use std::{collections::HashMap, io::Write};

use anyhow::{Context, Error};
use ratatui::{
    Terminal,
    crossterm::event::{self, Event as TermEvent},
    prelude::*,
};

use crate::{
    app::{key_bindings::KeyBindings, theme::Theme},
    ui::draw_ui,
};

#[derive(Debug)]
pub struct App {
    env: HashMap<String, String>,
    cmds: Vec<Cmd>,
    screen: Screen,
    shell: Shell,
    theme: Theme,
    key_bindings: KeyBindings,
    exit_state: ExitState,
}

impl App {
    pub fn new() -> Self {
        let env = std::env::vars().collect::<HashMap<String, String>>();
        Self {
            env,
            cmds: Vec::new(),
            screen: Screen::Home,
            shell: Shell::Fish,
            theme: Theme::default(),
            key_bindings: KeyBindings::default(),
            exit_state: ExitState::default(),
        }
    }

    /// Initialize the app with a specific set of environment variables.
    #[expect(dead_code)]
    fn with_env(mut self, env: HashMap<String, String>) -> Self {
        self.env = env;
        self
    }

    /// Initialize the app with some commands pre-populated.
    #[expect(dead_code)]
    fn with_cmds(mut self, cmds: &[Cmd]) -> Self {
        self.cmds = cmds.to_vec();
        self
    }

    /// Initialize the app with a specific starting screen.
    #[expect(dead_code)]
    fn with_screen(mut self, screen: Screen) -> Self {
        self.screen = screen;
        self
    }

    /// Initialize the app to generate commands for a specific shell.
    #[expect(dead_code)]
    fn with_shell(mut self, shell: Shell) -> Self {
        self.shell = shell;
        self
    }

    /// Prints the commands that the user's shell should source
    /// after the debugger exits.
    pub fn print_output(&self) -> Result<(), Error> {
        Self::print_cmds_inner(&self.cmds, self.shell, &mut std::io::stdout())
            .context("failed to write commands")?;
        Ok(())
    }

    /// Prints the commands that the user's shell should source to the specified
    /// buffer in the specified shell dialect.
    fn print_cmds_inner(cmds: &[Cmd], shell: Shell, out: &mut impl Write) -> Result<(), Error> {
        out.write_all("hello world".as_bytes())
            .context("failed to write commands to buffer")?;
        Ok(())
    }

    /// Returns the current screen.
    pub fn screen(&self) -> Screen {
        self.screen.clone()
    }

    /// Returns the configured theme.
    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }

    /// Returns the configured key bindings.
    pub fn key_bindings(&self) -> KeyBindings {
        self.key_bindings.clone()
    }

    /// Returns the current exit status.
    pub fn exit_state(&self) -> ExitState {
        self.exit_state.clone()
    }

    /// Get a mutable reference to the exit state
    pub fn exit_state_mut(&mut self) -> &mut ExitState {
        &mut self.exit_state
    }

    /// Set the exit state
    pub fn set_exit_state(&mut self, state: ExitState) {
        self.exit_state = state;
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum ExitState {
    #[default]
    NotExiting,
    PresentModal {
        highlighted_option: ExitOption,
    },
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum ExitOption {
    Ok,
    #[default]
    Cancel,
}

#[derive(Debug, Clone)]
pub enum Screen {
    Home,
    Prompt,
    Vars,
    Trace,
    Output,
}

impl Screen {
    /// Returns the index of the current tab to determine which tab
    /// to highlight in the UI.
    pub fn tab_index(&self) -> usize {
        match self {
            Screen::Home => 0,
            Screen::Prompt => 1,
            Screen::Vars => 2,
            Screen::Trace => 3,
            Screen::Output => 4,
        }
    }
}

impl std::fmt::Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Screen::Home => write!(f, "Home"),
            Screen::Prompt => write!(f, "Prompt"),
            Screen::Vars => write!(f, "Vars"),
            Screen::Trace => write!(f, "Trace"),
            Screen::Output => write!(f, "Output"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
}

/// A generic shell command.
#[derive(Debug, Clone)]
enum Cmd {
    /// Set a variable with global scope. This is called `export`
    /// in most shells.
    SetGlobalVar {
        /// The name of the variable to set.
        name: String,
        /// The value to give the variable (may contain expansions).
        value: String,
    },
    /// Unset a variable.
    Unset { name: String },
}

/// Navigation directions for UI elements
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavEvent {
    Up,
    Down,
    Left,
    Right,
    Select, // For Enter key
}

#[derive(Debug, Clone)]
pub enum Event {
    App(AppEvent),
    Nav(NavEvent), // Add this new variant
}

#[derive(Debug, Clone)]
pub enum AppEvent {
    /// The user requested that the next tab be displayed.
    NextTab,
    /// The user requested that the previous tab be displayed.
    PrevTab,
    /// The user requested that the application exit.
    ExitRequested,
}

pub fn run_app<B: Backend>(app: &mut App, terminal: &mut Terminal<B>) -> Result<(), Error> {
    loop {
        terminal
            .draw(|frame| draw_ui(app, frame))
            .context("failed to draw UI")?;

        let screen = app.screen();
        if let TermEvent::Key(key) = event::read().context("failed to read incoming events")? {
            let keymap = app
                .key_bindings()
                .current_keymap(&screen, &app.exit_state());
            if let Some(event) = keymap.get(&key) {
                let should_exit = handle_event(app, event);
                if should_exit {
                    break;
                }
            }
        }
    }
    Ok(())
}

/// Modifies the application state in response to an event, returning a boolean
/// indicating whether the application should exit.
fn handle_event(app: &mut App, event: &Event) -> bool {
    match event {
        Event::App(app_event) => match app_event {
            AppEvent::ExitRequested => {
                app.set_exit_state(ExitState::PresentModal {
                    highlighted_option: ExitOption::Cancel,
                });
                false
            }
            _ => false,
        },
        Event::Nav(nav_event) => {
            // Handle navigation only when modal is present
            if let ExitState::PresentModal { highlighted_option } = app.exit_state_mut() {
                match nav_event {
                    NavEvent::Left | NavEvent::Right => {
                        // Toggle between Ok and Cancel
                        *highlighted_option = match highlighted_option {
                            ExitOption::Ok => ExitOption::Cancel,
                            ExitOption::Cancel => ExitOption::Ok,
                        };
                    }
                    // Ignore up/down events in the exit modal
                    NavEvent::Up | NavEvent::Down => {}
                    NavEvent::Select => {
                        match highlighted_option {
                            ExitOption::Ok => {
                                app.set_exit_state(ExitState::NotExiting);
                                return true; // Signal to exit the application
                            }
                            ExitOption::Cancel => {
                                app.set_exit_state(ExitState::NotExiting);
                            }
                        }
                    }
                }
            }
            false
        }
    }
}
