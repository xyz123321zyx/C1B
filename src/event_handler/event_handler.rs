use crate::{event_handler::{NavBarEventHandler, TabEventHandler}, events::{Events, TabUIEvents}, models::C1BState};

pub struct EventHandler {}

impl EventHandler {
    pub fn handle_event(event: Events,state:& mut C1BState) {
        match event {
            Events::Tab(tab_event) =>{
                TabEventHandler::handle_tab_event(tab_event,state);
            },
            Events::NavBar(navbar_event) => {
                NavBarEventHandler::handle_navbar_event(navbar_event);
            },
        }
    }
}
