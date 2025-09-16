pub struct Tab {
    pub id: usize,
    pub title: Option<String>,
    pub url: String,
    pub favicon_url: Option<String>,
    pub is_active: bool,
    pub is_loading: bool,
    pub is_closed: bool,
    pub can_go_back: bool,
    pub can_go_forward: bool,
}
