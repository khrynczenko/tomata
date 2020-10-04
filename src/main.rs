mod settings;
mod state;
mod tomata;
mod widget;

use druid::{AppLauncher, MenuItem, PlatformError, Selector, WindowDesc};
use druid::{LocalizedString, MenuDesc};

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

fn main() -> Result<(), PlatformError> {
    let window = WindowDesc::new(TomataApp::new)
        .menu(make_menu())
        .title("tomata")
        .window_size(WINDOW_SIZE_PX)
        .resizable(false);

    let from_file_result = settings::load_settings_from_file("settings.json");
    let settings = if from_file_result.is_some() {
        from_file_result.unwrap()
    } else {
        let settings = Settings::default();
        settings::save_settings_to_file(&settings, "settings.json").expect(&format!(
            "{}{}",
            "Could not create `settings.json`", "to store the application settings."
        ));
        settings
    };

    let state = TomataState::new(settings);
    AppLauncher::with_window(window).launch(state)?;
    Ok(())
}
