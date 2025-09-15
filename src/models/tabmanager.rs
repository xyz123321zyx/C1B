use crate::{debugger, models::Tab};

use std::{fmt::format, sync::Arc};
use tokio::sync::Mutex;

pub struct TabManager {
    pub tabs: Vec<Tab>,
    pub active_tab_id: usize,
    pub tab_count: usize,
    pub next_id: usize,
    pub active_tabset_count: usize,
    pub tabset_count: usize,
    pub closed_tabset_count: usize,
}

impl TabManager {
    pub fn new() -> Self {
        let mut tabs = Vec::new();

        // Create the first tab
        let first_tab = Tab::new(
            1,
            "Tab 1".to_string(),
            "https://ztna14.cpcgw01.cachatto.co.in".to_string(),
            "https://ztna14.cpcgw01.cachatto.co.in/favicon.ico".to_string(),
        );
        tabs.push(first_tab);
        // Add 9 more tabs using a loop
        for i in 2..=10 {
            let tab = Tab::new(
                i,
                format!("Tab {}", i),
                format!("https://example.com/{}", i),
                "https://ztna14.cpcgw01.cachatto.co.in/favicon.ico".to_string(),
            );
            tabs.push(tab);
        }
        println!("TabManager created with {} tabs", tabs.len());

        TabManager {
            tabs,
            active_tab_id: 0, // first tab starts active
            tab_count: 1,     // currently only 1 tab
            next_id: 1,       // next tab created will get id 2
            active_tabset_count: 1,
            tabset_count: 1,
            closed_tabset_count: 0,
        }
    }

    pub fn add_tab(&mut self, title: String, url: String, favicon_url: String) {
        let id = self.next_id;

        let tab = Tab::new(id, title, url, favicon_url);
        self.tabs.push(tab);

        self.tab_count += 1;
        self.next_id += 1;

        debugger::StateDebugger::new_tab_created(id, None);
        
    }
}

// impl TabManager{
//     pub fn new() /*-> TabManager*/{
//         // TabManager{
//         //     tabs : Vec::new(),
//         //     selected_tab_id : 0,
//         //     tab_count : 0,
//         //     next_id : 0,
//         //     active_tabset : 0,
//         //     active_tabset_count : 0,
//         //     tabset_count : 0,
//         //     closed_tabset_count : 0
//         // }
//     }

//     pub fn add_tab(){

//     }

//     pub fn close_tab(){

//     }

//     pub fn get_active_tab(){

//     }

//     pub fn get_active_tabset(){

//     }
//     pub fn get_tab_count(){

//     }
//     pub fn switch_tab(){

//     }
// }
