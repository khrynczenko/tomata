mod settings;
mod state;
mod tomata;
mod widget;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use druid::{AppLauncher, MenuItem, PlatformError, Selector, WindowDesc};
use druid::{LocalizedString, MenuDesc};
use serde_json;

use settings::Settings;
use state::TomataState;
use tomata::WINDOW_SIZE_PX;
use widget::TomataApp;

fn make_menu() -> MenuDesc<TomataState> {
    let mut base = MenuDesc::empty();
    let mut help = MenuDesc::new(LocalizedString::new("Help"));
    help = help.append(MenuItem::new(
        LocalizedString::new("Settings"),
        Selector::new("Settings"),
    ));
    help = help.append(MenuItem::new(
        LocalizedString::new("About"),
        Selector::new("About"),
    ));
    base = base.append(help);
    base
}

fn load_settings(path: impl AsRef<Path>) -> Option<Settings> {
    match File::open(path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let result = serde_json::from_reader(reader);
            if let Ok(settings) = result {
                Some(settings)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn main() -> Result<(), PlatformError> {
    let window = WindowDesc::new(TomataApp::new)
        .menu(make_menu())
        .title("tomata")
        .window_size(WINDOW_SIZE_PX)
        .resizable(false);
    let settings = load_settings("settings.json").unwrap_or(Settings::default());
    let state = TomataState::new(settings);
    AppLauncher::with_window(window).launch(state)?;
    Ok(())
}
