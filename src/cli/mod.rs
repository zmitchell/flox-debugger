use clap::Parser;

use crate::app::Shell;

#[derive(Debug, Clone, Parser)]
pub struct Cli {
    #[arg(
        short = 's',
        long = "shell",
        help = "Which shell the debugger was invoked from"
    )]
    pub shell: Shell,
}
