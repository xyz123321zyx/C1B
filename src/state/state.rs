use std::fmt;

use crate::models::{C1BState, Tab, TabManager, WebViewManager};

impl C1BState {
    pub fn new() -> Self {
        C1BState {
            tabmanager: TabManager::new(),
            scroll_to_end: false,
            webviewmanager:WebViewManager::new(),
        }
    }

    pub fn new_tab(&mut self,url: String) {
        self.tabmanager.add_tab(url);
    }

    pub fn switch_tab(&mut self, index: usize) {
        self.tabmanager.switch_tab(index);
    }

    pub fn close_tab(&mut self, index: usize) {
        self.tabmanager.close_tab(index);
    }

    pub fn set_scroll(&mut self,flag:bool){
        self.scroll_to_end=flag;
    }
}

impl fmt::Display for C1BState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "C1BState {{")?;
        writeln!(f, "  {}", self.tabmanager)?;
        write!(f, "}}")
    }
}
