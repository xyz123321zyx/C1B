use crate::models::{C1BState, state};

pub struct StateDebugger {}

impl StateDebugger {
    pub fn state(state: &C1BState) {
        println!("State: {}", state);
    }

    pub fn active_tab_id(state: C1BState) {
        println!("Active tab id: {}", state.tabmanager.active_tab_id);
    }

    pub fn tab_clicked(index: usize, state: Option<&C1BState>) {
        if let Some(state) = state {
            println!("Clicked tab: {}\n Current State {}", index, state);
        } else {
            println!("Clicked tab: {}", index);
        }
    }

    pub fn clicked_close_button(index: usize, state: Option<&C1BState>) {
        if let Some(state) = state {
            println!("Clicked close button: {}\n Current State {}", index, state);
        } else {
            println!("Clicked close button: {}", index);
        }
    }

    pub fn new_tab_button_clicked(state: Option<&C1BState>) {
        if let Some(state) = state {
            println!("New tab button clicked\n Current State {}", state);
        } else {
            println!("New tab button clicked");
        }
    }

    pub fn new_tab_created(id: usize, state: Option<&C1BState>) {
        if let Some(state) = state {
            println!("New tab created: {}\n Current State {}", id, state);
        } else {
            println!("New tab created: {}", id);
        }
    }

}
