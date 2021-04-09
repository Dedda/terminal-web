use std::collections::HashMap;

#[derive(Debug)]
pub struct RenderedPage {
    pub location: String,
    pub rendered: String,
    pub links: HashMap<String, String>,
}