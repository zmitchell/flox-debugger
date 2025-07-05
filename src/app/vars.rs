use crate::app::{App, Event, NavEvent};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VarsEvent {
    RawDetail,
    SplitDetail,
}

pub fn handle_vars_event(app: &mut App, event: &Event) {
    if let Event::Nav(nav_event) = event {
        match nav_event {
            NavEvent::Up => app.env_mut().var_list_state().select_previous(),
            NavEvent::Down => app.env_mut().var_list_state().select_next(),
            NavEvent::Left => {}
            NavEvent::Right => {}
            NavEvent::Select => {}
        }
    }
}
