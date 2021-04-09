use crate::page::RenderedPage;
use std::path::{Path, MAIN_SEPARATOR};

pub trait PageSource {
    fn load(&self, referrer: &Option<String>, address: &str) -> RenderedPage;
}

pub struct FilesystemSource;

impl PageSource for FilesystemSource {
    fn load(&self, referrer: &Option<String>, address: &str) -> RenderedPage {
        let address = address.replace("/", &MAIN_SEPARATOR.to_string());
        let address = Path::new(&address);
        let resolved = if address.is_absolute() {
            address.to_path_buf()
        } else if let Some(referrer) = referrer {
            let referrer = Path::new(referrer);
            if let Some(parent) = referrer.parent() {
                parent.join(address)
            } else {
                panic!("Referrer has no parent")
            }
        } else {
            panic!("Relative link without referrer")
        };
        let source = std::fs::read_to_string(resolved.clone()).unwrap();
        // println!("Source: {}", source);
        let resolved = resolved.to_str().unwrap().into();
        render_from_string(resolved, source)
    }
}

fn render_from_string(location: String, source: String) -> RenderedPage {
    let mut link_lines = vec![];
    let mut rendered = String::new();
    let mut in_links = true;
    for line in source.lines() {
        if in_links {
            if line.trim().is_empty() {
                in_links = false;
            } else {
                link_lines.push(line.trim().to_string());
            }
        } else {
            rendered.push_str(line);
            rendered.push('\n');
        }
    }
    RenderedPage {
        location,
        rendered,
        links: link_lines.into_iter().map(link_from_line).collect(),
    }
}

fn link_from_line(line: String) -> (String, String) {
    let mut words: Vec<&str> = line.split_whitespace().collect();
    let target = words.pop().unwrap().into();
    let name = words.join(" ");
    (name, target)
}

pub struct WebSource;

impl PageSource for WebSource {
    fn load(&self, referrer: &Option<String>, address: &str) -> RenderedPage {
        let address = if address.starts_with("http://") || address.starts_with("https://") {
            address.into()
        } else {
            if let Some(refer) = referrer {
                let mut split: Vec<&str> = refer.split("/").collect();
                split.pop();
                let mut address = address.clone();
                loop {
                    if address.starts_with("./") {
                        address = address[2..].into();
                    } else if address.starts_with("../") {
                        address = address[3..].into();
                        split.pop();
                    } else {
                        break;
                    }
                }
                let parent: String = split.join("/");
                format!("{}/{}", parent, address)
            } else {
                panic!("Relative link without referrer")
            }
        };
        let source = reqwest::blocking::get(&address).unwrap().text().unwrap();
        render_from_string(address, source)
    }
}