pub struct Tab {
    pub id: usize,
    pub title: Option<String>,
    pub url: String,
    pub favicon_url: Option<String>,
    pub is_active: bool,
    pub is_loading: bool,
    pub is_selected: bool,
    pub is_closed: bool,
    pub can_go_back: bool,
    pub can_go_forward: bool,
}

impl Tab {
    pub fn new(id: usize, title: String, url: String, favicon_url: String) -> Self {
        Self {
            id,
            title: Some(title),
            url,
            favicon_url: Some(favicon_url),
            is_active: true,
            is_loading: true,
            is_selected: false,
            is_closed: false,
            can_go_back: false,
            can_go_forward: false,
        }
    }

    // pub fn set_active(&mut self, is_active:bool) {
    //     self.is_active = is_active;
    // }

    // pub fn set_loading(&mut self, is_loading:bool) {
    //     self.is_loading = is_loading;
    // }

    // pub fn set_selected(&mut self, is_selected:bool) {
    //     self.is_selected = is_selected;
    // }

    // pub fn set_audio_playing(&mut self, is_audio_playing:bool) {
    //     self.is_audio_playing = is_audio_playing;
    // }

    // pub fn set_audio_muted(&mut self, is_audio_muted:bool) {
    //     self.is_audio_muted = is_audio_muted;
    // }

    // pub fn set_closed(&mut self, is_closed:bool) {
    //     self.is_closed = is_closed;
    // }

    // pub fn set_can_go_back(&mut self, can_go_back:bool) {
    //     self.can_go_back = can_go_back;
    // }

    // pub fn set_can_go_forward(&mut self, can_go_forward:bool) {
    //     self.can_go_forward = can_go_forward;
    // }

    // pub fn set_title(&mut self, title:String) {
    //     self.title = title;
    // }

    // pub fn set_url(&mut self, url:String) {
    //     self.url = url;
    // }

    // pub fn set_favicon_url(&mut self, favicon_url:String) {
    //     self.favicon_url = favicon_url;
    // }
}
