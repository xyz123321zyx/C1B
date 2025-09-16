use std::collections::HashMap;

pub struct WebViewManager{
    webviewcontent:HashMap<usize,Option<wry::WebView>>
}


impl WebViewManager  {
    pub fn new() -> Self{
        Self{
            webviewcontent: HashMap::new(),
        }
    }
}