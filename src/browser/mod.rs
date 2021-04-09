use crate::ui::{NavigationSelection, Ui};
use crate::source::PageSource;

pub struct Browser<U, S> where U: Ui, S: PageSource {
    ui: U,
    source: S,
    history: Vec<String>,
}

impl<U, S> Browser<U, S> where U: Ui, S: PageSource {
    pub fn new(ui: U, source: S) -> Self {
        Self {
            ui,
            source,
            history: vec![],
        }
    }

    pub fn run(&mut self, location: String) {
        let mut referrer = None;
        self.history.push(location.clone());
        loop {
            if let Some(address) = self.history.last() {
                let page = self.source.load(&referrer, &address);
                let nav = self.ui.view(&page);
                match nav {
                    NavigationSelection::Quit => return,
                    NavigationSelection::Back => {
                        self.history.pop();
                        if self.history.len() > 1 {
                            referrer = self.history.get(self.history.len() - 2).cloned();
                        } else {
                            referrer = None;
                        }
                    },
                    NavigationSelection::GoTo(address) => {
                        referrer = self.history.last().cloned();
                        self.history.push(address);
                    }
                }
            } else {
                break;
            }
        }
    }
}
