use std::fmt;

use crate::{
    models::{Tab, TabManager},
    ui::NavBarUI,
};

use tokio::sync::mpsc;

pub struct C1BState {
    pub tabmanager: TabManager,
    pub scroll_to_end: bool,
    // pub navbar:NavBarUI
    // tab_msg_sender:mpsc::Sender<Msg>,
    // tab_msg_reciever:Option<mpsc::Receiver<Msg>>,
    // resource_manager:Option<String>
}

impl C1BState {
    pub fn new() -> Self {
        C1BState {
            tabmanager: TabManager::new(),
            scroll_to_end: false,
            // navbar:NavBarUI::new()
        }
    }
}

impl fmt::Display for C1BState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "C1BState {{")?;
        writeln!(f, "  {}", self.tabmanager)?;
        write!(f, "}}")
    }
}

impl fmt::Display for TabManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "TabManager {{")?;
        writeln!(f, "    active_tab_id: {}", self.active_tab_id)?;
        writeln!(f, "    tab_count: {}", self.tab_count)?;
        writeln!(f, "    next_id: {}", self.next_id)?;
        writeln!(f, "    active_tabset_count: {}", self.active_tabset_count)?;
        writeln!(f, "    tabset_count: {}", self.tabset_count)?;
        writeln!(f, "    closed_tabset_count: {}", self.closed_tabset_count)?;
        writeln!(f, "    tabs: [")?;
        for tab in &self.tabs {
            writeln!(f, "      {},", tab)?;
        }
        write!(f, "    ]\n  }}")
    }
}


impl fmt::Display for Tab {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let title = self.title.as_deref().unwrap_or("<untitled>");
        write!(f, "Tab {{ id: {}, title: \"{}\" }}", self.id, title)
    }
}
