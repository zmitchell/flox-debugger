pub mod key_bindings;
pub mod theme;
pub mod vars;

use std::{collections::HashMap, io::Write, str::FromStr};

use anyhow::{Context, Error, anyhow};
use ratatui::{
    Terminal,
    crossterm::event::{self, Event as TermEvent},
    prelude::*,
};

use crate::{
    app::{
        key_bindings::KeyBindings,
        theme::Theme,
        vars::{Env, VarsEvent, handle_vars_event},
    },
    cli::Cli,
    ui::draw_ui,
};

pub const TRACEPOINT_VAR_NAME: &str = "FLOX_DBG_TRACEPOINT";

#[derive(Debug)]
pub struct App {
    env: Env,
    screen: Screen,
    shell: Shell,
    output: String,
    theme: Theme,
    key_bindings: KeyBindings,
    exit_state: ExitState,
}

impl App {
    pub fn new(args: &Cli) -> Self {
        let env = Env::new();
        Self {
            env,
            screen: Screen::Home,
            shell: args.shell,
            output: Self::initial_output(args.shell),
            theme: Theme::default(),
            key_bindings: KeyBindings::default(),
            exit_state: ExitState::default(),
        }
    }

    /// Initialize the app with a specific set of environment variables.
    #[expect(dead_code)]
    fn with_env(mut self, env: &HashMap<String, String>) -> Self {
        self.env = Env::with_env(env);
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

    /// Returns the initial output that will be sourced when the debugger exits.
    fn initial_output(shell: Shell) -> String {
        let tracepoint = std::env::var(TRACEPOINT_VAR_NAME).unwrap_or_default();
        Self::initial_output_inner(shell, &tracepoint)
    }

    fn initial_output_inner(shell: Shell, tracepoint_var_value: &str) -> String {
        match tracepoint_var_value {
            "all" | "" => String::new(),
            _other => match shell {
                Shell::Bash => format!("unset {TRACEPOINT_VAR_NAME}\n"),
                Shell::Zsh => format!("unset {TRACEPOINT_VAR_NAME}\n"),
                Shell::Fish => format!("set -e {TRACEPOINT_VAR_NAME}\n"),
            },
        }
    }

    /// Returns a copy of the output commands.
    pub fn output(&self) -> String {
        self.output.clone()
    }

    /// Prints the commands that the user's shell should source
    /// after the debugger exits.
    pub fn print_output(&self) -> Result<(), Error> {
        Self::print_cmds_inner(&self.output, &mut std::io::stdout())
            .context("failed to write commands")?;
        Ok(())
    }

    /// Prints the commands that the user's shell should source to the specified
    /// buffer in the specified shell dialect.
    fn print_cmds_inner(output: &str, stream: &mut impl Write) -> Result<(), Error> {
        stream
            .write_all(output.as_bytes())
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

    /// Returns true if the application is presenting the exit modal.
    pub fn is_displaying_exit_modal(&self) -> bool {
        matches!(self.exit_state, ExitState::PresentModal { .. })
    }

    /// Get a mutable reference to the exit state
    pub fn exit_state_mut(&mut self) -> &mut ExitState {
        &mut self.exit_state
    }

    /// Set the exit state
    pub fn set_exit_state(&mut self, state: ExitState) {
        self.exit_state = state;
    }

    /// Switches to the next tab
    pub fn next_tab(&mut self) {
        self.screen = self.screen.next_tab()
    }

    /// Returns the current environment.
    pub fn env(&self) -> &Env {
        &self.env
    }

    /// Returns a mutable reference to the current environment.
    pub fn env_mut(&mut self) -> &mut Env {
        &mut self.env
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

    /// Returns the next tab
    pub fn next_tab(&self) -> Self {
        use Screen::*;
        match self {
            Home => Prompt,
            Prompt => Vars,
            Vars => Trace,
            Trace => Output,
            Output => Home,
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

impl FromStr for Shell {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bash" => Ok(Shell::Bash),
            "zsh" => Ok(Shell::Zsh),
            "fish" => Ok(Shell::Fish),
            _ => Err(anyhow!("unrecognized shell: {s}")),
        }
    }
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
    Nav(NavEvent),
    Vars(VarsEvent),
}

#[derive(Debug, Clone)]
pub enum AppEvent {
    /// The user requested that the next tab be displayed.
    NextTab,
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
    let should_exit = false;
    if app.is_displaying_exit_modal() {
        let should_exit = handle_exit_state(app, event);
        return should_exit;
    }
    if let Event::App(app_event) = event {
        match app_event {
            AppEvent::ExitRequested => {
                app.set_exit_state(ExitState::PresentModal {
                    highlighted_option: ExitOption::Cancel,
                });
            }
            AppEvent::NextTab => {
                app.next_tab();
            }
        }
        return should_exit;
    }
    match app.screen() {
        Screen::Home => {}
        Screen::Prompt => {}
        Screen::Vars => handle_vars_event(app, event),
        Screen::Trace => {}
        Screen::Output => {}
    }
    should_exit
}

/// Handles events when the user is being presented the exit modal.
fn handle_exit_state(app: &mut App, event: &Event) -> bool {
    if let ExitState::PresentModal { highlighted_option } = app.exit_state_mut() {
        if let Event::Nav(nav_event) = event {
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
    }
    false
}
