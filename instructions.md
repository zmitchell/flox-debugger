# Step-by-Step Instructions: Adding Navigation Events

## Overview
This guide details how to add navigation events (up, down, left, right) to the flox-debugger application and use them to switch between Ok/Cancel buttons in the exit modal.

## Step 1: Define Navigation Event Types

### 1.1 Create NavEvent Enum
In `src/app/mod.rs`, add a new enum after the existing enums:

```rust
/// Navigation directions for UI elements
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavEvent {
    Up,
    Down,
    Left,
    Right,
    Select,  // For Enter key
}
```

### 1.2 Update Event Enum
Modify the `Event` enum (around line 182) to include navigation:

```rust
#[derive(Debug, Clone)]
pub enum Event {
    App(AppEvent),
    Nav(NavEvent),  // Add this new variant
}
```

## Step 2: Add Arrow Key Bindings

### 2.1 Update KeyBindings Struct
In `src/app/key_bindings.rs`, modify the `KeyBindings` struct to handle navigation events in the `current_keymap` method:

```rust
impl KeyBindings {
    pub fn current_keymap(
        &self,
        screen: &Screen,
        exit_state: &ExitState,
    ) -> HashMap<KeyEvent, Event> {
        let mut keymap = HashMap::new();
        
        // Handle modal-specific bindings
        if let ExitState::PresentModal { highlighted_option } = exit_state {
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
            
            // Return early - no other keys should work when modal is shown
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
```

### 2.2 Import NavEvent
Add the import at the top of `key_bindings.rs`:

```rust
use crate::app::{AppEvent, Event, ExitState, NavEvent, Screen};
```

## Step 3: Handle Navigation Events in Exit Modal

### 3.1 Add Required Methods to App Struct
First, add the necessary accessor methods to the App struct:

```rust
impl App {
    /// Get a mutable reference to the exit state
    pub fn exit_state_mut(&mut self) -> &mut ExitState {
        &mut self.exit_state
    }
    
    /// Set the exit state
    pub fn set_exit_state(&mut self, state: ExitState) {
        self.exit_state = state;
    }
}
```

### 3.2 Update Event Handler
In `src/app/mod.rs`, update the existing `handle_event` function to handle both App and Nav events:

```rust
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
            // ... rest of the existing app event handlers
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
                                app.set_exit_state(ExitState::None);
                                return true; // Signal to exit the application
                            }
                            ExitOption::Cancel => {
                                app.set_exit_state(ExitState::None);
                            }
                        }
                    }
                }
            }
            false
        }
    }
}
```

Note: No changes are needed to the `run_app` function. It already correctly passes events to `handle_event` which returns a boolean indicating whether to exit.

## Step 4: Visual Feedback for Button Selection

The visual feedback is already implemented in `src/ui/exit_popup.rs`. The `render_exit_popup` function uses the theme's `selected_option` style for the highlighted button:

```rust
// Line 37-38: The highlighted button gets the selected style
let ok_style = if highlighted_option == &ExitOption::Ok {
    theme.selected_option
} else {
    Style::default()
};
```

## Step 5: Testing the Implementation

1. **Build the project**: `cargo check`
2. **Test the navigation** (when you run the application):
   - Press `Ctrl+Q` to trigger the exit modal
   - Use arrow keys (←, →) to switch between Ok and Cancel
   - Press Enter to select the highlighted option
   - Verify that Cancel closes the modal and Ok exits the application

## Step 6: Future Enhancements

This navigation system can be extended for:

1. **List Navigation**: Use Up/Down for navigating through lists in other screens
2. **Context-Aware Navigation**: Different key bindings based on current screen
3. **Focus Management**: Track which UI element has focus across different screens
4. **Accessibility**: Add screen reader support for navigation events

## Implementation Notes

- The navigation events are designed to be generic and reusable across different UI contexts
- The event handling is centralized in the `handle_event` method for consistency
- The visual feedback uses the existing theme system for a consistent look
- Arrow keys provide intuitive navigation that works with both horizontal (left/right) and vertical (up/down) layouts
