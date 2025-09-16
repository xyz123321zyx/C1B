use std::{collections::HashMap, fmt};

use crate::{
    models::{Tab, TabManager, WebViewManager},
    ui::NavBarUI,
};


pub struct C1BState {
    pub tabmanager: TabManager,
    pub scroll_to_end: bool,
    pub webviewmanager:WebViewManager,
    // resource_manager:Option<String>
}



