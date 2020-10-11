#![deny(warnings)]
#![deny(rust_2018_idioms)]
#![deny(clippy::all)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod settings;
mod sound;
mod state;
mod tomata;
mod widget;

use druid::{AppLauncher, PlatformError, WindowDesc};

use settings::Settings;
use sound::{SoundSystem, BEEPER};
use state::TomataState;
use tomata::{APPLICATION_NAME, WINDOW_SIZE_PX};
use widget::TomataApp;

fn main() -> Result<(), PlatformError> {
    let window = WindowDesc::new(TomataApp::new)
        .title(APPLICATION_NAME)
        .window_size(WINDOW_SIZE_PX)
        .resizable(false);
    BEEPER.set(SoundSystem::default()).unwrap();

    let settings_result = settings::load_settings_from_file("settings.json");
    let settings = if let Some(x) = settings_result {
        x
    } else {
        let settings = Settings::default();
        settings::save_settings_to_file(&settings, "settings.json").unwrap_or_else(|_| {
            panic!(
                "{} {}",
                "Could not create `settings.json`", "to store the application settings."
            )
        });
        settings
    };

    let state = TomataState::new(settings);
    AppLauncher::with_window(window).launch(state)?;
    Ok(())
}
