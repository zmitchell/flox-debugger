mod app;
mod cli;
mod ui;

use anyhow::{Context, Error};
use clap::Parser;
use ratatui::{
    Terminal,
    crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    prelude::*,
};

use crate::{
    app::{App, run_app},
    cli::Cli,
};

fn main() -> Result<(), Error> {
    let args = Cli::parse();
    // Setup the terminal in a TUI-friendly state, displaying the TUI via
    // stderr instead of stdout since we'll print shell commands on stdout
    // when the debugger exits.
    enable_raw_mode().context("failed to enable raw mode")?;
    let mut stderr = std::io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)
        .context("failed to enter alternate screen mode")?;

    // Create and run the app.
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend).context("failed to get terminal")?;
    let mut app = App::new(&args);
    let res = run_app(&mut app, &mut terminal);

    // Restore the terminal to its normal state.
    disable_raw_mode().context("failed to disable raw mode")?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .context("failed to leave alternate screen mode")?;
    terminal.show_cursor().context("failed to restore cursor")?;

    // Handle any errors that came from the TUI.
    match res.as_ref() {
        Ok(_) => {
            app.print_output().context("failed to print commands")?;
        }
        _err => {
            res.context("debugger encountered an error")?;
        }
    }
    Ok(())
}
