use crate::{debugger, models::Tab};

use std::{fmt::format, sync::Arc};


pub struct TabManager {
    pub tabs: Vec<Tab>,
    pub active_tab_id: usize,
    pub next_id: usize,
    pub active_tabset_count: usize,
    pub tabset_count: usize,
    pub closed_tabset_count: usize,
    // pub closed_tabset_stack: Vec<usize>,
}