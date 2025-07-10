use std::path::PathBuf;

use anyhow::{Context, Error, anyhow, bail};
use ratatui::widgets::ListState;

use crate::app::{App, Event, NavEvent, Shell};

/// Represents the call site for a function in the shell's execution trace.
#[derive(Debug, Clone)]
pub struct CallCtx {
    /// The file in which the function was called.
    pub file: PathBuf,
    /// The line number in the file at which the function was called.
    pub line: usize,
    /// The function in which the function was called.
    pub function: String,
}

#[derive(Debug, Clone)]
pub struct CallFrame {
    pub ctx: CallCtx,
    pub lines: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default)]
pub struct CallStack {
    pub frames: Vec<CallFrame>,
}

#[derive(Debug, Clone, Default)]
pub struct TraceState {
    tracepoint: Option<String>,
    call_stack: Option<CallStack>,
    list_state: Option<ListState>,
}

impl TraceState {
    pub fn new(tracepoint: Option<String>, call_stack: Option<CallStack>) -> Self {
        let list_state = if call_stack.is_some() {
            let mut state = ListState::default();
            state.select_first();
            Some(state)
        } else {
            None
        };
        Self {
            tracepoint,
            call_stack,
            list_state,
        }
    }

    pub fn tracepoint(&self) -> Option<&String> {
        self.tracepoint.as_ref()
    }

    pub fn call_stack(&self) -> Option<&CallStack> {
        self.call_stack.as_ref()
    }

    pub fn list_state(&mut self) -> Option<&mut ListState> {
        self.list_state.as_mut()
    }
}

/// Parses our custom stack trace format for Bash and Zsh.
///
/// Stack frames are passed to the CLI as a string with the following contents:
/// <file abs path>:<line number of call site>:<function called>
/// <file abs path>:<line number of call site>:<function called>
/// ...
fn parse_bash_or_zsh_stack_trace(input: &str) -> Result<Vec<CallCtx>, Error> {
    let lines = input
        .split('\n')
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                None
            } else {
                Some(line.to_string())
            }
        })
        .collect::<Vec<_>>();
    let mut frames = Vec::new();
    for line in lines {
        let parts = line.split(':').collect::<Vec<_>>();
        if parts.len() != 3 {
            bail!("failed to parse stack trace");
        }
        let cf = CallCtx {
            file: PathBuf::from(parts[0]),
            line: parts[1]
                .parse::<usize>()
                .context("failed to parse line number as usize")?,
            function: parts[2].to_string(),
        };
        frames.push(cf);
    }
    Ok(frames)
}

/// Parses our custom Fish stack trace format, which is just the Fish
/// built-in stack trace format with newlines swapped for ';' characters
/// to avoid needing to deal with joining the list with newlines in Fish.
///
/// This is an example of the format:
/// ```
/// in function 'otherfunc';        called on line 8 of file ./run.fish;in function 'myfunction';        called on line 19 of file ./run.fish
/// ```
fn parse_fish_stack_trace(input: &str) -> Result<Vec<CallCtx>, Error> {
    let lines = input
        .split(';')
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                None
            } else {
                Some(line.to_string())
            }
        })
        .collect::<Vec<_>>();

    let mut frames = Vec::new();

    // The lines come in pairs, so we need to chunk them together.
    for chunk in lines.chunks(2) {
        let [func_line, callsite_line] = chunk else {
            bail!("uneven number of lines in fish stack trace")
        };
        // Now we split the following kind of line on `'` characters to grab
        // the function name:
        // in function 'myfunction'
        let function = func_line
            .split('\'')
            .nth(1)
            .ok_or(anyhow!("failed to extract function name"))?;

        // Now we split the callsite line on space characters and cross
        // our fingers that we counted correctly after trimming the leading
        // whitespace earlier.
        let line = callsite_line
            .split(' ')
            .nth(3)
            .ok_or(anyhow!("failed to extract line number"))
            .and_then(|s| s.parse::<usize>().context("failed to parse line number"))?;
        let file = callsite_line
            .split(' ')
            .nth(6)
            .ok_or(anyhow!("failed to extract file path"))
            .and_then(|s| std::path::absolute(s).context("failed to get absolute path of file"))?;
        let cf = CallCtx {
            file,
            line,
            function: function.to_string(),
        };
        frames.push(cf);
    }
    Ok(frames)
}

/// Loads a call stack from a stack trace and a shell type.
pub fn load_call_stack(input: &str, shell: Shell) -> Result<CallStack, Error> {
    let callsites = match shell {
        Shell::Bash => parse_bash_or_zsh_stack_trace(input),
        Shell::Zsh => parse_bash_or_zsh_stack_trace(input),
        Shell::Fish => parse_fish_stack_trace(input),
    }?;
    let mut frames = Vec::new();
    for call in callsites {
        let lines = std::fs::read_to_string(&call.file)
            .map(|s| {
                s.split('\n')
                    .map(|line| line.to_string())
                    .collect::<Vec<_>>()
            })
            .ok();
        let frame = CallFrame { ctx: call, lines };
        frames.push(frame);
    }
    Ok(CallStack { frames })
}

/// Handles navigation and interaction in the Vars screen.
pub fn handle_trace_event(app: &mut App, event: &Event) {
    let trace = app.trace_mut();
    let len = trace.call_stack().map(|cs| cs.frames.len());
    if let Some(list_state) = trace.list_state() {
        let len = len.unwrap();
        match event {
            Event::Nav(NavEvent::Up) => {
                if let Some(idx) = list_state.selected() {
                    if idx > 0 {
                        list_state.select_previous();
                    }
                }
            }
            Event::Nav(NavEvent::Down) => {
                if let Some(idx) = list_state.selected() {
                    if idx < len - 1 {
                        list_state.select_next();
                    }
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_bash_stack_trace() {
        let st = "
            foo:1:func1
            bar:2:func2
        ";
        let frames = parse_bash_or_zsh_stack_trace(st).unwrap();
        assert_eq!(frames.len(), 2);
        assert_eq!(frames[0].function, "func1".to_string());
        assert_eq!(frames[1].function, "func2".to_string());
    }

    #[test]
    fn parses_fish_stack_trace() {
        let st = "in function 'otherfunc';        called on line 8 of file ./run.fish;in function 'myfunction';        called on line 19 of file ./run.fish";
        let frames = parse_fish_stack_trace(st).unwrap();
        assert_eq!(frames.len(), 2);
        assert_eq!(frames[0].function, "otherfunc".to_string());
        assert_eq!(frames[0].line, 8);
        assert_eq!(frames[1].function, "myfunction".to_string());
        assert_eq!(frames[1].line, 19);
    }
}
