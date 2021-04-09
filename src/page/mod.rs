use std::collections::HashMap;

pub struct RenderedPage {
    pub location: String,
    pub rendered: String,
    pub links: HashMap<String, String>,
}