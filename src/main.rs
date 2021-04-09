use crate::ui::ConsoleUi;
use crate::browser::Browser;
use crate::source::FilesystemSource;
use std::path::Path;

pub mod browser;
pub mod ui;
pub mod page;
pub mod source;

fn main() {
    let ui = ConsoleUi::new();
    let source = FilesystemSource;
    let mut browser = Browser::new(ui, source);
    let dir = std::env::current_dir().unwrap();
    let index = dir.join(Path::new("res/index.page"));
    browser.run(index.to_str().unwrap().to_string());
}
