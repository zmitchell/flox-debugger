use clap::Parser;

use crate::app::Shell;

#[derive(Debug, Clone, Parser)]
pub struct Cli {
    #[arg(long = "shell", help = "Which shell the debugger was invoked from")]
    pub shell: Shell,
    #[arg(
        long = "tracepoint",
        help = "The name of the tracepoint the debugger paused at"
    )]
    pub tracepoint: Option<String>,
    #[arg(long = "call-stack", help = "A call stack of shell execution")]
    pub call_stack: Option<String>,
}
