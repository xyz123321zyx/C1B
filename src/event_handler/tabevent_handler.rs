use crate::{
    debugger, events::{Events, TabUIEvents}, models::{state, C1BState}
};

pub struct TabEventHandler;

impl TabEventHandler {
    pub fn handle_tab_event(tab_event: TabUIEvents, state: &mut C1BState) {
        match tab_event {
            TabUIEvents::NewTab(url) => {
                state.new_tab(url);
            }
            TabUIEvents::CloseTab(index) => {
                state.close_tab(index);
                // debugger::StateDebugger::print_state(state);
            }
            TabUIEvents::SwitchTab(index) => {
                state.switch_tab(index);
            }
            TabUIEvents::RestoreTab(index) => {
                println!("{:?}", index);
            }
            TabUIEvents::RestoreAllTab => {},
            TabUIEvents::Scroll(flag)=>{
                state.set_scroll(flag);
            }
        }
    }
}
