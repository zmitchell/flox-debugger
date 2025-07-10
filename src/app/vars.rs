use std::collections::HashMap;

use ratatui::widgets::ListState;

use crate::app::{App, Event, NavEvent};

#[derive(Debug, Clone)]
pub struct Env {
    vars: Vec<String>,
    values: Vec<String>,
    var_list_focused: bool,
    detail_state: VarDetailState,
    var_list_state: ListState,
}

impl Env {
    /// Initializes the `Env` state by reading the environment.
    pub fn new() -> Self {
        let (vars, values) = {
            let mut vars_and_values = std::env::vars().collect::<Vec<_>>();
            vars_and_values.sort_by_key(|(var, _value)| var.clone());
            vars_and_values
                .into_iter()
                .collect::<(Vec<String>, Vec<String>)>()
        };
        let list_state = initial_list_state(&vars);
        Self {
            vars,
            var_list_focused: true,
            values,
            detail_state: VarDetailState::Raw,
            var_list_state: list_state,
        }
    }

    /// Initializes the `Env` state with a provided set of environment variables.
    pub fn with_env(env: &HashMap<String, String>) -> Self {
        let (vars, values) = env
            .iter()
            .map(|(var, value)| (var.clone(), value.clone()))
            .collect::<(Vec<String>, Vec<String>)>();
        let list_state = initial_list_state(&vars);
        Self {
            vars,
            var_list_focused: true,
            values,
            detail_state: VarDetailState::Raw,
            var_list_state: list_state,
        }
    }

    /// Returns a slice of the environment variable names.
    pub fn vars(&self) -> &[String] {
        self.vars.as_slice()
    }

    /// Returns the var list state for stateful rendering.
    pub fn var_list_state(&mut self) -> &mut ListState {
        &mut self.var_list_state
    }

    /// Returns how this variable is represented.
    pub fn var_detail_state(&self) -> &VarDetailState {
        &self.detail_state
    }

    /// Returns how this variable is represented.
    pub fn var_detail_state_mut(&mut self) -> &mut VarDetailState {
        &mut self.detail_state
    }

    /// Returns whether the variable list is focused. When false, the detail
    /// view is focused.
    pub fn var_list_focused(&self) -> bool {
        self.var_list_focused
    }

    /// Returns the selected variable name.
    pub fn selected_var(&self) -> Option<String> {
        self.var_list_state
            .selected()
            .map(|idx| self.vars[idx].clone())
    }

    /// Returns the value of the selected variable.
    pub fn selected_var_value(&self) -> Option<String> {
        self.var_list_state
            .selected()
            .map(|idx| self.values[idx].clone())
    }

    /// Returns the split view of the selected variable.
    pub fn selected_var_split_value(&self) -> Option<Vec<String>> {
        self.var_list_state
            .selected()
            .map(|idx| self.values[idx].clone())
            .map(|s| {
                std::env::split_paths(&s)
                    .map(|path| path.to_string_lossy().to_string())
                    .collect::<Vec<_>>()
            })
    }

    /// Returns the selected list item in the split detail view.
    pub fn selected_detail_item(&self) -> Option<String> {
        self.detail_state.selected_split_item()
    }
}

/// Returns an initialized list state that differs based on whether the
/// list of environment variables is empty or not (as a defensive measure).
fn initial_list_state(list_items: &[String]) -> ListState {
    if list_items.is_empty() {
        ListState::default()
    } else {
        let mut state = ListState::default();
        state.select_first();
        state
    }
}

#[derive(Debug, Clone)]
pub enum VarDetailState {
    Raw,
    Split {
        items: Vec<String>,
        list_state: ListState,
    },
}

impl VarDetailState {
    /// Initializes the split state from a list of items.
    pub fn init_split(items: &[String]) -> Self {
        let list_state = initial_list_state(items);
        Self::Split {
            items: items.to_vec(),
            list_state,
        }
    }

    /// Returns the selected item index in the split view if it exists.
    pub fn selected_split_item(&self) -> Option<String> {
        match self {
            VarDetailState::Raw => None,
            VarDetailState::Split { items, list_state } => {
                list_state.selected().map(|idx| items[idx].clone())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VarsEvent {
    RawDetail,
    SplitDetail,
}

/// Handles navigation and interaction in the Vars screen.
pub fn handle_vars_event(app: &mut App, event: &Event) {
    if let Event::Nav(nav_event) = event {
        match nav_event {
            NavEvent::Up => {
                if app.env().var_list_focused {
                    let list_state = app.env_mut().var_list_state();
                    if let Some(idx) = list_state.selected() {
                        if idx > 0 {
                            list_state.select_previous();
                        }
                    }
                } else if let VarDetailState::Split {
                    ref mut list_state, ..
                } = app.env_mut().detail_state
                {
                    if let Some(idx) = list_state.selected() {
                        if idx > 0 {
                            list_state.select_previous()
                        }
                    }
                }
            }
            NavEvent::Down => {
                if app.env().var_list_focused {
                    let len = app.env().vars().len();
                    let list_state = app.env_mut().var_list_state();
                    if let Some(idx) = list_state.selected() {
                        if idx < len - 1 {
                            list_state.select_next();
                        }
                    }
                } else if let VarDetailState::Split {
                    ref mut list_state,
                    ref items,
                } = app.env_mut().detail_state
                {
                    if let Some(idx) = list_state.selected() {
                        if idx < items.len() - 1 {
                            list_state.select_next()
                        }
                    }
                }
            }
            NavEvent::Left => {
                app.env_mut().var_list_focused = true;
            }
            NavEvent::Right => {
                app.env_mut().var_list_focused = false;
            }
            NavEvent::Select => {}
        }
    } else if let Event::Vars(vars_event) = event {
        match vars_event {
            VarsEvent::RawDetail => {
                let detail_state = app.env_mut().var_detail_state_mut();
                *detail_state = VarDetailState::Raw;
            }
            VarsEvent::SplitDetail => {
                let var_values = app.env().selected_var_split_value().unwrap().to_vec();
                let detail_state = app.env_mut().var_detail_state_mut();
                *detail_state = VarDetailState::init_split(&var_values);
            }
        }
    }
}
