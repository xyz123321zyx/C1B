use std::{fmt, ops::Index};

use crate::{
    debugger,
    models::{Tab, TabManager},
};

impl TabManager {
    pub fn new() -> Self {
        let mut tabs = Vec::new();

        // Create the first tab
        let first_tab = Tab::new(0, "https://ztna14.cpcgw01.cachatto.co.in".to_string());
        tabs.push(first_tab);
        TabManager {
            tabs,
            active_tab_id: 0,
            next_id: 1,
            active_tabset_count: 1,
            tabset_count: 1,
            closed_tabset_count: 0,
        }
    }

    pub fn add_tab(&mut self, url: String) {
        let id = self.next_id;

        let tab = Tab::new(id, url);
        self.tabs.push(tab);

        self.tabset_count += 1;
        self.active_tabset_count += 1;
        self.next_id += 1;
        self.active_tab_id = id;

        debugger::StateDebugger::new_tab_created(id, None);
    }

    pub fn switch_tab(&mut self, tab_id: usize) {
        self.active_tab_id = tab_id;
    }

    pub fn close_tab(&mut self, tab_id: usize) {
        if let Some(tab) = self.tabs.get_mut(tab_id) {
            tab.is_closed = true;
            self.closed_tabset_count += 1;
            self.active_tabset_count -= 1;

            // if the closed tab was active, pick a new active tab
            if self.active_tab_id == tab_id {
                // try next tab
                if let Some((idx, _)) = self
                    .tabs
                    .iter()
                    .enumerate()
                    .skip(tab_id + 1)
                    .find(|(_, t)| !t.is_closed)
                {
                    self.active_tab_id = idx;
                }
                // else try previous tab
                else if let Some((idx, _)) = self
                    .tabs
                    .iter()
                    .enumerate()
                    .rev()
                    .skip(self.tabs.len() - tab_id)
                    .find(|(_, t)| !t.is_closed)
                {
                    self.active_tab_id = idx;
                }
            }
        }
    }
}

impl fmt::Display for TabManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "TabManager {{")?;
        writeln!(f, "    active_tab_id: {}", self.active_tab_id)?;
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
