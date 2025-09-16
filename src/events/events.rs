use std::fmt;

pub enum Events{
    Tab(TabUIEvents),
    NavBar(NavBarUIEvents),
    
}

pub enum TabUIEvents{
    NewTab(String),
    CloseTab(usize),
    SwitchTab(usize),
    RestoreTab(usize),
    RestoreAllTab,
    Scroll(bool)
}



pub enum NavBarUIEvents {
    Refresh,
    Forward,
    Backward,
    Cut,
    Copy,
    Paste,
    Rdp,
    VirtalDisk,
}




impl fmt::Display for TabUIEvents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TabUIEvents::NewTab(name) => write!(f, "NewTab({})", name),
            TabUIEvents::CloseTab(index) => write!(f, "CloseTab({})", index),
            TabUIEvents::SwitchTab(index) => write!(f, "SwitchTab({})", index),
            TabUIEvents::RestoreTab(index) => write!(f, "RestoreTab({})", index),
            TabUIEvents::RestoreAllTab => write!(f, "RestoreAllTab"),
            TabUIEvents::Scroll(flag)=>write!(f,"Scroll({})",flag),
        }
    }
}

impl fmt::Display for NavBarUIEvents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NavBarUIEvents::Refresh => write!(f, "Refresh"),
            NavBarUIEvents::Forward => write!(f, "Forward"),
            NavBarUIEvents::Backward => write!(f, "Backward"),
            NavBarUIEvents::Cut => write!(f, "Cut"),
            NavBarUIEvents::Copy => write!(f, "Copy"),
            NavBarUIEvents::Paste => write!(f, "Paste"),
            NavBarUIEvents::Rdp => write!(f, "Rdp"),
            NavBarUIEvents::VirtalDisk => write!(f, "VirtalDisk"),
        }
    }
}
impl fmt::Display for Events {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Events::Tab(tab_event) => write!(f, "Tab Event: {}", tab_event),
            Events::NavBar(navbar_event) => write!(f, "NavBar Event: {}", navbar_event),
        }
    }
}




impl fmt::Debug for TabUIEvents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TabUIEvents::NewTab(name) => write!(f, "NewTab({})", name),
            TabUIEvents::CloseTab(index) => write!(f, "CloseTab({})", index),
            TabUIEvents::SwitchTab(index) => write!(f, "SwitchTab({})", index),
            TabUIEvents::RestoreTab(index) => write!(f, "RestoreTab({})", index),
            TabUIEvents::RestoreAllTab => write!(f, "RestoreAllTab"),
            TabUIEvents::Scroll(flag)=>write!(f,"Scroll({})",flag),
        }
    }
}

impl fmt::Debug for NavBarUIEvents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NavBarUIEvents::Refresh => write!(f, "Refresh"),
            NavBarUIEvents::Forward => write!(f, "Forward"),
            NavBarUIEvents::Backward => write!(f, "Backward"),
            NavBarUIEvents::Cut => write!(f, "Cut"),
            NavBarUIEvents::Copy => write!(f, "Copy"),
            NavBarUIEvents::Paste => write!(f, "Paste"),
            NavBarUIEvents::Rdp => write!(f, "Rdp"),
            NavBarUIEvents::VirtalDisk => write!(f, "VirtalDisk"),
        }
    }
} 
impl fmt::Debug for Events {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Events::Tab(tab_event) => write!(f, "Tab Event: {:?}", tab_event),
            Events::NavBar(navbar_event) => write!(f, "NavBar Event: {:?}", navbar_event),
        }
    }
}
