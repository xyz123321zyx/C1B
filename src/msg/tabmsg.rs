pub enum TabMsg{
    NewTab(String),
    CloseTab(usize),
    SwitchTab(usize),
    RestoreTab(usize),
    RestoreAllTab
}