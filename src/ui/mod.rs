use crate::page::RenderedPage;
use std::io::stdin;

pub enum NavigationSelection {
    Quit,
    Back,
    GoTo(String),
}

pub trait Ui {
    fn view(&self, page: &RenderedPage) -> NavigationSelection;
}

pub struct ConsoleUi {
    debug: bool,
}

impl ConsoleUi {
    pub fn new() -> Self {
        Self {
            debug: false,
        }
    }

    pub fn debug() -> Self {
        Self {
            debug: true,
        }
    }
}

impl Ui for ConsoleUi {
    fn view(&self, page: &RenderedPage) -> NavigationSelection {
        println!("\n\n\n\n===\n{}", page.rendered);
        let links: Vec<(String, String)> = page.links.iter().enumerate().map(|(index, (link_name, link_target))| (format!("{}   {}", index, link_name), link_target.clone())).collect();
        if self.debug {
            println!("\nLink Targets:\n{}\n", links.iter().map(|(name, target)| format!("{}     {}", name, target)).collect::<Vec<String>>().join("\n"));
        }
        if !links.is_empty() {
            println!("---\nLinks:\n{}", links.iter().map(|(link_title, _)| link_title.clone()).collect::<Vec<String>>().join("\n"));
        }

        let mut line = String::new();
        loop {
            stdin().read_line(&mut line).unwrap();
            let line = line.trim();
            if let Ok(number) = line.parse() {
                if let Some((_, link)) = links.get::<usize>(number) {
                    return NavigationSelection::GoTo(link.clone());
                }
            } else if line.to_lowercase().eq("q") {
                return NavigationSelection::Quit;
            } else if line.to_lowercase().eq("b") {
                return NavigationSelection::Back;
            }
        }
    }
}