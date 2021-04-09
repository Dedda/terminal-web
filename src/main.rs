use crate::ui::ConsoleUi;
use crate::browser::Browser;
use crate::source::{FilesystemSource, WebSource};
use std::path::{Path, MAIN_SEPARATOR};
use std::env::args;

pub mod browser;
pub mod ui;
pub mod page;
pub mod source;

fn main() {
    let mut args = args();
    args.next();
    if let Some(target) = args.next() {
        if target.starts_with("http://") || target.starts_with("https://") {
            run_on_web(target);
        } else {
            run_on_filesystem(target);
        }
    }
}

fn run_on_filesystem(path: String) {
    let ui = ConsoleUi::new();
    let source = FilesystemSource;
    let mut browser = Browser::new(ui, source);
    let path = if Path::new(&path).is_relative() {
        std::env::current_dir().unwrap().join(Path::new(&path.replace("/", &MAIN_SEPARATOR.to_string()))).to_str().unwrap().to_string()
    } else {
        path
    };
    browser.run(path);
}

fn run_on_web(target: String) {
    let ui = ConsoleUi::new();
    let source = WebSource;
    let mut browser = Browser::new(ui, source);
    browser.run(target);
}